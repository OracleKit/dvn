import { Address, Chain, createWalletClient, GetContractReturnType, Hex, http, publicActions } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { deployContract, getContract } from './evm';
import { getMockChain } from './chain';

export class ProviderWrapper {
    private readonly _wallet;
    private _endpoint: Hex;
    private _eid: number;
    private _dvn: Hex | undefined;
    private _mockApp: Hex | undefined;

    constructor(
        privateKey: Hex,
        rpcUrl: string,
        chain: Chain | number,
        endpoint: Hex,
        eid: number
    ) {
        chain = (typeof chain === 'number' ? getMockChain(chain) : chain);

        this._wallet = createWalletClient({
            account: privateKeyToAccount(privateKey),
            chain,
            transport: http(rpcUrl)
        }).extend(publicActions);

        this._endpoint = endpoint;
        this._eid = eid;
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

    set dvn( dvn: Hex | undefined ) {
        this._dvn = dvn;
    }

    set mockApp( mockApp: Hex | undefined ) {
        this._mockApp = mockApp;
    }

    async deployDVN() {
        if ( this._dvn ) throw new Error("DVN already deployed!");
        this._dvn = await deployContract(this, "DVN", [this._endpoint]);
    }

    async deployMockOApp() {
        if ( !this._dvn ) throw new Error("DVN not deployed!");
        if ( this._mockApp ) throw new Error("Already deployed oapp!");
        
        this._mockApp = await deployContract(this, "MockOApp", [this._endpoint, this._dvn]);
    }

    async setPeer(provider: ProviderWrapper) {
        if ( !this._mockApp || !provider.mockApp ) throw new Error("OApp not deployed");

        const oapp = await getContract(this, "MockOApp", this._mockApp);
        await this.wallet.waitForTransactionReceipt({
            hash: await oapp.write.initPeer([provider.eid, provider.mockApp!])
        });
    }
};
