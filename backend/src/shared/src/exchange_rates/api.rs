use candid::CandidType;
use serde::Deserialize;

use crate::utils::{Timestamp, USD};

use super::types::Ticker;

#[derive(CandidType, Deserialize)]
pub struct GetExchangeRatesRequest {
    pub timestamp: Timestamp,
}

#[derive(CandidType, Deserialize)]
pub struct GetExchangeRatesResponse {
    pub rates: Option<Vec<(Ticker, USD)>>,
}
