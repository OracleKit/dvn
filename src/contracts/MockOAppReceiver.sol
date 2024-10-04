// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { OAppReceiver, Origin } from "@layerzerolabs/oapp-evm/contracts/oapp/OAppReceiver.sol";
import { OAppCore } from "@layerzerolabs/oapp-evm/contracts/oapp/OAppCore.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { SetConfigParam } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/IMessageLibManager.sol";
import { UlnConfig } from "@layerzerolabs/lz-evm-messagelib-v2/contracts/uln/UlnBase.sol";

contract MockOAppReceiver is OAppReceiver {
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
        (address receiveLibrary_,) = endpoint.getReceiveLibrary(address(this), peerEid_);
        bytes32 peerBytes_ = bytes32(uint256(uint160(peerAddress_)));
        
        SetConfigParam[] memory params = new SetConfigParam[](1);
        params[0] = SetConfigParam(peerEid_, 2, _ulnConfig);

        endpoint.setConfig(address(this), receiveLibrary_, params);
        setPeer(peerEid_, peerBytes_);
    }
    
    function _lzReceive(
        Origin calldata _origin,
        bytes32 /*_guid*/,
        bytes calldata message,
        address /*executor*/,
        bytes calldata /*_extraData*/
    ) internal override {}
}