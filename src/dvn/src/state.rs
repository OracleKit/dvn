use std::{cell::RefCell, ops::Index, rc::Rc};
use crate::{chain::ChainState, signer::Signer};

thread_local! {
    static SIGNER: RefCell<Rc<Signer>> = RefCell::default();
    static CHAINS: RefCell<Vec<Rc<RefCell<ChainState>>>> = RefCell::default();
}
pub struct GlobalState;

impl GlobalState {
    pub async fn init() {
        let mut signer = Signer::new("test_key".to_string());
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

    pub fn chain_by_id(chain_id: u64) -> Rc<RefCell<ChainState>> {
        CHAINS.with(|chains| {
            let chains = chains.borrow();
            let chain = chains.iter().find(|chain| chain.borrow().chain_id == chain_id).unwrap();
            Rc::clone(chain)
        })
    }
    
    pub fn num_chains() -> usize {
        CHAINS.with(|chains| {
            chains.borrow().len()
        })
    }

    pub async fn add_chain(rpc_url: String, chain_id: u64, dvn_address: String) -> usize {
        let mut new_chain = ChainState::new(&rpc_url, chain_id, &dvn_address);
        new_chain.init().await;

        CHAINS.with(|chains| {
            chains.borrow_mut().push(Rc::new(RefCell::new(new_chain)));
            chains.borrow().len() - 1
        })
    }
}