import { ImgUploader } from "@components/img-uploader";
import { TextInput } from "@components/text-input";
import { useAuth } from "@store/auth";
import { useShops } from "@store/shops";
import { Result } from "@utils/types";
import { createEffect, createSignal, Match, on, onMount, Switch } from "solid-js";

export interface ICreateShopFormProps {
  id?: bigint;
}

export const CreateShopForm = (props: ICreateShopFormProps) => {
  const { shops, fetchMyShops } = useShops();
  const { isAuthorized } = useAuth();

  const [name, setName] = createSignal(Result.Err<string>(""));
  const [description, setDescription] = createSignal(Result.Err<string>(""));
  const [avatarBase64Src, setAvatarBase64Src] = createSignal(Result.Err<string>(""));

  const shop = () => (props.id ? shops[props.id.toString()] : undefined);

  onMount(() => {
    if (isAuthorized() && props.id !== undefined && !shop()) fetchMyShops();
  });

  createEffect(
    on(isAuthorized, (ready) => {
      if (ready && props.id !== undefined && !shop()) fetchMyShops();
    })
  );

  return (
    <div class="flex flex-col gap-10">
      <h1 class="font-semibold text-white text-6xl">
        <Switch>
          <Match when={props.id === undefined}>Register a New Shop</Match>
          <Match when={props.id !== undefined}>Edit Shop #{props.id!.toString()}</Match>
        </Switch>
      </h1>
      <div class="flex flex-col gap-4">
        <div class="flex flex-col gap-1">
          <p class="font-semibold text-white text-lg">
            Shop Name <span class="text-errorRed">*</span>
          </p>
          <p class="font-normal text-gray-140 text-xs">
            Choose the name wisely! It is important to not mislead your buyers. If you already have a website or a known
            project, it is best to name your shop the same way.
          </p>
        </div>
        <TextInput
          value={name().unwrap()}
          onChange={setName}
          validations={[{ required: null }, { minLen: 4 }, { maxLen: 128 }]}
        />
      </div>
      <div class="flex flex-col gap-3">
        <div class="flex flex-col gap-1">
          <p class="font-semibold text-white text-lg">
            Short Description <span class="text-errorRed">*</span>
          </p>
          <p class="font-normal text-gray-140 text-xs">
            Briefly describe your shop. Tell your buyers what you're selling and what other services you offer.
          </p>
        </div>
        <TextInput
          value={description().unwrap()}
          onChange={setDescription}
          validations={[{ required: null }, { minLen: 4 }, { maxLen: 512 }]}
        />
      </div>
      <div class="flex flex-col gap-3">
        <div class="flex flex-col gap-1">
          <p class="font-semibold text-white text-lg">
            Shop Logo <span class="text-errorRed">*</span>
          </p>
          <p class="font-normal text-gray-140 text-xs">
            Upload a logo of your shop. 50x50 px square, no transparent bg, 1kb max, .jpg, .png, .svg.
          </p>
        </div>
        <ImgUploader validations={[{ required: null }, { maxSizeBytes: 2048 }]} onChange={setAvatarBase64Src} />
      </div>
    </div>
  );
};
