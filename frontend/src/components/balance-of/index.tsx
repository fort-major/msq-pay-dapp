import { Principal } from "@dfinity/principal";
import { useAuth } from "@store/auth";
import { useTokens } from "@store/tokens";
import { E8s } from "@utils/math";
import { createEffect, on, onMount, Show } from "solid-js";

export interface IBalanceOfProps {
  tokenId: Principal;
  owner: Principal;
  subaccount?: Uint8Array;
  precision?: number;
}

export const BalanceOf = (props: IBalanceOfProps) => {
  const { balanceOf, fetchBalanceOf, supportedTokens } = useTokens();
  const { isAuthorized } = useAuth();

  const meta = () => supportedTokens[props.tokenId.toText()];
  const balance = () => {
    const m = meta();
    if (!m) return undefined;

    const b = balanceOf(props.tokenId, props.owner, props.subaccount);
    if (b === undefined) return undefined;

    return E8s.new(b, m.decimals);
  };

  onMount(() => {
    if (!isAuthorized()) return;

    if (!balance()) {
      fetchBalanceOf(props.tokenId, props.owner, props.subaccount);
    }
  });

  createEffect(
    on(isAuthorized, (ready) => {
      if (!ready) return;

      if (!balance()) {
        fetchBalanceOf(props.tokenId, props.owner, props.subaccount);
      }
    })
  );

  return (
    <div class="flex gap-2 items-center min-w-36">
      <Show when={meta()} fallback={<div class="w-6 h-6 rounded-full bg-gray-140 animate-pulse" />}>
        <img src={meta()!.logo_src} alt={meta()?.ticker} class="w-6 h-6 rounded-full" />
      </Show>
      <div class="flex gap-1 items-baseline">
        <p class="font-semibold text-white text-lg">
          {balance() ? balance()!.toPrecision(props.precision ?? 2) : "0.00"}
        </p>
        <p class="font-thin text-gray-140 text-sm">{meta()?.ticker ?? "TOK"}</p>
      </div>
    </div>
  );
};
