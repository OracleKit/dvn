use candid::{decode_args, encode_args};
use common::{encode_add_chain_args, get_admin_principal, rpc_request_loop, setup_pic, ChainStateMachineFactory};
use ethers_core::types::Address;
use pocket_ic::WasmResult;
use std::str::FromStr;

mod common;

#[test]
fn test_hello_world() {
    let (pic, canister_id) = setup_pic();

    pic.update_call(
        canister_id.clone(),
        get_admin_principal(),
        "init",
        encode_args(()).unwrap()
    ).unwrap();

    let WasmResult::Reply(address) = pic.query_call(
        canister_id.clone(),
        get_admin_principal(),
        "address",
        encode_args(()).unwrap()
    ).unwrap() else { panic!("Address query failed!") };

    let (address,) = decode_args::<(String,)>(&address).unwrap();
    let address = Address::from_str(&address).unwrap();

    let mut state_machine_factory = ChainStateMachineFactory::new(
        address,
        Address::from_str("0x8c9b2Efb7c64C394119270bfecE7f54763b958Ad").unwrap()
    );

    let chain = state_machine_factory.create();
    let rpc_url = chain.url().clone();
    let msg_id = pic.submit_call(
        canister_id,
        get_admin_principal(),
        "add_chain",
        encode_add_chain_args(
            vec![&chain.url()],
            chain.chain_id(),
            chain.endpoint_id(),
            chain.contract()
        )
    ).unwrap();

    let requests_collection = rpc_request_loop(&pic, &mut state_machine_factory).unwrap();
    let mut requests = requests_collection.filter_by_rpc(&rpc_url);
    requests.sort_by_key(|&request| request.data[0].data.as_u64());
    
    // TODO: Will panics be caught in cron jobs where we can't await calls?
    pic.await_call(msg_id).unwrap();

    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].data.len(), 4);
}