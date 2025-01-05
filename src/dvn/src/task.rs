use std::hash::Hash;

use ethers_core::{abi::Token, types::U256};

pub struct Task {
    #[allow(dead_code)]
    pub src_chain: u64,
    pub dest_chain: u64,
    pub gas: U256,
    pub message: Token
}

fn hash_token<H: std::hash::Hasher>(token: &Token, state: &mut H) {
    let serialized = serde_json::to_vec(token).unwrap();
    serialized.hash(state);
}

impl Hash for Task {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.src_chain.hash(state);
        self.dest_chain.hash(state);
        self.gas.hash(state);
        hash_token(&self.message, state);
    }
}