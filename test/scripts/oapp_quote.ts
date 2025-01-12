import { formatEther, Hex, parseEther } from "viem";
import { ProviderWrapper } from "../../src/utils/provider";
import { getContract } from "../../src/utils/evm";
import { getAdminConfig, getChainConfig } from "../../src/utils/env";
import dvns from "./dvn.json";

async function main() {
    const adminConfig = getAdminConfig();
    const senderConfig = getChainConfig('arbitrumOne');
    const destConfig = getChainConfig('polygonPoS');

    const provider = new ProviderWrapper(
        adminConfig.privateKey,
        senderConfig.rpcUrl,
        senderConfig.chainId,
        senderConfig.endpoint,
        senderConfig.endpointId,
        senderConfig.messageLibs,
        senderConfig.priceFeed
    );

    const oapp = await getContract(provider, "MockOApp", senderConfig.oapp!);
    const fees = await oapp.read.quote([destConfig.endpointId, "Hello world"]);
    console.log(formatEther(fees.nativeFee));
}

main();