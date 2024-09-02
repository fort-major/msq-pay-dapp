use candid::{CandidType, Nat};
use serde::Deserialize;

use crate::{exchange_rates::types::Ticker, utils::TokenId};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Token {
    pub id: TokenId,
    pub ticker: Ticker,
    pub decimals: u8,
    pub fee: Nat,
}
