use std::{cell::RefCell, rc::Rc};

use serde::de::DeserializeOwned;
use serde_json::value::RawValue;

use super::receipt::GenericReceipt;

#[derive(Default)]
struct InnerReceipt {
    inner: Rc<RefCell<Option<Box<RawValue>>>>
}

impl InnerReceipt {
    fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(None))
        }
    }

    fn receipt<T: DeserializeOwned>(&self) -> GenericReceipt<T> {
        GenericReceipt::new(self.inner.clone())
    }

    fn fulfill(self, data: Box<RawValue>) {
        self.inner.replace(Some(data));
    }
}

#[derive(Default)]
struct Request {
    data: Box<RawValue>,
    inner_receipt: InnerReceipt
}

#[derive(Default)]
pub struct Batcher {
    requests: Vec<Request>
}

impl Batcher {
    pub fn new() -> Self {
        Self {
            requests: vec![]
        }
    }

    pub fn clear(&mut self) {
        self.requests.clear();
    }

    pub fn next_id(&self) -> u64 {
        self.requests.len() as u64
    }

    pub fn queue_request<T: DeserializeOwned>(&mut self, data: Box<RawValue>) -> GenericReceipt<T> {
        let request = Request {
            data,
            inner_receipt: InnerReceipt::new()
        };

        let receipt: GenericReceipt<T> = request.inner_receipt.receipt();
        self.requests.push(request);

        receipt
    }

    pub fn collect_requests(&self) -> Vec<Box<RawValue>> {
        self.requests
            .iter()
            .map(|request| request.data.clone())
            .collect()
    }

    pub fn fulfill_request(&mut self, request_id: u64, response: Box<RawValue>) {
        let request = std::mem::take(&mut self.requests[request_id as usize]);
        request.inner_receipt.fulfill(response);
    }
}