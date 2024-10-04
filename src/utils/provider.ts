import { Chain, createPublicClient, createWalletClient, Hash, Hex, http } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { deployContract, getContract } from './evm';

export class ProviderWrapper {
    private readonly _public;
    private readonly _wallet;
    private _endpoint: Hex;
    private _eid: number;
    private _dvn: Hex | undefined;
    private _mockApp: Hex | undefined;
    private _isSender: boolean;

    constructor(privateKey: Hex, rpcUrl: string, chain: Chain, endpoint: Hex, eid: number) {
        this._wallet = createWalletClient({
            account: privateKeyToAccount(privateKey),
            chain,
            transport: http(rpcUrl)
        });

        this._public = createPublicClient({
            transport: http(rpcUrl),
        });

        this._endpoint = endpoint;
        this._eid = eid;
        this._isSender = false;
    }

    get wallet() {
        return this._wallet;
    }

    get public() {
        return this._public;
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

    async awaitTransaction(hash: Promise<Hash>) {
        return this.public.waitForTransactionReceipt({
            hash: await hash
        })
    }

    async deployDVN() {
        this._dvn = await deployContract(this, 'DVN', [this.endpoint]);
    }

    async deployMockSender() {
        if ( !this._dvn ) throw new Error("DVN not deployed");
        if ( this._mockApp ) throw new Error("Already deployed oapp");
        
        this._mockApp = await deployContract(this, "MockOAppSender", [this._endpoint, this._dvn]);
        this._isSender = true;
    }

    async deployMockReceiver() {
        if ( !this._dvn ) throw new Error("DVN not deployed");
        if ( this._mockApp ) throw new Error("Already deployed oapp");

        this._mockApp = await deployContract(this, "MockOAppReceiver", [this._endpoint, this._dvn]);
        this._isSender = false;
    }

    async setPeer(provider: ProviderWrapper) {
        if ( !this._mockApp || !provider.mockApp ) throw new Error("OApp not deployed");

        const oapp = await getContract(this, this._isSender ? "MockOAppSender" : "MockOAppReceiver", this._mockApp);
        await this.awaitTransaction( oapp.write.initPeer([provider.eid, provider.mockApp!]) );
    }
};
