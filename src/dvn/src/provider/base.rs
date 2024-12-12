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
    code: u128,
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
    batcher: Batcher
}

impl BaseProvider {
    pub fn new(url: String) -> Self {
        Self {
            url,
            batcher: Batcher::new()
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

    pub fn issue_request<T, R>(&mut self, method: &str, params: Option<T>) -> GenericReceipt<R>
    where
        T: Serialize,
        R: DeserializeOwned
    {
        let request_id = self.batcher.next_id();
        let payload = &Request {
            jsonrpc: "2.0",
            id: request_id.try_into().unwrap(),
            method,
            params
        };
        let payload = serde_json::to_string(payload).unwrap();
        let payload = RawValue::from_string(payload).unwrap();

        let receipt: GenericReceipt<R> = self.batcher.queue_request(payload);
        receipt
    }

    pub async fn commit(&mut self) {
        let requests = self.batcher.collect_requests();
        let serialized_requests = serde_json::to_vec(&requests).unwrap();

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
            body: Some(serialized_requests),
            transform: None
        }, 80_000_000_000).await.unwrap();
        
        let responses: Vec<Response> = serde_json::from_slice(&response.body).unwrap();
        for response in responses.into_iter() {
            self.batcher.fulfill_request(response.id.into(), response.result.unwrap());
        }

        self.batcher.clear();
    }
}
