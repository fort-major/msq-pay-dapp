use std::collections::BTreeSet;

use candid::{CandidType, Nat, Principal};
use ic_e8s::c::E8s;
use icrc_ledger_types::icrc1::{account::Account, transfer::Memo};
use serde::Deserialize;

use crate::utils::ShopId;

use super::types::{PubShop, ReferredShop, Shop};

#[derive(CandidType, Deserialize)]
pub struct RegisterShopRequest {
    pub invoice_creators: BTreeSet<Principal>,
    pub name: String,
    pub description: String,
    pub icon_base64: String,
    pub referal: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct RegisterShopResponse {
    pub shop_id: ShopId,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateShopRequest {
    pub id: ShopId,
    pub new_owner_opt: Option<Principal>,
    pub new_invoice_creators_opt: Option<BTreeSet<Principal>>,
    pub new_name_opt: Option<String>,
    pub new_description_opt: Option<String>,
    pub new_icon_base64_opt: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateShopRespose {}

#[derive(CandidType, Deserialize)]
pub struct GetMyShopsRequest {}

#[derive(CandidType, Deserialize)]
pub struct GetMyShopsResponse {
    pub shops: Vec<Shop>,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawProfitRequest {
    pub shop_id: ShopId,
    pub asset_id: Principal,
    pub to: Account,
    pub qty: E8s,
    pub memo: Option<Memo>,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawProfitResponse {
    pub block_idx: Nat,
}

#[derive(CandidType, Deserialize)]
pub struct GetMyReferredShopsRequest {}

#[derive(CandidType, Deserialize)]
pub struct GetMyReferredShopsResponse {
    pub shops: Vec<ReferredShop>,
}

#[derive(CandidType, Deserialize)]
pub struct GetShopByIdRequest {
    pub id: ShopId,
}

#[derive(CandidType, Deserialize)]
pub struct GetShopByIdResponse {
    pub shop: Option<PubShop>,
}
