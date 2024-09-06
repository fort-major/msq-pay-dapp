use candid::{CandidType, Principal};
use env::{CAN_INVOICE_HISTORY_CANISTER_ID, CAN_PAYMENT_HUB_CANISTER_ID, CAN_ROOT_KEY};
use lazy_static::lazy_static;
use serde::Deserialize;

mod env;
pub mod exchange_rates;
pub mod invoice_history;
pub mod invoices;
pub mod payment_hub;
pub mod shops;
pub mod supported_tokens;
pub mod utils;

lazy_static! {
    pub static ref ENV_VARS: EnvVarsState = EnvVarsState::new();
}

#[derive(CandidType, Deserialize, Clone)]
pub struct EnvVarsState {
    pub payment_hub_canister_id: Principal,
    pub invoice_history_canister_id: Principal,
    pub ic_root_key: Vec<u8>,
}

impl EnvVarsState {
    pub fn new() -> Self {
        Self {
            payment_hub_canister_id: Principal::from_text(CAN_PAYMENT_HUB_CANISTER_ID).unwrap(),
            invoice_history_canister_id: Principal::from_text(CAN_INVOICE_HISTORY_CANISTER_ID)
                .unwrap(),

            ic_root_key: CAN_ROOT_KEY
                .trim_start_matches("[")
                .trim_end_matches("]")
                .split(",")
                .map(|chunk| chunk.trim().parse().expect("Unable to parse ic root key"))
                .collect(),
        }
    }
}
