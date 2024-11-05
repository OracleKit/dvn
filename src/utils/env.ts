import { Hex } from "viem";

export type ChainConfig = {
    rpcUrl: string,
    rpcSslUrl: string,
    chainId: number,
    endpointId: number,
    endpoint: Hex,
    dvn: Hex | undefined
};

export type AdminConfig = {
    address: Hex,
    privateKey: Hex
};

export function getChainConfig(name: string): ChainConfig {
    const nameUpper = name.toUpperCase();
    const rpcUrl = process.env[nameUpper + "_RPC_URL"]!;
    const rpcSslUrl = process.env[nameUpper + "_RPC_SSL_URL"]!;
    const chainId = parseInt(process.env[nameUpper + "_CHAIN_ID"]!);
    const endpointId = parseInt(process.env[nameUpper + "_ENDPOINT_ID"]!);
    const endpoint = process.env[nameUpper + "_ENDPOINT_ADDRESS"]! as Hex;
    const dvn = process.env[nameUpper + "_DVN_ADDRESS"]! as Hex;

    if ( rpcUrl && rpcSslUrl && chainId && endpointId && endpoint ) {
        return { rpcUrl, rpcSslUrl, chainId, endpoint, endpointId, dvn }
    }

    throw new Error("Chain config not present");
}

export function getAdminConfig(): AdminConfig {
    const address = process.env.ADMIN_ADDRESS! as Hex;
    const privateKey = process.env.ADMIN_PRIVATE_KEY! as Hex;
    if ( address && privateKey ) return { address, privateKey };

    throw new Error("Admin config not present");
}