import { describe, it, expect } from "vitest";
import {
  formatInt,
  formatCompact,
  formatSeconds,
  superscript,
  formatScientific,
} from "./format";

describe("formatInt", () => {
  it("groups thousands (en-US) and rounds", () => {
    expect(formatInt(1000)).toBe("1,000");
    expect(formatInt(1234567)).toBe("1,234,567");
    expect(formatInt(12.6)).toBe("13");
    expect(formatInt(0)).toBe("0");
  });
});

describe("formatCompact", () => {
  it("stays plain below 10k", () => {
    expect(formatCompact(9999)).toBe("9,999");
  });
  it("uses compact notation at/above 10k", () => {
    expect(formatCompact(12_300)).toBe("12.3K");
    expect(formatCompact(1_200_000_000)).toBe("1.2B");
  });
});

describe("formatSeconds", () => {
  it("renders m:ss with zero-padded seconds", () => {
    expect(formatSeconds(0)).toBe("0:00");
    expect(formatSeconds(9)).toBe("0:09");
    expect(formatSeconds(60)).toBe("1:00");
    expect(formatSeconds(605)).toBe("10:05");
  });
  it("floors fractional seconds", () => {
    expect(formatSeconds(59.9)).toBe("0:59");
  });
});

describe("superscript", () => {
  it("maps digits and the minus sign to unicode superscripts", () => {
    expect(superscript(45)).toBe("⁴⁵");
    expect(superscript(0)).toBe("⁰");
    expect(superscript(-3)).toBe("⁻³");
  });
});

describe("formatScientific", () => {
  it("falls back to a grouped integer below the plain threshold", () => {
    expect(formatScientific(999)).toBe("999");
    expect(formatScientific(0)).toBe("0");
  });
  it("renders a mantissa × power of ten above the threshold", () => {
    expect(formatScientific(3.4e37)).toBe("3.4×10³⁷");
    expect(formatScientific(1000)).toBe("1.0×10³");
  });
  it("passes non-finite values through as-is", () => {
    expect(formatScientific(Infinity)).toBe("Infinity");
    expect(formatScientific(NaN)).toBe("NaN");
  });
  it("respects a custom plainBelow threshold", () => {
    expect(formatScientific(5000, 1e6)).toBe("5,000");
  });
});
