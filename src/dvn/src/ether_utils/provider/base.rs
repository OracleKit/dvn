use ic_cdk::api::management_canister::http_request::{http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::value::RawValue;

#[derive(Debug, Serialize)]
struct Request<'a, T: Serialize> {
    jsonrpc: &'a str,
    id: u32,
    method: &'a str,
    params: T
}

#[derive(Debug, Deserialize)]
struct ResponseError {
    code: u128,
    message: String
}

#[derive(Debug, Deserialize)]
struct Response<'a> {
    #[serde(borrow)]
    result: Option<&'a RawValue>,

    error: Option<ResponseError>
}

#[derive(Clone, Default)]
pub struct BaseProvider {
    url: String
}

impl BaseProvider {
    pub fn new(url: String) -> Self {
        Self {
            url
        }
    } 

    pub async fn request<T, R>(&self, method: &str, params: T) -> R
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
        }, 2_000_000_000).await.unwrap();

        let response: Response = serde_json::from_slice(&response.body).unwrap();
        serde_json::from_str(response.result.unwrap().get()).unwrap()
    }
}
