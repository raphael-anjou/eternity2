// Phase-1 "scaffold" backtrack. Operates on the full puzzle frame
// (corners + edges = 60 cells) plus an inner subset of 186 pieces
// chosen by the outer SA (phase-0).
//
// Variable order:
//   1. Corners (positions 0, w-1, (h-1)*w, (h-1)*w + (w-1))
//   2. Edge cells (perimeter, non-corner) in clockwise order
//   3. Inner cells in row-major order, BUT *constrained domains* on
//      cells where the SA's "worst performers" are placed first.
//
// Value order: lowest piece-id first (deterministic). With a single
// rotation pass per piece.
//
// Domain: for each cell, a vector of `(piece_id, rotation)` such that:
//   - frame cells: only corner/edge pieces of matching kind
//   - inner cells: only pieces in `inner_set`
//   - all placements satisfy local edge-color matches with placed
//     neighbours.
//
// Termination: backtrack DFS until all cells filled, all 186 inner
// pieces placed, or time budget reached. Track `best_depth` =
// max placements achieved, and `best_edges` = max matched-edges count
// over all visited prefixes.
//
// Crucially: this is NOT meant to solve to 480 — we're missing 10
// inner pieces. Phase-2 fills those 10 cells separately.

use std::time::{Duration, Instant};

use eternity2_core::{Color, PieceId, Puzzle, Rotation, BORDER};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct ScaffoldConfig {
    /// Inner piece IDs that the SA decided to commit to.
    pub inner_set: Vec<PieceId>,
    /// Inner piece IDs deferred to phase-2 (these are NOT placeable).
    pub deferred: Vec<PieceId>,
    /// Worst performers in the inner_set — preferred placement order.
    /// Should be a subset of inner_set, length ~10-20.
    pub worst_performers: Vec<PieceId>,
    /// Time budget for scaffold phase-1 search.
    pub time_budget: Duration,
    /// Max nodes to visit (0 = unlimited).
    pub node_budget: u64,
    /// Random seed for tiebreaks (deterministic when 0 ⇒ no randomness).
    pub seed: u64,
}

impl Default for ScaffoldConfig {
    fn default() -> Self {
        Self {
            inner_set: Vec::new(),
            deferred: Vec::new(),
            worst_performers: Vec::new(),
            time_budget: Duration::from_secs(30),
            node_budget: 0,
            seed: 0,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct ScaffoldResult {
    pub nodes: u64,
    pub elapsed_s: f64,
    pub best_depth: u32,
    pub best_edges: u32,
    pub best_placement: Vec<Option<(PieceId, u8)>>,
    pub timed_out: bool,
}

/// Build the cell-visit order. Corners first, then edges, then inner
/// cells with worst_performers preferred. For cells of the same kind,
/// chess-spiral order from center (matching solver-engine's chess
/// variable order) is used for inner cells to keep neighbors close.
fn build_path(puzzle: &Puzzle, worst_performers_first: bool) -> Vec<u32> {
    let n_pos = puzzle.cell_count() as usize;
    let mut path: Vec<u32> = Vec::with_capacity(n_pos);
    let mut seen = vec![false; n_pos];
    let w = puzzle.width;
    let h = puzzle.height;

    // 1. Corners
    let corners: [(u32, u32); 4] = [(0, 0), (w - 1, 0), (0, h - 1), (w - 1, h - 1)];
    for (x, y) in corners {
        let p = y * w + x;
        if !seen[p as usize] {
            path.push(p);
            seen[p as usize] = true;
        }
    }

    // 2. Border non-corner cells, row-major.
    for pos in 0..puzzle.cell_count() {
        if seen[pos as usize] { continue; }
        let mask = puzzle.border_mask(pos);
        if mask.iter().any(|m| *m) {
            path.push(pos);
            seen[pos as usize] = true;
        }
    }

    // 3. Inner cells, spiral-from-center (Chebyshev distance).
    let cx = (w - 1) as f32 / 2.0;
    let cy = (h - 1) as f32 / 2.0;
    let mut inner: Vec<(u32, u32)> = Vec::new();
    for pos in 0..puzzle.cell_count() {
        if seen[pos as usize] { continue; }
        let (x, y) = puzzle.xy(pos);
        let dx = (x as f32 - cx).abs();
        let dy = (y as f32 - cy).abs();
        let metric = (dx.max(dy) * 1000.0) as u32;
        inner.push((pos, metric * 10_000 + pos));
    }
    inner.sort_by_key(|(_, m)| *m);
    for (pos, _) in inner {
        path.push(pos);
    }

    // If worst_performers_first is true, the *value order* (not path)
    // will prefer worst-performer pieces; the path stays geometric.
    let _ = worst_performers_first;
    path
}

/// For each cell, the list of (piece_id, rotation) that match its
/// kind (corner/edge/inner) AND, for inner cells, only pieces in
/// `inner_set`.
fn build_domains(
    puzzle: &Puzzle,
    inner_set_mask: &[bool],
) -> Vec<Vec<(PieceId, u8)>> {
    let n_pos = puzzle.cell_count() as usize;
    let mut domains: Vec<Vec<(PieceId, u8)>> = vec![Vec::new(); n_pos];
    for pos in 0..puzzle.cell_count() {
        let mask = puzzle.border_mask(pos);
        let [on_top, on_right, on_bot, on_left] = mask;
        let is_corner_cell = mask.iter().filter(|m| **m).count() == 2;
        let is_edge_cell = mask.iter().filter(|m| **m).count() == 1;
        let is_inner_cell = !is_corner_cell && !is_edge_cell;
        for piece in puzzle.pieces() {
            // Kind match.
            if is_corner_cell && !piece.is_corner() { continue; }
            if is_edge_cell && !piece.is_edge() { continue; }
            if is_inner_cell {
                if !piece.is_inner() { continue; }
                if !inner_set_mask.get(piece.id as usize).copied().unwrap_or(false) {
                    continue;
                }
            }
            for r in 0u8..4 {
                let rot = Rotation::from_u8(r).unwrap();
                let e = piece.edges.rotated(rot);
                let [t, ri, b, l] = e.as_array();
                if on_top != (t == BORDER) { continue; }
                if on_right != (ri == BORDER) { continue; }
                if on_bot != (b == BORDER) { continue; }
                if on_left != (l == BORDER) { continue; }
                domains[pos as usize].push((piece.id, r));
            }
        }
    }
    domains
}

#[inline]
fn neighbors_xy(puzzle: &Puzzle, pos: u32) -> [Option<u32>; 4] {
    let w = puzzle.width;
    let h = puzzle.height;
    let (x, y) = puzzle.xy(pos);
    [
        if y > 0 { Some((y - 1) * w + x) } else { None },
        if x + 1 < w { Some(y * w + (x + 1)) } else { None },
        if y + 1 < h { Some((y + 1) * w + x) } else { None },
        if x > 0 { Some(y * w + (x - 1)) } else { None },
    ]
}

pub fn run_scaffold(puzzle: &Puzzle, cfg: &ScaffoldConfig) -> ScaffoldResult {
    let n_pos = puzzle.cell_count() as usize;
    let max_pid = puzzle.pieces().iter().map(|p| p.id as usize).max().unwrap_or(0) + 1;

    let mut inner_set_mask = vec![false; max_pid];
    for &pid in &cfg.inner_set {
        if (pid as usize) < inner_set_mask.len() {
            inner_set_mask[pid as usize] = true;
        }
    }

    let path = build_path(puzzle, !cfg.worst_performers.is_empty());
    let domains = build_domains(puzzle, &inner_set_mask);

    // Value order weights: prefer worst-performers first for inner cells.
    let mut worst_mask = vec![false; max_pid];
    for &pid in &cfg.worst_performers {
        if (pid as usize) < worst_mask.len() {
            worst_mask[pid as usize] = true;
        }
    }

    // Pre-sort each cell's domain so that worst-performers come first;
    // ties broken by piece-id then rotation. (Stable, deterministic.)
    let mut domains_sorted: Vec<Vec<(PieceId, u8)>> = domains
        .into_iter()
        .map(|mut v| {
            v.sort_by_key(|(pid, rot)| {
                let pri = if worst_mask.get(*pid as usize).copied().unwrap_or(false) { 0u8 } else { 1u8 };
                (pri, *pid, *rot)
            });
            v
        })
        .collect();

    // Cached rotated-edges per (piece_id, rotation) for fast edge lookup.
    let mut rotated_edges: Vec<[Color; 4]> = vec![[0; 4]; max_pid * 4];
    for piece in puzzle.pieces() {
        for r in 0u8..4 {
            let rot = Rotation::from_u8(r).unwrap();
            let e = piece.edges.rotated(rot);
            rotated_edges[piece.id as usize * 4 + r as usize] = e.as_array();
        }
    }

    let neighbors: Vec<[Option<u32>; 4]> = (0..puzzle.cell_count()).map(|p| neighbors_xy(puzzle, p)).collect();

    let mut placed: Vec<Option<(PieceId, u8)>> = vec![None; n_pos];
    let mut placed_edges: Vec<[Color; 4]> = vec![[255; 4]; n_pos];
    let mut used = vec![false; max_pid];

    let mut state = ScaffoldRun {
        path: &path,
        domains: &mut domains_sorted,
        rotated_edges: &rotated_edges,
        neighbors: &neighbors,
        placed: &mut placed,
        placed_edges: &mut placed_edges,
        used: &mut used,
        cfg,
        start: Instant::now(),
        nodes: 0,
        best_depth: 0,
        best_edges: 0,
        best_placement: vec![None; n_pos],
        timed_out: false,
    };
    state.dfs(0);

    ScaffoldResult {
        nodes: state.nodes,
        elapsed_s: state.start.elapsed().as_secs_f64(),
        best_depth: state.best_depth,
        best_edges: state.best_edges,
        best_placement: state.best_placement,
        timed_out: state.timed_out,
    }
}

struct ScaffoldRun<'a> {
    path: &'a [u32],
    domains: &'a mut [Vec<(PieceId, u8)>],
    rotated_edges: &'a [[Color; 4]],
    neighbors: &'a [[Option<u32>; 4]],
    placed: &'a mut [Option<(PieceId, u8)>],
    placed_edges: &'a mut [[Color; 4]],
    used: &'a mut [bool],
    cfg: &'a ScaffoldConfig,
    start: Instant,
    nodes: u64,
    best_depth: u32,
    best_edges: u32,
    best_placement: Vec<Option<(PieceId, u8)>>,
    timed_out: bool,
}

impl<'a> ScaffoldRun<'a> {
    fn time_up(&mut self) -> bool {
        if self.cfg.time_budget.is_zero() { return false; }
        let up = self.start.elapsed() >= self.cfg.time_budget;
        if up { self.timed_out = true; }
        up
    }

    fn current_edges_count(&self) -> u32 {
        let mut c = 0u32;
        for pos in 0..self.placed.len() {
            if self.placed[pos].is_none() { continue; }
            // Count matched edges with placed neighbors. Each internal
            // edge counted from both sides → divide by 2 at end. Or
            // we walk only (top, left) directions to count once.
            let edges = self.placed_edges[pos];
            let neis = &self.neighbors[pos];
            for side in 0..4usize {
                // Count only top (0) and left (3) to avoid double-counting.
                if side != 0 && side != 3 { continue; }
                let n = match neis[side] { Some(n) => n, None => continue };
                let np = self.placed[n as usize];
                if np.is_none() { continue; }
                let opposite = match side { 0 => 2, 3 => 1, _ => unreachable!() };
                let n_edges = self.placed_edges[n as usize];
                if edges[side] == n_edges[opposite] {
                    c += 1;
                }
            }
        }
        c
    }

    fn record_best(&mut self, depth: u32) {
        if depth > self.best_depth {
            self.best_depth = depth;
            self.best_placement.copy_from_slice(self.placed);
            self.best_edges = self.current_edges_count();
        } else if depth == self.best_depth {
            // Same depth — record the placement with more matched edges.
            let e = self.current_edges_count();
            if e > self.best_edges {
                self.best_edges = e;
                self.best_placement.copy_from_slice(self.placed);
            }
        }
    }

    fn dfs(&mut self, depth: usize) {
        self.nodes += 1;
        if self.nodes & 0xffff == 0 && self.time_up() { return; }

        // Reached end of path with all cells placed.
        if depth >= self.path.len() {
            self.record_best(depth as u32);
            return;
        }

        let pos = self.path[depth];

        // Iterate domain of this cell. Each candidate must:
        //   (a) not use an already-used piece.
        //   (b) match colors against placed neighbors.
        let dom = self.domains[pos as usize].clone(); // small clone; speed not critical here

        let mut tried_any = false;
        for (pid, rot) in dom {
            if self.used[pid as usize] { continue; }
            let edges = self.rotated_edges[pid as usize * 4 + rot as usize];
            let neis = &self.neighbors[pos as usize];
            let mut ok = true;
            for side in 0..4usize {
                let n = match neis[side] {
                    Some(n) => n,
                    None => continue,
                };
                let placed_neighbor = self.placed[n as usize];
                if placed_neighbor.is_none() { continue; }
                let n_edges = self.placed_edges[n as usize];
                let opposite = match side { 0 => 2, 1 => 3, 2 => 0, _ => 1 };
                if edges[side] != n_edges[opposite] {
                    ok = false;
                    break;
                }
            }
            if !ok { continue; }
            tried_any = true;
            // Place.
            self.placed[pos as usize] = Some((pid, rot));
            self.placed_edges[pos as usize] = edges;
            self.used[pid as usize] = true;
            // Record best before recursing — captures deepest reach.
            self.record_best((depth + 1) as u32);
            self.dfs(depth + 1);
            // Unplace.
            self.placed[pos as usize] = None;
            self.used[pid as usize] = false;
            if self.timed_out { return; }
            if self.cfg.node_budget > 0 && self.nodes >= self.cfg.node_budget { self.timed_out = true; return; }
        }
        // If no candidate worked, current depth is the deepest in this branch.
        if !tried_any {
            self.record_best(depth as u32);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_core::{Edges, Piece, Puzzle};

    fn p(id: u16, t: Color, r: Color, b: Color, l: Color) -> Piece {
        Piece::new(id, Edges::new(t, r, b, l))
    }

    #[test]
    fn scaffold_runs_on_tiny_2x2_with_full_pieces() {
        // 2x2 puzzle: just 4 corner cells. No inner cells.
        let pieces = vec![
            p(0, BORDER, 1, 1, BORDER), // TL
            p(1, BORDER, BORDER, 1, 1), // TR
            p(2, 1, 1, BORDER, BORDER), // BL
            p(3, 1, BORDER, BORDER, 1), // BR
        ];
        let puzzle = Puzzle::new(2, 2, 2, pieces).unwrap();
        let cfg = ScaffoldConfig {
            inner_set: Vec::new(),
            deferred: Vec::new(),
            worst_performers: Vec::new(),
            time_budget: Duration::from_secs(1),
            node_budget: 1_000_000,
            seed: 0,
        };
        let r = run_scaffold(&puzzle, &cfg);
        // The 4 corners can tile a 2x2: TL, TR, BL, BR.
        assert!(r.best_depth >= 1, "expected to place at least one piece, got depth {}", r.best_depth);
    }

    #[test]
    fn scaffold_respects_forbidden_inner_set() {
        // 3x3 with one inner cell. If inner_set is empty, the inner
        // cell has no placeable piece → max depth = 8 (all border).
        let pieces = vec![
            p(0, BORDER, 1, 1, BORDER), // TL
            p(1, BORDER, BORDER, 1, 1), // TR
            p(2, 1, 1, BORDER, BORDER), // BL
            p(3, 1, BORDER, BORDER, 1), // BR
            p(4, BORDER, 1, 1, 1), // top edge
            p(5, 1, 1, 1, BORDER), // left edge
            p(6, 1, BORDER, 1, 1), // right edge
            p(7, 1, 1, BORDER, 1), // bot edge
            p(8, 1, 1, 1, 1), // inner
        ];
        let puzzle = Puzzle::new(3, 3, 2, pieces).unwrap();
        let cfg_no_inner = ScaffoldConfig {
            inner_set: Vec::new(),
            deferred: vec![8],
            worst_performers: Vec::new(),
            time_budget: Duration::from_secs(1),
            node_budget: 1_000_000,
            seed: 0,
        };
        let r = run_scaffold(&puzzle, &cfg_no_inner);
        assert!(r.best_depth <= 8, "without inner piece, can place at most 8");
        let cfg_with_inner = ScaffoldConfig {
            inner_set: vec![8],
            deferred: Vec::new(),
            ..cfg_no_inner
        };
        let r2 = run_scaffold(&puzzle, &cfg_with_inner);
        assert!(r2.best_depth >= r.best_depth, "allowing inner piece should not reduce max depth");
    }
}
