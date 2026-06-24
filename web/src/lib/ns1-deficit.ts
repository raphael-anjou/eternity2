// NS-1 deficit: the border↔interior colour-balance invariant (Hopfer 2022,
// vault concept `ns1-deficit`). Across the interface between the outer border
// ring and the first interior ring, the multiset of colours seen from the
// border side (A) must equal the multiset seen from the interior side (B) — for
// any full solution. The deficit
//
//     Δ = ½ ∑_c |A[c] − B[c]|
//
// counts how many interface edges are mismatched. It is a *necessary-but-loose*
// condition: it catches a missing or extra interface colour, but it is blind to
// a permutation that preserves the multiset (a border swap leaves Δ unchanged)
// and to interior-interior mismatches entirely. This file is the single source
// of the computation used by both the lab demo and any data the page cites.

import { BORDER } from "@/lib/types";
import type { Edges } from "@/lib/bucas";

export interface Tally {
  /** colour -> count, border side of the interface. */
  a: Map<number, number>;
  /** colour -> count, interior side of the interface. */
  b: Map<number, number>;
  /** ½ ∑ |A−B|. 0 for any valid full board. */
  deficit: number;
}

const inc = (m: Map<number, number>, c: number): void => {
  if (c !== BORDER) m.set(c, (m.get(c) ?? 0) + 1);
};

const isBorder = (x: number, y: number, n: number): boolean =>
  x === 0 || y === 0 || x === n - 1 || y === n - 1;

/**
 * Compute the NS-1 tally for an n×n board given as cell -> Edges|null (URDL,
 * row-major). Empty cells contribute nothing. A counts each border cell's edges
 * that face an interior neighbour; B counts each interior cell's edges that face
 * a border neighbour. On a full valid board A === B (Δ = 0).
 */
export function ns1Deficit(cells: (Edges | null)[], n: number): Tally {
  const a = new Map<number, number>();
  const b = new Map<number, number>();
  const at = (x: number, y: number): Edges | null => cells[y * n + x] ?? null;

  for (let y = 0; y < n; y++) {
    for (let x = 0; x < n; x++) {
      const e = at(x, y);
      if (!e) continue;
      // URDL faces and the neighbour each points at.
      const faces: [number, number, number][] = [
        [x, y - 1, e[0]],
        [x + 1, y, e[1]],
        [x, y + 1, e[2]],
        [x - 1, y, e[3]],
      ];
      const here = isBorder(x, y, n);
      for (const [nx, ny, c] of faces) {
        if (nx < 0 || ny < 0 || nx >= n || ny >= n) continue; // rim
        const there = isBorder(nx, ny, n);
        if (here && !there) inc(a, c); // border cell facing interior
        if (!here && there) inc(b, c); // interior cell facing border
      }
    }
  }

  const colours = new Set<number>([...a.keys(), ...b.keys()]);
  let d = 0;
  for (const c of colours) d += Math.abs((a.get(c) ?? 0) - (b.get(c) ?? 0));
  return { a, b, deficit: d / 2 };
}
