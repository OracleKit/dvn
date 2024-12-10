mod parse;
mod process;

use ethers_core::types::{Eip1559TransactionRequest, Filter, Signature};
pub use parse::*;
use pocket_ic::{common::rest::MockCanisterHttpResponse, PocketIc};
pub use process::*;

use super::ChainStateMachineFactory;

#[derive(Clone)]
pub enum ParsedRpcRequestData {
    BlockNumber,
    ChainId,
    GetTransactionCount,
    GasPrice,
    MaxPriorityFeePerGas,
    GetLogs(Filter),
    SendRawTransaction((Eip1559TransactionRequest, Signature))
}

impl ParsedRpcRequestData {
    pub fn as_u64(&self) -> u64 {
        match self {
            ParsedRpcRequestData::BlockNumber => 0,
            ParsedRpcRequestData::ChainId => 1,
            ParsedRpcRequestData::GasPrice => 2,
            ParsedRpcRequestData::GetLogs(_) => 3,
            ParsedRpcRequestData::GetTransactionCount => 4,
            ParsedRpcRequestData::MaxPriorityFeePerGas => 5,
            ParsedRpcRequestData::SendRawTransaction(_) => 6
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct ParsedRpcBatch {
    pub request_id: u64,
    pub rpc_id: u64,
    pub url: String,
    pub data: Vec<ParsedRpcRequestData>
}

pub struct RequestCollection {
    requests: Vec<ParsedRpcBatch>
}

impl RequestCollection {
    pub fn new() -> Self {
        Self { requests: vec![] }
    }

    pub fn add_batch(&mut self, request: ParsedRpcBatch) {
        self.requests.push(request);
    }

    pub fn filter_by_rpc(&self, rpc_url: &str) -> Vec<&ParsedRpcBatch> {
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