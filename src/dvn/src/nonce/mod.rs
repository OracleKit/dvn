use ethers_core::types::U256;

#[derive(Default)]
pub struct NonceManager {
    nonce: U256
}

impl NonceManager {
    pub fn new() -> Self {
        Self {
            nonce: 0.into()
        }
    }

    pub async fn update(&mut self, nonce: U256) {
        self.nonce = nonce;
    }

    pub fn nonce(&mut self) -> U256 {
        let nonce = self.nonce.clone();
        self.nonce = self.nonce + 1;

        nonce
    }
}