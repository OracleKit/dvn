// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { Test } from "forge-std/Test.sol";
import { Helper } from "./Helper.sol";
import { DVN } from "../src/DVN.sol";
import { DVNProxy } from "../src/DVNProxy.sol";
import { IAccessControl } from "@openzeppelin/contracts/access/IAccessControl.sol";
import { ERC1967Utils } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Utils.sol";
import { IERC1967 } from "@openzeppelin/contracts/interfaces/IERC1967.sol";

contract MessageLibTest is Helper, Test {
    function setUp() public {
        _dvn = new DVN();
        _proxy = new DVNProxy(address(_dvn));
        _dvnBehindProxy = DVN(address(_proxy));
    }

    function test_grantRole_RevertIf_Called() public {
        address testAddress = makeAddr("address");
        bytes32 messageLibRole = _dvnBehindProxy.MESSAGE_LIB_ROLE();

        vm.expectRevert(DVN.Unimplemented.selector);
        _dvnBehindProxy.grantRole(messageLibRole, testAddress);
    }

    function test_revokeRole_RevertIf_Called() public {
        address testAddress = makeAddr("address");
        bytes32 messageLibRole = _dvnBehindProxy.MESSAGE_LIB_ROLE();

        vm.expectRevert(DVN.Unimplemented.selector);
        _dvnBehindProxy.revokeRole(messageLibRole, testAddress);
    }

    function test_addMessageLib_RevertIf_CalledByNonAdmin() public {
        address prankAddress = makeAddr("prank");
        address testAddress = makeAddr("address");
        
        vm.prank(prankAddress);
        vm.expectRevert(DVN.Unauthorized.selector);
        _dvnBehindProxy.addMessageLib(testAddress);
    }

    function test_addMessageLib_Success() public {
        address messageLib = makeAddr("messagelib");
        bytes32 messageLibRole = _dvnBehindProxy.MESSAGE_LIB_ROLE();

        vm.expectEmit(address(_proxy));
        emit IAccessControl.RoleGranted(messageLibRole, messageLib, address(this));
        _dvnBehindProxy.addMessageLib(messageLib);

        bool roleExists = _dvnBehindProxy.hasRole(messageLibRole, messageLib);
        assertEq(roleExists, true);
    }

    function test_removeMessageLib_RevertIf_CalledByNonAdmin() public {
        address prankAddress = makeAddr("prank");
        address testAddress = makeAddr("address");
        
        vm.prank(prankAddress);
        vm.expectRevert(DVN.Unauthorized.selector);
        _dvnBehindProxy.removeMessageLib(testAddress);
    }

    function test_removeMessageLib_Success() public {
        address messageLib = makeAddr("messagelib");
        bytes32 messageLibRole = _dvnBehindProxy.MESSAGE_LIB_ROLE();
        _dvnBehindProxy.addMessageLib(messageLib);

        vm.expectEmit(address(_proxy));
        emit IAccessControl.RoleRevoked(messageLibRole, messageLib, address(this));
        _dvnBehindProxy.removeMessageLib(messageLib);

        bool roleExists = _dvnBehindProxy.hasRole(messageLibRole, messageLib);
        assertEq(roleExists, false);
    }
}