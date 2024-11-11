use std::{cell::RefCell, ops::Index, rc::Rc};
use crate::{chain::ChainState, signer::Signer};

thread_local! {
    static SIGNER: RefCell<Rc<Signer>> = RefCell::default();
    static CHAINS: RefCell<Vec<Rc<ChainState>>> = RefCell::default();
}
pub struct GlobalState;

impl GlobalState {
    pub fn signer() -> Rc<Signer> {
        SIGNER.with(|s| {
            s.borrow().clone()
        })
    }

    pub fn chain(cid: usize) -> Rc<ChainState> {
        CHAINS.with(|chains| {
            chains.borrow().index(cid).clone()
        })
    }
    
    pub fn num_chains() -> usize {
        CHAINS.with(|chains| {
            chains.borrow().len()
        })
    }

    pub async fn add_chain(rpc_url: String, chain_id: String, dvn_address: String) -> usize {
        let new_chain = ChainState::new(&rpc_url, &chain_id, &dvn_address).await;

        CHAINS.with(|chains| {
            chains.borrow_mut().push(Rc::new(new_chain));
            chains.borrow().len() - 1
        })
    }
}