use std::{collections::HashMap, time::Duration};

use ethers_core::utils::hex::ToHexExt;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use state::GlobalState;
use task::Task;
use utils::guard_caller_is_controller;

mod contracts;
mod state;
mod signer;
mod provider;
mod chain;
mod task;
mod transaction;
mod nonce;
mod gas;
mod utils;

async fn _process_tasks() {
    let Some(_guard) = GlobalState::acquire_task_probe_job_lock() else {
        return;
    };

    let num_chains = GlobalState::num_chains();
    let mut check_futs = vec![];

    for i in 0..num_chains {
        let fut = async move {
            let chain = GlobalState::chain(i);
            let tasks = chain.borrow_mut().check_for_tasks().await;
            tasks
        };

        check_futs.push(fut);
    }

    let tasks: Vec<Task> = futures::future::join_all(check_futs).await.into_iter().flatten().collect();
    let mut tasks_by_chain: HashMap<u64, Vec<Task>> = HashMap::new();

    for task in tasks {
        let chain_id = task.dest_chain;
        if let Some(tasks_list) = tasks_by_chain.get_mut(&chain_id) {
            tasks_list.push(task);
        } else {
            tasks_by_chain.insert(chain_id, vec![task]);
        }
    }

    let mut process_futs = vec![];
    for (chain_id, tasks) in tasks_by_chain.into_iter() {
        let fut = async move {
            let chain = GlobalState::chain_by_id(chain_id);
            chain.borrow_mut().process_tasks(tasks).await;
        };

        process_futs.push(fut);
    }

    futures::future::join_all(process_futs).await;
}

#[ic_cdk::update(guard = "guard_caller_is_controller")]
async fn process_tasks() {
    _process_tasks().await;
}

#[ic_cdk::update(guard = "guard_caller_is_controller")]
async fn init() {
    GlobalState::init().await;

    ic_cdk_timers::set_timer_interval(
        Duration::from_secs(30), 
        || ic_cdk::spawn(_process_tasks())
    );
}

#[ic_cdk::update(guard = "guard_caller_is_controller")]
async fn add_chain(rpc_url: String, chain_id: u64, endpoint_id: u64, dvn_address: String) {
    GlobalState::add_chain(rpc_url, chain_id, endpoint_id, dvn_address).await;
}

#[ic_cdk::query(guard = "guard_caller_is_controller")]
async fn address() -> String {
    GlobalState::signer().address().encode_hex_with_prefix()
}

#[ic_cdk::query]
fn transform_rpc(args: TransformArgs) -> HttpResponse {
    provider::transform_rpc(args.response, args.context)
}

ic_cdk::export_candid!();
