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
import { ISendLib } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ISendLib.sol";
import { ILayerZeroPriceFeed } from "@layerzerolabs/lz-evm-messagelib-v2/contracts/interfaces/ILayerZeroPriceFeed.sol";

contract DVNTest is Test, Helper {
    function setUp() public {
        _dvn = new DVN();
        _proxy = new DVNProxy(address(_dvn));
        _dvnBehindProxy = DVN(address(_proxy));
        _priceFeed = makeAddr("priceFeed");

        ILayerZeroDVN.AssignJobParam memory task = getSampleTask();
        _dvnBehindProxy.setPriceConfig(DVN.PriceConfig(task.dstEid, 2000, 0, 100, 0));
        _dvnBehindProxy.setPriceFeed(_priceFeed);

        vm.mockCall(_priceFeed, abi.encodeWithSelector(ILayerZeroPriceFeed.estimateFeeByEid.selector), abi.encode(1, 1, 1, 1));
        vm.mockCall(_priceFeed, abi.encodeWithSelector(ILayerZeroPriceFeed.getPrice.selector), abi.encode(ILayerZeroPriceFeed.Price(1, 1, 1)));
    }
    
    function test_assignJob_RevertIf_NotCalledByMessageLib() public {
        address messageLib = makeAddr("messageLib");
        address endpoint = makeAddr("endpoint");
        bytes32 messageLibRole = _dvnBehindProxy.MESSAGE_LIB_ROLE();
        _dvnBehindProxy.setEndpoint(endpoint);
        _dvnBehindProxy.grantRole(messageLibRole, messageLib);

        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector,
                address(this),
                messageLibRole
            )
        );
        _dvnBehindProxy.assignJob(getSampleTask(), "");
    }

    function test_assignJob_Success() public {
        address endpoint = makeAddr("endpoint");
        address messageLib = makeAddr("messageLib");
        _dvnBehindProxy.setEndpoint(endpoint);
        _dvnBehindProxy.grantRole(_dvnBehindProxy.MESSAGE_LIB_ROLE(), messageLib);

        ILayerZeroDVN.AssignJobParam memory task = getSampleTask();
        uint256 predictedFees = _dvnBehindProxy.getFee(task.dstEid, task.confirmations, task.sender, "");
        
        vm.prank(messageLib);
        vm.expectEmit(address(_dvnBehindProxy));
        emit DVN.TaskAssigned(task.dstEid, task.confirmations, 1, task);
        uint256 fees = _dvnBehindProxy.assignJob(task, "");
        assertEq(fees, predictedFees);

        uint256 collectedFees = _dvnBehindProxy.feeCollected();
        assertEq(collectedFees, predictedFees);

        vm.mockCall(messageLib, abi.encodeWithSelector(ISendLib.withdrawFee.selector), "");
        vm.expectCall(messageLib, abi.encodeWithSelector(ISendLib.withdrawFee.selector, address(this), predictedFees));
        _dvnBehindProxy.withdrawFee(messageLib, address(this), predictedFees);
    }

    function test_verify_RevertIf_NotCalledByDvn() public {
        address dvnCanister = makeAddr("dvnCanister");
        address endpoint = makeAddr("endpoint");
        bytes32 dvnCanisterRole = _dvnBehindProxy.DVN_CANISTER_ROLE();
        _dvnBehindProxy.setEndpoint(endpoint);
        _dvnBehindProxy.grantRole(dvnCanisterRole, dvnCanister);

        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector,
                address(this),
                dvnCanisterRole
            )
        );
        _dvnBehindProxy.verify(getSampleTask());
    }

    function test_verify_SuccessfulVerify() public {
        address endpoint = makeAddr("endpoint");
        address dvnCanister = makeAddr("dvnCanister");
        address receiverLib = makeAddr("receiverLib");
        ILayerZeroDVN.AssignJobParam memory task = getSampleTask();
        bytes32 dvnCanisterRole = _dvnBehindProxy.DVN_CANISTER_ROLE();

        _dvnBehindProxy.setEndpoint(endpoint);
        _dvnBehindProxy.grantRole(dvnCanisterRole, dvnCanister);

        vm.prank(dvnCanister);

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
}