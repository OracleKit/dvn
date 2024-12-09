mod base;
use base::BaseProvider;
use ethers_core::{abi::{Address, Topic, TopicFilter}, types::{BlockNumber, Bytes, Filter, FilterBlockOption, Log, ValueOrArray, H256, U256, U64}, utils::hex::ToHexExt};
use crate::{gas::CurrentGasConfig, nonce::NonceConfig};

pub struct LogFilter {
    pub address: Address,
    pub from: BlockNumber,
    pub to: BlockNumber,
    pub topics: TopicFilter
}

#[derive(Clone, Default)]
pub struct Provider {
    base: BaseProvider
}

impl Provider {
    pub fn new(url: String) -> Self {
        Self {
            base: BaseProvider::new(url),
        }
    }

    pub async fn block_number(&self) -> u64 {
        let block_num: U64 = self.base.request("eth_blockNumber", None as Option<[u8; 0]>).await;
        block_num.as_u64()
    }

    pub async fn nonce(&self, account: &Address) -> NonceConfig {
        let nonce: U256 = self.base.request("eth_getTransactionCount", Some((account.encode_hex_with_prefix(), "latest"))).await;
        NonceConfig { nonce }
    }

    pub async fn gas(&self) -> CurrentGasConfig {
        let priority_fees: U256 = self.base.request("eth_maxPriorityFeePerGas", None as Option<[u8; 0]>).await;
        let base_fees: U256 = self.base.request("eth_gasPrice", None as Option<[u8; 0]>).await;

        CurrentGasConfig { base_fees, priority_fees }
    }

    pub async fn send(&self, txn: Bytes) -> String {
        self.base.request("eth_sendRawTransaction", Some((txn, ))).await
    }
    
    pub async fn logs(&self, filter: LogFilter) -> Vec<Log> {
        let topic_parse_fn = |topic: Topic<H256>| -> Option<ValueOrArray<Option<H256>>> {
            match topic {
                Topic::This(h) => Some(ValueOrArray::Value(Some(h))),
                Topic::Any => None,
                _ => None
            }
        };

        let filter = Filter {
            block_option: FilterBlockOption::Range { from_block: Some(filter.from), to_block: Some(filter.to) },
            address: Some(ValueOrArray::Value(filter.address)),
            topics: [
                topic_parse_fn(filter.topics.topic0),
                topic_parse_fn(filter.topics.topic1),
                topic_parse_fn(filter.topics.topic2),
                topic_parse_fn(filter.topics.topic3),
            ]
        };

        self.base.request("eth_getLogs", Some([filter])).await
    }
}