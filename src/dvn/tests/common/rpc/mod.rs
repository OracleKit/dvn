mod parse;
mod process;

use ethers_core::types::{Eip1559TransactionRequest, Filter, Signature};
pub use parse::*;
use pocket_ic::{common::rest::MockCanisterHttpResponse, PocketIc};
pub use process::*;

use super::ChainStateMachineFactory;

#[derive(Clone, Debug)]
pub enum RpcRequestData {
    BlockNumber,
    ChainId,
    GetTransactionCount,
    GasPrice,
    MaxPriorityFeePerGas,
    GetLogs(Filter),
    SendRawTransaction((Eip1559TransactionRequest, Signature))
}

impl RpcRequestData {
    pub fn as_u64(&self) -> u64 {
        match self {
            RpcRequestData::BlockNumber => 0,
            RpcRequestData::ChainId => 1,
            RpcRequestData::GasPrice => 2,
            RpcRequestData::GetLogs(_) => 3,
            RpcRequestData::GetTransactionCount => 4,
            RpcRequestData::MaxPriorityFeePerGas => 5,
            RpcRequestData::SendRawTransaction(_) => 6
        }
    }
}

#[derive(Clone, Debug)]
pub struct RpcRequest {
    pub id: u64,
    pub data: RpcRequestData
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RpcBatch {
    pub request_id: u64,
    pub url: String,
    pub is_batch: bool,
    pub data: Vec<RpcRequest>
}

pub struct RequestCollection {
    requests: Vec<RpcBatch>
}

impl RequestCollection {
    pub fn new() -> Self {
        Self { requests: vec![] }
    }

    pub fn add_batch(&mut self, request: RpcBatch) {
        self.requests.push(request);
    }

    pub fn filter_by_rpc(&self, rpc_url: &str) -> Vec<&RpcBatch> {
        self.requests.iter().filter(|&request| &request.url == rpc_url).collect()
    }
}

pub fn rpc_request_loop(pic: &PocketIc, state_machine_factory: &mut ChainStateMachineFactory) -> Result<RequestCollection, RpcParseError>  {
    let mut request_collection = RequestCollection::new();
    
    loop {
        pic.tick(); pic.tick();

        let requests = pic.get_canister_http();
        if requests.len() == 0 { break; }
        
        for request in requests {
            let parsed_request = parse_rpc_batch(&request, state_machine_factory)?;
            let response = process_rpc_batch(&parsed_request, state_machine_factory);
            request_collection.add_batch(parsed_request);

            pic.mock_canister_http_response(MockCanisterHttpResponse {
                subnet_id: request.subnet_id,
                request_id: request.request_id,
                response,
                additional_responses: vec![]
            });
        }

        pic.tick(); pic.tick();
    }

    return Ok(request_collection);
}