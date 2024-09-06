use std::cell::RefCell;

use candid::{CandidType, Nat};
use futures::join;
use ic_cdk::{
    api::time,
    caller, export_candid, id, init, post_upgrade, pre_upgrade, query, spawn,
    storage::{stable_restore, stable_save},
    update,
};
use ic_e8s::d::EDs;
use icrc_ledger_types::icrc1::{account::Account, transfer::TransferArg};
use serde::Deserialize;
use shared::{
    exchange_rates::api::{GetExchangeRatesRequest, GetExchangeRatesResponse},
    invoices::{
        api::{
            CreateInvoiceRequest, CreateInvoiceResponse, GetInvoiceRequest, GetInvoiceResponse,
            VerifyPaymentRequest, VerifyPaymentResponse,
        },
        types::InvoiceStatus,
    },
    payment_hub::state::State,
    shops::api::{
        GetMyReferredShopsRequest, GetMyReferredShopsResponse, GetMyShopsRequest,
        GetMyShopsResponse, GetShopByIdRequest, GetShopByIdResponse, RegisterShopRequest,
        RegisterShopResponse, UpdateShopRequest, UpdateShopRespose, WithdrawProfitRequest,
        WithdrawProfitResponse,
    },
    supported_tokens::{
        api::{
            AddSupportedTokenRequest, AddSupportedTokenResponse, GetSupportedTokensRequest,
            GetSupportedTokensResponse, RemoveSupportedTokenRequest, RemoveSupportedTokenResponse,
        },
        types::Token,
    },
    utils::calc_shop_subaccount,
};
use timers::init_timers;
use utils::{
    get_current_exchange_rate_timestamp, icrc3_block_to_transfer_txn, init_invoice_ids_seed,
    init_supported_tokens, refresh_exchange_rates, set_immediate, ICRC1CanisterClient,
};

mod timers;
mod utils;

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();
}

#[derive(CandidType, Deserialize)]
struct InitArgs {
    supported_tokens: Vec<Token>,
    fee_collector_account: Option<Account>,
    should_mock_exchange_rates: bool,
}

#[init]
fn init_hook(args: InitArgs) {
    STATE.with_borrow_mut(|s| {
        s.set_fee_collector_account(args.fee_collector_account);
        s.exchange_rates
            .set_should_mock(args.should_mock_exchange_rates);
    });

    init_timers();
    init_supported_tokens(args.supported_tokens);

    set_immediate(|| {
        spawn(refresh_exchange_rates());
        spawn(init_invoice_ids_seed());
    });
}

#[pre_upgrade]
fn pre_upgrade_hook() {
    STATE.with_borrow(|s| stable_save((s,)).expect("Unable to stable_save"));
}

#[post_upgrade]
fn post_upgrade_hook() {
    let (state,): (State,) = stable_restore().expect("Unable to stable_restore");

    STATE.with_borrow_mut(|s| *s = state);

    init_timers();

    set_immediate(|| {
        spawn(refresh_exchange_rates());
        spawn(init_invoice_ids_seed());
    });
}

#[query]
fn get_exchange_rates(req: GetExchangeRatesRequest) -> GetExchangeRatesResponse {
    let rates = STATE.with_borrow(|it| {
        Some(
            it.exchange_rates
                .get_rates(req.timestamp)?
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<Vec<_>>(),
        )
    });

    GetExchangeRatesResponse { rates }
}

#[query]
fn get_invoice(req: GetInvoiceRequest) -> GetInvoiceResponse {
    let invoice_opt =
        STATE.with_borrow(|it| it.invoices.all_invoices.get(&req.invoice_id).cloned());

    GetInvoiceResponse { invoice_opt }
}

#[update]
fn create_invoice(req: CreateInvoiceRequest) -> CreateInvoiceResponse {
    let can_create = STATE.with_borrow(|s| s.shops.can_create_invoices(&req.shop_id, &caller()));
    if !can_create {
        panic!("Access denied");
    }

    let exchange_rates_timestamp = get_current_exchange_rate_timestamp();

    let invoice_id = STATE.with_borrow_mut(|it| {
        it.invoices.create(
            req.qty_usd,
            req.shop_id,
            time(),
            exchange_rates_timestamp,
            caller(),
        )
    });

    CreateInvoiceResponse { invoice_id }
}

#[update]
async fn verify_payment(req: VerifyPaymentRequest) -> VerifyPaymentResponse {
    let (exchange_rates_timestamp, ttl, decimals) = STATE.with_borrow_mut(|s| {
        let invoice = s
            .invoices
            .all_invoices
            .get_mut(&req.invoice_id)
            .ok_or("Access denied".to_string())?;

        if invoice.creator != caller() {
            return Err("Access denied".to_string());
        }

        let ttl = match invoice.status {
            InvoiceStatus::Created { ttl } => ttl,
            _ => return Err("The invoice is already paid".to_string()),
        };

        let decimals = s
            .supported_tokens
            .get_by_id(&req.asset_id)
            .ok_or("Token not found")?
            .fee
            .decimals;

        invoice.status = InvoiceStatus::VerifyPayment;

        Ok((invoice.exchange_rates_timestamp, ttl, decimals))
    })?;

    let token = ICRC1CanisterClient::new(req.asset_id);
    let block = token.find_block(req.block_idx).await?;

    let txn = icrc3_block_to_transfer_txn(&block, req.asset_id, decimals)?;

    let ticker = STATE
        .with_borrow(|s| s.supported_tokens.ticker_by_token_id(&txn.token_id))
        .ok_or("Unsuported token".to_string())?;

    let exchange_rate = STATE.with_borrow(|s| {
        s.exchange_rates
            .get_exchange_rate(&exchange_rates_timestamp, &ticker)
            .clone()
    });

    let result = STATE.with_borrow_mut(|s| {
        s.invoices
            .verify_payment(&req.invoice_id, txn, exchange_rate, id(), time())
    });

    match result {
        // if failed, reset the invoice and return the error
        Err(err) => STATE.with_borrow_mut(|s| {
            let invoice = s.invoices.all_invoices.get_mut(&req.invoice_id).unwrap();
            invoice.status = InvoiceStatus::Created { ttl };

            Err(err)
        }),
        // if succeed, maybe delete outdated and return the invoice
        Ok((invoice, should_delete_outdated)) => {
            // if the active invoice list is empty now - delete the outdated exchange rates
            if should_delete_outdated {
                STATE.with_borrow_mut(|s| {
                    s.exchange_rates
                        .delete_outdated(&invoice.exchange_rates_timestamp);

                    let shop = s.shops.shops.get_mut(&invoice.shop_id).unwrap();
                    shop.total_earned_usd += &invoice.qty_usd;
                });
            }

            Ok(invoice)
        }
    }
}

#[update]
pub fn register_shop(req: RegisterShopRequest) -> RegisterShopResponse {
    // TODO: validate req

    let id = STATE.with_borrow_mut(|s| {
        s.shops.create_shop(
            req.invoice_creators,
            req.name,
            req.description,
            req.icon_base64,
            req.referal,
            caller(),
        )
    });

    RegisterShopResponse { shop_id: id }
}

#[update]
pub fn update_shop(req: UpdateShopRequest) -> UpdateShopRespose {
    // TODO: validate req

    STATE
        .with_borrow_mut(|s| {
            s.shops.update_shop(
                req.id,
                req.new_owner_opt,
                req.new_invoice_creators_opt,
                req.new_name_opt,
                req.new_description_opt,
                req.new_icon_base64_opt,
                caller(),
            )
        })
        .expect("Unable to update shop");

    UpdateShopRespose {}
}

#[query]
pub fn get_my_shops(_req: GetMyShopsRequest) -> GetMyShopsResponse {
    let shops = STATE.with_borrow(|s| s.shops.get_shops_by_owner(&caller()));

    GetMyShopsResponse { shops }
}

#[query]
pub fn get_my_referred_shops(_req: GetMyReferredShopsRequest) -> GetMyReferredShopsResponse {
    let shops = STATE.with_borrow(|s| s.shops.get_shops_by_referral(&caller()));

    GetMyReferredShopsResponse { shops }
}

#[query]
pub fn get_shop_by_id(req: GetShopByIdRequest) -> GetShopByIdResponse {
    let shop = STATE.with_borrow(|s| s.shops.shops.get(&req.id).map(|it| it.as_pub()));

    GetShopByIdResponse { shop }
}

#[update]
pub async fn withdraw_profit(req: WithdrawProfitRequest) -> WithdrawProfitResponse {
    // TODO: validate request

    let owner = STATE.with_borrow(|s| {
        s.shops
            .shops
            .get(&req.shop_id)
            .expect("Shop not found")
            .owner
    });

    if owner != caller() {
        panic!("Access Denied");
    }

    let system_fee = STATE
        .with_borrow(|s| {
            s.supported_tokens
                .get_by_id(&req.asset_id)
                .map(|it| it.fee.clone())
        })
        .expect("Unsupported token");

    let min_qty = &system_fee * 6u64;
    let qty = req.qty.to_dynamic().to_decimals(system_fee.decimals);

    if qty < min_qty {
        panic!("Insufficient funds");
    }

    let (fee_collector_account_opt, referral_opt) = STATE.with_borrow(|s| {
        let fee_collector = s.fee_collector_account;
        let referral = s.shops.get_referral(&req.shop_id);

        (fee_collector, referral)
    });

    // fmj gets 3% fee from the withdrawn amount, referal gets 20% fee from the fmj fee
    let (withdraw_qty, fmj_fee, referal_fee) = match (fee_collector_account_opt, referral_opt) {
        (Some(_), Some(_)) => {
            let withdraw_qty = &qty * 97u64 / 100u64;
            let fmj_fee = (&qty - &withdraw_qty) * 80u64 / 100u64;
            let referal_fee = &qty - &withdraw_qty - &fmj_fee;

            (withdraw_qty, fmj_fee, referal_fee)
        }
        (None, Some(_)) => {
            let withdraw_qty = &qty * 97u64 / 100u64;
            let fmj_fee = EDs::from((0u64, qty.decimals));
            let referal_fee = &qty - &withdraw_qty;

            (withdraw_qty, fmj_fee, referal_fee)
        }
        (Some(_), None) => {
            let withdraw_qty = &qty * 97u64 / 100u64;
            let fmj_fee = &qty - &withdraw_qty;
            let referal_fee = EDs::from((0u64, qty.decimals));

            (withdraw_qty, fmj_fee, referal_fee)
        }
        (None, None) => {
            let withdraw_qty = qty;
            let fmj_fee = EDs::from((0u64, withdraw_qty.decimals));
            let referal_fee = EDs::from((0u64, withdraw_qty.decimals));

            (withdraw_qty, fmj_fee, referal_fee)
        }
    };

    let token = ICRC1CanisterClient::new(req.asset_id);
    let shop_subaccount = calc_shop_subaccount(req.shop_id);

    let withdraw_transfer_future = async {
        let call_result = token
            .icrc1_transfer(TransferArg {
                from_subaccount: Some(shop_subaccount),
                to: req.to,
                fee: Some((&system_fee).into()),
                memo: req.memo,
                created_at_time: None,
                amount: (withdraw_qty - &system_fee).into(),
            })
            .await;

        if let Err((code, msg)) = call_result {
            return Err(format!(
                "Unable to make an ICRC-1 transfer call to {}: [{:?}] {}",
                req.asset_id, code, msg
            ));
        }

        let (transfer_result,) = call_result.unwrap();

        if let Err(transfer_err) = transfer_result {
            return Err(format!(
                "Unable to make an ICRC-1 transfer to {}: {}",
                req.asset_id, transfer_err
            ));
        }

        let block_idx = transfer_result.unwrap();

        Ok(block_idx)
    };

    let fmj_fee_transfer_future = async {
        if let Some(fee_collector_account) = fee_collector_account_opt {
            let _result = token
                .icrc1_transfer(TransferArg {
                    from_subaccount: Some(shop_subaccount),
                    to: fee_collector_account,
                    fee: Some((&system_fee).into()),
                    memo: None,
                    created_at_time: None,
                    amount: (fmj_fee - &system_fee).into(),
                })
                .await;
        }

        // a stupid way of defining the type
        if false {
            return Err(String::new());
        } else {
            Ok(Nat::from(0u8))
        }
    };

    let referal_fee_transfer_future = async {
        if let Some(referral) = referral_opt {
            let result = token
                .icrc1_transfer(TransferArg {
                    from_subaccount: Some(shop_subaccount),
                    to: Account {
                        owner: referral,
                        subaccount: None,
                    },
                    fee: Some((&system_fee).into()),
                    memo: None,
                    created_at_time: None,
                    amount: (&referal_fee - &system_fee).into(),
                })
                .await;

            if result.is_ok() {
                STATE.with_borrow_mut(|s| {
                    let earnings = s
                        .shops
                        .referral_to_shops
                        .get_mut(&referral)
                        .unwrap()
                        .get_mut(&req.shop_id)
                        .unwrap();

                    *earnings += referal_fee.to_decimals(8).to_const();
                })
            }
        }

        if false {
            return Err(String::new());
        } else {
            Ok(Nat::from(0u8))
        }
    };

    // complete all transfers in once (this may lead to problems)
    let (withdraw_transfer_result, _, _) = join!(
        withdraw_transfer_future,
        fmj_fee_transfer_future,
        referal_fee_transfer_future
    );

    // only fail if the withdraw fails, ignore other failures
    match withdraw_transfer_result {
        Err(qty_transfer_error) => panic!("{}", qty_transfer_error),
        Ok(block_idx) => WithdrawProfitResponse { block_idx },
    }
}

#[query]
fn get_supported_tokens(_req: GetSupportedTokensRequest) -> GetSupportedTokensResponse {
    let supported_tokens =
        STATE.with_borrow(|s| s.supported_tokens.get().cloned().collect::<Vec<_>>());

    GetSupportedTokensResponse { supported_tokens }
}

#[update]
fn add_supported_token(req: AddSupportedTokenRequest) -> AddSupportedTokenResponse {
    STATE.with_borrow_mut(|it| it.supported_tokens.add_token(req.token));

    AddSupportedTokenResponse {}
}

#[update]
fn remove_supported_token(req: RemoveSupportedTokenRequest) -> RemoveSupportedTokenResponse {
    STATE.with_borrow_mut(|it| it.supported_tokens.remove_token(req.ticker));

    RemoveSupportedTokenResponse {}
}

export_candid!();
