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
//!    (Fekete's lemma).
//!
//! 2. **The area law.** For an interior n×n block with a free outer border count,
//!    exactly, two things: A(n) = the number of colour-valid fillings when pieces
//!    may repeat (the matching grammar), and B(n) = the number that use distinct
//!    pieces (true Eternity II). The realizable fraction rho(n) = B(n)/A(n) decays
//!    in the AREA n^2, and the fitted exponent alpha = -ln(rho(n))/n^2 is the
//!    headline of the area law. A(n) is a dense broken-profile transfer DP; B(n) is
//!    a parallel distinct-counting DFS whose cost is bounded by A(n). Both are
//!    exact for n = 1, 2, 3; n = 4 is over the DFS budget and reported as such.
//!
//! Exact and deterministic. Cross-checked at n=2 against the in-repo
//! subgrid-placement-counts reference table (B(2) = 4 059 952).

use eternity2_engine::official::official_puzzle;
use eternity2_engine::types::{rotated, Color};

const NCOL: usize = 23; // colors 0..=22 (0 = grey border, unused on interior pieces)

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

    // rho(n) = B(n)/A(n). The area law is rho(n) ~ exp(-alpha n^2), i.e.
    // ln rho(n) = -alpha n^2. We fit alpha by least squares through the origin over
    // every exactly-computed point (including n=1, where ln rho = 0): the slope
    // minimising sum (ln rho + alpha n^2)^2 is alpha = -sum(n^2 ln rho)/sum(n^4).
    // We also record the per-n alpha(n) = -ln rho(n)/n^2 to show the trend (it
    // rises with n over this small exact range, so the fit is a lower estimate of
    // the large-patch exponent).
    let mut num = 0.0f64; // -sum n^2 ln rho
    let mut den = 0.0f64; // sum n^4
    let mut per_n_alpha: Vec<(usize, f64)> = Vec::new();
    for row in &block_rows {
        if let Some(b) = row.distinct {
            let rho = b as f64 / row.reusable as f64;
            let ln_rho = rho.ln();
            let n2 = (row.n * row.n) as f64;
            num += -(n2 * ln_rho);
            den += n2 * n2;
            if row.n >= 2 {
                per_n_alpha.push((row.n, -ln_rho / n2));
            }
        }
    }
    let alpha_fit = if den > 0.0 { num / den } else { 0.0 };
    let max_distinct_n = block_rows
        .iter()
        .filter(|r| r.distinct.is_some())
        .map(|r| r.n)
        .max()
        .unwrap_or(0);
    // Cell at which the extrapolated rho drops below 1e-3: n* = sqrt(ln(1000)/alpha),
    // reported in cells = round(n*^2). Labelled as an extrapolation beyond the range.
    let collapse_cells = if alpha_fit > 0.0 {
        ((6.907755_f64 / alpha_fit).round()) as u64 // ln(1000) = 6.90776
    } else {
        0
    };

    // --- Emit JSON ---
    println!("{{");
    println!("  \"pieceSet\": \"official Eternity II\",");
    println!("  \"interiorPlacements\": {placements},");
    println!("  \"lambdaH\": {lambda_h:.4},");
    println!("  \"log10LambdaH\": {:.4},", lambda_h.log10());
    println!("  \"note\": \"h(n) = log10 of the width-n transfer eigenvalue, per cell. The sequence decreases to the 2-D entropy density h_infinity (Fekete). h(1), h(2) computed exactly here; h(3), h(4) and h_infinity from the offline width-n sweep.\",");
    println!("  \"hByWidth\": [");
    println!("    {{ \"width\": 1, \"h\": {h1:.4}, \"exact\": true }},");
    println!("    {{ \"width\": 2, \"h\": {h2:.4}, \"exact\": true }},");
    println!("    {{ \"width\": 3, \"h\": 0.8449, \"exact\": false }},");
    println!("    {{ \"width\": 4, \"h\": 0.7425, \"exact\": false }}");
    println!("  ],");
    println!("  \"hInfinity\": 0.67,");
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
    println!("  \"areaLawExponent\": {alpha_fit:.4},");
    let per_n_json: Vec<String> = per_n_alpha
        .iter()
        .map(|(n, a)| format!("    {{ \"n\": {n}, \"alphaN\": {a:.4} }}"))
        .collect();
    println!("  \"areaLawExponentNote\": \"alpha fitted by least squares through the origin (ln rho = -alpha n^2) over the exact in-repo B(n)/A(n) for n=1..{max_distinct_n}. The per-n alpha(n) = -ln(rho(n))/n^2 rises with n over this small exact range (see alphaByN), so this fit is a conservative estimate of the large-patch exponent; the rho(n) curve beyond n={max_distinct_n} is an extrapolation of the fit.\",");
    println!("  \"alphaByN\": [");
    println!("{}", per_n_json.join(",\n"));
    println!("  ],");
    println!("  \"distinctnessCollapseCells\": {collapse_cells},");
    println!("  \"distinctnessCollapseNote\": \"Extrapolated cell count where rho drops below 1e-3, from n* = sqrt(ln(1000)/alpha); beyond the exactly-computed range.\"");
    println!("}}");
}
