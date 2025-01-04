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
struct BatchRequest {
    data: Option<Box<RawValue>>, // None when data gets collected
    inner_receipt: Option<InnerReceipt> // None when request is fulfilled
}

#[derive(Default)]
pub struct Batcher {
    max_response_bytes: u64,
    context: Vec<String>,
    requests: Vec<BatchRequest>
}

impl Batcher {
    pub fn new() -> Self {
        Self {
            max_response_bytes: 0,
            context: vec![],
            requests: vec![]
        }
    }

    // should be used in tandem with queue_request to predict the id
    pub fn next_id(&self) -> u64 {
        self.requests.len() as u64
    }

    pub fn queue_request<T: DeserializeOwned>(&mut self, data: Box<RawValue>, context: String, max_response_bytes: u64) -> (GenericReceipt<T>, u64) {
        let inner_receipt = InnerReceipt::new();
        let receipt: GenericReceipt<T> = inner_receipt.receipt();

        let request = BatchRequest {
            data: Some(data),
            inner_receipt: Some(inner_receipt)
        };

        self.requests.push(request);
        self.context.push(context);
        self.max_response_bytes += max_response_bytes;

        (receipt, self.requests.len() as u64 - 1)
    }

    pub fn collect_requests(&mut self) -> BatchRequestCollection {
        let requests = self.requests.drain(..).collect();
        let context = self.context.drain(..).collect();
        let max_response_bytes = self.max_response_bytes;
        self.max_response_bytes = 0;

        BatchRequestCollection::new(requests, context, max_response_bytes)
    }
}

pub struct BatchRequestCollection {
    max_response_bytes: u64,
    context: Vec<String>,
    requests: Vec<BatchRequest>
}

impl BatchRequestCollection {
    fn new(requests: Vec<BatchRequest>, context: Vec<String>, max_response_bytes: u64) -> Self {
        Self {
            max_response_bytes,
            context,
            requests
        }
    }

    pub fn data(&mut self) -> Vec<Box<RawValue>> {
        self.requests
            .iter_mut()
            .map(|request| request.data.take().expect("Data already collected from batch request"))
            .collect()
    }

    pub fn max_response_bytes(&self) -> u64 {
        self.max_response_bytes
    }

    pub fn context(&self) -> Vec<u8> {
        serde_json::to_vec(&self.context).unwrap()
    }

    pub fn fulfill(&mut self, id: u64, data: Box<RawValue>) {
        let inner_receipt = self.requests[id as usize].inner_receipt.take().expect("Request already fulfilled");
        inner_receipt.fulfill(data);
    }
}