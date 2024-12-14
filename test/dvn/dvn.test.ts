import { assert } from "chai";
import { getProvider } from "../../src/utils/provider";
import { getContract } from "../../src/utils/evm";
import { decodeEventLog, Log, parseEventLogs } from "viem";

describe("DVN", function() {
    it("E2E Test", async function() {
        this.timeout(60000);

        const srcChainName = process.env.SOURCE_CHAIN_NAME!;
        const destChainName = process.env.DESTINATION_CHAIN_NAME!;

        assert(srcChainName, "Source chain env present");
        assert(destChainName, "Destination chain env present");

        const srcProvider = getProvider(srcChainName);
        const destProvider = getProvider(destChainName);

        const srcBlockNum = await srcProvider.wallet.getBlockNumber();
        const destBlockNum = await destProvider.wallet.getBlockNumber();
        const srcDvn = await getContract(srcProvider, "DVN", srcProvider.dvn!);
        const srcOapp = await getContract(srcProvider, "MockOApp", srcProvider.mockApp!);
        const destEndpoint = await getContract(destProvider, "ILayerZeroEndpointV2", destProvider.endpoint);

        const fees = await srcOapp.read.quote([destProvider.eid, "Helllo"]);

        await srcProvider.wallet.waitForTransactionReceipt({
            hash: (await srcOapp.write.send([destProvider.eid, ""], {
                value: fees.nativeFee + 1000n
            }))
        });

        const events = await srcDvn.getEvents.TaskAssigned({}, {
            fromBlock: srcBlockNum,
            toBlock: 'latest'
        });

        assert(events.length == 1, "Tasks assigned");

        const [receiveLibraryAddress] = await destEndpoint.read.getReceiveLibrary([destProvider.mockApp!, destProvider.eid]);
        const destReceiveLibrary = await getContract(destProvider, "ReceiveUlnBase", receiveLibraryAddress);

        const logs = await new Promise<Log[]>((resolve, reject) => {
            const unwatch = destReceiveLibrary.watchEvent.PayloadVerified({
                onLogs: logs => resolve(logs),
                fromBlock: destBlockNum,
            });

            setTimeout(() => {
                unwatch();
                reject();
            }, 35000);
        });

        const decodedLog = decodeEventLog({
            abi: destReceiveLibrary.abi,
            data: logs[0].data,
            topics: logs[0].topics
        });

        assert(logs.length == 1, "PayloadVerified log found");
        assert(
            decodedLog.eventName === 'PayloadVerified' &&
            decodedLog.args.dvn.toLowerCase() === destProvider.dvn!.toLowerCase(),
            "PayloadVerified log correct"
        );
    });
});