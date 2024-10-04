import { Chain, Hex, parseEther } from 'viem';
import { ProviderWrapper } from '../../src/utils/provider';
import { mainnet } from 'viem/chains';
import { polygon } from 'viem/chains';

const ethereumName = "ETHMAINNET";
const polygonName = "POLYGONPOS";

function getProvider(name: string, chain: Chain) {
    const adminPrivateKey = process.env.ADMIN_PRIVATE_KEY as Hex;
    const rpcUrl = process.env[name + "_RPC_URL"] as string;
    const eid = parseInt(process.env[name + "_ENDPOINT_ID"] as string);
    const endpoint = process.env[name + "_ENDPOINT_ADDRESS"] as Hex;

    return new ProviderWrapper(adminPrivateKey, rpcUrl, chain, endpoint, eid);
}

async function main() {
    const ethProvider = getProvider(ethereumName, mainnet);
    const polyProvider = getProvider(polygonName, polygon);

    await Promise.all([
        await ethProvider
            .deployDVN()
            .then(ethProvider.deployMockSender.bind(ethProvider)),
        await polyProvider
            .deployDVN()
            .then(polyProvider.deployMockReceiver.bind(polyProvider))
    ]);

    await Promise.all([
        ethProvider.setPeer(polyProvider),
        polyProvider.setPeer(ethProvider)
    ]);

    console.log(`${ethereumName}_DVN_ADDRESS=${ethProvider.dvn!}`);
    console.log(`${ethereumName}_OAPP_ADDRESS=${ethProvider.mockApp!}`);
    console.log(`${polygonName}_DVN_ADDRESS=${polyProvider.dvn!}`);
    console.log(`${polygonName}_OAPP_ADDRESS=${polyProvider.mockApp!}`);
}

main();