use candid::CandidType;
use serde::Deserialize;

use crate::{e8s::EDs, exchange_rates::types::Ticker, utils::TokenId};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Token {
    pub id: TokenId,
    pub ticker: Ticker,
    pub xrc_ticker: Ticker,
    pub fee: EDs,
    pub logo_src: String,
}
