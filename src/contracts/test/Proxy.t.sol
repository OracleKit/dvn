// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { Test } from "forge-std/Test.sol";
import { DVN } from "../src/DVN.sol";
import { DVNProxy } from "../src/DVNProxy.sol";
import { Helper } from "./Helper.sol";
import { AddressCast } from "@layerzerolabs/lz-evm-protocol-v2/contracts/libs/AddressCast.sol";

contract ProxyTest is Helper, Test {
    function setUp() public {
        _dvn = new DVN();
        _proxy = new DVNProxy(address(_dvn));
        _dvnBehindProxy = DVN(address(_proxy));
    }
    
    function test_upgradeToAndCall_RevertIf_CalledByNonAdmin() public {
        address pranker = makeAddr("pranker");
        address newImplementation = makeAddr("newImplementation");

        vm.prank(pranker);
        vm.expectRevert(DVN.Unauthorized.selector);
        _dvnBehindProxy.upgradeToAndCall(newImplementation, "");
    }

    function test_upgradeToAndCall_Success() public {
        DVN newDVN = new DVN();

        address implementation = AddressCast.toAddress(
            vm.load(
                address(_dvnBehindProxy),
                _dvn.proxiableUUID()
            )
        );
        assertEq(implementation, address(_dvn));

        _dvnBehindProxy.upgradeToAndCall(address(newDVN), "");

        implementation = AddressCast.toAddress(
            vm.load(
                address(_dvnBehindProxy),
                _dvn.proxiableUUID()
            )
        );
        assertEq(implementation, address(newDVN));
    }

    function test_upgradeToAndCall_AdminStaysIntactAfterUpgrade() public {
        address testAdmin = makeAddr("test");
        DVN newDVN = new DVN();

        _dvnBehindProxy.setAdmin(testAdmin);
        vm.prank(testAdmin);
        _dvnBehindProxy.upgradeToAndCall(address(newDVN), "");

        address admin = _dvnBehindProxy.getAdmin();
        assertEq(admin, testAdmin);
    }

}