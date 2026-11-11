//! Exploratory, non-binding (PREREG_AMENDMENTS.md E1): per-thread CPU time
//! next to nodes and native propagation counts, for an equal-CPU comparison.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use hard_ladder::registry::find;
use hard_ladder::{run_one, LadderInstance};
use serde::Serialize;

use crate::driver::run_method;
use crate::search::Rule;

/// CPU time of the calling thread, ms.
#[must_use]
pub fn thread_cpu_ms() -> f64 {
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    // SAFETY: valid pointer to a timespec; CLOCK_THREAD_CPUTIME_ID is supported on Linux.
    let rc = unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut ts) };
    assert_eq!(rc, 0, "clock_gettime");
    ts.tv_sec as f64 * 1000.0 + ts.tv_nsec as f64 / 1e6
}

#[derive(Debug, Clone, Serialize)]
pub struct CpuRow {
    pub arm: String,
    pub config: String,
    pub instance: String,
    pub node_cap: u64,
    pub status: String,
    pub nodes: u64,
    pub removals: Option<u64>,
    pub events: Option<u64>,
    pub propagation: u64,
    pub cpu_ms: f64,
    pub wall_ms: f64,
    pub error: Option<String>,
}

/// `arm` is B1 / B2 (ladder registry) or B3 / V1 / V2 with `rule`.
#[must_use]
pub fn run_cpu(arm: &str, rule: Option<Rule>, insts: &[LadderInstance], rung: &str, cap: u64, threads: usize) -> Vec<CpuRow> {
    let next = AtomicUsize::new(0);
    let out: Mutex<Vec<Option<CpuRow>>> = Mutex::new(vec![None; insts.len()]);
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| loop {
                let k = next.fetch_add(1, Ordering::Relaxed);
                let Some(inst) = insts.get(k) else { break };
                let t0 = thread_cpu_ms();
                let row = if let Some(rule) = rule {
                    let r = run_method(arm, rule, inst, rung, cap);
                    CpuRow {
                        arm: arm.into(),
                        config: r.config,
                        instance: r.instance,
                        node_cap: cap,
                        status: r.status,
                        nodes: r.nodes,
                        removals: r.removals,
                        events: r.events,
                        propagation: r.propagation,
                        cpu_ms: 0.0,
                        wall_ms: r.wall_ms,
                        error: r.error,
                    }
                } else {
                    let name = if arm == "B1" { "scb-gacolor_ac3_random" } else { "hp-dfs" };
                    let entry = find(&[], name).expect("baseline");
                    let r = run_one(entry, inst, cap);
                    CpuRow {
                        arm: arm.into(),
                        config: r.solver,
                        instance: r.instance,
                        node_cap: cap,
                        status: r.status,
                        nodes: r.nodes,
                        removals: None,
                        events: None,
                        propagation: r.propagation,
                        cpu_ms: 0.0,
                        wall_ms: r.wall_ms,
                        error: r.error,
                    }
                };
                let row = CpuRow { cpu_ms: thread_cpu_ms() - t0, ..row };
                eprintln!("{arm} {} {} nodes={} cpu_ms={:.0}", row.instance, row.status, row.nodes, row.cpu_ms);
                out.lock().expect("lock")[k] = Some(row);
            });
        }
    });
    out.into_inner().expect("lock").into_iter().map(|r| r.expect("ran")).collect()
}
