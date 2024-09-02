use candid::{CandidType, Nat, Principal};
use serde::Deserialize;

use crate::utils::{InvoiceId, ShopId, Timestamp, TokenId, USD};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum InvoiceStatus {
    Created {
        ttl: u8,
    },
    VerifyPayment,
    Paid {
        timestamp: Timestamp,
        token_id: TokenId,
        qty: Nat,
        exchange_rate: USD,
    },
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Invoice {
    pub id: InvoiceId,
    pub status: InvoiceStatus,
    pub creator: Principal,
    pub qty_usd: Nat,
    pub created_at: Timestamp,
    pub exchange_rates_timestamp: Timestamp,
    pub shop_id: ShopId,
}
