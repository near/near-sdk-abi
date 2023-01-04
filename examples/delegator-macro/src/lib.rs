use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use near_sdk_abi::near_abi_ext;

near_abi_ext! { mod ext_adder trait Adder for "src/adder.json" }

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
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
        ext_adder::ext(adder_account_id).add(vec![a1.into(), a2.into()], vec![b1.into(), b2.into()])
    }
}
