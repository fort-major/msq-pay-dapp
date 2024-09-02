import { createContext, createEffect, on, useContext } from "solid-js";
import { IChildren } from "../utils/types";
import { ErrorCode, err, logInfo } from "../utils/error";
import { newPaymentHubActor, opt, optUnwrap } from "../utils/backend";
import { createStore, Store } from "solid-js/store";
import { useAuth } from "./auth";
import { Principal } from "@dfinity/principal";
import { calcShopSubaccount } from "@utils/security";

export type ShopId = bigint;
export type ShopIdStr = string;

export interface IShop {
  id: bigint;
  iconBase64Src: string;
  owner: Principal;
  name: string;
  description: string;
  invoiceCreators: Array<Principal>;
  referal?: Principal;
  subaccount: Uint8Array;
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
  shops: Store<Partial<Record<ShopIdStr, IShop>>>;
  fetchMyShops: () => Promise<void>;
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

  const [shops, setShops] = createStore<IShopsStoreContext["shops"]>();

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
      const iShop: IShop = {
        id: shop.id,
        name: shop.name,
        description: shop.description,
        iconBase64Src: shop.icon_base64,
        owner: shop.owner,
        invoiceCreators: shop.invoice_creators,
        referal: optUnwrap(shop.referal),
        subaccount: await calcShopSubaccount(shop.id),
      };

      setShops(shop.id.toString(), iShop);
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
    <ShopsContext.Provider value={{ shops, registerShop, updateShopInfo, fetchMyShops }}>
      {props.children}
    </ShopsContext.Provider>
  );
}
