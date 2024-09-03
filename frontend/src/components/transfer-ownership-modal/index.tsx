import { Btn } from "@components/btn";
import { Modal } from "@components/modal";
import { TextInput } from "@components/text-input";
import { Principal } from "@dfinity/principal";
import { ShopId, useShops } from "@store/shops";
import { COLORS } from "@utils/colors";
import { Result } from "@utils/types";
import { createSignal } from "solid-js";

export interface ITransferOwnershipModal {
  shopId: ShopId;
  name: string;
  hasLeftovers: boolean;
  onClose: () => void;
}

export const TransferOwnershipModal = (props: ITransferOwnershipModal) => {
  const { updateShopInfo, fetchMyShops } = useShops();

  const [newOwnerId, setNewOnwerId] = createSignal(Result.Err<string>(""));

  const canTransfer = () => {
    if (props.hasLeftovers) return false;

    if (newOwnerId().isErr()) return false;

    const idStr = newOwnerId().unwrapOk();
    if (idStr === "") return false;

    return true;
  };

  const handleTransferClick = async () => {
    const agreed = confirm("Are you sure? Double-check the new owner ID!");
    if (!agreed) return;

    const id = Principal.fromText(newOwnerId().unwrapOk());

    await updateShopInfo({
      id: props.shopId,
      newOwner: id,
    });

    fetchMyShops();

    props.onClose();
  };

  return (
    <Modal title="Transfer Shop Ownership" onClose={props.onClose}>
      <div class="flex flex-col gap-8">
        <div class="flex flex-col gap-2">
          <p class="font-normal text-lg">
            You are about to transfer ownership over <span>{props.name}</span>!
          </p>
          <p class="text-sm text-gray-140">You won't be able to control this shop or withdraft profit anymore!</p>
        </div>

        <div class="flex flex-col gap-2">
          <p class="font-semibold text-sm text-gray-140">Enter the Principal ID of the new owner to continue:</p>
          <TextInput
            placeholder={import.meta.env.VITE_PAYMENT_HUB_CANISTER_ID}
            value={newOwnerId().unwrap()}
            onChange={setNewOnwerId}
            disabled={props.hasLeftovers}
          />
        </div>
        <Btn
          onClick={handleTransferClick}
          disabled={!canTransfer()}
          bgColor={COLORS.orange}
          text={
            props.hasLeftovers ? "Withdraw leftover profit to continue" : `Transfer Shop #${props.shopId.toString()}`
          }
        />
      </div>
    </Modal>
  );
};
