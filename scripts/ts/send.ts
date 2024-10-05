import { Chain, Hex, parseEther } from 'viem';
import { ProviderWrapper } from '../../src/utils/provider';
import { mainnet, polygon } from 'viem/chains';
import { getContract } from '../../src/utils/evm';

const senderName = "ETHMAINNET";
const senderChain = mainnet;
const receiverName = "POLYGONPOS";
const receiverChain = polygon;

function getProvider(name: string, chain: Chain) {
    const adminPrivateKey = process.env.ADMIN_PRIVATE_KEY as Hex;
    const rpcUrl = process.env[name + "_RPC_URL"] as string;
    const eid = parseInt(process.env[name + "_ENDPOINT_ID"] as string);
    const endpoint = process.env[name + "_ENDPOINT_ADDRESS"] as Hex;
    const dvnAddress = process.env[name + "_DVN_ADDRESS"] as Hex;
    const oappAddress = process.env[name + "_OAPP_ADDRESS"] as Hex;

    const provider = new ProviderWrapper(adminPrivateKey, rpcUrl, chain, endpoint, eid);
    provider.dvn = dvnAddress;
    provider.mockApp = oappAddress;

    return provider;
}

async function main() {
    const senderProvider = getProvider(senderName, senderChain);
    const receiverProvider = getProvider(receiverName, receiverChain);

    const oappContract = await getContract(senderProvider, "MockOAppSender", senderProvider.mockApp!);
    const dvnContract = await getContract(senderProvider, "DVN", senderProvider.dvn!);
    const receipt = await senderProvider.awaitTransaction(
        oappContract.write.send([receiverProvider.eid, "Hello World"], {
            value: parseEther('1')
        })
    );

    console.log(await dvnContract.getEvents.JobAssigned());
}

main();