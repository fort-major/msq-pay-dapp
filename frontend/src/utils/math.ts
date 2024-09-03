import { strToTokens, tokensToStr } from "./encoding";
import { ErrorCode, err } from "./error";
import { ONE_DAY_NS, ONE_HOUR_NS, ONE_MIN_NS } from "./types";

export class E8s {
  constructor(public val: bigint, public decimals: number) {
    if (val < 0n) {
      err(ErrorCode.UNREACHEABLE, "Unable to negate E8s");
    }
  }

  public static base(decimals?: number) {
    return 10n ** BigInt(decimals ?? 8);
  }

  private assertSameDecimals(b: E8s) {
    if (this.decimals !== b.decimals) {
      err(ErrorCode.UNREACHEABLE, "Invalid E8s operation: decimal point mismatch");
    }
  }

  public static new(val: bigint, decimals?: number) {
    return new E8s(val, decimals ? decimals : 8);
  }

  public eq(b: E8s): boolean {
    this.assertSameDecimals(b);

    return this.val === b.val;
  }

  public gt(b: E8s): boolean {
    this.assertSameDecimals(b);

    return this.val > b.val;
  }

  public ge(b: E8s): boolean {
    this.assertSameDecimals(b);

    return this.val >= b.val;
  }

  public lt(b: E8s): boolean {
    this.assertSameDecimals(b);

    return this.val < b.val;
  }

  public le(b: E8s): boolean {
    this.assertSameDecimals(b);

    return this.val <= b.val;
  }

  public static zero(decimals?: number): E8s {
    return E8s.new(0n, decimals);
  }

  public static one(decimals?: number): E8s {
    return E8s.new(E8s.base(decimals), decimals);
  }

  public static f0_05(decimals?: number): E8s {
    return E8s.new(E8s.base(decimals) / 20n, decimals);
  }

  public static f0_1(decimals?: number): E8s {
    return E8s.new(E8s.base(decimals) / 10n, decimals);
  }

  public static f0_2(decimals?: number): E8s {
    return E8s.new(E8s.base(decimals) / 5n, decimals);
  }

  public static f0_25(decimals?: number): E8s {
    return E8s.new(E8s.base(decimals) / 4n, decimals);
  }

  public static f0_3(decimals?: number): E8s {
    return E8s.new((E8s.base(decimals) * 3n) / 10n, decimals);
  }

  public static f0_33(decimals?: number): E8s {
    return E8s.new(E8s.base(decimals) / 3n, decimals);
  }

  public static f0_4(decimals?: number): E8s {
    return E8s.new((E8s.base(decimals) * 2n) / 5n, decimals);
  }

  public static f0_5(decimals?: number): E8s {
    return E8s.new(E8s.base(decimals) / 2n, decimals);
  }

  public static f0_6(decimals?: number): E8s {
    return E8s.new((E8s.base(decimals) * 3n) / 5n, decimals);
  }

  public static f0_67(decimals?: number): E8s {
    return E8s.new((E8s.base(decimals) * 2n) / 3n, decimals);
  }

  public static f0_7(decimals?: number): E8s {
    return E8s.new((E8s.base(decimals) * 7n) / 10n, decimals);
  }

  public static f0_75(decimals?: number): E8s {
    return E8s.new((E8s.base(decimals) * 3n) / 4n, decimals);
  }

  public static f0_8(decimals?: number): E8s {
    return E8s.new((E8s.base(decimals) * 4n) / 5n, decimals);
  }

  public static f0_9(decimals?: number): E8s {
    return E8s.new((E8s.base(decimals) * 9n) / 10n, decimals);
  }

  public add(b: E8s): E8s {
    this.assertSameDecimals(b);

    return E8s.new(this.val + b.val, this.decimals);
  }

  public sub(b: E8s): E8s {
    this.assertSameDecimals(b);

    return E8s.new(this.val - b.val, this.decimals);
  }

  public mul(b: E8s): E8s {
    this.assertSameDecimals(b);

    return E8s.new((this.val * b.val) / E8s.base(this.decimals), this.decimals);
  }

  public div(b: E8s): E8s {
    return E8s.new((this.val * E8s.base(this.decimals)) / b.val, this.decimals);
  }

  public toString() {
    return tokensToStr(this.val, this.decimals);
  }

  public static fromString(s: string): E8s {
    return E8s.new(strToTokens(s, 8));
  }

  public toPrecision(digits: number, allowEmptyTail: boolean = false) {
    return tokensToStr(this.val, 8, digits, false, allowEmptyTail);
  }

  public toBool() {
    return this.val > 0n;
  }

  public isZero() {
    return this.val === 0n;
  }

  public toBigIntRaw() {
    return this.val;
  }

  public static fromBigIntBase(x: bigint, decimals?: number) {
    return E8s.new(x * E8s.base(decimals), decimals);
  }

  public toBigIntBase() {
    return this.val / E8s.base(this.decimals);
  }

  public static fromPercentNum(p: number, decimals?: number) {
    return E8s.new((BigInt(Math.floor(p)) * E8s.base(decimals)) / 100n, decimals);
  }

  public toPercentNum() {
    return Number((this.val * 100n) / E8s.base(this.decimals));
  }

  public toPercent() {
    return E8s.new(this.val * 100n);
  }
}
