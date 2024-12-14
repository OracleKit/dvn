import { readFile } from "fs/promises";
import path from "path";
import { DVN$Type } from "../../artifacts/src/contracts/src/DVN.sol/DVN";
import { MockOAppReceiver$Type } from "../../artifacts/src/contracts/src/MockOAppReceiver.sol/MockOAppReceiver";
import { MockOAppSender$Type } from "../../artifacts/src/contracts/src/MockOAppSender.sol/MockOAppSender";
import { MockOApp$Type } from "../../artifacts/src/contracts/src/MockOApp.sol/MockOApp";
import { Test$Type } from "../../artifacts/src/contracts/src/Test.sol/Test";
import { DVNProxy$Type } from "../../artifacts/src/contracts/src/DVNProxy.sol/DVNProxy";
import { ILayerZeroEndpointV2$Type } from "../../artifacts/@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol/ILayerZeroEndpointV2"
import { ReceiveUlnBase$Type } from "../../artifacts/@layerzerolabs/lz-evm-messagelib-v2/contracts/uln/ReceiveUlnBase.sol/ReceiveUlnBase"

export type ContractArtifacts = {
    DVN: DVN$Type,
    DVNProxy: DVNProxy$Type,
    MockOAppReceiver: MockOAppReceiver$Type,
    MockOAppSender: MockOAppSender$Type,
    MockOApp: MockOApp$Type,
    Test: Test$Type,
    ILayerZeroEndpointV2: ILayerZeroEndpointV2$Type,
    ReceiveUlnBase: ReceiveUlnBase$Type,
};

export const ContractArtifactsPaths = {
    DVN: "artifacts/src/contracts/src/DVN.sol/DVN.json",
    DVNProxy: "artifacts/src/contracts/src/DVNProxy.sol/DVNProxy.json",
    MockOAppReceiver: "artifacts/src/contracts/src/MockOAppReceiver.sol/MockOAppReceiver.json",
    MockOAppSender: "artifacts/src/contracts/src/MockOAppSender.sol/MockOAppSender.json",
    MockOApp: "artifacts/src/contracts/src/MockOApp.sol/MockOApp.json",
    Test: "artifacts/src/contracts/src/Test.sol/Test.json",
    ILayerZeroEndpointV2: "artifacts/@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol/ILayerZeroEndpointV2.json",
    ReceiveUlnBase: "artifacts/@layerzerolabs/lz-evm-messagelib-v2/contracts/uln/ReceiveUlnBase.sol/ReceiveUlnBase.json",
}

export type Contracts = keyof ContractArtifacts;

export async function loadContract<T extends keyof ContractArtifacts>(contract: T): Promise<ContractArtifacts[T]> {
    const artifactPath = path.resolve(ContractArtifactsPaths[contract]);
    const rawArtifact = await readFile(artifactPath, { encoding: 'utf-8' });
    return JSON.parse(rawArtifact) as ContractArtifacts[T];
}