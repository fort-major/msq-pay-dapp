use candid::{CandidType, Principal};
use serde::Deserialize;

use super::types::ArchivedInvoice;

#[derive(CandidType, Deserialize)]
pub struct PushBatchRequest {
    pub batch: Vec<ArchivedInvoice>,
}

#[derive(CandidType, Deserialize)]
pub struct PushBatchResponse {}

#[derive(CandidType, Deserialize)]
pub struct GetInvoiceRequest {
    pub idx: u64,
}

pub type GetInvoiceResponse = Result<ArchivedInvoice, Principal>;

#[derive(CandidType, Deserialize)]
pub struct SetNextRequest {
    next: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct SetNextResponse {}
