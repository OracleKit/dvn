use common::{get_signer_address, rpc_request_loop, setup_pic, submit_init_call, submit_setup_chain_call, ChainStateMachineFactory};
use ethers_core::types::Address;
use std::str::FromStr;

mod common;

#[test]
fn test_hello_world() {
    let (pic, canister_id) = setup_pic();

    let msg_id = submit_init_call(&pic, canister_id);
    pic.tick(); pic.tick();
    pic.await_call(msg_id).unwrap();

    let address = get_signer_address(&pic, canister_id);

    let mut state_machine_factory = ChainStateMachineFactory::new(
        address,
        Address::from_str("0x8c9b2Efb7c64C394119270bfecE7f54763b958Ad").unwrap()
    );

    let chain = state_machine_factory.create();
    let rpc_url = chain.url().clone();
    let msg_id = submit_setup_chain_call(&pic, canister_id, &chain);

    let requests_collection = rpc_request_loop(&pic, &mut state_machine_factory).unwrap();
    let mut requests = requests_collection.filter_by_rpc(&rpc_url);
    requests.sort_by_key(|&request| request.data.as_u64());
    
    pic.await_call(msg_id).unwrap();

    assert_eq!(requests.len(), 4);
}