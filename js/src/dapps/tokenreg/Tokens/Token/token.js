import React, { Component, PropTypes } from 'react';
import Paper from 'material-ui/Paper';
import { RaisedButton, TextField } from 'material-ui';
import FindIcon from 'material-ui/svg-icons/action/find-in-page';
import DeleteIcon from 'material-ui/svg-icons/action/delete';
import AddIcon from 'material-ui/svg-icons/content/add';

import Loading from '../../Loading';
import Chip from '../../Chip';
import AddMeta from './add-meta';

import styles from './token.css';

export default class Token extends Component {
  static propTypes = {
    handleAddMeta: PropTypes.func,
    handleUnregister: PropTypes.func,
    handleMetaLookup: PropTypes.func,
    isLoading: PropTypes.bool,
    isPending: PropTypes.bool,
    isOwner: PropTypes.bool,
    isTokenOwner: PropTypes.bool,
    address: PropTypes.string,
    tla: PropTypes.string,
    name: PropTypes.string,
    base: PropTypes.number,
    index: PropTypes.number,
    meta: PropTypes.object,
    owner: PropTypes.string
  };

  state = {
    metaQuery: ''
  };

  render () {
    const { isLoading, address, tla, base, name, meta, owner } = this.props;

    if (isLoading) {
      return (
        <div className={ styles.token }>
          <Loading size={ 1 } />
        </div>
      );
    }

    return (
      <Paper zDepth={ 2 } className={ styles.token }>
        { this.renderIsPending() }
        <div className={ styles.title }>{ tla }</div>
        <div className={ styles.name }>"{ name }"</div>

        { this.renderBase(base) }
        { this.renderAddress(address) }
        { this.renderOwner(owner) }

        <div className={ styles.metaForm }>
          <TextField
            autoComplete='off'
            floatingLabelFixed
            fullWidth
            floatingLabelText='Meta Key'
            hintText='The key of the meta-data to lookup'
            value={ this.state.metaQuery }
            onChange={ this.onMetaQueryChange } />

          <RaisedButton
            label='Lookup'
            icon={ <FindIcon /> }
            primary
            fullWidth
            onTouchTap={ this.onMetaLookup } />
        </div>

        { this.renderMeta(meta) }
        { this.renderAddMeta() }
        { this.renderUnregister() }
      </Paper>
    );
  }

  renderBase (base) {
    if (!base || base < 0) return null;
    return (
      <Chip
        value={ base.toString() }
        label='Base' />
    );
  }

  renderAddress (address) {
    if (!address) return null;
    return (
      <Chip
        isAddress
        value={ address }
        label='Address' />
    );
  }

  renderOwner (owner) {
    if (!owner) return null;
    return (
      <Chip
        isAddress
        value={ owner }
        label='Owner' />
    );
  }

  renderIsPending () {
    const { isPending } = this.props;

    if (!isPending) return null;

    return (
      <div className={ styles.pending } />
    );
  }

  renderAddMeta () {
    return (
      <AddMeta
        handleAddMeta={ this.props.handleAddMeta }
        isTokenOwner={ this.props.isTokenOwner }
        index={ this.props.index } />
    );
  }

  renderUnregister () {
    if (!this.props.isOwner) return null;

    return (
      <RaisedButton
        className={ styles.unregister }
        label='Unregister'
        icon={ <DeleteIcon /> }
        secondary
        fullWidth
        onTouchTap={ this.onUnregister } />
    );
  }

  renderMeta (meta) {
    if (!meta) return;

    return (<div>
      <p className={ styles['meta-query'] }>
        Meta for key
        <span className={ styles['meta-key'] }>{ meta.query }</span>:
      </p>
      <p className={ styles['meta-value'] }>{ meta.value }</p>
    </div>);
  }

  onUnregister = () => {
    let index = this.props.index;
    this.props.handleUnregister(index);
  }

  onMetaLookup = () => {
    let query = this.state.metaQuery;
    let index = this.props.index;

    this.props.handleMetaLookup(index, query);
  }

  onMetaQueryChange = (event, metaQuery) => {
    this.setState({ metaQuery });
  }
}
