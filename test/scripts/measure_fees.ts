import { Hex } from "viem";
import { ProviderWrapper } from "../../src/utils/provider";
import { getContract } from "../../src/utils/evm";
import { getAdminConfig, getChainConfig } from "../../src/utils/env";
import dvns from "./dvn.json";

async function main() {
    const adminConfig = getAdminConfig();
    const optimismConfig = getChainConfig('optimismMainnet');
    const polygonConfig = getChainConfig('polygonPos');

    const provider = new ProviderWrapper(
        adminConfig.privateKey,
        polygonConfig.rpcUrl,
        polygonConfig.chainId,
        polygonConfig.endpoint,
        polygonConfig.endpointId
    );

    const fees: { name: string, fees: number }[] = [];

    await Promise.all(
        Object.keys(dvns).map(async (dvn) => {
            try {
                const contract = await getContract(provider, "DVN", dvns[dvn as keyof typeof dvns].polygon as Hex);
                const fee = await contract.read.getFee([optimismConfig.endpointId, 5n, adminConfig.address, '0x']);
                fees.push({ name: dvn, fees: parseFloat((fee / 1000000000n).toString()) });
            } catch (e: any) {
                fees.push({ name: dvn, fees: -1 });
            }
        })
    );

    fees.sort((a, b) => a.fees - b.fees);

    for ( const fee of fees ) {
        console.log(`${fee.name}: ${fee.fees}`)
    }
}

main();