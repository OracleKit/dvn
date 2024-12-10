use ethers_core::abi::AbiEncode;
use pocket_ic::common::rest::{CanisterHttpReply, CanisterHttpResponse};
use serde::Serialize;
use serde_json::value::RawValue;

use crate::common::ChainStateMachine;

use super::{super::{utils::encode_hex, ChainStateMachineFactory}, RpcBatch, RpcRequest, RpcRequestData};

#[derive(Serialize)]
struct Response {
    jsonrpc: String,
    id: u64,
    result: Box<RawValue>
}

fn serialize_interm<T: Serialize>(val: &T) -> Box<RawValue> {
    let serialized = serde_json::to_string(val).unwrap();
    RawValue::from_string(serialized).unwrap()
}

fn process_rpc_request(request: &RpcRequest, state_machine: &mut ChainStateMachine) -> Box<RawValue> {
    let id = request.id;
    let data = &request.data;

    let result: Box<RawValue> = match data {
        RpcRequestData::BlockNumber => {
            let block_number = encode_hex(state_machine.block_number().as_u64());
            serialize_interm(&block_number)
        },
        RpcRequestData::ChainId => {
            let chain_id = encode_hex(state_machine.chain_id());
            serialize_interm(&chain_id)
        },
        RpcRequestData::GetTransactionCount => {
            let nonce = encode_hex(state_machine.transaction_count());
            serialize_interm(&nonce)
        },
        RpcRequestData::GasPrice => {
            let base_fees = state_machine.base_gas();
            let priority_fees = state_machine.priority_gas();
            let gas_price = base_fees.checked_add(priority_fees).unwrap();
            serialize_interm(&encode_hex(gas_price.as_u128()))
        },
        RpcRequestData::MaxPriorityFeePerGas => {
            let priority_fees = state_machine.priority_gas();
            serialize_interm(&encode_hex(priority_fees.as_u128()))
        },
        RpcRequestData::GetLogs(filter) => {
            let logs = state_machine.get_logs(filter);
            serialize_interm(&logs)
        },
        RpcRequestData::SendRawTransaction(txn) => {
            let hash = state_machine.transact(txn.clone());
            serialize_interm(&vec![hash.encode_hex()])
        }
    };

    serialize_interm(&Response {
        jsonrpc: "2.0".to_string(),
        id,
        result
    })
}

pub fn process_rpc_batch(batch: &RpcBatch, state_machine_factory: &mut ChainStateMachineFactory) -> CanisterHttpResponse {
    let state_machine = state_machine_factory.get_mut(&batch.url).unwrap();
    let mut results: Vec<Box<RawValue>> = vec![];

    for request in batch.data.iter() {
        let result = process_rpc_request(request, state_machine);
        results.push(result);
    }

    let response = if batch.is_batch {
        serde_json::to_vec(&vec![results]).unwrap()
    } else if results.len() == 1 {
        serde_json::to_vec(&results[0]).unwrap()
    } else {
        panic!("More than one requests while not sending batch.");
    };

    CanisterHttpResponse::CanisterHttpReply(CanisterHttpReply {
        status: 200,
        headers: vec![],
        body: response
    })
}
