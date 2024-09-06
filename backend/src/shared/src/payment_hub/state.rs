use std::collections::{hash_map::Entry, BTreeMap, HashMap};

use candid::CandidType;
use ic_e8s::d::EDs;
use ic_xrc_types::ExchangeRate;
use icrc_ledger_types::icrc1::account::Account;
use num_bigint::BigUint;
use serde::Deserialize;

use crate::{
    exchange_rates::{state::ExchangeRatesState, types::Ticker},
    invoices::{state::InvoicesState, types::InvoiceStatus},
    shops::state::ShopsState,
    supported_tokens::state::SupportedTokensState,
    utils::{Timestamp, RECYCLING_TTL},
};

#[derive(CandidType, Deserialize, Default)]
pub struct State {
    pub shops: ShopsState,
    pub invoices: InvoicesState,
    pub supported_tokens: SupportedTokensState,
    pub exchange_rates: ExchangeRatesState,
    pub fee_collector_account: Option<Account>,
}

impl State {
    pub fn set_fee_collector_account(&mut self, new_fee_collector_account: Option<Account>) {
        self.fee_collector_account = new_fee_collector_account;
    }

    pub fn purge_expired_invoices(&mut self) {
        let mut purged_invoices = HashMap::new();

        for (exchange_rates_timestamp, active_invoices) in self.invoices.active_invoices.iter() {
            let mut cur_purged_invoices = Vec::new();

            for id in active_invoices {
                let mut remove = false;

                {
                    let invoice = self.invoices.all_invoices.get_mut(id).unwrap();

                    if let InvoiceStatus::Created { ttl } = invoice.status {
                        if ttl > RECYCLING_TTL {
                            invoice.status = InvoiceStatus::Created { ttl: ttl - 1 };
                        } else {
                            remove = true;
                        }
                    } else {
                        unreachable!("Invoice should be in Created state");
                    }
                }

                if remove {
                    self.invoices.all_invoices.remove(id);
                    cur_purged_invoices.push(*id);
                }
            }

            purged_invoices.insert(*exchange_rates_timestamp, cur_purged_invoices);
        }

        for (exchange_rates_timestamp, invoices) in purged_invoices {
            let mut remove = false;

            {
                let active_invoices = self
                    .invoices
                    .active_invoices
                    .get_mut(&exchange_rates_timestamp)
                    .unwrap();

                for id in invoices {
                    active_invoices.remove(&id);
                }

                if active_invoices.is_empty() {
                    remove = true;
                }
            }

            if remove {
                self.invoices
                    .active_invoices
                    .remove(&exchange_rates_timestamp);

                self.exchange_rates
                    .delete_outdated(&exchange_rates_timestamp);
            }
        }
    }

    pub fn update_exchange_rates(
        &mut self,
        exchange_rates_external: Vec<ExchangeRate>,
        timestamp: Timestamp,
    ) {
        // if there are no invoices which refer to the previosly actual exchange rates - remove those rates from memory
        let previous_timestamp_referenced_by_active_invoices = self
            .invoices
            .active_invoices
            .get(&self.exchange_rates.last_updated_at)
            .map(|it| it.is_empty())
            .unwrap_or_default();

        if !previous_timestamp_referenced_by_active_invoices {
            self.exchange_rates
                .rates
                .remove(&self.exchange_rates.last_updated_at);
        }

        // store new exchange rates as actual
        self.exchange_rates.last_updated_at = timestamp;

        for rate in exchange_rates_external {
            let ticker_from = rate.base_asset.symbol;

            if self.supported_tokens.contains_ticker(&ticker_from) {
                let usd_rate = EDs::new(BigUint::from(rate.rate), rate.metadata.decimals as u8)
                    .to_decimals(8)
                    .to_const::<8>();

                match self
                    .exchange_rates
                    .rates
                    .entry(self.exchange_rates.last_updated_at)
                {
                    Entry::Occupied(mut e) => {
                        e.get_mut().insert(Ticker::from(ticker_from), usd_rate);
                    }
                    Entry::Vacant(e) => {
                        let mut m = BTreeMap::new();
                        m.insert(Ticker::from(ticker_from), usd_rate);

                        e.insert(m);
                    }
                }
            }
        }
    }
}
