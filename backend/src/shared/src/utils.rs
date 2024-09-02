use candid::{CandidType, Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;
use serde::Deserialize;

pub const USD_DECIMALS: u8 = 8;
pub const DEFAULT_TTL: u8 = 1;
pub const RECYCLING_TTL: u8 = 0;

pub const ID_GENERATION_DOMAIN: &[u8] = b"msq-id-generation";
pub const MEMO_GENERATION_DOMAIN: &[u8] = b"msq-memo_generation";
pub const SHOP_ID_SUBACCOUNT_DOMAIN: &[u8] = b"msq-shop-id-subaccount";
pub const EXCHANGE_RATES_CANISTER_ID: &str = "u45jl-liaaa-aaaam-abppa-cai";

pub fn f64_to_usd(f: f64) -> USD {
    if f.is_nan() || f.is_infinite() || f.is_sign_negative() || f == 0f64 {
        panic!("Invalid f64 - {f}");
    }

    USD::from((f * 10f64.powi(USD_DECIMALS as i32)) as u128)
}

pub type Timestamp = u64;
pub type USD = Nat;
pub type InvoiceId = [u8; 32];
pub type ShopId = u64;
pub type TokenId = Principal;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferTxn {
    pub from: Account,
    pub to: Account,
    pub qty: Nat,
    pub token_id: TokenId,
    pub memo: [u8; 32],
    pub fee: Nat,
}

pub fn calc_shop_subaccount(shop_id: ShopId) -> [u8; 32] {
    blake3::Hasher::new()
        .update(SHOP_ID_SUBACCOUNT_DOMAIN)
        .update(&shop_id.to_le_bytes())
        .finalize()
        .into()
}
