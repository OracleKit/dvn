use ethers_core::abi::AbiEncode;
use pocket_ic::common::rest::{CanisterHttpReply, CanisterHttpResponse};
use serde::Serialize;
use serde_json::value::RawValue;

use super::{super::{utils::encode_hex, ChainStateMachineFactory}, ParsedRpcRequest, RpcRequest};

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

pub fn process_rpc_request(req: &ParsedRpcRequest, state_machine_factory: &mut ChainStateMachineFactory) -> CanisterHttpResponse {
    let state_machine = state_machine_factory.get_mut(&req.url).unwrap();
    let result: Box<RawValue>;

    match &req.data {
        RpcRequest::BlockNumber => {
            let block_number = encode_hex(state_machine.block_number().as_u64());
            result = serialize_interm(&block_number);
        },
        RpcRequest::ChainId => {
            let chain_id = encode_hex(state_machine.chain_id());
            result = serialize_interm(&chain_id);
        },
        RpcRequest::GetTransactionCount => {
            let nonce = encode_hex(state_machine.transaction_count());
            result = serialize_interm(&nonce);
        },
        RpcRequest::GasPrice => {
            let base_fees = state_machine.base_gas();
            let priority_fees = state_machine.priority_gas();
            let gas_price = base_fees.checked_add(priority_fees).unwrap();
            result = serialize_interm(&encode_hex(gas_price.as_u128()));
        },
        RpcRequest::MaxPriorityFeePerGas => {
            let priority_fees = state_machine.priority_gas();
            result = serialize_interm(&encode_hex(priority_fees.as_u128()));
        },
        RpcRequest::GetLogs(filter) => {
            let logs = state_machine.get_logs(filter);
            result = serialize_interm(&logs);
        },
        RpcRequest::SendRawTransaction(txn) => {
            let hash = state_machine.transact(txn.clone());
            result = serialize_interm(&vec![hash.encode_hex()]);
        }
    };

    let serialized_response = serde_json::to_vec(&Response {
        jsonrpc: "2.0".into(),
        id: req.rpc_id,
        result
    }).unwrap();

    CanisterHttpResponse::CanisterHttpReply(CanisterHttpReply {
        status: 200,
        headers: vec![],
        body: serialized_response
    })
}
