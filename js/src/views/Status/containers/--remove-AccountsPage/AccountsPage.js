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

import React, { Component, PropTypes } from 'react';
import { connect } from 'react-redux';

class AccountsPage extends Component {

  render () {
    return (
      <div className='dapp-flex-content'>
        <main className='dapp-content'>
          <h1>Accounts</h1>
        </main>
      </div>
    );
  }

  static propTypes = {
    logger: PropTypes.object.isRequired,
    actions: PropTypes.object.isRequired,
    status: PropTypes.object.isRequired
  }
}

function mapStateToProps (state) {
  return state;
}

function mapDispatchToProps (dispatch) {
  return {};
}

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(AccountsPage);