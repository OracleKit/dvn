import { ContractConstructorArgs, Hex, getContract as getContractInstance } from "viem";
import { ProviderWrapper } from "./provider";
import { ContractArtifacts, Contracts, loadContract } from "./artifact";

type Artifact<T extends Contracts> = ContractArtifacts[T];
type ABI<T extends Contracts> = Artifact<T>['abi'];
type DeployArgs<T extends Contracts> = ContractConstructorArgs<ABI<T>>

export async function deployContract<T extends Contracts>(provider: ProviderWrapper, contract: T, args: DeployArgs<T>): Promise<Hex> {
    const artifact = await loadContract(contract);

    const receipt = await provider.awaitTransaction(
        provider.wallet.deployContract({
            abi: artifact.abi,
            bytecode: artifact.bytecode,
            args: args as any
        })
    );

    return receipt.contractAddress as Hex;
}

export async function getContract<T extends Contracts>(provider: ProviderWrapper, contract: T, address: Hex) {
    const { abi } = await loadContract(contract);
    const instance = getContractInstance({
        address,
        abi,
        client: {
            public: provider.public,
            wallet: provider.wallet
        }
    });

    return instance;
}