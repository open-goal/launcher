import React from 'react';
import { useColorMode, useColorModeValue, Button } from '@chakra-ui/react';

export const ColorModeSwitcher = props => {
  const text = useColorModeValue('dark', 'light');
  const { colorMode, toggleColorMode } = useColorMode();

  return (
    <Button
      size="md"
      aria-label={`Switch to ${text} mode`}
      colorScheme="telegram"
      onClick={toggleColorMode}
      {...props}
    >Toggle {colorMode === 'light' ? 'Dark Mode' : 'Light Mode'}</Button>
  );
};