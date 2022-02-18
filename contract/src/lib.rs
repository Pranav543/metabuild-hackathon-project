// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, setup_alloc, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};

// use crate::r3::validate_r3;
use crate::state::*;
use crate::internal::*;


// mod r3;
mod state;
mod internal;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    // pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    // //keeps track of the token struct for a given token ID
    // pub tokens_by_id: LookupMap<TokenId, Token>,

    // //keeps track of the token metadata for a given token ID
    // pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    // //keeps track of the metadata for the contract
    // pub metadata: LazyOption<NFTContractMetadata>,
}
