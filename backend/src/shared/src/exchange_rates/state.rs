use std::collections::{BTreeMap, HashMap};

use candid::CandidType;
use serde::Deserialize;

use crate::utils::{Timestamp, USD};

use super::types::Ticker;

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct ExchangeRatesState {
    pub last_updated_at: Timestamp,
    pub rates: HashMap<Timestamp, BTreeMap<Ticker, USD>>,
}

impl ExchangeRatesState {
    pub fn get_exchange_rate(&self, updated_at: &Timestamp, ticker: &Ticker) -> &USD {
        self.rates.get(updated_at).unwrap().get(ticker).unwrap()
    }

    pub fn get_current_rates(&self) -> &BTreeMap<Ticker, USD> {
        self.rates
            .get(&self.last_updated_at)
            .expect("Current rates are not ready yet, try again later...")
    }

    pub fn get_rates(&self, timestamp: Timestamp) -> Option<&BTreeMap<Ticker, USD>> {
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
