use std::borrow::Borrow;

use ethers_core::{types::{transaction::eip2718::TypedTransaction, Address, Bytes, Eip1559TransactionRequest, Filter, NameOrAddress, Signature}, utils::rlp::Rlp};
use pocket_ic::common::rest::{CanisterHttpMethod, CanisterHttpRequest};
use serde::Deserialize;
use serde_json::value::RawValue;

use super::{super::{ChainStateMachine, ChainStateMachineFactory}, RpcBatch, RpcRequest, RpcRequestData};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    jsonrpc: String,
    id: u64,
    method: String,
    params: Option<Box<RawValue>>
}

#[derive(Debug)]
pub enum RpcParseError {
    EmptyBatch,
    MissingParams,
    FoundParams,
    InvalidParams,
    InvalidRpcUrl,
    InvalidHttpMethod,
    InvalidBody,
    InvalidJsonRpcVersion,
    InvalidRpcMethod,
    InvalidRlpEncoding,
    InvalidTxnSignature,
    InvalidTxnNonce,
    InvalidTxnChainId,
    InvalidTxnSender,
    InvalidTxnDestination,
    NonZeroTxnValue,
}

fn parse_txn(raw_txn: &Bytes, state_machine: &ChainStateMachine) -> Result<(Eip1559TransactionRequest, Signature), RpcParseError> {
    let rlp = Rlp::new(raw_txn.as_ref());

    let (txn, signature) = 
        Eip1559TransactionRequest::decode_signed_rlp(&rlp).map_err(|_| RpcParseError::InvalidRlpEncoding)?;
    
    let sighash = TypedTransaction::Eip1559(txn.clone()).sighash();
    let signer = signature.recover(sighash).map_err(|_| RpcParseError::InvalidTxnSignature)?;

    if !txn.from.is_some_and(|from| from == signer) {
        return Err(RpcParseError::InvalidTxnSignature);
    }

    if txn.nonce == Some(state_machine.transaction_count()) {
        return Err(RpcParseError::InvalidTxnNonce);
    }

    if txn.chain_id != Some(state_machine.chain_id().into()) {
        return Err(RpcParseError::InvalidTxnChainId);
    }

    if txn.from != Some(state_machine.sender()) {
        return Err(RpcParseError::InvalidTxnSender);
    }

    if txn.to != Some(NameOrAddress::Address(state_machine.contract())) {
        return Err(RpcParseError::InvalidTxnDestination);
    }

    if txn.value != Some(0.into()) && txn.value != None {
        return Err(RpcParseError::NonZeroTxnValue);
    }

    Ok((txn, signature))
}

fn parse_rpc_request(request: &Request, state_machine: &ChainStateMachine) -> Result<RpcRequest, RpcParseError> {
    if request.jsonrpc != "2.0" {
        return Err(RpcParseError::InvalidJsonRpcVersion);
    }

    let request_data = match request.method.as_str() {
        "eth_blockNumber" => {
            if request.params.is_some() {
                return Err(RpcParseError::FoundParams);
            }

            RpcRequestData::BlockNumber
        },

        "eth_chainId" => {
            if request.params.is_some() {
                return Err(RpcParseError::FoundParams);
            }
            
            RpcRequestData::ChainId
        },

        "eth_getTransactionCount" => {
            if let Some(params) = &request.params {
                let Ok(parsed_params) = serde_json::from_str::<(Address, String)>(params.get()) else {
                    return Err(RpcParseError::InvalidParams);
                };

                if parsed_params.0 != state_machine.sender() || parsed_params.1 != "latest" {
                    return Err(RpcParseError::InvalidParams);
                }

                RpcRequestData::GetTransactionCount
            } else {
                return Err(RpcParseError::FoundParams);
            }
        },

        "eth_gasPrice" => {
            if request.params.is_some() {
                return Err(RpcParseError::FoundParams);
            }
            
            RpcRequestData::GasPrice
        },

        "eth_maxPriorityFeePerGas" => {
            if request.params.is_some() {
                return Err(RpcParseError::FoundParams);
            }
            
            RpcRequestData::MaxPriorityFeePerGas
        },

        "eth_getLogs" => {
            if let Some(params) = &request.params {
                if let Ok(parsed_params) = serde_json::from_str::<Vec<Filter>>(params.get()) {
                    if parsed_params.len() != 1 { return Err(RpcParseError::InvalidParams); }

                    RpcRequestData::GetLogs(parsed_params[0].clone())
                } else {
                    return Err(RpcParseError::InvalidParams);
                }
            } else {
                return Err(RpcParseError::MissingParams);
            }
        },
        
        "eth_sendRawTransaction" => {
            if let Some(params) = &request.params {
                if let Ok(parsed_params) = serde_json::from_str::<Vec<Bytes>>(params.get()) {
                    if parsed_params.len() != 1 { return Err(RpcParseError::InvalidParams); }

                    let txn = parse_txn(&parsed_params[0], state_machine)?;
                    RpcRequestData::SendRawTransaction(txn)
                } else {
                    return Err(RpcParseError::InvalidParams);
                }
            } else {
                return Err(RpcParseError::MissingParams);
            }
        },
        
        _ => return Err(RpcParseError::InvalidRpcMethod),
    };

    return Ok(RpcRequest {
        id: request.id,
        data: request_data
    });
}

pub fn parse_rpc_batch(http_request: &CanisterHttpRequest, state_machine_factory: &ChainStateMachineFactory) -> Result<RpcBatch, RpcParseError> {
    let id = http_request.request_id;
    let url = &http_request.url;
    let method = &http_request.http_method;
    let data = &http_request.body;
    let is_batch: bool;

    let Some(state_machine) = state_machine_factory.get(&url) else {
        return Err(RpcParseError::InvalidRpcUrl);
    };

    if method != &CanisterHttpMethod::POST {
        return Err(RpcParseError::InvalidHttpMethod);
    }

    let requests: Vec<Request>;

    if let Ok(result) = serde_json::from_slice::<Request>(data.borrow()) {
        is_batch = false;
        requests = vec![result];
    } else if let Ok(result) = serde_json::from_slice::<Vec<Request>>(data.borrow()) {
        if result.len() == 0 {
            return Err(RpcParseError::EmptyBatch);
        }

        is_batch = true;
        requests = result;
    } else {
        return Err(RpcParseError::InvalidBody);
    }

    let mut parsed_requests = vec![];
    for request in requests.into_iter() {
        let parsed_request = parse_rpc_request(&request, state_machine)?;
        parsed_requests.push(parsed_request);
    }

    Ok(RpcBatch {
        request_id: id,
        url: url.clone(),
        is_batch: is_batch,
        data: parsed_requests
    })
}