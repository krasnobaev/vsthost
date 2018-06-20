import React from 'react';
import { Box, Button, Text } from 'proton-native';
var addon = require('../../../native');

const VSTHostApp = () => (
  <Box padded>
    <Text stretchy={false}>{addon.hello()}</Text>
    <Text stretchy={false}>{addon.vstpluginfo()}</Text>
    <Button stretchy={false}>Toggle</Button>
  </Box>
)

export default VSTHostApp
