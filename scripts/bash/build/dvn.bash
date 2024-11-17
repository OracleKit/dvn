set -e

dfx build --check dvn
candid-extractor target/wasm32-unknown-unknown/release/dvn.wasm > src/dvn/dvn.did
dfx generate dvn