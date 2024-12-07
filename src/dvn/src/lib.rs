use std::{collections::HashMap, time::Duration};

use ethers_core::utils::hex::ToHexExt;
use state::GlobalState;
use task::Task;

mod contracts;
mod state;
mod signer;
mod provider;
mod chain;
mod task;
mod transaction;
mod nonce;
mod gas;

async fn _process_tasks() {
    let num_chains = GlobalState::num_chains();
    let mut tasks_by_chain: HashMap<u64, Vec<Task>> = HashMap::new();

    for i in 0..num_chains {
        let chain = GlobalState::chain(i);
        let tasks = chain.borrow_mut().check_for_tasks().await;

        for task in tasks {
            let chain_id = task.dest_chain;
            if let Some(tasks_list) = tasks_by_chain.get_mut(&chain_id) {
                tasks_list.push(task);
            } else {
                tasks_by_chain.insert(chain_id, vec![task]);
            }
        }
    }

    for (chain_id, tasks) in tasks_by_chain.into_iter() {
        let chain = GlobalState::chain_by_id(chain_id);
        chain.borrow_mut().process_task(tasks).await;
    }
}

#[ic_cdk::update]
async fn process_tasks() {
    ic_cdk::println!("{:?}", ic_cdk::api::instruction_counter());
    _process_tasks().await;
    ic_cdk::println!("{:?}", ic_cdk::api::instruction_counter());
}

#[ic_cdk::update]
async fn init() {
    GlobalState::init().await;

    ic_cdk_timers::set_timer_interval(
        Duration::from_secs(30), 
        || ic_cdk::spawn(_process_tasks())
    );
}

#[ic_cdk::update]
async fn add_chain(rpc_url: String, chain_id: u64, dvn_address: String) {
    GlobalState::add_chain(rpc_url, chain_id, dvn_address).await;
}

#[ic_cdk::query]
async fn address() -> String {
    GlobalState::signer().address().encode_hex_with_prefix()
}

ic_cdk::export_candid!();
