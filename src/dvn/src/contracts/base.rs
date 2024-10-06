use std::rc::Rc;
use ethers_core::{abi::{self, Contract, Hash, Log, RawLog, RawTopicFilter, Token}, types::{self, transaction::eip2930::AccessList, Address, Bytes, Eip1559TransactionRequest, Filter, FilterBlockOption, U64}};
use crate::ether_utils::Provider;

#[derive(Clone, Default)]
pub struct BaseContract {
    provider: Provider,
    address: Address,
    abi: Rc<Contract>
}

impl BaseContract {
    pub fn new(provider: Provider, address: Address, abi: Rc<Contract>) -> Self {
        Self {
            provider,
            address,
            abi
        }
    }

    pub async fn write(&self, function_name: &str, args: &[Token]) -> String {
        let function = &self.abi.functions_by_name(function_name).unwrap()[0];
        let data = function.encode_input(args).unwrap();
        self.provider.send_transaction(Eip1559TransactionRequest {
            chain_id: None,
            from: None,
            to: Some(ethers_core::types::NameOrAddress::Address(self.address.clone())),
            gas: None,
            value: None,
            data: Some(Bytes::from(data)),
            nonce: None,
            access_list: AccessList::default(),
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None
        }).await
    }

    fn _abi_to_types_topic(&self, topic: abi::Topic<Hash>) -> types::Topic {
        match topic {
            abi::Topic::This(hash) => hash.into(),
            _ => types::Topic::Value(None)
        }
    }

    pub async fn events(&self, event_name: &str, topics: RawTopicFilter) -> Vec<Log> {
        let event = &self.abi.events_by_name(event_name).unwrap()[0];
        let filter = event.filter(topics).unwrap();
        let logs = self.provider.get_logs(&Filter {
            block_option: FilterBlockOption::Range { from_block: Some(types::BlockNumber::Number(U64::from(2474874))), to_block: None },
            address: Some(ethers_core::types::ValueOrArray::Value(self.address.clone())),
            topics: [
                Some(self._abi_to_types_topic(filter.topic0)),
                None,
                None,
                None
            ]
        }).await;

        let mut parsed_logs = vec![];
        for log in logs.into_iter() {
            let parsed_log = event.parse_log(RawLog {
                topics: log.topics,
                data: log.data.to_vec()
            }).unwrap();

            parsed_logs.push(parsed_log);
        }

        parsed_logs
    }
}