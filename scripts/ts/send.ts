import { Chain, Hex, parseEther } from 'viem';
import { ProviderWrapper } from '../../src/utils/provider';
import { mainnet } from 'viem/chains';
import { polygon } from 'viem/chains';
import { getContract } from '../../src/utils/evm';

const ethereumName = "ETHMAINNET";
const polygonName = "POLYGONPOS";

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
    const ethProvider = getProvider(ethereumName, mainnet);
    const oappContract = await getContract(ethProvider, "MockOAppSender", ethProvider.mockApp!);
    const dvnContract = await getContract(ethProvider, "DVN", ethProvider.dvn!);
    const receipt = await ethProvider.awaitTransaction(
        oappContract.write.send([30109, "Hello Polygon from Ethereum"], {
            value: parseEther('1')
        })
    );

    console.log(await dvnContract.getEvents.JobAssigned());
}

main();