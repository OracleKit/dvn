use std::{cmp::min, fmt::LowerHex};
use ethers_core::types::{transaction::eip2718::TypedTransaction, BlockNumber, Eip1559TransactionRequest, Signature, H256, U256};

pub fn extract_block_number ( block_number: BlockNumber, start: U256, latest: U256 ) -> U256 {
    match block_number {
        BlockNumber::Latest => latest,
        BlockNumber::Earliest => start,
        BlockNumber::Finalized => min(latest - 5, start.into()),
        BlockNumber::Safe => min(latest - 5, start.into()),
        BlockNumber::Pending => latest,
        BlockNumber::Number(v) => v.as_u64().into(),
    }
}

pub fn get_txn_hash(txn: &Eip1559TransactionRequest, signature: &Signature) -> H256 {
    let typed_txn = TypedTransaction::Eip1559(txn.clone());
    typed_txn.hash(&signature)
}

pub fn encode_hex<T: LowerHex>(v: T) -> String {
    format!("{:#x}", v)
}