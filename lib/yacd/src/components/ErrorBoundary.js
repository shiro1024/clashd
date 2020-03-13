import React, { Component } from 'react';
import PropTypes from 'prop-types';
import ErrorBoundaryFallback from './ErrorBoundaryFallback';
import { deriveMessageFromError } from '../misc/errors';

// XXX this is no Hook equivalents for componentDidCatch
// we have to use class for now

class ErrorBoundary extends Component {
  static propTypes = {
    children: PropTypes.node
  };

  state = { error: null };

  // static getDerivedStateFromError(error) {
  //   return { error };
  // }

  componentDidCatch(error, _info) {
    this.setState({ error });
    // eslint-disable-next-line no-console
    // console.log(error, errorInfo);
    // this.setState({ error });
  }

  render() {
    if (this.state.error) {
      const { message, detail } = deriveMessageFromError(this.state.error);
      //render fallback UI
      return <ErrorBoundaryFallback message={message} detail={detail} />;
    } else {
      return this.props.children;
    }
  }
}

export default ErrorBoundary;
