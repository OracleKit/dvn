// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { ILayerZeroDVN } from "@layerzerolabs/lz-evm-messagelib-v2/contracts/uln/interfaces/ILayerZeroDVN.sol";
import { ILayerZeroEndpointV2, Origin } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";
import { ILayerZeroPriceFeed } from "@layerzerolabs/lz-evm-messagelib-v2/contracts/interfaces/ILayerZeroPriceFeed.sol";
import { PacketV1Codec } from "@layerzerolabs/lz-evm-protocol-v2/contracts/messagelib/libs/PacketV1Codec.sol";
import { IReceiveUlnE2 } from "@layerzerolabs/lz-evm-messagelib-v2/contracts/uln/interfaces/IReceiveUlnE2.sol";
import { ERC1967Utils } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Utils.sol";
import { UUPSUpgradeable } from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import { AccessControl } from "@openzeppelin/contracts/access/AccessControl.sol";
import { ISendLib } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ISendLib.sol";
import "hardhat/console.sol";

contract DVN is ILayerZeroDVN, UUPSUpgradeable, AccessControl {
    using PacketV1Codec for bytes;
    bytes32 public constant MESSAGE_LIB_ROLE = keccak256("MESSAGE_LIB");
    bytes32 public constant DVN_CANISTER_ROLE = keccak256("DVN_CANISTER");

    struct PriceConfig {
        uint32 dstEid;
        uint16 premiumBps;
        uint256 canisterFeeInUSD; // uses PriceRatioDenominator
        uint256 verifyGas;
        uint256 verifyCalldataSize;
    }

    address _endpoint;
    address _priceFeed;
    uint256 _feeCollected;

    mapping(uint256 => PriceConfig) _dstEidPriceConfigs;

    event TaskAssigned(
        uint32 indexed dstEid,
        uint64 indexed numConfirmations,
        uint256 indexed maxUnitGasPrice,
        AssignJobParam task
    );

    error Unauthorized();
    error Unimplemented();
    error WithdrawNotFromMessageLib();
    error WithdrawAmountInvalid();
    error FeeCalculationWentWrong();
    error InvalidRole();

    modifier onlyAdmin() {
        if ( msg.sender != ERC1967Utils.getAdmin() ) {
            revert Unauthorized();
        }
        _;
    }

    function admin() external view onlyProxy returns (address) {
        return ERC1967Utils.getAdmin();
    }

    function endpoint() external view onlyProxy returns (address) {
        return _endpoint;
    }

    function priceFeed() external view onlyProxy returns (address) {
        return _priceFeed;
    }

    function priceConfig(uint256 eid_) external view onlyProxy returns (PriceConfig memory) {
        return _dstEidPriceConfigs[eid_];
    }

    function feeCollected() external view onlyProxy onlyAdmin returns (uint256) {
        return _feeCollected;
    }

    function setAdmin(address newAdmin) external onlyProxy onlyAdmin {
        ERC1967Utils.changeAdmin(newAdmin);
    }

    function setEndpoint(address endpoint_) external onlyProxy onlyAdmin {
        _endpoint = endpoint_;
    }

    function setPriceFeed(address priceFeed_) external onlyProxy onlyAdmin {
        _priceFeed = priceFeed_;
    }

    function setPriceConfig(PriceConfig calldata priceConfig_) external onlyProxy onlyAdmin {
        _dstEidPriceConfigs[priceConfig_.dstEid] = priceConfig_;
    }

    function _isValidRole(bytes32 role) internal pure {
        if (
            MESSAGE_LIB_ROLE == role ||
            DVN_CANISTER_ROLE == role
        ) {
            return;
        }
        
        revert InvalidRole();
    }

    function grantRole(bytes32 role, address account) public override onlyProxy onlyAdmin {
        _isValidRole(role);
        _grantRole(role, account);
    }

    function revokeRole(bytes32 role, address account) public override onlyProxy onlyAdmin {
        _isValidRole(role);
        _revokeRole(role, account);
    }

    function _bytes32ToAddress(bytes32 _b) internal pure returns (address) {
        return address(uint160(uint256(_b)));
    }

    function _calculateFee(uint32 _dstEid) internal view returns (uint256 unitGasPrice, uint256 totalFee) {
        PriceConfig memory priceConfig_ = _dstEidPriceConfigs[_dstEid];
        
        if ( priceConfig_.dstEid == 0 ) {
            revert Unimplemented();
        }

        (uint256 fee,,, uint256 nativePriceUsd) = ILayerZeroPriceFeed(_priceFeed).estimateFeeByEid(_dstEid % 30000, priceConfig_.verifyCalldataSize, priceConfig_.verifyGas);
        ILayerZeroPriceFeed.Price memory price = ILayerZeroPriceFeed(_priceFeed).getPrice(_dstEid % 30000);

        uint256 gasFee = fee;
        uint256 canisterFee = (priceConfig_.canisterFeeInUSD  * 1e18) / nativePriceUsd;
        uint256 multiplierBps = 10000 + priceConfig_.premiumBps;
        totalFee = gasFee + canisterFee;
        totalFee = (totalFee * multiplierBps) / 10000;
        unitGasPrice = price.gasPriceInUnit;
    }

    function assignJob(
        AssignJobParam calldata task_,
        bytes calldata /*_options*/
    ) external payable onlyProxy onlyRole(MESSAGE_LIB_ROLE) returns (uint256) {
        (uint256 unitGasPrice, uint256 totalFee) = _calculateFee(task_.dstEid);
        if ( unitGasPrice <= 0 || totalFee <= 0 ) revert FeeCalculationWentWrong();

        _feeCollected += totalFee;
        emit TaskAssigned(task_.dstEid, task_.confirmations, unitGasPrice, task_);
        return totalFee;
    }

    function verify(AssignJobParam calldata task_) external onlyProxy onlyRole(DVN_CANISTER_ROLE) {
        bytes calldata packetHeader_ = task_.packetHeader;
        address receiver_ = _bytes32ToAddress(packetHeader_.receiver());

        (address receiveLibrary,)= ILayerZeroEndpointV2(_endpoint).getReceiveLibrary(receiver_, packetHeader_.dstEid());
        IReceiveUlnE2(receiveLibrary).verify(packetHeader_, task_.payloadHash, task_.confirmations);
    }
    
    function getFee(
        uint32 dstEid_,
        uint64 /*_confirmations*/,
        address /*_sender*/,
        bytes calldata /*_options*/
    ) external view onlyProxy returns (uint256) {
        (,uint256 totalFee) = _calculateFee(dstEid_);
        if ( totalFee <= 0 ) revert FeeCalculationWentWrong();
        return totalFee;
    }

    function withdrawFee(address from_, address to_, uint256 amount_) external onlyProxy onlyAdmin {
        if ( !hasRole(MESSAGE_LIB_ROLE, from_) ) revert WithdrawNotFromMessageLib();
        if ( amount_ <= 0 || amount_ > _feeCollected ) revert WithdrawAmountInvalid();

        _feeCollected -= amount_;
        ISendLib(from_).withdrawFee(to_, amount_);
    }

    /** UUPSUpgradeable Functions */

    // Authorizes upgradeToAndCall() from UUPSUpgradeable
    function _authorizeUpgrade(address /*newImplementation*/) internal view override onlyAdmin {}
}