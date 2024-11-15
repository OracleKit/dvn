use ethers_core::types::U256;

pub struct CurrentGasConfig {
    pub base_fees: U256,
    pub priority_fees: U256
}

pub struct TransactionGasConfig {
    pub max_fees: U256,
    pub max_priority_fees: U256
}

#[derive(Default)]
pub struct GasManager {
    current_base_fees: U256,
    current_priority_fees: U256
}

impl GasManager {
    pub fn new() -> Self {
        Self {
            current_base_fees: 0.into(),
            current_priority_fees: 0.into()
        }
    }

    pub fn predicted_fees(&self) -> TransactionGasConfig {
        let base_fees = self.current_base_fees.checked_mul(11.into()).unwrap();
        let base_fees = base_fees.checked_div(10.into()).unwrap();
        let priority_fees = self.current_priority_fees;
        let max_fees = base_fees.checked_add(priority_fees).unwrap();

        TransactionGasConfig {
            max_fees: max_fees,
            max_priority_fees: priority_fees
        }
    }

    pub fn current_fees(&mut self, current_fees: CurrentGasConfig) {
        self.current_base_fees = current_fees.base_fees;
        self.current_priority_fees = current_fees.priority_fees;
    }
}