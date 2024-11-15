use ethers_core::types::{transaction::eip2718::TypedTransaction, Bytes, Eip1559TransactionRequest, U256};
use crate::{contracts::{FunctionExecConfig, DVN}, gas::GasManager, signer::Signer};

pub struct Transaction {
    txn: Eip1559TransactionRequest
}

impl Transaction {
    pub fn new(config: FunctionExecConfig) -> Self {
        let txn = 
            Eip1559TransactionRequest::new()
                .data(config.data)
                .gas(config.gas);

        Self { txn }
    }

    pub fn contract(&mut self, contract: &DVN) {
        self.txn.chain_id = Some(contract.chain.into());
        self.txn.to = Some(ethers_core::types::NameOrAddress::Address(contract.address));
    }

    pub fn gas(&mut self, gas_manager: &GasManager) {
        let config = gas_manager.predicted_fees();
        self.txn.max_fee_per_gas = Some(config.max_fees);
        self.txn.max_priority_fee_per_gas = Some(config.max_priority_fees);
    }

    pub fn nonce(&mut self, nonce: &U256) {
        self.txn.nonce = Some(nonce.clone());
    }

    pub fn signer(&mut self, signer: &Signer) {
        self.txn.from = Some(signer.address());
    }

    pub async fn sign(self, signer: &Signer) -> Bytes {
        let txn: TypedTransaction = self.txn.into();
        let signature = signer.sign_transaction(&txn).await;
        txn.rlp_signed(&signature)
    }
}