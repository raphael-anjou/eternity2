#![forbid(unsafe_code)]

use eternity2_time::Clock;
use eternity2_core::{Board, Color, PathPolicy, PieceId, Position, Puzzle, Rotation, BORDER};
use eternity2_events::{
    BacktrackCause, EventBody, EventSink, FinalStats, SelectionReason, SolverEvent,
};
use eternity2_solver_trait::{
    HeuristicProfile, SolveMode, SolveOpts, SolveOutcome, Solver, SolverId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Traversal {
    RowByRow,
    Spiral,
    StrictPath,  // requires PathPolicy::Strict and a non-empty path
}

pub struct NaiveSolver {
    traversal: Traversal,
    allow_rotation: bool,
}

impl NaiveSolver {
    #[must_use]
    pub const fn new(traversal: Traversal, allow_rotation: bool) -> Self {
        Self { traversal, allow_rotation }
    }

    #[must_use]
    pub const fn row_by_row() -> Self {
        Self::new(Traversal::RowByRow, true)
    }

    #[must_use]
    pub const fn spiral() -> Self {
        Self::new(Traversal::Spiral, true)
    }

    fn profile_name(&self) -> &'static str {
        match self.traversal {
            Traversal::RowByRow => "row_by_row",
            Traversal::Spiral => "spiral",
            Traversal::StrictPath => "strict_path",
        }
    }
}

impl Solver for NaiveSolver {
    fn id(&self) -> SolverId { SolverId("naive".into()) }
    fn heuristic_profile(&self) -> HeuristicProfile {
        HeuristicProfile(self.profile_name().into())
    }

    fn supports_path_policy(&self, policy: &PathPolicy) -> bool {
        matches!(policy, PathPolicy::Strict | PathPolicy::Ignored)
    }

    fn solve(
        &mut self,
        puzzle: &Puzzle,
        opts: &SolveOpts,
        sink: &mut dyn EventSink,
    ) -> SolveOutcome {
        let order = match (self.traversal, &opts.path_policy) {
            (Traversal::StrictPath, PathPolicy::Strict) => {
                if opts.path.len() as u32 != puzzle.cell_count() {
                    return SolveOutcome::Error(format!(
                        "Strict path must cover all {} cells; got {}",
                        puzzle.cell_count(), opts.path.len()
                    ));
                }
                opts.path.clone()
            }
            (Traversal::StrictPath, _) => {
                return SolveOutcome::Error(
                    "StrictPath traversal requires PathPolicy::Strict".into(),
                );
            }
            (Traversal::RowByRow, _) => row_by_row(puzzle),
            (Traversal::Spiral, _) => spiral(puzzle),
        };

        let state = SearchState::new(puzzle, self, opts, order);
        state.run(sink)
    }
}

fn row_by_row(puzzle: &Puzzle) -> Vec<Position> {
    (0..puzzle.cell_count()).collect()
}

// Outer ring (top row, right col, bottom row reversed, left col reversed),
// then recurse on the (w-2)×(h-2) interior. Classic spiral order.
fn spiral(puzzle: &Puzzle) -> Vec<Position> {
    let w = puzzle.width;
    let h = puzzle.height;
    let mut out = Vec::with_capacity((w * h) as usize);
    let (mut x0, mut y0) = (0u32, 0u32);
    let (mut x1, mut y1) = (w, h);
    while x0 < x1 && y0 < y1 {
        for x in x0..x1 { out.push(y0 * w + x); }
        for y in (y0 + 1)..y1 { out.push(y * w + (x1 - 1)); }
        if y1 > y0 + 1 {
            for x in (x0..(x1 - 1)).rev() { out.push((y1 - 1) * w + x); }
        }
        if x1 > x0 + 1 {
            for y in ((y0 + 1)..(y1 - 1)).rev() { out.push(y * w + x0); }
        }
        x0 += 1; y0 += 1;
        if x1 > 0 { x1 -= 1; }
        if y1 > 0 { y1 -= 1; }
    }
    out
}

// Per (piece, rotation) cache. Indexed as piece_id * 4 + rotation.
// `valid` marks rotations distinct under symmetry (the canonical
// representative for an equivalence class).
struct OrientationTable {
    edges: Vec<[Color; 4]>,
    valid: Vec<bool>,
}

impl OrientationTable {
    fn build(puzzle: &Puzzle) -> Self {
        let pieces = puzzle.pieces();
        let n: usize = pieces.iter().map(|p| (p.id as usize) + 1).max().unwrap_or(0);
        let mut edges = vec![[0u8; 4]; n * 4];
        let mut valid = vec![false; n * 4];
        for piece in pieces {
            let pid = piece.id as usize;
            let mut seen: [Option<[Color; 4]>; 4] = [None; 4];
            for r in 0..4u8 {
                let rot = Rotation::from_u8(r).unwrap();
                let e = piece.edges.rotated(rot).as_array();
                edges[pid * 4 + r as usize] = e;
                let mut is_new = true;
                for s in seen.iter().take(r as usize) {
                    if let Some(prev) = s {
                        if *prev == e { is_new = false; break; }
                    }
                }
                if is_new {
                    seen[r as usize] = Some(e);
                    valid[pid * 4 + r as usize] = true;
                }
            }
        }
        let _ = n;
        Self { edges, valid }
    }

    #[inline]
    fn edges_of(&self, piece_id: PieceId, rot: u8) -> [Color; 4] {
        self.edges[piece_id as usize * 4 + rot as usize]
    }

    #[inline]
    fn is_valid_rotation(&self, piece_id: PieceId, rot: u8) -> bool {
        self.valid[piece_id as usize * 4 + rot as usize]
    }
}

struct SearchState<'a> {
    puzzle: &'a Puzzle,
    solver_id: String,
    profile: String,
    opts: &'a SolveOpts,
    order: Vec<Position>,
    table: OrientationTable,
    started: Clock,
    node_id: u64,
    stats: FinalStats,
    best_depth: u32,
    best_partial: Option<Board>,
    allow_rotation: bool,
}

impl<'a> SearchState<'a> {
    fn new(puzzle: &'a Puzzle, solver: &NaiveSolver, opts: &'a SolveOpts, order: Vec<Position>) -> Self {
        Self {
            puzzle,
            solver_id: "naive".into(),
            profile: solver.profile_name().into(),
            opts,
            order,
            table: OrientationTable::build(puzzle),
            started: Clock::now(),
            node_id: 0,
            stats: FinalStats::default(),
            best_depth: 0,
            best_partial: None,
            allow_rotation: solver.allow_rotation,
        }
    }

    fn elapsed_us(&self) -> u64 {
        self.started.elapsed_us()
    }

    fn timed_out(&self) -> bool {
        if self.opts.time_budget_ms == 0 { return false; }
        self.elapsed_us() / 1000 >= self.opts.time_budget_ms
    }

    fn emit(&mut self, sink: &mut dyn EventSink, depth: u32, body: EventBody) {
        self.node_id += 1;
        let event = SolverEvent {
            schema_version: 1,
            solver_run_id: self.opts.solver_run_id,
            node_id: self.node_id,
            depth,
            timestamp_us: self.elapsed_us(),
            body,
        };
        sink.emit(event);
    }

    fn fits(&self, board: &Board, position: Position, e: [Color; 4]) -> bool {
        let puzzle = self.puzzle;
        let (x, y) = puzzle.xy(position);
        let [top, right, bottom, left] = e;
        // Border constraints: edges facing outside the board must be BORDER;
        // interior-facing edges must NOT be BORDER (no gray inside).
        let on_top = y == 0;
        let on_bot = y == puzzle.height - 1;
        let on_left = x == 0;
        let on_right = x == puzzle.width - 1;
        if on_top && top != BORDER { return false; }
        if !on_top && top == BORDER { return false; }
        if on_bot && bottom != BORDER { return false; }
        if !on_bot && bottom == BORDER { return false; }
        if on_left && left != BORDER { return false; }
        if !on_left && left == BORDER { return false; }
        if on_right && right != BORDER { return false; }
        if !on_right && right == BORDER { return false; }
        // Match neighbors already placed.
        if !on_top {
            let up = (y - 1) * puzzle.width + x;
            if let Some((pid, rot)) = board.get(up) {
                let up_e = self.table.edges_of(pid, rot.as_u8());
                if up_e[2] != top { return false; }
            }
        }
        if !on_left {
            let lf = y * puzzle.width + (x - 1);
            if let Some((pid, rot)) = board.get(lf) {
                let lf_e = self.table.edges_of(pid, rot.as_u8());
                if lf_e[1] != left { return false; }
            }
        }
        // Solvers fill in `order`, which may not be left→right top→bottom.
        // Naive doesn't sort by border-first; the right/bottom checks are
        // symmetric in case the order placed those first.
        if !on_right {
            let rt = y * puzzle.width + (x + 1);
            if let Some((pid, rot)) = board.get(rt) {
                let rt_e = self.table.edges_of(pid, rot.as_u8());
                if rt_e[3] != right { return false; }
            }
        }
        if !on_bot {
            let dn = (y + 1) * puzzle.width + x;
            if let Some((pid, rot)) = board.get(dn) {
                let dn_e = self.table.edges_of(pid, rot.as_u8());
                if dn_e[0] != bottom { return false; }
            }
        }
        true
    }

    fn run(mut self, sink: &mut dyn EventSink) -> SolveOutcome {
        self.emit(sink, 0, EventBody::Started {
            solver_id: self.solver_id.clone(),
            heuristic_profile: self.profile.clone(),
            puzzle_fingerprint: self.puzzle.fingerprint(),
            seed: self.opts.seed,
            started_wall_us: 0,
        });

        let n = self.puzzle.cell_count() as usize;
        let mut board = Board::empty(self.puzzle);
        let n_pieces = self.puzzle.pieces().len();
        let mut used = vec![false; n_pieces];
        let mut hint_used = vec![false; n_pieces];

        // Honor hints by pre-placing them. Hints are honored regardless of
        // policy (per spec); if a hint position appears later in `order`,
        // the recursion sees it filled and skips selection there.
        for h in &self.opts.hints.hints {
            if h.position >= self.puzzle.cell_count() { continue; }
            if let Some(piece) = self.puzzle.piece(h.piece_id) {
                let e = self.table.edges_of(piece.id, h.rotation.as_u8());
                if !self.fits(&board, h.position, e) {
                    return SolveOutcome::Error(format!(
                        "hint at position {} does not satisfy constraints", h.position
                    ));
                }
                board.place(h.position, piece.id, h.rotation);
                used[piece.id as usize] = true;
                hint_used[piece.id as usize] = true;
            }
        }

        let mut solutions = Vec::<Board>::new();
        let outcome = self.recurse(sink, &mut board, &mut used, &hint_used, 0, n, &mut solutions);

        match outcome {
            RecurseResult::Found if matches!(self.opts.mode, SolveMode::FirstSolution) => {
                self.stats.time_ms = self.started.elapsed_us() / 1000;
                self.stats.solutions_found = 1;
                let board = solutions.into_iter().next().unwrap();
                self.emit(sink, 0, EventBody::Solved {
                    board: board.clone(),
                    final_stats: self.stats,
                });
                SolveOutcome::Solved(board)
            }
            RecurseResult::Found | RecurseResult::Exhausted => {
                self.stats.time_ms = self.started.elapsed_us() / 1000;
                self.stats.solutions_found = solutions.len() as u64;
                if matches!(self.opts.mode, SolveMode::FirstSolution) {
                    self.emit(sink, 0, EventBody::Exhausted {
                        final_stats: self.stats,
                        solutions_found: 0,
                    });
                    SolveOutcome::Exhausted
                } else {
                    self.emit(sink, 0, EventBody::Exhausted {
                        final_stats: self.stats,
                        solutions_found: solutions.len() as u64,
                    });
                    if solutions.is_empty() {
                        SolveOutcome::Exhausted
                    } else {
                        SolveOutcome::AllSolutions(solutions)
                    }
                }
            }
            RecurseResult::TimedOut => {
                self.stats.time_ms = self.started.elapsed_us() / 1000;
                let best = self.best_partial.clone().unwrap_or_else(|| Board::empty(self.puzzle));
                self.emit(sink, 0, EventBody::TimedOut {
                    final_stats: self.stats,
                    best_partial: best.clone(),
                    best_depth: self.best_depth,
                });
                SolveOutcome::TimedOut { best_partial: best, best_depth: self.best_depth }
            }
            RecurseResult::Cancelled => {
                self.stats.time_ms = self.started.elapsed_us() / 1000;
                let best = self.best_partial.clone().unwrap_or_else(|| Board::empty(self.puzzle));
                self.emit(sink, 0, EventBody::Cancelled {
                    final_stats: self.stats,
                    best_partial: best.clone(),
                    best_depth: self.best_depth,
                    solutions_so_far: solutions.clone(),
                });
                SolveOutcome::Cancelled {
                    best_partial: best,
                    best_depth: self.best_depth,
                    solutions_so_far: solutions,
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn recurse(
        &mut self,
        sink: &mut dyn EventSink,
        board: &mut Board,
        used: &mut [bool],
        hint_used: &[bool],
        depth: u32,
        n: usize,
        solutions: &mut Vec<Board>,
    ) -> RecurseResult {
        if depth as usize == n {
            solutions.push(board.clone());
            return RecurseResult::Found;
        }

        // Find next unfilled position in traversal order. Skipping over
        // already-hint-filled positions in order keeps the depth metric
        // honest: each non-hint placement increments depth.
        let position = loop {
            let idx = depth as usize;
            if idx >= self.order.len() {
                return RecurseResult::Exhausted;
            }
            let pos = self.order[idx];
            if board.get(pos).is_some() {
                // already filled by a hint; advance.
                return self.recurse(sink, board, used, hint_used, depth + 1, n, solutions);
            }
            break pos;
        };

        if depth > self.best_depth {
            self.best_depth = depth;
            self.best_partial = Some(board.clone());
        }

        // Periodic cancel + timeout check. Cheap.
        if self.node_id & 0xff == 0 {
            if !sink.should_continue() {
                return RecurseResult::Cancelled;
            }
            if self.timed_out() {
                return RecurseResult::TimedOut;
            }
        }

        self.emit(sink, depth, EventBody::VariableSelected {
            position,
            domain_size: 0, // naive doesn't precompute domains
            score: 0.0,
            reason: SelectionReason::PathStrict,
        });

        let n_pieces = self.puzzle.pieces().len();
        let rot_count = if self.allow_rotation { 4 } else { 1 };

        for pid in 0..n_pieces {
            if used[pid] || hint_used[pid] { continue; }
            let piece_id = self.puzzle.pieces()[pid].id;
            for r in 0..rot_count {
                if !self.table.is_valid_rotation(piece_id, r) { continue; }
                let e = self.table.edges_of(piece_id, r);
                if !self.fits(board, position, e) { continue; }
                let rotation = Rotation::from_u8(r).unwrap();
                self.emit(sink, depth, EventBody::ValueTried {
                    position, piece_id, rotation,
                });
                board.place(position, piece_id, rotation);
                used[pid] = true;
                self.stats.nodes += 1;

                match self.recurse(sink, board, used, hint_used, depth + 1, n, solutions) {
                    RecurseResult::Found => {
                        if matches!(self.opts.mode, SolveMode::FirstSolution) {
                            return RecurseResult::Found;
                        }
                        if self.opts.max_solutions > 0
                            && solutions.len() as u32 >= self.opts.max_solutions
                        {
                            return RecurseResult::Found;
                        }
                        // continue searching for more
                    }
                    other @ (RecurseResult::TimedOut | RecurseResult::Cancelled) => {
                        return other;
                    }
                    RecurseResult::Exhausted => {}
                }
                board.clear(position);
                used[pid] = false;
                self.stats.backtracks += 1;
                self.emit(sink, depth, EventBody::Backtrack {
                    from_depth: depth + 1,
                    to_depth: depth,
                    cause: BacktrackCause::NoMatch,
                });
            }
        }

        RecurseResult::Exhausted
    }
}

enum RecurseResult {
    Found,
    Exhausted,
    TimedOut,
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_core::{Edges, Piece};

    fn p(id: PieceId, t: Color, r: Color, b: Color, l: Color) -> Piece {
        Piece::new(id, Edges::new(t, r, b, l))
    }

    #[test]
    fn solves_2x2_trivial() {
        // Hand-built solvable 2x2 with colors 0 (border) and 1.
        // Layout (rotation 0):
        //   [01,01] | [01,01]   --> all pieces have top/left = BORDER + 1 each
        // We construct: corner piece with edges (top=0,right=1,bottom=1,left=0)
        // at pos 0, and copies fitting clockwise.
        let pieces = vec![
            p(0, 0, 1, 1, 0),  // TL corner
            p(1, 0, 0, 1, 1),  // TR corner
            p(2, 1, 1, 0, 0),  // BL corner
            p(3, 1, 0, 0, 1),  // BR corner
        ];
        let puzzle = Puzzle::new(2, 2, 2, pieces).unwrap();
        let mut solver = NaiveSolver::row_by_row();
        let mut sink = eternity2_events::BufferSink::new();
        let outcome = solver.solve(&puzzle, &SolveOpts::default(), &mut sink);
        assert!(matches!(outcome, SolveOutcome::Solved(_)), "got {outcome:?}");
    }

    #[test]
    fn solves_4x4_from_generator_within_budget() {
        use eternity2_generator::{generate, GeneratorConfig};
        let puzzle = generate(GeneratorConfig {
            size: 4, interior_colors: 4, seed: 7,
        }).unwrap();
        let mut solver = NaiveSolver::row_by_row();
        let mut sink = eternity2_events::BufferSink::new();
        let mut opts = SolveOpts::default();
        opts.time_budget_ms = 30_000;
        let outcome = solver.solve(&puzzle, &opts, &mut sink);
        assert!(matches!(outcome, SolveOutcome::Solved(_)), "got {outcome:?}");
    }

    #[test]
    fn solves_3x3_from_generator() {
        use eternity2_generator::{generate, GeneratorConfig};
        let puzzle = generate(GeneratorConfig {
            size: 3, interior_colors: 3, seed: 42,
        }).unwrap();
        let mut solver = NaiveSolver::row_by_row();
        let mut sink = eternity2_events::BufferSink::new();
        let outcome = solver.solve(&puzzle, &SolveOpts::default(), &mut sink);
        assert!(matches!(outcome, SolveOutcome::Solved(_)), "got {outcome:?}");
    }

    #[test]
    fn spiral_order_correct_for_3x3() {
        let pieces = (0..9).map(|i| p(i, 1, 1, 1, 1)).collect();
        let puz = Puzzle::new(3, 3, 2, pieces).unwrap();
        let order = spiral(&puz);
        // outer ring CW from (0,0): 0,1,2, 5, 8,7,6, 3 → then center 4.
        assert_eq!(order, vec![0, 1, 2, 5, 8, 7, 6, 3, 4]);
    }

    #[test]
    fn strict_path_unsupported_without_path() {
        let puzzle = Puzzle::new(2, 2, 2, vec![
            p(0, 0, 1, 1, 0), p(1, 0, 0, 1, 1),
            p(2, 1, 1, 0, 0), p(3, 1, 0, 0, 1),
        ]).unwrap();
        let mut solver = NaiveSolver::new(Traversal::StrictPath, true);
        let mut sink = eternity2_events::BufferSink::new();
        let outcome = solver.solve(&puzzle, &SolveOpts::default(), &mut sink);
        assert!(matches!(outcome, SolveOutcome::Error(_)));
    }
}
