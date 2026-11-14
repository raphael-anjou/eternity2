//! Copy of research/experiments/real-puzzle-profile/engine/src/bw.rs (Track 10
//! port of blackwood_bt, stage 4, one attempt, no border intersperse), with
//! switches that are all off by default (off = the Track 10 tree, gated):
//!   legal accounting (instrumentation only): hist_legal, nodes_legal
//!   legal_cap: stop after this many legal nodes (default u64::MAX)
//!   legal_break: refuse wrong-class / wrongly rotated pieces in the break path
//!   ring_break_off: also refuse a break where the candidate's side carries a
//!     ring colour (Blackwood's own rule)
//!   no_isolation: drop the port's rule that a break may not touch a cell
//!     that already carries a break (Blackwood's code has no such rule)
//!   stage 1 (A1): privileged quota ramps from q(160) to all privileged sides
//!     by depth 176; stage 2 (A2): A1 plus, from depth 161, the interior
//!     non-privileged colour with fewest sides left must be fully on the board
//!     by depth 200 (linear ramp from its count at depth 161)
//! Quirk kept from the original: in the no-break path a piece may show grey on
//! an interior side (it is then matched like a colour).

use crate::puzzle::{rotate, Puzzle, Rng};

const N: usize = 256;
const W: usize = 16;
const BORDER: u8 = 0;
const EMPTYP: u16 = u16::MAX;
/// Ring (frame) colours have 24 sides on the official set; interior ones 48 or 50.
const RING_SIDES: u16 = 24;

#[derive(Clone, Copy)]
struct RotatedPiece {
    top: u8,
    right: u8,
    bottom: u8,
    left: u8,
    heuristic_count: u8,
}

fn build_quota(scale: f32) -> [u16; 256] {
    let mut a = [0u16; 256];
    for i in 0..256 {
        let f = i as f32;
        let v: f32 = if i <= 16 {
            0.0
        } else if i <= 26 {
            (f - 16.0) * 2.8
        } else if i <= 56 {
            (f - 26.0) * 1.43333 + 28.0
        } else if i <= 76 {
            (f - 56.0) * 0.9 + 71.0
        } else if i <= 102 {
            (f - 76.0) * 0.6538 + 89.0
        } else if i <= 160 {
            (f - 102.0) / 4.4615 + 106.0
        } else {
            (160.0 - 102.0) / 4.4615 + 106.0
        };
        a[i] = (v * scale) as u16;
    }
    a
}

pub fn attempt_key(seed: u64, attempt: usize) -> u64 {
    let mut z = seed
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add((attempt as u64).wrapping_mul(0xD1B5_4A32_D192_ED03));
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

pub struct Solver {
    npieces: usize,
    quota: [u16; 256],
    order: Vec<usize>,
    border_mask: [[bool; 4]; N],
    hint_at: [Option<(u16, u8)>; N],
    use_quota: bool,
    unlock: Vec<usize>,
    max_breaks: u8,
    rotpieces: Vec<[RotatedPiece; 4]>,
    distinct_rots: Vec<u8>,
    ncolors: usize,
    cand_by_tl: Vec<Vec<(u16, u8)>>,
    cand_by_tl_pristine: Vec<Vec<(u16, u8)>>,
    board_piece: [u16; N],
    board_rot: [u8; N],
    used: Vec<bool>,
    priv_on_board: u16,
    breaks_used: u8,
    pub nodes: u64,
    node_cap: u64,
    pub best_score: u16,
    pub completions: u64,
    pub deepest: usize,
    pub breaks_at_deepest: u8,
    pub hist: Vec<u64>,
    checkpoints: Vec<u64>,
    next_cp: usize,
    pub snapshots: Vec<(u64, Vec<u64>)>,
    // added (all inert when off)
    illegal: u32,
    pub hist_legal: Vec<u64>,
    pub nodes_legal: u64,
    legal_cap: u64,
    legal_checkpoints: Vec<u64>,
    next_lcp: usize,
    pub legal_snapshots: Vec<(u64, u64, Vec<u64>, Vec<u64>)>,
    pub legal_break: bool,
    pub ring_break_off: bool,
    pub no_isolation: bool,
    stage: u8,
    color_total: Vec<u16>,
    color_on_board: Vec<u16>,
    privileged: Vec<bool>,
    c3: usize,
    c3_base: u16,
    pub c3_hist: Vec<u64>,
}

impl Solver {
    pub fn new(
        p: &Puzzle,
        priv_colors: &[u8],
        use_quota: bool,
        unlock: Vec<usize>,
        max_breaks: u8,
        node_cap: u64,
        checkpoints: Vec<u64>,
    ) -> Self {
        assert_eq!(p.n, W);
        let edges = &p.pieces;
        let npieces = edges.len();
        let mut privileged = [false; 256];
        for &c in priv_colors {
            privileged[c as usize] = true;
        }
        let mut border_mask = [[false; 4]; N];
        for pos in 0..N {
            let x = pos % W;
            let y = pos / W;
            border_mask[pos] = [y == 0, x == W - 1, y == W - 1, x == 0];
        }
        let mut hint_at: [Option<(u16, u8)>; N] = [None; N];
        for &(pos, pid, rot) in &p.hints {
            hint_at[pos] = Some((pid, rot));
        }
        let mut rotpieces = Vec::with_capacity(npieces);
        let mut distinct_rots = Vec::with_capacity(npieces);
        for pid in 0..npieces {
            let e = edges[pid];
            let mut arr = [RotatedPiece {
                top: 0,
                right: 0,
                bottom: 0,
                left: 0,
                heuristic_count: 0,
            }; 4];
            let mut seen: Vec<[u8; 4]> = Vec::new();
            for r in 0..4u8 {
                let re = rotate(e, r);
                let hc = re.iter().filter(|&&c| privileged[c as usize]).count() as u8;
                arr[r as usize] = RotatedPiece {
                    top: re[0],
                    right: re[1],
                    bottom: re[2],
                    left: re[3],
                    heuristic_count: hc,
                };
                if !seen.contains(&re) {
                    seen.push(re);
                }
            }
            distinct_rots.push(seen.len() as u8);
            rotpieces.push(arr);
        }
        let ncolors = edges
            .iter()
            .flat_map(|e| e.iter().copied())
            .max()
            .unwrap_or(0) as usize
            + 1;
        let mut color_total = vec![0u16; ncolors];
        for e in edges.iter() {
            for &c in e.iter() {
                color_total[c as usize] += 1;
            }
        }
        let mut cand_by_tl: Vec<Vec<(u16, u8)>> = vec![Vec::new(); ncolors * ncolors];
        for pid in 0..npieces {
            let ndr = distinct_rots[pid] as usize;
            for r in 0..ndr {
                let rp = rotpieces[pid][r];
                let key = (rp.top as usize) * ncolors + (rp.left as usize);
                cand_by_tl[key].push((pid as u16, r as u8));
            }
        }
        for list in cand_by_tl.iter_mut() {
            list.sort_by(|a, b| {
                let ha = rotpieces[a.0 as usize][a.1 as usize].heuristic_count;
                let hb = rotpieces[b.0 as usize][b.1 as usize].heuristic_count;
                hb.cmp(&ha)
            });
        }
        let order: Vec<usize> = (0..W)
            .flat_map(|y| (0..W).map(move |x| y * W + x))
            .collect();
        Self {
            npieces,
            quota: build_quota(1.0),
            order,
            border_mask,
            hint_at,
            use_quota,
            unlock,
            max_breaks,
            rotpieces,
            distinct_rots,
            ncolors,
            cand_by_tl_pristine: cand_by_tl.clone(),
            cand_by_tl,
            board_piece: [EMPTYP; N],
            board_rot: [0u8; N],
            used: vec![false; npieces],
            priv_on_board: 0,
            breaks_used: 0,
            nodes: 0,
            node_cap,
            best_score: 0,
            completions: 0,
            deepest: 0,
            breaks_at_deepest: 0,
            hist: vec![0; N + 1],
            checkpoints,
            next_cp: 0,
            snapshots: Vec::new(),
            illegal: 0,
            hist_legal: vec![0; N + 1],
            nodes_legal: 0,
            legal_cap: u64::MAX,
            legal_checkpoints: Vec::new(),
            next_lcp: 0,
            legal_snapshots: Vec::new(),
            legal_break: false,
            ring_break_off: false,
            no_isolation: false,
            stage: 0,
            color_total,
            color_on_board: vec![0; ncolors],
            privileged: privileged[..ncolors].to_vec(),
            c3: 0,
            c3_base: 0,
            c3_hist: vec![0; ncolors],
        }
    }

    /// Stop after `cap` legal nodes; snapshot both histograms at `cps` legal nodes.
    pub fn set_legal_cap(&mut self, cap: u64, cps: Vec<u64>) {
        self.legal_cap = cap;
        self.legal_checkpoints = cps;
    }

    /// Staging: 0 = published quota (A0), 1 = A1, 2 = A2.
    pub fn set_stage(&mut self, stage: u8) {
        self.stage = stage;
        if stage >= 1 {
            let total: u16 = (0..self.ncolors)
                .filter(|&c| self.privileged[c])
                .map(|c| self.color_total[c])
                .sum();
            let base = self.quota[160];
            for d in 161..256 {
                let k = (d.min(176) - 160) as u16;
                self.quota[d] = base + (total - base) * k / 16;
            }
        }
    }

    #[inline]
    fn stop(&self) -> bool {
        self.nodes >= self.node_cap || self.nodes_legal >= self.legal_cap
    }

    #[inline]
    fn wrong_class(&self, pos: usize, rp: &RotatedPiece) -> bool {
        let bm = self.border_mask[pos];
        (rp.top == BORDER) != bm[0]
            || (rp.right == BORDER) != bm[1]
            || (rp.bottom == BORDER) != bm[2]
            || (rp.left == BORDER) != bm[3]
    }

    /// Blackwood's rule (Util.cs, Get_Rotated_Pieces): a mismatch is not
    /// admitted when the candidate's own side there carries a ring colour.
    #[inline]
    fn breaks_on_ring(&self, rp: &RotatedPiece, cells: &[Option<usize>; 4]) -> bool {
        let sides = [rp.top, rp.right, rp.bottom, rp.left];
        (0..4).any(|k| {
            cells[k].is_some()
                && sides[k] != BORDER
                && self.color_total[sides[k] as usize] <= RING_SIDES
        })
    }

    #[inline]
    fn count_colors(&mut self, rp: &RotatedPiece, add: bool) {
        for c in [rp.top, rp.right, rp.bottom, rp.left] {
            if add {
                self.color_on_board[c as usize] += 1;
            } else {
                self.color_on_board[c as usize] -= 1;
            }
        }
    }

    /// A2: pick the third colour on entry to depth 161 (fewest sides left,
    /// ties to the lowest colour number; ring colours and privileged colours
    /// excluded) and record its count on the board as the ramp base.
    fn pick_c3(&mut self) {
        let mut best = usize::MAX;
        let mut best_left = u16::MAX;
        for c in 1..self.ncolors {
            if self.privileged[c] || self.color_total[c] <= RING_SIDES {
                continue;
            }
            let left = self.color_total[c] - self.color_on_board[c];
            if left < best_left {
                best_left = left;
                best = c;
            }
        }
        self.c3 = best;
        self.c3_base = self.color_on_board[best];
        self.c3_hist[best] += 1;
    }

    #[inline]
    fn c3_required(&self, depth: usize) -> u16 {
        let t = self.color_total[self.c3];
        let k = (depth.min(200) - 160) as u16;
        self.c3_base + (t - self.c3_base) * k / 40
    }

    fn reshuffle_candidates(&mut self, rng: &mut Rng) {
        self.cand_by_tl.clone_from(&self.cand_by_tl_pristine);
        for list in self.cand_by_tl.iter_mut() {
            rng.shuffle(list);
        }
        let rotpieces = &self.rotpieces;
        for list in self.cand_by_tl.iter_mut() {
            list.sort_by(|a, b| {
                let ha = rotpieces[a.0 as usize][a.1 as usize].heuristic_count;
                let hb = rotpieces[b.0 as usize][b.1 as usize].heuristic_count;
                hb.cmp(&ha)
            });
        }
    }

    fn reset(&mut self) {
        self.board_piece = [EMPTYP; N];
        self.board_rot = [0u8; N];
        for u in self.used.iter_mut() {
            *u = false;
        }
        self.priv_on_board = 0;
        self.breaks_used = 0;
        self.nodes = 0;
        for c in self.color_on_board.iter_mut() {
            *c = 0;
        }
        for pos in 0..N {
            if let Some((pid, rot)) = self.hint_at[pos] {
                self.board_piece[pos] = pid;
                self.board_rot[pos] = rot;
                self.used[pid as usize] = true;
                let rp = self.rotpieces[pid as usize][rot as usize];
                self.priv_on_board += u16::from(rp.heuristic_count);
                self.count_colors(&rp, true);
            }
        }
    }

    #[inline]
    fn breaks_allowed_at(&self, depth: usize) -> u8 {
        self.unlock.iter().filter(|&&d| d <= depth).count() as u8
    }

    #[inline]
    fn oriented(&self, pos: usize) -> RotatedPiece {
        let pid = self.board_piece[pos];
        let rot = self.board_rot[pos];
        self.rotpieces[pid as usize][rot as usize]
    }

    #[inline]
    fn neighbor_colors(&self, pos: usize) -> (Option<u8>, Option<u8>, Option<u8>, Option<u8>) {
        let x = pos % W;
        let y = pos / W;
        let above = if y > 0 && self.board_piece[pos - W] != EMPTYP {
            Some(self.oriented(pos - W).bottom)
        } else {
            None
        };
        let left = if x > 0 && self.board_piece[pos - 1] != EMPTYP {
            Some(self.oriented(pos - 1).right)
        } else {
            None
        };
        let right = if x < W - 1 && self.board_piece[pos + 1] != EMPTYP {
            Some(self.oriented(pos + 1).left)
        } else {
            None
        };
        let below = if y < W - 1 && self.board_piece[pos + W] != EMPTYP {
            Some(self.oriented(pos + W).top)
        } else {
            None
        };
        (above, left, right, below)
    }

    #[inline]
    fn eval_candidate(
        &self,
        pos: usize,
        rp: &RotatedPiece,
        break_count_at: &[u8; N],
    ) -> Option<(u8, [Option<usize>; 4])> {
        let x = pos % W;
        let y = pos / W;
        let mut new_breaks = 0u8;
        let mut broke_side_cell: [Option<usize>; 4] = [None; 4];
        let bm = self.border_mask[pos];
        if y > 0 {
            let np = pos - W;
            if self.board_piece[np] != EMPTYP && self.oriented(np).bottom != rp.top {
                new_breaks += 1;
                broke_side_cell[0] = Some(np);
            }
        }
        if bm[0] && rp.top != BORDER {
            new_breaks += 1;
            broke_side_cell[0] = Some(usize::MAX);
        }
        if x < W - 1 {
            let np = pos + 1;
            if self.board_piece[np] != EMPTYP && self.oriented(np).left != rp.right {
                new_breaks += 1;
                broke_side_cell[1] = Some(np);
            }
        }
        if bm[1] && rp.right != BORDER {
            new_breaks += 1;
            broke_side_cell[1] = Some(usize::MAX);
        }
        if y < W - 1 {
            let np = pos + W;
            if self.board_piece[np] != EMPTYP && self.oriented(np).top != rp.bottom {
                new_breaks += 1;
                broke_side_cell[2] = Some(np);
            }
        }
        if bm[2] && rp.bottom != BORDER {
            new_breaks += 1;
            broke_side_cell[2] = Some(usize::MAX);
        }
        if x > 0 {
            let np = pos - 1;
            if self.board_piece[np] != EMPTYP && self.oriented(np).right != rp.left {
                new_breaks += 1;
                broke_side_cell[3] = Some(np);
            }
        }
        if bm[3] && rp.left != BORDER {
            new_breaks += 1;
            broke_side_cell[3] = Some(usize::MAX);
        }
        if new_breaks > 1 {
            return None;
        }
        for s in 0..4 {
            if self.no_isolation {
                break;
            }
            if let Some(nc) = broke_side_cell[s] {
                if nc != usize::MAX && break_count_at[nc] > 0 {
                    return None;
                }
            }
        }
        Some((new_breaks, broke_side_cell))
    }

    fn search(&mut self, depth: usize, break_count_at: &mut [u8; N]) {
        if self.stop() {
            return;
        }
        if depth == N {
            let score = 480u16.saturating_sub(u16::from(self.breaks_used));
            self.completions += 1;
            if score > self.best_score {
                self.best_score = score;
            }
            return;
        }
        if depth > self.deepest {
            self.deepest = depth;
            self.breaks_at_deepest = self.breaks_used;
        }
        let pos = self.order[depth];
        if self.hint_at[pos].is_some() {
            self.search(depth + 1, break_count_at);
            return;
        }

        self.nodes += 1;
        self.hist[depth] += 1;
        if self.next_cp < self.checkpoints.len() && self.nodes == self.checkpoints[self.next_cp] {
            self.snapshots.push((self.nodes, self.hist.clone()));
            self.next_cp += 1;
        }
        if self.illegal == 0 {
            self.hist_legal[depth] += 1;
            self.nodes_legal += 1;
            if self.next_lcp < self.legal_checkpoints.len()
                && self.nodes_legal == self.legal_checkpoints[self.next_lcp]
            {
                self.legal_snapshots.push((
                    self.nodes_legal,
                    self.nodes,
                    self.hist.clone(),
                    self.hist_legal.clone(),
                ));
                self.next_lcp += 1;
            }
        }

        if self.use_quota && self.priv_on_board < self.quota[depth] {
            return;
        }
        if self.stage >= 2 && depth > 160 {
            if depth == 161 {
                self.pick_c3();
            }
            if self.color_on_board[self.c3] < self.c3_required(depth) {
                return;
            }
        }
        let budget = self.breaks_allowed_at(depth).min(self.max_breaks);
        let (top_req, left_req, right_req, below_req) = self.neighbor_colors(pos);
        let bm = self.border_mask[pos];

        if budget == 0 {
            let treq = match top_req {
                Some(c) => c,
                None => {
                    if bm[0] {
                        BORDER
                    } else {
                        self.search(depth + 1, break_count_at);
                        return;
                    }
                }
            };
            let lreq = match left_req {
                Some(c) => c,
                None => {
                    if bm[3] {
                        BORDER
                    } else {
                        self.search(depth + 1, break_count_at);
                        return;
                    }
                }
            };
            let key = (treq as usize) * self.ncolors + (lreq as usize);
            let list_len = self.cand_by_tl[key].len();
            for idx in 0..list_len {
                if self.stop() {
                    return;
                }
                let (pid16, r) = self.cand_by_tl[key][idx];
                let pid = pid16 as usize;
                if self.used[pid] {
                    continue;
                }
                let rp = self.rotpieces[pid][r as usize];
                if let Some(rc) = right_req {
                    if rp.right != rc {
                        continue;
                    }
                } else if bm[1] && rp.right != BORDER {
                    continue;
                }
                if let Some(bc) = below_req {
                    if rp.bottom != bc {
                        continue;
                    }
                } else if bm[2] && rp.bottom != BORDER {
                    continue;
                }
                self.board_piece[pos] = pid16;
                self.board_rot[pos] = r;
                self.used[pid] = true;
                self.priv_on_board += u16::from(rp.heuristic_count);
                let ill = u32::from(self.wrong_class(pos, &rp));
                self.illegal += ill;
                if self.stage >= 2 {
                    self.count_colors(&rp, true);
                }
                self.search(depth + 1, break_count_at);
                if self.stage >= 2 {
                    self.count_colors(&rp, false);
                }
                self.illegal -= ill;
                self.priv_on_board -= u16::from(rp.heuristic_count);
                self.used[pid] = false;
                self.board_piece[pos] = EMPTYP;
            }
            return;
        }

        let mut cands: Vec<(u16, u8, u8, [Option<usize>; 4])> = Vec::with_capacity(64);
        for pid in 0..self.npieces {
            if self.used[pid] {
                continue;
            }
            let ndr = self.distinct_rots[pid] as usize;
            for r in 0..ndr {
                let rp = self.rotpieces[pid][r];
                if self.legal_break && self.wrong_class(pos, &rp) {
                    continue;
                }
                if let Some((nb, cells)) = self.eval_candidate(pos, &rp, break_count_at) {
                    if self.breaks_used + nb > budget {
                        continue;
                    }
                    if self.ring_break_off && nb > 0 && self.breaks_on_ring(&rp, &cells) {
                        continue;
                    }
                    cands.push((pid as u16, r as u8, nb, cells));
                }
            }
        }
        if cands.is_empty() {
            return;
        }
        cands.sort_by(|a, b| {
            let ha = self.rotpieces[a.0 as usize][a.1 as usize].heuristic_count;
            let hb = self.rotpieces[b.0 as usize][b.1 as usize].heuristic_count;
            hb.cmp(&ha).then(a.2.cmp(&b.2))
        });
        for i in 0..cands.len() {
            if self.stop() {
                return;
            }
            let (pid16, r, nb, cells) = cands[i];
            let pid = pid16 as usize;
            let rp = self.rotpieces[pid][r as usize];
            self.board_piece[pos] = pid16;
            self.board_rot[pos] = r;
            self.used[pid] = true;
            self.priv_on_board += u16::from(rp.heuristic_count);
            self.breaks_used += nb;
            break_count_at[pos] += nb;
            let mut touched: [Option<usize>; 4] = [None; 4];
            for s in 0..4 {
                if let Some(nc) = cells[s] {
                    if nc != usize::MAX {
                        break_count_at[nc] += 1;
                        touched[s] = Some(nc);
                    }
                }
            }
            let ill = u32::from(self.wrong_class(pos, &rp));
            self.illegal += ill;
            if self.stage >= 2 {
                self.count_colors(&rp, true);
            }
            self.search(depth + 1, break_count_at);
            if self.stage >= 2 {
                self.count_colors(&rp, false);
            }
            self.illegal -= ill;
            for s in 0..4 {
                if let Some(nc) = touched[s] {
                    break_count_at[nc] -= 1;
                }
            }
            break_count_at[pos] -= nb;
            self.breaks_used -= nb;
            self.priv_on_board -= u16::from(rp.heuristic_count);
            self.used[pid] = false;
            self.board_piece[pos] = EMPTYP;
        }
    }

    /// One attempt as blackwood_bt's run_attempt_scored(seed, attempt 0, shuffle).
    pub fn run(&mut self, seed: u64) {
        let mut rng = Rng::new(attempt_key(seed, 0));
        self.reset();
        self.reshuffle_candidates(&mut rng);
        let mut bca = [0u8; N];
        self.search(0, &mut bca);
    }
}
