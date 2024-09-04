use candid::CandidType;
use serde::Deserialize;

use crate::{e8s::E8s, utils::Timestamp};

use super::types::Ticker;

#[derive(CandidType, Deserialize)]
pub struct GetExchangeRatesRequest {
    pub timestamp: Timestamp,
}

#[derive(CandidType, Deserialize)]
pub struct GetExchangeRatesResponse {
    pub rates: Option<Vec<(Ticker, E8s)>>,
}
