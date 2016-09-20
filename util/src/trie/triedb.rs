// Copyright 2015, 2016 Ethcore (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use common::*;
use hashdb::*;
use nibbleslice::*;
use rlp::*;
use super::node::Node;
use super::recorder::{Recorder, NoOp};
use super::{Trie, TrieItem, TrieError};

/// A `Trie` implementation using a generic `HashDB` backing database.
///
/// Use it as a `Trie` trait object. You can use `db()` to get the backing database object, `keys`
/// to get the keys belonging to the trie in the backing database, and `db_items_remaining()` to get
/// which items in the backing database do not belong to this trie. If this is the only trie in the
/// backing database, then `db_items_remaining()` should be empty.
///
/// # Example
/// ```
/// extern crate ethcore_util as util;
///
/// use util::trie::*;
/// use util::hashdb::*;
/// use util::memorydb::*;
/// use util::hash::*;
///
/// fn main() {
///   let mut memdb = MemoryDB::new();
///   let mut root = H256::new();
///   TrieDBMut::new(&mut memdb, &mut root).insert(b"foo", b"bar").unwrap();
///   let t = TrieDB::new(&memdb, &root).unwrap();
///   assert!(t.contains(b"foo").unwrap());
///   assert_eq!(t.get(b"foo").unwrap().unwrap(), b"bar");
///   assert!(t.db_items_remaining().unwrap().is_empty());
/// }
/// ```
pub struct TrieDB<'db> {
	db: &'db HashDB,
	root: &'db H256,
	/// The number of hashes performed so far in operations on this trie.
	pub hash_count: usize,
}

#[cfg_attr(feature="dev", allow(wrong_self_convention))]
impl<'db> TrieDB<'db> {
	/// Create a new trie with the backing database `db` and `root`
	/// Returns an error if `root` does not exist
	pub fn new(db: &'db HashDB, root: &'db H256) -> super::Result<Self> {
		if !db.contains(root) {
			Err(Box::new(TrieError::InvalidStateRoot(*root)))
		} else {
			Ok(TrieDB {
				db: db,
				root: root,
				hash_count: 0
			})
		}
	}

	/// Get the backing database.
	pub fn db(&'db self) -> &'db HashDB {
		self.db
	}

	/// Determine all the keys in the backing database that belong to the trie.
	pub fn keys(&self) -> super::Result<Vec<H256>> {
		let mut ret: Vec<H256> = Vec::new();
		ret.push(self.root.clone());
		try!(self.accumulate_keys(try!(self.root_node(&mut NoOp)), &mut ret));
		Ok(ret)
	}

	/// Convert a vector of hashes to a hashmap of hash to occurrences.
	pub fn to_map(hashes: Vec<H256>) -> HashMap<H256, u32> {
		let mut r: HashMap<H256, u32> = HashMap::new();
		for h in hashes.into_iter() {
			*r.entry(h).or_insert(0) += 1;
		}
		r
	}

	/// Determine occurrences of items in the backing database which are not related to this
	/// trie.
	pub fn db_items_remaining(&self) -> super::Result<HashMap<H256, i32>> {
		let mut ret = self.db.keys();
		for (k, v) in Self::to_map(try!(self.keys())).into_iter() {
			let keycount = *ret.get(&k).unwrap_or(&0);
			match keycount <= v as i32 {
				true => ret.remove(&k),
				_ => ret.insert(k, keycount - v as i32),
			};
		}
		Ok(ret)
	}

	/// Recursion helper for `keys`.
	fn accumulate_keys(&self, node: Node, acc: &mut Vec<H256>) -> super::Result<()> {
		let mut handle_payload = |payload| {
			let p = Rlp::new(payload);
			if p.is_data() && p.size() == 32 {
				acc.push(p.as_val());
			}

			self.accumulate_keys(try!(self.get_node(payload, &mut NoOp, 0)), acc)
		};

		match node {
			Node::Extension(_, payload) => try!(handle_payload(payload)),
			Node::Branch(payloads, _) => for payload in &payloads { try!(handle_payload(payload)) },
			_ => {},
		}

		Ok(())
	}

	/// Get the root node's RLP.
	fn root_node<R: Recorder>(&self, r: &mut R) -> super::Result<Node> {
		self.root_data(r).map(Node::decoded)
	}

	/// Get the data of the root node.
	fn root_data<'a, R: 'a + Recorder>(&self, r: &'a mut R) -> super::Result<&[u8]> {
		self.db.get(self.root).ok_or_else(|| Box::new(TrieError::InvalidStateRoot(*self.root)))
			.map(|node| { r.record(self.root, node, 0); node })
	}

	/// Get the root node as a `Node`.
	fn get_node<'a, R: 'a + Recorder>(&'db self, node: &'db [u8], r: &'a mut R, depth: u32) -> super::Result<Node> {
		self.get_raw_or_lookup(node, r, depth).map(Node::decoded)
	}

	/// Indentation helper for `formal_all`.
	fn fmt_indent(&self, f: &mut fmt::Formatter, size: usize) -> fmt::Result {
		for _ in 0..size {
			try!(write!(f, "  "));
		}
		Ok(())
	}

	/// Recursion helper for implementation of formatting trait.
	fn fmt_all(&self, node: Node, f: &mut fmt::Formatter, deepness: usize) -> fmt::Result {
		match node {
			Node::Leaf(slice, value) => try!(writeln!(f, "'{:?}: {:?}.", slice, value.pretty())),
			Node::Extension(ref slice, item) => {
				try!(write!(f, "'{:?} ", slice));
				if let Ok(node) = self.get_node(item, &mut NoOp, 0) {
					try!(self.fmt_all(node, f, deepness));
				}
			},
			Node::Branch(ref nodes, ref value) => {
				try!(writeln!(f, ""));
				if let Some(v) = *value {
					try!(self.fmt_indent(f, deepness + 1));
					try!(writeln!(f, "=: {:?}", v.pretty()))
				}
				for i in 0..16 {
					match self.get_node(nodes[i], &mut NoOp, 0) {
						Ok(Node::Empty) => {},
						Ok(n) => {
							try!(self.fmt_indent(f, deepness + 1));
							try!(write!(f, "'{:x} ", i));
							try!(self.fmt_all(n, f, deepness + 1));
						}
						Err(e) => {
							try!(write!(f, "ERROR: {}", e));
						}
					}
				}
			},
			// empty
			Node::Empty => {
				try!(writeln!(f, "<empty>"));
			}
		};
		Ok(())
	}

	/// Return optional data for a key given as a `NibbleSlice`. Returns `None` if no data exists.
	fn do_lookup<'key, R: 'key>(&'db self, key: &NibbleSlice<'key>, r: &'key mut R) -> super::Result<Option<&'db [u8]>>
		where 'db: 'key, R: Recorder
	{
		let root_rlp = try!(self.root_data(r));
		self.get_from_node(root_rlp, key, r, 1)
	}

	/// Recursible function to retrieve the value given a `node` and a partial `key`. `None` if no
	/// value exists for the key.
	///
	/// Note: Not a public API; use Trie trait functions.
	fn get_from_node<'key, R: 'key>(
		&'db self,
		node: &'db [u8],
		key: &NibbleSlice<'key>,
		r: &'key mut R,
		d: u32
	) -> super::Result<Option<&'db [u8]>> where 'db: 'key, R: Recorder {
		match Node::decoded(node) {
			Node::Leaf(ref slice, value) if key == slice => Ok(Some(value)),
			Node::Extension(ref slice, item) if key.starts_with(slice) => {
				let data = try!(self.get_raw_or_lookup(item, r, d));
				self.get_from_node(data, &key.mid(slice.len()), r, d + 1)
			},
			Node::Branch(ref nodes, value) => match key.is_empty() {
				true => Ok(value),
				false => {
					let node = try!(self.get_raw_or_lookup(nodes[key.at(0) as usize], r, d));
					self.get_from_node(node, &key.mid(1), r, d + 1)
				}
			},
			_ => Ok(None)
		}
	}

	/// Given some node-describing data `node`, return the actual node RLP.
	/// This could be a simple identity operation in the case that the node is sufficiently small, but
	/// may require a database lookup.
	fn get_raw_or_lookup<R: Recorder>(&'db self, node: &'db [u8], rec: &mut R, d: u32) -> super::Result<&'db [u8]> {
		// check if its sha3 + len
		let r = Rlp::new(node);
		match r.is_data() && r.size() == 32 {
			true => {
				let key = r.as_val::<H256>();
				self.db.get(&key).ok_or_else(|| Box::new(TrieError::IncompleteDatabase(key)))
					.map(|raw| { rec.record(&key, raw, d); raw })
			}
			false => Ok(node)
		}
	}
}

#[derive(Clone, Eq, PartialEq)]
enum Status {
	Entering,
	At,
	AtChild(usize),
	Exiting,
}

#[derive(Clone, Eq, PartialEq)]
struct Crumb<'a> {
	node: Node<'a>,
	status: Status,
}

impl<'a> Crumb<'a> {
	/// Move on to next status in the node's sequence.
	fn increment(&mut self) {
		self.status = match (&self.status, &self.node) {
			(_, &Node::Empty) => Status::Exiting,
			(&Status::Entering, _) => Status::At,
			(&Status::At, &Node::Branch(_, _)) => Status::AtChild(0),
			(&Status::AtChild(x), &Node::Branch(_, _)) if x < 15 => Status::AtChild(x + 1),
			_ => Status::Exiting,
		}
	}
}

/// Iterator for going through all values in the trie.
#[derive(Clone)]
pub struct TrieDBIterator<'a> {
	db: &'a TrieDB<'a>,
	trail: Vec<Crumb<'a>>,
	key_nibbles: Bytes,
}

impl<'a> TrieDBIterator<'a> {
	/// Create a new iterator.
	pub fn new(db: &'a TrieDB) -> TrieDBIterator<'a> {
		let mut r = TrieDBIterator {
			db: db,
			trail: vec![],
			key_nibbles: Vec::new(),
		};
		r.descend(db.root_data(&mut NoOp).unwrap());
		r
	}

	/// Descend into a payload.
	fn descend(&mut self, d: &'a [u8]) {
		self.trail.push(Crumb {
			status: Status::Entering,
			node: self.db.get_node(d, &mut NoOp, 0).unwrap(),
		});
		match self.trail.last().unwrap().node {
			Node::Leaf(n, _) | Node::Extension(n, _) => { self.key_nibbles.extend(n.iter()); },
			_ => {}
		}
	}

	/// Descend into a payload and get the next item.
	fn descend_next(&mut self, d: &'a [u8]) -> Option<(Bytes, &'a [u8])> { self.descend(d); self.next() }

	/// The present key.
	fn key(&self) -> Bytes {
		// collapse the key_nibbles down to bytes.
		self.key_nibbles.iter().step(2).zip(self.key_nibbles.iter().skip(1).step(2)).map(|(h, l)| h * 16 + l).collect()
	}
}

impl<'a> Iterator for TrieDBIterator<'a> {
	type Item = (Bytes, &'a [u8]);

	fn next(&mut self) -> Option<Self::Item> {
		let b = match self.trail.last_mut() {
			Some(mut b) => { b.increment(); b.clone() },
			None => return None
		};
		match (b.status, b.node) {
			(Status::Exiting, n) => {
				match n {
					Node::Leaf(n, _) | Node::Extension(n, _) => {
						let l = self.key_nibbles.len();
						self.key_nibbles.truncate(l - n.len());
					},
					Node::Branch(_, _) => { self.key_nibbles.pop(); },
					_ => {}
				}
				self.trail.pop();
				self.next()
			},
			(Status::At, Node::Leaf(_, v)) | (Status::At, Node::Branch(_, Some(v))) => Some((self.key(), v)),
			(Status::At, Node::Extension(_, d)) => self.descend_next(d),
			(Status::At, Node::Branch(_, _)) => self.next(),
			(Status::AtChild(i), Node::Branch(children, _)) if children[i].len() > 0 => {
				match i {
					0 => self.key_nibbles.push(0),
					i => *self.key_nibbles.last_mut().unwrap() = i as u8,
				}
				self.descend_next(children[i])
			},
			(Status::AtChild(i), Node::Branch(_, _)) => {
				if i == 0 { self.key_nibbles.push(0); }
				self.next()
			},
			_ => panic!() // Should never see Entering or AtChild without a Branch here.
		}
	}
}

impl<'db> Trie for TrieDB<'db> {
	fn iter<'a>(&'a self) -> Box<Iterator<Item = TrieItem> + 'a> {
		Box::new(TrieDBIterator::new(self))
	}

	fn root(&self) -> &H256 { self.root }

	fn get_recorded<'a, 'b, R: 'b>(&'a self, key: &'b [u8], rec: &'b mut R) -> super::Result<Option<&'a [u8]>>
		where 'a: 'b, R: Recorder
	{
		self.do_lookup(&NibbleSlice::new(key), rec)
	}
}

impl<'db> fmt::Debug for TrieDB<'db> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		try!(writeln!(f, "c={:?} [", self.hash_count));
		let root_rlp = self.db.get(self.root).expect("Trie root not found!");
		try!(self.fmt_all(Node::decoded(root_rlp), f, 0));
		writeln!(f, "]")
	}
}

#[test]
fn iterator() {
	use memorydb::*;
	use super::TrieMut;
	use super::triedbmut::*;

	let d = vec![ &b"A"[..], &b"AA"[..], &b"AB"[..], &b"B"[..] ];

	let mut memdb = MemoryDB::new();
	let mut root = H256::new();
	{
		let mut t = TrieDBMut::new(&mut memdb, &mut root);
		for x in &d {
			t.insert(x, x).unwrap();
		}
	}

	let t = TrieDB::new(&memdb, &root).unwrap();
	assert_eq!(d.iter().map(|i|i.to_vec()).collect::<Vec<_>>(), t.iter().map(|x|x.0).collect::<Vec<_>>());
	assert_eq!(d, t.iter().map(|x|x.1).collect::<Vec<_>>());
}
