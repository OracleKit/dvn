// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import { ERC1967Utils } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Utils.sol";

contract DVNProxy is ERC1967Proxy {
    constructor(address implementation) ERC1967Proxy(implementation, "") {
        ERC1967Utils.changeAdmin(msg.sender);
    }
}