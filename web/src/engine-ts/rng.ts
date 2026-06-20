// The deterministic random number generator (ALGORITHM.md §3).
//
// Puzzle generation must be reproducible: the same (size, colors, seed) must
// produce the exact same puzzle in every language. We therefore carry our own
// tiny generator rather than the host's built-in RNG. This is the single most
// fragile part of any port — every downstream choice (which colours, which
// piece order, which rotations) flows from these bits, so the 64-bit math must
// be reproduced *exactly*.
//
// JavaScript `number` cannot represent 64-bit integers, so we do every step in
// `BigInt` masked to 64 bits with `& MASK64`. This mirrors Rust's
// `u64::wrapping_*` operations (overflow wraps; that is intentional).
//
// Canonical source: engine/src/generator.rs (`struct XorShift`).

/** 64-bit mask: keeps a BigInt in the unsigned 0..2^64 range, emulating u64. */
const MASK64 = 0xffffffffffffffffn;

/** splitmix64 mixing constants (ALGORITHM.md §3.1). */
const SPLITMIX_ADD = 0x9e3779b97f4a7c15n;
const SPLITMIX_MUL1 = 0xbf58476d1ce4e5b9n;
const SPLITMIX_MUL2 = 0x94d049bb133111ebn;

/**
 * XorShift64 generator, seeded by one round of splitmix64.
 *
 * Mirrors `XorShift` in engine/src/generator.rs. `next_u32` returns the high
 * 32 bits of the 64-bit state after one xorshift step.
 */
export class XorShift {
  /** The 64-bit state, held as a BigInt masked to 64 bits. */
  private state: bigint;

  /**
   * Seed with one round of splitmix64 (ALGORITHM.md §3.1). A raw 32-bit seed
   * makes a poor 64-bit state (lots of zero bits), so we mix it first and then
   * force the low bit on (xorshift dies on an all-zero state).
   */
  constructor(seed: number) {
    // `seed` is a u32; coerce to a non-negative BigInt before mixing.
    let z = (BigInt(seed >>> 0) + SPLITMIX_ADD) & MASK64;
    z = ((z ^ (z >> 30n)) * SPLITMIX_MUL1) & MASK64;
    z = ((z ^ (z >> 27n)) * SPLITMIX_MUL2) & MASK64;
    z = (z ^ (z >> 31n)) & MASK64;
    // `... | 1` matches Rust's `z ^ (z >> 31) | 1` (| binds looser than ^).
    this.state = (z | 1n) & MASK64;
  }

  /**
   * One xorshift64 step; returns the high 32 bits as an unsigned number in
   * 0..2^32 (ALGORITHM.md §3.2).
   */
  nextU32(): number {
    let x = this.state;
    x = (x ^ ((x << 13n) & MASK64)) & MASK64;
    x = (x ^ (x >> 7n)) & MASK64;
    x = (x ^ ((x << 17n) & MASK64)) & MASK64;
    this.state = x;
    // High half: a u32, safely representable as a JS number.
    return Number((x >> 32n) & 0xffffffffn);
  }

  /** Uniform integer in [0, n) via `next_u32() % n` (ALGORITHM.md §3.3). */
  below(n: number): number {
    return this.nextU32() % n;
  }

  /**
   * In-place Fisher–Yates shuffle, iterating from the last index down to 1 and
   * swapping `i` with `below(i + 1)` (ALGORITHM.md §3.3). The descending
   * direction and the exact `below(i + 1)` call order are load-bearing: change
   * either and the permutation differs, breaking golden parity.
   */
  shuffle<T>(slice: T[]): void {
    for (let i = slice.length - 1; i >= 1; i--) {
      const j = this.below(i + 1);
      const tmp = slice[i] as T;
      slice[i] = slice[j] as T;
      slice[j] = tmp;
    }
  }
}
