import { Hex } from "viem";
import { ProviderWrapper } from "../../../src/utils/provider";
import assert from "assert";

export function getProvider(name: string) {
    name = name.toUpperCase();

    const adminPrivateKey = process.env.ADMIN_PRIVATE_KEY as Hex;
    const rpcUrl = process.env[name + "_RPC_URL"] as string;
    const chainId = parseInt(process.env[name + "_CHAIN_ID"] as string);
    const eid = parseInt(process.env[name + "_ENDPOINT_ID"] as string);
    const endpoint = process.env[name + "_ENDPOINT_ADDRESS"] as Hex;
    const dvnAddress = process.env[name + "_DVN_ADDRESS"] as Hex;
    const oappAddress = process.env[name + "_OAPP_ADDRESS"] as Hex;
    
    assert(adminPrivateKey);
    assert(rpcUrl);
    assert(chainId);
    assert(eid);
    assert(endpoint);

    const provider = new ProviderWrapper(adminPrivateKey, rpcUrl, chainId, endpoint, eid);
    if ( dvnAddress ) provider.dvn = dvnAddress;
    if ( oappAddress ) provider.mockApp = oappAddress;

    return provider;
}