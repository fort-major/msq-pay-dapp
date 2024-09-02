use std::collections::BTreeSet;

use candid::{CandidType, Principal};
use serde::Deserialize;

use crate::utils::ShopId;

#[derive(CandidType, Deserialize, Clone)]
pub struct Shop {
    pub id: ShopId,
    pub owner: Principal,
    pub invoice_creators: BTreeSet<Principal>,
    pub name: String,
    pub description: String,
    pub icon_base64: String,
    pub referal: Option<Principal>,
}
