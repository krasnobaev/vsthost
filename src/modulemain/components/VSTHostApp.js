import React, { Component } from 'react';
import { Box, Button, Dialog, Tab, Text, TextInput } from 'proton-native';
var addon = require('../../../native');
let vsthost = addon.get_vsthost_instance();

const _initplug = '/Library/Audio/Plug-Ins/VST/Replika.vst/Contents/MacOS/Replika';

class VSTHostApp extends Component {
  constructor(props) {
    super(props);

    vsthost.loadvstplugin(_initplug);

    this.state = {
      curplug: _initplug,
      plugins: new Array({
        index: 0,
        filename: _initplug,
        info: vsthost.vstpluginfo(0),
      }),
      pluginfo: vsthost.vstpluginfo(0),
      beepIsStopped: false,
    };
  }

  open() {
    const filename = Dialog('Open') || '';
    if (filename.match(/.vst\/Contents\/MacOS\//)) {
      const index = vsthost.loadvstplugin(filename) - 1;

      this.setState({
        curplug: filename,
        pluginfo: vsthost.vstpluginfo(index),
        plugins: [
          ...this.state.plugins, {
            index,
            filename,
            info: vsthost.vstpluginfo(index),
          }
        ]
      });
    }
  }

  beep() {
    addon.beep(!this.state.beepIsStopped);

    this.setState({
      ...this.state,
      beepIsStopped: !this.state.beepIsStopped,
    });
  }

  PlugList() {
    const plugins = this.state.plugins.map((plug, i) =>
      <Box label={`plug${i}`} padded>
        <TextInput multiline>{`filename: ${plug.filename}\n${plug.info}`}</TextInput>
      </Box>
    );

    return (
      <Tab>{plugins}</Tab>
    );
  }

  render() {
    return (
      <Box padded>
        <Button stretchy={false} onClick={() => this.open()}>Open VST</Button>
        <Button stretchy={false} onClick={() => this.beep()}>Beep</Button>

        {this.PlugList()}

        <Tab>
          <Box label="Audio Devices">
            <TextInput multiline>{addon.list_audio_devices()}</TextInput>
          </Box>
        </Tab>
      </Box>
    );
  }
}

export default VSTHostApp
