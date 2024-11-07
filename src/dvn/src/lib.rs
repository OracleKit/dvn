use std::time::Duration;

use ethers_core::utils::hex::ToHexExt;
use state::{ChainState, ETHEREUM_HOLESKY, POLYGON_AMOY};
mod contracts;
mod state;
mod signer;
mod provider;

#[ic_cdk::update]
async fn address() -> String {
    let provider = POLYGON_AMOY.with_borrow(|state| state.provider.clone());
    let address = provider.address().await.encode_hex_with_prefix();
    address
}

async fn _process_jobs() {
    let source_contract = ETHEREUM_HOLESKY.with_borrow(|state| state.dvn.clone());
    let destination_contract = POLYGON_AMOY.with_borrow(|state| state.dvn.clone());
    let last_processed_block = ETHEREUM_HOLESKY.with_borrow(|state| state.last_processed_block.clone());
    let current_block = ETHEREUM_HOLESKY.with_borrow(|state| state.provider.clone()).get_current_block().await;
    ETHEREUM_HOLESKY.with_borrow_mut(|state| state.last_processed_block = current_block.as_u64());

    let jobs = source_contract.get_assigned_jobs(last_processed_block + 1).await;

    ic_cdk::println!("Found {:?} jobs from blocks {:?} to {:?}", jobs.len(), last_processed_block + 1, current_block.as_u64());

    for job in jobs.into_iter() {
        ic_cdk::println!("Verifying job...");
        destination_contract.verify(job).await;
        ic_cdk::println!("Job verified!");
    };
}

#[ic_cdk::update]
async fn process_jobs() {
    ic_cdk::println!("{:?}", ic_cdk::api::instruction_counter());
    _process_jobs().await;
    ic_cdk::println!("{:?}", ic_cdk::api::instruction_counter());
}

#[ic_cdk::update]
async fn init_dvn() {
    let mut state = ChainState::new(
        env!("POLYGONAMOY_RPC_SSL_URL"),
        env!("POLYGONAMOY_CHAIN_ID"),
        env!("POLYGONAMOY_DVN_ADDRESS")
    ).await;
    POLYGON_AMOY.replace(state);

    let mut state = ChainState::new(
        env!("ETHEREUMHOLESKY_RPC_SSL_URL"),
        env!("ETHEREUMHOLESKY_CHAIN_ID"),
        env!("ETHEREUMHOLESKY_DVN_ADDRESS")
    ).await;
    ETHEREUM_HOLESKY.replace(state);

    ic_cdk_timers::set_timer_interval(
        Duration::from_secs(30), 
        || ic_cdk::spawn(_process_jobs())
    );
}