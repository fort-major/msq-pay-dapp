import { bigIntToBytes, bytesToHex, debugStringify, numberToBytes, strToBytes } from "./encoding";
import { ErrorCode, err } from "./error";
import { ONE_MIN_NS, ONE_SEC_NS } from "./types";
import { fromCBOR, hexToBytes, Principal, toCBOR } from "@fort-major/msq-shared";
import { Agent } from "@fort-major/agent-js-fork";
import { ShopId } from "@store/shops";

export function eventHandler<E extends Event>(fn: (e: E) => void | Promise<void>) {
  return (e: E) => {
    if (!e.isTrusted) {
      e.preventDefault();
      e.stopImmediatePropagation();
      e.stopPropagation();

      err(ErrorCode.SECURITY_VIOLATION, "No automation allowed!");
    }

    Promise.resolve(fn(e)).catch((e) => console.error(ErrorCode.UNKNOWN, debugStringify(e)));
  };
}

function bufsLE(a: Uint8Array, b: Uint8Array) {
  if (a.length != b.length) return false;
  for (let i = 0; i < a.length; i++) if (a[i] > b[i]) return false;
  return true;
}

export const SHOP_ID_SUBACCOUNT_DOMAIN = strToBytes("msq-shop-id-subaccount");

export const calcShopSubaccount = async (id: ShopId): Promise<Uint8Array> => {
  const buf = new Uint8Array([...SHOP_ID_SUBACCOUNT_DOMAIN, ...bigIntToBytes(id)]);

  return await crypto.subtle.digest("SHA-256", buf).then((it) => new Uint8Array(it));
};
