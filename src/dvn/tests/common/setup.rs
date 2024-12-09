use std::fs;

use candid::{decode_args, encode_args, Principal};
use ethers_core::{types::Address, utils::hex::ToHex};
use pocket_ic::{common::rest::RawMessageId, PocketIc, PocketIcBuilder, WasmResult};
use std::str::FromStr;
use super::ChainStateMachine;

const DVN_WASM: &str = "../../target/wasm32-unknown-unknown/release/dvn.wasm";

pub fn get_admin_principal() -> Principal {
    Principal::self_authenticating([0])
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

pub fn submit_setup_chain_call(pic: &PocketIc, canister: Principal, chain: &ChainStateMachine) -> RawMessageId {
    pic.submit_call(
        canister,
        get_admin_principal(),
        "add_chain",
        encode_args((
            chain.url(),
            chain.chain_id(),
            chain.endpoint_id(),
            chain.contract().as_bytes().encode_hex::<String>()
        )).unwrap()
    ).unwrap()
}

pub fn submit_init_call(pic: &PocketIc, canister: Principal) -> RawMessageId {
    pic.submit_call(
        canister,
        get_admin_principal(),
        "init",
        encode_args(()).unwrap()
    ).unwrap()
}

pub fn get_signer_address(pic: &PocketIc, canister: Principal) -> Address {
    let WasmResult::Reply(result) = pic.query_call(
        canister,
        get_admin_principal(),
        "address",
        encode_args(()).unwrap()
    ).unwrap() else {
        panic!("Invalid response from canister on calling 'address'");
    };

    let (address, ) = decode_args(result.as_slice()).unwrap();
    Address::from_str(address).unwrap()
}