use candid::{CandidType, Nat, Principal};
use ic_cdk::call;
use ic_e8s::{c::E8s, d::EDs};
use serde::Deserialize;

pub type InvoiceId = [u8; 32];

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum InvoiceStatus {
    Created {
        ttl: u8,
    },
    VerifyPayment,
    Paid {
        timestamp: u64,
        token_id: Principal,
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
    pub created_at: u64,
    pub exchange_rates_timestamp: u64,
    pub shop_id: u64,
}

#[derive(CandidType, Deserialize)]
pub struct GetInvoiceRequest {
    pub invoice_id: InvoiceId,
}

#[derive(CandidType, Deserialize)]
pub struct GetInvoiceResponse {
    pub invoice_opt: Option<Invoice>,
}

#[derive(CandidType, Deserialize)]
pub struct CreateInvoiceRequest {
    pub shop_id: u64,
    pub qty_usd: E8s,
}

#[derive(CandidType, Deserialize)]
pub struct CreateInvoiceResponse {
    pub invoice_id: InvoiceId,
}

#[derive(CandidType, Deserialize)]
pub struct VerifyPaymentRequest {
    pub invoice_id: InvoiceId,
    pub asset_id: Principal,
    pub block_idx: Nat,
}

pub type VerifyPaymentResponse = Result<Invoice, String>;

pub const MSQ_PAY_CANISTER_ID: &str = "dqerg-34aaa-aaaaa-qaapq-cai";
pub const CREATE_INVOICE_METHOD: &str = "create_invoice";
pub const GET_INVOICE_METHOD: &str = "get_invoice";
pub const VERIFY_PAYMENT_METHOD: &str = "verify_payment";

pub struct InterCanisterClient(pub Principal);

impl InterCanisterClient {
    pub fn new(canister_id_opt: Option<Principal>) -> Self {
        if let Some(canister_id) = canister_id_opt {
            Self(canister_id)
        } else {
            Self(Principal::from_text(MSQ_PAY_CANISTER_ID).unwrap())
        }
    }

    pub async fn create_invoice(
        &self,
        shop_id: u64,
        qty_usd_e8s: Nat,
    ) -> Result<InvoiceId, String> {
        let arg = CreateInvoiceRequest {
            shop_id,
            qty_usd: E8s::new(qty_usd_e8s.0),
        };

        let (resp,) = call::<(CreateInvoiceRequest,), (CreateInvoiceResponse,)>(
            self.0,
            CREATE_INVOICE_METHOD,
            (arg,),
        )
        .await
        .map_err(|(code, msg)| format!("Unable to call MSQ Pay canister: [{:?}] {}", code, msg))?;

        Ok(resp.invoice_id)
    }

    pub async fn get_invoice(&self, invoice_id: InvoiceId) -> Result<Option<Invoice>, String> {
        let arg = GetInvoiceRequest { invoice_id };

        let (resp,) =
            call::<(GetInvoiceRequest,), (GetInvoiceResponse,)>(self.0, GET_INVOICE_METHOD, (arg,))
                .await
                .map_err(|(code, msg)| {
                    format!("Unable to call MSQ Pay canister: [{:?}] {}", code, msg)
                })?;

        Ok(resp.invoice_opt)
    }

    pub async fn verify_payment(
        &self,
        invoice_id: InvoiceId,
        token_id: Principal,
        block_idx: Nat,
    ) -> Result<Invoice, String> {
        let arg = VerifyPaymentRequest {
            invoice_id,
            asset_id: token_id,
            block_idx,
        };

        let (resp,) = call::<(VerifyPaymentRequest,), (VerifyPaymentResponse,)>(
            self.0,
            VERIFY_PAYMENT_METHOD,
            (arg,),
        )
        .await
        .map_err(|(code, msg)| format!("Unable to call MSQ Pay canister: [{:?}] {}", code, msg))?;

        resp
    }
}
