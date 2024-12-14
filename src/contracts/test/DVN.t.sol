// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { Test } from "forge-std/Test.sol";
import { DVN } from "../src/DVN.sol";
import { DVNProxy } from "../src/DVNProxy.sol";
import { Packet } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ISendLib.sol";
import { PacketV1Codec } from "@layerzerolabs/lz-evm-protocol-v2/contracts/messagelib/libs/PacketV1Codec.sol";
import { ILayerZeroDVN } from "@layerzerolabs/lz-evm-messagelib-v2/contracts/uln/interfaces/ILayerZeroDVN.sol";
import { ILayerZeroEndpointV2, Origin } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";
import { IMessageLibManager } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/IMessageLibManager.sol";
import { IReceiveUlnE2 } from "@layerzerolabs/lz-evm-messagelib-v2/contracts/uln/interfaces/IReceiveUlnE2.sol";
import { Helper } from "./Helper.sol";
import { IAccessControl } from "@openzeppelin/contracts/access/IAccessControl.sol";

contract DVNTest is Test, Helper {
    function setUp() public {
        _dvn = new DVN();
        _proxy = new DVNProxy(address(_dvn));
        _dvnBehindProxy = DVN(address(_proxy));
    }
    
    function test_assignJob_RevertIf_CalledByAdmin() public {
        address endpoint = makeAddr("endpoint");
        address messageLib = makeAddr("messageLib");
        bytes32 messageLibRole = _dvnBehindProxy.MESSAGE_LIB_ROLE();

        _dvnBehindProxy.setEndpoint(endpoint);
        _dvnBehindProxy.addMessageLib(messageLib);

        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector,
                address(this),
                messageLibRole
            )
        );
        _dvnBehindProxy.assignJob(getSampleTask(), "");
    }

    function test_assignJob_RevertIf_CalledByEndpoint() public {
        address endpoint = makeAddr("endpoint");
        address messageLib = makeAddr("messageLib");
        bytes32 messageLibRole = _dvnBehindProxy.MESSAGE_LIB_ROLE();
        _dvnBehindProxy.setEndpoint(endpoint);
        _dvnBehindProxy.addMessageLib(messageLib);

        vm.prank(endpoint);
        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector,
                endpoint,
                messageLibRole
            )
        );
        _dvnBehindProxy.assignJob(getSampleTask(), "");
    }

    function test_assignJob_CalledByMessageLib() public {
        address endpoint = makeAddr("endpoint");
        address messageLib = makeAddr("messageLib");
        _dvnBehindProxy.setEndpoint(endpoint);
        _dvnBehindProxy.addMessageLib(messageLib);

        ILayerZeroDVN.AssignJobParam memory task = getSampleTask();
        
        vm.prank(messageLib);
        vm.expectEmit(address(_dvnBehindProxy));
        emit DVN.TaskAssigned(task.dstEid, task.confirmations, task);
        uint256 fees = _dvnBehindProxy.assignJob(task, "");
        assertEq(fees, 0);
    }

    // function test_verify_RevertIf_UnauthorizedVerifyAttempt() public {
    //     address pranker = makeAddr("pranker");
    //     ILayerZeroDVN.AssignJobParam memory task = getSampleTask();

    //     vm.prank(pranker);
    //     vm.expectRevert(DVN.Unauthorized.selector);
    //     _dvnBehindProxy.verify(task);
    // }

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