use std::{cell::RefCell, future::Future};

use ethers_core::types::transaction::response;
use ic_cdk::api::management_canister::http_request::{http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformContext};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::value::RawValue;

use super::{batch::Batcher, receipt::GenericReceipt};

#[derive(Debug, Serialize)]
struct Request<'a, T: Serialize> {
    jsonrpc: &'a str,
    id: u32,
    method: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<T>
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct ResponseError {
    code: i128,
    message: String
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
struct Response {
    id: u32,
    result: Option<Box<RawValue>>,
    error: Option<ResponseError>
}

#[derive(Default)]
pub struct BaseProvider {
    url: String,
    batcher: RefCell<Batcher>
}

impl BaseProvider {
    pub fn new(url: String) -> Self {
        Self {
            url,
            batcher: RefCell::new(Batcher::new())
        }
    } 

    pub async fn request<T, R>(&self, method: &str, params: Option<T>) -> R
        where
            T: Serialize,
            R: DeserializeOwned
    {
        let payload = serde_json::to_vec(&Request {
            jsonrpc: "2.0",
            id: 1,
            method,
            params
        }).unwrap();

        let (response, ) = http_request(CanisterHttpRequestArgument {
            url: self.url.clone(),
            max_response_bytes: None,
            method: HttpMethod::POST,
            headers: vec![
                HttpHeader {
                    name: "Content-Type".to_string(),
                    value: "application/json".to_string()
                }
            ],
            body: Some(payload),
            transform: None
        }, 80_000_000_000).await.unwrap();

        let response: Response = serde_json::from_slice(&response.body).unwrap();
        serde_json::from_str(response.result.unwrap().get()).unwrap()
    }

    pub fn issue_request<T, R>(&self, method: &str, params: Option<T>, max_response_bytes: u64) -> GenericReceipt<R>
    where
        T: Serialize,
        R: DeserializeOwned
    {
        let request_id = self.batcher.borrow().next_id();
        let payload = &Request {
            jsonrpc: "2.0",
            id: request_id.try_into().unwrap(),
            method,
            params
        };
        let payload = serde_json::to_string(payload).unwrap();
        let payload = RawValue::from_string(payload).unwrap();

        let (receipt, _) = self.batcher.borrow_mut().queue_request::<R>(payload, method.to_string(), max_response_bytes);
        receipt
    }

    pub fn commit<'a>(&'a self) -> impl Future<Output = ()> + 'a {
        // should collect requests from batcher at the moment commit is issued
        let mut requests = self.batcher.borrow_mut().collect_requests();
        
        async move {
            let serialized_requests = serde_json::to_vec(&requests.data()).unwrap();
            let transform_context = requests.context();
            let max_response_bytes = requests.max_response_bytes() + 500;

            let (response, ) = http_request(CanisterHttpRequestArgument {
                url: self.url.clone(),
                max_response_bytes: Some(max_response_bytes),
                method: HttpMethod::POST,
                headers: vec![
                    HttpHeader {
                        name: "Content-Type".to_string(),
                        value: "application/json".to_string()
                    }
                ],
                body: Some(serialized_requests),
                transform: Some(
                    TransformContext::from_name(
                        "transform_rpc".into(),
                        transform_context
                    )
                )
            }, 1_000_000_000).await.unwrap();
            
            let responses: Vec<Response> = serde_json::from_slice(&response.body).unwrap();
            for response in responses.into_iter() {
                requests.fulfill(response.id.into(), response.result.unwrap());
            }
        }
    }
}

pub fn transform_rpc(request: HttpResponse, context: Vec<u8>) -> HttpResponse {
    let methods: Vec<String> = serde_json::from_slice(&context).unwrap();
    let data = request.body;
    let parsed_data: Vec<Response> = serde_json::from_slice(&data).unwrap();

    let mut parsed_data: Vec<Response> = parsed_data
        .into_iter()
        .map(|mut response| -> Response {
            match methods[response.id as usize].as_str() {
                "eth_getLogs" => {
                    let result = response.result.as_ref().unwrap().get();
                    let mut logs: Vec<Box<RawValue>> = serde_json::from_str(result).unwrap();
                    logs.sort_by_key(|log| log.get().to_string());

                    response.result = Some(RawValue::from_string(serde_json::to_string(&logs).unwrap()).unwrap());
                    response
                },
                _ => response
            }
        })
        .collect();

    parsed_data.sort_by_key(|request| request.id);
    let data = serde_json::to_vec(&parsed_data).unwrap();

    HttpResponse {
        status: request.status.clone(),
        headers: vec![],
        body: data
    }
}