// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { Test } from "forge-std/Test.sol";
import { Helper } from "./Helper.sol";
import { DVN } from "../src/DVN.sol";
import { DVNProxy } from "../src/DVNProxy.sol";
import { IAccessControl } from "@openzeppelin/contracts/access/IAccessControl.sol";
import { ERC1967Utils } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Utils.sol";
import { IERC1967 } from "@openzeppelin/contracts/interfaces/IERC1967.sol";

contract RoleTest is Helper, Test {
    function setUp() public {
        _dvn = new DVN();
        _proxy = new DVNProxy(address(_dvn));
        _dvnBehindProxy = DVN(address(_proxy));
    }

    function test_DVN_CANISTER_ROLE_SetCorrectly() public view {
        bytes32 role = _dvnBehindProxy.DVN_CANISTER_ROLE();
        assertEq(role, keccak256("DVN_CANISTER"));
    }

    function test_MESSAGE_LIB_ROLE_SetCorrectly() public view {
        bytes32 role = _dvnBehindProxy.MESSAGE_LIB_ROLE();
        assertEq(role, keccak256("MESSAGE_LIB"));
    }

    function test_grantRole_RevertIf_CalledByNonAdmin() public {
        address prankAddress = makeAddr("prank");
        address testAddress = makeAddr("address");
        bytes32 role = _dvnBehindProxy.DVN_CANISTER_ROLE();
        
        vm.prank(prankAddress);
        vm.expectRevert(DVN.Unauthorized.selector);
        _dvnBehindProxy.grantRole(role, testAddress);
    }

    function test_grantRole_RevertIf_CalledWithInvalidRole() public {
        address testAddress = makeAddr("address");
        bytes32 role = bytes32(0);
        
        vm.expectRevert(DVN.InvalidRole.selector);
        _dvnBehindProxy.grantRole(role, testAddress);
    }

    function test_grantRole_CorrectlyGrantsRole() public {
        address testAddress = makeAddr("address");
        bytes32 role = _dvnBehindProxy.DVN_CANISTER_ROLE();
        
        bool hasRole = _dvnBehindProxy.hasRole(role, testAddress);
        assertEq(hasRole, false);

        vm.expectEmit(address(_proxy));
        emit IAccessControl.RoleGranted(role, testAddress, address(this));
        _dvnBehindProxy.grantRole(role, testAddress);

        hasRole = _dvnBehindProxy.hasRole(role, testAddress);
        assertEq(hasRole, true);
    }

    function test_revokeRole_RevertIf_CalledByNonAdmin() public {
        address prankAddress = makeAddr("prank");
        address testAddress = makeAddr("address");
        bytes32 role = _dvnBehindProxy.DVN_CANISTER_ROLE();
        
        vm.prank(prankAddress);
        vm.expectRevert(DVN.Unauthorized.selector);
        _dvnBehindProxy.revokeRole(role, testAddress);
    }

    function test_revokeRole_RevertIf_CalledWithInvalidRole() public {
        address testAddress = makeAddr("address");
        bytes32 role = bytes32(0);
        
        vm.expectRevert(DVN.InvalidRole.selector);
        _dvnBehindProxy.revokeRole(role, testAddress);
    }

    function test_revokeRole_CorrectlyGrantsRole() public {
        address testAddress = makeAddr("address");
        bytes32 role = _dvnBehindProxy.DVN_CANISTER_ROLE();
        
        _dvnBehindProxy.grantRole(role, testAddress);
        bool hasRole = _dvnBehindProxy.hasRole(role, testAddress);
        assertEq(hasRole, true);

        vm.expectEmit(address(_proxy));
        emit IAccessControl.RoleRevoked(role, testAddress, address(this));
        _dvnBehindProxy.revokeRole(role, testAddress);

        hasRole = _dvnBehindProxy.hasRole(role, testAddress);
        assertEq(hasRole, false);
    }
}