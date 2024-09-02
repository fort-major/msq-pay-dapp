use std::cell::RefCell;

use ic_cdk::{
    caller, export_candid, post_upgrade, pre_upgrade, query,
    storage::{stable_restore, stable_save},
    update,
};
use shared::{
    invoice_history::{
        api::{GetInvoiceRequest, GetInvoiceResponse, PushBatchRequest, PushBatchResponse},
        state::State,
    },
    ENV_VARS,
};

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
}

#[pre_upgrade]
fn pre_upgrade_hook() {
    STATE
        .with_borrow(|s| stable_save((s,)))
        .expect("Unable to stable_save");
}

#[post_upgrade]
fn post_upgrade_hook() {
    let (state,): (State,) = stable_restore().expect("Unable to stable_restore");

    STATE.with_borrow_mut(|s| *s = state);
}

#[update(guard=only_parent)]
fn push_batch(req: PushBatchRequest) -> PushBatchResponse {
    STATE.with_borrow_mut(|it| {
        it.log.extend(req.batch);
    });

    PushBatchResponse {}
}

#[query]
fn get_invoice(req: GetInvoiceRequest) -> GetInvoiceResponse {
    STATE.with_borrow(|state| {
        let entry = state.log.get((req.idx) as usize).expect("No invoice found");

        Ok(entry.clone())
    })
}

fn only_parent() -> Result<(), String> {
    if caller() == ENV_VARS.payment_hub_canister_id {
        Ok(())
    } else {
        Err(String::from("Access denied"))
    }
}

export_candid!();
