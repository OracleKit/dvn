#[macro_export]
macro_rules! include_abi {
    ($file:expr $(,)?) => {{
        match serde_json::from_str::<ethers_core::abi::Contract>(include_str!($file)) {
            Ok(contract) => contract,
            Err(err) => panic!("Error loading ABI contract {:?}: {}", $file, err),
        }
    }};
}