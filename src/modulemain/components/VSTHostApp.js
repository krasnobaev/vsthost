import React, { Component } from 'react';
import { Box, Button, Dialog, Tab, Text, TextInput } from 'proton-native';
var addon = require('../../../native');

const _initplug = '/Library/Audio/Plug-Ins/VST/Replika.vst/Contents/MacOS/Replika';

class VSTHostApp extends Component {
  constructor(props) {
    super(props);

    this.state = {
      curplug: _initplug,
      pluginfo: addon.vstpluginfo(_initplug),
    };
  }

  open() {
    const filename = Dialog('Open') || '';
    if (filename.match(/.vst\/Contents\/MacOS\//)) {
      this.setState({
        curplug: filename,
        pluginfo: addon.vstpluginfo(filename),
      });
    }
  }

  render() {
    return (
      <Box padded>
        <Button stretchy={false} onClick={() => this.open()}>Open VST</Button>
        <Text>{this.state.pluginfo}</Text>
        <Tab>
          <Box label="Tab1" padded>
            <TextInput />
          </Box>
          <Box label="Tab2">
            <TextInput multiline>{addon.listAudioDevices()}</TextInput>
          </Box>
        </Tab>
      </Box>
    );
  }
}

export default VSTHostApp
