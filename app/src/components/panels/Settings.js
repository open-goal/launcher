import React from "react";
import { Button, VStack } from '@chakra-ui/react';
import { ColorModeSwitcher } from '../../ColorModeSwitcher';
const { invoke } = window.api;

async function handleCheckUpdates() {
    try {
        let response = await invoke('checkUpdates');
        console.log(response);
    } catch (err) {
        console.log(err);
    }
}

function Settings() {
    return (
        <VStack alignSelf="center" minH="inherit" p='6' spacing="4" justifyContent="center">
            <ColorModeSwitcher as={Button} w="50%" shadow='base' />
            <Button onClick={handleCheckUpdates} w="50%" shadow='base' colorScheme="telegram">Check for Updates</Button>
            {/* <Button onClick={decompile} w="50%" shadow='base' colorScheme="telegram">Decompile Game</Button> */}
            {/* <Button onClick={handleBuildGame} w="50%" shadow='base' colorScheme="telegram">Build Game</Button> */}
        </VStack >
    );
}

export default Settings;