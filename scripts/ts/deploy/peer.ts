import { argv } from "../utils/argv";
import { ChainIds, getProvider } from "../utils/provider";

async function main(chainA: string, chainB: string) {
    const providerA = getProvider(chainA as ChainIds);
    const providerB = getProvider(chainB as ChainIds);

    await Promise.all([
        providerA.setPeer(providerB),
        providerB.setPeer(providerA)
    ]);

    console.log(`Peering between ${chainA} and ${chainB} successful.`);
}


main(argv(), argv());