use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum ArchivedInvoice {
    V0001(ArchivedInvoiceV0001),
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ArchivedInvoiceV0001 {}
