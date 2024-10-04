mod base;
use base::BaseProvider;
use ethers_core::{abi::Address, types::{transaction::eip2718::TypedTransaction, Eip1559TransactionRequest, Filter, Log, U256}, utils::hex::ToHexExt};

use super::Signer;

#[derive(Clone, Default)]
pub struct Provider {
    base: BaseProvider,
    signer: Signer,
    chain: u64
}

impl Provider {
    pub fn new(url: String, chain: u64) -> Self {
        Self {
            base: BaseProvider::new(url),
            signer: Signer::new("dfx_test_key".to_string()),
            chain
        }
    }

    pub async fn init(&mut self) {
        self.signer.init().await;
    }

    pub async fn address(&self) -> Address {
        self.signer.address()
    }

    pub async fn get_balance(&self, account: &str) -> U256 {
        self.base.request("eth_getBalance", (account, "latest")).await
    }

    pub async fn get_nonce(&self, account: &str) -> U256 {
        self.base.request("eth_getTransactionCount", (account, "latest")).await
    }

    pub async fn get_current_priority_fees(&self) -> U256 {
        self.base.request("eth_maxPriorityFeePerGas", vec![] as Vec<u8>).await
    }

    pub async fn get_current_base_fees(&self) -> U256 {
        self.base.request("eth_gasPrice", vec![] as Vec<u8>).await
    }

    pub async fn send_transaction(&self, txn: Eip1559TransactionRequest) -> String {
        let mut txn = txn;
        let last_base_fee = self.get_current_base_fees().await;
        let last_priority_fee = self.get_current_priority_fees().await;
        
        let base_fee = last_base_fee.checked_mul(11.into()).unwrap();
        let base_fee = base_fee.checked_div(10.into()).unwrap();
        let priority_fee = last_priority_fee;
        let max_fee = base_fee.checked_add(priority_fee.clone()).unwrap();

        txn.max_fee_per_gas = Some(max_fee);
        txn.max_priority_fee_per_gas = Some(priority_fee);
        txn.gas = Some(300_000.into());
        txn.chain_id = Some(self.chain.into());
        txn.from = Some(self.signer.address());
        txn.nonce = Some(self.get_nonce(&self.signer.address().encode_hex_with_prefix()).await.into());

        let txn: TypedTransaction = txn.into();

        let signature = self.signer.sign_transaction(&txn).await;
        let raw_signed = txn.rlp_signed(&signature);
        ic_cdk::println!("{:?}", &raw_signed);
        self.base.request("eth_sendRawTransaction", (raw_signed, )).await
    }

    pub async fn get_logs(&self, filter: &Filter) -> Vec<Log> {
        self.base.request("eth_getLogs", [filter]).await
    }
}