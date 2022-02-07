import React from "react";
import { Heading, Img } from "@chakra-ui/react";
const logo = require('../assets/images/header-logo.png');

function Header() {
    return (
        <Heading py={4}>
            <Img src={logo} width="30vw" m="0 auto" draggable="false" />
        </Heading>
    );
}

export default Header;