// Microbenchmark helpers for the hot-path audit.
//
// This crate is intentionally lean: it exposes builder functions that
// the criterion benches in `benches/` use to construct realistic inputs
// (puzzles, boards, domains) without re-implementing solver internals.
//
// Each "candidate" function here corresponds to a hot-path operation
// identified in the audit. Some have a "baseline" (matches current
// engine implementation) and a "candidate" (a proposed reformulation:
// packed Edges-as-u32, branchless rotation, SIMD-friendly score loop).
// The bench file compares them head-to-head.

#![forbid(unsafe_code)]

// pub mod border_ub; // dropped: used only by non-published bins
// pub mod border_ub_lifted; // dropped: used only by non-published bins
// pub mod cluster_repair; // dropped: used only by non-published bins
pub mod mini; // BANDSAW: exact meet-in-the-middle endgame band solver

use std::io::Write;
use std::path::Path;
use std::time::Instant;

use eternity2_core::{Board, Color, PieceId, Position, Puzzle, Rotation};
use eternity2_events::{EventBody, EventSink, FinalStats, SolverEvent};
use eternity2_generator::{generate, GeneratorConfig};
use eternity2_localsearch::alns::score_board as score_board_baseline;
use eternity2_propagators::{PlacementInfo, PropagatorContext, gacolor_check, parity_check};

/// Reusable progress sink for bench bins.
///
/// Tracks `best_depth` across all events; captures `FinalStats` on any
/// terminal event (Solved / Exhausted / TimedOut / Cancelled); appends
/// a progress line `[<elapsed_ms> ms]  depth=N  best_depth=M  node_id=K`
/// to the log file every `period_ms` milliseconds.
///
/// Usage:
/// ```ignore
/// let mut sink = ProgressSink::new(&log_path, 5_000)?;
/// sink.write_line("=== my run header ===");
/// let outcome = solver.solve(&puzzle, &opts, &mut sink);
/// // sink.final_stats / sink.best_depth populated.
/// ```
pub struct ProgressSink {
    pub log: std::fs::File,
    started: Instant,
    last_log_ms: u64,
    period_ms: u64,
    pub best_depth: u32,
    pub final_stats: Option<FinalStats>,
}

impl ProgressSink {
    pub fn new(path: &Path, period_ms: u64) -> std::io::Result<Self> {
        Ok(Self {
            log: std::fs::File::create(path)?,
            started: Instant::now(),
            last_log_ms: 0,
            period_ms,
            best_depth: 0,
            final_stats: None,
        })
    }

    pub fn write_line(&mut self, line: &str) {
        let _ = writeln!(self.log, "{line}");
        let _ = self.log.flush();
    }
}

impl EventSink for ProgressSink {
    fn emit(&mut self, event: SolverEvent) {
        let elapsed_ms = self.started.elapsed().as_millis() as u64;

        if let EventBody::Backtrack { from_depth, .. } = &event.body {
            if *from_depth > self.best_depth { self.best_depth = *from_depth; }
        }
        if event.depth > self.best_depth { self.best_depth = event.depth; }

        if elapsed_ms.saturating_sub(self.last_log_ms) >= self.period_ms {
            self.last_log_ms = elapsed_ms;
            let _ = writeln!(self.log,
                "[{:>7} ms]  depth={:>4}  best_depth={:>4}  node_id={}",
                elapsed_ms, event.depth, self.best_depth, event.node_id);
            let _ = self.log.flush();
        }

        match event.body {
            EventBody::Solved { final_stats, .. }
            | EventBody::Exhausted { final_stats, .. }
            | EventBody::TimedOut { final_stats, .. }
            | EventBody::Cancelled { final_stats, .. } => {
                let _ = writeln!(self.log,
                    "=== terminal event at {elapsed_ms}ms: nodes={} backtracks={} max_depth_seen={} ===",
                    final_stats.nodes, final_stats.backtracks, final_stats.max_depth_seen);
                let _ = self.log.flush();
                self.final_stats = Some(final_stats);
            }
            _ => {}
        }
    }
}

/// Vol-33 T5 — silent sink. No file, no logging; just captures the
/// final `FinalStats` and tracks `best_depth`. Several bins (compare,
/// run_8x8_solve, profile_*) want this when they're batching A/B
/// configurations and don't need per-config log files.
#[derive(Default)]
pub struct QuietSink {
    pub best_depth: u32,
    pub final_stats: Option<FinalStats>,
}

impl QuietSink {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl EventSink for QuietSink {
    fn emit(&mut self, event: SolverEvent) {
        if let EventBody::Backtrack { from_depth, .. } = &event.body {
            if *from_depth > self.best_depth {
                self.best_depth = *from_depth;
            }
        }
        if event.depth > self.best_depth {
            self.best_depth = event.depth;
        }
        match event.body {
            EventBody::Solved { final_stats, .. }
            | EventBody::Exhausted { final_stats, .. }
            | EventBody::TimedOut { final_stats, .. }
            | EventBody::Cancelled { final_stats, .. } => {
                self.final_stats = Some(final_stats);
            }
            _ => {}
        }
    }
}

// ---------- Vol-16 Cat-3 — shared harness helpers ----------
//
// The bench bins in `src/bin/` share ~80 lines of boilerplate each.
// These helpers move the common parts into one place so each bin is
// just CLI parsing + a Pipeline invocation. Migration is incremental:
// bins added in vol-12 / vol-14 / vol-15 originally inlined their own
// copies of the helpers below; vol-16 swaps them out one at a time.

// Reporting / scoring helpers now live in `eternity2-export`. Re-export so
// existing call sites (`use eternity2_bench_audit::{score_board, ...}`)
// keep working. New code should import from `eternity2_export` directly.
pub use eternity2_export::{internal_edge_count, placed_count, render_board, score_board};

/// Score `board` against the puzzle's *total internal-edge count*
/// (= `internal_edge_count(puzzle)`). Returns `(matched, total)`.
/// Equivalent to `score_board` for fully-placed boards; differs on
/// partials because the denominator includes adjacencies where one
/// or both cells are empty.
#[must_use]
pub fn score_board_dense(puzzle: &Puzzle, board: &Board) -> (u32, u32) {
    let total = internal_edge_count(puzzle);
    let (matched, _) = score_board(puzzle, board);
    (matched, total)
}

/// Vol-51 — extracted from `bin/edge_bound_ascent.rs` so other drivers
/// can compute the greedy-relaxed score between rounds.
///
/// **⚠️ NOT AN UPPER BOUND ON INTEGER SCORE ⚠️**
///
/// Returns the score achievable by greedy cell-local optimization,
/// IGNORING piece-uniqueness (so pieces CAN be reused). Because piece
/// reuse fakes matches that real integer assignments cannot achieve,
/// **the return value is OFTEN HIGHER than the true integer ceiling**.
/// On the vol-32 458 board this function returns 462 — 4 above the
/// true integer optimum confirmed by vol-44 MIP.
///
/// This is a HEURISTIC INDICATOR of matching density, useful for
/// COMPARATIVE ranking of basins. It is NOT a valid mathematical
/// upper bound. Do not write "X has bound Y" using this value.
///
/// For TRUE upper bounds on integer score, use:
/// - `border_lp_ub.rs` (LP relaxation; sound UB)
/// - `border_mip.rs` or vol-55 cluster MIP (sound integer ceiling)
///
/// Per `feedback_no_false_metrics` memory: cheapness is not an
/// excuse for falsehood. Use the right tool for the claim.
#[must_use]
pub fn relaxed_bound(puzzle: &eternity2_core::Puzzle, board: &eternity2_core::Board) -> u32 {
    use eternity2_core::{Rotation, BORDER};
    let mut b = board.clone();
    let n_cells = puzzle.cell_count();
    for _ in 0..20 {
        let mut changes = 0u32;
        for pos in 0..n_cells {
            let cur_local = b.get(pos).map(|(pid, rot)| {
                let e = puzzle.piece(pid).unwrap().edges.rotated(rot).as_array();
                cell_local_score_for_edges(puzzle, &b, pos, e)
            }).unwrap_or(0);
            let mut best: Option<(u16, Rotation, u32)> = None;
            let w = puzzle.width;
            let h = puzzle.height;
            let x = pos % w;
            let y = pos / w;
            for piece in puzzle.pieces() {
                for rot in Rotation::ALL {
                    let e = piece.edges.rotated(rot).as_array();
                    if (e[0] == BORDER) != (y == 0) { continue; }
                    if (e[1] == BORDER) != (x + 1 == w) { continue; }
                    if (e[2] == BORDER) != (y + 1 == h) { continue; }
                    if (e[3] == BORDER) != (x == 0) { continue; }
                    let s = cell_local_score_for_edges(puzzle, &b, pos, e);
                    if best.map(|(_, _, b_)| s > b_).unwrap_or(true) {
                        best = Some((piece.id, rot, s));
                    }
                }
            }
            if let Some((pid, rot, s)) = best {
                let cur = b.get(pos);
                let cur_pid = cur.map(|(p, _)| p).unwrap_or(0);
                let cur_rot = cur.map(|(_, r)| r).unwrap_or(Rotation::R0);
                if (pid, rot) != (cur_pid, cur_rot) && s > cur_local {
                    b.place(pos, pid, rot);
                    changes += 1;
                }
            }
        }
        if changes == 0 { break; }
    }
    score_board(puzzle, &b).0
}

/// Vol-60 — alias for `relaxed_bound` with a name that doesn't lie.
/// This function does NOT compute an upper bound. It runs greedy
/// local search ignoring piece-uniqueness. Use this name in new code
/// to avoid the documentation hazard of `relaxed_bound`.
#[must_use]
pub fn greedy_relaxed_score(puzzle: &eternity2_core::Puzzle, board: &eternity2_core::Board) -> u32 {
    relaxed_bound(puzzle, board)
}

fn cell_local_score_for_edges(puzzle: &eternity2_core::Puzzle, board: &eternity2_core::Board, pos: u32, edges: [u8; 4]) -> u32 {
    use eternity2_core::BORDER;
    let w = puzzle.width;
    let h = puzzle.height;
    let x = pos % w;
    let y = pos / w;
    let mut s = 0u32;
    let get_edges = |p: u32| -> Option<[u8; 4]> {
        board.get(p).map(|(pid, rot)| puzzle.piece(pid).expect("piece").edges.rotated(rot).as_array())
    };
    if y > 0 {
        if let Some(ne) = get_edges(pos - w) {
            if edges[0] != BORDER && ne[2] != BORDER && edges[0] == ne[2] { s += 1; }
        }
    }
    if x + 1 < w {
        if let Some(ne) = get_edges(pos + 1) {
            if edges[1] != BORDER && ne[3] != BORDER && edges[1] == ne[3] { s += 1; }
        }
    }
    if y + 1 < h {
        if let Some(ne) = get_edges(pos + w) {
            if edges[2] != BORDER && ne[0] != BORDER && edges[2] == ne[0] { s += 1; }
        }
    }
    if x > 0 {
        if let Some(ne) = get_edges(pos - 1) {
            if edges[3] != BORDER && ne[1] != BORDER && edges[3] == ne[1] { s += 1; }
        }
    }
    s
}

/// Vol-33 T5 — shared bin-harness helpers.
///
/// Most bench-audit bins follow the same shape:
///   1. load puzzle + hints
///   2. open a ProgressSink
///   3. build a solver + opts, call solver.solve()
///   4. pattern-match SolveOutcome → (verdict, best_partial_board)
///   5. format a human summary (verdict, wall-clock, FinalStats, score)
///   6. serialize a JSON post-mortem (placement + stats + bucas URL)
///
/// Steps 4–6 are the largest copy-paste. The helpers below collapse
/// them into ~3 calls. The bins remain free-form because every bin
/// has bespoke headers / extra knobs, so this is a *toolkit*, not a
/// framework.
pub mod harness {
    use eternity2_core::{Board, Puzzle};
    use eternity2_events::FinalStats;
    use eternity2_export::{bucas_url, placed_count, score_board};
    use eternity2_solver_trait::SolveOutcome;
    use serde_json::{json, Value};

    /// The two things every bin pulls out of a `SolveOutcome` afterwards:
    /// a human-readable verdict label and the best board (final or partial).
    pub struct OutcomeView {
        pub verdict: String,
        pub board: Option<Board>,
    }

    /// Reduce any `SolveOutcome` to (verdict, board). The board is the
    /// solved board on success, the best partial on timeout/cancel, the
    /// first solution on AllSolutions, or None on Exhausted/Error.
    #[must_use]
    pub fn outcome_to_view(outcome: SolveOutcome) -> OutcomeView {
        match outcome {
            SolveOutcome::Solved(b) => OutcomeView {
                verdict: "SOLVED".into(),
                board: Some(b),
            },
            SolveOutcome::TimedOut {
                best_partial,
                best_depth,
            } => OutcomeView {
                verdict: format!("TIMEOUT (best_depth={best_depth})"),
                board: Some(best_partial),
            },
            SolveOutcome::Cancelled {
                best_partial,
                best_depth,
                ..
            } => OutcomeView {
                verdict: format!("CANCELLED (best_depth={best_depth})"),
                board: Some(best_partial),
            },
            SolveOutcome::Exhausted => OutcomeView {
                verdict: "EXHAUSTED".into(),
                board: None,
            },
            SolveOutcome::AllSolutions(bs) => OutcomeView {
                verdict: format!("ALL ({})", bs.len()),
                board: bs.into_iter().next(),
            },
            SolveOutcome::Error(e) => OutcomeView {
                verdict: format!("ERROR: {e}"),
                board: None,
            },
        }
    }

    /// Append the standard "FINAL RESULT" block to `out`: verdict,
    /// wall-clock, FinalStats, score+placed, ASCII board, bucas URL.
    /// If `board` is None, omits the score/board parts.
    pub fn write_summary(
        out: &mut String,
        puzzle: &Puzzle,
        verdict: &str,
        elapsed_secs: f64,
        final_stats: Option<&FinalStats>,
        board: Option<&Board>,
        puzzle_name: &str,
    ) {
        out.push_str("\n=== FINAL RESULT ===\n");
        out.push_str(&format!("verdict: {verdict}\n"));
        out.push_str(&format!("wall_clock: {elapsed_secs:.2} s\n"));
        if let Some(s) = final_stats {
            out.push_str(&format!(
                "nodes: {}\nbacktracks: {}\npropagations: {}\ndomain_wipeouts: {}\nmax_depth_seen: {}\nsolutions_found: {}\nnodes_per_sec: {:.0}\n",
                s.nodes,
                s.backtracks,
                s.propagations,
                s.domain_wipeouts,
                s.max_depth_seen,
                s.solutions_found,
                s.nodes as f64 / elapsed_secs.max(1e-6)
            ));
        }
        if let Some(b) = board {
            let (matched, total) = score_board(puzzle, b);
            let placed = placed_count(b, puzzle);
            let internal_total = 2 * puzzle.width * puzzle.height - puzzle.width - puzzle.height;
            out.push_str(&format!(
                "pieces_placed: {}/{}\nedge_matches: {}/{} (counting only placed-placed joins)\ninternal_total_edges: {}\n",
                placed,
                puzzle.cell_count(),
                matched,
                total,
                internal_total
            ));
            out.push_str("\nBoard (piece_id:rotation):\n");
            out.push_str(&eternity2_export::render_board(puzzle, b));
            let bucas = bucas_url(puzzle, b, puzzle_name);
            out.push_str(&format!("\nbucas: {bucas}\n"));
        }
    }

    /// Serialize the standard postmortem JSON: verdict, elapsed, score,
    /// stats, per-cell placement, bucas URL. Caller writes it to disk.
    #[must_use]
    pub fn build_postmortem_json(
        puzzle: &Puzzle,
        verdict: &str,
        elapsed_secs: f64,
        final_stats: Option<&FinalStats>,
        board: Option<&Board>,
        puzzle_name: &str,
    ) -> Value {
        let stats_json = final_stats.map(|s| {
            json!({
                "time_ms": s.time_ms,
                "nodes": s.nodes,
                "backtracks": s.backtracks,
                "propagations": s.propagations,
                "domain_wipeouts": s.domain_wipeouts,
                "current_depth": s.current_depth,
                "max_depth_seen": s.max_depth_seen,
                "solutions_found": s.solutions_found,
            })
        });
        let internal_total = 2 * puzzle.width * puzzle.height - puzzle.width - puzzle.height;
        if let Some(b) = board {
            let (matched, total) = score_board(puzzle, b);
            let placed = placed_count(b, puzzle);
            let bucas = bucas_url(puzzle, b, puzzle_name);
            let placement: Vec<Value> = (0..puzzle.cell_count())
                .map(|pos| match b.get(pos) {
                    Some((pid, rot)) => json!({
                        "pos": pos,
                        "piece_id": u32::from(pid),
                        "rotation": rot.as_u8(),
                    }),
                    None => Value::Null,
                })
                .collect();
            json!({
                "schema_version": 1,
                "verdict": verdict,
                "elapsed_s": elapsed_secs,
                "pieces_placed": placed,
                "edge_matches": matched,
                "edge_total": total,
                "internal_total_edges": internal_total,
                "bucas_url": bucas,
                "placement": placement,
                "final_stats": stats_json,
            })
        } else {
            json!({
                "schema_version": 1,
                "verdict": verdict,
                "elapsed_s": elapsed_secs,
                "internal_total_edges": internal_total,
                "final_stats": stats_json,
            })
        }
    }
}

// ---------- Workload builders ----------

/// Build a deterministic random puzzle of a given size. Used by all
/// benches to keep inputs reproducible across runs.
#[must_use]
pub fn build_puzzle(size: u32, colors: u32, seed: u64) -> Puzzle {
    generate(GeneratorConfig { size, interior_colors: colors, seed })
        .expect("generator must succeed for valid sizes")
}

/// Place every piece of the puzzle at its canonical position (the
/// generator emits pieces in solved row-major order with each piece's
/// rotation = R0). Produces a fully-matched board to exercise scoring
/// at saturation.
#[must_use]
pub fn solved_board(puzzle: &Puzzle) -> Board {
    let mut b = Board::empty(puzzle);
    for (i, p) in puzzle.pieces().iter().enumerate() {
        b.place(i as Position, p.id, Rotation::R0);
    }
    b
}

// ---------- Edges rotation ----------

/// Baseline: array-based, current implementation in eternity2-core.
#[inline(always)]
#[must_use]
pub fn rotate_edges_baseline(e: [Color; 4], r: u8) -> [Color; 4] {
    let [t, ri, b, l] = e;
    match r & 0b11 {
        0 => [t, ri, b, l],
        1 => [l, t, ri, b],
        2 => [b, l, t, ri],
        _ => [ri, b, l, t],
    }
}

/// Candidate: pack [Color; 4] into u32, rotate by byte-shift, unpack.
/// On aarch64-apple-darwin this collapses to a single ROR on the
/// packed u32 (then a load to memory). Layout: byte0=top, byte1=right,
/// byte2=bottom, byte3=left (little-endian).
#[inline(always)]
#[must_use]
pub fn rotate_edges_packed(e: [Color; 4], r: u8) -> [Color; 4] {
    let packed = u32::from_le_bytes(e);
    let rot = (r & 0b11) * 8;
    // Baseline R90 sends (t,r,b,l) → (l,t,r,b). In LE bytes: byte0=t
    // moves to byte1, byte3=l moves to byte0. That's a rotate-left of
    // the *value* by 8 bits, which (in LE) maps to byte-position +1.
    let rotated = packed.rotate_left(rot.into());
    rotated.to_le_bytes()
}

// ---------- Score ----------

/// Candidate score: a flat over-row+column loop that always touches
/// edges in a packed [u32] cache instead of `Option<(...)>` calls.
/// Demonstrates the per-cell-edge-cache idea from the audit.
#[must_use]
pub fn score_packed(puzzle: &Puzzle, edges_grid: &[[Color; 4]]) -> u32 {
    let w = puzzle.width as usize;
    let h = puzzle.height as usize;
    let mut m: u32 = 0;
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            let e = edges_grid[i];
            if x + 1 < w {
                let ne = edges_grid[i + 1];
                if e[1] != 0 && e[1] == ne[3] { m += 1; }
            }
            if y + 1 < h {
                let ne = edges_grid[i + w];
                if e[2] != 0 && e[2] == ne[0] { m += 1; }
            }
        }
    }
    m
}

/// Build the dense edges-grid used by `score_packed`.
#[must_use]
pub fn build_edges_grid(puzzle: &Puzzle, board: &Board) -> Vec<[Color; 4]> {
    let n = puzzle.cell_count() as usize;
    let mut out = vec![[0u8; 4]; n];
    // Materialise edges_for(pid, rot) via Piece::edges and Rotation::rotated.
    for pos in 0..puzzle.cell_count() {
        if let Some((pid, rot)) = board.get(pos) {
            if let Some(p) = puzzle.piece(pid) {
                out[pos as usize] = p.edges.rotated(rot).as_array();
            }
        }
    }
    out
}

/// Re-export of the localsearch baseline scorer so benches can compare.
pub fn score_board_baseline_wrapper(puzzle: &Puzzle, board: &Board) -> u32 {
    score_board_baseline(puzzle, board)
}

// ---------- Propagator state ----------

/// Build a `PropagatorContext`-shaped tuple. Returns the owned buffers
/// so the bench can hand them to the propagator without re-creating
/// them inside the timed section.
#[must_use]
pub fn build_propagator_inputs(puzzle: &Puzzle, board: &Board)
    -> (Vec<Option<PlacementInfo>>, Vec<bool>, Vec<u64>, usize)
{
    let n = puzzle.cell_count() as usize;
    let mut placed = vec![None; n];
    let max_pid = puzzle.pieces().iter().map(|p| usize::from(p.id) + 1).max().unwrap_or(0);
    let mut used = vec![false; max_pid];
    for pos in 0..puzzle.cell_count() {
        if let Some((pid, rot)) = board.get(pos) {
            if let Some(p) = puzzle.piece(pid) {
                placed[pos as usize] = Some(PlacementInfo {
                    edges_after_rotation: p.edges.rotated(rot).as_array(),
                });
                used[usize::from(pid)] = true;
            }
        }
    }
    let n_rows = max_pid * 4;
    let wpp = n_rows.div_ceil(64).max(1);
    let domain_bits = vec![0u64; n * wpp];
    (placed, used, domain_bits, wpp)
}

pub fn run_gacolor(puzzle: &Puzzle,
                   placed: &[Option<PlacementInfo>],
                   used: &[bool],
                   domain_bits: &[u64],
                   words_per_pos: usize)
{
    let ctx = PropagatorContext { puzzle, placed, used_pieces: used, domain_bits, words_per_pos };
    let _ = gacolor_check(&ctx);
}

pub fn run_parity(puzzle: &Puzzle,
                  placed: &[Option<PlacementInfo>],
                  used: &[bool],
                  domain_bits: &[u64],
                  words_per_pos: usize)
{
    let ctx = PropagatorContext { puzzle, placed, used_pieces: used, domain_bits, words_per_pos };
    let _ = parity_check(&ctx);
}

// ---------- Board cell representation ----------

/// Candidate Board representation: pack (piece_id u16, rotation u8) into
/// u32 with sentinel 0xFFFF_FFFF for empty. Avoids the `Option<(u16,u8)>`
/// niche churn and gives a 4-byte aligned cell — friendly to SIMD loads.
#[derive(Debug, Clone)]
pub struct PackedBoard {
    pub width: u32,
    pub height: u32,
    /// One u32 per cell: low 16 bits = piece_id, bits 16..18 = rotation,
    /// bits 24..32 = flags. 0xFFFF_FFFF means empty.
    pub cells: Vec<u32>,
}

const EMPTY_CELL: u32 = 0xFFFF_FFFF;

impl PackedBoard {
    #[must_use]
    pub fn empty(puzzle: &Puzzle) -> Self {
        let n = (puzzle.width as usize) * (puzzle.height as usize);
        Self {
            width: puzzle.width,
            height: puzzle.height,
            cells: vec![EMPTY_CELL; n],
        }
    }

    #[inline(always)]
    pub fn place(&mut self, pos: Position, piece_id: PieceId, rot: Rotation) {
        let packed = u32::from(piece_id) | ((rot.as_u8() as u32) << 16);
        self.cells[pos as usize] = packed;
    }

    #[inline(always)]
    #[must_use]
    pub fn get(&self, pos: Position) -> Option<(PieceId, Rotation)> {
        let c = self.cells[pos as usize];
        if c == EMPTY_CELL { return None; }
        Some((c as u16, Rotation::from_u8(((c >> 16) & 0b11) as u8).unwrap()))
    }
}

/// Build a `PackedBoard` from a regular `Board`.
#[must_use]
pub fn pack_board(puzzle: &Puzzle, board: &Board) -> PackedBoard {
    let mut p = PackedBoard::empty(puzzle);
    for pos in 0..puzzle.cell_count() {
        if let Some((pid, rot)) = board.get(pos) {
            p.place(pos, pid, rot);
        }
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_packed_matches_baseline() {
        for t in 0..3 { for r in 0..3 { for b in 0..3 { for l in 0..3 {
            let e = [t as Color, r as Color, b as Color, l as Color];
            for rot in 0..4u8 {
                assert_eq!(rotate_edges_baseline(e, rot), rotate_edges_packed(e, rot),
                    "rotation mismatch on {:?} r={}", e, rot);
            }
        }}}}
    }

    #[test]
    fn score_packed_matches_baseline() {
        let p = build_puzzle(6, 5, 42);
        let b = solved_board(&p);
        let grid = build_edges_grid(&p, &b);
        // The solved board is a valid solution so every interior edge matches:
        // the bench just confirms scoring is not catastrophically broken.
        let s_packed = score_packed(&p, &grid);
        let s_base = score_board_baseline_wrapper(&p, &b);
        assert_eq!(s_packed, s_base);
    }

    #[test]
    fn packed_board_roundtrip() {
        let p = build_puzzle(5, 4, 7);
        let b = solved_board(&p);
        let pb = pack_board(&p, &b);
        for pos in 0..p.cell_count() {
            assert_eq!(pb.get(pos), b.get(pos));
        }
    }

}
