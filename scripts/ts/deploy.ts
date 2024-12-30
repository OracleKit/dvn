import { Hex } from "viem";
import { deployContract, getContract } from "../../src/utils/evm";
import { getProvider } from "../../src/utils/provider";
import { setSuiteEnv } from "./utils";

async function main(chains: string[]) {
    const providers = chains.map(chain => getProvider(chain));
    const dvnAddress = process.env.DVN_CANISTER_ADDRESS! as Hex;

    await Promise.all(
        providers.map(
            async provider => {
                const dvn = await deployContract(provider, "DVN", []);
                const proxy = await deployContract(provider, "DVNProxy", [dvn]);
                const oapp = await deployContract(provider, "MockOApp", [provider.endpoint, proxy]);
                const dvnContract = await getContract(provider, "DVN", proxy);
                const messageLibRole = await dvnContract.read.MESSAGE_LIB_ROLE();
                const dvnCanisterRole = await dvnContract.read.DVN_CANISTER_ROLE();

                await provider.wallet.waitForTransactionReceipt({
                    hash: await dvnContract.write.setEndpoint([provider.endpoint])
                });

                await provider.wallet.waitForTransactionReceipt({
                    hash: await dvnContract.write.setPriceFeed([provider.priceFeed])
                });

                for ( const messageLib of provider.messageLibs ) {
                    await provider.wallet.waitForTransactionReceipt({
                        hash: await dvnContract.write.grantRole([messageLibRole, messageLib])
                    });
                }

                await provider.wallet.waitForTransactionReceipt({
                    hash: await dvnContract.write.grantRole([dvnCanisterRole, dvnAddress])
                });

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
                            const dvnA = await getContract(providerA, "DVN", providerA.dvn!);
                            const dvnB = await getContract(providerB, "DVN", providerB.dvn!);

                            await Promise.all([
                                (async () => {
                                    await providerA.wallet.waitForTransactionReceipt({
                                        hash: await oappA.write.initPeer([ providerB.eid, providerB.mockApp! ])
                                    });
                                    await providerA.wallet.waitForTransactionReceipt({
                                        hash: await dvnA.write.setPriceConfig([{
                                            dstEid: providerB.eid,
                                            premiumBps: 2000,
                                            canisterFeeInUSD: BigInt(1e20 * 0.1),
                                            verifyGas: 80000n,
                                            verifyCalldataSize: 1000n,
                                        }])
                                    });
                                })(),
                                (async () => {
                                    await providerB.wallet.waitForTransactionReceipt({
                                        hash: await oappB.write.initPeer([ providerA.eid, providerA.mockApp! ])
                                    });
                                    await providerB.wallet.waitForTransactionReceipt({
                                        hash: await dvnB.write.setPriceConfig([{
                                            dstEid: providerA.eid,
                                            premiumBps: 2000,
                                            canisterFeeInUSD: BigInt(1e20 * 0.1),
                                            verifyGas: 80000n,
                                            verifyCalldataSize: 1000n,
                                        }])
                                    });
                                })()
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