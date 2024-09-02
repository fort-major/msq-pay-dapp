use candid::{CandidType, Principal};
use serde::Deserialize;

use super::types::ArchivedInvoice;

#[derive(CandidType, Deserialize, Default)]
pub struct State {
    pub next: Option<Principal>,
    pub log: Vec<ArchivedInvoice>,
}
