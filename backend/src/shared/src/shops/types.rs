use std::collections::BTreeSet;

use candid::{CandidType, Principal};
use serde::Deserialize;

use crate::{e8s::E8s, utils::ShopId};

#[derive(CandidType, Deserialize, Clone)]
pub struct Shop {
    pub id: ShopId,
    pub owner: Principal,
    pub invoice_creators: BTreeSet<Principal>,
    pub name: String,
    pub description: String,
    pub icon_base64: String,
    pub referral: Option<Principal>,
    pub total_earned_usd: E8s,
}

impl Shop {
    pub fn as_referred(&self, referral_earnings_usd: E8s) -> ReferredShop {
        ReferredShop {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            icon_base64: self.icon_base64.clone(),
            referral: self.referral.unwrap(),
            referral_earnings_usd,
        }
    }

    pub fn as_pub(&self) -> PubShop {
        PubShop {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            icon_base64: self.icon_base64.clone(),
        }
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub struct ReferredShop {
    pub id: ShopId,
    pub name: String,
    pub description: String,
    pub icon_base64: String,
    pub referral: Principal,
    pub referral_earnings_usd: E8s,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct PubShop {
    pub id: ShopId,
    pub name: String,
    pub description: String,
    pub icon_base64: String,
}
