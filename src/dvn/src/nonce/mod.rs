use std::sync::Arc;
use ethers_core::types::U256;
use futures::lock::{Mutex, OwnedMutexGuard};

#[derive(Default)]
pub struct NonceManager {
    nonce: Arc<Mutex<U256>>,
    lock_guard: Option<OwnedMutexGuard<U256>>
}

impl NonceManager {
    pub fn new() -> Self {
        Self {
            nonce: Arc::new(Mutex::new(0.into())),
            lock_guard: None,
        }
    }

    pub async fn update(&mut self, nonce: U256) {
        *self.nonce.lock().await = nonce;
    }

    pub async fn nonce(&mut self) -> U256 {
        let nonce = Arc::clone(&self.nonce);
        let guard = nonce.lock_owned().await;
        let nonce = guard.clone();
        self.lock_guard = Some(guard);

        nonce
    }

    pub fn commit(&mut self) {
        let guard = self.lock_guard.as_mut().unwrap();
        guard.checked_add(1.into());
        
        self.lock_guard = None;
    }
}