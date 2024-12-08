use ethers_core::{abi::Address, types::BlockNumber};
use crate::{contracts::DVN, gas::GasManager, nonce::NonceManager, provider::Provider, signer::Signer, state::GlobalState, task::Task, transaction::Transaction};
use std::{rc::Rc, str::FromStr};

pub struct ChainState {
    pub chain_id: u64,
    pub endpoint_id: u64,
    provider: Provider,
    dvn: DVN,
    gas: GasManager,
    nonce: NonceManager,
    signer: Rc<Signer>,
    last_processed_block: u64
}

impl ChainState {
    pub fn new(rpc_url: &str, chain_id: u64, endpoint_id: u64, dvn_address: &str) -> Self {
        let provider = Provider::new(rpc_url.to_string());
        let dvn = DVN::new(Address::from_str(dvn_address).unwrap(), chain_id);
        let gas = GasManager::new();
        let nonce = NonceManager::new();
        let signer = GlobalState::signer();

        Self {
            chain_id,
            endpoint_id,
            provider,
            dvn,
            gas,
            nonce,
            signer,
            last_processed_block: 0
        }
    }

    pub async fn init(&mut self) {
        let current_block = self.provider.block_number().await;
        let address = self.signer.address();
        let nonce = self.provider.nonce(&address).await;
        let gas = self.provider.gas().await;
        
        self.last_processed_block = current_block;
        self.nonce.update(nonce).await;
        self.gas.current_fees(gas);
    }

    pub async fn check_for_tasks(&mut self) -> Vec<Task> {
        let from_block = self.last_processed_block;
        let to_block = self.provider.block_number().await;

        let tasks_filter = self.dvn.jobs_filter(
            BlockNumber::Number(from_block.into()),
            BlockNumber::Number(to_block.into())
        );

        let raw_logs = self.provider.logs(tasks_filter).await;
        let tasks: Vec<Task> = raw_logs.into_iter().map(|log| self.dvn.jobs_parse(log)).collect();        
        self.gas.current_fees(self.provider.gas().await);
        self.last_processed_block = to_block;

        tasks
    }

    pub async fn process_task(&mut self, tasks: Vec<Task>) -> Vec<String> {
        let mut txn_hashes = vec![];

        for task in tasks.into_iter() {
            let nonce = self.nonce.nonce().await;
            let exec_config = self.dvn.verify_config(task);

            let mut txn = Transaction::new(exec_config);
            txn.contract(&self.dvn);
            txn.gas(&self.gas);
            txn.nonce(&nonce);
            txn.signer(&self.signer);
            let raw_txn = txn.sign(&self.signer).await;

            let hash = self.provider.send(raw_txn).await;
            txn_hashes.push(hash);

            self.nonce.commit();
        }

        txn_hashes
    }
}