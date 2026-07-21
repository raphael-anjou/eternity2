// Built-in cell-visit orders ("paths") (ALGORITHM.md §6).
//
// A path is the order in which the DFS fills cells: a permutation of
// 0..w*h (row-major cell indices). Path order is a first-class search
// hyperparameter — on the same puzzle, different paths can differ by orders of
// magnitude in nodes-to-solve. The solver checks all four neighbours when
// testing a fit, so even a non-contiguous path (e.g. `random`) is correct;
// order only affects how fast.
//
// Canonical source: engine/src/paths.rs. Each branch mirrors the Rust line for
// line, including loop bounds and push order — these determine the permutation.

import { XorShift } from "./rng.ts";

/** The 9 path names, in the engine's canonical order (ALGORITHM.md §6). */
export const PATH_KINDS: readonly string[] = [
  "row-major",
  "snake",
  "column-major",
  "spiral-in",
  "spiral-out",
  "diagonal",
  "border-first",
  "double-snake",
  "random",
];

/**
 * Build the cell-visit permutation for `kind` on a width×height board.
 * Returns `null` for an unknown kind (Rust returns `None`). Only `random`
 * touches the RNG, via `seed`.
 *
 * Mirrors `build_path` in engine/src/paths.rs.
 */
export function buildPath(
  kind: string,
  width: number,
  height: number,
  seed: number,
): Uint16Array | null {
  const w = width;
  const h = height;
  const idx = (x: number, y: number): number => y * w + x;
  const out: number[] = [];

  switch (kind) {
    case "row-major": {
      for (let y = 0; y < h; y++) {
        for (let x = 0; x < w; x++) {
          out.push(idx(x, y));
        }
      }
      break;
    }
    case "snake": {
      for (let y = 0; y < h; y++) {
        if (y % 2 === 0) {
          for (let x = 0; x < w; x++) {
            out.push(idx(x, y));
          }
        } else {
          for (let x = w - 1; x >= 0; x--) {
            out.push(idx(x, y));
          }
        }
      }
      break;
    }
    case "column-major": {
      for (let x = 0; x < w; x++) {
        for (let y = 0; y < h; y++) {
          out.push(idx(x, y));
        }
      }
      break;
    }
    case "spiral-in":
    case "spiral-out": {
      let x0 = 0;
      let y0 = 0;
      let x1 = w;
      let y1 = h;
      while (x0 < x1 && y0 < y1) {
        for (let x = x0; x < x1; x++) {
          out.push(idx(x, y0));
        }
        for (let y = y0 + 1; y < y1; y++) {
          out.push(idx(x1 - 1, y));
        }
        if (y1 > y0 + 1) {
          for (let x = x1 - 2; x >= x0; x--) {
            out.push(idx(x, y1 - 1));
          }
        }
        if (x1 > x0 + 1) {
          for (let y = y1 - 2; y >= y0 + 1; y--) {
            out.push(idx(x0, y));
          }
        }
        x0 += 1;
        y0 += 1;
        x1 -= 1;
        y1 -= 1;
      }
      if (kind === "spiral-out") {
        out.reverse();
      }
      break;
    }
    case "diagonal": {
      for (let d = 0; d < w + h - 1; d++) {
        for (let y = 0; y < h; y++) {
          if (d >= y && d - y < w) {
            out.push(idx(d - y, y));
          }
        }
      }
      break;
    }
    case "border-first": {
      // Rim clockwise from (0,0), then interior row-major.
      for (let x = 0; x < w; x++) {
        out.push(idx(x, 0));
      }
      for (let y = 1; y < h; y++) {
        out.push(idx(w - 1, y));
      }
      if (h > 1) {
        for (let x = w - 2; x >= 0; x--) {
          out.push(idx(x, h - 1));
        }
      }
      if (w > 1) {
        for (let y = h - 2; y >= 1; y--) {
          out.push(idx(0, y));
        }
      }
      // Interior row-major. Rust uses `1..h.saturating_sub(1)` /
      // `1..w.saturating_sub(1)`: empty when h<2 or w<2.
      const yEnd = h >= 1 ? h - 1 : 0;
      const xEnd = w >= 1 ? w - 1 : 0;
      for (let y = 1; y < yEnd; y++) {
        for (let x = 1; x < xEnd; x++) {
          out.push(idx(x, y));
        }
      }
      break;
    }
    case "double-snake": {
      // Two rows at a time, zig-zagging keeps a compact frontier.
      let y = 0;
      while (y < h) {
        const rows = y + 1 < h ? [y, y + 1] : [y];
        if (Math.floor(y / 2) % 2 === 0) {
          for (let x = 0; x < w; x++) {
            for (const r of rows) {
              out.push(idx(x, r));
            }
          }
        } else {
          for (let x = w - 1; x >= 0; x--) {
            for (const r of rows) {
              out.push(idx(x, r));
            }
          }
        }
        y += 2;
      }
      break;
    }
    case "random": {
      for (let c = 0; c < w * h; c++) {
        out.push(c);
      }
      new XorShift(seed).shuffle(out);
      break;
    }
    default:
      return null;
  }

  return Uint16Array.from(out);
}
