use candid::{CandidType, Principal};
use serde::Deserialize;

use crate::{
    e8s::{E8s, EDs},
    utils::{InvoiceId, ShopId, Timestamp, TokenId},
};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum InvoiceStatus {
    Created {
        ttl: u8,
    },
    VerifyPayment,
    Paid {
        timestamp: Timestamp,
        token_id: TokenId,
        qty: EDs,
        exchange_rate: EDs,
    },
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Invoice {
    pub id: InvoiceId,
    pub status: InvoiceStatus,
    pub creator: Principal,
    pub qty_usd: E8s,
    pub created_at: Timestamp,
    pub exchange_rates_timestamp: Timestamp,
    pub shop_id: ShopId,
}
