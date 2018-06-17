import React, { Component } from 'react';
import { Provider } from 'react-redux';
import { render, Window, App } from 'proton-native';

import { configureStore } from './src/modulemain/ducks/store.js';
import VSTHostApp from './src/modulemain/components/VSTHostApp';

class ProtonMain extends Component {
  render() {
    return (
      <Provider store={configureStore()}>
        <App>
          <Window title="VST host" size={{w: 300, h: 300}} menuBar={false}>
            <VSTHostApp />
          </Window>
        </App>
      </Provider>
    );
  }
}

render(<ProtonMain />);
