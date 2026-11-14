//! M1: a port of research/experiments/single-core-benchmark/engine/crates/
//! bench-audit/src/bin/blackwood_bt.rs (stage 4, one attempt, no border
//! intersperse). The search is unchanged; added: a node counter per depth,
//! checkpoints of that histogram, a configurable unlock list, the breaks used
//! at the deepest depth. Quirk kept from the original: in the no-break path a
//! piece may show grey on an interior side (it is then matched like a colour).

use crate::puzzle::{rotate, Puzzle, Rng};

const N: usize = 256;
const W: usize = 16;
const BORDER: u8 = 0;
const EMPTYP: u16 = u16::MAX;

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
    pub probe_depth: usize,
    pub probe_max: usize,
    pub probe_rows: Vec<String>,
    pub probe_seen: u64,
    sub_deepest: usize,
    sub_breaks: u8,
    illegal: u32,
    pub hist_legal: Vec<u64>,
    pub done_legal: Vec<u64>,
    pub done_all: Vec<u64>,
    nodes_legal: u64,
    sub_deepest_legal: usize,
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
            probe_depth: usize::MAX,
            probe_max: 0,
            probe_rows: Vec::new(),
            probe_seen: 0,
            sub_deepest: 0,
            sub_breaks: 0,
            illegal: 0,
            hist_legal: vec![0; N + 1],
            done_legal: vec![0; 64],
            done_all: vec![0; 64],
            nodes_legal: 0,
            sub_deepest_legal: 0,
        }
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
        for pos in 0..N {
            if let Some((pid, rot)) = self.hint_at[pos] {
                self.board_piece[pos] = pid;
                self.board_rot[pos] = rot;
                self.used[pid as usize] = true;
                let rp = self.rotpieces[pid as usize][rot as usize];
                self.priv_on_board += u16::from(rp.heuristic_count);
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
            if let Some(nc) = broke_side_cell[s] {
                if nc != usize::MAX && break_count_at[nc] > 0 {
                    return None;
                }
            }
        }
        Some((new_breaks, broke_side_cell))
    }

    fn census(&self, depth: usize) -> String {
        // sides per colour among unused pieces; colours on the open frontier
        // (the bottom side of the last placed cell of each column, and the
        // right side of the last placed cell).
        let mut pool = vec![0u32; self.ncolors];
        for pid in 0..self.npieces {
            if !self.used[pid] {
                let rp = self.rotpieces[pid][0];
                for c in [rp.top, rp.right, rp.bottom, rp.left] {
                    pool[c as usize] += 1;
                }
            }
        }
        let mut fr = vec![0u32; self.ncolors];
        // cells not yet placed in scan order (clue cells are placed from the start)
        for d in depth..N {
            let pos = self.order[d];
            if self.board_piece[pos] != EMPTYP {
                continue;
            }
            let (a, l, r, b) = self.neighbor_colors(pos);
            for c in [a, l, r, b].into_iter().flatten() {
                fr[c as usize] += 1;
            }
        }
        let f = |v: &Vec<u32>| {
            v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")
        };
        format!("\"pool\":[{}],\"front\":[{}]", f(&pool), f(&fr))
    }

    fn search(&mut self, depth: usize, break_count_at: &mut [u8; N]) {
        if depth == self.probe_depth && self.nodes < self.node_cap {
            let n0 = self.nodes;
            let (sd, sb) = (self.sub_deepest, self.sub_breaks);
            self.sub_deepest = depth;
            self.sub_breaks = self.breaks_used;
            let cen = if self.probe_rows.len() < self.probe_max {
                Some(self.census(depth))
            } else {
                None
            };
            let c0 = self.completions;
            let nl0 = self.nodes_legal;
            self.sub_deepest_legal = 0;
            self.search_inner(depth, break_count_at);
            self.probe_seen += 1;
            if let Some(cen) = cen {
                if self.nodes < self.node_cap {
                    self.probe_rows.push(format!(
                        "{{\"at\":{},\"sub\":{},\"deep\":{},\"brk\":{},\"priv\":{},\"done\":{},\"subl\":{},\"deepl\":{},{}}}",
                        n0,
                        self.nodes - n0,
                        self.sub_deepest,
                        self.sub_breaks,
                        self.priv_on_board,
                        self.completions - c0,
                        self.nodes_legal - nl0,
                        self.sub_deepest_legal,
                        cen
                    ));
                }
            }
            self.sub_deepest = sd.max(self.sub_deepest);
            if self.sub_deepest == sd {
                self.sub_breaks = sb;
            }
            return;
        }
        self.search_inner(depth, break_count_at);
    }

    fn search_inner(&mut self, depth: usize, break_count_at: &mut [u8; N]) {
        if self.nodes >= self.node_cap {
            return;
        }
        if depth > self.sub_deepest {
            self.sub_deepest = depth;
            self.sub_breaks = self.breaks_used;
        }
        if depth == N {
            let score = 480u16.saturating_sub(u16::from(self.breaks_used));
            self.completions += 1;
            self.done_all[self.breaks_used as usize] += 1;
            if self.illegal == 0 {
                self.done_legal[self.breaks_used as usize] += 1;
            }
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
        if self.illegal == 0 {
            self.hist_legal[depth] += 1;
            self.nodes_legal += 1;
            if depth > self.sub_deepest_legal {
                self.sub_deepest_legal = depth;
            }
        }
        if self.next_cp < self.checkpoints.len() && self.nodes == self.checkpoints[self.next_cp] {
            self.snapshots.push((self.nodes, self.hist.clone()));
            self.next_cp += 1;
        }

        if self.use_quota && self.priv_on_board < self.quota[depth] {
            return;
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
                if self.nodes >= self.node_cap {
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
                let ill = u32::from((rp.top == BORDER) != bm[0] || (rp.right == BORDER) != bm[1] || (rp.bottom == BORDER) != bm[2] || (rp.left == BORDER) != bm[3]);
                self.illegal += ill;
                self.search(depth + 1, break_count_at);
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
                if let Some((nb, cells)) = self.eval_candidate(pos, &rp, break_count_at) {
                    if self.breaks_used + nb > budget {
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
            if self.nodes >= self.node_cap {
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
            let ill = u32::from((rp.top == BORDER) != bm[0] || (rp.right == BORDER) != bm[1] || (rp.bottom == BORDER) != bm[2] || (rp.left == BORDER) != bm[3]);
            self.illegal += ill;
            self.search(depth + 1, break_count_at);
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

// ---------------------------------------------------------------------------
// M1b (amendment A2): the same tree, estimated by population SMC instead of
// walked depth first. A particle is the search state on entry to a depth.
// The child function below mirrors `search` line by line: a node is an entry
// at a non-clue depth (counted before the quota test), a quota failure has no
// child, clue cells pass through.
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct BState {
    bp: [u16; N],
    br: [u8; N],
    used: [u64; 4],
    priv_on_board: u16,
    breaks_used: u8,
    bca: [u8; N],
}

type Cand = (u16, u8, u8, [Option<usize>; 4]);

impl Solver {
    pub fn root_state(&mut self, seed: u64) -> BState {
        let mut rng = Rng::new(attempt_key(seed, 0));
        self.reset();
        self.reshuffle_candidates(&mut rng);
        let mut used = [0u64; 4];
        for (pid, &u) in self.used.iter().enumerate() {
            if u {
                used[pid / 64] |= 1 << (pid % 64);
            }
        }
        BState {
            bp: self.board_piece,
            br: self.board_rot,
            used,
            priv_on_board: self.priv_on_board,
            breaks_used: 0,
            bca: [0; N],
        }
    }

    pub fn is_hint_depth(&self, depth: usize) -> bool {
        self.hint_at[self.order[depth]].is_some()
    }

    fn load(&mut self, st: &BState) {
        self.board_piece = st.bp;
        self.board_rot = st.br;
        for pid in 0..self.npieces {
            self.used[pid] = st.used[pid / 64] >> (pid % 64) & 1 == 1;
        }
        self.priv_on_board = st.priv_on_board;
        self.breaks_used = st.breaks_used;
    }

    /// Candidates of `search` at a non-clue depth for state `st`, in its order.
    pub fn cands(&mut self, depth: usize, st: &BState) -> Vec<Cand> {
        self.load(st);
        let mut out = Vec::new();
        if self.use_quota && self.priv_on_board < self.quota[depth] {
            return out;
        }
        let pos = self.order[depth];
        let budget = self.breaks_allowed_at(depth).min(self.max_breaks);
        let (top_req, left_req, right_req, below_req) = self.neighbor_colors(pos);
        let bm = self.border_mask[pos];
        if budget == 0 {
            let treq = top_req.unwrap_or(BORDER);
            let lreq = left_req.unwrap_or(BORDER);
            // in a top-left row scan top/left are always known or the border
            assert!(top_req.is_some() || bm[0]);
            assert!(left_req.is_some() || bm[3]);
            let key = (treq as usize) * self.ncolors + (lreq as usize);
            for &(pid16, r) in &self.cand_by_tl[key] {
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
                out.push((pid16, r, 0, [None; 4]));
            }
            return out;
        }
        for pid in 0..self.npieces {
            if self.used[pid] {
                continue;
            }
            for r in 0..self.distinct_rots[pid] as usize {
                let rp = self.rotpieces[pid][r];
                if let Some((nb, cells)) = self.eval_candidate(pos, &rp, &st.bca) {
                    if self.breaks_used + nb <= budget {
                        out.push((pid as u16, r as u8, nb, cells));
                    }
                }
            }
        }
        out
    }

    pub fn child(&self, depth: usize, st: &BState, c: &Cand) -> BState {
        let (pid16, r, nb, cells) = *c;
        let pos = self.order[depth];
        let mut s = st.clone();
        s.bp[pos] = pid16;
        s.br[pos] = r;
        s.used[pid16 as usize / 64] |= 1 << (pid16 % 64);
        s.priv_on_board += u16::from(self.rotpieces[pid16 as usize][r as usize].heuristic_count);
        s.breaks_used += nb;
        s.bca[pos] += nb;
        for nc in cells.iter().flatten() {
            if *nc != usize::MAX {
                s.bca[*nc] += 1;
            }
        }
        s
    }

    /// One SMC repetition from `root` at depth `d0`. Returns est[d] = estimated
    /// number of entries at depth d (est[256] = completions), with clue depths
    /// carrying the pass-through value; and the max breaks seen at the last
    /// populated depth.
    pub fn smc(&mut self, root: &BState, d0: usize, np: usize, seed: u64) -> Vec<f64> {
        let mut rng = Rng::new(seed);
        let mut est = vec![0.0f64; N + 1];
        let mut pop = vec![root.clone()];
        let mut w = 1.0f64;
        est[d0] = 1.0;
        for depth in d0..N {
            if self.is_hint_depth(depth) {
                est[depth + 1] = w;
                continue;
            }
            let mut counts = Vec::with_capacity(pop.len());
            let mut e = 0u64;
            for st in &pop {
                let c = self.cands(depth, st).len();
                counts.push(c);
                e += c as u64;
            }
            w = w * e as f64 / pop.len() as f64;
            est[depth + 1] = w;
            if e == 0 {
                break;
            }
            let chosen: Vec<u64> = if e as usize <= np {
                (0..e).collect()
            } else {
                let mut set = std::collections::HashSet::with_capacity(np * 2);
                let mut out = Vec::with_capacity(np);
                for j in (e - np as u64)..e {
                    let r = rng.next_u64() % (j + 1);
                    let v = if set.contains(&r) { j } else { r };
                    set.insert(v);
                    out.push(v);
                }
                out.sort_unstable();
                out
            };
            let mut next = Vec::with_capacity(chosen.len());
            let (mut ci, mut base) = (0usize, 0u64);
            for (j, st) in pop.iter().enumerate() {
                let c = counts[j] as u64;
                if ci < chosen.len() && chosen[ci] < base + c {
                    let cs = self.cands(depth, st);
                    while ci < chosen.len() && chosen[ci] < base + c {
                        next.push(self.child(depth, st, &cs[(chosen[ci] - base) as usize]));
                        ci += 1;
                    }
                }
                base += c;
            }
            pop = next;
        }
        est
    }

    /// Exact entries per depth of the subtree under `st` at depth d0 (the DFS
    /// node histogram restricted to that subtree), with a node cap.
    pub fn exact_from(&mut self, st: &BState, d0: usize, cap: u64) -> (Vec<u64>, bool) {
        let mut h = vec![0u64; N + 1];
        let mut nodes = 0u64;
        let ok = self.ex_rec(st, d0, &mut h, &mut nodes, cap);
        (h, ok)
    }

    fn ex_rec(
        &mut self,
        st: &BState,
        depth: usize,
        h: &mut [u64],
        nodes: &mut u64,
        cap: u64,
    ) -> bool {
        h[depth] += 1;
        if depth == N {
            return true;
        }
        if self.is_hint_depth(depth) {
            h[depth] -= 1;
            return self.ex_rec(st, depth + 1, h, nodes, cap);
        }
        *nodes += 1;
        if *nodes > cap {
            return false;
        }
        let cs = self.cands(depth, st);
        for c in &cs {
            let s2 = self.child(depth, st, c);
            if !self.ex_rec(&s2, depth + 1, h, nodes, cap) {
                return false;
            }
        }
        true
    }

    /// States on entry to depth d0 met by a depth-first walk of the tree in
    /// the engine's own order, keeping every `stride`-th one, at most `k`.
    pub fn capture(
        &mut self,
        root: &BState,
        d0: usize,
        k: usize,
        stride: u64,
        cap: u64,
    ) -> Vec<BState> {
        let mut out = Vec::new();
        let mut seen = 0u64;
        let mut nodes = 0u64;
        self.cap_rec(root, 0, d0, k, stride, cap, &mut out, &mut seen, &mut nodes);
        out
    }

    #[allow(clippy::too_many_arguments)]
    fn cap_rec(
        &mut self,
        st: &BState,
        depth: usize,
        d0: usize,
        k: usize,
        stride: u64,
        cap: u64,
        out: &mut Vec<BState>,
        seen: &mut u64,
        nodes: &mut u64,
    ) {
        if out.len() >= k || *nodes >= cap {
            return;
        }
        if depth == d0 {
            if *seen % stride == 0 {
                out.push(st.clone());
            }
            *seen += 1;
            return;
        }
        if self.is_hint_depth(depth) {
            return self.cap_rec(st, depth + 1, d0, k, stride, cap, out, seen, nodes);
        }
        *nodes += 1;
        let cs = self.cands(depth, st);
        for c in &cs {
            let s2 = self.child(depth, st, c);
            self.cap_rec(&s2, depth + 1, d0, k, stride, cap, out, seen, nodes);
            if out.len() >= k || *nodes >= cap {
                return;
            }
        }
    }
}
