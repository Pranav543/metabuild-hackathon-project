use crate::*;
// use near_sdk::serde::{Deserialize, Serialize};

#[derive(
    BorshDeserialize, BorshSerialize, Default, Serialize, Deserialize, Clone, Copy, PartialEq,
)]
#[serde(crate = "near_sdk::serde")]
pub struct Location {
    pub cur_index: Option<Measurement>,
    // amount of money invested here
    pub total_invested: Balance,
    pub current_invested: Balance,
    // number of individual investments made (people)
    pub total_investments: u64,
    pub current_investments: u64,
}

#[derive(
    Serialize, Deserialize, BorshDeserialize, BorshSerialize, Default, Clone, Copy, PartialEq,
)]
#[serde(crate = "near_sdk::serde")]
pub struct Measurement {
    pub value: f64,
    // unix time (UTC) in seconds
    pub time: u64,
}

impl Measurement {
    pub fn new(value: f64, time: u64) -> Measurement {
        Measurement { value, time }
    }
}

impl Location {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_investment(&mut self, amount: Balance) {
        self.total_invested += amount;
        self.current_invested += amount;
        self.total_investments += 1;
        self.current_investments += 1;
    }

    pub fn finish_investment(&mut self, amount: Balance, count: u64) {
        self.current_investments -= count;
    }
}
