import { assert } from "chai";
import { getProvider } from "../../src/utils/provider";
import { getContract } from "../../src/utils/evm";
import { decodeEventLog, Hex, Log, parseEventLogs } from "viem";

describe("DVN", function() {
    it("E2E Test", async function() {
        this.timeout(120000);

        const srcChainName = process.env.SOURCE_CHAIN_NAME!;
        const destChainName = process.env.DESTINATION_CHAIN_NAME!;
        const dvnAddress = process.env.DVN_CANISTER_ADDRESS as Hex;

        assert(srcChainName, "Source chain env present");
        assert(destChainName, "Destination chain env present");

        const srcProvider = getProvider(srcChainName);
        const destProvider = getProvider(destChainName);

        const srcBlockNum = await srcProvider.wallet.getBlockNumber();
        const destBlockNum = await destProvider.wallet.getBlockNumber();
        const srcDvn = await getContract(srcProvider, "DVN", srcProvider.dvn!);
        const srcOapp = await getContract(srcProvider, "MockOApp", srcProvider.mockApp!);
        const destEndpoint = await getContract(destProvider, "ILayerZeroEndpointV2", destProvider.endpoint);
        const srcPriceFeed = await getContract(srcProvider, "ILayerZeroPriceFeed", srcProvider.priceFeed);

        const fees = await srcOapp.read.quote([destProvider.eid, "Helllo"]);

        const dvnBalanceBeforeTxns = await destProvider.wallet.getBalance({ address: dvnAddress });

        const numTransactions = 5;
        for ( let i = 0; i < numTransactions; i++ ) {
            await srcProvider.wallet.waitForTransactionReceipt({
                hash: (await srcOapp.write.send([destProvider.eid, ""], {
                    value: fees.nativeFee + 1000n
                }))
            });
        }

        const events = await srcDvn.getEvents.TaskAssigned({}, {
            fromBlock: srcBlockNum,
            toBlock: 'latest'
        });

        assert(events.length == numTransactions, "Tasks assigned");

        const [receiveLibraryAddress] = await destEndpoint.read.getReceiveLibrary([destProvider.mockApp!, destProvider.eid]);
        const destReceiveLibrary = await getContract(destProvider, "ReceiveUlnBase", receiveLibraryAddress);

        const logs = await new Promise<Log[]>((resolve, reject) => {
            let foundLogs: Log[] = [];

            const unwatch = destReceiveLibrary.watchEvent.PayloadVerified({
                onLogs: logs => foundLogs = foundLogs.concat(logs),
                fromBlock: destBlockNum,
            });

            setTimeout(() => {
                unwatch();
                resolve(foundLogs);
            }, 60000);
        });

        assert(logs.length == numTransactions, "PayloadVerified log found");

        for ( const log of logs ) {
            const decodedLog = decodeEventLog({
                abi: destReceiveLibrary.abi,
                data: log.data,
                topics: log.topics
            });
    
            assert(
                decodedLog.eventName === 'PayloadVerified' &&
                decodedLog.args.dvn.toLowerCase() === destProvider.dvn!.toLowerCase(),
                "PayloadVerified log correct"
            );
        }

        const dvnBalanceAfterTxns = await destProvider.wallet.getBalance({ address: dvnAddress });
        const dvnCollectedFees = await srcDvn.read.feeCollected();
        const { priceRatio } = await srcPriceFeed.read.getPrice([destProvider.eid % 30000]);

        console.log(dvnBalanceAfterTxns, dvnCollectedFees, dvnBalanceBeforeTxns, priceRatio, dvnBalanceAfterTxns + ((dvnCollectedFees * BigInt(1e20)) / priceRatio));
        assert(dvnBalanceAfterTxns + ((dvnCollectedFees * BigInt(1e20)) / priceRatio) > dvnBalanceBeforeTxns, "DVN balance increased");
    });
});