import { ROOT } from "@/routes";
import { Avatar } from "@components/avatar";
import { BalanceOf } from "@components/balance-of";
import { Btn } from "@components/btn";
import { Copyable } from "@components/copyable";
import { EIconKind, Icon } from "@components/icon";
import { Page } from "@components/page";
import { useNavigate } from "@solidjs/router";
import { useAuth } from "@store/auth";
import { useTokens } from "@store/tokens";
import { COLORS } from "@utils/colors";
import { logInfo } from "@utils/error";
import { eventHandler } from "@utils/security";
import { createEffect, createMemo, createResource, For, on, Show } from "solid-js";

export const MePage = () => {
  const { isAuthorized, identity, agent, deauthorize, autoAuth } = useAuth();
  const { supportedTokens } = useTokens();
  const navigate = useNavigate();

  const [pseudonym] = createResource(identity, (i) => i.getPseudonym());
  const [avatarSrc] = createResource(identity, (i) => i.getAvatarSrc());
  const principalId = () => identity()?.getPrincipal();
  const tokenIds = createMemo(() => Object.values(supportedTokens).map((it) => it!.id));
  const referralLink = () =>
    principalId()
      ? `${window.location.origin}${ROOT.$.shops.$.register.path}?referral=${principalId()!.toText()}`
      : undefined;

  const handleLinkCopyClick = eventHandler(() => {
    navigator.clipboard.writeText(referralLink()!);
    logInfo("Copied!");
  });

  createEffect(
    on(autoAuth, (status) => {
      if (status === "fail" || status === "unavailable") {
        navigate(ROOT.path);
      }
    })
  );

  return (
    <Page slim>
      <div class="flex gap-10 items-center justify-between">
        <div class="flex gap-5 items-center">
          <Avatar url={avatarSrc()} size="lg" borderColor={COLORS.chartreuse} />
          <div class="flex flex-col gap-3">
            <p class="font-semibold text-white text-4xl">{pseudonym() ? pseudonym() : "Anonymous"}</p>
            <Copyable text={principalId() ? principalId()!.toText() : "aaaaa-aa"} />
          </div>
        </div>
        <Show when={isAuthorized()}>
          <Btn text="Sign Out" bgColor={COLORS.chartreuse} class="text-black font-semibold" onClick={deauthorize} />
        </Show>
      </div>

      <div class="flex flex-col gap-8">
        <div class="flex flex-col gap-4">
          <div class="flex items-center justify-between">
            <p class="text-white font-semibold text-2xl">Balances</p>
            <a class="underline text-blue font-thin italic cursor-pointer text-sm" href="">
              How To Transfer Out
            </a>
          </div>
          <Show when={isAuthorized()} fallback={<p class="text-gray-120 text-sm">Sign In to see</p>}>
            <div class="flex flex-wrap gap-5">
              <For each={tokenIds()} fallback={<p class="text-gray-120 text-sm">Nothing here :(</p>}>
                {(id) => <BalanceOf tokenId={id} owner={principalId()!} />}
              </For>
            </div>
          </Show>
        </div>

        <div class="flex flex-col gap-4">
          <div class="flex flex-col gap-1">
            <p class="text-white font-semibold text-2xl">Referral Program</p>
            <p class="text-gray-140 font-normal text-sm">
              Join the MSQ Pay Referral Program and unlock <span class="font-semibold text-white">0.6%</span> earnings
              on every transaction made by the businesses you refer. Share your unique link, guide them through seamless
              integration, and start enjoying effortless, ongoing rewards with each sale.
            </p>
          </div>

          <Show when={isAuthorized()} fallback={<p class="text-gray-120 text-sm">Sign In to see</p>}>
            <div class="flex overflow-hidden rounded-3xl">
              <div class="flex bg-gray-110 text-ellipsis text-gray-140 p-4 truncate relative">
                <span>{referralLink()}</span>
                <div class="absolute h-full w-10 top-0 bottom-0 right-0 bg-gradient-to-l from-gray-110 via-gray-110" />
              </div>
              <div
                class="flex bg-gray-165 text-white p-4 text-nowrap font-semibold gap-2 cursor-pointer"
                onClick={handleLinkCopyClick}
              >
                <span>Copy Your Link</span>
                <Icon kind={EIconKind.Copy} color={COLORS.white} />
              </div>
            </div>
          </Show>
        </div>
      </div>
    </Page>
  );
};
