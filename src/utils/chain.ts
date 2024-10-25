import { Chain } from "viem";

export function getMockChain(chainId: number): Chain {
    return {
        id: chainId,
        name: 'Mock Chain',
        nativeCurrency: {
            name: 'MCC',
            symbol: 'MCC',
            decimals: 18
        },
        rpcUrls: {
            default: {
                http: []
            }
        },
    };
}