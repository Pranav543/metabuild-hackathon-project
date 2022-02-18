mod location;
mod investment;

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use std::collections::HashMap;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, setup_alloc, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};
use std::convert::TryInto;
use std::str::FromStr;


// use crate::r3::validate_r3;
use crate::location::*;
use crate::investment::*;

// setup_alloc!();

pub(crate) fn assert_initialized() {
    assert!(!env::state_exists(), "Already initialized");
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // address of the fungible token that we use for payment
    pub fungible_token_account_id: AccountId,
    // maximum amount that can be invested in one hex
    pub max_investment_hex: Balance,
    // how many days the investment takes until maturity (eg. we pay out in the results in 30 days, 180 days)
    pub maturity_days: u64,
    // how many days margin we have from measurement to usage.
    // when investing, the latest data must be within X days
    // when investment finishes, there must be data within X days of maturity
    pub measurement_window: u64,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        fungible_token_account_id: AccountId,
        max_investment_hex: Balance,
        maturity_days: u64,
        measurement_window: u64,
    ) -> Self {
        assert_initialized();
        Self {
            fungible_token_account_id,
            max_investment_hex,
            maturity_days,
            measurement_window
        }
    }
}
