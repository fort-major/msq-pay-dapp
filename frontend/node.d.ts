interface ImportMeta {
  readonly env: {
    DEV: boolean;
    MODE: "dev" | "ic";
    VITE_PAYMENT_HUB_CANISTER_ID: string;
    VITE_INVOICE_HISTORY_CANISTER_ID: string;
    VITE_ROOT_KEY: string;
    VITE_IC_HOST: string;
  };
}
