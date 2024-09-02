import { ROOT } from "@/routes";
import { Btn } from "@components/btn";
import { EIconKind } from "@components/icon";
import { Page } from "@components/page";
import { Shop } from "@components/shop";
import { A } from "@solidjs/router";
import { useShops } from "@store/shops";
import { COLORS } from "@utils/colors";
import { createMemo, For } from "solid-js";

export const ShopsPage = () => {
  const { shops } = useShops();

  const shopIds = createMemo(() => Object.keys(shops));

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
          <Btn text="Register" class="bg-orange" icon={EIconKind.Plus} iconColor={COLORS.white} />
        </A>
      </div>
      <div class="flex flex-col gap-6">
        <p class="font-semibold text-white text-6xl">Your Shops</p>

        <For fallback={shopsFallback()} each={shopIds()}>
          {(id) => <Shop info={shops[id]!} />}
        </For>
      </div>
    </Page>
  );
};
