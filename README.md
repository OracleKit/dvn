# ICP LayerZero DVN

Decentralized LayerZero V2 DVN running on ICP network, supports SendUln30X and ReceiveUln30X message libraries. <br />
*Currently in active development.*

## Trust Model

The current implementation relies on multiple JSON-RPC providers accessed through idempotent proxies. The canister does a threshold-2 consensus between the RPC providers before approving a payload.

## Deployments

Deployed canister: [ccpzm-baaaa-aaaam-adzza-cai](https://dashboard.internetcomputer.org/canister/ccpzm-baaaa-aaaam-adzza-cai)

Supported chains and DVN contract addresses:
- Arbitrum One: [0x7f6fa7938ff66db7944af8ca326900fd62ed5862](https://arbiscan.io/address/0x7f6fa7938ff66db7944af8ca326900fd62ed5862)
- Polygon PoS Mainnet [0x7f6fa7938ff66db7944af8ca326900fd62ed5862](https://polygonscan.com/address/0x7f6fa7938ff66db7944af8ca326900fd62ed5862)
