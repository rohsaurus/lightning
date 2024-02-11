import React, { useState, useEffect } from 'react';
import { Box, Heading, Text, Button, Center } from "@chakra-ui/react";
import { fs } from '@tauri-apps/api';
import Search from './Search';
import Setup from './Setup';

function App() {
  const [firstVisit, setFirstVisit] = useState(true);

  useEffect(() => {
    fs.readTextFile('settings.json')
      .then(() => setFirstVisit(false))
      .catch(() => setFirstVisit(true));
  }, []);

  if (firstVisit) {
    return (
      <Center h="100vh" w="100vw" bg="gray.200">
        <Box p={5} shadow="md" borderWidth={2} borderRadius="md">
          <Heading as="h2" size="xl" mb={5}>
            Welcome to Lightning
          </Heading>
          <Text fontSize="xl" mb={5}>
            Lightning Fast Searches, at your fingertips.
          </Text>
          <Button colorScheme="teal" size="lg" onClick={() => setFirstVisit(false)}>
            Get Started
          </Button>
        </Box>
      </Center>
    );
  } else {
    return <Search />;
  }
}

export default App;