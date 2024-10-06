import { Chain, Hex } from "viem";
import { ProviderWrapper } from "../../../src/utils/provider";
import assert from "assert";
import { holesky, polygonAmoy, polygonZkEvm, polygonZkEvmCardona } from "viem/chains";

type ChainEnvDetails = {
    name: string,
    chain: Chain
}

const ChainNamesMap: Record<string, ChainEnvDetails> = {
    'polygonAmoy': { name: "POLYGONAMOY", chain: polygonAmoy },
    'polygonZkEvmCardona': { name: "POLYGONCARDONA", chain: polygonZkEvmCardona },
    'ethereumHolesky': { name: 'ETHEREUMHOLESKY', chain: holesky }
} as const;

export type ChainIds = keyof typeof ChainNamesMap;

export function isValidChainId(chainId: string): boolean {
    return !!ChainNamesMap[chainId as ChainIds];
}

export function getChain(chainId: ChainIds): ChainEnvDetails {
    return ChainNamesMap[chainId];
}

export function getProvider(chainId: ChainIds) {
    assert(isValidChainId(chainId), "Not a valid chain id");

    const { name, chain } = ChainNamesMap[chainId];
    const adminPrivateKey = process.env.ADMIN_PRIVATE_KEY as Hex;
    const rpcUrl = process.env[name + "_RPC_SSL_URL"] as string;
    const eid = parseInt(process.env[name + "_ENDPOINT_ID"] as string);
    const endpoint = process.env[name + "_ENDPOINT_ADDRESS"] as Hex;
    const dvnAddress = process.env[name + "_DVN_ADDRESS"] as Hex;
    const oappAddress = process.env[name + "_OAPP_ADDRESS"] as Hex;
    
    assert(adminPrivateKey);
    assert(rpcUrl);
    assert(eid);
    assert(endpoint);

    const provider = new ProviderWrapper(adminPrivateKey, rpcUrl, chain, endpoint, eid);
    if ( dvnAddress ) provider.dvn = dvnAddress;
    if ( oappAddress ) provider.mockApp = oappAddress;

    return provider;
}