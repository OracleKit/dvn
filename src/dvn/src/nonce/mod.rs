use std::cell::RefCell;
use ethers_core::types::U256;

#[derive(Default)]
pub struct NonceManager {
    nonce: RefCell<U256>
}

impl NonceManager {
    pub fn new() -> Self {
        Self {
            nonce: RefCell::new(0.into())
        }
    }

    pub async fn update(&self, nonce: U256) {
        self.nonce.replace(nonce);
    }

    pub fn nonce(&mut self) -> U256 {
        let nonce = self.nonce.borrow().clone();
        self.nonce.replace(nonce + 1);

        nonce
    }
}