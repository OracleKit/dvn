use std::borrow::Borrow;

use ethers_core::{types::{transaction::eip2718::TypedTransaction, Address, Bytes, Eip1559TransactionRequest, Filter, NameOrAddress, Signature}, utils::rlp::Rlp};
use pocket_ic::common::rest::{CanisterHttpMethod, CanisterHttpRequest};
use serde::Deserialize;
use serde_json::value::RawValue;

use super::super::{ChainStateMachine, ChainStateMachineFactory};


#[derive(Deserialize, Clone)]
pub enum RpcRequest {
    BlockNumber,
    ChainId,
    GetTransactionCount,
    GasPrice,
    MaxPriorityFeePerGas,
    GetLogs(Filter),
    SendRawTransaction((Eip1559TransactionRequest, Signature))
}

impl RpcRequest {
    pub fn as_u64(&self) -> u64 {
        match self {
            RpcRequest::BlockNumber => 0,
            RpcRequest::ChainId => 1,
            RpcRequest::GasPrice => 2,
            RpcRequest::GetLogs(_) => 3,
            RpcRequest::GetTransactionCount => 4,
            RpcRequest::MaxPriorityFeePerGas => 5,
            RpcRequest::SendRawTransaction(_) => 6
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct ParsedRpcRequest {
    pub id: u64,
    pub url: String,
    pub rpc_id: u64,
    pub data: RpcRequest
}

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

pub fn parse_rpc_request(req: &CanisterHttpRequest, state_machine_factory: &ChainStateMachineFactory) -> Result<ParsedRpcRequest, RpcParseError> {
    let id = req.request_id;
    let url = &req.url;
    let method = &req.http_method;
    let data = &req.body;

    let Some(state_machine) =  state_machine_factory.get(&url) else {
        return Err(RpcParseError::InvalidRpcUrl);
    };

    if method != &CanisterHttpMethod::POST {
        return Err(RpcParseError::InvalidHttpMethod);
    }

    let parsed_req: Request = 
        serde_json::from_slice(data.borrow()).map_err(|_| RpcParseError::InvalidBody)?;

    if parsed_req.jsonrpc != "2.0" {
        return Err(RpcParseError::InvalidJsonRpcVersion);
    }

    let rpc_data = match parsed_req.method.borrow() {
        "eth_blockNumber" => {
            if parsed_req.params.is_some() {
                return Err(RpcParseError::FoundParams);
            }

            RpcRequest::BlockNumber
        },

        "eth_chainId" => {
            if parsed_req.params.is_some() {
                return Err(RpcParseError::FoundParams);
            }
            
            RpcRequest::ChainId
        },

        "eth_getTransactionCount" => {
            if let Some(params) = parsed_req.params {
                let Ok(parsed_params) = serde_json::from_str::<(Address, String)>(params.get()) else {
                    return Err(RpcParseError::InvalidParams);
                };

                if parsed_params.0 != state_machine.sender() || parsed_params.1 != "latest" {
                    return Err(RpcParseError::InvalidParams);
                }

                RpcRequest::GetTransactionCount
            } else {
                return Err(RpcParseError::FoundParams);
            }
        },

        "eth_gasPrice" => {
            if parsed_req.params.is_some() {
                return Err(RpcParseError::FoundParams);
            }
            
            RpcRequest::GasPrice
        },

        "eth_maxPriorityFeePerGas" => {
            if parsed_req.params.is_some() {
                return Err(RpcParseError::FoundParams);
            }
            
            RpcRequest::MaxPriorityFeePerGas
        },

        "eth_getLogs" => {
            if let Some(params) = parsed_req.params {
                if let Ok(parsed_params) = serde_json::from_str::<Vec<Filter>>(params.get()) {
                    if parsed_params.len() != 1 { return Err(RpcParseError::InvalidParams); }

                    RpcRequest::GetLogs(parsed_params[0].clone())
                } else {
                    return Err(RpcParseError::InvalidParams);
                }
            } else {
                return Err(RpcParseError::MissingParams);
            }
        },
        
        "eth_sendRawTransaction" => {
            if let Some(params) = parsed_req.params {
                if let Ok(parsed_params) = serde_json::from_str::<Vec<Bytes>>(params.get()) {
                    if parsed_params.len() != 1 { return Err(RpcParseError::InvalidParams); }

                    let txn = parse_txn(&parsed_params[0], state_machine)?;
                    RpcRequest::SendRawTransaction(txn)
                } else {
                    return Err(RpcParseError::InvalidParams);
                }
            } else {
                return Err(RpcParseError::MissingParams);
            }
        },
        
        _ => return Err(RpcParseError::InvalidRpcMethod),
    };

    Ok(
        ParsedRpcRequest {
            id,
            url: url.clone(),
            rpc_id: parsed_req.id,
            data: rpc_data
        }
    )
}