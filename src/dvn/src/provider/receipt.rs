use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use ethers_core::types::U256;
use serde::de::DeserializeOwned;
use serde_json::value::RawValue;

use crate::gas::CurrentGasConfig;

pub trait Receipt<T> {
    fn take(self) -> T;
}

pub struct GenericReceipt<T: DeserializeOwned> {
    inner: Rc<RefCell<Option<Box<RawValue>>>>,
    inner_type: PhantomData<T>
}

impl<T: DeserializeOwned> Receipt<T> for GenericReceipt<T> {
    fn take(self) -> T {
        let data = self.inner.take().unwrap();
        serde_json::from_str(data.get()).unwrap()
    }
}

impl<T: DeserializeOwned> GenericReceipt<T> {
    pub fn new(inner: Rc<RefCell<Option<Box<RawValue>>>>) -> Self {
        Self {
            inner,
            inner_type: PhantomData::default()
        }
    }
}

pub struct CurrentGasConfigReceipt {
    priority_fees_receipt: GenericReceipt<U256>,
    base_fees_receipt: GenericReceipt<U256>
}

impl Receipt<CurrentGasConfig> for CurrentGasConfigReceipt {
    fn take(self) -> CurrentGasConfig {
        CurrentGasConfig {
            base_fees: self.base_fees_receipt.take(),
            priority_fees: self.priority_fees_receipt.take()
        }
    }
}

impl CurrentGasConfigReceipt {
    pub fn new(base_fees_receipt: GenericReceipt<U256>, priority_fees_receipt: GenericReceipt<U256>) -> Self {
        Self {
            base_fees_receipt,
            priority_fees_receipt
        }
    }
}