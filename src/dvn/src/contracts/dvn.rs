use std::rc::Rc;
use ethers_core::abi::{Address, Contract, RawTopicFilter, Token};
use crate::{ether_utils::Provider, include_abi};
use super::base::BaseContract;

thread_local! {
    static ABI: Rc<Contract> = Rc::new(include_abi!("./dvn.json"));
}

#[derive(Clone, Default)]
pub struct DVN {
    base: BaseContract
}

impl DVN {
    pub fn new(provider: Provider, address: Address) -> Self {
        Self {
            base: BaseContract::new(provider, address, ABI.with(Rc::clone))
        }
    }

    pub async fn get_assigned_jobs(&self, from_block: u64) -> Vec<Token> {
        let logs = self.base.events("JobAssigned", RawTopicFilter {
            topic0: ethers_core::abi::Topic::Any,
            topic1: ethers_core::abi::Topic::Any,
            topic2: ethers_core::abi::Topic::Any
        }, Some(from_block)).await;

        let mut jobs = vec![];
        for mut log in logs.into_iter() {
            jobs.push(log.params.remove(0).value);
        };

        jobs
    }

    pub async fn verify(&self, job: Token) -> String {
        let hash = self.base.write("verify", &[job]).await;
        hash
    }
}