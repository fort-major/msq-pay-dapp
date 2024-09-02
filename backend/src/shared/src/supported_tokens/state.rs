use std::collections::{btree_map::Values, BTreeMap};

use candid::CandidType;
use serde::Deserialize;

use crate::{exchange_rates::types::Ticker, utils::TokenId};

use super::types::Token;

#[derive(Default, CandidType, Deserialize, Clone)]
pub struct SupportedTokensState {
    tokens: BTreeMap<TokenId, Token>,
    tokens_by_ticker: BTreeMap<Ticker, TokenId>,
}

impl SupportedTokensState {
    pub fn add_token(&mut self, token: Token) {
        self.tokens_by_ticker.insert(token.ticker, token.id);
        self.tokens.insert(token.id, token);
    }

    pub fn remove_token(&mut self, ticker: Ticker) {
        if let Some(token_id) = self.tokens_by_ticker.remove(&ticker) {
            self.tokens.remove(&token_id);
        }
    }

    pub fn contains_id(&self, id: &TokenId) -> bool {
        self.tokens.contains_key(id)
    }

    pub fn contains_ticker(&self, ticker: &str) -> bool {
        self.tokens_by_ticker.contains_key(ticker)
    }

    pub fn get(&self) -> Values<TokenId, Token> {
        self.tokens.values()
    }

    pub fn get_by_id(&self, id: &TokenId) -> Option<&Token> {
        self.tokens.get(id)
    }

    pub fn ticker_by_token_id(&self, token_id: &TokenId) -> Option<Ticker> {
        self.tokens.get(token_id).map(|it| it.ticker)
    }
}
