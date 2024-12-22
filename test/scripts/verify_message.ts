import { Hex, parseEther } from "viem";
import { ProviderWrapper } from "../../src/utils/provider";
import { getContract } from "../../src/utils/evm";
import { getAdminConfig, getChainConfig } from "../../src/utils/env";

async function main() {
    const adminConfig = getAdminConfig();
    const senderChainConfig = getChainConfig('ethereumHolesky');
    const receiverChainConfig = getChainConfig('polygonAmoy');

    const provider = new ProviderWrapper(
        adminConfig.privateKey,
        receiverChainConfig.rpcUrl,
        receiverChainConfig.chainId,
        receiverChainConfig.endpoint,
        receiverChainConfig.endpointId,
        receiverChainConfig.messageLibs
    );

    provider.dvn = receiverChainConfig.dvn;
    provider.mockApp = receiverChainConfig.oapp;

    const oappContract = await getContract(provider, "MockOApp", provider.mockApp!);
    const dvnContract = await getContract(provider, "DVN", provider.dvn!);
    const endpointContract = await getContract(provider, "ILayerZeroEndpointV2", provider.endpoint);

    const [receiveLibraryAddress, ] = await endpointContract.read.getReceiveLibrary([oappContract.address, senderChainConfig.endpointId]);
    const receiveLibrary = await getContract(provider, "ReceiveUlnBase", receiveLibraryAddress);

    const events = await receiveLibrary.getEvents.PayloadVerified({
        fromBlock: 'earliest',
        toBlock: 'latest'
    });

    console.log(`Found ${events.length} events!`);
}

main();