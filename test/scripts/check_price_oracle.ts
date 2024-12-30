import { Hex } from "viem";
import { ProviderWrapper } from "../../src/utils/provider";
import { getContract } from "../../src/utils/evm";
import { getAdminConfig, getChainConfig } from "../../src/utils/env";
import dvns from "./dvn.json";

async function main() {
    const adminConfig = getAdminConfig();
    const polygonConfig = getChainConfig('polygonPos');

    const provider = new ProviderWrapper(
        adminConfig.privateKey,
        polygonConfig.rpcUrl,
        polygonConfig.chainId,
        polygonConfig.endpoint,
        polygonConfig.endpointId,
        polygonConfig.messageLibs
    );

    const priceFeed = await getContract(provider, "ILayerZeroPriceFeed", "0x119C04C4E60158fa69eCf4cdDF629D09719a7572");
    console.log(await priceFeed.read.nativeTokenPriceUSD());
    console.log(await priceFeed.read.getPriceRatioDenominator());
}

main();