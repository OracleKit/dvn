import { readFile } from "fs/promises";
import path from "path";
import { DVN$Type } from "../../artifacts/src/contracts/DVN.sol/DVN";
import { MockOAppReceiver$Type } from "../../artifacts/src/contracts/MockOAppReceiver.sol/MockOAppReceiver";
import { MockOAppSender$Type } from "../../artifacts/src/contracts/MockOAppSender.sol/MockOAppSender";
import { Test$Type } from "../../artifacts/src/contracts/Test.sol/Test";

export type ContractArtifacts = {
    DVN: DVN$Type,
    MockOAppReceiver: MockOAppReceiver$Type,
    MockOAppSender: MockOAppSender$Type,
    Test: Test$Type,
};

export type Contracts = keyof ContractArtifacts;

export async function loadContract<T extends keyof ContractArtifacts>(contract: T): Promise<ContractArtifacts[T]> {
    const artifactPath = path.resolve(`artifacts/src/contracts/${contract}.sol/${contract}.json`);
    const rawArtifact = await readFile(artifactPath, { encoding: 'utf-8' });
    return JSON.parse(rawArtifact) as ContractArtifacts[T];
}