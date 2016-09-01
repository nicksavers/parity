import React, { Component } from 'react';
import { bindActionCreators } from 'redux';
import { connect } from 'react-redux';

import { updateAppState } from '../../actions/signer';

import Offline from '../../components/Offline';

class OfflinePage extends Component {
  render () {
    return (
      <Offline { ...this.props } />
    );
  }
}

function mapStateToProps (state) {
  return {
    parityUrl: state.signer.url
  };
}

function mapDispatchToProps (dispatch) {
  return bindActionCreators({ updateAppState }, dispatch);
}

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(OfflinePage);
