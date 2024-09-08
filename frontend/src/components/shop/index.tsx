import { ROOT } from "@/routes";
import { BalanceOf } from "@components/balance-of";
import { Btn } from "@components/btn";
import { Copyable } from "@components/copyable";
import { EIconKind, Icon } from "@components/icon";
import { Spoiler } from "@components/spoiler";
import { TextInput } from "@components/text-input";
import { TransferOwnershipModal } from "@components/transfer-ownership-modal";
import { Principal } from "@dfinity/principal";
import { useNavigate } from "@solidjs/router";
import { useAuth } from "@store/auth";
import { IMyReferredShop, IMyShop, useShops } from "@store/shops";
import { useTokens } from "@store/tokens";
import { COLORS } from "@utils/colors";
import { logInfo } from "@utils/error";
import { EDs } from "@utils/math";
import { Result } from "@utils/types";
import { createEffect, createMemo, createSignal, For, on, onMount, Show } from "solid-js";

export interface IShopProps {
  info: IMyShop;
}

export const Shop = (props: IShopProps) => {
  const { supportedTokens, balanceOf, withdrawProfit, fetchShopSubaccount, shopSubaccounts } = useTokens();
  const { isAuthorized } = useAuth();
  const { updateShopInfo, fetchMyShops } = useShops();

  const [newInvoiceCreatorId, setNewInvoiceCreatorId] = createSignal(Result.Err<string>(""));
  const [transferOwnershipModalVisible, setTransferOwnershipModalVisible] = createSignal(false);

  const shopSubaccount = () => shopSubaccounts[props.info.id.toString()];
  const shopAccountOwner = () => Principal.fromText(import.meta.env.VITE_PAYMENT_HUB_CANISTER_ID);
  const tokenIds = createMemo(() => Object.keys(supportedTokens));

  onMount(() => {
    if (!isAuthorized()) return;

    const sub = shopSubaccount();
    if (sub) return;

    fetchShopSubaccount(props.info.id);
  });

  createEffect(
    on(isAuthorized, (ready) => {
      if (!ready) return;

      const sub = shopSubaccount();
      if (sub) return;

      fetchShopSubaccount(props.info.id);
    })
  );

  const canWithdrawAny = () => {
    const sub = shopSubaccount();
    if (!sub) return false;

    for (let tokenIdStr in supportedTokens) {
      const info = supportedTokens[tokenIdStr]!;

      const balance = balanceOf(info.id, shopAccountOwner(), sub);
      if (balance === undefined) continue;

      if (EDs.new(balance, info.fee.decimals).ge(info.fee.mulNum(6n))) return true;
    }

    return false;
  };

  const canSubmitInvoiceCreator = () => {
    const idRes = newInvoiceCreatorId();

    if (idRes.isErr()) return false;
    if (!isAuthorized()) return false;

    const idStr = idRes.unwrapOk();
    if (!idStr) return false;

    const id = Principal.fromText(idStr);

    const idx = props.info.invoiceCreators.findIndex((it) => it.compareTo(id) === "eq");
    if (idx !== -1) return false;

    return true;
  };

  const submitInvoiceCreator = async () => {
    const newCreator = Principal.fromText(newInvoiceCreatorId().unwrapOk());
    const creators = [...props.info.invoiceCreators, newCreator];

    await updateShopInfo({
      id: props.info.id,
      newInvoiceCreators: creators,
    });

    setNewInvoiceCreatorId(Result.Err<string>(""));

    fetchMyShops();
  };

  const removeInvoiceCreator = async (id: Principal) => {
    const creators = props.info.invoiceCreators.filter((it) => it.compareTo(id) !== "eq");

    await updateShopInfo({
      id: props.info.id,
      newInvoiceCreators: creators,
    });

    fetchMyShops();
  };

  const handleWithdrawClick = async () => {
    for (let tokenIdStr in supportedTokens) {
      const info = supportedTokens[tokenIdStr]!;

      const balance = balanceOf(info.id, shopAccountOwner(), shopSubaccount());
      if (balance === undefined) continue;

      const b = EDs.new(balance, info.fee.decimals);

      if (b.lt(info.fee.mulNum(6n))) continue;

      logInfo(`Withdrawing ${info.ticker}`);

      await withdrawProfit(props.info.id, info.id, b.sub(info.fee).toDecimals(8).val);

      logInfo(`Success!`);
    }
  };

  const handleTransferOwnershipClick = () => {
    setTransferOwnershipModalVisible(true);
  };

  const handleTransferOwnershipModalClose = () => {
    setTransferOwnershipModalVisible(false);
  };

  onMount(() => {
    console.log("shop", props.info);
  });

  return (
    <>
      <div class="flex flex-col border border-gray-115 rounded-3xl p-6 gap-10">
        <ShopHeader
          id={props.info.id}
          iconSrc={props.info.iconBase64Src}
          name={props.info.name}
          description={props.info.description}
          editable
        />
        <div class="flex justify-between items-baseline">
          <p class="text-gray-140 text-sm">Total Earned</p>
          <p class="text-white font-semibold text-2xl">
            ${props.info.totalEarnedUsd.toDynamic().toDecimals(2).toString()}
          </p>
        </div>
        <div class="flex flex-col gap-4">
          <div class="flex flex-col gap-1">
            <p class="font-semibold text-white text-md">Invoice Creators</p>
            <p class="font-normal text-gray-140 text-xs">
              Principal IDs who are allowed to create invoices in your shop. Be very careful! Only add here Principal
              IDs you control.
            </p>
          </div>
          <div class="flex flex-col gap-2">
            <For
              each={props.info.invoiceCreators}
              fallback={<p class="font-normal text-gray-120 text-xs">Nothing here :(</p>}
            >
              {(creatorId) => (
                <div class="flex justify-between items-center gap-4">
                  <div class="flex gap-4 items-center">
                    <Copyable text={creatorId.toText()} />
                  </div>
                  <Icon
                    kind={EIconKind.Minus}
                    color={COLORS.gray[140]}
                    class="cursor-pointer"
                    hoverColor={COLORS.white}
                    onClick={() => removeInvoiceCreator(creatorId)}
                  />
                </div>
              )}
            </For>
          </div>
          <div class="flex flex-col gap-1">
            <div class="flex gap-10 items-center">
              <TextInput
                value={newInvoiceCreatorId().unwrap()}
                onChange={setNewInvoiceCreatorId}
                placeholder={import.meta.env.VITE_PAYMENT_HUB_CANISTER_ID}
                validations={[{ principal: null }]}
              />
              <Icon
                kind={EIconKind.Plus}
                disabled={!canSubmitInvoiceCreator()}
                color={canSubmitInvoiceCreator() ? COLORS.gray[190] : COLORS.gray[140]}
                hoverColor={canSubmitInvoiceCreator() ? COLORS.white : COLORS.gray[140]}
                class={canSubmitInvoiceCreator() ? "cursor-pointer" : ""}
                onClick={submitInvoiceCreator}
              />
            </div>
          </div>
        </div>
        <div class="flex flex-col gap-4">
          <p class="font-semibold text-white text-md">Balances</p>
          <div class="flex gap-10 items-start">
            <div class="flex gap-x-8 gap-y-2 flex-wrap">
              <Show when={shopSubaccount()}>
                <For each={tokenIds()}>
                  {(tokenId) => (
                    <BalanceOf
                      tokenId={Principal.fromText(tokenId)}
                      owner={shopAccountOwner()}
                      subaccount={shopSubaccount()}
                    />
                  )}
                </For>
              </Show>
            </div>
            <Btn bgColor={COLORS.orange} text="Withdraw" disabled={!canWithdrawAny()} onClick={handleWithdrawClick} />
          </div>
        </div>
        <Spoiler header="Danger Zone">
          <div class="flex flex-col gap-4">
            <p class="font-semibold text-gray-140 text-lg">Attention! These actions require extra care!</p>
            <Btn bgColor={COLORS.errorRed} text="Transfer Ownership" onClick={handleTransferOwnershipClick} />
          </div>
        </Spoiler>
      </div>
      <Show when={transferOwnershipModalVisible()}>
        <TransferOwnershipModal
          hasLeftovers={canWithdrawAny()}
          onClose={handleTransferOwnershipModalClose}
          shopId={props.info.id}
          name={props.info.name}
        />
      </Show>
    </>
  );
};

export interface IReferredShopProps {
  info: IMyReferredShop;
}

export const ReferredShop = (props: IReferredShopProps) => {
  return (
    <div class="flex flex-col border border-gray-115 rounded-3xl p-6 gap-10">
      <ShopHeader
        id={props.info.id}
        iconSrc={props.info.iconBase64Src}
        name={props.info.name}
        description={props.info.description}
      />

      <div class="flex justify-between items-baseline">
        <p class="text-gray-140 text-sm">Collected fees</p>
        <p class="text-white font-semibold text-2xl">
          ${props.info.referralEarningsUsd.toDynamic().toDecimals(2).toString()}
        </p>
      </div>
    </div>
  );
};

export const ShopHeader = (props: {
  id: bigint;
  iconSrc: string;
  name: string;
  description: string;
  editable?: boolean;
}) => {
  const navigate = useNavigate();

  const handleEditClick = () => {
    navigate(`${ROOT.$.shops.$.register.path}?id=${props.id.toString()}`);
  };

  return (
    <div class="flex items-start justify-between">
      <div class="flex gap-4 items-start">
        <img src={props.iconSrc} class="rounded-full h-12 w-12" />
        <div class="flex flex-col gap-2">
          <p class="font-semibold text-white text-2xl">{props.name}</p>
          <p class="font-normal text-gray-140 text-sm">{props.description}</p>
        </div>
      </div>
      <div class="flex flex-col gap-4">
        <Copyable before="ID:" text={props.id.toString()} />
        <Show when={props.editable}>
          <div class="flex gap-2 items-center cursor-pointer" onClick={handleEditClick}>
            <p class="text-sm font-semibold text-gray-140">Edit</p>
            <Icon kind={EIconKind.Edit} color={COLORS.gray[140]} hoverColor={COLORS.white} class="cursor-pointer" />
          </div>
        </Show>
      </div>
    </div>
  );
};
