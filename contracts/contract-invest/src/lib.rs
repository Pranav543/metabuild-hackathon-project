mod investment;
mod location;
mod response;

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, ValidAccountId, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, setup_alloc, AccountId, Balance, CryptoHash, PanicOnDefault, Promise,
    PromiseOrValue,
};
use std::str::FromStr;

// use crate::r3::validate_r3;
use crate::investment::*;
use crate::location::*;
use crate::response::*;

// setup_alloc!();

pub(crate) fn assert_initialized() {
    assert!(!env::state_exists(), "Already initialized");
}

pub(crate) fn validate_r3(input: String) -> String {
    let lower = input.to_lowercase();
    assert!(lower.len() == 15, "Hex has invalid length!");
    assert!(is_hex(&lower), "Not a valid Hex value!");
    lower
}

fn is_hex(input: &str) -> bool {
    input.chars().all(|b| matches!(b, '0'..='9' | 'a'..='f'))
}

#[derive(BorshSerialize)]
pub enum StorageKey {
    Locations,
    Investments,
    AccountHexMap,
}

#[derive(Debug, PartialEq)]
enum ContractInteraction {
    Invest,
    Withdraw,
}

impl FromStr for ContractInteraction {
    type Err = ();

    fn from_str(input: &str) -> Result<ContractInteraction, Self::Err> {
        match input {
            "invest" => Ok(ContractInteraction::Invest),
            "withdraw" => Ok(ContractInteraction::Withdraw),
            _ => Err(()),
        }
    }
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

    pub locations: LookupMap<String, Location>,

    pub investments: LookupMap<(AccountId, String), Vec<Investment>>,

    pub account_hex_map: LookupMap<AccountId, Vec<String>>,
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
        let this = Self {
            fungible_token_account_id,
            max_investment_hex,
            maturity_days,
            measurement_window,
            locations: LookupMap::new(StorageKey::Locations.try_to_vec().unwrap()),
            investments: LookupMap::new(StorageKey::Investments.try_to_vec().unwrap()),
            account_hex_map: LookupMap::new(StorageKey::AccountHexMap.try_to_vec().unwrap()),
        };

        this
    }
    pub fn invest(
        &mut self,
        sender: AccountId,
        amount: Balance,
        hex: String,
        hex_index: f64,
    ) -> Result<Balance, &str> {
        let invested = env::block_timestamp() / 1000_000_000;
        let maturity_date = invested + self.maturity_days * 86400;

        // update investment info in Location
        let mut location = self.locations.get(&hex).unwrap_or_else(|| {
            //if the location hasn't initialized we create new location
            Location::default()
        });
        location.add_investment(amount, invested, hex_index);
        self.locations.insert(&hex, &location);
        log!(
            "block_timestamp is = {:?}",
            env::block_timestamp() / 1000_000_000
        );

        let last_index = location.cur_index;
        assert!(
            last_index.time
                > (env::block_timestamp() / 1000_000_000) - (self.measurement_window * 86400),
            "Data to old!"
        );

        let investment = Investment {
            amount: amount,
            baseline_index: last_index.value,
            invested_time: invested,
            maturity_time: maturity_date,
        };
        let mut investments = self
            .investments
            .get(&(sender.clone(), hex.clone()))
            .unwrap_or_else(|| Vec::new());
        investments.push(investment);
        self.investments
            .insert(&(sender.clone(), hex.clone()), &investments);
        let mut hex_arr = self.account_hex_map.get(&sender).unwrap_or_else(|| {
            //if the location hasn't initialized we create new location
            vec![]
        });
        hex_arr.push(hex.clone());
        hex_arr.dedup();
        self.account_hex_map.insert(&sender, &hex_arr);
        Ok(0)
    }

    pub fn list_investments(&self, investor: AccountId, hex: String) -> ListInvestmentsResponse {
        let hex = validate_r3(hex.clone());
        let loc = self.locations.get(&hex);
        if let Some(loc) = loc {
            loc
        } else {
            return ListInvestmentsResponse {
                investments: vec![],
            };
        };
        let invs = self.investments.get(&(investor.clone(), hex.clone()));
        let investments = if let Some(invs) = invs {
            let loc = self.locations.get(&hex).unwrap();
            self.investments
                .get(&(investor.clone(), hex.clone()))
                .unwrap()
                .into_iter()
                .map(|inv| {
                    InvestmentResponse::new(inv, &hex, self.measurement_window.clone(), &loc)
                })
                .collect()
        } else {
            return ListInvestmentsResponse {
                investments: vec![],
            };
        };

        ListInvestmentsResponse { investments }
    }

    pub fn get_investor_investments(&self, investor: AccountId) -> Vec<ListInvestmentsResponse> {
        let mut all_investments = vec![];
        let hex_arr = self.account_hex_map.get(&investor);
        if let Some(hex_arr) = hex_arr {
            for hex in hex_arr {
                let investment = self.list_investments(investor.clone(), hex.clone());
                all_investments.push(investment);
            }
        }
        all_investments
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    /// If given `msg: "take-my-money", immediately returns U128::From(0)
    /// Otherwise, makes a cross-contract call to own `value_please` function, passing `msg`
    /// value_please will attempt to parse `msg` as an integer and return a U128 version of it
    fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        // Verifying that we were called by fungible token contract that we expect.
        assert!(
            env::predecessor_account_id() == self.fungible_token_account_id,
            "Only supports the one fungible token contract"
        );
        log!(
            "in {} tokens from @{} ft_on_transfer, msg = {}",
            amount.0,
            sender_id.as_ref(),
            msg
        );
        let m: Vec<&str> = msg.split("::").collect();
        let action: String = m[0].to_string();
        let hex: String = validate_r3(m[1].to_string());
        let hex_index: f64 = m[2].parse().unwrap();

        match action.parse().unwrap() {
            ContractInteraction::Invest => {
                let result = self.invest(sender_id.into(), amount.into(), hex, hex_index);
                match result {
                    Ok(s) => PromiseOrValue::Value(s.into()),
                    Err(e) => {
                        log!(e);
                        PromiseOrValue::Value(amount)
                    }
                }
            }
            _ => {
                log!("Invalid instruction for invest call");
                PromiseOrValue::Value(amount)
            }
        }
    }
}
