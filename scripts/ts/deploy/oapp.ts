import { argv } from "../utils/argv";
import { ChainIds, getChain, getProvider } from "../utils/provider";

async function main(chain: string, isSender: string) {
    const provider = getProvider(chain as ChainIds);
    await (isSender === 'sender' ? provider.deployMockSender() : provider.deployMockReceiver());

    const name = getChain(chain as ChainIds).name;
    console.log(`${name}_OAPP_ADDRESS=${provider.mockApp}`);
}


main(argv(), argv());