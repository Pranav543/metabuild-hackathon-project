use crate::*;
// use near_sdk::serde::{Deserialize, Serialize};

#[derive(
    BorshDeserialize, BorshSerialize, Default, Serialize, Deserialize, Clone, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub struct ListInvestmentsResponse {
    pub investments: Vec<InvestmentResponse>,
}

#[derive(
    Serialize, Deserialize, BorshDeserialize, BorshSerialize, Default, Clone, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub struct InvestmentResponse {
    pub hex: String,
    // how much was invested
    pub amount: Balance,
    // starting value when investment was created
    pub baseline_index: f64,
    // starting value when investment was created
    pub latest_index: Measurement,
    // true iff we can withdraw this Investment now
    pub can_withdraw: bool,
    // if we withdrew today, how much would we get
    pub withdraw_amount: Balance,
    // when this investment was made - in UNIX seconds UTC
    pub invested: u64,
    // when this investment can be claimed - in UNIX seconds UTC
    pub maturity_date: u64,
}


impl InvestmentResponse {
    pub fn new(invest: Investment, hex: &str, measurement_window: u64, loc: &Location) -> Self {
        let (withdraw_amount, can_withdraw) = match invest.reward(loc, measurement_window) {
            Some(reward) => (reward, true),
            None => (invest.would_reward(loc), false),
        };
        InvestmentResponse {
            hex: hex.into(),
            amount: invest.amount,
            baseline_index: invest.baseline_index,
            // this will always be Some, as we never allow investing if it was None
            latest_index: loc.cur_index,
            withdraw_amount,
            can_withdraw,
            invested: invest.invested_time,
            maturity_date: invest.maturity_time,
        }
    }
}
