set -e

cargo build --release --target wasm32-unknown-unknown --package dvn
candid-extractor target/wasm32-unknown-unknown/release/dvn.wasm > src/dvn/dvn.did
dfx build --check dvn
dfx generate dvn