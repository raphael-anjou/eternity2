// Vol-218 BANDSAW testbed: parametric N×N E2-like instances with
// hints, exact band machinery (distinct-piece counts, exhaustive B&B,
// meet-in-the-middle band-split solve) and the repeats-allowed
// counting/tropical oracle — all selftested against brute force.
//
// All band/endgame work is COLUMN-MAJOR: within a band, vertical
// coupling (N edges + border structure of the bottom row) binds at
// every column instead of after a full free row — the same collapse
// that makes conditioned band sub-problems enumerable at all
// (vol-217 finding; row-major free-row enumeration is ~1e8× wider).
//
// Cost convention (stage4_finish-compatible): each cell pays its N
// edge and its W edge; board-edge sides are structural (candidates
// filtered to have BORDER exactly on out-facing sides). A band's
// first row pays N against a given frontier (or nothing, if the
// frontier is None — BANDSAW bottom bands pay the join at join time).

use std::collections::HashMap;

use eternity2_generator::{generate_with_solution, GeneratorConfig};

pub const NONE_COLOR: u8 = u8::MAX;

pub struct Mini {
    pub n: usize,
    pub colors: u32,
    pub seed: u64,
    pub nc: usize,                    // dense color range 0..nc (border=0)
    pub rot: Vec<[[u8; 4]; 4]>,       // pid -> rot -> [N,E,S,W]
    pub hints: Vec<(usize, u16, u8)>, // (pos, pid, rot)
    pub solution: Vec<(u16, u8)>,     // canonical (pid, rot) per pos
}

fn rotate_edges(e: [u8; 4], r: u8) -> [u8; 4] {
    let mut out = [0u8; 4];
    for i in 0..4 {
        out[i] = e[(i + 4 - r as usize) % 4];
    }
    out
}

/// E2-analog hint cells for an n×n board: four quarter hints at
/// rows/cols {2, n-3} plus the center analog of E2's (7,8).
#[must_use]
pub fn hint_cells(n: usize) -> [usize; 5] {
    let q = n - 3;
    let (cx, cy) = (n / 2 - 1, n / 2);
    [
        2 * n + 2,
        2 * n + q,
        cy * n + cx,
        q * n + 2,
        q * n + q,
    ]
}

impl Mini {
    /// Wrap an already-loaded puzzle (e.g. the official E2 CSV) in the
    /// band machinery. `solution` stays empty (unknown).
    #[must_use]
    pub fn from_puzzle(puzzle: &eternity2_core::Puzzle, hints: Vec<(usize, u16, u8)>) -> Self {
        let n = puzzle.width as usize;
        let mut rot = vec![[[0u8; 4]; 4]; n * n];
        let mut maxc = 0u8;
        for p in puzzle.pieces() {
            let e = [p.edges.top(), p.edges.right(), p.edges.bottom(), p.edges.left()];
            maxc = maxc.max(*e.iter().max().unwrap());
            let mut rr = [[0u8; 4]; 4];
            for r in 0..4 {
                rr[r] = rotate_edges(e, r as u8);
            }
            rot[p.id as usize] = rr;
        }
        Mini {
            n,
            colors: u32::from(maxc),
            seed: 0,
            nc: usize::from(maxc) + 1,
            rot,
            hints,
            solution: Vec::new(),
        }
    }

    #[must_use]
    pub fn from_seed(n: usize, colors: u32, seed: u64, hint_cells: &[usize]) -> Self {
        let (puzzle, solution) = generate_with_solution(GeneratorConfig {
            size: n as u32,
            interior_colors: colors,
            seed,
        })
        .expect("generator must succeed");
        // pieces() is SHUFFLED by the generator — index rot by piece id,
        // not iteration order (vol-218 M0 catch: 30 phantom breaks).
        let mut rot = vec![[[0u8; 4]; 4]; n * n];
        for p in puzzle.pieces() {
            let e = [p.edges.top(), p.edges.right(), p.edges.bottom(), p.edges.left()];
            let mut rr = [[0u8; 4]; 4];
            for r in 0..4 {
                rr[r] = rotate_edges(e, r as u8);
            }
            rot[p.id as usize] = rr;
        }
        // pieces are shuffled; solution[pos] indexes by position
        let mut sol = vec![(0u16, 0u8); n * n];
        for pl in &solution {
            sol[pl.position as usize] = (pl.piece_id, pl.rotation.as_u8());
        }
        let hints = hint_cells
            .iter()
            .map(|&pos| (pos, sol[pos].0, sol[pos].1))
            .collect();
        Mini { n, colors, seed, nc: colors as usize + 1, rot, hints, solution: sol }
    }

    /// Structural candidate test: BORDER exactly on out-facing sides.
    #[must_use]
    pub fn fits_cell(&self, pid: u16, rt: u8, r: usize, c: usize) -> bool {
        let o = self.rot[pid as usize][rt as usize];
        let out = [r == 0, c == self.n - 1, r == self.n - 1, c == 0];
        (0..4).all(|s| (o[s] == 0) == out[s])
    }

    /// All structural candidates at (r,c) from an availability mask
    /// over piece ids (bit pid set = available).
    #[must_use]
    pub fn cands_at(&self, r: usize, c: usize, avail: &[bool]) -> Vec<(u16, u8)> {
        let mut out = Vec::new();
        for pid in 0..self.n * self.n {
            if !avail[pid] {
                continue;
            }
            for rt in 0..4u8 {
                if self.fits_cell(pid as u16, rt, r, c) {
                    out.push((pid as u16, rt));
                }
            }
        }
        out
    }

    /// Breaks of a fully/partially placed board (placement[pos]);
    /// counts N and W edges between two PLACED cells only.
    #[must_use]
    pub fn breaks_of(&self, placement: &[Option<(u16, u8)>]) -> u32 {
        let n = self.n;
        let mut b = 0;
        for pos in 0..n * n {
            let Some((p, rt)) = placement[pos] else { continue };
            let o = self.rot[p as usize][rt as usize];
            let (r, c) = (pos / n, pos % n);
            if r > 0 {
                if let Some((p2, rt2)) = placement[pos - n] {
                    let o2 = self.rot[p2 as usize][rt2 as usize];
                    b += u32::from(o2[2] != o[0]);
                }
            }
            if c > 0 {
                if let Some((p2, rt2)) = placement[pos - 1] {
                    let o2 = self.rot[p2 as usize][rt2 as usize];
                    b += u32::from(o2[1] != o[3]);
                }
            }
        }
        b
    }
}

// ---------------- band region description ----------------

/// A band of `k` full rows `r0..r0+k`. `frontier[c]` = the color the
/// first band row's N edge must match at column c; NONE_COLOR = free
/// (no charge). Pool = explicit piece list; forced = (pos -> cand).
pub struct Band<'a> {
    pub mini: &'a Mini,
    pub r0: usize,
    pub k: usize,
    pub frontier: Option<Vec<u8>>,
    pub pool: Vec<u16>,
    pub forced: HashMap<usize, (u16, u8)>,
}

impl Band<'_> {
    /// Mixed-radix state index of the east colors entering column c
    /// (east[] is the per-cell DFS array, column-major).
    fn col_state(&self, east: &[u8], c: usize) -> usize {
        let nc = self.mini.nc;
        let mut st = 0usize;
        for j in 0..self.k {
            st = st * nc + east[(c - 1) * self.k + j] as usize;
        }
        st
    }

    /// Column-major cell sequence: (idx within band, r, c).
    fn cells(&self) -> Vec<(usize, usize)> {
        let mut v = Vec::with_capacity(self.k * self.mini.n);
        for c in 0..self.mini.n {
            for r in self.r0..self.r0 + self.k {
                v.push((r, c));
            }
        }
        v
    }

    /// Per-cell structural candidates over the band pool (pool-index
    /// based). Forced cells get exactly their single candidate.
    #[must_use]
    pub fn cand_table(&self) -> Vec<Vec<Cand>> {
        let m = self.mini;
        let mut avail = vec![false; m.n * m.n];
        for &p in &self.pool {
            avail[p as usize] = true;
        }
        let pool_idx: HashMap<u16, u8> = self
            .pool
            .iter()
            .enumerate()
            .map(|(i, &p)| (p, u8::try_from(i).expect("pool <= 64")))
            .collect();
        self.cells()
            .iter()
            .map(|&(r, c)| {
                let pos = r * m.n + c;
                let list: Vec<(u16, u8)> = match self.forced.get(&pos) {
                    Some(&f) => vec![f],
                    None => m.cands_at(r, c, &avail),
                };
                list.into_iter()
                    .map(|(p, rt)| {
                        let o = m.rot[p as usize][rt as usize];
                        Cand { pid: p, pi: pool_idx[&p], rt, n: o[0], e: o[1], s: o[2], w: o[3] }
                    })
                    .collect()
            })
            .collect()
    }
}

/// Per-cell cost-sorted candidate lists indexed by the (north, west)
/// target colors — `enter` becomes a pointer assignment instead of a
/// 200-candidate scan (the post-alloc-fix profile's residual hot
/// zone). Slot layout: [cell][tn * (nc+1) + tw], NONE targets mapped
/// to slot `nc`. Within a slot, candidates are ordered cost-0 then
/// cost-1 then cost-2, preserving cand-list order within each class —
/// IDENTICAL iteration order to the scan version (labels comparable).
pub struct CostIndex {
    pub ncw: usize,
    pub slots: Vec<Vec<Vec<(u16, u8)>>>, // [cell][tn*ncw+tw] -> (ci, cost)
}

#[must_use]
pub fn build_cost_index(band: &Band, cands: &[Vec<Cand>]) -> CostIndex {
    let nc = band.mini.nc;
    let ncw = nc + 1;
    let none = nc; // slot for NONE_COLOR
    let mut slots = Vec::with_capacity(cands.len());
    for list in cands {
        let mut cell = vec![Vec::new(); ncw * ncw];
        for (slot, entry) in cell.iter_mut().enumerate() {
            let (tn, tw) = (slot / ncw, slot % ncw);
            for want in 0u8..3 {
                for (ci, cd) in list.iter().enumerate() {
                    let mut cost = 0u8;
                    if tn != none {
                        cost += u8::from(cd.n as usize != tn);
                    }
                    if tw != none {
                        cost += u8::from(cd.w as usize != tw);
                    }
                    if cost == want {
                        entry.push((u16::try_from(ci).unwrap(), cost));
                    }
                }
            }
        }
        slots.push(cell);
    }
    CostIndex { ncw, slots }
}

impl CostIndex {
    #[inline]
    #[must_use]
    pub fn slot(&self, tn: u8, tw: u8) -> u32 {
        let nc = self.ncw - 1;
        let ti = if tn == NONE_COLOR { nc } else { tn as usize };
        let wi = if tw == NONE_COLOR { nc } else { tw as usize };
        u32::try_from(ti * self.ncw + wi).unwrap()
    }

    #[inline]
    #[must_use]
    pub fn list(&self, d: usize, slot: u32) -> &[(u16, u8)] {
        &self.slots[d][slot as usize]
    }
}

#[derive(Clone, Copy)]
pub struct Cand {
    pub pid: u16,
    pub pi: u8, // pool index (mask bit)
    pub rt: u8,
    pub n: u8,
    pub e: u8,
    pub s: u8,
    pub w: u8,
}

// ---------------- exact distinct-piece counting ----------------

pub struct ExactCounts {
    pub by_b: Vec<u64>, // completions paying exactly b, b <= bmax
    pub nodes: u64,
    pub capped: bool,
}

/// DFS count of distinct-piece band fillings by exact total cost.
/// Column-major. Budget prune at bmax + optional tropical-suffix
/// prune at column entries. Loud cap.
#[must_use]
pub fn exact_count(band: &Band, bmax: u32, node_cap: u64) -> ExactCounts {
    exact_count_pruned(band, bmax, node_cap, None)
}

#[must_use]
pub fn exact_count_pruned(
    band: &Band,
    bmax: u32,
    node_cap: u64,
    suffix: Option<&[Vec<u32>]>,
) -> ExactCounts {
    let m = band.mini;
    let n = m.n;
    let cells = band.cells();
    let cands = band.cand_table();
    let total = cells.len();
    let mut used = vec![false; band.pool.len()];
    let mut south = vec![vec![0u8; n]; band.k]; // by band-row
    let mut east = vec![0u8; total];
    let mut by_b = vec![0u64; bmax as usize + 1];
    let mut nodes = 0u64;
    let mut capped = false;

    #[allow(clippy::too_many_arguments)]
    fn rec(
        d: usize,
        spent: u32,
        cells: &[(usize, usize)],
        cands: &[Vec<Cand>],
        band: &Band,
        used: &mut [bool],
        south: &mut [Vec<u8>],
        east: &mut [u8],
        by_b: &mut [u64],
        nodes: &mut u64,
        capped: &mut bool,
        bmax: u32,
        node_cap: u64,
        suffix: Option<&[Vec<u32>]>,
    ) {
        if *capped {
            return;
        }
        if d == cells.len() {
            by_b[spent as usize] += 1;
            return;
        }
        if d > 0 && d % band.k == 0 {
            if let Some(sfx) = suffix {
                let c = d / band.k;
                if spent + sfx[c][band.col_state(east, c)] > bmax {
                    return;
                }
            }
        }
        let (r, c) = cells[d];
        let i = r - band.r0;
        let tn = if i == 0 {
            band.frontier.as_ref().map_or(NONE_COLOR, |f| f[c])
        } else {
            south[i - 1][c]
        };
        let tw = if c == 0 { NONE_COLOR } else { east[d - band.k] };
        for cd in &cands[d] {
            if used[cd.pi as usize] {
                continue;
            }
            let mut cost = spent;
            if tn != NONE_COLOR {
                cost += u32::from(cd.n != tn);
            }
            if tw != NONE_COLOR {
                cost += u32::from(cd.w != tw);
            }
            if cost > bmax {
                continue;
            }
            *nodes += 1;
            if *nodes >= node_cap {
                *capped = true;
                return;
            }
            used[cd.pi as usize] = true;
            south[i][c] = cd.s;
            east[d] = cd.e;
            rec(d + 1, cost, cells, cands, band, used, south, east, by_b, nodes, capped, bmax, node_cap, suffix);
            used[cd.pi as usize] = false;
        }
    }
    rec(0, 0, &cells, &cands, band, &mut used, &mut south, &mut east, &mut by_b, &mut nodes, &mut capped, bmax, node_cap, suffix);
    ExactCounts { by_b, nodes, capped }
}

// ---------------- repeats-allowed counting DP ----------------

/// Repeats-allowed count of band fillings by exact total cost
/// (transfer DP, column-major, inclusion-exclusion contractions).
/// Exact for the relaxed model — selftested against brute force.
#[must_use]
pub fn relax_profile(band: &Band, bmax: u32) -> Vec<f64> {
    weighted_profile(band, bmax, None)
}

/// z-weighted repeats-allowed profile: each filling contributes
/// prod_p z[pool_idx(p)]^{uses of p}. z = None ⇒ all 1 (plain count).
/// The grand-canonical object the fugacity correction needs.
#[must_use]
pub fn weighted_profile(band: &Band, bmax: u32, z: Option<&[f64]>) -> Vec<f64> {
    let m = band.mini;
    let n = m.n;
    let nc = m.nc;
    let k = band.k;
    let b = bmax as usize + 1;
    let cands = band.cand_table();

    // state tensors: with-s  = [s][e_0..e_{k-1}][b]
    //                without = [e_0..e_{k-1}][b]
    let ek: usize = nc.pow(k as u32);
    let mut cur: Vec<f64> = Vec::new(); // without-s at column boundaries
    let mut have_any = false;

    for c in 0..n {
        let mut mid: Vec<f64> = Vec::new(); // with-s inside the column
        for i in 0..k {
            let d = c * k + i;
            let last = i == k - 1;
            let tn_frontier = if i == 0 {
                band.frontier.as_ref().map_or(NONE_COLOR, |f| f[c])
            } else {
                NONE_COLOR // matched against s axis
            };
            // group candidates by (n, w, e, s)
            let mut groups: HashMap<(u8, u8, u8, u8), f64> = HashMap::new();
            for cd in &cands[d] {
                *groups.entry((cd.n, cd.w, cd.e, cd.s)).or_insert(0.0) +=
                    z.map_or(1.0, |zz| zz[cd.pi as usize]);
            }
            // output tensor
            let out_has_s = !last;
            let out_sz = if out_has_s { nc * ek * b } else { ek * b };
            let mut out = vec![0.0f64; out_sz];
            // strides for e axes in the e-block (row-major e_0..e_{k-1})
            let estr: Vec<usize> = (0..k).map(|j| nc.pow((k - 1 - j) as u32)).collect();

            if !have_any && c == 0 && i == 0 {
                // very first cell: park e_1..e_{k-1} at 0
                for (&(cn, _w, e, s), &mult) in &groups {
                    let mut cost = 0usize;
                    if tn_frontier != NONE_COLOR {
                        cost += usize::from(cn != tn_frontier);
                    }
                    if cost >= b {
                        continue;
                    }
                    let eblock = e as usize * estr[0];
                    let off = if out_has_s {
                        (s as usize * ek + eblock) * b
                    } else {
                        eblock * b
                    };
                    out[off + cost] += mult;
                }
            } else if i == 0 {
                // column start: input = without-s [e][b]; contract w on e_0
                // sums over e_0:
                let mut sum_w = vec![0.0f64; ek / nc * b]; // [e_1..e_{k-1}][b]
                let rest = ek / nc;
                for w in 0..nc {
                    let base = w * rest * b;
                    for q in 0..rest * b {
                        sum_w[q] += cur[base + q];
                    }
                }
                for (&(cn, w, e, s), &mult) in &groups {
                    let mut c0 = 0usize;
                    if tn_frontier != NONE_COLOR {
                        c0 += usize::from(cn != tn_frontier);
                    }
                    let wmatch_base = w as usize * rest * b;
                    for q in 0..rest {
                        // q indexes e_1..e_{k-1}; output e_0 = e, others = q
                        let eblock = e as usize * estr[0] + q;
                        let off = if out_has_s {
                            (s as usize * ek + eblock) * b
                        } else {
                            eblock * b
                        };
                        let qb = q * b;
                        for bb in 0..b {
                            let matched = cur[wmatch_base + qb + bb];
                            let tot = sum_w[qb + bb];
                            // w match: cost c0; w mismatch: c0+1
                            if bb + c0 < b {
                                out[off + bb + c0] += mult * matched;
                            }
                            if bb + c0 + 1 < b {
                                out[off + bb + c0 + 1] += mult * (tot - matched);
                            }
                        }
                    }
                }
            } else {
                // mid column: input = with-s [s][e][b]; contract n on s
                // axis and (if c>0) w on e_i axis.
                let ei = estr[i];
                let with_w = c > 0;
                // we need, per (kn, kw, q over other axes): tin[kn,kw,q],
                // sum_s[kw,q], sum_w[kn,q], sum_sw[q].
                let oth: Vec<usize> = (0..k).filter(|&j| j != i).map(|j| estr[j]).collect();
                let rest: usize = nc.pow((k - 1) as u32);
                // q -> offset within the e-block: mixed radix over the
                // other axes (last axis = least significant digit)
                let mut qoff = vec![0usize; rest];
                for (q, slot) in qoff.iter_mut().enumerate() {
                    let mut rem = q;
                    let mut off = 0usize;
                    for &st in oth.iter().rev() {
                        off += (rem % nc) * st;
                        rem /= nc;
                    }
                    *slot = off;
                }
                let sb = ek * b; // s stride in with-s tensor
                // sums
                let mut sum_s = vec![0.0f64; ek * b]; // [e][b]
                for s in 0..nc {
                    let base = s * sb;
                    for x in 0..ek * b {
                        sum_s[x] += mid[base + x];
                    }
                }
                let (sum_w, sum_sw) = if with_w {
                    let mut sw = vec![0.0f64; nc * rest * b]; // [s][q][b]
                    let mut ssw = vec![0.0f64; rest * b]; // [q][b]
                    for s in 0..nc {
                        for w in 0..nc {
                            let base = s * sb + w * ei * b;
                            for q in 0..rest {
                                let src = base + qoff[q] * b;
                                let dst = (s * rest + q) * b;
                                for bb in 0..b {
                                    sw[dst + bb] += mid[src + bb];
                                }
                            }
                        }
                    }
                    for s in 0..nc {
                        let base = s * rest * b;
                        for x in 0..rest * b {
                            ssw[x] += sw[base + x];
                        }
                    }
                    (sw, ssw)
                } else {
                    (Vec::new(), Vec::new())
                };

                for (&(cn, w, e, s), &mult) in &groups {
                    let kn = cn as usize;
                    let kw = w as usize;
                    for q in 0..rest {
                        let eblock_out = e as usize * ei + qoff[q];
                        let off_out = if out_has_s {
                            (s as usize * ek + eblock_out) * b
                        } else {
                            eblock_out * b
                        };
                        if with_w {
                            let t_both = kn * sb + kw * ei * b + qoff[q] * b;
                            let t_sums = kw * ei * b + qoff[q] * b; // in sum_s
                            let t_sumw = (kn * rest + q) * b;
                            let t_ssw = q * b;
                            for bb in 0..b {
                                let both = mid[t_both + bb];
                                let nmatch = sum_w[t_sumw + bb]; // n matched, any w
                                let wmatch = sum_s[t_sums + bb]; // w matched, any n
                                let tot = sum_sw[t_ssw + bb];
                                let n_only = nmatch - both; // w mismatch
                                let w_only = wmatch - both; // n mismatch
                                let neither = tot - nmatch - wmatch + both;
                                if bb < b {
                                    out[off_out + bb] += mult * both;
                                }
                                if bb + 1 < b {
                                    out[off_out + bb + 1] += mult * (n_only + w_only);
                                }
                                if bb + 2 < b {
                                    out[off_out + bb + 2] += mult * neither;
                                }
                            }
                        } else {
                            // c == 0: only the n (s-axis) contraction
                            let t_match = kn * sb + qoff[q] * b; // e_i parked at 0
                            // sum over s of mid at e-block (e_i parked 0, q)
                            let t_tot = qoff[q] * b;
                            for bb in 0..b {
                                let matched = mid[t_match + bb];
                                let tot = sum_s[t_tot + bb];
                                if bb < b {
                                    out[off_out + bb] += mult * matched;
                                }
                                if bb + 1 < b {
                                    out[off_out + bb + 1] += mult * (tot - matched);
                                }
                            }
                        }
                    }
                }
            }
            if last {
                cur = out;
                have_any = true;
            } else {
                mid = out;
            }
        }
    }
    // readout: sum over all e axes
    let mut profile = vec![0.0f64; b];
    for chunk in cur.chunks(b) {
        for (bb, v) in chunk.iter().enumerate() {
            profile[bb] += v;
        }
    }
    profile
}

/// Repeats-allowed min cost (tropical floor): smallest b with
/// relax count > 0, escalating bmax. Admissible LB for the exact
/// distinct-piece min-break.
#[must_use]
pub fn relax_floor(band: &Band, bmax0: u32) -> u32 {
    let mut bmax = bmax0;
    loop {
        let prof = relax_profile(band, bmax);
        if let Some(bb) = prof.iter().position(|&x| x > 0.0) {
            return u32::try_from(bb).unwrap();
        }
        bmax *= 2;
        assert!(bmax <= 4096, "relax_floor: no filling at any cost?");
    }
}

pub struct FugacityResult {
    pub ln_naive: f64,
    pub ln_corrected: f64,
    pub iters: u32,
    pub max_usage_err: f64,
}

/// Equal-case fugacity correction (vol-216 method) for the
/// distinct-piece count of fillings at cost <= bmax:
///   ln N ≈ min_z [ ln Z_{<=bmax}(z) − Σ_p ln z_p ]
/// solved by damped Sinkhorn z_p ← z_p / u_p^damp with
/// u_p = z_p ∂ln Z/∂z_p (forward finite differences — O(pool)
/// profile evaluations per iteration; fine at testbed scale).
/// Forced pieces keep z = 1 (used exactly once; factor cancels).
#[must_use]
pub fn fugacity_corrected_lncount(
    band: &Band,
    bmax: u32,
    tol: f64,
    max_iters: u32,
) -> FugacityResult {
    let np = band.pool.len();
    let forced_pi: Vec<bool> = {
        let mut v = vec![false; np];
        for (i, &p) in band.pool.iter().enumerate() {
            if band.forced.values().any(|&(fp, _)| fp == p) {
                v[i] = true;
            }
        }
        v
    };
    let lnz_total = |z: &[f64]| -> f64 {
        let prof = weighted_profile(band, bmax, Some(z));
        prof.iter().sum::<f64>().ln()
    };
    let mut z = vec![1.0f64; np];
    let naive = lnz_total(&z);
    let h = 0.05f64;
    let mut iters = 0;
    let mut max_err = f64::INFINITY;
    while iters < max_iters {
        iters += 1;
        let base = lnz_total(&z);
        let mut usage = vec![1.0f64; np];
        max_err = 0.0;
        for p in 0..np {
            if forced_pi[p] {
                continue;
            }
            let zp = z[p];
            z[p] = zp * (1.0 + h);
            let up = (lnz_total(&z) - base) / (1.0 + h).ln();
            z[p] = zp;
            usage[p] = up.max(1e-12);
            max_err = max_err.max((up - 1.0).abs());
        }
        if max_err < tol {
            break;
        }
        for p in 0..np {
            if !forced_pi[p] {
                // damped multiplicative step toward u_p = 1
                z[p] /= usage[p].powf(0.6);
            }
        }
    }
    let zfinal = lnz_total(&z);
    let corr: f64 = zfinal - z.iter().map(|&x| x.ln()).sum::<f64>();
    FugacityResult { ln_naive: naive, ln_corrected: corr, iters, max_usage_err: max_err }
}

const INF: u32 = u32::MAX / 4;

/// Backward tropical (min-plus) suffix table: `suffix[c][state]` =
/// minimum repeats-allowed cost of filling columns c..n-1 given the
/// east colors of the k band rows entering column c (mixed-radix
/// NC^k state; column 0 state is all-don't-care so suffix[0] is the
/// band's tropical floor at every reachable state). Admissible lower
/// bound for distinct-piece search (relaxation: piece reuse allowed,
/// pool ignored). Used as a column-entry prune in enumerate/bb.
#[must_use]
pub fn tropical_suffix(band: &Band) -> Vec<Vec<u32>> {
    let m = band.mini;
    let n = m.n;
    let nc = m.nc;
    let k = band.k;
    let cands = band.cand_table();
    let ek: usize = nc.pow(k as u32);
    let estr: Vec<usize> = (0..k).map(|j| nc.pow((k - 1 - j) as u32)).collect();

    // suffix[n] = 0
    let mut suffix: Vec<Vec<u32>> = vec![Vec::new(); n + 1];
    suffix[n] = vec![0; ek];

    // Backward over columns AND backward over rows within a column.
    // H_{i} = min over fillings of rows i..k-1 of column c (plus all
    // of columns c+1..) as a function of:
    //   s axis (iff i > 0): the south color row i's N edge will see;
    //   a_j axes, j < i : OUTGOING e_j (parameters consumed by nxt);
    //   a_j axes, j >= i: ENTERING w_j (already introduced).
    // Transition reads point values H_{i+1}[cand.s][a_i := cand.e];
    // the mismatch charges (cand.n vs s, cand.w vs w_i) attach to the
    // OUTPUT axes, so no exclusion tables are needed — exact min-plus
    // via the A/B/C/D class trick per rest-state.
    for c in (0..n).rev() {
        let fc = band.frontier.as_ref().map_or(NONE_COLOR, |f| f[c]);
        suffix[c] = trop_col_step(band, &cands, c, &suffix[c + 1], fc);
    }
    suffix
}

/// One backward column step of the tropical suffix: given the table
/// for columns c+1.. (`nxt`, indexed by the east colors entering
/// column c+1), produce the table for columns c.. with the first
/// band row's N edge charged against `frontier_c` (NONE_COLOR =
/// free). The reusable core of [`tropical_suffix`] and of the
/// fb-suffix-incremental what-if asks.
fn trop_col_step(
    band: &Band,
    cands: &[Vec<Cand>],
    c: usize,
    nxt: &[u32],
    frontier_c: u8,
) -> Vec<u32> {
    let m = band.mini;
    let nc = m.nc;
    let k = band.k;
    let ek: usize = nc.pow(k as u32);
    let estr: Vec<usize> = (0..k).map(|j| nc.pow((k - 1 - j) as u32)).collect();
    {
        let mut h: Vec<u32> = nxt.to_vec(); // H_k: no s axis
        for i in (0..k).rev() {
            let d = c * k + i;
            let tn_frontier = if i == 0 { frontier_c } else { NONE_COLOR };
            let mut groups: HashMap<(u8, u8, u8, u8), ()> = HashMap::new();
            for cd in &cands[d] {
                groups.entry((cd.n, cd.w, cd.e, cd.s)).or_insert(());
            }
            let in_has_s = i < k - 1;
            let out_has_s = i > 0;
            let out_sz = if out_has_s { nc * ek } else { ek };
            let mut out = vec![INF; out_sz];
            let ei = estr[i];
            let rest: usize = ek / nc;
            let oth: Vec<usize> = (0..k).filter(|&j| j != i).map(|j| estr[j]).collect();
            let mut qoff = vec![0usize; rest];
            for (q, slot) in qoff.iter_mut().enumerate() {
                let mut rem = q;
                let mut off = 0usize;
                for &st in oth.iter().rev() {
                    off += (rem % nc) * st;
                    rem /= nc;
                }
                *slot = off;
            }
            let charge_w = c > 0;
            for q in 0..rest {
                let bq = qoff[q];
                // per-group scalar v_g = H_{i+1}[g.s][a_i := g.e, q]
                // class tables over the OUTPUT (s, w) axes:
                // a[n][w] exact, bn[n] any-w, cw[w] any-n, dd any-any
                let mut a = vec![INF; nc * nc];
                let mut bn = vec![INF; nc];
                let mut cw = vec![INF; nc];
                let mut dd = INF;
                for (&(gn, gw, ge, gs), ()) in &groups {
                    let read = ge as usize * ei + bq;
                    let v = if in_has_s {
                        h[gs as usize * ek + read]
                    } else {
                        h[read]
                    };
                    if v >= INF {
                        continue;
                    }
                    let v = v
                        + if tn_frontier == NONE_COLOR {
                            0
                        } else {
                            u32::from(gn != tn_frontier)
                        };
                    let (ni, wi) = (gn as usize, gw as usize);
                    if v < a[ni * nc + wi] {
                        a[ni * nc + wi] = v;
                    }
                    if v < bn[ni] {
                        bn[ni] = v;
                    }
                    if v < cw[wi] {
                        cw[wi] = v;
                    }
                    if v < dd {
                        dd = v;
                    }
                }
                // fill output over (s_out, w_out)
                if out_has_s {
                    for s in 0..nc {
                        for w in 0..nc {
                            let val = if charge_w {
                                a[s * nc + w]
                                    .min(bn[s].saturating_add(1))
                                    .min(cw[w].saturating_add(1))
                                    .min(dd.saturating_add(2))
                            } else {
                                // no W charge: min over groups of
                                // v + (n != s) — w axis is dummy
                                bn[s].min(dd.saturating_add(1))
                            };
                            let oi = s * ek + w * ei + bq;
                            if val < out[oi] {
                                out[oi] = val;
                            }
                        }
                    }
                } else {
                    // i == 0: frontier charge already inside v
                    for w in 0..nc {
                        let val = if charge_w {
                            cw[w].min(dd.saturating_add(1))
                        } else {
                            dd
                        };
                        let oi = w * ei + bq;
                        if val < out[oi] {
                            out[oi] = val;
                        }
                    }
                }
            }
            h = out;
        }
        h
    }
}

/// One FORWARD column step: given `prev` = table for columns 0..c
/// (indexed by the east colors EXPOSED by column c-1, with the
/// boundary (c-1,c) edge NOT yet charged), produce the table for
/// columns 0..=c (indexed by column c's exposed east colors,
/// boundary (c,c+1) not yet charged). Charges: column c's W edges
/// against `prev`'s axes (exclusion mins — exact via row min/argmin/
/// second-min), within-column N chaining, and the first band row's
/// N edge against `frontier_c` (NONE_COLOR = free).
fn trop_forward_step(
    band: &Band,
    cands: &[Vec<Cand>],
    c: usize,
    prev: &[u32],
    frontier_c: u8,
) -> Vec<u32> {
    let m = band.mini;
    let nc = m.nc;
    let k = band.k;
    let ek: usize = nc.pow(k as u32);
    let estr: Vec<usize> = (0..k).map(|j| nc.pow((k - 1 - j) as u32)).collect();
    let charge_w = c > 0;

    // G_i = min cost of rows 0..i-1 of column c (plus columns < c),
    // as a function of:
    //   s axis (iff i > 0): the south exposed to row i;
    //   a_j, j < i : column c's OUTGOING e_j (already placed);
    //   a_j, j >= i: column c-1's exposed e_j (not yet consumed).
    // Start: G_0 = prev (no s axis). Step: place row i's candidate —
    // consume a_i (W charge, exclusion over the axis), set a_i :=
    // cand.e, require cand.n vs s axis (exclusion, i>0) or charge vs
    // frontier (i==0), expose new s = cand.s (i < k-1).
    let mut g: Vec<u32> = prev.to_vec();
    for i in 0..k {
        let d = c * k + i;
        let mut groups: HashMap<(u8, u8, u8, u8), ()> = HashMap::new();
        for cd in &cands[d] {
            groups.entry((cd.n, cd.w, cd.e, cd.s)).or_insert(());
        }
        let in_has_s = i > 0;
        let out_has_s = i < k - 1;
        let out_sz = if out_has_s { nc * ek } else { ek };
        let mut out = vec![INF; out_sz];
        let ei = estr[i];
        let rest: usize = ek / nc;
        let oth: Vec<usize> = (0..k).filter(|&j| j != i).map(|j| estr[j]).collect();
        let mut qoff = vec![0usize; rest];
        for (q, slot) in qoff.iter_mut().enumerate() {
            let mut rem = q;
            let mut off = 0usize;
            for &st in oth.iter().rev() {
                off += (rem % nc) * st;
                rem /= nc;
            }
            *slot = off;
        }
        for q in 0..rest {
            let bq = qoff[q];
            // Exact contraction tables over the input (s, a_i) plane:
            // m_sx[s][x], row mins over x per s (m1/arg/m2), col mins
            // over s per x, global min structure for double exclusion.
            // Input value read: in_has_s ? g[s*ek + x*ei + bq] : g[x*ei + bq].
            // For each group (gn, gw, ge, gs): cost contribution =
            //   W: (x == gw ? 0 : 1) if charge_w else 0  [consume a_i]
            //   N: i==0 -> (gn vs frontier); i>0 -> (s == gn ? 0 : 1)
            // Output slot: s_out = gs (if out_has_s), a_i := ge.
            // We need, per group: min over (s, x) of g[s][x] + charges.
            // = min( g[gn][gw],            both match
            //        rowminex_x(gn, gw)+1, n match, w mismatch
            //        colminex_s(gw, gn)+1, w match, n mismatch
            //        minex_both(gn, gw)+2 )
            // (i==0 / no-W cases degenerate accordingly.)
            let sdim = if in_has_s { nc } else { 1 };
            // row stats per s: min, argmin(x), second
            let mut rm1 = vec![INF; sdim];
            let mut ra1 = vec![usize::MAX; sdim];
            let mut rm2 = vec![INF; sdim];
            for s in 0..sdim {
                for x in 0..nc {
                    let v = if in_has_s {
                        g[s * ek + x * ei + bq]
                    } else {
                        g[x * ei + bq]
                    };
                    if v < rm1[s] {
                        rm2[s] = rm1[s];
                        rm1[s] = v;
                        ra1[s] = x;
                    } else if v < rm2[s] {
                        rm2[s] = v;
                    }
                }
            }
            // col stats per x over s (only needed when in_has_s)
            let (mut cm1, mut ca1, mut cm2) =
                (vec![INF; nc], vec![usize::MAX; nc], vec![INF; nc]);
            if in_has_s {
                for x in 0..nc {
                    for s in 0..nc {
                        let v = g[s * ek + x * ei + bq];
                        if v < cm1[x] {
                            cm2[x] = cm1[x];
                            cm1[x] = v;
                            ca1[x] = s;
                        } else if v < cm2[x] {
                            cm2[x] = v;
                        }
                    }
                }
            }
            for (&(gn, gw, ge, gs), ()) in &groups {
                let frontier_cost = if i == 0 && frontier_c != NONE_COLOR {
                    u32::from(gn != frontier_c)
                } else {
                    0
                };
                let (ni, wi) = (gn as usize, gw as usize);
                let prev_min = if !charge_w {
                    // a_i is consumed without charge: min over x
                    // (and the N contraction over s if present)
                    if in_has_s {
                        // min over x of (min over s of g + (s != gn))
                        // = min( rowmin over x at s=gn,
                        //        global min excluding row gn + 1 )
                        let row_gn = rm1[ni];
                        let mut other = INF;
                        for s in 0..nc {
                            if s != ni {
                                other = other.min(rm1[s]);
                            }
                        }
                        row_gn.min(other.saturating_add(1))
                    } else {
                        rm1[0]
                    }
                } else if in_has_s {
                    // both contractions with exact double exclusion
                    let both = g[ni * ek + wi * ei + bq];
                    let n_only = if ra1[ni] == wi { rm2[ni] } else { rm1[ni] };
                    let w_only = if ca1[wi] == ni { cm2[wi] } else { cm1[wi] };
                    // min over s != gn, x != gw: scan columns x != gw
                    // using col-min-excluding-row-gn
                    let mut neither = INF;
                    for x in 0..nc {
                        if x == wi {
                            continue;
                        }
                        let v = if ca1[x] == ni { cm2[x] } else { cm1[x] };
                        if v < neither {
                            neither = v;
                        }
                    }
                    both.min(n_only.saturating_add(1))
                        .min(w_only.saturating_add(1))
                        .min(neither.saturating_add(2))
                } else {
                    // i == 0 with W charge, no s axis
                    let exact = g[wi * ei + bq];
                    let excl = if ra1[0] == wi { rm2[0] } else { rm1[0] };
                    exact.min(excl.saturating_add(1))
                };
                if prev_min >= INF {
                    continue;
                }
                let total = prev_min + frontier_cost;
                let oe = ge as usize * ei + bq;
                let oi = if out_has_s {
                    gs as usize * ek + oe
                } else {
                    oe
                };
                if total < out[oi] {
                    out[oi] = total;
                }
            }
        }
        g = out;
    }
    g
}

/// Forward prefix tables: prefix[c] = min cost of columns 0..c-1
/// (boundary (c-1,c) NOT charged), indexed by column c-1's exposed
/// east colors. prefix[0] = all-zero. Identity (validated in tests):
/// for every c, min over states of prefix[c][x] + suffix[c][x] =
/// the band floor.
#[must_use]
pub fn tropical_prefix(band: &Band) -> Vec<Vec<u32>> {
    let m = band.mini;
    let n = m.n;
    let nc = m.nc;
    let ek: usize = nc.pow(band.k as u32);
    let cands = band.cand_table();
    let mut prefix: Vec<Vec<u32>> = vec![Vec::new(); n + 1];
    prefix[0] = vec![0; ek];
    for c in 0..n {
        let fc = band.frontier.as_ref().map_or(NONE_COLOR, |f| f[c]);
        prefix[c + 1] = trop_forward_step(band, &cands, c, &prefix[c], fc);
    }
    prefix
}

/// fb-suffix-incremental: what-if floor when ONE frontier column is
/// recolored (Mode A: full frontier known — re-rank/steer the row
/// above a banked band). One column-step + a min-dot: ~ms at k=3.
#[must_use]
pub fn whatif_floor(
    band: &Band,
    cands: &[Vec<Cand>],
    prefix: &[Vec<u32>],
    suffix: &[Vec<u32>],
    c: usize,
    new_color: u8,
) -> u32 {
    let g = trop_col_step(band, cands, c, &suffix[c + 1], new_color);
    prefix[c]
        .iter()
        .zip(g.iter())
        .map(|(&p, &s)| p.saturating_add(s))
        .min()
        .unwrap()
}

// ---------------- exhaustive B&B (ground truth) ----------------

pub struct BbResult {
    pub best: Option<u32>,
    pub nodes: u64,
    pub elapsed_ms: u128,
    pub capped: bool,
    pub best_fill: Vec<(usize, u16, u8)>, // (pos, pid, rot)
}

/// Anytime branch-and-bound min-break fill of a band; exhaustive when
/// budget/node caps are not hit (capped=false ⇒ best is EXACT).
/// Column-major, cost-bucketed candidate order (greedy leftmost),
/// optional tropical-suffix prune at column entries.
#[must_use]
pub fn bb_min_break(
    band: &Band,
    max_breaks: u32,
    budget_ms: u64,
    node_cap: u64,
    suffix: Option<&[Vec<u32>]>,
) -> BbResult {
    bb_scan_core(band, max_breaks, budget_ms, node_cap, suffix)
}

fn bb_scan_core(
    band: &Band,
    max_breaks: u32,
    budget_ms: u64,
    node_cap: u64,
    suffix: Option<&[Vec<u32>]>,
) -> BbResult {
    let m = band.mini;
    let n = m.n;
    let cells = band.cells();
    let cands = band.cand_table();
    let total = cells.len();
    let t0 = std::time::Instant::now();

    let mut used = vec![false; band.pool.len()];
    let mut south = vec![vec![0u8; n]; band.k];
    let mut east = vec![0u8; total];
    let mut order: Vec<Vec<(u16, u8)>> = vec![Vec::new(); total]; // (cand idx, cost)
    let mut cursor = vec![0usize; total];
    let mut chosen = vec![0usize; total];
    let mut spent = vec![0u32; total + 1];
    let mut best: Option<u32> = None;
    let mut best_fill: Vec<(usize, u16, u8)> = Vec::new();
    let mut nodes = 0u64;
    let mut capped = false;

    let enter = |d: usize,
                 spent_here: u32,
                 budget_now: u32,
                 used: &[bool],
                 south: &[Vec<u8>],
                 east: &[u8],
                 order: &mut Vec<Vec<(u16, u8)>>,
                 cursor: &mut Vec<usize>| {
        if d > 0 && d % band.k == 0 {
            if let Some(sfx) = suffix {
                let cc = d / band.k;
                if spent_here + sfx[cc][band.col_state(east, cc)] > budget_now {
                    order[d].clear();
                    cursor[d] = 0;
                    return;
                }
            }
        }
        let (r, c) = cells[d];
        let i = r - band.r0;
        let tn = if i == 0 {
            band.frontier.as_ref().map_or(NONE_COLOR, |f| f[c])
        } else {
            south[i - 1][c]
        };
        let tw = if c == 0 { NONE_COLOR } else { east[d - band.k] };
        let list = &cands[d];
        // three cost passes straight into the retained-capacity order
        // buffer — per-entry Vec allocation here dominated the whole
        // solver (malloc/free ≈ half the vol-218 hot-zone samples)
        let o = &mut order[d];
        o.clear();
        for want in 0u8..3 {
            for (ci, cd) in list.iter().enumerate() {
                if used[cd.pi as usize] {
                    continue;
                }
                let mut cost = 0u8;
                if tn != NONE_COLOR {
                    cost += u8::from(cd.n != tn);
                }
                if tw != NONE_COLOR {
                    cost += u8::from(cd.w != tw);
                }
                if cost == want {
                    o.push((u16::try_from(ci).unwrap(), cost));
                }
            }
        }
        cursor[d] = 0;
    };

    let mut depth = 0usize;
    enter(0, 0, max_breaks, &used, &south, &east, &mut order, &mut cursor);
    loop {
        if (nodes & 0x3FFF) == 0
            && (t0.elapsed().as_millis() as u64 >= budget_ms || nodes >= node_cap)
        {
            capped = true;
            break;
        }
        let budget = best.map_or(max_breaks, |bv| bv.saturating_sub(1).min(max_breaks));
        let mut advanced = false;
        {
            let mut oi = cursor[depth];
            while oi < order[depth].len() {
                let (ci, cost) = order[depth][oi];
                let ns = spent[depth] + u32::from(cost);
                if ns > budget {
                    oi = order[depth].len();
                    break;
                }
                let cd = cands[depth][ci as usize];
                if used[cd.pi as usize] {
                    oi += 1;
                    continue;
                }
                nodes += 1;
                chosen[depth] = oi;
                used[cd.pi as usize] = true;
                let (r, c) = cells[depth];
                south[r - band.r0][c] = cd.s;
                east[depth] = cd.e;
                spent[depth + 1] = ns;
                cursor[depth] = oi;
                depth += 1;
                advanced = true;
                break;
            }
            if !advanced {
                cursor[depth] = oi;
            }
        }
        if advanced {
            if depth == total {
                let tot = spent[total];
                if best.is_none_or(|bv| tot < bv) {
                    best = Some(tot);
                    best_fill = (0..total)
                        .map(|d| {
                            let cd = cands[d][order[d][chosen[d]].0 as usize];
                            let (r, c) = cells[d];
                            (r * n + c, cd.pid, cd.rt)
                        })
                        .collect();
                    if tot == 0 {
                        break;
                    }
                }
                depth -= 1;
                let cd = cands[depth][order[depth][chosen[depth]].0 as usize];
                used[cd.pi as usize] = false;
                cursor[depth] = chosen[depth] + 1;
            } else {
                enter(depth, spent[depth], best.map_or(max_breaks, |bv| bv.saturating_sub(1).min(max_breaks)), &used, &south, &east, &mut order, &mut cursor);
            }
        } else {
            if depth == 0 {
                break;
            }
            depth -= 1;
            let cd = cands[depth][order[depth][chosen[depth]].0 as usize];
            used[cd.pi as usize] = false;
            cursor[depth] = chosen[depth] + 1;
        }
    }
    BbResult { best, nodes, elapsed_ms: t0.elapsed().as_millis(), capped, best_fill }
}



/// EXPERIMENTAL indexed variant of `bb_min_break` (per-cell (tn,tw)
/// cost-sorted lists). Vol-218 contended A/B: label phase 2.1×
/// faster, deterministic proof tree 1.86× SLOWER — the ~27 MB index
/// thrashes M1's L2 where the scan's ~100 KB tables stay hot. Bench
/// on a quiet machine before adopting; the scan core stays default.
#[must_use]
pub fn bb_min_break_idx(
    band: &Band,
    max_breaks: u32,
    budget_ms: u64,
    node_cap: u64,
    suffix: Option<&[Vec<u32>]>,
    idx_in: Option<&CostIndex>,
) -> BbResult {
    let m = band.mini;
    let n = m.n;
    let cells = band.cells();
    let cands = band.cand_table();
    let built_idx: Option<CostIndex> = if idx_in.is_none() {
        Some(build_cost_index(band, &cands))
    } else {
        None
    };
    let idx: &CostIndex = idx_in.unwrap_or_else(|| built_idx.as_ref().unwrap());
    let total = cells.len();
    let t0 = std::time::Instant::now();

    let mut used = vec![false; band.pool.len()];
    let mut south = vec![vec![0u8; n]; band.k];
    let mut east = vec![0u8; total];
    const PRUNED: u32 = u32::MAX;
    // order stores the (tn,tw) SLOT id per depth; lists live in idx
    let mut order: Vec<u32> = vec![PRUNED; total];
    let mut cursor = vec![0usize; total];
    let mut chosen = vec![0usize; total];
    let mut spent = vec![0u32; total + 1];
    let mut best: Option<u32> = None;
    let mut best_fill: Vec<(usize, u16, u8)> = Vec::new();
    let mut nodes = 0u64;
    let mut capped = false;

    let enter = |d: usize,
                 spent_here: u32,
                 budget_now: u32,
                 south: &[Vec<u8>],
                 east: &[u8],
                 order: &mut Vec<u32>,
                 cursor: &mut Vec<usize>| {
        if d > 0 && d % band.k == 0 {
            if let Some(sfx) = suffix {
                let cc = d / band.k;
                if spent_here + sfx[cc][band.col_state(east, cc)] > budget_now {
                    order[d] = PRUNED;
                    cursor[d] = 0;
                    return;
                }
            }
        }
        let (r, c) = cells[d];
        let i = r - band.r0;
        let tn = if i == 0 {
            band.frontier.as_ref().map_or(NONE_COLOR, |f| f[c])
        } else {
            south[i - 1][c]
        };
        let tw = if c == 0 { NONE_COLOR } else { east[d - band.k] };
        // O(1): record the (tn,tw) slot; the precomputed cost-sorted
        // list lives in idx. used[] filtering happens at iteration.
        // Identical visit order to the scan version.
        order[d] = idx.slot(tn, tw);
        cursor[d] = 0;
    };

    let mut depth = 0usize;
    enter(0, 0, max_breaks, &south, &east, &mut order, &mut cursor);
    loop {
        if (nodes & 0x3FFF) == 0
            && (t0.elapsed().as_millis() as u64 >= budget_ms || nodes >= node_cap)
        {
            capped = true;
            break;
        }
        let budget = best.map_or(max_breaks, |bv| bv.saturating_sub(1).min(max_breaks));
        let mut advanced = false;
        {
            let list = if order[depth] == PRUNED {
                &[][..]
            } else {
                idx.list(depth, order[depth])
            };
            let mut oi = cursor[depth];
            while oi < list.len() {
                let (ci, cost) = list[oi];
                let ns = spent[depth] + u32::from(cost);
                if ns > budget {
                    oi = list.len();
                    break;
                }
                let cd = cands[depth][ci as usize];
                if used[cd.pi as usize] {
                    oi += 1;
                    continue;
                }
                nodes += 1;
                chosen[depth] = oi;
                used[cd.pi as usize] = true;
                let (r, c) = cells[depth];
                south[r - band.r0][c] = cd.s;
                east[depth] = cd.e;
                spent[depth + 1] = ns;
                cursor[depth] = oi;
                depth += 1;
                advanced = true;
                break;
            }
            if !advanced {
                cursor[depth] = oi;
            }
        }
        if advanced {
            if depth == total {
                let tot = spent[total];
                if best.is_none_or(|bv| tot < bv) {
                    best = Some(tot);
                    best_fill = (0..total)
                        .map(|d| {
                            let ci = idx.list(d, order[d])[chosen[d]].0;
                            let cd = cands[d][ci as usize];
                            let (r, c) = cells[d];
                            (r * n + c, cd.pid, cd.rt)
                        })
                        .collect();
                    if tot == 0 {
                        break;
                    }
                }
                depth -= 1;
                let ci = idx.list(depth, order[depth])[chosen[depth]].0;
                used[cands[depth][ci as usize].pi as usize] = false;
                cursor[depth] = chosen[depth] + 1;
            } else {
                enter(depth, spent[depth], best.map_or(max_breaks, |bv| bv.saturating_sub(1).min(max_breaks)), &south, &east, &mut order, &mut cursor);
            }
        } else {
            if depth == 0 {
                break;
            }
            depth -= 1;
            let ci = idx.list(depth, order[depth])[chosen[depth]].0;
            used[cands[depth][ci as usize].pi as usize] = false;
            cursor[depth] = chosen[depth] + 1;
        }
    }
    BbResult { best, nodes, elapsed_ms: t0.elapsed().as_millis(), capped, best_fill }
}

// ---------------- BANDSAW: meet-in-the-middle exact solve ----------------

pub struct BandsawResult {
    /// Proven-optimal min-break (None if not proven within caps).
    pub best: Option<u32>,
    /// Best completion found even if not proven optimal (valid UB).
    pub upper: Option<u32>,
    /// Exact certified lower bound: every budget round ≤ lb-1
    /// completed without a join, so b* >= lb. Distinctness-aware
    /// (unlike the relax floor).
    pub lb: u32,
    pub final_budget: u32,
    pub top_raw: u64,
    pub bottom_raw: u64,
    pub top_keys: usize,
    pub bottom_keys: usize,
    pub bottom_mask_groups: usize,
    pub join_pairs: u64,
    pub elapsed_ms: u128,
    pub capped: bool,
}

fn pack_vec(v: &[u8]) -> u128 {
    let mut x = 0u128;
    for (i, &c) in v.iter().enumerate() {
        x |= u128::from(c) << (5 * i);
    }
    x
}

fn vec_mismatch(a: u128, b: u128, n: usize) -> u32 {
    let mut cnt = 0;
    for i in 0..n {
        let ca = (a >> (5 * i)) & 31;
        let cb = (b >> (5 * i)) & 31;
        cnt += u32::from(ca != cb);
    }
    cnt
}

/// Enumerate all distinct-piece fillings of a band at cost <= budget,
/// reporting (used-pool-mask, interface vector, cost) at each leaf.
/// `interface`: Top = souths of the LAST band row; Bottom = norths of
/// the FIRST band row (whose N edges are NOT charged here).
fn enumerate_band(
    band: &Band,
    budget: u32,
    charge_first_row_n: bool,
    interface_top: bool,
    node_cap: u64,
    suffix: Option<&[Vec<u32>]>,
    mut leaf: impl FnMut(u64, u128, u32),
) -> (u64, u64, bool) {
    let m = band.mini;
    let n = m.n;
    let cells = band.cells();
    let cands = band.cand_table();
    let total = cells.len();
    let mut used_mask = 0u64;
    let mut used = vec![false; band.pool.len()];
    let mut south = vec![vec![0u8; n]; band.k];
    let mut north0 = vec![0u8; n];
    let mut east = vec![0u8; total];
    let mut raw = 0u64;
    let mut nodes = 0u64;
    let mut capped = false;

    #[allow(clippy::too_many_arguments)]
    fn rec(
        d: usize,
        spent: u32,
        budget: u32,
        charge_first: bool,
        iface_top: bool,
        cells: &[(usize, usize)],
        cands: &[Vec<Cand>],
        band: &Band,
        used: &mut [bool],
        used_mask: &mut u64,
        south: &mut [Vec<u8>],
        north0: &mut [u8],
        east: &mut [u8],
        raw: &mut u64,
        nodes: &mut u64,
        capped: &mut bool,
        node_cap: u64,
        suffix: Option<&[Vec<u32>]>,
        leaf: &mut impl FnMut(u64, u128, u32),
    ) {
        if *capped {
            return;
        }
        if d > 0 && d % band.k == 0 {
            if let Some(sfx) = suffix {
                let c = d / band.k;
                if spent + sfx[c][band.col_state(east, c)] > budget {
                    return;
                }
            }
        }
        if d == cells.len() {
            *raw += 1;
            let iface = if iface_top {
                pack_vec(&south[band.k - 1])
            } else {
                pack_vec(north0)
            };
            leaf(*used_mask, iface, spent);
            return;
        }
        let (r, c) = cells[d];
        let i = r - band.r0;
        let tn = if i == 0 {
            if charge_first {
                band.frontier.as_ref().map_or(NONE_COLOR, |f| f[c])
            } else {
                NONE_COLOR
            }
        } else {
            south[i - 1][c]
        };
        let tw = if c == 0 { NONE_COLOR } else { east[d - band.k] };
        for cd in &cands[d] {
            if used[cd.pi as usize] {
                continue;
            }
            let mut cost = spent;
            if tn != NONE_COLOR {
                cost += u32::from(cd.n != tn);
            }
            if tw != NONE_COLOR {
                cost += u32::from(cd.w != tw);
            }
            if cost > budget {
                continue;
            }
            *nodes += 1;
            if *nodes >= node_cap {
                *capped = true;
                return;
            }
            used[cd.pi as usize] = true;
            *used_mask |= 1u64 << cd.pi;
            south[i][c] = cd.s;
            if i == 0 {
                north0[c] = cd.n;
            }
            east[d] = cd.e;
            rec(d + 1, cost, budget, charge_first, iface_top, cells, cands, band, used, used_mask, south, north0, east, raw, nodes, capped, node_cap, suffix, leaf);
            used[cd.pi as usize] = false;
            *used_mask &= !(1u64 << cd.pi);
        }
    }
    rec(0, 0, budget, charge_first_row_n, interface_top, &cells, &cands, band, &mut used, &mut used_mask, &mut south, &mut north0, &mut east, &mut raw, &mut nodes, &mut capped, node_cap, suffix, &mut leaf);
    (raw, nodes, capped)
}

/// Exact min-break of a 2h-row band via meet-in-the-middle with EXACT
/// complementary-pool accounting: enumerate the top h rows at budget
/// B keyed by (pool mask, south vector); group tops by mask; for each
/// distinct top mask enumerate the bottom h rows from EXACTLY the
/// complement pool (narrow branching — this is where the
/// vertical-coupling collapse pays); join on interface vectors.
/// Iterative deepening on B from an admissible lower bound.
/// Exact when not capped.
#[must_use]
pub fn bandsaw(band: &Band, lb: u32, node_cap: u64) -> BandsawResult {
    assert!(band.k % 2 == 0, "bandsaw needs an even row count");
    let m = band.mini;
    let h = band.k / 2;
    let t0 = std::time::Instant::now();

    let top_band = Band {
        mini: m,
        r0: band.r0,
        k: h,
        frontier: band.frontier.clone(),
        pool: band.pool.clone(),
        forced: band.forced.clone(),
    };
    // full-pool bottom band: its tropical suffix is admissible for
    // every complement-pool group (superset pool ⇒ weaker relaxation)
    let bot_full = Band {
        mini: m,
        r0: band.r0 + h,
        k: h,
        frontier: None,
        pool: band.pool.clone(),
        forced: band.forced.clone(),
    };
    let top_sfx = tropical_suffix(&top_band);
    let bot_sfx = tropical_suffix(&bot_full);
    let top_floor = *top_sfx[0].iter().min().unwrap();
    let bot_floor = *bot_sfx[0].iter().min().unwrap();

    // the optimum splits b* = ct + cb + H with cb >= bot_floor and
    // ct >= top_floor, so tops need enumeration only to B - bot_floor
    // and the ID can start at the split lower bound.
    let mut budget = lb.max(top_floor + bot_floor);
    let mut global_upper: Option<u32> = None;
    loop {
        if global_upper.is_some_and(|u| u <= budget) {
            // rounds < budget all completed joinless ⇒ b* = u
            return BandsawResult {
                best: global_upper,
                upper: global_upper,
                lb: global_upper.unwrap(),
                final_budget: budget,
                top_raw: 0,
                bottom_raw: 0,
                top_keys: 0,
                bottom_keys: 0,
                bottom_mask_groups: 0,
                join_pairs: 0,
                elapsed_ms: t0.elapsed().as_millis(),
                capped: false,
            };
        }
        let mut top: HashMap<(u64, u128), u32> = HashMap::new();
        let top_budget = budget - bot_floor;
        let (top_raw, _tn, tcap) = enumerate_band(
            &top_band,
            top_budget,
            true,
            true,
            node_cap,
            Some(&top_sfx),
            |mask, v, cost| {
                top.entry((mask, v))
                    .and_modify(|e| *e = (*e).min(cost))
                    .or_insert(cost);
            },
        );

        // group tops by pool mask
        let mut groups: HashMap<u64, Vec<(u128, u32)>> = HashMap::new();
        for (&(mask, v), &ct) in &top {
            groups.entry(mask).or_default().push((v, ct));
        }

        let mut best: Option<u32> = None;
        let mut join_pairs = 0u64;
        let mut bottom_raw = 0u64;
        let mut bottom_keys = 0usize;
        let mut capped = tcap;
        let mask_groups = groups.len();

        for (mask, tops) in &groups {
            if capped {
                break;
            }
            let ct_min = tops.iter().map(|&(_, c)| c).min().unwrap();
            if ct_min > budget {
                continue;
            }
            // complement pool, in band-pool index space
            let comp_pool: Vec<u16> = band
                .pool
                .iter()
                .enumerate()
                .filter(|&(i, _)| mask & (1u64 << i) == 0)
                .map(|(_, &p)| p)
                .collect();
            // a top that consumed a bottom-forced piece admits no bottom
            let bottom_forced_ok = band
                .forced
                .iter()
                .filter(|&(&pos, _)| pos >= (band.r0 + h) * m.n)
                .all(|(_, &(p, _))| comp_pool.contains(&p));
            if !bottom_forced_ok {
                continue;
            }
            let bot_band = Band {
                mini: m,
                r0: band.r0 + h,
                k: h,
                frontier: None,
                pool: comp_pool,
                forced: band.forced.clone(),
            };
            let mut bots: HashMap<u128, u32> = HashMap::new();
            let (braw, _bn, bcap) = enumerate_band(
                &bot_band,
                budget - ct_min,
                false,
                false,
                node_cap,
                Some(&bot_sfx),
                |_bmask, u, cb| {
                    bots.entry(u)
                        .and_modify(|e| *e = (*e).min(cb))
                        .or_insert(cb);
                },
            );
            bottom_raw += braw;
            bottom_keys += bots.len();
            capped |= bcap;
            for &(v, ct) in tops {
                for (&u, &cb) in &bots {
                    join_pairs += 1;
                    let tot = ct + cb + vec_mismatch(v, u, m.n);
                    if best.is_none_or(|bv| tot < bv) {
                        best = Some(tot);
                    }
                }
            }
        }

        if let Some(b) = best {
            if global_upper.is_none_or(|u| b < u) {
                global_upper = Some(b);
            }
        }
        let done = global_upper.is_some_and(|bv| bv <= budget);
        if done || capped || budget > 512 {
            return BandsawResult {
                best: if done { global_upper } else { None },
                upper: global_upper,
                // capped during round `budget` ⇒ rounds < budget done
                lb: if done {
                    global_upper.unwrap()
                } else if capped {
                    budget
                } else {
                    budget + 1
                },
                final_budget: budget,
                top_raw,
                bottom_raw,
                top_keys: top.len(),
                bottom_keys,
                bottom_mask_groups: mask_groups,
                join_pairs,
                elapsed_ms: t0.elapsed().as_millis(),
                capped,
            };
        }
        budget += 1;
    }
}

// ---------------- entry helpers ----------------

/// Build the endgame band (rows r0..n) for an entry whose rows 0..r0
/// are placed. Pool = all pieces not used by the entry; forced = hints
/// inside the band. frontier = souths of row r0-1.
#[must_use]
pub fn endgame_band<'a>(m: &'a Mini, placement: &[Option<(u16, u8)>], r0: usize) -> Band<'a> {
    let n = m.n;
    let mut used = vec![false; n * n];
    for pos in 0..r0 * n {
        let (p, _) = placement[pos].expect("entry rows must be complete");
        assert!(!used[p as usize], "dup piece in entry");
        used[p as usize] = true;
    }
    let pool: Vec<u16> = (0..n * n)
        .filter(|&p| !used[p])
        .map(|p| u16::try_from(p).unwrap())
        .collect();
    let frontier: Vec<u8> = (0..n)
        .map(|c| {
            let (p, rt) = placement[(r0 - 1) * n + c].unwrap();
            m.rot[p as usize][rt as usize][2]
        })
        .collect();
    let forced: HashMap<usize, (u16, u8)> = m
        .hints
        .iter()
        .filter(|&&(pos, _, _)| pos >= r0 * n)
        .map(|&(pos, p, rt)| (pos, (p, rt)))
        .collect();
    Band { mini: m, r0, k: n - r0, frontier: Some(frontier), pool, forced }
}

// ---------------- tests ----------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Plain brute force: distinct-piece fillings by exact cost,
    /// row-major recursion with NO pruning tricks (independent of the
    /// column-major machinery under test).
    fn brute_exact(band: &Band, bmax: u32) -> Vec<u64> {
        let m = band.mini;
        let n = m.n;
        let mut avail0 = vec![false; n * n];
        for &p in &band.pool {
            avail0[p as usize] = true;
        }
        let mut by_b = vec![0u64; bmax as usize + 1];
        // row-major cells
        let cells: Vec<(usize, usize)> = (band.r0..band.r0 + band.k)
            .flat_map(|r| (0..n).map(move |c| (r, c)))
            .collect();
        fn rec(
            d: usize,
            spent: u32,
            cells: &[(usize, usize)],
            band: &Band,
            grid: &mut Vec<Option<(u16, u8)>>,
            avail: &mut [bool],
            by_b: &mut [u64],
            bmax: u32,
        ) {
            let m = band.mini;
            let n = m.n;
            if d == cells.len() {
                by_b[spent as usize] += 1;
                return;
            }
            let (r, c) = cells[d];
            let pos = r * n + c;
            let forced = band.forced.get(&pos).copied();
            let cands: Vec<(u16, u8)> = match forced {
                Some(f) => vec![f],
                None => m.cands_at(r, c, avail),
            };
            for (p, rt) in cands {
                if !avail[p as usize] {
                    continue;
                }
                let o = m.rot[p as usize][rt as usize];
                let mut cost = spent;
                if r == band.r0 {
                    if let Some(f) = &band.frontier {
                        if f[c] != NONE_COLOR {
                            cost += u32::from(o[0] != f[c]);
                        }
                    }
                } else {
                    let (p2, rt2) = grid[pos - n].unwrap();
                    cost += u32::from(m.rot[p2 as usize][rt2 as usize][2] != o[0]);
                }
                if c > 0 {
                    if let Some((p2, rt2)) = grid[pos - 1] {
                        cost += u32::from(m.rot[p2 as usize][rt2 as usize][1] != o[3]);
                    }
                }
                if cost > bmax {
                    continue;
                }
                avail[p as usize] = false;
                grid[pos] = Some((p, rt));
                rec(d + 1, cost, cells, band, grid, avail, by_b, bmax);
                grid[pos] = None;
                avail[p as usize] = true;
            }
        }
        let mut grid = vec![None; n * n];
        rec(0, 0, &cells, band, &mut grid, &mut avail0, &mut by_b, bmax);
        by_b
    }

    /// Brute force with repeats allowed (pieces never consumed).
    fn brute_relax(band: &Band, bmax: u32) -> Vec<u64> {
        let m = band.mini;
        let n = m.n;
        let mut avail = vec![true; n * n];
        for p in 0..n * n {
            avail[p] = band.pool.contains(&(u16::try_from(p).unwrap()));
        }
        let mut by_b = vec![0u64; bmax as usize + 1];
        let cells: Vec<(usize, usize)> = (band.r0..band.r0 + band.k)
            .flat_map(|r| (0..n).map(move |c| (r, c)))
            .collect();
        fn rec(
            d: usize,
            spent: u32,
            cells: &[(usize, usize)],
            band: &Band,
            grid: &mut Vec<Option<(u16, u8)>>,
            avail: &[bool],
            by_b: &mut [u64],
            bmax: u32,
        ) {
            let m = band.mini;
            let n = m.n;
            if d == cells.len() {
                by_b[spent as usize] += 1;
                return;
            }
            let (r, c) = cells[d];
            let pos = r * n + c;
            let cands: Vec<(u16, u8)> = match band.forced.get(&pos).copied() {
                Some(f) => vec![f],
                None => m.cands_at(r, c, avail),
            };
            for (p, rt) in cands {
                let o = m.rot[p as usize][rt as usize];
                let mut cost = spent;
                if r == band.r0 {
                    if let Some(f) = &band.frontier {
                        if f[c] != NONE_COLOR {
                            cost += u32::from(o[0] != f[c]);
                        }
                    }
                } else {
                    let (p2, rt2) = grid[pos - n].unwrap();
                    cost += u32::from(m.rot[p2 as usize][rt2 as usize][2] != o[0]);
                }
                if c > 0 {
                    if let Some((p2, rt2)) = grid[pos - 1] {
                        cost += u32::from(m.rot[p2 as usize][rt2 as usize][1] != o[3]);
                    }
                }
                if cost > bmax {
                    continue;
                }
                grid[pos] = Some((p, rt));
                rec(d + 1, cost, cells, band, grid, avail, by_b, bmax);
                grid[pos] = None;
            }
        }
        let mut grid = vec![None; n * n];
        rec(0, 0, &cells, band, &mut grid, &avail, &mut by_b, bmax);
        by_b
    }

    fn canonical_prefix(m: &Mini, r0: usize) -> Vec<Option<(u16, u8)>> {
        let n = m.n;
        let mut placement = vec![None; n * n];
        for pos in 0..r0 * n {
            placement[pos] = Some(m.solution[pos]);
        }
        placement
    }

    #[test]
    fn dbg_canonical_zero_breaks_5x5() {
        let m = Mini::from_seed(5, 4, 11, &[]);
        let n = m.n;
        let mut placement = vec![None; n * n];
        for pos in 0..n * n {
            placement[pos] = Some(m.solution[pos]);
        }
        assert_eq!(m.breaks_of(&placement), 0, "canonical must score 0");
        // canonical band filling must be admissible candidate-wise
        let prefix = canonical_prefix(&m, 3);
        let band = endgame_band(&m, &prefix, 3);
        let cands = band.cand_table();
        for (d, &(r, c)) in band.cells().iter().enumerate() {
            let pos = r * n + c;
            let (p, rt) = m.solution[pos];
            assert!(
                cands[d].iter().any(|cd| cd.pid == p && cd.rt == rt),
                "canonical ({p},{rt}) missing from cands at ({r},{c})"
            );
        }
        let counts = exact_count(&band, 0, u64::MAX);
        assert!(counts.by_b[0] >= 1, "canonical 0-break filling not counted");
    }

    #[test]
    fn m0_exact_count_matches_brute_force() {
        for seed in [11u64, 12, 13] {
            let m = Mini::from_seed(5, 4, seed, &[]);
            for r0 in [3usize, 2] {
                let placement = canonical_prefix(&m, r0);
                let band = endgame_band(&m, &placement, r0);
                let bmax = 3;
                let brute = brute_exact(&band, bmax);
                let fast = exact_count(&band, bmax, u64::MAX);
                assert!(!fast.capped);
                assert_eq!(brute, fast.by_b, "seed {seed} r0 {r0}");
            }
        }
    }

    #[test]
    fn m0_relax_profile_matches_brute_force() {
        for seed in [11u64, 12, 13] {
            let m = Mini::from_seed(4, 3, seed, &[]);
            for r0 in [2usize] {
                let placement = canonical_prefix(&m, r0);
                let band = endgame_band(&m, &placement, r0);
                let bmax = 3;
                let brute = brute_relax(&band, bmax);
                let fast = relax_profile(&band, bmax);
                for bb in 0..=bmax as usize {
                    let bf = brute[bb] as f64;
                    assert!(
                        (fast[bb] - bf).abs() <= 1e-6 * bf.max(1.0),
                        "seed {seed} r0 {r0} b {bb}: relax {} vs brute {}",
                        fast[bb],
                        brute[bb]
                    );
                }
            }
        }
    }

    #[test]
    fn m0_tropical_suffix_floor_and_prune_exactness() {
        for seed in [11u64, 12, 13] {
            let m = Mini::from_seed(5, 4, seed, &[]);
            for r0 in [3usize, 2] {
                let placement = canonical_prefix(&m, r0);
                let band = endgame_band(&m, &placement, r0);
                let sfx = tropical_suffix(&band);
                let fl = relax_floor(&band, 4);
                assert_eq!(
                    *sfx[0].iter().min().unwrap(),
                    fl,
                    "suffix floor != relax floor (seed {seed} r0 {r0})"
                );
                // the suffix prune must not change exact counts
                let a = exact_count(&band, 3, u64::MAX);
                let b = exact_count_pruned(&band, 3, u64::MAX, Some(&sfx));
                assert_eq!(a.by_b, b.by_b, "prune changed counts (seed {seed} r0 {r0})");
                assert!(b.nodes <= a.nodes, "prune must not add nodes");
            }
        }
    }

    #[test]
    fn m0_prefix_suffix_identity_and_whatif() {
        for seed in [11u64, 12, 13] {
            let m = Mini::from_seed(5, 4, seed, &[]);
            for r0 in [3usize, 2] {
                let placement = canonical_prefix(&m, r0);
                let mut band = endgame_band(&m, &placement, r0);
                let mut f = band.frontier.clone().unwrap();
                f[1] = if f[1] == 1 { 2 } else { 1 }; // non-trivial floor
                band.frontier = Some(f);
                let cands = band.cand_table();
                let sfx = tropical_suffix(&band);
                let pfx = tropical_prefix(&band);
                let floor = *sfx[0].iter().min().unwrap();
                for c in 0..m.n {
                    let v = pfx[c]
                        .iter()
                        .zip(sfx[c].iter())
                        .map(|(&p, &s)| p.saturating_add(s))
                        .min()
                        .unwrap();
                    assert_eq!(v, floor, "identity broken at c {c} (seed {seed} r0 {r0})");
                }
                assert_eq!(*pfx[m.n].iter().min().unwrap(), floor, "prefix[n] != floor");
                for c in 0..m.n {
                    for color in 0..u8::try_from(m.nc).unwrap() {
                        let w = whatif_floor(&band, &cands, &pfx, &sfx, c, color);
                        let mut f2 = band.frontier.clone().unwrap();
                        f2[c] = color;
                        let band2 = Band {
                            mini: band.mini,
                            r0: band.r0,
                            k: band.k,
                            frontier: Some(f2),
                            pool: band.pool.clone(),
                            forced: band.forced.clone(),
                        };
                        let full = *tropical_suffix(&band2)[0].iter().min().unwrap();
                        assert_eq!(
                            w, full,
                            "whatif != full (seed {seed} r0 {r0} c {c} color {color})"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn m0_bb_and_bandsaw_match_brute_min() {
        for seed in [11u64, 12, 13, 14] {
            let m = Mini::from_seed(5, 4, seed, &[]);
            let r0 = 3usize; // last 2 rows -> bandsaw splits 1+1
            let placement = canonical_prefix(&m, r0);
            let band = endgame_band(&m, &placement, r0);
            // brute min over generous bmax
            let brute = brute_exact(&band, 12);
            let bstar = brute
                .iter()
                .position(|&x| x > 0)
                .expect("some filling must exist") as u32;
            let bb = bb_min_break(&band, 16, u64::MAX, u64::MAX, None);
            assert!(!bb.capped);
            assert_eq!(bb.best, Some(bstar), "bb seed {seed}");
            let lb = relax_floor(&band, 4);
            assert!(lb <= bstar, "relax floor must lower-bound: seed {seed}");
            let saw = bandsaw(&band, lb, u64::MAX);
            assert!(!saw.capped);
            assert_eq!(saw.best, Some(bstar), "bandsaw seed {seed}");
        }
    }

    // 10×10 M0 checks: run in release (`cargo test -p
    // eternity2-bench-audit --release -- --ignored`); too slow in the
    // default debug suite.
    #[test]
    #[ignore]
    fn m0_canonical_solution_scores_zero() {
        let m = Mini::from_seed(10, 8, 101, &hint_cells(10));
        let n = m.n;
        let mut placement = vec![None; n * n];
        for pos in 0..n * n {
            placement[pos] = Some(m.solution[pos]);
        }
        assert_eq!(m.breaks_of(&placement), 0);
        // hints really are the canonical placements
        for &(pos, p, rt) in &m.hints {
            assert_eq!(m.solution[pos], (p, rt));
        }
        // endgame from the canonical prefix must complete at 0
        let prefix = canonical_prefix(&m, 6);
        let band = endgame_band(&m, &prefix, 6);
        let lb = relax_floor(&band, 4);
        assert_eq!(lb, 0);
        let saw = bandsaw(&band, lb, u64::MAX);
        assert!(!saw.capped);
        assert_eq!(saw.best, Some(0));
    }

    #[test]
    #[ignore]
    fn m0_bandsaw_matches_bb_on_canonical_10x10_perturbed() {
        // canonical prefix with 1-2 frontier columns corrupted:
        // nonzero min-break territory where exhaustive bb is still
        // tractable (a fully alien frontier puts b* high enough that
        // budget-bounded exhaustion explodes — measured 600 s+).
        for (seed, ncorrupt) in [(102u64, 1usize), (103, 2)] {
            let m = Mini::from_seed(10, 8, seed, &hint_cells(10));
            let prefix = canonical_prefix(&m, 6);
            let mut band = endgame_band(&m, &prefix, 6);
            let mut f = band.frontier.clone().unwrap();
            for j in 0..ncorrupt {
                let col = 3 + 3 * j;
                f[col] = if f[col] == 1 { 2 } else { 1 };
            }
            band.frontier = Some(f);
            let lb = relax_floor(&band, 8);
            let sfx = tropical_suffix(&band);
            assert_eq!(
                *sfx[0].iter().min().unwrap(),
                lb,
                "suffix floor must equal relax floor (seed {seed})"
            );
            // iterative-deepening bb (a slack ceiling explodes: 600 s+)
            let mut ceil = lb;
            let bb_best = loop {
                let bb = bb_min_break(&band, ceil, 600_000, u64::MAX, Some(&sfx));
                assert!(!bb.capped, "bb must exhaust (seed {seed} ceil {ceil})");
                if bb.best.is_some() {
                    break bb.best;
                }
                ceil += 1;
                assert!(ceil <= 64);
            };
            let saw = bandsaw(&band, lb, u64::MAX);
            assert!(!saw.capped);
            assert_eq!(saw.best, bb_best, "seed {seed} corrupt {ncorrupt}");
        }
    }
}
