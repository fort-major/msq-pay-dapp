use std::collections::{BTreeMap, HashMap};

use candid::CandidType;
use ic_e8s::c::E8s;
use serde::Deserialize;

use crate::utils::Timestamp;

use super::types::Ticker;

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct ExchangeRatesState {
    pub mock: bool,
    pub last_updated_at: Timestamp,
    pub rates: HashMap<Timestamp, BTreeMap<Ticker, E8s>>,
}

impl ExchangeRatesState {
    pub fn should_mock(&self) -> bool {
        self.mock
    }

    pub fn set_should_mock(&mut self, mock: bool) {
        self.mock = mock;
    }

    pub fn get_exchange_rate(&self, updated_at: &Timestamp, ticker: &Ticker) -> &E8s {
        self.rates.get(updated_at).unwrap().get(ticker).unwrap()
    }

    pub fn get_current_rates(&self) -> &BTreeMap<Ticker, E8s> {
        self.rates
            .get(&self.last_updated_at)
            .expect("Current rates are not ready yet, try again later...")
    }

    pub fn get_rates(&self, timestamp: Timestamp) -> Option<&BTreeMap<Ticker, E8s>> {
        let mut keys = Vec::new();

        for key in self.rates.keys() {
            match keys.binary_search(key) {
                Err(idx) => keys.insert(idx, *key),
                _ => {}
            }
        }

        match keys.binary_search(&timestamp) {
            Ok(idx) => self.rates.get(&keys[idx]),
            Err(idx) => {
                if idx == 0 {
                    None
                } else {
                    self.rates.get(&keys[idx - 1])
                }
            }
        }
    }

    pub fn delete_outdated(&mut self, timestamp: &Timestamp) {
        if *timestamp == self.last_updated_at {
            return;
        }

        self.rates.remove(&timestamp);
    }
}
