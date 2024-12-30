import { Address, Chain, createWalletClient, GetContractReturnType, Hex, http, publicActions } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { deployContract, getContract } from './evm';
import { getMockChain } from './chain';
import assert from 'assert';


export function getProvider(name: string) {
    name = name.toUpperCase();

    const adminPrivateKey = process.env.ADMIN_PRIVATE_KEY as Hex;
    const rpcUrl = process.env[name + "_RPC_URL"] as string;
    const chainId = parseInt(process.env[name + "_CHAIN_ID"] as string);
    const eid = parseInt(process.env[name + "_ENDPOINT_ID"] as string);
    const endpoint = process.env[name + "_ENDPOINT_ADDRESS"] as Hex;
    const dvnAddress = process.env[name + "_DVN_ADDRESS"] as Hex;
    const oappAddress = process.env[name + "_OAPP_ADDRESS"] as Hex;
    const messageLibs = [
        process.env[name + "_MESSAGE_LIB_SEND_ULN_301"],
        process.env[name + "_MESSAGE_LIB_SEND_ULN_302"]
    ] as Hex[];
    const priceFeed = process.env[name + "_PRICE_FEED"] as Hex;
    
    assert(adminPrivateKey);
    assert(rpcUrl);
    assert(chainId);
    assert(eid);
    assert(endpoint);
    assert(messageLibs[0]);
    assert(messageLibs[1]);
    assert(priceFeed);

    const provider = new ProviderWrapper(
        adminPrivateKey,
        rpcUrl,
        chainId,
        endpoint,
        eid,
        messageLibs,
        priceFeed
    );
    if ( dvnAddress ) provider.dvn = dvnAddress;
    if ( oappAddress ) provider.mockApp = oappAddress;

    return provider;
}

export class ProviderWrapper {
    private readonly _wallet;
    private _endpoint: Hex;
    private _eid: number;
    private _dvn: Hex | undefined;
    private _mockApp: Hex | undefined;
    private _messageLibs: Hex[];
    private _priceFeed: Hex;

    constructor(
        privateKey: Hex,
        rpcUrl: string,
        chain: Chain | number,
        endpoint: Hex,
        eid: number,
        messageLibs: Hex[],
        priceFeed: Hex
    ) {
        chain = (typeof chain === 'number' ? getMockChain(chain) : chain);

        this._wallet = createWalletClient({
            account: privateKeyToAccount(privateKey),
            chain,
            transport: http(rpcUrl)
        }).extend(publicActions);

        this._endpoint = endpoint;
        this._eid = eid;
        this._messageLibs = messageLibs;
        this._priceFeed = priceFeed;
    }

    get wallet() {
        return this._wallet;
    }

    get account() {
        return this._wallet.account;
    }

    get eid() {
        return this._eid;
    }

    get endpoint() {
        return this._endpoint;
    }

    get dvn() {
        return this._dvn;
    }

    get mockApp() {
        return this._mockApp;
    }

    get messageLibs() {
        return this._messageLibs;
    }

    get priceFeed() {
        return this._priceFeed;
    }

    set dvn( dvn: Hex | undefined ) {
        this._dvn = dvn;
    }

    set mockApp( mockApp: Hex | undefined ) {
        this._mockApp = mockApp;
    }
};
