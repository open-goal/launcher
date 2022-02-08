import React from "react";
import { Flex, Tabs, TabList, TabPanels, TabPanel, Tab } from "@chakra-ui/react";
import { Home, GettingStarted, ImportantLinks, Settings } from './panels/index.js';

function Landing() {
    return (
        <Flex direction="column" mx={4}>
            <Tabs isFitted isLazy>
                <TabList>
                    <Tab>Home</Tab>
                    <Tab>Getting Started</Tab>
                    <Tab>Links</Tab>
                    <Tab>Settings</Tab>
                </TabList>
                <TabPanels>
                    <TabPanel minH="70vh">
                        <Home />
                    </TabPanel>
                    <TabPanel minH="70vh">
                        <GettingStarted />
                    </TabPanel>
                    <TabPanel minH="70vh">
                        <ImportantLinks />
                    </TabPanel>
                    <TabPanel minH="70vh">
                        <Settings />
                    </TabPanel>
                </TabPanels>
            </Tabs>
        </Flex>
    );
}

export default Landing;