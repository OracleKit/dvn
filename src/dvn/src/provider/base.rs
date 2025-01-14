use std::{cell::RefCell, future::Future, ops::Deref};

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
    urls: Vec<String>,
    last_used_url: RefCell<usize>,
    batcher: RefCell<Batcher>,
    uid: String,
    idempotency_counter: RefCell<u128>
}

impl BaseProvider {
    pub fn new(urls: Vec<String>) -> Self {
        let mut hasher = blake3::Hasher::new();
        urls.iter().for_each(|url| { hasher.update(url.as_bytes()); });

        let current_cycles_balance = ic_cdk::api::canister_balance128().to_string();
        hasher.update(current_cycles_balance.as_bytes());

        let uid = hasher.finalize().to_string();

        Self {
            urls,
            last_used_url: 0.into(),
            batcher: RefCell::new(Batcher::new()),
            uid,
            idempotency_counter: RefCell::new(0)
        }
    }

    fn pick_rpc_url(&self) -> String {
        let mut last_used_url = self.last_used_url.borrow_mut();
        let current_url = (last_used_url.deref() + 1) % self.urls.len();
        *last_used_url = current_url;

        self.urls[current_url].clone()
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
        let idempotency_index = self.idempotency_counter.borrow().clone();
        *self.idempotency_counter.borrow_mut() += 1;
        
        async move {
            let serialized_requests = serde_json::to_vec(&requests.data()).unwrap();
            let transform_context = requests.context();
            let max_response_bytes = requests.max_response_bytes() + 1000;

            let (response, ) = http_request(CanisterHttpRequestArgument {
                url: self.pick_rpc_url(),
                max_response_bytes: Some(max_response_bytes),
                method: HttpMethod::POST,
                headers: vec![
                    HttpHeader {
                        name: "Content-Type".to_string(),
                        value: "application/json".to_string()
                    },
                    HttpHeader {
                        name: "X-Idempotency-Key".to_string(),
                        value: format!("{}-{}", self.uid.as_str(), idempotency_index)
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