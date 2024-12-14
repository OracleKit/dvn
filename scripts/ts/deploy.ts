import { deployContract, getContract } from "../../src/utils/evm";
import { getProvider } from "../../src/utils/provider";
import { setSuiteEnv } from "./utils";

async function main(chains: string[]) {
    const providers = chains.map(chain => getProvider(chain));

    await Promise.all(
        providers.map(
            async provider => {
                const dvn = await deployContract(provider, "DVN", []);
                const proxy = await deployContract(provider, "DVNProxy", [dvn]);
                const oapp = await deployContract(provider, "MockOApp", [provider.endpoint, proxy]);
                const dvnContract = await getContract(provider, "DVN", proxy);
                await provider.wallet.waitForTransactionReceipt({
                    hash: await dvnContract.write.setEndpoint([provider.endpoint])
                });

                for ( const messageLib of provider.messageLibs ) {
                    await provider.wallet.waitForTransactionReceipt({
                        hash: await dvnContract.write.addMessageLib([messageLib])
                    });
                }

                provider.dvn = proxy;
                provider.mockApp = oapp;
            }
        )
    );

    await Promise.all(
        providers.map(
            async (providerA, i) => {
                await Promise.all(
                    providers.slice(i+1).map(
                        async providerB => {
                            const oappA = await getContract(providerA, "MockOApp", providerA.mockApp!);
                            const oappB = await getContract(providerB, "MockOApp", providerB.mockApp!);

                            await Promise.all([
                                providerA.wallet.waitForTransactionReceipt({
                                    hash: await oappA.write.initPeer([ providerB.eid, providerB.mockApp! ])
                                }),
                                providerB.wallet.waitForTransactionReceipt({
                                    hash: await oappB.write.initPeer([ providerA.eid, providerA.mockApp! ])
                                })
                            ]);
                        }
                    )
                )
            }
        )
    );

    providers.forEach(async (provider, i) => {
        let name = chains[i].toUpperCase();
        await setSuiteEnv(`${name}_DVN_ADDRESS`, provider.dvn!);
        await setSuiteEnv(`${name}_OAPP_ADDRESS`, provider.mockApp!);
    })
}

main(process.argv.slice(2));