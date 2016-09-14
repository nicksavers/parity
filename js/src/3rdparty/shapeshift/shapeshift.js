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

export default function (rpc) {
  const subscriptions = [];

  function getCoins () {
    return rpc.get('getcoins');
  }

  function getMarketInfo (pair) {
    return rpc.get(`marketinfo/${pair}`);
  }

  function getStatus (depositAddress) {
    return rpc.get(`txStat/${depositAddress}`);
  }

  function shift (toAddress, returnAddress, pair) {
    return rpc.post('shift', {
      withdrawal: toAddress,
      pair: pair,
      returnAddress: returnAddress
    });
  }

  function subscribe (depositAddress, callback) {
    const idx = subscriptions.length;

    subscriptions.push({
      depositAddress,
      callback,
      idx
    });

    return idx;
  }

  function _getStatusSubscription (subscription) {
    if (!subscription) {
      return;
    }

    getStatus(subscription.depositAddress)
      .then((result) => {
        switch (result.status) {
          case 'no_deposits':
          case 'received':
            subscription.callback(null, status);
            return;

          case 'complete':
          case 'failed':
            subscription.callback(status.error, status);
            subscriptions[subscription.idx] = null;
            return;
        }
      })
      .catch((error) => subscription.callback(error.message));
  }

  function _pollStatus () {
    subscriptions.map(_getStatusSubscription);
  }

  setInterval(_pollStatus, 2000);

  return {
    getCoins,
    getMarketInfo,
    getStatus,
    shift,
    subscribe
  };
}
