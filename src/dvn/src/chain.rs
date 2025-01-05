use ethers_core::{abi::Address, types::BlockNumber};
use crate::{consensus::Consensus, contracts::DVN, gas::GasManager, nonce::NonceManager, provider::{Provider, Receipt}, signer::Signer, state::GlobalState, task::Task, transaction::Transaction};
use std::{rc::Rc, str::FromStr, time::Duration};

#[allow(dead_code)]
pub struct ChainState {
    pub chain_id: u64,
    pub endpoint_id: u64,
    dvn: DVN,
    gas: GasManager,
    nonce: NonceManager,
    consensus: Consensus,
    signer: Rc<Signer>,
    provider: Rc<Provider>,
    last_processed_block: u64,
    last_consensed_block: u64
}

impl ChainState {
    pub fn new(rpc_urls: Vec<String>, chain_id: u64, endpoint_id: u64, dvn_address: &str) -> Self {
        let provider = Rc::new(Provider::new(rpc_urls));
        let dvn = DVN::new(Address::from_str(dvn_address).unwrap(), chain_id);
        let gas = GasManager::new();
        let nonce = NonceManager::new();
        let consensus = Consensus::new();
        let signer = GlobalState::signer();

        Self {
            chain_id,
            endpoint_id,
            provider,
            dvn,
            gas,
            nonce,
            consensus,
            signer,
            last_processed_block: 0,
            last_consensed_block: 0
        }
    }

    pub async fn init(&mut self) {
        let address = self.signer.address();
        let current_block_receipt = self.provider.block_number();
        let nonce_receipt = self.provider.nonce(&address);
        let gas_receipt = self.provider.gas();

        self.provider.commit().await;
        
        self.last_consensed_block = current_block_receipt.take().as_u64() - 1;
        self.last_processed_block = self.last_consensed_block;
        self.nonce.update(nonce_receipt.take()).await;
        self.gas.current_fees(gas_receipt.take());
    }

    pub async fn check_for_tasks(&mut self) -> Vec<Task> {
        let from_block = self.last_consensed_block + 1;

        let tasks_filter = self.dvn.jobs_filter(
            BlockNumber::Number(from_block.into()),
            BlockNumber::Latest
        );

        let logs_receipt = self.provider.logs(tasks_filter);
        let gas_receipt = self.provider.gas();
        let block_number_receipt = self.provider.block_number();

        self.provider.commit().await;

        let raw_logs = logs_receipt.take();
        let tasks: Vec<Task> = raw_logs.into_iter().map(|log| self.dvn.jobs_parse(log)).collect();
        let consensed_tasks=  self.consensus.consensus(tasks);
        
        self.gas.current_fees(gas_receipt.take());
        self.last_consensed_block = self.last_processed_block;
        self.last_processed_block = block_number_receipt.take().as_u64();

        consensed_tasks
    }

    pub async fn process_tasks(&mut self, tasks: Vec<Task>) {
        let mut txns = vec![];

        for task in tasks.into_iter() {
            let nonce = self.nonce.nonce();
            let exec_config = self.dvn.verify_config(&task);

            let mut txn = Transaction::new(exec_config, task);
            txn.contract(&self.dvn);
            txn.gas(&self.gas);
            txn.nonce(&nonce);
            txn.signer(&self.signer);
            
            txns.push(txn);
        }

        for txn in txns {
            let signer = Rc::clone(&self.signer);
            let provider = Rc::clone(&self.provider);

            ic_cdk_timers::set_timer(
                Duration::from_secs(0),
                || {
                    ic_cdk::spawn(async move {
                        let raw_txn = txn.sign(&signer).await;
                        provider.send(raw_txn);
                        provider.commit().await;
                    });
                }
            );
        }
    }
}