import { readFile } from "fs/promises";
import path from "path";
import { DVN$Type } from "../../artifacts/src/contracts/src/DVN.sol/DVN";
import { MockOAppReceiver$Type } from "../../artifacts/src/contracts/src/MockOAppReceiver.sol/MockOAppReceiver";
import { MockOAppSender$Type } from "../../artifacts/src/contracts/src/MockOAppSender.sol/MockOAppSender";
import { MockOApp$Type } from "../../artifacts/src/contracts/src/MockOApp.sol/MockOApp";
import { Test$Type } from "../../artifacts/src/contracts/src/Test.sol/Test";
import { DVNProxy$Type } from "../../artifacts/src/contracts/src/DVNProxy.sol/DVNProxy";

export type ContractArtifacts = {
    DVN: DVN$Type,
    DVNProxy: DVNProxy$Type,
    MockOAppReceiver: MockOAppReceiver$Type,
    MockOAppSender: MockOAppSender$Type,
    MockOApp: MockOApp$Type,
    Test: Test$Type,
};

export type Contracts = keyof ContractArtifacts;

export async function loadContract<T extends keyof ContractArtifacts>(contract: T): Promise<ContractArtifacts[T]> {
    const artifactPath = path.resolve(`artifacts/src/contracts/src/${contract}.sol/${contract}.json`);
    const rawArtifact = await readFile(artifactPath, { encoding: 'utf-8' });
    return JSON.parse(rawArtifact) as ContractArtifacts[T];
}