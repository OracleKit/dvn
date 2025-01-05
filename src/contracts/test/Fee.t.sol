// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { Test } from "forge-std/Test.sol";
import { Helper } from "./Helper.sol";
import { DVN } from "../src/DVN.sol";
import { DVNProxy } from "../src/DVNProxy.sol";
import { ILayerZeroPriceFeed } from "@layerzerolabs/lz-evm-messagelib-v2/contracts/interfaces/ILayerZeroPriceFeed.sol";

contract FeeTest is Helper, Test {
    uint256 priceRatioDenominator = 1e20;
    uint16 premiumBps = 2000;
    uint256 canisterFeeInUSD = 3 * priceRatioDenominator;
    uint256 verifyGas = 80000;
    uint256 verifyCalldataSize = 500;
    uint32 gasPerByte = 60;
    uint64 gasPrice = 0.1 * 1e9;
    uint256 usdPrice = 6 * priceRatioDenominator;
    uint256 totalGasFee = (verifyGas + verifyCalldataSize * gasPerByte) * gasPrice;
    uint256 canisterFee = (canisterFeeInUSD * 1e18) / usdPrice;
    uint256 totalFee = ((totalGasFee + canisterFee) * (1e4 + premiumBps)) / 1e4;

    function setUp() public {
        _dvn = new DVN();
        _proxy = new DVNProxy(address(_dvn));
        _dvnBehindProxy = DVN(address(_proxy));
    }

    function test_setPriceFeed_RevertIf_CalledByNonAdmin() public {
        address prankAddress = makeAddr("pranker");
        address testAddress = makeAddr("test");

        vm.prank(prankAddress);
        vm.expectRevert(DVN.Unauthorized.selector);
        _dvnBehindProxy.setPriceFeed(testAddress);
    }

    function test_setPriceFeed_CorrectlySetsPriceFeed() public {
        address priceFeed = makeAddr("priceFeed");

        _dvnBehindProxy.setPriceFeed(priceFeed);
        address actualPriceFeed = _dvnBehindProxy.priceFeed();
        assertEq(actualPriceFeed, priceFeed);
    }

    function test_setPriceConfig_RevertIf_CalledByNonAdmin() public {
        address prankAddress = makeAddr("pranker");
        DVN.PriceConfig memory priceConfig = DVN.PriceConfig(1, 2, 3, 4, 5);

        vm.prank(prankAddress);
        vm.expectRevert(DVN.Unauthorized.selector);
        _dvnBehindProxy.setPriceConfig(priceConfig);
    }

    function test_setPriceConfig_CorrectlySetsConfig() public {
        DVN.PriceConfig memory priceConfig = DVN.PriceConfig(1, 2, 3, 4, 5);

        _dvnBehindProxy.setPriceConfig(priceConfig);
        DVN.PriceConfig memory actualPriceConfig = _dvnBehindProxy.priceConfig(priceConfig.dstEid);
        assertEq(actualPriceConfig.dstEid, priceConfig.dstEid);
        assertEq(actualPriceConfig.canisterFeeInUSD, priceConfig.canisterFeeInUSD);
        assertEq(actualPriceConfig.premiumBps, priceConfig.premiumBps);
        assertEq(actualPriceConfig.verifyCalldataSize, priceConfig.verifyCalldataSize);
        assertEq(actualPriceConfig.verifyGas, priceConfig.verifyGas);
    }

    function test_feeCollected_StartsWithZero() public view {
        uint256 feeCollected = _dvnBehindProxy.feeCollected();
        assertEq(feeCollected, 0);
    }
    
    function test_getFee_Success() public {
        address priceFeed = makeAddr("priceFeed");
        uint32 dstEid = 1;

        assertEq(totalGasFee, 1.1 * 1e13);
        assertEq(canisterFee, 5 * 1e17);
        assertEq(totalFee, 6000132 * 1e11);

        _dvnBehindProxy.setPriceConfig(
            DVN.PriceConfig(
                dstEid,
                premiumBps,
                canisterFeeInUSD,
                verifyGas,
                verifyCalldataSize
            )
        );
        _dvnBehindProxy.setPriceFeed(priceFeed);

        vm.mockCall(
            priceFeed,
            abi.encodeWithSelector(
                ILayerZeroPriceFeed.estimateFeeByEid.selector,
                dstEid % 30000,
                verifyCalldataSize,
                verifyGas
            ),
            abi.encode(
                totalGasFee,
                1,
                priceRatioDenominator,
                usdPrice
            )
        );

        vm.mockCall(
            priceFeed,
            abi.encodeWithSelector(
                ILayerZeroPriceFeed.getPrice.selector,
                dstEid % 30000
            ),
            abi.encode(
                ILayerZeroPriceFeed.Price(1, gasPrice, gasPerByte)
            )
        );

        uint256 fee = _dvnBehindProxy.getFee(dstEid, 5, address(this), "");
        assertEq(fee, totalFee);
    }

    function test_withdrawFee_RevertIf_CalledByNonAdmin() public {
        address prankAddress = makeAddr("pranker");

        vm.prank(prankAddress);
        vm.expectRevert(DVN.Unauthorized.selector);
        _dvnBehindProxy.withdrawFee(makeAddr("random"), address(this), 1);
    }

    function test_withdrawFee_RevertIf_FromIsNotMessageLib() public {
        address randomAddress = makeAddr("random");

        vm.expectRevert(DVN.WithdrawNotFromMessageLib.selector);
        _dvnBehindProxy.withdrawFee(randomAddress, address(this), 1);
    }

    function test_withdrawFee_RevertIf_AmountIsNotAvailable() public {
        address messageLib = makeAddr("messageLib");
        _dvnBehindProxy.grantRole(_dvnBehindProxy.MESSAGE_LIB_ROLE(), messageLib);

        vm.expectRevert(DVN.WithdrawAmountInvalid.selector);
        _dvnBehindProxy.withdrawFee(messageLib, address(this), 10);
    }
}