import { Button, Text, Progress, VStack, Divider } from "@chakra-ui/react";
import React, { useEffect, useState } from "react";
const { receive, send } = window.api;

function GettingStarted() {
    const [status, setStatus] = useState('Awaiting Jak ISO File');

    function handleClick() {
        send('getISO');
    }

    useEffect(() => {
        receive('status', newStatus => {
            console.log(newStatus);
            setStatus(newStatus);
        });
    }, [])


    return (
        <VStack alignSelf="center" minH="inherit" p='6' spacing="4" justifyContent="center">
            {status === 'Awaiting Jak ISO File' ?
                <>
                    <Text fontWeight="bold">Please select your Jak and Daxter ISO below</Text>
                    <Button colorScheme="telegram" onClick={handleClick} w="50%" shadow='base'>Open your ISO File</Button>
                    <Divider w="50%" />
                </> : null
            }
            <Progress size='lg' w="50%" colorScheme="green" rounded="md" isIndeterminate />
            {status ? <Text fontWeight="extrabold">Status: {status}</Text> : null}
        </VStack>
    );
}

export default GettingStarted;