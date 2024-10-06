import { argv } from "../utils/argv";
import { ChainIds, getChain, getProvider } from "../utils/provider";

async function main(chain: string) {
    const provider = getProvider(chain as ChainIds);
    await provider.deployDVN();

    const name = getChain(chain as ChainIds).name;
    console.log(`${name}_DVN_ADDRESS=${provider.dvn}`);
}


main(argv());