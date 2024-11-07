use std::cell::RefCell;
use ethers_core::abi::Address;
use crate::{contracts::DVN, provider::Provider};
use std::str::FromStr;

#[derive(Clone, Default)]
pub struct ChainState {
    pub provider: Provider,
    pub dvn: DVN,
    pub last_processed_block: u64
}

impl ChainState {
    #[allow(dead_code)]
    pub async fn new(rpc_url: &str, chain_id: &str, dvn_address: &str) -> Self {
        let chain_id: u64 = chain_id.parse().unwrap();
        let mut provider = Provider::new(rpc_url.to_string(), chain_id);
        provider.init().await;

        let last_processed_block = provider.get_current_block().await.as_u64();
        let dvn = DVN::new(provider.clone(), Address::from_str(dvn_address).unwrap());

        Self { provider, dvn, last_processed_block }
    }
}

thread_local! {
    pub static ETHEREUM_MAINNET: RefCell<ChainState> = RefCell::default();    
    pub static POLYGON_POS: RefCell<ChainState> = RefCell::default();
    pub static ETHEREUM_HOLESKY: RefCell<ChainState> = RefCell::default();
    pub static POLYGON_AMOY: RefCell<ChainState> = RefCell::default();
}
