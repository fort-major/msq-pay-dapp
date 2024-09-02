use candid::CandidType;
use serde::Deserialize;

use crate::exchange_rates::types::Ticker;

use super::types::Token;

#[derive(CandidType, Deserialize)]
pub struct GetSupportedTokensRequest {}

#[derive(CandidType, Deserialize)]
pub struct GetSupportedTokensResponse {
    pub supported_tokens: Vec<Token>,
}

#[derive(CandidType, Deserialize)]
pub struct AddSupportedTokenRequest {
    pub token: Token,
}

#[derive(CandidType, Deserialize)]
pub struct AddSupportedTokenResponse {}

#[derive(CandidType, Deserialize)]
pub struct RemoveSupportedTokenRequest {
    pub ticker: Ticker,
}

#[derive(CandidType, Deserialize)]
pub struct RemoveSupportedTokenResponse {}
