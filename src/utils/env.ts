import { Hex } from "viem";

export type ChainConfig = {
    rpcUrl: string,
    chainId: number,
    endpointId: number,
    endpoint: Hex,
    messageLibs: Hex[],
    priceFeed: Hex,
    dvn: Hex | undefined,
    oapp: Hex | undefined,
};

export type AdminConfig = {
    address: Hex,
    privateKey: Hex
};

export function getChainConfig(name: string): ChainConfig {
    const nameUpper = name.toUpperCase();
    const rpcUrl = process.env[nameUpper + "_RPC_URL"]!;
    const chainId = parseInt(process.env[nameUpper + "_CHAIN_ID"]!);
    const endpointId = parseInt(process.env[nameUpper + "_ENDPOINT_ID"]!);
    const endpoint = process.env[nameUpper + "_ENDPOINT_ADDRESS"]! as Hex;
    const messageLibs = [
        process.env[nameUpper + "_MESSAGE_LIB_SEND_ULN_301"] as Hex,
        process.env[nameUpper + "_MESSAGE_LIB_SEND_ULN_302"] as Hex,
    ];
    const dvn = process.env[nameUpper + "_DVN_ADDRESS"]! as Hex;
    const oapp = process.env[nameUpper + "_OAPP_ADDRESS"]! as Hex;
    const priceFeed = process.env[nameUpper + "_PRICE_FEED"]! as Hex;

    if ( rpcUrl && chainId && endpointId && endpoint && messageLibs.length && priceFeed ) {
        return { rpcUrl, chainId, endpoint, endpointId, messageLibs, dvn, oapp, priceFeed }
    }

    throw new Error("Chain config not present");
}

export function getAdminConfig(): AdminConfig {
    const address = process.env.ADMIN_ADDRESS! as Hex;
    const privateKey = process.env.ADMIN_PRIVATE_KEY! as Hex;
    if ( address && privateKey ) return { address, privateKey };

    throw new Error("Admin config not present");
}