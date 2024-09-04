use candid::{CandidType, Principal};
use icrc_ledger_types::icrc1::account::Account;
use serde::Deserialize;
use sha2::Digest;

use crate::e8s::EDs;

pub const USD_DECIMALS: u8 = 8;
pub const DEFAULT_TTL: u8 = 1;
pub const RECYCLING_TTL: u8 = 0;

pub const ID_GENERATION_DOMAIN: &[u8] = b"msq-id-generation";
pub const MEMO_GENERATION_DOMAIN: &[u8] = b"msq-memo_generation";
pub const SHOP_ID_SUBACCOUNT_DOMAIN: &[u8] = b"msq-shop-id-subaccount";
pub const EXCHANGE_RATES_CANISTER_ID: &str = "uf6dk-hyaaa-aaaaq-qaaaq-cai";

pub type Timestamp = u64;
pub type InvoiceId = [u8; 32];
pub type ShopId = u64;
pub type TokenId = Principal;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferTxn {
    pub from: Account,
    pub to: Account,
    pub qty: EDs,
    pub token_id: TokenId,
    pub memo: [u8; 32],
    pub fee: EDs,
}

pub fn calc_shop_subaccount(shop_id: ShopId) -> [u8; 32] {
    let mut hasher = sha2::Sha256::new();

    hasher.update(SHOP_ID_SUBACCOUNT_DOMAIN);
    hasher.update(&shop_id.to_le_bytes());

    hasher.finalize().into()
}
