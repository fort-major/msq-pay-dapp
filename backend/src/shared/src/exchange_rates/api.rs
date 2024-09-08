use candid::CandidType;
use ic_e8s::c::E8s;
use serde::Deserialize;

use crate::utils::Timestamp;

use super::types::Ticker;

#[derive(CandidType, Deserialize)]
pub struct GetExchangeRatesRequest {
    pub timestamp: Option<Timestamp>,
}

#[derive(CandidType, Deserialize)]
pub struct GetExchangeRatesResponse {
    pub rates: Option<Vec<(Ticker, E8s)>>,
}
