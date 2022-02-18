use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractMetadata {
     // address of oracle contract (this allows writing data)
    pub oracle: AccountId,
    // address of the cw20 token that we use for payment
    pub token: AccountId,
    // maximum amount that can be invested in one hex
    pub max_investment_hex: u64,
    // how many days the investment takes until maturity (eg. we pay out in the results in 30 days, 180 days)
    pub maturity_days: u64,
    // how many days margin we have from measurement to usage.
    // when investing, the latest data must be within X days
    // when investment finishes, there must be data within X days of maturity
    pub measurement_window: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct Location {
    pub cur_index: Option<Measurement>,
    // amount of money invested here
    pub total_invested: u64,
    pub current_invested: u64,
    // number of individual investments made (people)
    pub total_investments: u64,
    pub current_investments: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, PartialEq, Clone, Default)]
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

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Investment {
    // how much was invested
    pub amount: u64,
    // starting value when investment was created
    pub baseline_index: f64,
    // when this investment was made - in UNIX seconds UTC
    pub invested_time: u64,
    // when this investment can be claimed - in UNIX seconds UTC
    pub maturity_time: u64,
}