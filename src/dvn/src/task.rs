use ethers_core::{abi::Token, types::U256};

pub struct Task {
    #[allow(dead_code)]
    pub src_chain: u64,
    pub dest_chain: u64,
    pub gas: U256,
    pub message: Token
}
