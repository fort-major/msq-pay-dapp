use std::time::Duration;

use candid::{Nat, Principal};
use futures::FutureExt;
use ic_cdk::{
    api::{
        call::{call_with_payment, CallResult},
        management_canister::main::raw_rand,
        time,
    },
    call,
};
use ic_cdk_timers::set_timer;
use ic_e8s::d::EDs;
use ic_xrc_types::{
    Asset, AssetClass, ExchangeRate, ExchangeRateMetadata, GetExchangeRateRequest,
    GetExchangeRateResult,
};
use icrc_ledger_types::{
    icrc::generic_value::ICRC3Value,
    icrc1::{
        account::Account,
        transfer::{BlockIndex, TransferArg, TransferError},
    },
    icrc3::blocks::{BlockWithId, GetBlocksRequest, GetBlocksResult},
};
use shared::{
    supported_tokens::types::Token,
    utils::{Timestamp, TransferTxn, EXCHANGE_RATES_CANISTER_ID},
};

use crate::STATE;

const XRC_ATTACHED_CYCLES: u64 = 1_000_000_000u64;
const XRC_QUOTE_ASSET: &str = "USD";

pub fn set_immediate(func: impl FnOnce() + 'static) {
    set_timer(Duration::ZERO, func);
}

pub async fn fetch_exchange_rates() -> Vec<ExchangeRate> {
    let (should_mock, tickers) = STATE.with_borrow(|s| {
        let should_mock = s.exchange_rates.should_mock();
        let tickers: Vec<_> = s
            .supported_tokens
            .get()
            .map(|it| (it.ticker, it.xrc_ticker))
            .collect();

        (should_mock, tickers)
    });

    if should_mock {
        return tickers
            .into_iter()
            .map(|(ticker, _xrc_ticker)| ExchangeRate {
                base_asset: Asset {
                    symbol: ticker.0.to_string(),
                    class: AssetClass::Cryptocurrency,
                },
                quote_asset: Asset {
                    symbol: XRC_QUOTE_ASSET.to_string(),
                    class: AssetClass::FiatCurrency,
                },
                timestamp: time(),
                rate: time() % 1_0000_0000u64,
                metadata: ExchangeRateMetadata {
                    decimals: 8,
                    base_asset_num_queried_sources: 10,
                    base_asset_num_received_rates: 10,
                    quote_asset_num_queried_sources: 10,
                    quote_asset_num_received_rates: 10,
                    standard_deviation: 1000u64,
                    forex_timestamp: None,
                },
            })
            .collect();
    }

    let xrc_id = Principal::from_text(EXCHANGE_RATES_CANISTER_ID).expect("Invalid xrc canister id");

    let mut results = Vec::new();

    for (ticker, xrc_ticker) in tickers {
        let args = GetExchangeRateRequest {
            base_asset: Asset {
                symbol: xrc_ticker.0.to_string(),
                class: AssetClass::Cryptocurrency,
            },
            quote_asset: Asset {
                symbol: XRC_QUOTE_ASSET.to_string(),
                class: AssetClass::FiatCurrency,
            },
            timestamp: None,
        };

        let base_symbol = ticker.0.to_string();

        let res = call_with_payment::<(GetExchangeRateRequest,), (GetExchangeRateResult,)>(
            xrc_id,
            "get_exchange_rate",
            (args,),
            XRC_ATTACHED_CYCLES,
        )
        .then(|it| async {
            match it {
                Ok((resp,)) => match resp {
                    Ok(mut rate) => {
                        rate.base_asset.symbol = base_symbol;

                        Some(rate)
                    }
                    _ => None,
                },
                _ => None,
            }
        })
        .await;

        if let Some(result) = res {
            results.push(result);
        }
    }

    results
}

#[derive(Clone, Copy)]
pub struct ICRC1CanisterClient {
    pub canister_id: Principal,
}

impl ICRC1CanisterClient {
    pub fn new(canister_id: Principal) -> Self {
        Self { canister_id }
    }

    pub async fn icrc1_transfer(
        &self,
        arg: TransferArg,
    ) -> CallResult<(Result<BlockIndex, TransferError>,)> {
        call(self.canister_id, "icrc1_transfer", (arg,)).await
    }

    pub async fn icrc3_get_blocks(&self, arg: GetBlocksRequest) -> CallResult<(GetBlocksResult,)> {
        call(self.canister_id, "icrc3_get_blocks", (vec![arg],)).await
    }

    pub async fn find_block(&self, idx: Nat) -> Result<BlockWithId, String> {
        let (mut get_blocks_result,) = self
            .icrc3_get_blocks(GetBlocksRequest {
                start: idx.clone(),
                length: Nat::from(1u64),
            })
            .await
            .map_err(|e| {
                format!(
                    "Unable to fetch ICRC3 blocks of token {}: [{:?}] {}",
                    self.canister_id, e.0, e.1
                )
            })?;

        if get_blocks_result.log_length < idx {
            return Err(format!(
                "Block {} does not exist (total block len {})",
                idx, get_blocks_result.log_length
            ));
        }

        // loop over archives until the block is found
        while get_blocks_result.blocks.get(0).is_none() {
            let archive = get_blocks_result
                .archived_blocks
                .get(0)
                .ok_or("No good archive found for the block".to_string())?;

            (get_blocks_result,) = call(
                archive.callback.canister_id,
                &archive.callback.method,
                (GetBlocksRequest {
                    start: idx.clone(),
                    length: Nat::from(1u64),
                },),
            )
            .await
            .map_err(|e| {
                format!(
                    "Unable to fetch ICRC3 blocks of token {}: [{:?}] {}",
                    self.canister_id, e.0, e.1
                )
            })?;
        }

        let block = get_blocks_result.blocks.remove(0);
        if block.id != idx {
            return Err(format!("Invalid block id from an ICRC-3 ledger"));
        }

        Ok(block)
    }
}

pub fn icrc3_block_to_transfer_txn(
    block: &BlockWithId,
    token_id: Principal,
    token_decimals: u8,
) -> Result<TransferTxn, String> {
    match &block.block {
        ICRC3Value::Map(block_fields) => {
            let btype_is_1xfer = block_fields
                .get("btype")
                .map(|it| match it {
                    ICRC3Value::Text(v) => v == "1xfer",
                    _ => false,
                })
                .unwrap_or(false);

            let tx = block_fields
                .get("tx")
                .ok_or("No 'tx' field found in block".to_string())?;

            match tx {
                ICRC3Value::Map(tx_fields) => {
                    let tx_op_is_xfer = tx_fields
                        .get("op")
                        .map(|it| match it {
                            ICRC3Value::Text(v) => v == "xfer",
                            _ => false,
                        })
                        .unwrap_or(false);

                    if !(tx_op_is_xfer || btype_is_1xfer) {
                        return Err("Invalid txn type".to_string());
                    }

                    let amount_val = tx_fields
                        .get("amt")
                        .ok_or("The block contains no 'amt' field".to_string())?;
                    let amount = match amount_val {
                        ICRC3Value::Nat(a) => a,
                        _ => return Err("Invalid 'amt' field".to_string()),
                    };

                    let to_val = tx_fields
                        .get("to")
                        .ok_or("The block contains no 'to' field".to_string())?;
                    let to = match to_val {
                        ICRC3Value::Array(to_arr) => {
                            let to_owner_val = to_arr
                                .get(0)
                                .ok_or("No recepient principal found in the block".to_string())?;
                            let to_subaccount_val = to_arr
                                .get(1)
                                .ok_or("No recepient subaccount found in the block".to_string())?;

                            let to_owner = match to_owner_val {
                                ICRC3Value::Blob(b) => Principal::from_slice(b.as_slice()),
                                _ => return Err("Invalid 'to_owner' field".to_string()),
                            };
                            let to_subaccount_slice = match to_subaccount_val {
                                ICRC3Value::Blob(b) => b.as_slice(),
                                _ => return Err("Invalid 'to_subaccount' field".to_string()),
                            };

                            let mut to_subaccount = [0u8; 32];
                            to_subaccount.copy_from_slice(&to_subaccount_slice);

                            Account {
                                owner: to_owner,
                                subaccount: Some(to_subaccount),
                            }
                        }
                        _ => return Err("Invalid 'to' field".to_string()),
                    };

                    let from_val = tx_fields
                        .get("from")
                        .ok_or("The block contains no 'from' field".to_string())?;

                    let from = match from_val {
                        ICRC3Value::Array(from_arr) => {
                            let from_owner_val = from_arr
                                .get(0)
                                .ok_or("No sender principal found in the block".to_string())?;
                            let from_subaccount = from_arr.get(1).map(|it| {
                                let from_subaccount_slice = match it {
                                    ICRC3Value::Blob(b) => b.as_slice(),
                                    _ => unreachable!("Invalid 'from_subaccount' field"),
                                };

                                let mut from_subaccount = [0u8; 32];
                                from_subaccount.copy_from_slice(&from_subaccount_slice);

                                from_subaccount
                            });

                            let from_owner = match from_owner_val {
                                ICRC3Value::Blob(b) => Principal::from_slice(b.as_slice()),
                                _ => return Err("Invalid 'from_owner' field".to_string()),
                            };

                            Account {
                                owner: from_owner,
                                subaccount: from_subaccount,
                            }
                        }
                        _ => return Err("Invalid 'to' field".to_string()),
                    };

                    let memo_val = tx_fields
                        .get("memo")
                        .ok_or("The block contains no 'memo' field".to_string())?;

                    let memo = match memo_val {
                        ICRC3Value::Blob(b) => {
                            let mut res = [0u8; 32];
                            res.copy_from_slice(b.as_slice());

                            res
                        }
                        _ => return Err("Invalid 'memo' field".to_string()),
                    };

                    Ok(TransferTxn {
                        from,
                        to,
                        qty: EDs::new(amount.0.clone(), token_decimals),
                        token_id,
                        memo,
                    })
                }
                _ => Err("Invalid tx format".to_string()),
            }
        }
        _ => Err("Invalid block format".to_string()),
    }
}

pub fn init_supported_tokens(tokens: Vec<Token>) {
    STATE.with_borrow_mut(|s| {
        for t in tokens {
            s.supported_tokens.add_token(t);
        }
    });
}

pub async fn archive_inactive_invoices() {
    let batch = STATE.with_borrow_mut(|s| s.invoices.prepare_archive_batch(100));

    // TODO: make external call
    // TODO: if failed - reapply batch

    STATE.with_borrow_mut(|s| s.invoices.reapply_archive_batch(batch));
}

/**
 * It should be safe to invoke this function up to once every minute - the rest of the system is ready for multiple concurrent
 * exchange rates being present in it. In this scenario, each created invoice will use the most actual exchange rate available,
 * locking on it until it is either paid or garbage collected.
 */
pub async fn refresh_exchange_rates() {
    let external_rates = fetch_exchange_rates().await;

    STATE.with_borrow_mut(|s| {
        s.update_exchange_rates(external_rates, time());
    });
}

pub fn get_current_exchange_rate_timestamp() -> Timestamp {
    STATE.with_borrow(|it| it.exchange_rates.last_updated_at)
}

#[inline]
pub fn garbage_collect_invoices() {
    STATE.with_borrow_mut(|s| s.purge_expired_invoices());
}

#[inline]
pub async fn init_invoice_ids_seed() {
    let (rand,) = raw_rand().await.unwrap();

    STATE.with_borrow_mut(|it| it.invoices.init_id_seed(&rand));
}
