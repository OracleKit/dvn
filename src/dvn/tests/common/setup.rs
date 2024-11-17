use std::fs;

use candid::Principal;
use pocket_ic::PocketIc;

const DVN_WASM: &str = "../../target/wasm32-unknown-unknown/release/dvn.wasm";

pub fn setup_pic() -> (PocketIc, Principal) {
    let pic = PocketIc::new();
    let canister = pic.create_canister();
    pic.add_cycles(canister, 2_000_000_000_000); // 2T Cycles
    let wasm = fs::read(DVN_WASM).expect("Wasm file not found.");
    pic.install_canister(canister, wasm, vec![], None);
    (pic, canister)
}