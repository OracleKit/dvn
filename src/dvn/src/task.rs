use ethers_core::types::Bytes;

pub struct Task {
    src_chain: u64,
    dest_chain: u64,
    message: Bytes
}

impl Task {
    // TODO: Add parsing from log in dvn contract
    pub fn new(src_chain: u64, dest_chain: u64, message: Bytes) -> Self {
        Self { src_chain, dest_chain, message }
    }
}
