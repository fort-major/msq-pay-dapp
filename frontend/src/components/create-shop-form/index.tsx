import { ROOT } from "@/routes";
import { Btn } from "@components/btn";
import { EIconKind } from "@components/icon";
import { ImgUploader } from "@components/img-uploader";
import { TextInput } from "@components/text-input";
import { Principal } from "@dfinity/principal";
import { useNavigate } from "@solidjs/router";
import { useAuth } from "@store/auth";
import { useShops } from "@store/shops";
import { COLORS } from "@utils/colors";
import { Result } from "@utils/types";
import { batch, createEffect, createSignal, Match, on, onMount, Show, Switch } from "solid-js";

export interface ICreateShopFormProps {
  id?: bigint;
  referral?: Principal;
}

export const CreateShopForm = (props: ICreateShopFormProps) => {
  const { shops, fetchMyShops, registerShop, updateShopInfo } = useShops();
  const { isAuthorized } = useAuth();
  const navigate = useNavigate();

  const [name, setName] = createSignal(Result.Err<string>(""));
  const [description, setDescription] = createSignal(Result.Err<string>(""));
  const [logoBase64Src, setLogoBase64Src] = createSignal(Result.Err<string>(""));
  const [referral, setReferral] = createSignal(Result.Err<string>(""));

  const shop = () => (props.id ? shops[props.id.toString()] : undefined);

  onMount(() => {
    if (isAuthorized() && props.id !== undefined && !shop()) fetchMyShops();
  });

  onMount(() => {
    if (props.referral === undefined) return;
    setReferral(Result.Ok(props.referral.toText()));
  });

  createEffect(
    on(isAuthorized, (ready) => {
      if (ready && props.id !== undefined && !shop()) fetchMyShops();
    })
  );

  createEffect(
    on(shop, (s) => {
      if (!s) return;
      if (props.id === undefined) return;

      batch(() => {
        setName(Result.Ok(s.name));
        setDescription(Result.Ok(s.description));
        setLogoBase64Src(Result.Ok(s.iconBase64Src));
      });
    })
  );

  const canSubmit = () => {
    if (name().isErr()) return false;
    if (description().isErr()) return false;
    if (logoBase64Src().isErr()) return false;
    if (referral().unwrap() && referral().isErr()) return false;

    return true;
  };

  const handleSubmitClick = async () => {
    if (props.id === undefined) {
      const ref = referral().unwrap();

      await registerShop({
        name: name().unwrapOk(),
        description: description().unwrapOk(),
        iconBase64Src: logoBase64Src().unwrapOk(),
        referal: ref ? Principal.fromText(ref) : undefined,
        invoiceCreators: [],
      });
    } else {
      await updateShopInfo({
        id: props.id,
        newName: name().unwrapOk(),
        newDescription: description().unwrapOk(),
        newIconBase64Src: logoBase64Src().unwrapOk(),
      });
    }

    fetchMyShops();
    navigate(ROOT.$.shops.path);
  };

  return (
    <div class="flex flex-col gap-10">
      <h1 class="font-semibold text-white text-6xl">
        <Switch>
          <Match when={props.id === undefined}>Register a New Shop</Match>
          <Match when={props.id !== undefined}>Edit Shop #{props.id!.toString()}</Match>
        </Switch>
      </h1>
      <div class="flex flex-col gap-8">
        <div class="flex flex-col gap-4">
          <div class="flex flex-col gap-1">
            <p class="font-semibold text-white text-lg">
              Shop Name <span class="text-errorRed">*</span>
            </p>
            <p class="font-normal text-gray-140 text-xs">
              Choose the name wisely! It is important to not mislead your buyers. If you already have a website or a
              known project, it is best to name your shop the same way.
            </p>
          </div>
          <TextInput
            placeholder="Fort Major DAO Merch"
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
            placeholder="We sell official FMJ merch: T-shirts, hoodies, cups, stickers and many more!"
            value={description().unwrap()}
            onChange={setDescription}
            validations={[{ required: null }, { minLen: 4 }, { maxLen: 256 }]}
          />
        </div>
        <div class="flex flex-col gap-3">
          <div class="flex flex-col gap-1">
            <p class="font-semibold text-white text-lg">
              Logo <span class="text-errorRed">*</span>
            </p>
            <p class="font-normal text-gray-140 text-xs">
              Upload a logo of your shop. 50x50 px square, no transparent bg, 1kb max, .jpg, .png, .svg.
            </p>
          </div>
          <ImgUploader validations={[{ required: null }, { maxSizeBytes: 2048 }]} onChange={setLogoBase64Src} />
        </div>
        <Show when={props.id === undefined}>
          <div class="flex flex-col gap-3">
            <div class="flex flex-col gap-1">
              <p class="font-semibold text-white text-lg">Referral</p>
            </div>
            <TextInput
              disabled={!!props.referral}
              value={referral().unwrap()}
              onChange={setReferral}
              validations={[{ principal: null }]}
            />
          </div>
        </Show>
      </div>

      <div class="flex justify-end">
        <Btn
          text={props.id === undefined ? "Register Shop" : "Submit Changes"}
          icon={EIconKind.Plus}
          bgColor={COLORS.orange}
          iconColor={canSubmit() ? COLORS.white : COLORS.gray[140]}
          disabled={!canSubmit()}
          onClick={handleSubmitClick}
        />
      </div>
    </div>
  );
};
