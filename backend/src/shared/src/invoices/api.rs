use candid::{CandidType, Nat, Principal};
use serde::Deserialize;

use crate::utils::{InvoiceId, ShopId, USD};

use super::types::Invoice;

#[derive(CandidType, Deserialize)]
pub struct GetInvoiceRequest {
    pub invoice_id: InvoiceId,
}

#[derive(CandidType, Deserialize)]
pub struct GetInvoiceResponse {
    pub invoice_opt: Option<Invoice>,
}

#[derive(CandidType, Deserialize)]
pub struct CreateInvoiceRequest {
    pub shop_id: ShopId,
    pub qty_usd: USD,
}

#[derive(CandidType, Deserialize)]
pub struct CreateInvoiceResponse {
    pub invoice_id: InvoiceId,
}

#[derive(CandidType, Deserialize)]
pub struct VerifyPaymentRequest {
    pub invoice_id: InvoiceId,
    pub asset_id: Principal,
    pub block_idx: Nat,
}

pub type VerifyPaymentResponse = Result<Invoice, String>;
