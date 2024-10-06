import { Chain, Hex, parseEther } from 'viem';
import { ProviderWrapper } from '../../src/utils/provider';
import { getContract } from '../../src/utils/evm';
import { polygonAmoy, polygonZkEvmCardona, holesky } from 'viem/chains';

const senderName = "ETHEREUMHOLESKY";
const senderChain = holesky;
const receiverName = "POLYGONAMOY";
const receiverChain = polygonAmoy;

function getProvider(name: string, chain: Chain) {
    const adminPrivateKey = process.env.ADMIN_PRIVATE_KEY as Hex;
    const rpcUrl = process.env[name + "_RPC_SSL_URL"] as string;
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

    const senderDvn = await getContract(senderProvider, "DVN", senderProvider.dvn!);
    const receiverDvn = await getContract(receiverProvider, "DVN", receiverProvider.dvn!);
    const senderOApp = await getContract(senderProvider, "MockOAppSender", senderProvider.mockApp!);
    const receiverOApp = await getContract(receiverProvider, "MockOAppReceiver", receiverProvider.mockApp!);

    const senderText = await senderOApp.read.getText();
    const receiverText = await receiverOApp.read.getText();

    console.log(`Sender Message (Ethereum Holensky): ${senderText}`);
    console.log(`Receiver Message (Polygon Amoy): ${receiverText}`);
}

main();