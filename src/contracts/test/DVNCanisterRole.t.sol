// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { Test } from "forge-std/Test.sol";
import { Helper } from "./Helper.sol";
import { DVN } from "../src/DVN.sol";
import { DVNProxy } from "../src/DVNProxy.sol";
import { IAccessControl } from "@openzeppelin/contracts/access/IAccessControl.sol";
import { ERC1967Utils } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Utils.sol";
import { IERC1967 } from "@openzeppelin/contracts/interfaces/IERC1967.sol";

contract DVNCanisterRoleTest is Helper, Test {
    function setUp() public {
        _dvn = new DVN();
        _proxy = new DVNProxy(address(_dvn));
        _dvnBehindProxy = DVN(address(_proxy));
    }

    function test_grantRole_RevertIf_Called() public {
        address testAddress = makeAddr("address");
        bytes32 dvnCanisterRole = _dvnBehindProxy.DVN_CANISTER_ROLE();

        vm.expectRevert(DVN.Unimplemented.selector);
        _dvnBehindProxy.grantRole(dvnCanisterRole, testAddress);
    }

    function test_revokeRole_RevertIf_Called() public {
        address testAddress = makeAddr("address");
        bytes32 dvnCanisterRole = _dvnBehindProxy.DVN_CANISTER_ROLE();

        vm.expectRevert(DVN.Unimplemented.selector);
        _dvnBehindProxy.revokeRole(dvnCanisterRole, testAddress);
    }

    function test_DVN_CANISTER_ROLE_SetCorrectly() public view {
        bytes32 dvnCanisterRole = _dvnBehindProxy.DVN_CANISTER_ROLE();
        assertEq(dvnCanisterRole, keccak256("DVN_CANISTER"));
    }

    function test_addDvnCanister_RevertIf_CalledByNonAdmin() public {
        address prankAddress = makeAddr("prank");
        address testAddress = makeAddr("address");
        
        vm.prank(prankAddress);
        vm.expectRevert(DVN.Unauthorized.selector);
        _dvnBehindProxy.addDvnCanister(testAddress);
    }

    function test_addDvnCanister_Success() public {
        address dvnCanister = makeAddr("dvncanister");
        bytes32 dvnCanisterRole = _dvnBehindProxy.DVN_CANISTER_ROLE();

        vm.expectEmit(address(_proxy));
        emit IAccessControl.RoleGranted(dvnCanisterRole, dvnCanister, address(this));
        _dvnBehindProxy.addDvnCanister(dvnCanister);

        bool roleExists = _dvnBehindProxy.hasRole(dvnCanisterRole, dvnCanister);
        assertEq(roleExists, true);
    }

    function test_removeDvnCanister_RevertIf_CalledByNonAdmin() public {
        address prankAddress = makeAddr("prank");
        address testAddress = makeAddr("address");
        
        vm.prank(prankAddress);
        vm.expectRevert(DVN.Unauthorized.selector);
        _dvnBehindProxy.removeDvnCanister(testAddress);
    }

    function test_removeDvnCanister_Success() public {
        address dvnCanister = makeAddr("dvncanister");
        bytes32 dvnCanisterRole = _dvnBehindProxy.DVN_CANISTER_ROLE();
        _dvnBehindProxy.addDvnCanister(dvnCanister);

        vm.expectEmit(address(_proxy));
        emit IAccessControl.RoleRevoked(dvnCanisterRole, dvnCanister, address(this));
        _dvnBehindProxy.removeDvnCanister(dvnCanister);

        bool roleExists = _dvnBehindProxy.hasRole(dvnCanisterRole, dvnCanister);
        assertEq(roleExists, false);
    }
}