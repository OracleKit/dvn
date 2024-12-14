// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { ILayerZeroDVN } from "@layerzerolabs/lz-evm-messagelib-v2/contracts/uln/interfaces/ILayerZeroDVN.sol";
import { ILayerZeroEndpointV2, Origin } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";
import { PacketV1Codec } from "@layerzerolabs/lz-evm-protocol-v2/contracts/messagelib/libs/PacketV1Codec.sol";
import { IReceiveUlnE2 } from "@layerzerolabs/lz-evm-messagelib-v2/contracts/uln/interfaces/IReceiveUlnE2.sol";
import { ERC1967Utils } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Utils.sol";
import { UUPSUpgradeable } from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import { AccessControl } from "@openzeppelin/contracts/access/AccessControl.sol";
import "hardhat/console.sol";

contract DVN is ILayerZeroDVN, UUPSUpgradeable, AccessControl {
    using PacketV1Codec for bytes;

    address _endpoint;

    event TaskAssigned(
        uint32 indexed dstEid,
        uint64 indexed numConfirmations,
        AssignJobParam task
    );

    error Unauthorized();
    error Unimplemented();

    modifier onlyAdmin() {
        if ( msg.sender != ERC1967Utils.getAdmin() ) {
            revert Unauthorized();
        }
        _;
    }

    modifier onlyEndpoint() {
        if ( msg.sender != _endpoint ) {
            revert Unauthorized();
        }
        _;
    }

    function setAdmin(address newAdmin) external onlyProxy onlyAdmin {
        ERC1967Utils.changeAdmin(newAdmin);
    }

    function getAdmin() external view onlyProxy returns (address) {
        return ERC1967Utils.getAdmin();
    }

    function setEndpoint(address endpoint_) external onlyProxy onlyAdmin {
        _endpoint = endpoint_;
    }

    function getEndpoint() external view onlyProxy returns (address) {
        return _endpoint;
    }

    function MESSAGE_LIB_ROLE() public view onlyProxy returns (bytes32) {
        return keccak256("MESSAGE_LIB");
    }

    function addMessageLib(address lib_) external onlyProxy onlyAdmin {
        _grantRole(MESSAGE_LIB_ROLE(), lib_);
    }

    function removeMessageLib(address lib_) external onlyProxy onlyAdmin {
        _revokeRole(MESSAGE_LIB_ROLE(), lib_);
    }

    function _bytes32ToAddress(bytes32 _b) internal pure returns (address) {
        return address(uint160(uint256(_b)));
    }

    function assignJob(
        AssignJobParam calldata task_,
        bytes calldata /*_options*/
    ) external payable onlyProxy onlyRole(MESSAGE_LIB_ROLE()) returns (uint256) {
        emit TaskAssigned(task_.dstEid, task_.confirmations, task_);
        return 0;
    }

    function verify(AssignJobParam calldata task_) external onlyProxy onlyAdmin {
        bytes calldata packetHeader_ = task_.packetHeader;
        address receiver_ = _bytes32ToAddress(packetHeader_.receiver());

        (address receiveLibrary,)= ILayerZeroEndpointV2(_endpoint).getReceiveLibrary(receiver_, packetHeader_.dstEid());
        IReceiveUlnE2(receiveLibrary).verify(packetHeader_, task_.payloadHash, task_.confirmations);
    }

    function verified(AssignJobParam calldata task_) external view onlyProxy returns (bool) {
        bytes calldata packetHeader_ = task_.packetHeader;
        address receiver_ = _bytes32ToAddress(packetHeader_.receiver());

        Origin memory origin_ = Origin(packetHeader_.srcEid(), packetHeader_.sender(), packetHeader_.nonce());
        return ILayerZeroEndpointV2(_endpoint).verifiable(origin_, receiver_);
    }

    function getFee(
        uint32 /*_dstEid*/,
        uint64 /*_confirmations*/,
        address /*_sender*/,
        bytes calldata /*_options*/
    ) external view onlyProxy returns (uint256) {
        return 0;
    }

    /** AccessControl overrides */

    function grantRole(bytes32, address) public pure override {
        revert Unimplemented();
    }

    function revokeRole(bytes32, address) public pure override {
        revert Unimplemented();
    }

    /** UUPSUpgradeable Functions */

    // Authorizes upgradeToAndCall() from UUPSUpgradeable
    function _authorizeUpgrade(address /*newImplementation*/) internal view override onlyAdmin {}
}