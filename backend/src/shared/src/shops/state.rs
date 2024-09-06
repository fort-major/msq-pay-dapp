use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};

use candid::{CandidType, Principal};

use ic_e8s::c::E8s;
use serde::Deserialize;

use crate::utils::ShopId;

use super::types::{ReferredShop, Shop};

#[derive(CandidType, Deserialize, Default)]
pub struct ShopsState {
    pub shop_id_generator: ShopId,
    pub shops: BTreeMap<ShopId, Shop>,
    pub owner_to_shops: BTreeMap<Principal, BTreeSet<ShopId>>,
    pub referral_to_shops: BTreeMap<Principal, BTreeMap<ShopId, E8s>>,
}

impl ShopsState {
    pub fn create_shop(
        &mut self,
        invoice_creators: BTreeSet<Principal>,
        name: String,
        description: String,
        icon_base64: String,
        referral_opt: Option<Principal>,
        caller: Principal,
    ) -> ShopId {
        let id = self.generate_shop_id();
        let shop = Shop {
            id,
            owner: caller,
            invoice_creators,
            name,
            description,
            icon_base64,
            referral: referral_opt,
            total_earned_usd: E8s::zero(),
        };

        self.shops.insert(id, shop);

        match self.owner_to_shops.entry(caller) {
            Entry::Occupied(mut e) => {
                e.get_mut().insert(id);
            }
            Entry::Vacant(e) => {
                let mut s = BTreeSet::new();
                s.insert(id);

                e.insert(s);
            }
        };

        if let Some(referral) = referral_opt {
            match self.referral_to_shops.entry(referral) {
                Entry::Occupied(mut e) => {
                    e.get_mut().insert(id, E8s::zero());
                }
                Entry::Vacant(e) => {
                    let mut s = BTreeMap::new();
                    s.insert(id, E8s::zero());

                    e.insert(s);
                }
            }
        }

        id
    }

    pub fn update_shop(
        &mut self,
        id: ShopId,
        new_owner_opt: Option<Principal>,
        new_invoice_creators_opt: Option<BTreeSet<Principal>>,
        new_name_opt: Option<String>,
        new_description_opt: Option<String>,
        new_icon_base64_opt: Option<String>,
        caller: Principal,
    ) -> Result<(), String> {
        let shop = self.shops.get_mut(&id).ok_or(format!("Shop not found"))?;

        if shop.owner != caller {
            return Err(format!("Access denied"));
        }

        if let Some(new_owner) = new_owner_opt {
            self.owner_to_shops
                .get_mut(&shop.owner)
                .as_mut()
                .ok_or(format!("Unreachable - no owner to shop relation found"))?
                .remove(&id);

            match self.owner_to_shops.entry(new_owner) {
                Entry::Occupied(mut e) => {
                    e.get_mut().insert(id);
                }
                Entry::Vacant(e) => {
                    let mut s = BTreeSet::new();
                    s.insert(id);

                    e.insert(s);
                }
            };

            shop.owner = new_owner;
        }

        if let Some(new_invoice_creators) = new_invoice_creators_opt {
            shop.invoice_creators = new_invoice_creators;
        }

        if let Some(new_name) = new_name_opt {
            shop.name = new_name;
        }

        if let Some(new_description) = new_description_opt {
            shop.description = new_description;
        }

        if let Some(new_icon_base64) = new_icon_base64_opt {
            shop.icon_base64 = new_icon_base64;
        }

        Ok(())
    }

    pub fn get_referral(&self, shop_id: &ShopId) -> Option<Principal> {
        let shop = self.shops.get(shop_id)?;

        shop.referral
    }

    pub fn get_shops_by_referral(&self, referral: &Principal) -> Vec<ReferredShop> {
        if let Some(ids) = self.referral_to_shops.get(referral) {
            ids.iter()
                .map(|(id, earnings)| {
                    self.shops
                        .get(id)
                        .map(|it| it.as_referred(earnings.clone()))
                        .unwrap()
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_shops_by_owner(&self, owner: &Principal) -> Vec<Shop> {
        if let Some(ids) = self.owner_to_shops.get(owner) {
            ids.iter()
                .map(|id| self.shops.get(id).cloned().unwrap())
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn can_create_invoices(&self, shop_id: &ShopId, caller: &Principal) -> bool {
        if let Some(shop) = self.shops.get(shop_id) {
            shop.invoice_creators.contains(caller)
        } else {
            false
        }
    }

    fn generate_shop_id(&mut self) -> ShopId {
        let val = self.shop_id_generator;
        self.shop_id_generator += 1;

        return val;
    }
}
