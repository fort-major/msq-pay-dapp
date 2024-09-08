import { ROOT } from "@/routes";
import { Btn } from "@components/btn";
import { EIconKind } from "@components/icon";
import { Page } from "@components/page";
import { Shop } from "@components/shop";
import { A, useNavigate } from "@solidjs/router";
import { useAuth } from "@store/auth";
import { useShops } from "@store/shops";
import { COLORS } from "@utils/colors";
import { createEffect, createMemo, For, on, onMount } from "solid-js";

export const ShopsPage = () => {
  const { isAuthorized } = useAuth();
  const { myShops } = useShops();
  const navigate = useNavigate();

  const shopIds = createMemo(() => Object.keys(myShops));

  onMount(() => {
    if (!isAuthorized()) navigate(ROOT.path);
  });

  createEffect(
    on(isAuthorized, (ready) => {
      if (!ready) navigate(ROOT.path);
    })
  );

  const shopsFallback = () => (
    <div class="flex flex-col gap-2">
      <p class="font-semibold text-gray-140 text-md">No Shops Found :(</p>
      <p class="font-semibold text-gray-115 text-sm">
        Try creating one, refreshing the page or{" "}
        <a class="underline" href="https://t.me/fortmajoricp/16" target="_blank">
          contacting us
        </a>
        .
      </p>
    </div>
  );

  return (
    <Page slim>
      <div class="flex gap-10 items-center justify-between">
        <div class="flex flex-col gap-1">
          <p class="font-semibold text-white text-4xl">Register a Shop</p>
          <p class="font-normal text-gray-140 text-lg">and start accepting cryptocurrencies in no time!</p>
        </div>
        <A href={ROOT.$.shops.$.register.path}>
          <Btn text="Register" bgColor={COLORS.orange} icon={EIconKind.Plus} iconColor={COLORS.white} />
        </A>
      </div>
      <div class="flex flex-col gap-6">
        <p class="font-semibold text-white text-6xl">Your Shops</p>

        <For fallback={shopsFallback()} each={shopIds()}>
          {(id) => <Shop info={myShops[id]!} />}
        </For>
      </div>
    </Page>
  );
};
