use std::{cell::RefCell, ops::Index, rc::Rc, sync::Arc};
use futures::lock::{Mutex, OwnedMutexGuard};
use crate::{chain::ChainState, signer::Signer};

thread_local! {
    static SIGNER: RefCell<Rc<Signer>> = RefCell::default();
    static CHAINS: RefCell<Vec<Rc<RefCell<ChainState>>>> = RefCell::default();
    static TASK_PROBE_JOB_LOCK: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}
pub struct GlobalState;

impl GlobalState {
    pub async fn init() {
        let key_name = if option_env!("CANISTER_PRODUCTION_BUILD").is_none() {
            "dfx_test_key"
        } else { 
            "key_1"
        };

        let mut signer = Signer::new(key_name.to_string());
        signer.init().await;
        
        SIGNER.replace(Rc::new(signer));
    }

    pub fn signer() -> Rc<Signer> {
        SIGNER.with(|s| {
            s.borrow().clone()
        })
    }

    pub fn chain(index: usize) -> Rc<RefCell<ChainState>> {
        CHAINS.with(|chains| {
            chains.borrow().index(index).clone()
        })
    }

    pub fn chain_by_id(id: u64) -> Rc<RefCell<ChainState>> {
        CHAINS.with(|chains| {
            let chains = chains.borrow();
            let chain = chains.iter().find(|chain| chain.borrow().endpoint_id == id).unwrap();
            Rc::clone(chain)
        })
    }
    
    pub fn num_chains() -> usize {
        CHAINS.with(|chains| {
            chains.borrow().len()
        })
    }

    pub async fn add_chain(rpc_urls: Vec<String>, chain_id: u64, endpoint_id: u64, dvn_address: String) -> usize {
        let mut new_chain = ChainState::new(rpc_urls, chain_id, endpoint_id, &dvn_address);
        new_chain.init().await;

        CHAINS.with(|chains| {
            chains.borrow_mut().push(Rc::new(RefCell::new(new_chain)));
            chains.borrow().len() - 1
        })
    }

    pub fn acquire_task_probe_job_lock() -> Option<OwnedMutexGuard<bool>> {
        TASK_PROBE_JOB_LOCK.with(|lock| lock.try_lock_owned())
    }
}