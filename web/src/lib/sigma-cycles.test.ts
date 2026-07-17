import { describe, it, expect } from "vitest";
import { sigmaPermutation, decomposeCycles } from "./sigma-cycles";

// Board encoding: each cell holds pieceId*4 + rotation. σ carries A's arrangement
// to B: sigma[i] = the cell in B holding the piece that sits at cell i in A.
// Rotation is dropped (pieceId = value >> 2), so these tests vary rotation freely
// to confirm only piece identity drives σ.

const enc = (pieceId: number, rot = 0) => pieceId * 4 + rot;

describe("sigmaPermutation", () => {
  it("is the identity when both boards place the same pieces in the same cells", () => {
    const a = Int32Array.from([enc(10), enc(11), enc(12)]);
    const b = Int32Array.from([enc(10, 1), enc(11, 2), enc(12, 3)]); // same pieces, different rot
    expect(sigmaPermutation(a, b)).toEqual([0, 1, 2]);
  });

  it("maps each cell to where its piece moved in B", () => {
    // A: cells [p0, p1, p2].  B: same pieces at [p2, p0, p1] (a rotation of positions).
    const a = Int32Array.from([enc(0), enc(1), enc(2)]);
    const b = Int32Array.from([enc(2), enc(0), enc(1)]);
    // piece p0 sits at cell 0 in A, and at cell 1 in B → sigma[0] = 1
    // piece p1 at cell 1 in A, at cell 2 in B → sigma[1] = 2
    // piece p2 at cell 2 in A, at cell 0 in B → sigma[2] = 0
    expect(sigmaPermutation(a, b)).toEqual([1, 2, 0]);
  });

  it("ignores rotation entirely (only piece identity matters)", () => {
    const a = Int32Array.from([enc(5, 0), enc(6, 0)]);
    const b = Int32Array.from([enc(6, 3), enc(5, 1)]);
    expect(sigmaPermutation(a, b)).toEqual([1, 0]);
  });
});

describe("decomposeCycles", () => {
  it("drops fixed points and returns nothing for the identity", () => {
    expect(decomposeCycles([0, 1, 2, 3])).toEqual([]);
  });

  it("returns a single 2-cycle for a transposition", () => {
    // 0↔1, 2 fixed
    expect(decomposeCycles([1, 0, 2])).toEqual([[0, 1]]);
  });

  it("returns the full cycle for a 3-rotation", () => {
    // 0→1→2→0
    expect(decomposeCycles([1, 2, 0])).toEqual([[0, 1, 2]]);
  });

  it("splits disjoint cycles and sorts them longest-first", () => {
    // 0→1→2→0 (len 3) and 3↔4 (len 2), 5 fixed
    const cycles = decomposeCycles([1, 2, 0, 4, 3, 5]);
    expect(cycles).toEqual([
      [0, 1, 2],
      [3, 4],
    ]);
  });

  it("skips negative (unmapped) entries without looping forever", () => {
    // -1 means the piece had no home in B; must be treated as a fixed/skip, not traversed.
    expect(decomposeCycles([1, 0, -1])).toEqual([[0, 1]]);
  });
});
