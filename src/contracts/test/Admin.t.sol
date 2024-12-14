// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { Test } from "forge-std/Test.sol";
import { Helper } from "./Helper.sol";
import { DVN } from "../src/DVN.sol";
import { DVNProxy } from "../src/DVNProxy.sol";
import { IAccessControl } from "@openzeppelin/contracts/access/IAccessControl.sol";
import { ERC1967Utils } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Utils.sol";
import { IERC1967 } from "@openzeppelin/contracts/interfaces/IERC1967.sol";

contract AdminTest is Helper, Test {
    function setUp() public {
        _dvn = new DVN();
        _proxy = new DVNProxy(address(_dvn));
        _dvnBehindProxy = DVN(address(_proxy));
    }

    function test_getAdmin_defaultAdminIsCreator() public view {
        address admin = _dvnBehindProxy.getAdmin();
        assertEq(admin, address(this));
    }

    function test_setAdmin_RevertIf_CalledByNonAdmin() public {
        address prankAddress = makeAddr("pranker");
        address testAddress = makeAddr("test");

        vm.prank(prankAddress);
        vm.expectRevert(DVN.Unauthorized.selector);
        _dvnBehindProxy.setAdmin(testAddress);
    }

    function test_setAdmin_RevertIf_NewAddressIsZero() public {
        vm.expectRevert(abi.encodeWithSelector(ERC1967Utils.ERC1967InvalidAdmin.selector, address(0)));
        _dvnBehindProxy.setAdmin(address(0));
    }

    function test_setAdmin_SuccessfulChange() public {
        address testAddress = makeAddr("test");
        
        vm.expectEmit(address(_dvnBehindProxy));
        emit IERC1967.AdminChanged(address(this), testAddress);
        _dvnBehindProxy.setAdmin(testAddress);

        address admin = _dvnBehindProxy.getAdmin();
        assertEq(admin, testAddress);
    }

    function test_admin_RevertIf_OriginalAdminCanSetAfterChange() public {
        address testAddressA = makeAddr("testA");
        address testAddressB = makeAddr("testB");
        
        vm.expectEmit(address(_dvnBehindProxy));
        emit IERC1967.AdminChanged(address(this), testAddressA);
        _dvnBehindProxy.setAdmin(testAddressA);

        vm.expectRevert(DVN.Unauthorized.selector);
        _dvnBehindProxy.setAdmin(testAddressB);
    }

    function test_setAdmin_NewAdminCanSet() public {
        address testAddressA = makeAddr("testA");
        address testAddressB = makeAddr("testB");
        
        vm.expectEmit(address(_dvnBehindProxy));
        emit IERC1967.AdminChanged(address(this), testAddressA);
        _dvnBehindProxy.setAdmin(testAddressA);

        vm.prank(testAddressA);
        vm.expectEmit(address(_dvnBehindProxy));
        emit IERC1967.AdminChanged(testAddressA, testAddressB);
        _dvnBehindProxy.setAdmin(testAddressB);

        address admin = _dvnBehindProxy.getAdmin();
        assertEq(admin, testAddressB);
    }
}