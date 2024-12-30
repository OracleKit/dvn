use ethers_core::{abi::{Address, Contract, RawLog, RawTopicFilter, Token}, types::{BlockNumber, Bytes, Log, U256}};
use crate::{include_abi, provider::LogFilter, task::Task};
use super::base::BaseContract;

thread_local! {
    static ABI: Contract = include_abi!("./abi/dvn.json");
}

pub struct FunctionExecConfig {
    pub data: Bytes,
    pub gas: U256
}

#[derive(Clone, Default)]
pub struct DVN {
    pub address: Address,
    pub chain: u64,
}

impl DVN {
    pub fn new(address: Address, chain: u64) -> Self {
        Self { address, chain }
    }

    pub fn jobs_filter(&self, from_block: BlockNumber, to_block: BlockNumber) -> LogFilter {
        ABI.with(|abi| {
            let topic_filter = BaseContract::event_topic_filter(abi, "TaskAssigned", RawTopicFilter {
                topic0: ethers_core::abi::Topic::Any,
                topic1: ethers_core::abi::Topic::Any,
                topic2: ethers_core::abi::Topic::Any
            });

            LogFilter {
                address: self.address.clone(),
                from: from_block,
                to: to_block,
                topics: topic_filter
            }
        })
    }

    pub fn jobs_parse(&self, log: Log) -> Task  {
        ABI.with(|abi| {
            let parsed_log = BaseContract::event_parse_raw_log(abi, "TaskAssigned", RawLog {
                data: log.data.to_vec(),
                topics: log.topics
            });

            let mut dest_chain: Option<u64> = None;
            let mut message: Option<Token> = None;
            let mut gas: Option<U256> = None;
            for param in parsed_log.params.into_iter() {
                if param.name.as_str() == "dstEid" {
                    if let Token::Uint(v) = param.value {
                        dest_chain = Some(v.as_u64());
                    }
                } else if param.name.as_str() == "task" {
                    message = Some(param.value);
                } else if param.name.as_str() == "maxUnitGasPrice" {
                    if let Token::Uint(v) = param.value {
                        gas = Some(v);
                    }
                }
            }

            Task {
                src_chain: self.chain,
                dest_chain: dest_chain.unwrap(),
                gas: gas.unwrap(),
                message: message.unwrap()
            }
        })
    }

    pub fn verify_config(&self, job: &Task) -> FunctionExecConfig {
        ABI.with(|abi| {
            let data = BaseContract::function_data(
                abi,
                "verify", 
                &[job.message.clone()]
            );

            FunctionExecConfig {
                data,
                gas: U256::from("300000")
            }
        })
    }
}