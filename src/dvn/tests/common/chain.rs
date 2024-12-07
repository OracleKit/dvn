use std::cmp;
use ethers_core::{abi::{encode, Event, EventParam, ParamType, Token}, types::{Address, Bloom, Bytes, Eip1559TransactionRequest, Filter, FilterBlockOption, Log, NameOrAddress, OtherFields, Signature, TransactionReceipt, ValueOrArray, H256, U256}};
use super::utils::{extract_block_number, get_txn_hash};

#[derive(Debug)]
struct Block {
    hash: H256,
    num_txns: usize,
}

#[derive(Debug)]
pub struct ChainStateMachineFactory {
    sender: Address,
    contract: Address,
    chains: Vec<ChainStateMachine>
}

impl ChainStateMachineFactory {
    pub fn new(sender: Address, contract: Address) -> Self {
        Self {
            sender,
            contract,
            chains: vec![]
        }
    }

    pub fn create(&mut self) -> &mut ChainStateMachine {
        let machine = ChainStateMachine::new(
            self.chains.len().try_into().unwrap(),
            self.sender.clone(),
            self.contract.clone()
        );

        self.chains.push(machine);
        
        let last_chain_index = self.chains.len() - 1;
        &mut self.chains[last_chain_index]
    }

    pub fn get(&self, url: &str) -> Option<&ChainStateMachine> {
        for i in self.chains.iter() {
            if i.url() == url {
                return Some(i);
            }
        }

        None
    }

    pub fn get_mut(&mut self, url: &str) -> Option<&mut ChainStateMachine> {
        for i in self.chains.iter_mut() {
            if i.url() == url {
                return Some(i);
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct ChainStateMachine {
    url: String,
    chain_id: u64,
    endpoint_id: u64,
    base_gas: U256,
    priority_gas: U256,
    nonce: U256,
    block_number: U256,
    sender: Address,
    contract: Address,
    logs: Vec<Log>,
    mempool: Vec<(Eip1559TransactionRequest, Signature)>,
    receipts: Vec<TransactionReceipt>,
    blocks: Vec<Block>
}

impl ChainStateMachine {
    pub fn new(offset: u64, sender: Address, contract: Address) -> Self {
        Self {
            url: format!("https://localhost:{}/", (9000 + offset).to_string()),
            chain_id: offset,
            endpoint_id: 70000 + offset,
            base_gas: U256::from(100),
            priority_gas: U256::from(100),
            nonce: U256::from(0),
            block_number: U256::from(0),
            sender,
            contract,
            logs: vec![],
            mempool: vec![],
            receipts: vec![],
            blocks: vec![
                Block { hash: H256::zero(), num_txns: 0 }
            ]
        }
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    pub fn endpoint_id(&self) -> u64 {
        self.endpoint_id
    }

    pub fn base_gas(&self) -> U256 {
        self.base_gas
    }

    pub fn priority_gas(&self) -> U256 {
        self.priority_gas
    }

    pub fn transaction_count(&self) -> U256 {
        self.nonce + 1
    }

    pub fn block_number(&self) -> U256 {
        self.block_number
    }

    pub fn sender(&self) -> Address {
        self.sender
    }

    pub fn contract(&self) -> Address {
        self.contract
    }

    pub fn mine(&mut self) {
        self.block_number = self.block_number.checked_add(1.into()).unwrap();
        let mut mine_txn_index: Option<usize> = None;
        let mut mine_gas_price: Option<U256> = None;

        let current_nonce = self.transaction_count();
        self.mempool.retain(|(txn, _)| {
            txn.nonce == Some(current_nonce)
        });

        for (i, (txn, _)) in self.mempool.iter().enumerate() {
            let txn_max_priority_fees = txn.max_priority_fee_per_gas.unwrap();
            let txn_max_fees = txn.max_fee_per_gas.unwrap();

            let block_base_fees = self.base_gas;
            let block_priority_fees = self.priority_gas;

            if txn_max_fees < block_base_fees {
                continue;
            }

            let txn_priority_fees = cmp::max(txn_max_fees - block_base_fees, txn_max_priority_fees);

            if txn_priority_fees < block_priority_fees {
                continue;
            }

            mine_txn_index = Some(i);
            mine_gas_price = Some(block_base_fees + txn_priority_fees);
            break;
        }
        
        self.blocks.push(Block { hash: H256::random(), num_txns: 0 });
        self.block_number += 1.into();

        let block_number = self.blocks.len() - 1;
        let block = &mut self.blocks[block_number];

        let Some(mine_txn_index) = mine_txn_index else {
            return;
        };

        let (txn, signature) = self.mempool.swap_remove(mine_txn_index);
        let hash = get_txn_hash(&txn, &signature);

        block.num_txns += 1;
        self.nonce += 1.into();

        self.receipts.push(TransactionReceipt {
            transaction_hash: hash,
            transaction_index: 0.into(),
            block_hash: Some(block.hash.clone()),
            block_number: Some(block_number.into()),
            from: txn.from.unwrap(),
            to: Some(txn.to.unwrap().as_address().unwrap().clone()),
            contract_address: None,
            cumulative_gas_used: 0.into(), // not calculated
            gas_used: Some(0.into()), // not calculated
            logs: vec![],
            status: Some(1.into()),
            root: None,
            logs_bloom: Bloom::zero(),
            transaction_type: Some(1.into()),
            effective_gas_price: Some(mine_gas_price.unwrap()),
            other: OtherFields::default()
        });


        let current_nonce = self.transaction_count();
        self.mempool.retain(|(txn, _)| {
            txn.nonce == Some(current_nonce)
        });
    }

    pub fn set_base_gas(&mut self, base_gas: U256) {
        self.base_gas = base_gas;
    }

    pub fn set_priority_gas(&mut self, priority_gas: U256) {
        self.priority_gas = priority_gas;
    }

    pub fn emit_log(&mut self, dst_eid: U256, num_confirmations: U256, data: Bytes) {
        let event = Event {
            name: "TaskAssigned".into(),
            inputs: vec![
                EventParam {
                    name: "dstEid".into(),
                    kind: ParamType::Uint(256),
                    indexed: true
                },
                EventParam {
                    name: "numConfirmations".into(),
                    kind: ParamType::Uint(256),
                    indexed: true
                },
                EventParam {
                    name: "data".into(),
                    kind: ParamType::Uint(256),
                    indexed: false
                }
            ],
            anonymous: false
        };

        let block_number = self.block_number.as_usize();
        let block = &mut self.blocks[block_number];

        let log = Log {
            address: self.contract.clone(),
            topics: vec![
                event.signature(),
                H256::from_slice(encode(&[Token::Uint(dst_eid)]).as_slice()),
                H256::from_slice(encode(&[Token::Uint(num_confirmations)]).as_slice()),
            ],
            data,
            block_hash: Some(block.hash.clone()),
            block_number: Some(block_number.into()),
            transaction_hash: Some(H256::random()),
            transaction_index: Some(block.num_txns.into()),
            log_index: Some(0.into()),
            transaction_log_index: None,
            log_type: None,
            removed: Some(false)
        };

        let receipt = TransactionReceipt {
            transaction_hash: H256::random(),
            transaction_index: block.num_txns.into(),
            block_hash: Some(block.hash.clone()),
            block_number: Some(block_number.into()),
            from: Address::zero(),
            to: Some(self.contract.clone()),
            contract_address: None,
            cumulative_gas_used: 0.into(), // not calculated
            gas_used: Some(0.into()), // not calculated
            logs: vec![ log.clone() ],
            status: Some(1.into()),
            root: None,
            logs_bloom: Bloom::zero(),
            transaction_type: Some(1.into()),
            effective_gas_price: Some(0.into()),
            other: OtherFields::default()
        };

        block.num_txns += 1;

        self.logs.push(log);
        self.receipts.push(receipt);
    }

    pub fn get_logs(&self, filter: &Filter) -> Vec<Log> {
        if let FilterBlockOption::Range {
            from_block,
            to_block
        } = filter.block_option {
            let from_block = from_block
                .map(| v |
                    extract_block_number(
                        v,
                        1.into(), 
                        self.block_number.clone()
                    )
                )
                .unwrap_or(1.into());

            let to_block = to_block
                .map(| v |
                    extract_block_number(
                        v,
                        1.into(), 
                        self.block_number.clone()
                    )
                )
                .unwrap_or(self.block_number.clone());

            let mut logs: Vec<Log> = self.logs
                .iter()
                .filter(|log| {
                    log.block_number.unwrap() <= to_block.as_u64().into() &&
                    log.block_number.unwrap() >= from_block.as_u64().into()
                })
                .map(|log| log.clone())
                .collect();

            if let Some(addresses) = &filter.address {
                if let ValueOrArray::Value(address) = addresses {
                    logs = logs
                        .into_iter()
                        .filter(|log| &log.address == address)
                        .collect();
                } else {
                    panic!("Not implemented yet!");
                }
            }

            logs
        } else {
            panic!("Not implemented yet!");
        }
    }

    pub fn transact(&mut self, (txn, signature): (Eip1559TransactionRequest, Signature)) -> H256 {
        let hash = get_txn_hash(&txn, &signature);
        self.mempool.push((txn, signature));

        hash
    }
}