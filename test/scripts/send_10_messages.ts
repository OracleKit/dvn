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
        senderChainConfig.rpcUrl,
        senderChainConfig.chainId,
        senderChainConfig.endpoint,
        senderChainConfig.endpointId,
        senderChainConfig.messageLibs
    );

    provider.dvn = senderChainConfig.dvn;
    provider.mockApp = senderChainConfig.oapp;

    const oappContract = await getContract(provider, "MockOApp", provider.mockApp!);
    const dvnContract = await getContract(provider, "DVN", provider.dvn!);

    for ( let i = 0; i < 2; i++ ) {
        const hash = await oappContract.write.send([receiverChainConfig.endpointId, "Hello World"], { value: parseEther("0.01") });
        const receipt = await provider.wallet.waitForTransactionReceipt({ hash });

        const events = await dvnContract.getEvents.TaskAssigned({}, { blockHash: receipt.blockHash });
        console.log(`Sent ${events.length} messages`);
    }
}

main();