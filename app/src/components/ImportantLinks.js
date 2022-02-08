import { Button, VStack } from "@chakra-ui/react";
import React from "react";
const data = require('../assets/data/links.json');

function Feature({ title, link }) {
    return (
        <Button w="50%" shadow='base' colorScheme="telegram">
            <a href={link} rel="noreferrer" target="_blank">
                {title}
            </a>
        </Button>
    )
}

function ImportantLinks() {
    return (
        <VStack alignSelf="center" minH="inherit" p='6' spacing="4" justifyContent="center">
            {data.map((item, index) => (
                <Feature
                    key={index}
                    title={item.name}
                    link={item.link}
                />
            ))}
        </VStack>
    )
}

export default ImportantLinks;