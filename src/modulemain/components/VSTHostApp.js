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
      pluginfo: vsthost.vstpluginfo(0),
      beepIsStopped: false,
    };
  }

  open() {
    const filename = Dialog('Open') || '';
    if (filename.match(/.vst\/Contents\/MacOS\//)) {
      vsthost.loadvstplugin(filename);

      this.setState({
        curplug: filename,
        pluginfo: vsthost.vstpluginfo(0),
      });
    }
  }

  refresh(ind) {
    this.setState({
      ...this.state,
      pluginfo: vsthost.vstpluginfo(ind),
    });
  }

  beep() {
    addon.beep(!this.state.beepIsStopped);

    this.setState({
      ...this.state,
      beepIsStopped: !this.state.beepIsStopped,
    });
  }

  render() {
    return (
      <Box padded>
      <Button stretchy={false} onClick={() => this.open()}>Open VST</Button>
      <Button stretchy={false} onClick={() => this.beep()}>Beep</Button>
      <Button stretchy={false} onClick={() => this.refresh(0)}>Refresh 0</Button>
      <Button stretchy={false} onClick={() => this.refresh(1)}>Refresh 1</Button>
        <Text>{this.state.pluginfo}</Text>
        <Tab>
          <Box label="Tab1" padded>
            <TextInput />
          </Box>
          <Box label="Tab2">
            <TextInput multiline>{addon.list_audio_devices()}</TextInput>
          </Box>
        </Tab>
      </Box>
    );
  }
}

export default VSTHostApp
