import React, { Component } from 'react';
import { Box, Button, Dialog, Text } from 'proton-native';
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
        <Text stretchy={false}>{this.state.pluginfo}</Text>
      </Box>
    );
  }
}

export default VSTHostApp
