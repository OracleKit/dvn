use std::cell::RefCell;
use ethers_core::abi::Address;
use crate::{contracts::DVN, ether_utils::Provider};
use std::str::FromStr;

#[derive(Clone, Default)]
pub struct ChainState {
    pub provider: Provider,
    pub dvn: DVN
}

impl ChainState {
    pub async fn new(rpc_url: &str, chain_id: &str, dvn_address: &str) -> Self {
        let chain_id: u64 = chain_id.parse().unwrap();
        let mut provider = Provider::new(rpc_url.to_string(), chain_id);
        provider.init().await;
        let dvn = DVN::new(provider.clone(), Address::from_str(dvn_address).unwrap());

        Self { provider, dvn }
    }
}

thread_local! {
    pub static ETHEREUM_MAINNET: RefCell<ChainState> = RefCell::default();    
    pub static POLYGON_POS: RefCell<ChainState> = RefCell::default();
}
