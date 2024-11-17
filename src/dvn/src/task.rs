use ethers_core::abi::Token;

pub struct Task {
    #[allow(dead_code)]
    pub src_chain: u64,
    pub dest_chain: u64,
    pub message: Token
}
