//! Entropy density and the area law of Eternity II's interior grammar.
//!
//! Two halves, both computed here from the official piece set:
//!
//! 1. **Grammar entropy.** Treat the 196 interior pieces as reusable tiles
//!    (forget the one-of-each rule) and ask how many ways an n-wide strip of
//!    matched tiles can grow downwards. The growth rate per cell is an entropy
//!    density h(n). Computed exactly for widths 1 and 2 via the row-transfer
//!    matrix's dominant eigenvalue, plus the 1-D horizontal bound lambda_H. The
//!    sequence h(1), h(2), ... decreases to the true 2-D density h_infinity
//!    (Fekete's lemma), so each h(n) bounds it from above. Widths 3 to 6 are
//!    carried, with their source named in the output, from a stdlib script
//!    that agrees with this crate at widths 1 and 2. No value of h_infinity
//!    is stated, only the upper bound the smallest computed h(n) gives.
//!
//! 2. **The area law.** For an interior n×n block with a free outer border count,
//!    exactly, two things: A(n) = the number of colour-valid fillings when pieces
//!    may repeat (the matching grammar), and B(n) = the number that use distinct
//!    pieces (true Eternity II). The realizable fraction is rho(n) = B(n)/A(n),
//!    reported with the per-n -ln(rho(n))/n^2. No exponent is fitted and nothing
//!    is extrapolated past n = 3: an earlier version fitted an area law over
//!    n <= 3 and extrapolated a collapse point, both withdrawn on 2026-11-13
//!    (sampled fractions of larger patches fall far faster than the fit; the
//!    output keeps the withdrawn values under "withdrawn" with the reason).
//!    A(n) is a dense broken-profile transfer DP; B(n) is
//!    a parallel distinct-counting DFS whose cost is bounded by A(n). Both are
//!    exact for n = 1, 2, 3; n = 4 is over the DFS budget and reported as such.
//!
//! Exact and deterministic. Cross-checked at n=2 against the in-repo
//! subgrid-placement-counts reference table (B(2) = 4 059 952).

use eternity2_engine::official::official_puzzle;
use eternity2_engine::types::{rotated, Color};

const NCOL: usize = 23; // colors 0..=22 (0 = grey border, unused on interior pieces)

// --- Values carried into the output, not computed here ---------------------
// Each block names its source. They are emitted verbatim so that this crate's
// output is the committed results file.

/// h(3) to h(6), from research/topics/entropy-area-law/compute/scripts/
/// strip_entropy.js (broken-profile power iteration, stdlib Node).
const H_CARRIED: &str = r#"    { "width": 3, "lambda": 342.502, "h": 0.8449, "source": "strip_entropy.js" },
    { "width": 4, "lambda": 932.949, "h": 0.7425, "source": "strip_entropy.js" },
    { "width": 5, "lambda": 2541.305, "h": 0.6810, "source": "strip_entropy.js" },
    { "width": 6, "lambda": 6922.392, "h": 0.6400, "source": "strip_entropy.js" }
"#;

/// log10(lambda_n / lambda_(n-1)) for widths 3 to 6, same source.
const INCREMENTS_CARRIED: &str = r#"    { "width": 3, "increment": 0.4352 },
    { "width": 4, "increment": 0.4352 },
    { "width": 5, "increment": 0.4352 },
    { "width": 6, "increment": 0.4352 }
"#;

const H_INFINITY_CARRIED: &str = r#"  "hInfinityUpperBound": 0.6400,
  "hInfinityNote": "No value of h_infinity is computed. Because h_infinity is the infimum of h(n), the smallest computed h(n), at width 6, bounds it from above. Each added row multiplies lambda by about 10^0.435 (hIncrementByWidth), so h(n) is close to 0.435 + 1.23/n over widths 1 to 6; the research page reads the limit as close to 0.435 from that, a reading of six widths and not a computed limit.",
"#;

/// Sampled distinct-piece fractions (two seeded runs), from
/// research/topics/entropy-area-law/compute/scripts/distinct_fraction.js, the interpolated
/// 1e-3 crossing from entropy_fields.py in the same folder, and the values an
/// earlier version of this crate reported, kept with the reason they were
/// withdrawn.
const SAMPLED_AND_WITHDRAWN_CARRIED: &str = r#"  "sampledDistinctFractionNote": "Share of colour-valid patches (pieces reusable, every internal joint matched, outer border free) that use distinct pieces, estimated by drawing patches uniformly at random. Not exact: seeded samples. Two independent runs, seeds 1 to 6 (sampledDistinctFraction) and seeds 101 to 106 (sampledDistinctFractionSecondRun), agree within their combined standard error at every size (largest gap 1.8 combined standard errors, at 32 cells). The 2x2 and 3x3 rows reproduce the exact blockCounts ratios within sampling error. Source: research/topics/entropy-area-law/compute/scripts/distinct_fraction.js (stdlib Node, single thread).",
  "sampledDistinctFraction": [
    { "rows": 2, "cols": 2, "cells": 4, "rho": 0.8921, "standardError": 0.00031, "samples": 1000000 },
    { "rows": 3, "cols": 3, "cells": 9, "rho": 0.6473, "standardError": 0.00048, "samples": 1000000 },
    { "rows": 4, "cols": 4, "cells": 16, "rho": 0.3263, "standardError": 0.00074, "samples": 400000 },
    { "rows": 4, "cols": 6, "cells": 24, "rho": 0.1060, "standardError": 0.00049, "samples": 400000 },
    { "rows": 4, "cols": 8, "cells": 32, "rho": 0.02383, "standardError": 0.00024, "samples": 400000 },
    { "rows": 4, "cols": 12, "cells": 48, "rho": 0.000375, "standardError": 0.000019, "samples": 1000000 }
  ],
  "sampledDistinctFractionSecondRun": [
    { "rows": 2, "cols": 2, "cells": 4, "rho": 0.8918, "standardError": 0.00031, "samples": 1000000 },
    { "rows": 3, "cols": 3, "cells": 9, "rho": 0.6470, "standardError": 0.00048, "samples": 1000000 },
    { "rows": 4, "cols": 4, "cells": 16, "rho": 0.3261, "standardError": 0.00074, "samples": 400000 },
    { "rows": 4, "cols": 6, "cells": 24, "rho": 0.1071, "standardError": 0.00049, "samples": 400000 },
    { "rows": 4, "cols": 8, "cells": 32, "rho": 0.02443, "standardError": 0.00024, "samples": 400000 },
    { "rows": 4, "cols": 12, "cells": 48, "rho": 0.000389, "standardError": 0.000020, "samples": 1000000 }
  ],
  "distinctFractionBelowOnePerThousandCells": 44,
  "distinctFractionBelowOnePerThousandNote": "Log-linear interpolation between the sampled 32-cell and 48-cell fractions crosses 1e-3 at 44.2 cells with seeds 1 to 6 and at 44.4 cells with seeds 101 to 106 (research/topics/entropy-area-law/compute/scripts/entropy_fields.py). Inside the sampled range, not an extrapolation; patches four cells wide only.",
  "withdrawn": {
    "date": "2026-11-13",
    "hInfinity": {
      "value": 0.67,
      "wasNote": "h_infinity from the offline width-n sweep.",
      "reason": "Not computed by this crate: a constant carried from an off-repo sweep. h_infinity is the infimum of h(n), and h(6) = 0.6400 is already below 0.67, so the value is impossible. Replaced by hInfinityUpperBound."
    },
    "areaLawExponent": {
      "value": 0.0445,
      "wasNote": "alpha fitted by least squares through the origin (ln rho = -alpha n^2) over the exact in-repo B(n)/A(n) for n=1..3. The per-n alpha(n) = -ln(rho(n))/n^2 rises with n over this small exact range (see alphaByN), so this fit is a conservative estimate of the large-patch exponent; the rho(n) curve beyond n=3 is an extrapolation of the fit.",
      "reason": "A three-point fit extrapolated far past its range. The sampled fraction at 48 cells is 0.000375, about 315 times below the fit's exp(-0.0445 x 48) = 0.118, and -ln(rho) per cell keeps rising (0.070 at 16 cells, 0.164 at 48), so no single exponent describes the decay. Replaced by sampledDistinctFraction."
    },
    "distinctnessCollapseCells": {
      "value": 155,
      "wasNote": "Extrapolated cell count where rho drops below 1e-3, from n* = sqrt(ln(1000)/alpha); beyond the exactly-computed range.",
      "reason": "Derived from the withdrawn exponent (ln(1000)/0.0445 = 155.2). The sampled fraction crosses 1e-3 near 44 cells. Replaced by distinctFractionBelowOnePerThousandCells."
    }
  }
"#;

// --- Part 1: grammar entropy (row-transfer eigenvalues) --------------------

/// Dominant eigenvalue of a dense nonnegative matrix by power iteration.
fn spectral_radius(m: &[f64], dim: usize) -> f64 {
    let mut v = vec![1.0; dim];
    let mut lambda = 0.0;
    for _ in 0..4000 {
        let mut nv = vec![0.0; dim];
        for i in 0..dim {
            let row = i * dim;
            let mut acc = 0.0;
            for j in 0..dim {
                acc += m[row + j] * v[j];
            }
            nv[i] = acc;
        }
        let norm: f64 = nv.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm == 0.0 {
            return 0.0;
        }
        for x in nv.iter_mut() {
            *x /= norm;
        }
        // Rayleigh quotient v^T M v.
        let mut num = 0.0;
        for i in 0..dim {
            let row = i * dim;
            let mut t = 0.0;
            for j in 0..dim {
                t += m[row + j] * nv[j];
            }
            num += nv[i] * t;
        }
        lambda = num;
        v = nv;
    }
    lambda
}

// --- Part 2: interior area-law block counts --------------------------------

/// Interior oriented placements, as `(up, right, down, left)` colours.
///
/// The 196 interior pieces (no grey edge), each in its 4 rotations = 784 oriented
/// placements. This is the reusable-tile pool of the matching grammar and the
/// distinct-piece pool of true E2 alike (an interior block, having no board rim,
/// only ever holds interior pieces).
struct Pool {
    /// per placement: (piece_id, [up, right, down, left])
    placements: Vec<(u16, [Color; 4])>,
}

fn interior_pool() -> Pool {
    let puzzle = official_puzzle();
    let mut placements = Vec::new();
    for (pid, &e) in puzzle.pieces.iter().enumerate() {
        if e.contains(&0) {
            continue; // corner/edge piece, not interior
        }
        for r in 0..4u8 {
            placements.push((pid as u16, rotated(e, r)));
        }
    }
    Pool { placements }
}

/// A(n): number of colour-valid fillings of an interior n×n block with a FREE
/// outer border, pieces reusable. Exact, via a dense broken-profile transfer DP.
///
/// State between cells is the broken profile: the `north[c]` colour on the bottom
/// edge of the cell placed above column c (for the current partial row), and the
/// `west` colour on the right edge of the cell just placed to the left. The border
/// is free, so the top row and left column impose no colour on the outer edges;
/// only shared internal edges must match. The profile alphabet is the interior
/// colours (1..=22) plus a "free" sentinel, so the state space is bounded and the
/// map stays small; counts are u128.
fn count_reusable(pool: &Pool, n: usize) -> u128 {
    // Index placements by (up_colour, left_colour) so each cell transition reads a
    // precomputed candidate list. For a cell with an internal up-neighbour we need
    // up == north[c]; with a free (top-row) up-edge the up-colour is unconstrained.
    // Same for left. We bucket by (up,left) with a "free" slot 0 meaning "no
    // constraint on that side"; a top-row / left-column cell reads slot 0.
    //
    // by_in[up*NCOL + left] -> Vec<(down, right)>: candidates whose up==`up` and
    // left==`left`. For unconstrained sides we must union over all colours, so we
    // additionally keep by_up_only, by_left_only, and all_cands.
    let placements = &pool.placements;

    // Group down/right outputs by their (up,left) input colours.
    let mut by_both: Vec<Vec<(Color, Color)>> = vec![Vec::new(); NCOL * NCOL];
    let mut by_up: Vec<Vec<(Color, Color)>> = vec![Vec::new(); NCOL]; // left free, keyed by up
    let mut by_left: Vec<Vec<(Color, Color)>> = vec![Vec::new(); NCOL]; // up free, keyed by left
    let mut all: Vec<(Color, Color)> = Vec::new(); // both free
    for &(_, e) in placements {
        let (u, r, d, l) = (e[0], e[1], e[2], e[3]);
        by_both[u as usize * NCOL + l as usize].push((d, r));
        by_up[u as usize].push((d, r));
        by_left[l as usize].push((d, r));
        all.push((d, r));
    }

    // Broken-profile DP. Encode the state as the north profile (n colours, sentinel
    // 0 = "free/top") packed base-NCOL, times NCOL for the west colour (0 = free).
    use std::collections::HashMap;
    let mut cur: HashMap<u64, u128> = HashMap::new();
    cur.insert(0, 1); // all-free north profile, free west
    let mut north = vec![0u8; n];
    for i in 0..n * n {
        let (r, c) = (i / n, i % n);
        let top_row = r == 0;
        let left_col = c == 0;
        let mut next: HashMap<u64, u128> = HashMap::with_capacity(cur.len() * 2);
        for (&key, &cnt) in &cur {
            // Decode north[..] and west from key.
            let mut k = key;
            let west = (k % NCOL as u64) as u8;
            k /= NCOL as u64;
            for j in (0..n).rev() {
                north[j] = (k % NCOL as u64) as u8;
                k /= NCOL as u64;
            }
            let up_c = if top_row { 0 } else { north[c] };
            let left_c = if left_col { 0 } else { west };
            let cands: &[(Color, Color)] = match (top_row, left_col) {
                (true, true) => &all,
                (true, false) => &by_left[left_c as usize],
                (false, true) => &by_up[up_c as usize],
                (false, false) => &by_both[up_c as usize * NCOL + left_c as usize],
            };
            for &(down, right) in cands {
                // New north[c] = down; new west = right (reset to free at row end,
                // handled naturally: the next cell in a new row is left_col so reads
                // free regardless of the stored west).
                let old = north[c];
                north[c] = down;
                let mut nk = 0u64;
                for &x in north.iter() {
                    nk = nk * NCOL as u64 + x as u64;
                }
                nk = nk * NCOL as u64 + right as u64;
                north[c] = old;
                *next.entry(nk).or_insert(0) += cnt;
            }
        }
        cur = next;
    }
    cur.values().sum()
}

/// B(n): number of colour-valid fillings of an interior n×n block with a free
/// outer border that use DISTINCT pieces. Exact, via a parallel distinct-counting
/// DFS whose running cost is bounded by A(n). Returns `None` if A(n) exceeds the
/// budget (the block would take too long to enumerate exactly).
fn count_distinct(pool: &Pool, n: usize, reusable: u128) -> Option<u128> {
    const DFS_BUDGET: u128 = 500_000_000_000; // 5e11 nodes; ~a few minutes on 8 cores
    if reusable > DFS_BUDGET {
        return None;
    }
    let cells = n * n;
    if cells == 0 {
        return Some(1);
    }

    // Flat candidate index by (up,left) colour, sentinel 31 = free side.
    // slot = up*32 + left; entries are (piece_id, down, right).
    let placements = &pool.placements;
    let mut by_in: Vec<Vec<(u16, u8, u8)>> = vec![Vec::new(); 32 * 32];
    for &(pid, e) in placements {
        let (u, r, d, l) = (e[0], e[1], e[2], e[3]);
        // A placement can be used at a cell whose up/left are either exactly its
        // (u,l) or free. We index it under the concrete (u,l) slot and also under
        // the free slots so the DFS reads one contiguous list per (up,left) query.
        by_in[u as usize * 32 + l as usize].push((pid, d, r)); // both constrained
        by_in[31 * 32 + l as usize].push((pid, d, r)); // up free (top row)
        by_in[u as usize * 32 + 31].push((pid, d, r)); // left free (left col)
        by_in[31 * 32 + 31].push((pid, d, r)); // both free (top-left)
    }
    // Flatten into contiguous storage.
    let mut flat: Vec<(u16, u8, u8)> = Vec::new();
    let mut off = vec![0u32; 32 * 32 + 1];
    for slot in 0..32 * 32 {
        off[slot] = flat.len() as u32;
        flat.extend_from_slice(&by_in[slot]);
    }
    off[32 * 32] = flat.len() as u32;

    #[inline]
    fn slot_of(i: usize, n: usize, north: &[u8], west: u8) -> usize {
        let (r, c) = (i / n, i % n);
        let up = if r == 0 { 31 } else { north[c] } as usize;
        let left = if c == 0 { 31 } else { west } as usize;
        up * 32 + left
    }

    #[allow(clippy::too_many_arguments)]
    fn go(
        flat: &[(u16, u8, u8)],
        off: &[u32],
        n: usize,
        cells: usize,
        i: usize,
        north: &mut [u8],
        west: u8,
        used: &mut [bool; 256],
        count: &mut u128,
    ) {
        if i == cells {
            *count += 1;
            return;
        }
        let c = i % n;
        let slot = slot_of(i, n, north, west);
        let (lo, hi) = (off[slot] as usize, off[slot + 1] as usize);
        for &(pid, down, right) in &flat[lo..hi] {
            let u = &mut used[pid as usize];
            if *u {
                continue;
            }
            *u = true;
            let old = north[c];
            north[c] = down;
            go(flat, off, n, cells, i + 1, north, right, used, count);
            north[c] = old;
            used[pid as usize] = false;
        }
    }

    // Parallel fan-out over a prefix of cells (first row + up to 2 cells), each
    // seed carrying its full continuation state.
    type Seed = (Vec<u8>, u8, Vec<u16>); // (north profile, west, used pids)
    let seed_n = (n + 2).min(cells);
    let mut seeds: Vec<Seed> = Vec::new();
    {
        #[allow(clippy::too_many_arguments)]
        fn prefix(
            flat: &[(u16, u8, u8)],
            off: &[u32],
            n: usize,
            seed_n: usize,
            i: usize,
            north: &mut [u8],
            west: u8,
            used: &mut [bool; 256],
            used_ids: &mut Vec<u16>,
            seeds: &mut Vec<Seed>,
        ) {
            if i == seed_n {
                seeds.push((north.to_vec(), west, used_ids.clone()));
                return;
            }
            let c = i % n;
            let slot = slot_of(i, n, north, west);
            let (lo, hi) = (off[slot] as usize, off[slot + 1] as usize);
            for &(pid, down, right) in &flat[lo..hi] {
                if used[pid as usize] {
                    continue;
                }
                used[pid as usize] = true;
                used_ids.push(pid);
                let old = north[c];
                north[c] = down;
                prefix(flat, off, n, seed_n, i + 1, north, right, used, used_ids, seeds);
                north[c] = old;
                used_ids.pop();
                used[pid as usize] = false;
            }
        }
        let mut north = vec![0u8; n];
        let mut used = [false; 256];
        let mut used_ids: Vec<u16> = Vec::new();
        prefix(&flat, &off, n, seed_n, 0, &mut north, 31, &mut used, &mut used_ids, &mut seeds);
    }

    if seed_n == cells {
        return Some(seeds.len() as u128);
    }

    let nthreads = std::thread::available_parallelism()
        .map(|n| n.get().min(8))
        .unwrap_or(4);
    let flat = &flat;
    let off = &off;
    let seeds = std::sync::Arc::new(seeds);
    let total: u128 = std::thread::scope(|scope| {
        let next = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut handles = Vec::new();
        for _ in 0..nthreads {
            let next = next.clone();
            let seeds = seeds.clone();
            handles.push(scope.spawn(move || {
                let mut local: u128 = 0;
                let mut used = [false; 256];
                let mut north = vec![0u8; n];
                loop {
                    let k = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if k >= seeds.len() {
                        break;
                    }
                    let (seed_north, seed_west, seed_used) = &seeds[k];
                    north.copy_from_slice(seed_north);
                    for &pid in seed_used {
                        used[pid as usize] = true;
                    }
                    go(flat, off, n, cells, seed_n, &mut north, *seed_west, &mut used, &mut local);
                    for &pid in seed_used {
                        used[pid as usize] = false;
                    }
                }
                local
            }));
        }
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    });
    Some(total)
}

fn main() {
    let puzzle = official_puzzle();

    // --- Part 1: grammar entropy ---
    let interior: Vec<[u8; 4]> = puzzle
        .pieces
        .iter()
        .filter(|e| !e.contains(&0))
        .copied()
        .collect();
    let mut pl: Vec<(u8, u8, u8, u8)> = Vec::new();
    for &e in &interior {
        for r in 0..4u8 {
            let x = rotated(e, r);
            pl.push((x[0], x[1], x[2], x[3]));
        }
    }
    let placements = pl.len();

    // 1-D horizontal compatibility matrix A[w][e]; lambda_H bounds every h(n).
    let mut a = vec![0.0; NCOL * NCOL];
    for &(_, e, _, w) in &pl {
        a[(w as usize) * NCOL + e as usize] += 1.0;
    }
    let lambda_h = spectral_radius(&a, NCOL);

    // Width-1 transfer T1[s][n]: vertical step over a single placement.
    let mut t1 = vec![0.0; NCOL * NCOL];
    for &(n, _, s, _) in &pl {
        t1[(s as usize) * NCOL + n as usize] += 1.0;
    }
    let lambda1 = spectral_radius(&t1, NCOL);
    let h1 = lambda1.log10();

    // Width-2 transfer over profile pairs (c0,c1) indexed c0*23+c1.
    let np = NCOL * NCOL;
    let mut t2 = vec![0.0; np * np];
    for &(n0, e0, s0, _) in &pl {
        for &(n1, _, s1, w1) in &pl {
            if e0 == w1 {
                let top = (n0 as usize) * NCOL + n1 as usize;
                let bot = (s0 as usize) * NCOL + s1 as usize;
                t2[bot * np + top] += 1.0;
            }
        }
    }
    let lambda2 = spectral_radius(&t2, np);
    let h2 = lambda2.log10() / 2.0;

    // --- Part 2: interior area-law block counts ---
    // A(n) reusable and B(n) distinct for n = 1..MAX_N. B(n) may be None (over
    // budget); A(n) is exact for the n we attempt. We attempt through n=4: A(4) is
    // computed by the transfer DP (cheap), which then reports B(4) as out of reach.
    let pool = interior_pool();
    const MAX_N: usize = 4;
    struct Row {
        n: usize,
        cells: usize,
        reusable: u128,
        distinct: Option<u128>,
        wall_seconds: f64,
    }
    let mut block_rows: Vec<Row> = Vec::new();
    for n in 1..=MAX_N {
        let t0 = std::time::Instant::now();
        let reusable = count_reusable(&pool, n);
        let distinct = count_distinct(&pool, n, reusable);
        let wall = t0.elapsed().as_secs_f64();
        eprintln!(
            "block n={n}: A={reusable} B={} ({wall:.2}s)",
            distinct.map(|x| x.to_string()).unwrap_or_else(|| "over-budget".into())
        );
        block_rows.push(Row {
            n,
            cells: n * n,
            reusable,
            distinct,
            wall_seconds: wall,
        });
    }

    // Per-n alpha(n) = -ln rho(n)/n^2 over the exactly computed points. This
    // describes those points only; no exponent is fitted from them.
    let mut per_n_alpha: Vec<(usize, f64)> = Vec::new();
    for row in &block_rows {
        if let Some(b) = row.distinct {
            let rho = b as f64 / row.reusable as f64;
            if row.n >= 2 {
                per_n_alpha.push((row.n, -rho.ln() / (row.n * row.n) as f64));
            }
        }
    }
    let max_distinct_n = block_rows
        .iter()
        .filter(|r| r.distinct.is_some())
        .map(|r| r.n)
        .max()
        .unwrap_or(0);
    let increment2 = (lambda2 / lambda1).log10();

    // --- Emit JSON ---
    println!("{{");
    println!("  \"pieceSet\": \"official Eternity II\",");
    println!("  \"interiorPlacements\": {placements},");
    println!("  \"lambdaH\": {lambda_h:.4},");
    println!("  \"log10LambdaH\": {:.4},", lambda_h.log10());
    println!("  \"note\": \"h(n) = log10(lambda_n)/n, lambda_n the dominant eigenvalue of the width-n row-transfer operator (pieces reusable, strip sides free). h(n) decreases to the 2-D entropy density h_infinity = inf_n h(n) (Fekete), so every h(n) is an upper bound on h_infinity. Widths 1 and 2 are computed by this crate; widths 3 to 6 are carried from research/topics/entropy-area-law/compute/scripts/strip_entropy.js (stdlib Node, single thread; it agrees with this crate at widths 1 and 2).\",");
    println!("  \"hByWidth\": [");
    println!("    {{ \"width\": 1, \"lambda\": {lambda1:.3}, \"h\": {h1:.4}, \"source\": \"compute crate\" }},");
    println!("    {{ \"width\": 2, \"lambda\": {lambda2:.3}, \"h\": {h2:.4}, \"source\": \"compute crate\" }},");
    print!("{}", H_CARRIED);
    println!("  ],");
    println!("  \"hIncrementByWidth\": [");
    println!("    {{ \"width\": 2, \"increment\": {increment2:.4} }},");
    print!("{}", INCREMENTS_CARRIED);
    println!("  ],");
    print!("{}", H_INFINITY_CARRIED);
    // Area-law block counts, computed exactly in-repo over the range below.
    println!("  \"blockCountsNote\": \"A(n) = colour-valid n×n interior fillings with pieces reusable (the matching grammar); B(n) = those that use distinct pieces (true E2); rho = B/A. Computed exactly in-repo here: A(n) by a dense broken-profile transfer DP, B(n) by a parallel distinct-counting DFS bounded by A(n). B is exact through n={max_distinct_n}; n=4 is over the DFS budget (A(4) still computed). Cross-checked at n=2 against research/topics/subgrid-placement-counts (B(2)=4059952).\",");
    println!("  \"blockCounts\": [");
    let rows_json: Vec<String> = block_rows
        .iter()
        .map(|r| {
            let ratio = r
                .distinct
                .map(|b| format!("{:.6}", b as f64 / r.reusable as f64))
                .unwrap_or_else(|| "null".into());
            let distinct = r
                .distinct
                .map(|b| b.to_string())
                .unwrap_or_else(|| "null".into());
            format!(
                "    {{ \"n\": {}, \"cells\": {}, \"reusable\": {}, \"distinct\": {}, \"ratio\": {}, \"wallSeconds\": {:.2} }}",
                r.n, r.cells, r.reusable, distinct, ratio, r.wall_seconds
            )
        })
        .collect();
    println!("{}", rows_json.join(",\n"));
    println!("  ],");
    // Per-cell reusable entropy density s/n^2 = log10(A(n))/n^2 (drawn by the Lab).
    println!("  \"entropyDensityByN\": [");
    let dens_json: Vec<String> = block_rows
        .iter()
        .map(|r| {
            let s_over_n2 = (r.reusable as f64).log10() / (r.n * r.n) as f64;
            format!("    {{ \"n\": {}, \"sOverN2\": {:.3} }}", r.n, s_over_n2)
        })
        .collect();
    println!("{}", dens_json.join(",\n"));
    println!("  ],");
    let per_n_json: Vec<String> = per_n_alpha
        .iter()
        .map(|(n, a)| format!("    {{ \"n\": {n}, \"alphaN\": {a:.4} }}"))
        .collect();
    println!("  \"alphaByNNote\": \"-ln(rho(n))/n^2 at each exactly computed n = 2..{max_distinct_n}. It describes those points; no exponent is fitted from them.\",");
    println!("  \"alphaByN\": [");
    println!("{}", per_n_json.join(",\n"));
    println!("  ],");
    print!("{}", SAMPLED_AND_WITHDRAWN_CARRIED);
    println!("}}");
}
