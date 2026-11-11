//! Runs a solver on instances under a cap and records one row per instance.

use std::panic::{self, AssertUnwindSafe};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::counter::NodeCounter;
use crate::instance::{verify_board, LadderInstance};
use crate::solver::{Outcome, SolverEntry};

/// One raw result row. `status` is `solved`, `censored` (cap reached; `nodes`
/// is then the cap, a right-censored observation) or `error`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunRecord {
    pub solver: String,
    pub run_seed: u64,
    pub instance: String,
    pub ladder: String,
    pub role: String,
    pub n: u8,
    pub c: u8,
    pub seed: u32,
    pub cap: u64,
    pub status: String,
    pub nodes: u64,
    pub nodes_counted: u64,
    pub native_nodes: Option<u64>,
    pub backtracks: u64,
    pub fit_tests: u64,
    pub propagation: u64,
    pub inconsistent: u64,
    pub equals_planted: bool,
    pub wall_ms: f64,
    pub error: Option<String>,
}

#[must_use]
pub fn run_one(entry: &SolverEntry, inst: &LadderInstance, cap: u64) -> RunRecord {
    let mut counter = NodeCounter::new(cap);
    let puzzle = inst.puzzle.clone();
    let t = Instant::now();
    let out = panic::catch_unwind(AssertUnwindSafe(|| {
        let mut solver = (entry.make)(entry.run_seed);
        solver.solve(&puzzle, &mut counter)
    }));
    let wall_ms = t.elapsed().as_secs_f64() * 1000.0;
    let mut equals_planted = false;
    let (status, error) = match out {
        Err(p) => {
            let msg = p
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_string()))
                .unwrap_or_else(|| "panic".into());
            ("error", Some(msg))
        }
        Ok(Outcome::Exhausted) => ("error", Some("exhausted on a planted instance".into())),
        Ok(Outcome::CapReached) => ("censored", None),
        Ok(Outcome::Solved(board)) => match verify_board(&inst.puzzle, &board) {
            Err(e) => ("error", Some(format!("unverified board: {e}"))),
            Ok(()) if counter.nodes() > cap => ("censored", None),
            Ok(()) => {
                equals_planted = board == inst.planted;
                ("solved", None)
            }
        },
    };
    let nodes = if status == "censored" { cap } else { counter.nodes() };
    RunRecord {
        solver: entry.name.into(),
        run_seed: entry.run_seed,
        instance: inst.id.clone(),
        ladder: inst.ladder.clone(),
        role: inst.role.clone(),
        n: inst.n,
        c: inst.c,
        seed: inst.seed,
        cap,
        status: status.into(),
        nodes,
        nodes_counted: counter.nodes(),
        native_nodes: counter.native_nodes,
        backtracks: counter.backtracks,
        fit_tests: counter.fit_tests,
        propagation: counter.propagation,
        inconsistent: counter.inconsistent,
        equals_planted,
        wall_ms,
        error,
    }
}

/// Runs every instance on `threads` workers (largest boards first for load
/// balance); results come back in input order.
#[must_use]
pub fn run_all(entry: &SolverEntry, insts: &[LadderInstance], cap: u64, threads: usize) -> Vec<RunRecord> {
    let mut order: Vec<usize> = (0..insts.len()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(insts[i].n));
    let next = AtomicUsize::new(0);
    let out: Mutex<Vec<Option<RunRecord>>> = Mutex::new(vec![None; insts.len()]);
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| loop {
                let k = next.fetch_add(1, Ordering::Relaxed);
                let Some(&i) = order.get(k) else { break };
                let rec = run_one(entry, &insts[i], cap);
                eprintln!(
                    "{} {} {} nodes={} wall_ms={:.0}",
                    rec.solver, rec.instance, rec.status, rec.nodes, rec.wall_ms
                );
                out.lock().expect("lock")[i] = Some(rec);
            });
        }
    });
    out.into_inner().expect("lock").into_iter().map(|r| r.expect("ran")).collect()
}
