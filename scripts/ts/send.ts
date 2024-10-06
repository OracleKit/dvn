import { Chain, Hex, parseEther } from 'viem';
import { ProviderWrapper } from '../../src/utils/provider';
import { holesky, mainnet, polygon, polygonAmoy } from 'viem/chains';
import { getContract } from '../../src/utils/evm';

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

    const oappContract = await getContract(senderProvider, "MockOAppSender", senderProvider.mockApp!);
    const dvnContract = await getContract(senderProvider, "DVN", senderProvider.dvn!);

    const fees = await oappContract.read.quote([receiverProvider.eid, "Hello polygon"]);
    const nativeFees = fees.nativeFee;

    const receipt = await senderProvider.awaitTransaction(
        oappContract.write.send([receiverProvider.eid, "Hello polygon"], {
            value: nativeFees
        })
    );

    console.log(receipt);
}

main();