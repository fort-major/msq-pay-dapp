import { Backlink } from "@components/backlink";
import { Btn } from "@components/btn";
import { CreateShopForm } from "@components/create-shop-form";
import { EIconKind } from "@components/icon";
import { Page } from "@components/page";
import { Principal } from "@dfinity/principal";
import { useLocation } from "@solidjs/router";
import { useAuth } from "@store/auth";
import { COLORS } from "@utils/colors";
import { debugStringify } from "@utils/encoding";
import { ErrorCode, logErr } from "@utils/error";
import { createMemo, Match, Show, Switch } from "solid-js";

export const RegisterShopPage = () => {
  const { query } = useLocation();
  const { isAuthorized, authorize } = useAuth();

  const id = createMemo(() => (query["id"] ? BigInt(query["id"]) : undefined));
  const referral = createMemo(() => {
    const r = query["referral"];

    if (!r) return undefined;

    try {
      return Principal.fromText(r);
    } catch (e) {
      logErr(ErrorCode.VALIDATION, debugStringify(e));
    }

    return undefined;
  });

  return (
    <Page slim class="flex-grow" outerClass="items-center justify-center">
      <Switch>
        <Match when={isAuthorized()}>
          <Show when={!referral()}>
            <Backlink />
          </Show>
          <CreateShopForm id={id()} referral={referral()} />
        </Match>
        <Match when={!isAuthorized()}>
          <div class="flex flex-grow self-stretch items-center justify-center">
            <div class="flex gap-5 items-end">
              <div class="flex flex-col self-stretch justify-evenly gap-5">
                <h2 class="font-semibold text-4xl text-white">Welcome to MSQ.Pay</h2>
                <p class="font-normal text-xl">
                  Effortlessly collect payments in top cryptocurrencies from the Internet Computer ecosystem. Invoice in
                  USD, get paid in crypto, and withdraw your earnings anytime. No buyer fees—only a small charge when
                  you withdraw.
                </p>
              </div>
              <div class="relative min-w-[320px] min-h-[320px] flex items-end justify-center">
                <img class="absolute w-full top-0 right-0" src="/wallet-illustration.svg" />
                <Btn
                  text="Sign in to get started!"
                  class="text-black font-semibold relative bottom-8"
                  bgColor={COLORS.chartreuse}
                  icon={EIconKind.MetaMask}
                  onClick={authorize}
                />
              </div>
            </div>
          </div>
        </Match>
      </Switch>
    </Page>
  );
};
