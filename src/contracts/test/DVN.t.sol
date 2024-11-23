// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { Test } from "forge-std/Test.sol";
import { DVN } from "../src/DVN.sol";
import { DVNProxy } from "../src/DVNProxy.sol";
import { Packet } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ISendLib.sol";
import { AddressCast } from "@layerzerolabs/lz-evm-protocol-v2/contracts/libs/AddressCast.sol";
import { PacketV1Codec } from "@layerzerolabs/lz-evm-protocol-v2/contracts/messagelib/libs/PacketV1Codec.sol";
import { ILayerZeroDVN } from "@layerzerolabs/lz-evm-messagelib-v2/contracts/uln/interfaces/ILayerZeroDVN.sol";
import { ERC1967Utils } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Utils.sol";
import { IERC1967 } from "@openzeppelin/contracts/interfaces/IERC1967.sol";
import { ILayerZeroEndpointV2, Origin } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";
import { IMessageLibManager } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/IMessageLibManager.sol";
import { IReceiveUlnE2 } from "@layerzerolabs/lz-evm-messagelib-v2/contracts/uln/interfaces/IReceiveUlnE2.sol";

abstract contract Helper {
    DVN _dvn;
    DVNProxy _proxy;
    DVN _dvnBehindProxy;

    function getSampleTask() internal view returns (ILayerZeroDVN.AssignJobParam memory) {
        Packet memory packet;
        packet.nonce = 0;
        packet.srcEid = 1;
        packet.sender = address(this);
        packet.dstEid = 100;
        packet.receiver = AddressCast.toBytes32(address(this));
        packet.guid = AddressCast.toBytes32(address(this));
        packet.message = "Hello World";

        ILayerZeroDVN.AssignJobParam memory task;
        task.dstEid = packet.dstEid;
        task.packetHeader = PacketV1Codec.encodePacketHeader(packet);
        task.payloadHash = keccak256(packet.message); // Taken from PacketV1Codec
        task.confirmations = 5;
        task.sender = packet.sender;

        return task;
    }

    function getSampleOrigin() internal view returns (Origin memory) {
        return Origin(1, AddressCast.toBytes32(address(this)), 0);
    }
}

contract DVNTest is Test, Helper {
    function setUp() public {
        _dvn = new DVN();
        _proxy = new DVNProxy(address(_dvn));
        _dvnBehindProxy = DVN(address(_proxy));
    }

    function test_admin_defaultAdminIsCreator() public view {
        address admin = _dvnBehindProxy.getAdmin();
        assertEq(admin, address(this));
    }

    function test_admin_RevertIf_UnauthorizedSetAttempt() public {
        address prankAddress = makeAddr("pranker");
        address testAddress = makeAddr("test");

        vm.prank(prankAddress);
        vm.expectRevert(DVN.Unauthorized.selector);
        _dvnBehindProxy.setAdmin(testAddress);
    }

    function test_admin_RevertIf_NewAddressIsZero() public {
        vm.expectRevert(abi.encodeWithSelector(ERC1967Utils.ERC1967InvalidAdmin.selector, address(0)));
        _dvnBehindProxy.setAdmin(address(0));
    }

    function test_admin_SuccessfulChange() public {
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

    function test_admin_NewAdminCanSet() public {
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

    function test_endpoint_RevertIf_UnauthorizedSetAttempt() public {
        address endpoint = makeAddr("endpoint");
        address pranker = makeAddr("prank");

        vm.prank(pranker);
        vm.expectRevert(DVN.Unauthorized.selector);
        _dvnBehindProxy.setEndpoint(endpoint);
    }

    function test_endpoint_AdminCanSet() public {
        address endpoint = makeAddr("endpoint");

        _dvnBehindProxy.setEndpoint(endpoint);
        address setEndpoint = _dvnBehindProxy.getEndpoint();
        assertEq(setEndpoint, endpoint);
    }
    
    function test_upgrade_RevertIf_UnauthorizedAttempt() public {
        address pranker = makeAddr("pranker");
        address newImplementation = makeAddr("newImplementation");

        vm.prank(pranker);
        vm.expectRevert(DVN.Unauthorized.selector);
        _dvnBehindProxy.upgradeToAndCall(newImplementation, "");
    }

    function test_upgrade_SuccessfullyUpgrades() public {
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

    function test_upgrade_AdminStaysIntactAfterUpgrade() public {
        address testAdmin = makeAddr("test");
        DVN newDVN = new DVN();

        _dvnBehindProxy.setAdmin(testAdmin);
        vm.prank(testAdmin);
        _dvnBehindProxy.upgradeToAndCall(address(newDVN), "");

        address admin = _dvnBehindProxy.getAdmin();
        assertEq(admin, testAdmin);
    }

    function test_assign_RevertIf_UnauthorizedAssignAttempt() public {
        address endpoint = makeAddr("endpoint");
        _dvnBehindProxy.setEndpoint(endpoint);

        vm.expectRevert(DVN.Unauthorized.selector);
        _dvnBehindProxy.assignJob(getSampleTask(), "");
    }

    function test_assign_SuccessfulAssign() public {
        address endpoint = makeAddr("endpoint");
        _dvnBehindProxy.setEndpoint(endpoint);

        ILayerZeroDVN.AssignJobParam memory task = getSampleTask();
        
        vm.prank(endpoint);
        vm.expectEmit(address(_dvnBehindProxy));
        emit DVN.TaskAssigned(task.dstEid, task.confirmations, task);
        uint256 fees = _dvnBehindProxy.assignJob(task, "");
        assertEq(fees, 0);
    }

    function test_verify_RevertIf_UnauthorizedVerifyAttempt() public {
        address pranker = makeAddr("pranker");
        ILayerZeroDVN.AssignJobParam memory task = getSampleTask();

        vm.prank(pranker);
        vm.expectRevert(DVN.Unauthorized.selector);
        _dvnBehindProxy.verify(task);
    }

    function test_verify_SuccessfulVerify() public {
        address endpoint = makeAddr("endpoint");
        address receiverLib = makeAddr("receiverLib");
        ILayerZeroDVN.AssignJobParam memory task = getSampleTask();

        _dvnBehindProxy.setEndpoint(endpoint);

        vm.mockCall(endpoint, abi.encodeWithSelector(IMessageLibManager.getReceiveLibrary.selector), abi.encode(receiverLib, false));
        vm.mockCall(receiverLib, abi.encodeWithSelector(IReceiveUlnE2.verify.selector), "");

        vm.expectCall(
            endpoint,
            abi.encodeWithSelector(
                IMessageLibManager.getReceiveLibrary.selector,
                address(this),
                task.dstEid
            )
        );

        vm.expectCall(
            receiverLib,
            abi.encodeWithSelector(
                IReceiveUlnE2.verify.selector,
                task.packetHeader,
                task.payloadHash,
                task.confirmations
            )
        );

        _dvnBehindProxy.verify(task);
    }

    function test_verified_Success() public {
        address endpoint = makeAddr("endpoint");
        ILayerZeroDVN.AssignJobParam memory task = getSampleTask();
        Origin memory origin = getSampleOrigin();

        _dvnBehindProxy.setEndpoint(endpoint);
        vm.mockCall(endpoint, abi.encodeWithSelector(ILayerZeroEndpointV2.verifiable.selector), abi.encode(true));

        vm.expectCall(
            endpoint,
            abi.encodeWithSelector(
                ILayerZeroEndpointV2.verifiable.selector,
                origin,
                address(this)
            )
        );

        bool verified = _dvnBehindProxy.verified(task);
        assertEq(verified, true);
    }

    function test_fees_ShouldBeZero() public view {
        uint256 fees = _dvnBehindProxy.getFee(0, 0, address(this), "");
        assertEq(fees, 0);
    }
}