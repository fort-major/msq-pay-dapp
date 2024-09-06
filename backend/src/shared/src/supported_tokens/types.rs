use candid::CandidType;
use serde::Deserialize;

use crate::{exchange_rates::types::Ticker, utils::TokenId};
use ic_e8s::d::EDs;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Token {
    pub id: TokenId,
    pub ticker: Ticker,
    pub xrc_ticker: Ticker,
    pub fee: EDs,
    pub logo_src: String,
}
