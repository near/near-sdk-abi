use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

#[path = "../gen/adder.rs"]
mod adder;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
#[borsh(crate = "near_sdk::borsh")]
pub struct Delegator {}

#[near_bindgen]
impl Delegator {
    pub fn delegate(
        &self,
        a1: u32,
        a2: u32,
        b1: u32,
        b2: u32,
        adder_account_id: near_sdk::AccountId,
    ) -> near_sdk::Promise {
        adder::ext_abi::ext(adder_account_id)
            .add(vec![a1.into(), a2.into()], vec![b1.into(), b2.into()])
    }
}
