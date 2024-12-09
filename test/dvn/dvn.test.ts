import { assert } from "chai";
import { getProvider } from "../../src/utils/provider";
import { getContract } from "../../src/utils/evm";

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
        const srcDvn = await getContract(srcProvider, "DVN", srcProvider.dvn!);
        const srcOapp = await getContract(srcProvider, "MockOApp", srcProvider.mockApp!);
        const destDvn = await getContract(destProvider, "DVN", destProvider.dvn!);
        const destOapp = await getContract(destProvider, "MockOApp", destProvider.mockApp!);

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

        await new Promise<void>((resolve, reject) => {
            setTimeout(() => resolve(), 35000)
        });

        const received = await destDvn.read.verified([events[0].args.task!])
        assert(received, "Tasks received");
    });
});