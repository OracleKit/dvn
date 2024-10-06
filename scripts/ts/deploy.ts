import { Chain, Hex } from 'viem';
import { ProviderWrapper } from '../../src/utils/provider';
import { holesky, mainnet, polygonAmoy } from 'viem/chains';
import { polygon } from 'viem/chains';

const senderName = "ETHMAINNET";
const senderChain = holesky;
const receiverName = "POLYGONPOS";
const receiverChain = polygonAmoy;

function getProvider(name: string, chain: Chain) {
    const adminPrivateKey = process.env.ADMIN_PRIVATE_KEY as Hex;
    const rpcUrl = process.env[name + "_RPC_URL"] as string;
    const eid = parseInt(process.env[name + "_ENDPOINT_ID"] as string);
    const endpoint = process.env[name + "_ENDPOINT_ADDRESS"] as Hex;

    return new ProviderWrapper(adminPrivateKey, rpcUrl, chain, endpoint, eid);
}

async function main() {
    const senderProvider = getProvider(senderName, senderChain);
    const receiverProvider = getProvider(receiverName, receiverChain);

    await Promise.all([
        await senderProvider
            .deployDVN()
            .then(senderProvider.deployMockSender.bind(senderProvider)),
        await receiverProvider
            .deployDVN()
            .then(receiverProvider.deployMockReceiver.bind(receiverProvider))
    ]);

    await Promise.all([
        senderProvider.setPeer(receiverProvider),
        receiverProvider.setPeer(senderProvider)
    ]);

    console.log(`${senderName}_DVN_ADDRESS=${senderProvider.dvn!}`);
    console.log(`${senderName}_OAPP_ADDRESS=${senderProvider.mockApp!}`);
    console.log(`${receiverName}_DVN_ADDRESS=${receiverProvider.dvn!}`);
    console.log(`${receiverName}_OAPP_ADDRESS=${receiverProvider.mockApp!}`);
}

// main();