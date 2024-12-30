// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { DVN } from "../src/DVN.sol";
import { DVNProxy } from "../src/DVNProxy.sol";
import { ILayerZeroDVN } from "@layerzerolabs/lz-evm-messagelib-v2/contracts/uln/interfaces/ILayerZeroDVN.sol";
import { Packet } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ISendLib.sol";
import { AddressCast } from "@layerzerolabs/lz-evm-protocol-v2/contracts/libs/AddressCast.sol";
import { PacketV1Codec } from "@layerzerolabs/lz-evm-protocol-v2/contracts/messagelib/libs/PacketV1Codec.sol";
import { Origin } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";

abstract contract Helper {
    DVN _dvn;
    DVNProxy _proxy;
    DVN _dvnBehindProxy;
    address _priceFeed;

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