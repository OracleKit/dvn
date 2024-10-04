// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {OAppSender, MessagingFee } from "@layerzerolabs/oapp-evm/contracts/oapp/OAppSender.sol";
import { OptionsBuilder } from "@layerzerolabs/oapp-evm/contracts/oapp/libs/OptionsBuilder.sol";
import { OAppCore } from "@layerzerolabs/oapp-evm/contracts/oapp/OAppCore.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { SetConfigParam } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/IMessageLibManager.sol";
import { UlnConfig } from "@layerzerolabs/lz-evm-messagelib-v2/contracts/uln/UlnBase.sol";

contract MockOAppSender is OAppSender {
    using OptionsBuilder for bytes;
    bytes _options = OptionsBuilder.newOptions().addExecutorLzReceiveOption(50000, 0);
    bytes _ulnConfig;

    constructor(
        address endpoint_,
        address dvn_
    ) OAppCore(endpoint_, msg.sender) Ownable(msg.sender) {
        address[] memory requiredDVNs_ = new address[](1);
        requiredDVNs_[0] = dvn_;

        UlnConfig memory config_ = UlnConfig({
            confirmations: 0,
            requiredDVNCount: 1,
            optionalDVNCount: 0,
            optionalDVNThreshold: 0,
            requiredDVNs: requiredDVNs_,
            optionalDVNs: new address[](0)
        });

        _ulnConfig = abi.encode(config_);
    }

    function initPeer(uint32 peerEid_, address peerAddress_) external {
        address sendLib_ = endpoint.getSendLibrary(address(this), peerEid_);
        bytes32 peerBytes_ = bytes32(uint256(uint160(peerAddress_)));
        
        SetConfigParam[] memory params = new SetConfigParam[](1);
        params[0] = SetConfigParam(peerEid_, 2, _ulnConfig);

        endpoint.setConfig(address(this), sendLib_, params);
        setPeer(peerEid_, peerBytes_);
    }

    function send(
        uint32 dstEid_,
        string memory message_
    ) external payable {
        bytes memory payload_ = abi.encode(message_);
        address payable refundAddress_ = payable(address(this));

        _lzSend(
            dstEid_,
            payload_,
            _options,
            MessagingFee(msg.value, 0),
            refundAddress_
        );
    }

    receive() external payable {}
}