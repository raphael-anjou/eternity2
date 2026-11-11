// Root-level work-stealing parallelism for the engine.
//
// Algorithm (matches legacy solvers/v3/parallel/parallel_dlx_solver.cpp):
//
//   Phase 1 — Enumerate work units up to `split_depth`. Each unit is the
//             sequence of (position, row_id) placements chosen so far.
//             Single-threaded; uses NullSink so no events leak.
//
//   Phase 2 — Dispatch units across rayon workers. Each worker:
//             a) constructs a fresh SearchState,
//             b) replays its prefix via place_and_propagate,
//             c) calls recurse() to finish the search.
//             A shared atomic flips when the first worker finds a
//             solution; the others see it via should_continue and bail.
//
// Events are forwarded through a crossbeam-channel to the main thread,
// which writes them to the caller-supplied sink. The sink stays
// single-threaded; workers stay event-pure relative to each other.

use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;

use crossbeam_channel::Sender;
use eternity2_core::{Board, Puzzle};
use eternity2_events::{EventBody, EventSink, FinalStats, SolverEvent};
use eternity2_solver_trait::{Objective, SolveMode, SolveOpts, SolveOutcome};

use crate::{EngineSolver, Parallelism, PropagationOutcome, RecurseResult, SearchState};

pub fn solve_parallel(
    solver: &EngineSolver,
    puzzle: &Puzzle,
    opts: &SolveOpts,
    sink: &mut dyn EventSink,
) -> SolveOutcome {
    let split_depth = match solver.config.parallelism {
        Parallelism::RootSplit { split_depth } => split_depth,
        Parallelism::SingleThread => unreachable!(),
    };
    let n_threads = rayon::current_num_threads().max(1);
    let target_depth = if split_depth == 0 {
        auto_split_depth(puzzle, n_threads)
    } else {
        split_depth
    };

    // Emit Started once for the whole parallel run.
    let wall_started_us = now_micros();
    let started = SolverEvent {
        schema_version: 1,
        solver_run_id: opts.solver_run_id,
        node_id: 0,
        depth: 0,
        timestamp_us: 0,
        body: EventBody::Started {
            solver_id: solver.solver_id.clone(),
            heuristic_profile: solver.heuristic_profile.clone(),
            puzzle_fingerprint: puzzle.fingerprint(),
            seed: opts.seed,
            started_wall_us: wall_started_us,
        },
    };
    sink.emit(started);

    // Phase 1: enumerate work units. Apply symmetry-breaking and user
    // hints first so the enumeration tree respects pinned cells; otherwise
    // the parallel path silently ignores hints (see lib.rs::run for the
    // single-threaded counterpart).
    let mut enum_state = SearchState::new(puzzle, solver, opts);
    let mut enum_null = eternity2_events::NullSink;
    if let Err(out) = enum_state.apply_symmetry_and_hints(&mut enum_null) {
        return out;
    }
    let cap = (n_threads * 16).max(8);
    let mut units: Vec<Vec<(eternity2_core::Position, u32)>> = Vec::new();
    let mut prefix = Vec::new();
    enum_state.enumerate_units(0, target_depth, &mut prefix, &mut units, cap);

    if units.is_empty() {
        // Either no solution or already complete; fall back to sequential
        // so the caller still gets the proper outcome event.
        return SearchState::new(puzzle, solver, opts).run(sink);
    }

    // Phase 2: dispatch. rayon::scope can't borrow `&mut sink` (not Send).
    // We split into two scoped threads via std::thread::scope:
    //   - A worker thread runs rayon::scope, spawning per-unit tasks.
    //     Workers emit events into a crossbeam channel.
    //   - The main thread drains the channel into `sink` until the
    //     worker thread finishes and drops its tx clone.
    let cancel = Arc::new(AtomicBool::new(false));
    let solutions_found = Arc::new(AtomicU64::new(0));
    let (tx, rx) = crossbeam_channel::unbounded::<WorkerMsg>();
    let stop_on_first = matches!(opts.mode, SolveMode::FirstSolution);
 // shared best-score cutoff for cross-worker BnB prune.
    // Only allocated when MaxScore is set; workers without it use
    // their local `best_score` (`None` arm of `attach_shared_best_score`).
    let shared_best_score: Option<Arc<AtomicU32>> =
        if matches!(opts.objective, Some(Objective::MaxScore)) {
            Some(Arc::new(AtomicU32::new(0)))
        } else {
            None
        };

    let mut all: Vec<WorkerOutcome> = Vec::with_capacity(units.len());

    // Shared deadline for the WHOLE parallel solve. Each worker's
    // SearchState builds its own Clock at construction, which made
    // `opts.time_budget_ms` a per-worker budget — so 100 sequential
    // workers each running 60s would total 100 minutes wall, blowing
    // the bench cell budget. The watchdog below flips `cancel` once
    // wall-time exceeds budget, which the workers see via
    // `sink.should_continue() == false`.
    let bench_start = std::time::Instant::now();
    let budget_ms = opts.time_budget_ms;
    std::thread::scope(|tscope| {
        // Worker thread: runs the rayon scope.
        let cancel_for_workers = cancel.clone();
        let solutions_found_for_workers = solutions_found.clone();
        let tx_for_workers = tx.clone();
        let units_for_workers = units;
        let outcomes_handle = tscope.spawn(move || {
            let outcomes = std::sync::Mutex::new(
                Vec::<WorkerOutcome>::with_capacity(units_for_workers.len()),
            );
            rayon::scope(|scope| {
                for unit in units_for_workers {
                    let tx = tx_for_workers.clone();
                    let cancel = cancel_for_workers.clone();
                    let solutions_found = solutions_found_for_workers.clone();
                    let outcomes = &outcomes;
                    let shared_bs = shared_best_score.clone();
                    scope.spawn(move |_| {
                        let outcome = run_unit(
                            puzzle, solver, opts, &unit,
                            target_depth, cancel.clone(), solutions_found.clone(),
                            stop_on_first, &tx, shared_bs,
                        );
                        let found = matches!(outcome.result, RecurseResult::Found);
                        outcomes.lock().unwrap().push(outcome);
                        if stop_on_first && found {
                            cancel.store(true, Ordering::SeqCst);
                        }
                        drop(tx);
                    });
                }
            });
            drop(tx_for_workers);
            outcomes.into_inner().unwrap()
        });

        // Drop our own tx clone so rx.recv() returns Err once workers finish.
        drop(tx);

        // Deadline watchdog: flip `cancel` once wall time exceeds budget.
        // Polls at 50 ms granularity; cheap and ensures bench cells respect
        // their advertised budget within ~50 ms.
        let cancel_for_watchdog = cancel.clone();
        let watchdog_done = Arc::new(AtomicBool::new(false));
        let watchdog_done_for_thread = watchdog_done.clone();
        let _watchdog = if budget_ms > 0 {
            Some(tscope.spawn(move || {
                let deadline = bench_start + std::time::Duration::from_millis(budget_ms);
                while !watchdog_done_for_thread.load(Ordering::Relaxed) {
                    if std::time::Instant::now() >= deadline {
                        cancel_for_watchdog.store(true, Ordering::SeqCst);
                        return;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            }))
        } else {
            None
        };

        // Main thread: drain events.
        while let Ok(msg) = rx.recv() {
            match msg {
                WorkerMsg::Event(ev) => {
                    sink.emit(ev);
                    if !sink.should_continue() {
                        cancel.store(true, Ordering::SeqCst);
                    }
                }
            }
        }

        all = outcomes_handle.join().unwrap();
        watchdog_done.store(true, Ordering::Relaxed);
    });

    // Aggregate results.
    let mut final_stats = FinalStats::default();
    let mut best_partial: Option<Board> = None;
    let mut best_depth = 0u32;
    let mut solved_board: Option<Board> = None;
    let mut all_solutions: Vec<Board> = Vec::new();
    let mut any_timeout = false;
    let mut any_cancel = false;
 // cross-worker max-by-score aggregation. The shared atom
    // already bounded the per-worker prune, but each worker only saved
    // its OWN best leaf; we still have to pick the winner here.
    let mut best_score: u32 = 0;
    let mut best_score_partial: Option<Board> = None;
    let max_score_objective = matches!(opts.objective, Some(Objective::MaxScore));
    for o in all.drain(..) {
        final_stats.nodes += o.stats.nodes;
        final_stats.backtracks += o.stats.backtracks;
        final_stats.propagations += o.stats.propagations;
        final_stats.domain_wipeouts += o.stats.domain_wipeouts;
        if o.stats.max_depth_seen > final_stats.max_depth_seen {
            final_stats.max_depth_seen = o.stats.max_depth_seen;
        }
        final_stats.solutions_found += o.stats.solutions_found;
        if o.best_depth > best_depth {
            best_depth = o.best_depth;
            best_partial = o.best_partial.clone();
        }
        if max_score_objective && o.best_score > best_score {
            best_score = o.best_score;
            best_score_partial = o.best_score_partial.clone();
        }
        match o.result {
            RecurseResult::Found => {
                if solved_board.is_none() {
                    solved_board = o.solution.clone();
                }
                if let Some(b) = o.solution { all_solutions.push(b); }
            }
            RecurseResult::TimedOut => any_timeout = true,
            RecurseResult::Cancelled => any_cancel = true,
            RecurseResult::Exhausted => {}
        }
    }
    final_stats.time_ms = (now_micros().saturating_sub(wall_started_us)) / 1000;

 // under MaxScore, the best leaf observed across workers is
    // the canonical answer; surface it as Solved before checking
    // FirstSolution / TimedOut / Cancelled paths. Falls through if no
    // worker reached a leaf.
    if max_score_objective {
        if let Some(board) = best_score_partial.clone() {
            sink.emit(SolverEvent {
                schema_version: 1, solver_run_id: opts.solver_run_id,
                node_id: 0, depth: 0, timestamp_us: final_stats.time_ms * 1000,
                body: EventBody::Solved { board: board.clone(), final_stats },
            });
            return SolveOutcome::Solved(board);
        }
        // No leaf reached. Fall through to the depth-based paths below
        // (TimedOut/Cancelled/Exhausted) so the caller still gets a
        // usable partial.
    }

    // Decide outcome event + return.
    if let Some(board) = solved_board {
        if stop_on_first {
            sink.emit(SolverEvent {
                schema_version: 1, solver_run_id: opts.solver_run_id,
                node_id: 0, depth: 0, timestamp_us: final_stats.time_ms * 1000,
                body: EventBody::Solved { board: board.clone(), final_stats },
            });
            return SolveOutcome::Solved(board);
        }
        sink.emit(SolverEvent {
            schema_version: 1, solver_run_id: opts.solver_run_id,
            node_id: 0, depth: 0, timestamp_us: final_stats.time_ms * 1000,
            body: EventBody::Exhausted { final_stats, solutions_found: all_solutions.len() as u64 },
        });
        return SolveOutcome::AllSolutions(all_solutions);
    }
    if any_timeout {
        let best = best_partial.unwrap_or_else(|| Board::empty(puzzle));
        sink.emit(SolverEvent {
            schema_version: 1, solver_run_id: opts.solver_run_id,
            node_id: 0, depth: 0, timestamp_us: final_stats.time_ms * 1000,
            body: EventBody::TimedOut { final_stats, best_partial: best.clone(), best_depth },
        });
        return SolveOutcome::TimedOut { best_partial: best, best_depth };
    }
    if any_cancel {
        let best = best_partial.unwrap_or_else(|| Board::empty(puzzle));
        sink.emit(SolverEvent {
            schema_version: 1, solver_run_id: opts.solver_run_id,
            node_id: 0, depth: 0, timestamp_us: final_stats.time_ms * 1000,
            body: EventBody::Cancelled {
                final_stats, best_partial: best.clone(), best_depth,
                solutions_so_far: Vec::new(),
            },
        });
        return SolveOutcome::Cancelled {
            best_partial: best, best_depth, solutions_so_far: Vec::new(),
        };
    }
    let _ = solutions_found;
    sink.emit(SolverEvent {
        schema_version: 1, solver_run_id: opts.solver_run_id,
        node_id: 0, depth: 0, timestamp_us: final_stats.time_ms * 1000,
        body: EventBody::Exhausted { final_stats, solutions_found: 0 },
    });
    SolveOutcome::Exhausted
}

enum WorkerMsg {
    Event(SolverEvent),
}

struct WorkerOutcome {
    result: RecurseResult,
    solution: Option<Board>,
    best_partial: Option<Board>,
    best_depth: u32,
    stats: FinalStats,
 /// highest matched-edge count this worker saw on a leaf.
    /// 0 when no leaf was reached or MaxScore is off.
    best_score: u32,
 /// board snapshot at the moment the worker observed its
    /// `best_score`. None when no leaf reached.
    best_score_partial: Option<Board>,
}

// Replay the prefix and run recurse() from depth=prefix.len().
fn run_unit(
    puzzle: &Puzzle,
    solver: &EngineSolver,
    opts: &SolveOpts,
    prefix: &[(eternity2_core::Position, u32)],
    target_depth: u32,
    cancel: Arc<AtomicBool>,
    solutions_found: Arc<AtomicU64>,
    stop_on_first: bool,
    tx: &Sender<WorkerMsg>,
    shared_best_score: Option<Arc<AtomicU32>>,
) -> WorkerOutcome {
    let mut state = SearchState::new(puzzle, solver, opts);
    if let Some(atom) = shared_best_score {
        state.attach_shared_best_score(atom);
    }
    // Replay placements. Each placement uses place_and_propagate against
    // a sink that channels events back to the main thread.
    let mut sink = ChannelSink {
        tx: tx.clone(),
        cancel: cancel.clone(),
        solutions_found: solutions_found.clone(),
        stop_on_first,
    };

    // Apply symmetry-breaking and user hints in the worker's fresh state
    // BEFORE replaying the prefix, mirroring the enum_state setup. If
    // either fails the worker bails as Exhausted.
    if state.apply_symmetry_and_hints(&mut sink).is_err() {
        return WorkerOutcome {
            result: RecurseResult::Exhausted,
            solution: None,
            best_partial: None,
            best_depth: 0,
            stats: state.stats,
            best_score: 0,
            best_score_partial: None,
        };
    }

    for (depth, &(pos, row_id)) in prefix.iter().enumerate() {
        if state.placed[pos as usize].is_some() {
            // Shouldn't happen with our enumeration, but treat as wipeout.
            return WorkerOutcome {
                result: RecurseResult::Exhausted,
                solution: None,
                best_partial: None,
                best_depth: depth as u32,
                stats: state.stats,
                best_score: 0,
                best_score_partial: None,
            };
        }
        // Clear domain bits at `pos` the way the main recursion does, so
        // place_and_propagate sees an empty domain at this position.
        let base = (pos as usize) * state.words_per_pos;
        for w in &mut state.domain_bits[base..base + state.words_per_pos] { *w = 0; }
        match state.place_and_propagate(&mut sink, depth as u32, pos, row_id) {
            PropagationOutcome::Ok { .. } => {}
            PropagationOutcome::Wipeout { .. } => {
                return WorkerOutcome {
                    result: RecurseResult::Exhausted,
                    solution: None,
                    best_partial: None,
                    best_depth: depth as u32,
                    stats: state.stats,
                    best_score: 0,
                    best_score_partial: None,
                };
            }
        }
    }

    let mut solutions: Vec<Board> = Vec::new();
    let res = state.recurse(&mut sink, target_depth, &mut solutions);
    if matches!(res, RecurseResult::Found) {
        solutions_found.fetch_add(1, Ordering::SeqCst);
    }
    WorkerOutcome {
        result: res,
        solution: solutions.into_iter().next(),
        best_partial: state.best_partial.clone(),
        best_depth: state.best_depth,
        stats: state.stats,
        best_score: state.best_score,
        best_score_partial: state.best_score_partial.clone(),
    }
}

struct ChannelSink {
    tx: Sender<WorkerMsg>,
    cancel: Arc<AtomicBool>,
    solutions_found: Arc<AtomicU64>,
    stop_on_first: bool,
}

impl EventSink for ChannelSink {
    fn emit(&mut self, event: SolverEvent) {
        // Skip the per-worker Started — the coordinator emits one for
        // the whole parallel run. Drop terminal events too; the
        // coordinator synthesizes a single final event after aggregating.
        match &event.body {
            EventBody::Started { .. }
            | EventBody::Solved { .. }
            | EventBody::Exhausted { .. }
            | EventBody::TimedOut { .. }
            | EventBody::Cancelled { .. } => return,
            _ => {}
        }
        let _ = self.tx.send(WorkerMsg::Event(event));
    }
    fn should_continue(&self) -> bool {
        if self.cancel.load(Ordering::Relaxed) { return false; }
        if self.stop_on_first && self.solutions_found.load(Ordering::Relaxed) > 0 {
            return false;
        }
        true
    }
}

// Legacy heuristic from solvers/v3/parallel/parallel_dlx_solver.cpp,
// adapted for our puzzle sizes (we benchmark up to 12×12, not 16×16).
fn auto_split_depth(puzzle: &Puzzle, n_threads: usize) -> u32 {
    let positions = puzzle.cell_count() as usize;
    if positions >= 144 {
        // 12×12 and up
        if n_threads <= 4 { 5 } else if n_threads <= 8 { 6 } else { 7 }
    } else if positions >= 64 {
        // 8×8 to 11×11
        if n_threads <= 4 { 4 } else if n_threads <= 8 { 5 } else { 6 }
    } else if positions >= 25 {
        // 5×5 to 7×7
        if n_threads <= 4 { 3 } else { 4 }
    } else {
        // 2×2 to 4×4 — parallelism doesn't help; very shallow split.
        2
    }
}

fn now_micros() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_micros() as u64)
        .unwrap_or(0)
}
