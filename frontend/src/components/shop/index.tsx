import { BalanceOf } from "@components/balance-of";
import { Btn } from "@components/btn";
import { Copyable } from "@components/copyable";
import { EIconKind, Icon } from "@components/icon";
import { Spoiler } from "@components/spoiler";
import { TextInput } from "@components/text-input";
import { TransferOwnershipModal } from "@components/transfer-ownership-modal";
import { Principal } from "@dfinity/principal";
import { useAuth } from "@store/auth";
import { IShop, useShops } from "@store/shops";
import { useTokens } from "@store/tokens";
import { COLORS } from "@utils/colors";
import { logInfo } from "@utils/error";
import { calcShopSubaccount } from "@utils/security";
import { Result } from "@utils/types";
import { createMemo, createResource, createSignal, For, Show } from "solid-js";

export interface IShopProps {
  info: IShop;
}

export const Shop = (props: IShopProps) => {
  const { supportedTokens, balanceOf, withdrawProfit } = useTokens();
  const { isAuthorized } = useAuth();
  const { updateShopInfo, fetchMyShops } = useShops();

  const [newInvoiceCreatorId, setNewInvoiceCreatorId] = createSignal(Result.Err<string>(""));
  const [transferOwnershipModalVisible, setTransferOwnershipModalVisible] = createSignal(false);

  const [shopSubaccount] = createResource(() => calcShopSubaccount(props.info.id));
  const shopAccountOwner = () => Principal.fromText(import.meta.env.VITE_PAYMENT_HUB_CANISTER_ID);
  const tokenIds = createMemo(() => Object.keys(supportedTokens));

  const canWithdrawAny = () => {
    for (let tokenIdStr in supportedTokens) {
      const info = supportedTokens[tokenIdStr]!;

      const balance = balanceOf(info.id, shopAccountOwner(), shopSubaccount());
      if (balance === undefined) continue;

      if (balance >= info.fee * 5n) return true;
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

      if (balance < info.fee * 5n) continue;

      logInfo(`Withdrawing ${info.ticker}`);

      await withdrawProfit(props.info.id, info.id, balance - info.fee);

      logInfo(`Success!`);
    }
  };

  const handleTransferOwnershipClick = () => {
    setTransferOwnershipModalVisible(true);
  };

  const handleTransferOwnershipModalClose = () => {
    setTransferOwnershipModalVisible(false);
  };

  return (
    <>
      <div class="flex flex-col border border-gray-115 rounded-3xl p-6 gap-10">
        <div class="flex items-start justify-between">
          <div class="flex gap-4 items-start">
            <img src={props.info.iconBase64Src} class="rounded-full h-12 w-12" />
            <div class="flex flex-col gap-2">
              <p class="font-semibold text-white text-2xl">{props.info.name}</p>
              <p class="font-normal text-gray-140 text-sm">{props.info.description}</p>
            </div>
          </div>
          <Copyable before="ID:" text={props.info.id.toString()} />
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
              <For each={tokenIds()}>
                {(tokenId) => (
                  <BalanceOf
                    tokenId={Principal.fromText(tokenId)}
                    owner={shopAccountOwner()}
                    subaccount={shopSubaccount()}
                  />
                )}
              </For>
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
