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

//! Eth rpc interface.
use std::sync::Arc;
use jsonrpc_core::*;

/// Eth rpc interface.
pub trait Eth: Sized + Send + Sync + 'static {
	/// Returns protocol version.
	fn protocol_version(&self, _: Params) -> Result<Value, Error>;

	/// Returns an object with data about the sync status or false. (wtf?)
	fn syncing(&self, _: Params) -> Result<Value, Error>;

	/// Returns the number of hashes per second that the node is mining with.
	fn hashrate(&self, _: Params) -> Result<Value, Error>;

	/// Returns block author.
	fn author(&self, _: Params) -> Result<Value, Error>;

	/// Returns true if client is actively mining new blocks.
	fn is_mining(&self, _: Params) -> Result<Value, Error>;

	/// Returns current gas_price.
	fn gas_price(&self, _: Params) -> Result<Value, Error>;

	/// Returns accounts list.
	fn accounts(&self, _: Params) -> Result<Value, Error>;

	/// Returns highest block number.
	fn block_number(&self, _: Params) -> Result<Value, Error>;

	/// Returns balance of the given account.
	fn balance(&self, _: Params) -> Result<Value, Error>;

	/// Returns content of the storage at given address.
	fn storage_at(&self, _: Params) -> Result<Value, Error>;

	/// Returns block with given hash.
	fn block_by_hash(&self, _: Params) -> Result<Value, Error>;

	/// Returns block with given number.
	fn block_by_number(&self, _: Params) -> Result<Value, Error>;

	/// Returns the number of transactions sent from given address at given time (block number).
	fn transaction_count(&self, _: Params) -> Result<Value, Error>;

	/// Returns the number of transactions in a block with given hash.
	fn block_transaction_count_by_hash(&self, _: Params) -> Result<Value, Error>;

	/// Returns the number of transactions in a block with given block number.
	fn block_transaction_count_by_number(&self, _: Params) -> Result<Value, Error>;

	/// Returns the number of uncles in a block with given hash.
	fn block_uncles_count_by_hash(&self, _: Params) -> Result<Value, Error>;

	/// Returns the number of uncles in a block with given block number.
	fn block_uncles_count_by_number(&self, _: Params) -> Result<Value, Error>;

	/// Returns the code at given address at given time (block number).
	fn code_at(&self, _: Params) -> Result<Value, Error>;

	/// Sends signed transaction.
	fn send_raw_transaction(&self, _: Params) -> Result<Value, Error>;

	/// Call contract.
	fn call(&self, _: Params) -> Result<Value, Error>;

	/// Estimate gas needed for execution of given contract.
	fn estimate_gas(&self, _: Params) -> Result<Value, Error>;

	/// Get transaction by its hash.
	fn transaction_by_hash(&self, _: Params) -> Result<Value, Error>;

	/// Returns transaction at given block hash and index.
	fn transaction_by_block_hash_and_index(&self, _: Params) -> Result<Value, Error>;

	/// Returns transaction by given block number and index.
	fn transaction_by_block_number_and_index(&self, _: Params) -> Result<Value, Error>;

	/// Returns transaction receipt.
	fn transaction_receipt(&self, _: Params) -> Result<Value, Error>;

	/// Returns an uncles at given block and index.
	fn uncle_by_block_hash_and_index(&self, _: Params) -> Result<Value, Error>;

	/// Returns an uncles at given block and index.
	fn uncle_by_block_number_and_index(&self, _: Params) -> Result<Value, Error>;

	/// Returns available compilers.
	fn compilers(&self, _: Params) -> Result<Value, Error>;

	/// Compiles lll code.
	fn compile_lll(&self, _: Params) -> Result<Value, Error>;

	/// Compiles solidity.
	fn compile_solidity(&self, _: Params) -> Result<Value, Error>;

	/// Compiles serpent.
	fn compile_serpent(&self, _: Params) -> Result<Value, Error>;

	/// Returns logs matching given filter object.
	fn logs(&self, _: Params) -> Result<Value, Error>;

	/// Returns the hash of the current block, the seedHash, and the boundary condition to be met.
	fn work(&self, _: Params) -> Result<Value, Error>;

	/// Used for submitting a proof-of-work solution.
	fn submit_work(&self, _: Params) -> Result<Value, Error>;

	/// Used for submitting mining hashrate.
	fn submit_hashrate(&self, _: Params) -> Result<Value, Error>;

	/// Should be used to convert object to io delegate.
	fn to_delegate(self) -> IoDelegate<Self> {
		let mut delegate = IoDelegate::new(Arc::new(self));
		delegate.add_method("eth_protocolVersion", Eth::protocol_version);
		delegate.add_method("eth_syncing", Eth::syncing);
		delegate.add_method("eth_hashrate", Eth::hashrate);
		delegate.add_method("eth_coinbase", Eth::author);
		delegate.add_method("eth_mining", Eth::is_mining);
		delegate.add_method("eth_gasPrice", Eth::gas_price);
		delegate.add_method("eth_accounts", Eth::accounts);
		delegate.add_method("eth_blockNumber", Eth::block_number);
		delegate.add_method("eth_getBalance", Eth::balance);
		delegate.add_method("eth_getStorageAt", Eth::storage_at);
		delegate.add_method("eth_getTransactionCount", Eth::transaction_count);
		delegate.add_method("eth_getBlockTransactionCountByHash", Eth::block_transaction_count_by_hash);
		delegate.add_method("eth_getBlockTransactionCountByNumber", Eth::block_transaction_count_by_number);
		delegate.add_method("eth_getUncleCountByBlockHash", Eth::block_uncles_count_by_hash);
		delegate.add_method("eth_getUncleCountByBlockNumber", Eth::block_uncles_count_by_number);
		delegate.add_method("eth_getCode", Eth::code_at);
		delegate.add_method("eth_sendRawTransaction", Eth::send_raw_transaction);
		delegate.add_method("eth_call", Eth::call);
		delegate.add_method("eth_estimateGas", Eth::estimate_gas);
		delegate.add_method("eth_getBlockByHash", Eth::block_by_hash);
		delegate.add_method("eth_getBlockByNumber", Eth::block_by_number);
		delegate.add_method("eth_getTransactionByHash", Eth::transaction_by_hash);
		delegate.add_method("eth_getTransactionByBlockHashAndIndex", Eth::transaction_by_block_hash_and_index);
		delegate.add_method("eth_getTransactionByBlockNumberAndIndex", Eth::transaction_by_block_number_and_index);
		delegate.add_method("eth_getTransactionReceipt", Eth::transaction_receipt);
		delegate.add_method("eth_getUncleByBlockHashAndIndex", Eth::uncle_by_block_hash_and_index);
		delegate.add_method("eth_getUncleByBlockNumberAndIndex", Eth::uncle_by_block_number_and_index);
		delegate.add_method("eth_getCompilers", Eth::compilers);
		delegate.add_method("eth_compileLLL", Eth::compile_lll);
		delegate.add_method("eth_compileSolidity", Eth::compile_solidity);
		delegate.add_method("eth_compileSerpent", Eth::compile_serpent);
		delegate.add_method("eth_getLogs", Eth::logs);
		delegate.add_method("eth_getWork", Eth::work);
		delegate.add_method("eth_submitWork", Eth::submit_work);
		delegate.add_method("eth_submitHashrate", Eth::submit_hashrate);
		delegate
	}
}

/// Eth filters rpc api (polling).
// TODO: do filters api properly
pub trait EthFilter: Sized + Send + Sync + 'static {
	/// Returns id of new filter.
	fn new_filter(&self, _: Params) -> Result<Value, Error>;

	/// Returns id of new block filter.
	fn new_block_filter(&self, _: Params) -> Result<Value, Error>;

	/// Returns id of new block filter.
	fn new_pending_transaction_filter(&self, _: Params) -> Result<Value, Error>;

	/// Returns filter changes since last poll.
	fn filter_changes(&self, _: Params) -> Result<Value, Error>;

	/// Returns all logs matching given filter (in a range 'from' - 'to').
	fn filter_logs(&self, _: Params) -> Result<Value, Error>;

	/// Uninstalls filter.
	fn uninstall_filter(&self, _: Params) -> Result<Value, Error>;

	/// Should be used to convert object to io delegate.
	fn to_delegate(self) -> IoDelegate<Self> {
		let mut delegate = IoDelegate::new(Arc::new(self));
		delegate.add_method("eth_newFilter", EthFilter::new_filter);
		delegate.add_method("eth_newBlockFilter", EthFilter::new_block_filter);
		delegate.add_method("eth_newPendingTransactionFilter", EthFilter::new_pending_transaction_filter);
		delegate.add_method("eth_getFilterChanges", EthFilter::filter_changes);
		delegate.add_method("eth_getFilterLogs", EthFilter::filter_logs);
		delegate.add_method("eth_uninstallFilter", EthFilter::uninstall_filter);
		delegate
	}
}

/// Signing methods implementation relying on unlocked accounts.
pub trait EthSigning: Sized + Send + Sync + 'static {
	/// Signs the data with given address signature.
	fn sign(&self, _: Params) -> Result<Value, Error>;

	/// Posts sign request asynchronously.
	/// Will return a confirmation ID for later use with check_transaction.
	fn post_sign(&self, _: Params) -> Result<Value, Error>;

	/// Sends transaction; will block for 20s to try to return the
	/// transaction hash.
	/// If it cannot yet be signed, it will return a transaction ID for
	/// later use with check_transaction.
	fn send_transaction(&self, _: Params) -> Result<Value, Error>;

	/// Posts transaction asynchronously.
	/// Will return a transaction ID for later use with check_transaction.
	fn post_transaction(&self, _: Params) -> Result<Value, Error>;

	/// Checks the progress of a previously posted request (transaction/sign).
	/// Should be given a valid send_transaction ID.
	/// Returns the transaction hash, the zero hash (not yet available),
	/// or the signature,
	/// or an error.
	fn check_request(&self, _: Params) -> Result<Value, Error>;

	/// Decrypt some ECIES-encrypted message.
	/// First parameter is the address with which it is encrypted, second is the ciphertext.
	fn decrypt_message(&self, _: Params) -> Result<Value, Error>;

	/// Should be used to convert object to io delegate.
	fn to_delegate(self) -> IoDelegate<Self> {
		let mut delegate = IoDelegate::new(Arc::new(self));
		delegate.add_method("eth_sign", EthSigning::sign);
		delegate.add_method("eth_sendTransaction", EthSigning::send_transaction);
		delegate.add_method("eth_postSign", EthSigning::post_sign);
		delegate.add_method("eth_postTransaction", EthSigning::post_transaction);
		delegate.add_method("eth_checkRequest", EthSigning::check_request);
		delegate.add_method("ethcore_decryptMessage", EthSigning::decrypt_message);
		delegate
	}
}
