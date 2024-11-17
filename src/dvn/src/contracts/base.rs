use ethers_core::{abi::{Contract, Log, RawLog, RawTopicFilter, Token, TopicFilter}, types::Bytes};

pub struct BaseContract;

impl BaseContract {
    pub fn function_data(abi: &Contract, function_name: &str, args: &[Token]) -> Bytes {
        let function = &abi.functions_by_name(function_name).unwrap()[0];
        let data = function.encode_input(args).unwrap();
        
        Bytes::from(data)
    }

    pub fn event_topic_filter(abi: &Contract, event_name: &str, topics: RawTopicFilter) -> TopicFilter {
        let event = &abi.events_by_name(event_name).unwrap()[0];
        event.filter(topics).unwrap()
    }

    pub fn event_parse_raw_log(abi: &Contract, event_name: &str, log: RawLog) -> Log {
        let event = &abi.events_by_name(event_name).unwrap()[0];
        event.parse_log(log).unwrap()
    }
}