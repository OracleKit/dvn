import { Chain, decodeErrorResult, Hex, parseEther } from 'viem';
import { ProviderWrapper } from '../../src/utils/provider';
import { mainnet } from 'viem/chains';
import { polygon } from 'viem/chains';
import { getContract } from '../../src/utils/evm';
import { loadContract } from '../../src/utils/artifact';

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
    const polyProvider = getProvider(polygonName, polygon);
    const dvnContract = await getContract(polyProvider, "DVN", polyProvider.dvn!);
    let numJobs = await dvnContract.read.test_verifiedJobsNum();
    console.log(`Num of verified jobs: ${numJobs}`);

    let isLastJobVerified = await dvnContract.read.test_verified([numJobs - 1n]);
    console.log(`Last job is verified?: ${isLastJobVerified ? "Yes" : "No"}`);
}

main();