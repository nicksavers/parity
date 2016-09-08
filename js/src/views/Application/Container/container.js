import React, { Component, PropTypes } from 'react';

import { FirstRun } from '../../../modals';
import { Errors, Tooltips } from '../../../ui';

import styles from '../application.css';

export default class Container extends Component {
  static propTypes = {
    children: PropTypes.node,
    showFirstRun: PropTypes.bool,
    onCloseFirstRun: PropTypes.func
  };

  render () {
    const { children, showFirstRun, onCloseFirstRun } = this.props;

    return (
      <div className={ styles.container }>
        <FirstRun
          visible={ showFirstRun }
          onClose={ onCloseFirstRun } />
        <Tooltips />
        <Errors />
        { children }
      </div>
    );
  }
}