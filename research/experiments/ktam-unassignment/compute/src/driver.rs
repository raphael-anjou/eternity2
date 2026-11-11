//! Rungs, instances, runs and raw rows.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use eternity2_engine::rotated;
use hard_ladder::cli::load_instances;
use hard_ladder::instance::{generate, owen_criterion, FRAME_COLOURS};
use hard_ladder::{verify_board, Ladder, LadderInstance, LadderSolver, NodeCounter, Outcome, Role, RunRecord};
use serde::{Deserialize, Serialize};

use crate::search::{Rule, Unassign};

/// Event budget E (PREREG.md section 4).
pub const EVENT_BUDGET: u64 = 1_000_000;
/// Run seed for the method arms.
pub const RUN_SEED: u64 = 1;

#[must_use]
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// (rung name, n)
pub const RUNGS: [(&str, u8); 3] = [("R6", 6), ("R7", 7), ("R8", 8)];

#[must_use]
pub fn rung_n(rung: &str) -> Option<u8> {
    RUNGS.iter().find(|r| r.0 == rung).map(|r| r.1)
}

/// The R8 instance (8x8, c = 9): the ladder's generator call and scramble.
#[must_use]
pub fn r8_instance(role: Role, seed: u32) -> LadderInstance {
    let (n, c) = (8u8, 9u8);
    let (puzzle, planted) = generate(n, c, seed);
    LadderInstance {
        id: format!("n{n}-c{c}-s{seed}"),
        ladder: "8-c9".into(),
        role: role.name().into(),
        n,
        c,
        seed,
        owen_criterion: owen_criterion(n),
        generator: format!(
            "generate_solved_framed_bc({n}, {}, {seed}, true, {FRAME_COLOURS}) + XorShift({seed} ^ 0xA5A5A5A5) scramble",
            c + FRAME_COLOURS
        ),
        puzzle,
        planted,
    }
}

#[must_use]
pub fn r8_path(role: Role) -> PathBuf {
    root().join("instances").join(format!("8-c9-{}.jsonl", role.name()))
}

#[must_use]
pub fn r8_jsonl(role: Role) -> String {
    role.seeds().into_iter().map(|s| serde_json::to_string(&r8_instance(role, s)).expect("json") + "\n").collect()
}

pub fn instances(rung: &str, role: Role) -> Result<Vec<LadderInstance>, String> {
    let n = rung_n(rung).ok_or_else(|| format!("unknown rung {rung}"))?;
    if rung == "R8" {
        let text = fs::read_to_string(r8_path(role)).map_err(|e| format!("{}: {e}", r8_path(role).display()))?;
        text.lines().filter(|l| !l.trim().is_empty()).map(|l| serde_json::from_str(l).map_err(|e| e.to_string())).collect()
    } else {
        Ok(load_instances(Ladder::Primary, role)?.into_iter().filter(|i| i.n == n).collect())
    }
}

/// One raw row. `status` follows the ladder at the node cap; `within_e` is
/// the primary (event-budget) outcome.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Row {
    pub arm: String,
    pub config: String,
    pub instance: String,
    pub rung: String,
    pub role: String,
    pub n: u8,
    pub c: u8,
    pub seed: u32,
    pub node_cap: u64,
    pub event_budget: u64,
    pub status: String,
    pub nodes: u64,
    /// placements + removals when the run stopped (method arms) or
    /// 2 * nodes - n^2 for a solved DFS run; None for a censored DFS run.
    pub events: Option<u64>,
    pub within_e: bool,
    pub removals: Option<u64>,
    pub dead_ends: Option<u64>,
    pub removals_planted: Option<u64>,
    pub placements_planted: Option<u64>,
    pub backtracks: u64,
    pub propagation: u64,
    pub inconsistent: u64,
    pub equals_planted: bool,
    pub wall_ms: f64,
    pub error: Option<String>,
}

fn base_row(arm: &str, config: &str, inst: &LadderInstance, rung: &str, cap: u64) -> Row {
    Row {
        arm: arm.into(),
        config: config.into(),
        instance: inst.id.clone(),
        rung: rung.into(),
        role: inst.role.clone(),
        n: inst.n,
        c: inst.c,
        seed: inst.seed,
        node_cap: cap,
        event_budget: EVENT_BUDGET,
        status: String::new(),
        nodes: 0,
        events: None,
        within_e: false,
        removals: None,
        dead_ends: None,
        removals_planted: None,
        placements_planted: None,
        backtracks: 0,
        propagation: 0,
        inconsistent: 0,
        equals_planted: false,
        wall_ms: 0.0,
        error: None,
    }
}

/// Method arm on one instance. The planted board is read only after `solve` returns.
#[must_use]
pub fn run_method(arm: &str, rule: Rule, inst: &LadderInstance, rung: &str, cap: u64) -> Row {
    let mut row = base_row(arm, &rule.name(), inst, rung, cap);
    let mut counter = NodeCounter::new(cap);
    let mut solver = Unassign::new(rule, RUN_SEED * 1_000_003 + u64::from(inst.seed));
    let puzzle = inst.puzzle.clone();
    let t = Instant::now();
    let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| solver.solve(&puzzle, &mut counter)));
    row.wall_ms = t.elapsed().as_secs_f64() * 1000.0;
    let cells = usize::from(inst.n) * usize::from(inst.n);
    match out {
        Err(_) => {
            row.status = "error".into();
            row.error = Some("panic".into());
        }
        Ok(Outcome::Exhausted) => {
            row.status = "error".into();
            row.error = Some("exhausted".into());
        }
        Ok(Outcome::CapReached) => row.status = "censored".into(),
        Ok(Outcome::Solved(board)) => match verify_board(&inst.puzzle, &board) {
            Err(e) => {
                row.status = "error".into();
                row.error = Some(format!("unverified board: {e}"));
            }
            Ok(()) => {
                row.status = "solved".into();
                row.equals_planted = board == inst.planted;
            }
        },
    }
    let s = &solver.stats;
    row.nodes = counter.nodes();
    let events = counter.nodes() + s.removals;
    debug_assert_eq!(events, 2 * counter.nodes() - s.filled_at_end);
    row.events = Some(events);
    row.within_e = row.status == "solved" && events <= EVENT_BUDGET;
    row.removals = Some(s.removals);
    row.dead_ends = Some(s.dead_ends);
    // A value counts as the planted placement when it is the planted piece in
    // the planted orientation (equal edges, so symmetric rotations match).
    let planted_sum = |hist: &[u32]| -> u64 {
        (0..cells)
            .map(|cell| {
                let pv = inst.planted[cell];
                let (p, want) = ((pv / 4) as usize, rotated(inst.puzzle.pieces[(pv / 4) as usize], (pv % 4) as u8));
                (0..4u8)
                    .filter(|&r| rotated(inst.puzzle.pieces[p], r) == want)
                    .map(|r| u64::from(hist[cell * 4 * cells + p * 4 + usize::from(r)]))
                    .sum::<u64>()
            })
            .sum()
    };
    row.removals_planted = Some(planted_sum(&s.removed_at));
    row.placements_planted = Some(planted_sum(&s.placed_at));
    row.backtracks = counter.backtracks;
    row.propagation = counter.propagation;
    row.inconsistent = counter.inconsistent;
    row
}

/// Ladder registry baseline row converted to events.
#[must_use]
pub fn from_ladder(arm: &str, rec: &RunRecord, rung: &str) -> Row {
    let mut row = Row {
        arm: arm.into(),
        config: rec.solver.clone(),
        instance: rec.instance.clone(),
        rung: rung.into(),
        role: rec.role.clone(),
        n: rec.n,
        c: rec.c,
        seed: rec.seed,
        node_cap: rec.cap,
        event_budget: EVENT_BUDGET,
        status: rec.status.clone(),
        nodes: rec.nodes,
        events: None,
        within_e: false,
        removals: None,
        dead_ends: None,
        removals_planted: None,
        placements_planted: None,
        backtracks: rec.backtracks,
        propagation: rec.propagation,
        inconsistent: rec.inconsistent,
        equals_planted: rec.equals_planted,
        wall_ms: rec.wall_ms,
        error: rec.error.clone(),
    };
    if rec.status == "solved" {
        let cells = u64::from(rec.n) * u64::from(rec.n);
        let ev = 2 * rec.nodes - cells;
        row.events = Some(ev);
        row.within_e = ev <= EVENT_BUDGET;
    }
    row
}

pub fn write_rows(out: &Path, rows: &[Row]) -> Result<(), String> {
    if let Some(d) = out.parent() {
        fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    let mut f = fs::File::create(out).map_err(|e| e.to_string())?;
    for r in rows {
        f.write_all((serde_json::to_string(r).expect("json") + "\n").as_bytes()).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Runs jobs (rule, instance) on `threads` workers; results in input order.
#[must_use]
pub fn run_jobs(arm: &str, jobs: &[(Rule, &LadderInstance, &str)], cap: impl Fn(u8) -> u64 + Sync, threads: usize) -> Vec<Row> {
    let next = AtomicUsize::new(0);
    let out: Mutex<Vec<Option<Row>>> = Mutex::new(vec![None; jobs.len()]);
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| loop {
                let k = next.fetch_add(1, Ordering::Relaxed);
                let Some(&(rule, inst, rung)) = jobs.get(k) else { break };
                let row = run_method(arm, rule, inst, rung, cap(inst.n));
                eprintln!(
                    "{} {} {} {} nodes={} events={:?} wall_ms={:.0}",
                    row.config, row.rung, row.instance, row.status, row.nodes, row.events, row.wall_ms
                );
                out.lock().expect("lock")[k] = Some(row);
            });
        }
    });
    out.into_inner().expect("lock").into_iter().map(|r| r.expect("ran")).collect()
}
