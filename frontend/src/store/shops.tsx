import { createContext, createEffect, on, useContext } from "solid-js";
import { IChildren } from "../utils/types";
import { ErrorCode, err, logInfo } from "../utils/error";
import { newPaymentHubActor, opt, optUnwrap } from "../utils/backend";
import { createStore, Store } from "solid-js/store";
import { useAuth } from "./auth";
import { Principal } from "@dfinity/principal";
import { E8s } from "@utils/math";

export type ShopId = bigint;
export type ShopIdStr = string;

export interface IMyShop {
  id: bigint;
  iconBase64Src: string;
  owner: Principal;
  name: string;
  description: string;
  invoiceCreators: Array<Principal>;
  referral?: Principal;
  totalEarnedUsd: E8s;
}

export interface IMyReferredShop {
  id: bigint;
  iconBase64Src: string;
  name: string;
  description: string;
  referral: Principal;
  referralEarningsUsd: E8s;
}

export type IRegisterShopArgs = {
  name: string;
  description: string;
  iconBase64Src: string;
  invoiceCreators: Principal[];
  referal?: Principal;
};

export type IUpdateShopInfoArgs = {
  id: ShopId;
  newName?: string;
  newDescription?: string;
  newIconBase64Src?: string;
  newInvoiceCreators?: Principal[];
  newOwner?: Principal;
};

export interface IShopsStoreContext {
  myShops: Store<Partial<Record<ShopIdStr, IMyShop>>>;
  myReferredShops: Store<Partial<Record<ShopIdStr, IMyReferredShop>>>;
  fetchMyShops: () => Promise<void>;
  fetchMyReferredShops: () => Promise<void>;
  registerShop: (args: IRegisterShopArgs) => Promise<void>;
  updateShopInfo: (args: IUpdateShopInfoArgs) => Promise<void>;
}

const ShopsContext = createContext<IShopsStoreContext>();

export function useShops(): IShopsStoreContext {
  const ctx = useContext(ShopsContext);

  if (!ctx) {
    err(ErrorCode.UNREACHEABLE, "Shops context is not initialized");
  }

  return ctx;
}

export function ShopsStore(props: IChildren) {
  const { assertAuthorized, agent, disable, enable } = useAuth();

  const [myShops, setMyShops] = createStore<IShopsStoreContext["myShops"]>();
  const [myReferredShops, setMyReferredShops] = createStore<IShopsStoreContext["myReferredShops"]>();

  createEffect(
    on(agent, (agent) => {
      if (!agent) return;

      fetchMyShops();
    })
  );

  const fetchMyShops: IShopsStoreContext["fetchMyShops"] = async () => {
    assertAuthorized();

    const actor = newPaymentHubActor(agent()!);
    const { shops } = await actor.get_my_shops({});

    for (let shop of shops) {
      const iShop: IMyShop = {
        id: shop.id,
        name: shop.name,
        description: shop.description,
        iconBase64Src: shop.icon_base64,
        owner: shop.owner,
        invoiceCreators: shop.invoice_creators,
        referral: optUnwrap(shop.referral),
        totalEarnedUsd: E8s.new(shop.total_earned_usd),
      };

      setMyShops(shop.id.toString(), iShop);
    }
  };

  const fetchMyReferredShops: IShopsStoreContext["fetchMyReferredShops"] = async () => {
    assertAuthorized();

    const actor = newPaymentHubActor(agent()!);
    const { shops } = await actor.get_my_referred_shops({});

    for (let shop of shops) {
      const iShop: IMyReferredShop = {
        id: shop.id,
        name: shop.name,
        description: shop.description,
        iconBase64Src: shop.icon_base64,
        referral: shop.referral,
        referralEarningsUsd: E8s.new(shop.referral_earnings_usd),
      };

      setMyReferredShops(shop.id.toString(), iShop);
    }
  };

  const registerShop: IShopsStoreContext["registerShop"] = async (args) => {
    assertAuthorized();

    disable();

    try {
      const actor = newPaymentHubActor(agent()!);
      const { shop_id } = await actor.register_shop({
        name: args.name,
        description: args.description,
        icon_base64: args.iconBase64Src,
        invoice_creators: args.invoiceCreators,
        referal: opt(args.referal),
      });

      logInfo(`Shop #${shop_id.toString()} is registered!`);

      fetchMyShops();
    } finally {
      enable();
    }
  };

  const updateShopInfo: IShopsStoreContext["updateShopInfo"] = async (args) => {
    assertAuthorized();

    disable();

    try {
      const actor = newPaymentHubActor(agent()!);
      await actor.update_shop({
        id: args.id,
        new_name_opt: opt(args.newName),
        new_description_opt: opt(args.newDescription),
        new_icon_base64_opt: opt(args.newIconBase64Src),
        new_invoice_creators_opt: opt(args.newInvoiceCreators),
        new_owner_opt: opt(args.newOwner),
      });

      logInfo(`Shop #${args.id.toString()} info is updated!`);

      fetchMyShops();
    } finally {
      enable();
    }
  };

  return (
    <ShopsContext.Provider
      value={{ myShops, myReferredShops, registerShop, updateShopInfo, fetchMyShops, fetchMyReferredShops }}
    >
      {props.children}
    </ShopsContext.Provider>
  );
}
