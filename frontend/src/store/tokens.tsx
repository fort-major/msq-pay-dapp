import { createContext, createEffect, on, useContext } from "solid-js";
import { IChildren } from "../utils/types";
import { ErrorCode, err } from "../utils/error";
import { createStore, Store } from "solid-js/store";
import { useAuth } from "./auth";
import { Principal } from "@dfinity/principal";
import { E8s } from "@utils/math";
import { bytesToHex } from "@utils/encoding";
import { IcrcLedgerCanister, IcrcMetadataResponseEntries } from "@dfinity/ledger-icrc";
import { newPaymentHubActor, opt } from "@utils/backend";
import { Token } from "@/declarations/payment_hub/payment_hub.did";
import { ShopId } from "./shops";
import { calcShopSubaccount } from "@utils/security";

export type TPrincipalStr = string;
export type TSubaccountStr = string;
export type TSubaccount = Uint8Array;

export interface ITokensStoreContext {
  supportedTokens: Store<Partial<Record<TPrincipalStr, Token>>>;
  fetchSupportedTokens: () => Promise<void>;

  balances: Store<
    Partial<Record<TPrincipalStr, Partial<Record<TPrincipalStr, Partial<Record<TSubaccountStr, bigint>>>>>>
  >;
  balanceOf: (tokenId: Principal, owner: Principal, subaccount?: TSubaccount) => bigint | undefined;
  fetchBalanceOf: (tokenId: Principal, owner: Principal, subaccount?: TSubaccount) => Promise<void>;

  withdrawProfit: (shopId: ShopId, tokenId: Principal, qty: bigint) => Promise<void>;
}

const TokensContext = createContext<ITokensStoreContext>();

export function useTokens(): ITokensStoreContext {
  const ctx = useContext(TokensContext);

  if (!ctx) {
    err(ErrorCode.UNREACHEABLE, "Tokens context is not initialized");
  }

  return ctx;
}

export function TokensStore(props: IChildren) {
  const { assertReadyToFetch, assertAuthorized, anonymousAgent, isAuthorized, agent, identity } = useAuth();

  const [supportedTokens, setSupportedTokens] = createStore<ITokensStoreContext["supportedTokens"]>();
  const [balances, setBalances] = createStore<ITokensStoreContext["balances"]>();

  createEffect(
    on(anonymousAgent, (a) => {
      if (!a) return;

      fetchSupportedTokens();
    })
  );

  const balanceOf: ITokensStoreContext["balanceOf"] = (tokenId, owner, subaccount) => {
    return balances[tokenId.toText()]?.[owner.toText()]?.[bytesToHex(orDefaultSubaccount(subaccount))];
  };

  const fetchBalanceOf: ITokensStoreContext["fetchBalanceOf"] = async (tokenId, owner, subaccount) => {
    assertReadyToFetch();

    const ledger = IcrcLedgerCanister.create({ agent: anonymousAgent()!, canisterId: tokenId });
    const balance = await ledger.balance({ owner: owner, subaccount });

    const tId = tokenId.toText();
    const oId = owner.toText();
    const sub = bytesToHex(orDefaultSubaccount(subaccount));

    if (!balances[tId]) {
      setBalances(tId, {});
    }

    if (!balances[tId]?.[oId]) {
      setBalances(tId, oId, {});
    }

    setBalances(tId, oId, sub, balance);
  };

  const fetchSupportedTokens: ITokensStoreContext["fetchSupportedTokens"] = async () => {
    assertReadyToFetch();

    const actor = newPaymentHubActor(anonymousAgent()!);
    const { supported_tokens } = await actor.get_supported_tokens({});

    for (let token of supported_tokens) {
      setSupportedTokens(token.id.toText(), token);
    }
  };

  const withdrawProfit: ITokensStoreContext["withdrawProfit"] = async (shopId, tokenId, qty) => {
    assertAuthorized();

    const myId = identity()!.getPrincipal();
    const shopSubaccount = await calcShopSubaccount(shopId);

    const actor = newPaymentHubActor(agent()!);
    const { block_idx } = await actor.withdraw_profit({
      asset_id: tokenId,
      shop_id: shopId,
      qty,
      to: { owner: myId, subaccount: [] },
      memo: [],
    });

    fetchBalanceOf(tokenId, myId);
    fetchBalanceOf(tokenId, Principal.fromText(import.meta.env.VITE_PAYMENT_HUB_CANISTER_ID), shopSubaccount);
  };

  return (
    <TokensContext.Provider
      value={{
        balances,
        balanceOf,
        fetchBalanceOf,
        supportedTokens,
        fetchSupportedTokens,
        withdrawProfit,
      }}
    >
      {props.children}
    </TokensContext.Provider>
  );
}

function orDefaultSubaccount(subaccount?: TSubaccount): TSubaccount {
  return subaccount ? subaccount : new Uint8Array(32);
}
