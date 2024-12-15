use std::fs;

use candid::{encode_args, Principal};
use ethers_core::{types::Address, utils::hex::ToHex};
use pocket_ic::{PocketIc, PocketIcBuilder};

const DVN_WASM: &str = "../../target/wasm32-unknown-unknown/release/dvn.wasm";

pub fn get_admin_principal() -> Principal {
    Principal::self_authenticating([0])
}

pub fn get_alice_principal() -> Principal {
    Principal::self_authenticating([1])
}

pub fn setup_pic() -> (PocketIc, Principal) {
    let pic = PocketIcBuilder::new().with_fiduciary_subnet().build();
    let fiduciary_subnet_id = pic.topology().get_fiduciary().unwrap();
    let sender = Some(get_admin_principal());
    let canister = pic.create_canister_on_subnet(sender, None, fiduciary_subnet_id);
    let wasm = fs::read(DVN_WASM).expect("Wasm file not found.");

    pic.add_cycles(canister, 2_000_000_000_000); // 2T Cycles
    pic.install_canister(canister, wasm, vec![], sender);
    
    (pic, canister)
}

pub fn encode_add_chain_args(rpc_url: &str, chain_id: u64, endpoint_id: u64, dvn_address: Address) -> Vec<u8> {
    encode_args((
        rpc_url,
        chain_id,
        endpoint_id,
        dvn_address.as_bytes().encode_hex::<String>()
    )).unwrap()
}