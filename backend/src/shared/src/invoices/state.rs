use std::collections::{hash_map::Entry, BTreeMap, BTreeSet, HashMap};

use candid::{CandidType, Nat, Principal};
use serde::Deserialize;

use crate::utils::{
    calc_shop_subaccount, InvoiceId, ShopId, Timestamp, TransferTxn, DEFAULT_TTL,
    ID_GENERATION_DOMAIN, MEMO_GENERATION_DOMAIN, USD,
};

use super::types::{Invoice, InvoiceStatus};

#[derive(Default, CandidType, Deserialize, Clone, Debug)]
pub struct InvoicesState {
    pub invoice_id_generator: InvoiceId,

    pub all_invoices: BTreeMap<InvoiceId, Invoice>,

    pub active_invoices: HashMap<Timestamp, BTreeSet<InvoiceId>>,
    pub inactive_invoices: BTreeSet<InvoiceId>,

    pub total_processed_in_usd: USD,
}

impl InvoicesState {
    #[inline]
    pub fn init_id_seed(&mut self, seed: &[u8]) {
        self.invoice_id_generator.copy_from_slice(seed);
    }

    pub fn create(
        &mut self,
        qty_usd: Nat,
        shop_id: ShopId,
        timestamp: Timestamp,
        exchange_rates_timestamp: Timestamp,
        caller: Principal,
    ) -> InvoiceId {
        let id = self.generate_id(&timestamp.to_le_bytes());

        let inv = Invoice {
            id,
            creator: caller,
            status: InvoiceStatus::Created { ttl: DEFAULT_TTL },
            qty_usd,
            exchange_rates_timestamp,
            created_at: timestamp,
            shop_id,
        };

        match self.active_invoices.entry(inv.exchange_rates_timestamp) {
            Entry::Occupied(mut e) => {
                e.get_mut().insert(id);
            }
            Entry::Vacant(e) => {
                let mut s = BTreeSet::new();
                s.insert(id);

                e.insert(s);
            }
        }

        self.all_invoices.insert(id, inv);

        id
    }

    pub fn verify_payment(
        &mut self,
        invoice_id: &InvoiceId,
        transfer_txn: TransferTxn,
        exchange_rate: Nat,
        this_canister_id: Principal,
        now: Timestamp,
    ) -> Result<(Invoice, bool), String> {
        let invoice = self
            .all_invoices
            .get_mut(invoice_id)
            .ok_or("Invoice not found".to_string())?;

        if !matches!(invoice.status, InvoiceStatus::VerifyPayment) {
            return Err("Invalid invoice status".to_string());
        }

        // check if the transfer was sent to the correct recepient
        let expected_recepient_principal = this_canister_id;
        let actual_recepient_principal = transfer_txn.to.owner;

        if expected_recepient_principal != actual_recepient_principal {
            return Err(format!(
                "Invalid recepient - funds are lost: expected {}, actual {}",
                expected_recepient_principal, actual_recepient_principal
            ));
        }

        let expected_shop_subaccount = calc_shop_subaccount(invoice.shop_id);
        let actual_shop_subaccount = transfer_txn.to.subaccount.unwrap_or([0u8; 32]);

        if actual_shop_subaccount != expected_shop_subaccount {
            return Err(format!(
                "Invalid recepient subaccount: expected {:?}, actual {:?}",
                expected_shop_subaccount, actual_shop_subaccount
            ));
        }

        // is memo valid
        let expected_memo = Self::make_invoice_memo(invoice_id);
        let actual_memo = transfer_txn.memo;

        if expected_memo != actual_memo {
            return Err(format!(
                "Txn memo field doesn't match the invoice one: expected {:?}, actual {:?}",
                expected_memo, actual_memo
            ));
        }

        // check if the sum sent is enough to cover the invoice
        let expected_qty_usd = invoice.qty_usd.clone();
        let actual_qty_usd = exchange_rate.clone() * transfer_txn.qty.clone();

        if actual_qty_usd < invoice.qty_usd {
            return Err(format!(
                "Insufficient transfer: expected (usd e8s) {}, actual (usd e8s) {}",
                expected_qty_usd, actual_qty_usd
            ));
        }

        invoice.status = InvoiceStatus::Paid {
            timestamp: now,
            token_id: transfer_txn.token_id,
            qty: transfer_txn.qty,
            exchange_rate,
        };

        // delete the invoice from the list of active invoices (which is segregated by exchange rate used)
        let active_invoices = self
            .active_invoices
            .get_mut(&invoice.exchange_rates_timestamp)
            .unwrap();

        active_invoices.remove(invoice_id);

        // move the invoice to paid list
        self.inactive_invoices.insert(*invoice_id);

        Ok((invoice.clone(), self.active_invoices.is_empty()))
    }

    pub fn prepare_archive_batch(&mut self, size: usize) -> Vec<Invoice> {
        let mut ids_to_archive = Vec::new();

        let mut i = 0;
        for id in self.inactive_invoices.iter() {
            if i == size {
                break;
            }

            ids_to_archive.push(*id);

            i += 1;
        }

        let mut batch = Vec::new();

        for id in ids_to_archive.iter() {
            self.inactive_invoices.remove(id);
            let invoice = self.all_invoices.remove(id).unwrap();

            batch.push(invoice);
        }

        batch
    }

    pub fn reapply_archive_batch(&mut self, batch: Vec<Invoice>) {
        for invoice in batch {
            self.inactive_invoices.insert(invoice.id);
            self.all_invoices.insert(invoice.id, invoice);
        }
    }

    fn generate_id(&mut self, salt: &[u8]) -> InvoiceId {
        blake3::Hasher::new()
            .update(&self.invoice_id_generator)
            .update(ID_GENERATION_DOMAIN)
            .update(salt)
            .finalize()
            .into()
    }

    fn make_invoice_memo(id: &InvoiceId) -> [u8; 32] {
        blake3::Hasher::new()
            .update(MEMO_GENERATION_DOMAIN)
            .update(id)
            .finalize()
            .into()
    }
}
