// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { ILayerZeroDVN } from "@layerzerolabs/lz-evm-messagelib-v2/contracts/uln/interfaces/ILayerZeroDVN.sol";
import { ILayerZeroEndpointV2, Origin } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";
import { PacketV1Codec } from "@layerzerolabs/lz-evm-protocol-v2/contracts/messagelib/libs/PacketV1Codec.sol";
import { IReceiveUlnE2 } from "@layerzerolabs/lz-evm-messagelib-v2/contracts/uln/interfaces/IReceiveUlnE2.sol";
import "hardhat/console.sol";

contract DVN is ILayerZeroDVN {
    using PacketV1Codec for bytes;

    address _endpoint;
    AssignJobParam[] test_Jobs;

    event JobAssigned(AssignJobParam params);

    constructor(address endpoint_) {
        _endpoint = endpoint_;
    }

    function assignJob(
        AssignJobParam calldata _param,
        bytes calldata /*_options*/
    ) external payable returns (uint256) {
        emit JobAssigned(_param);
        return 0;
    }

    function verify(AssignJobParam calldata param_) external {
        test_Jobs.push(param_); // only for testing
        bytes calldata packetHeader_ = param_.packetHeader;
        address receiver_ = _bytes32ToAddress(packetHeader_.receiver());

        (address receiveLibrary,)= ILayerZeroEndpointV2(_endpoint).getReceiveLibrary(receiver_, packetHeader_.dstEid());
        IReceiveUlnE2(receiveLibrary).verify(packetHeader_, param_.payloadHash, param_.confirmations);
    }

    function verified(AssignJobParam calldata param_) external view returns (bool) {
        bytes calldata packetHeader_ = param_.packetHeader;
        address receiver_ = _bytes32ToAddress(packetHeader_.receiver());

        Origin memory origin_ = Origin(packetHeader_.srcEid(), packetHeader_.sender(), packetHeader_.nonce());
        return ILayerZeroEndpointV2(_endpoint).verifiable(origin_, receiver_);
    }

    function test_verifiedJobsNum() external view returns (uint) {
        return test_Jobs.length;
    }
    
    function test_verified(uint index) external view returns (bool) {
        return this.verified(test_Jobs[index]);
    }

    function getFee(
        uint32 /*_dstEid*/,
        uint64 /*_confirmations*/,
        address /*_sender*/,
        bytes calldata /*_options*/
    ) external pure returns (uint256) {
        return 0;
    }

    function _bytes32ToAddress(bytes32 _b) internal pure returns (address) {
        return address(uint160(uint256(_b)));
    }
}