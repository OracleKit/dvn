/**
 * Deploys all smart contracts & funds DVN with 500 ETH
 * Usage:
 *  [SOURCE_NETWORK_PORT] [SOURCE_NETWORK_PRIVATE_KEY] [TARGET_NETWORK_PORT] [TARGET_NETWORK_PRIVATE_KEY] [DVN_ADDRESS]
 * 
 */

import { Hex, parseEther } from 'viem';
import { loadContract } from '../../src/utils/artifact';
import { ProviderWrapper, initSingletonProvider } from '../../src/utils/provider';
import { awaitTransaction } from '../../src/utils/evm';

async function main(argv: string[]) {
    const sourceRpcPort = argv.shift();
    const sourcePrivateKey = argv.shift() as Hex;
    const targetRpcPort = argv.shift();
    const targetPrivateKey = argv.shift() as Hex;
    const dvnAddress = argv.shift() as Hex;

    const sourceProvider = new ProviderWrapper(sourcePrivateKey, `http://localhost:${sourceRpcPort}`);
    const targetProvider = new ProviderWrapper(targetPrivateKey, `http://localhost:${targetRpcPort}`);
    const { abi, bytecode } = await loadContract('Test');

    initSingletonProvider(sourceProvider);

    // Deploy contracts
    await awaitTransaction(
        sourceProvider.wallet.deployContract({ abi, bytecode })
    );

    // Setup funds for DVN
    await awaitTransaction(
        sourceProvider.wallet.sendTransaction({
            to: dvnAddress,
            value: parseEther("500")
        })
    );

    initSingletonProvider(targetProvider);

    // Deploy contracts
    await awaitTransaction(
        targetProvider.wallet.deployContract({ abi, bytecode })
    );

    // Setup funds for DVN
    await awaitTransaction(
        targetProvider.wallet.sendTransaction({
            to: dvnAddress,
            value: parseEther("500")
        })
    );
}

const argv = process.argv;
argv.shift();
argv.shift();

main(argv);
