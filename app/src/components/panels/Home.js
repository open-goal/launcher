import React from "react";
import { VStack, Button } from '@chakra-ui/react';
const { send } = window.api;

function handleLaunch() {
    send('launch');
}

function Home() {
    return (
        <VStack alignSelf="center" minH="inherit" p='6' spacing="4" justifyContent="center">
            <Button colorScheme="telegram" onClick={handleLaunch} w="50%" shadow='base'>Launch Game</Button>
        </VStack>
    );
}

export default Home;