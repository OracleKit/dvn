use std::{cell::RefCell, future::Future};

use ic_cdk::api::management_canister::http_request::{http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod};
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
#[derive(Debug, Deserialize)]
struct ResponseError {
    code: i128,
    message: String
}

#[derive(Debug, Deserialize)]
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

    #[allow(dead_code)]
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

        let (receipt, _) = self.batcher.borrow_mut().queue_request::<R>(payload, max_response_bytes);
        receipt
    }

    pub fn commit<'a>(&'a self) -> impl Future<Output = ()> + 'a {
        // should collect requests from batcher at the moment commit is issued
        let mut requests = self.batcher.borrow_mut().collect_requests();
        
        async move {
            let serialized_requests = serde_json::to_vec(&requests.data()).unwrap();
            let max_response_bytes = requests.max_response_bytes();

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
                transform: None
            }, 1_000_000_000).await.unwrap();
            
            let responses: Vec<Response> = serde_json::from_slice(&response.body).unwrap();
            for response in responses.into_iter() {
                requests.fulfill(response.id.into(), response.result.unwrap());
            }
        }
    }
}
