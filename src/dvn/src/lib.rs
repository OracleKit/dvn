use ethers_core::utils::hex::ToHexExt;
use state::{ChainState, ETHEREUM_MAINNET, POLYGON_AMOY, POLYGON_POS, ETHEREUM_HOLESKY};
mod ether_utils;
mod contracts;
mod state;

#[ic_cdk::update]
async fn public_key() -> String {
    let provider = POLYGON_AMOY.with_borrow(|state| state.provider.clone());
    let address = provider.address().await.encode_hex_with_prefix();
    address
}

// #[ic_cdk::update]
// async fn get_gas() -> String {
//     let mut provider = ether_utils::Provider::new("https://localhost:9010".to_string(), 31337);
//     provider.init().await;
//     provider.get_current_base_fees().await.to_string()
// }


#[ic_cdk::update]
async fn greet() {
    // let ic_provider = IcProvider::from_str("https://eth-mainnet.public.blastapi.io").unwrap();
    // let provider = Provider::<IcProvider>::new(ic_provider);

    // let z = provider.get_balance("0x00000000219ab540356cbb839cbe05303d7705fa", None).await.unwrap();
    // z.to_string()

    // let public_key = hex::encode(IcSigner::public_key().await.to_sec1_bytes());
    // public_key

    // let provider = ether_utils::Provider::new("https://eth-mainnet.public.blastapi.io");
    // provider.get_balance(name.as_str()).await.to_string()
    // let mut provider = ether_utils::Provider::new("https://localhost:9001".to_string(), 1);
    // provider.init().await;

    // let contract = DVN::new(provider, Address::from_str("0x10823151f81a6940c41502eb740cf30632a1510c").unwrap());
    // contract.get_assigned_jobs().await;
    
    // "YESSS".to_string()

    let source_contract = ETHEREUM_HOLESKY.with_borrow(|state| state.dvn.clone());
    let destination_contract = POLYGON_AMOY.with_borrow(|state| state.dvn.clone());
    let jobs = source_contract.get_assigned_jobs().await;
    ic_cdk::println!("Verifying jobs: {:?}", jobs.len());
    ic_cdk::println!("Verifying jobs: {:?}", jobs.len());

    for job in jobs.into_iter() {
        ic_cdk::println!("Verifying job: {:?}", job);
        let hash = destination_contract.verify(job).await;
        ic_cdk::println!("Job verified: {hash}");
    }

    // let txn = Eip1559TransactionRequest {
    //     from: None,
    //     to: Some(ethers_core::types::NameOrAddress::Address(Address::from_str("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266").unwrap())),
    //     gas: None,
    //     value: Some(U256::exp10(18)),
    //     data: None,
    //     nonce: None,
    //     access_list: AccessList::default(),
    //     max_priority_fee_per_gas: None,
    //     max_fee_per_gas: None,
    //     chain_id: None
    // };

    // let signer = IcSigner {};
    // let wallet = Wallet::new_with_signer(signer, IcSigner::address().await.clone(), 31337);
    // let sign = wallet.sign_transaction_sync(&TypedTransaction::Eip1559(txn.clone())).unwrap();
    // let encoded = txn.rlp_signed(&sign);
    
    // provider.get_current_base_fees().await.to_string()
    // provider.send_transaction(txn).await
}

#[ic_cdk::update]
async fn init_providers() {
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
}