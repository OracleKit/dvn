use candid::encode_args;
use common::{encode_add_chain_args, get_admin_principal, get_alice_principal, setup_pic};
use ethers_core::types::Address;
use pocket_ic::WasmResult;

mod common;

#[test]
fn reject_call_if_caller_is_not_controller() {
    let (pic, canister_id) = setup_pic();
    let admin_principal = get_admin_principal();
    let alice_principal = get_alice_principal();

    assert_ne!(admin_principal.to_string(), alice_principal.to_string());

    // init
    let result = pic.update_call(
        canister_id,
        alice_principal,
        "init",
        encode_args(()).unwrap()
    ).unwrap();

    match result {
        WasmResult::Reject(e) => assert_eq!(&e, "Caller not a controller, unauthorized"),
        _ => panic!("Canister didn't reject")
    }

    // add_chain
    let result = pic.update_call(
        canister_id,
        alice_principal,
        "add_chain",
        encode_add_chain_args("", 0, 0, Address::random())
    ).unwrap();

    match result {
        WasmResult::Reject(e) => assert_eq!(&e, "Caller not a controller, unauthorized"),
        _ => panic!("Canister didn't reject")
    }

    // process_tasks
    let result = pic.update_call(
        canister_id,
        alice_principal,
        "process_tasks",
        encode_args(()).unwrap()
    ).unwrap();

    match result {
        WasmResult::Reject(e) => assert_eq!(&e, "Caller not a controller, unauthorized"),
        _ => panic!("Canister didn't reject")
    }

    // address
    let result = pic.query_call(
        canister_id,
        alice_principal,
        "address",
        encode_args(()).unwrap()
    ).unwrap();

    match result {
        WasmResult::Reject(e) => assert_eq!(&e, "Caller not a controller, unauthorized"),
        _ => panic!("Canister didn't reject")
    }
}