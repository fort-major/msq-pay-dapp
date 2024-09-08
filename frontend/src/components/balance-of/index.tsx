import { Principal } from "@dfinity/principal";
import { useAuth } from "@store/auth";
import { useTokens } from "@store/tokens";
import { EDs } from "@utils/math";
import { createEffect, on, onMount, Show } from "solid-js";

export interface IBalanceOfProps {
  tokenId: Principal;
  owner: Principal;
  subaccount?: Uint8Array;
  precision?: number;
}

export const BalanceOf = (props: IBalanceOfProps) => {
  const { balanceOf, fetchBalanceOf, supportedTokens, exchangeRates } = useTokens();
  const { isAuthorized } = useAuth();

  const meta = () => supportedTokens[props.tokenId.toText()];
  const balance = () => {
    const m = meta();
    if (!m) return undefined;

    const b = balanceOf(props.tokenId, props.owner, props.subaccount);
    if (b === undefined) return undefined;

    return EDs.new(b, m.fee.decimals);
  };
  const usd = () => {
    const b = balance();
    if (!b) return undefined;

    const m = meta()!;

    const rate = exchangeRates[m.ticker];

    if (!rate) return undefined;

    const balanceE8s = b.toDecimals(8).toE8s();

    return balanceE8s.mul(rate);
  };

  onMount(() => {
    if (!isAuthorized()) return;

    if (!balance()) {
      console.log("fetching balance");
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
    <div class="flex gap-2 items-center min-w-40">
      <Show when={meta()} fallback={<div class="w-6 h-6 rounded-full bg-gray-140 animate-pulse" />}>
        <img src={meta()!.logoSrc} alt={meta()?.ticker} class="w-6 h-6 rounded-full" />
      </Show>
      <div class="flex gap-1 items-baseline">
        <p class="font-semibold text-white text-lg">{balance() ? balance()!.toDecimals(2).toString() : "0.00"}</p>
        <p class="font-thin text-gray-140 text-sm">{meta()?.ticker ?? "TOK"}</p>
      </div>
      <Show when={usd()}>
        <p class="text-gray-140 font-semibold text-sm">≈ ${usd()!.toDynamic().toDecimals(2).toString()}</p>
      </Show>
    </div>
  );
};
