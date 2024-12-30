import { Hex } from "viem";
import { ProviderWrapper } from "../../src/utils/provider";
import { getContract } from "../../src/utils/evm";
import { getAdminConfig, getChainConfig } from "../../src/utils/env";
import dvns from "./dvn.json";

async function main() {
    const adminConfig = getAdminConfig();
    const polygonConfig = getChainConfig('polygonAmoy');
    const ethereumConfig = getChainConfig('ethereumHolesky');

    const provider = new ProviderWrapper(
        adminConfig.privateKey,
        ethereumConfig.rpcUrl,
        ethereumConfig.chainId,
        ethereumConfig.endpoint,
        ethereumConfig.endpointId,
        ethereumConfig.messageLibs,
        ethereumConfig.priceFeed
    );

    const priceFeed = await getContract(provider, "ILayerZeroPriceFeed", provider.priceFeed);
    console.log(await priceFeed.read.nativeTokenPriceUSD());
    console.log(await priceFeed.read.getPriceRatioDenominator());
    console.log(await priceFeed.read.getPrice([polygonConfig.endpointId - 30000]));
}

main();