import { setSuiteEnv } from "./utils/env";
import { getProvider } from "./utils/provider";

async function main(chains: string[]) {
    const providers = chains.map(chain => getProvider(chain));

    await Promise.all(
        providers.map(
            async provider => {
                await provider.deployDVN();
                await provider.deployMockOApp();
            }
        )
    );

    await Promise.all(
        providers.map(
            async (providerA, i) => {
                await Promise.all(
                    providers.slice(i+1).map(
                        async providerB => {
                            await providerA.setPeer(providerB);
                            await providerB.setPeer(providerA);
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