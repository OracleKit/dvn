// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { Test } from "forge-std/Test.sol";
import { Helper } from "./Helper.sol";
import { DVN } from "../src/DVN.sol";
import { DVNProxy } from "../src/DVNProxy.sol";
import { IAccessControl } from "@openzeppelin/contracts/access/IAccessControl.sol";
import { ERC1967Utils } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Utils.sol";
import { IERC1967 } from "@openzeppelin/contracts/interfaces/IERC1967.sol";

contract EndpointTest is Helper, Test {
    function setUp() public {
        _dvn = new DVN();
        _proxy = new DVNProxy(address(_dvn));
        _dvnBehindProxy = DVN(address(_proxy));
    }

    function test_setEndpoint_RevertIf_CalledByNonAdmin() public {
        address endpoint = makeAddr("endpoint");
        address pranker = makeAddr("prank");

        vm.prank(pranker);
        vm.expectRevert(DVN.Unauthorized.selector);
        _dvnBehindProxy.setEndpoint(endpoint);
    }

    function test_setEndpoint_Success() public {
        address endpoint = makeAddr("endpoint");

        _dvnBehindProxy.setEndpoint(endpoint);
        address setEndpoint = _dvnBehindProxy.endpoint();
        assertEq(setEndpoint, endpoint);
    }
}