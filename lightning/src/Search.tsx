import { VStack, Text, Heading, Box, useColorModeValue, Card, CardHeader, CardBody, CardFooter, SimpleGrid, Button, Center, Spacer } from "@chakra-ui/react";
import { invoke } from "@tauri-apps/api";
import ReactDOM from "react-dom";
import { InstantSearch, SearchBox, Hits, Highlight, useHits } from 'react-instantsearch';
import TypesenseInstantSearchAdapter from "typesense-instantsearch-adapter";

const typesenseInstantsearchAdapter = new TypesenseInstantSearchAdapter({
  server: {
    apiKey: "GkxVS70yMTFS7RJ9BEz6sjpPG6Ij7kdqTx7tGLIZ0AM1fJ4X",
    nodes: [
      {
        host: "localhost",
        port: 8108,
        protocol: "http",
      },
    ],
    cacheSearchResultsForSeconds: 2 * 60,
  },
  additionalSearchParameters: {
    query_by: "file_name",
  },
});
const searchClient = typesenseInstantsearchAdapter.searchClient;

function HitsComponent() {
  const { hits } = useHits();

  return hits.map(hit => <Hit hit={hit} key={hit.objectID} />);
}


function Hit(props: any) {
  const cardBackground = useColorModeValue('white', 'gray.800');
  const cardFooterBackground = useColorModeValue('gray.100', 'gray.700');

  return (
      <Card bg={cardBackground} boxShadow="base" borderRadius="md" overflow="auto" p={4} m={4} w={["100%", "80%", "60%", "40%"]} h={["240px", "240px", "260px", "290px"]}>
        <CardHeader p={4}>
          <Heading size='md'>{props.hit.file_name}</Heading>
        </CardHeader>
        <CardBody p={4}>
          <Text>Creation Date: {props.hit.data_created}.</Text>
          <Text>Modified Date: {props.hit.date_modified}.</Text>
          <Text>Location: {props.hit.file_location}.</Text>
          <Text>Type: {props.hit.file_type}.</Text>
        </CardBody>
        <CardFooter bg={cardFooterBackground} p={3} justifyContent="space-between">
        <Spacer />
        <Button onClick={() => openFile(props.hit.file_location)}>Open File</Button>
        <Spacer />
        <Button onClick={() => openFileLocation(props.hit.file_location)}>Open Location</Button>
        <Spacer />
        </CardFooter>
      </Card>
  );
  }
function openFile(fileLocation: string) {
  console.log(fileLocation)
  // calling tauri command to open file in default application for the file type 
  invoke("open_file", { path: fileLocation });
}

function Search() {
  return (
    <InstantSearch searchClient={searchClient} indexName="files">
      <Box position="sticky" top={0} zIndex={1} bg="white" p={4}>
        <Center>
        <SearchBox />
        </Center>
      </Box>
        <VStack spacing={5} p={5}>
        <HitsComponent />
      </VStack>
    </InstantSearch>
  );
}

export default Search;