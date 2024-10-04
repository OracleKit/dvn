// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { EndpointV2 } from "@layerzerolabs/lz-evm-protocol-v2/contracts/EndpointV2.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";

contract EndpointV2Mock is EndpointV2 {
    constructor(uint32 _eid, address _owner) EndpointV2(_eid, _owner) Ownable(_owner) {
        _transferOwnership(_owner);
    }
}