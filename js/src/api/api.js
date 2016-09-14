<<<<<<< HEAD
import { Http, Ws } from './transport';
import Contract from './contract';

import { Db, Eth, Ethcore, Net, Personal, Shh, Trace, Web3 } from './rpc';
import Subscriptions from './subscriptions';
import format from './format';
=======
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

import { Http, Ws } from './transport/index';
import Contract from './contract/index';

import { Db, Eth, Ethcore, Net, Personal, Shh, Trace, Web3 } from './rpc/index';
import Subscriptions from './subscriptions/index';
import format from './format/index';
import util from './util/index';
>>>>>>> js
import { isFunction } from './util/types';

export default class Api {
  constructor (transport) {
    if (!transport || !isFunction(transport.execute)) {
      throw new Error('EthApi needs transport with execute() function defined');
    }

    this._db = new Db(transport);
    this._eth = new Eth(transport);
    this._ethcore = new Ethcore(transport);
    this._net = new Net(transport);
    this._personal = new Personal(transport);
    this._shh = new Shh(transport);
    this._trace = new Trace(transport);
    this._web3 = new Web3(transport);

    this._subscriptions = new Subscriptions(this);
  }

  get db () {
    return this._db;
  }

  get eth () {
    return this._eth;
  }

  get ethcore () {
    return this._ethcore;
  }

  get net () {
    return this._net;
  }

  get personal () {
    return this._personal;
  }

  get shh () {
    return this._shh;
  }

  get trace () {
    return this._trace;
  }

  get web3 () {
    return this._web3;
  }

  get format () {
    return format;
  }

  get util () {
    return util;
  }

  newContract (abi, address) {
    return new Contract(this, abi).at(address);
  }

  subscribe (subscriptionName, callback) {
    return this._subscriptions.subscribe(subscriptionName, callback);
  }

  unsubscribe (subscriptionId) {
    return this._subscriptions.unsubscribe(subscriptionId);
  }

  static Transport = {
    Http: Http,
    Ws: Ws
  }
}
