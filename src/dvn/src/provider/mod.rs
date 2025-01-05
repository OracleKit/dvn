mod base;
mod batch;
mod receipt;

use base::BaseProvider;
pub use base::transform_rpc;
pub use receipt::{CurrentGasConfigReceipt, GenericReceipt, Receipt};
use ethers_core::{abi::{Address, Topic, TopicFilter}, types::{BlockNumber, Bytes, Filter, FilterBlockOption, Log, ValueOrArray, H256, U256}, utils::hex::ToHexExt};

pub struct LogFilter {
    pub address: Address,
    pub from: BlockNumber,
    pub to: BlockNumber,
    pub topics: TopicFilter
}

#[derive(Default)]
pub struct Provider {
    base: BaseProvider
}

impl Provider {
    pub fn new(urls: Vec<String>) -> Self {
        Self {
            base: BaseProvider::new(urls),
        }
    }

    pub fn block_number(&self) -> GenericReceipt<U256> {
        self.base.issue_request(
            "eth_blockNumber",
            None as Option<u8>,
            0
        )
    }

    pub fn nonce(&self, account: &Address) -> GenericReceipt<U256> {
        self.base.issue_request(
            "eth_getTransactionCount",
            Some((account.encode_hex_with_prefix(), "latest")),
            80
        )
    }

    pub fn gas(&self) -> CurrentGasConfigReceipt {
        CurrentGasConfigReceipt::new(
            self.base.issue_request(
                "eth_gasPrice",
                None as Option<u8>,
                80
            ),
            self.base.issue_request(
                "eth_maxPriorityFeePerGas",
                None as Option<u8>,
                80
            )
        )
    }

    pub fn send(&self, txn: Bytes) -> GenericReceipt<String> {
        self.base.issue_request(
            "eth_sendRawTransaction",
            Some((txn, )),
            100
        )
    }
    
    pub fn logs(&self, filter: LogFilter) -> GenericReceipt<Vec<Log>> {
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

        self.base.issue_request(
            "eth_getLogs",
            Some([filter]),
            8000
        )
    }

    pub async fn commit(&self) {
        self.base.commit().await;
    }
}