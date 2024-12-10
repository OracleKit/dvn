mod parse;
mod process;

pub use parse::*;
use pocket_ic::{common::rest::MockCanisterHttpResponse, PocketIc};
pub use process::*;

use super::ChainStateMachineFactory;

pub struct RequestCollection {
    requests: Vec<ParsedRpcRequest>
}

impl RequestCollection {
    pub fn new() -> Self {
        Self { requests: vec![] }
    }

    pub fn add_request(&mut self, request: ParsedRpcRequest) {
        self.requests.push(request);
    }

    pub fn filter_by_rpc(&self, rpc_url: &str) -> Vec<&ParsedRpcRequest> {
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
            let parsed_request = parse_rpc_request(&request, state_machine_factory)?;
            request_collection.add_request(parsed_request.clone());

            let response = process_rpc_request(&parsed_request, state_machine_factory);
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