//! Runner for hall-on-mrv (see `../PREREG.md`).
//!
//! hall_on_mrv run --rung r6|r7|r8 --role test|calibration --arms a,b,.. [--reps K]
//!                 [--cap 1000000] [--guard 600] [--threads 2] --out F.jsonl
//! hall_on_mrv trace-check --rung R --role ROLE --arm hall-v1|hall-v2 [--cap N] --out F.jsonl
//! hall_on_mrv list
//!
//! `run` interleaves arms on each instance: jobs are ordered repetition-major,
//! then instance, then arm, with the arm order rotated by (instance + rep) so
//! no arm always runs first; `--threads` workers take jobs in that order. Each
//! row is written as soon as its job finishes.

use std::fs;
use std::io::Write;
use std::panic::{self, AssertUnwindSafe};
use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use backdoor_branching::{BbSolver, Rule};
use hall_on_mrv::{subtree_removal, Hall, HallMrv, HallStats, OFF};
use hard_ladder::cli::load_instances;
use hard_ladder::{verify_board, Ladder, LadderInstance, LadderSolver, NodeCounter, Outcome, Role};
use mrv_baseline::scb::GuardScb;
use mrv_baseline::{MrvSolver, Order};
use serde_json::{json, Value};

const ARMS: [(&str, &str); 10] = [
    (
        "bb-mrv",
        "decision baseline: A16 BbSolver Rule::Mrv, unchanged code",
    ),
    (
        "hall-off",
        "implementation check: this crate's solver, Hall off (must equal bb-mrv)",
    ),
    ("hall-v1", "V1: Hall on frontier cells, every node, prunes"),
    (
        "hall-v2",
        "V2: Hall on every unassigned cell, every 4th propagation, prunes",
    ),
    ("cost-v1", "cost control: V1 check computed, never prunes"),
    ("cost-v2", "cost control: V2 check computed, never prunes"),
    (
        "mrv-rand-1",
        "order control: bb-mrv with scb random value order, run seed 1 (A22 mrv-rand)",
    ),
    ("mrv-rand-2", "order control: run seed 2"),
    ("mrv-rand-3", "order control: run seed 3"),
    (
        "scb",
        "adopted R6/R7 baseline: scb-gacolor_ac3_random, run seed 1, guarded wrapper (A22)",
    ),
];

const V1: Hall = Hall {
    scope: 1,
    period: 1,
    prune: true,
};
const V2: Hall = Hall {
    scope: 2,
    period: 4,
    prune: true,
};

fn hall_of(arm: &str) -> Option<Hall> {
    match arm {
        "hall-off" => Some(OFF),
        "hall-v1" => Some(V1),
        "hall-v2" => Some(V2),
        "cost-v1" => Some(Hall { prune: false, ..V1 }),
        "cost-v2" => Some(Hall { prune: false, ..V2 }),
        _ => None,
    }
}

fn scb_orig() -> Box<dyn eternity2_solver_trait::Solver> {
    Box::new(eternity2_solver_engine::EngineSolver::gacolor_ac3_random())
}

fn arg<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
}

fn instances(rung: &str, role: Role) -> Result<Vec<LadderInstance>, String> {
    let take = |l: Ladder,
                n: Option<u8>,
                seeds: Option<(u32, u32)>|
     -> Result<Vec<LadderInstance>, String> {
        let mut v = load_instances(l, role)?;
        if let Some(n) = n {
            v.retain(|i| i.n == n);
        }
        if let Some((a, b)) = seeds {
            v.retain(|i| (a..=b).contains(&i.seed));
        }
        Ok(v)
    };
    match (rung, role) {
        ("r6", _) => take(Ladder::Primary, Some(6), None),
        ("r7", _) => take(Ladder::Primary, Some(7), None),
        ("r8", Role::Test) => {
            let mut v = take(Ladder::EightC10, None, Some((21, 30)))?;
            v.extend(take(Ladder::EightC10Fresh, None, Some((31, 50)))?);
            Ok(v)
        }
        ("r8", Role::Calibration) => take(Ladder::EightC10, None, None),
        _ => Err(format!("bad rung {rung}")),
    }
}

fn fnv(xs: &[i32]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for x in xs {
        for b in x.to_le_bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    h
}

/// One run, with `hard_ladder::run_one`'s status rules, plus Hall counters.
fn run_job(arm: &str, inst: &LadderInstance, cap: u64, guard: Duration) -> Value {
    let mut counter = NodeCounter::new(cap);
    let puzzle = inst.puzzle.clone();
    let t = Instant::now();
    let mut hs: Option<HallStats> = None;
    let out = panic::catch_unwind(AssertUnwindSafe(|| {
        if let Some(h) = hall_of(arm) {
            let mut s = HallMrv::new(h, guard);
            let o = s.solve(&puzzle, &mut counter);
            hs = Some(s.stats.clone());
            return o;
        }
        let mut s: Box<dyn LadderSolver> = match arm {
            "bb-mrv" => Box::new(BbSolver {
                rule: Rule::Mrv,
                guard,
            }),
            "mrv-rand-1" | "mrv-rand-2" | "mrv-rand-3" => {
                let seed: u64 = arm[9..].parse().expect("seed");
                Box::new(MrvSolver {
                    order: Order::Free,
                    random: Some(seed),
                    hs: true,
                    guard,
                })
            }
            "scb" => Box::new(GuardScb {
                make: scb_orig,
                seed: 1,
                guard,
            }),
            _ => panic!("unknown arm {arm}"),
        };
        s.solve(&puzzle, &mut counter)
    }));
    let wall_ms = t.elapsed().as_secs_f64() * 1000.0;
    let mut board_hash = None;
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
                board_hash = Some(format!("{:016x}", fnv(&board)));
                ("solved", None)
            }
        },
    };
    let nodes = if status == "censored" {
        cap
    } else {
        counter.nodes()
    };
    let guard_stop = status == "censored" && counter.nodes() < cap;
    let mut v = json!({
        "arm": arm, "instance": inst.id, "ladder": inst.ladder, "role": inst.role,
        "n": inst.n, "c": inst.c, "seed": inst.seed, "cap": cap, "status": status,
        "nodes": nodes, "nodes_counted": counter.nodes(), "backtracks": counter.backtracks,
        "propagation": counter.propagation, "inconsistent": counter.inconsistent,
        "equals_planted": equals_planted, "board_hash": board_hash, "wall_ms": wall_ms,
        "guard_stop": guard_stop, "error": error,
    });
    if let Some(h) = hs {
        v["hall_checks"] = json!(h.checks);
        v["hall_fails"] = json!(h.fails);
        v["seq_hash"] = json!(format!("{:016x}", h.seq_hash));
    }
    v
}

fn run(args: &[String]) -> Result<(), String> {
    let rung = arg(args, "--rung").ok_or("--rung required")?;
    let role = Role::parse(arg(args, "--role").unwrap_or("test")).ok_or("bad --role")?;
    let arms: Vec<String> = arg(args, "--arms")
        .ok_or("--arms required")?
        .split(',')
        .map(str::to_string)
        .collect();
    for a in &arms {
        if !ARMS.iter().any(|(n, _)| n == a) {
            return Err(format!("unknown arm {a}"));
        }
    }
    let reps: usize = arg(args, "--reps")
        .unwrap_or("1")
        .parse()
        .map_err(|_| "bad --reps")?;
    let cap: u64 = arg(args, "--cap")
        .unwrap_or("1000000")
        .parse()
        .map_err(|_| "bad --cap")?;
    let guard = Duration::from_secs(
        arg(args, "--guard")
            .unwrap_or("600")
            .parse()
            .map_err(|_| "bad --guard")?,
    );
    let threads: usize = arg(args, "--threads")
        .unwrap_or("2")
        .parse()
        .map_err(|_| "bad --threads")?;
    let out = arg(args, "--out").ok_or("--out required")?;
    let insts = instances(rung, role)?;
    let mut jobs = Vec::new();
    for rep in 0..reps {
        for (i, _) in insts.iter().enumerate() {
            for k in 0..arms.len() {
                jobs.push((rep, i, (k + i + rep) % arms.len()));
            }
        }
    }
    eprintln!(
        "{rung} {}: {} instances x {} arms x {reps} reps = {} jobs, cap {cap}, {threads} threads",
        role.name(),
        insts.len(),
        arms.len(),
        jobs.len()
    );
    if let Some(d) = std::path::Path::new(out).parent() {
        fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    let f = Mutex::new(fs::File::create(out).map_err(|e| e.to_string())?);
    let next = AtomicUsize::new(0);
    let t0 = Instant::now();
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| loop {
                let j = next.fetch_add(1, Ordering::Relaxed);
                let Some(&(rep, i, a)) = jobs.get(j) else {
                    break;
                };
                let start_s = t0.elapsed().as_secs_f64();
                let mut v = run_job(&arms[a], &insts[i], cap, guard);
                v["rep"] = json!(rep);
                v["job"] = json!(j);
                v["start_s"] = json!(start_s);
                eprintln!(
                    "{} {} rep{} {} nodes={} wall_ms={:.0}",
                    arms[a],
                    insts[i].id,
                    rep,
                    v["status"],
                    v["nodes"],
                    v["wall_ms"].as_f64().unwrap_or(0.0)
                );
                let mut f = f.lock().expect("lock");
                writeln!(f, "{v}").expect("write");
                f.flush().expect("flush");
            });
        }
    });
    Ok(())
}

/// Runs hall-off and a Hall arm with traces and checks subtree removal.
fn trace_check(args: &[String]) -> Result<(), String> {
    let rung = arg(args, "--rung").ok_or("--rung required")?;
    let role = Role::parse(arg(args, "--role").unwrap_or("calibration")).ok_or("bad --role")?;
    let arm = arg(args, "--arm").ok_or("--arm required")?;
    let h = hall_of(arm)
        .filter(|h| h.prune)
        .ok_or("--arm must be a pruning Hall arm")?;
    let cap: u64 = arg(args, "--cap")
        .unwrap_or("1000000")
        .parse()
        .map_err(|_| "bad --cap")?;
    let out = arg(args, "--out").ok_or("--out required")?;
    let mut f = fs::File::create(out).map_err(|e| e.to_string())?;
    for inst in instances(rung, role)? {
        let guard = Duration::from_secs(3600);
        let mut base = HallMrv::new(OFF, guard);
        base.trace = Some(Vec::new());
        let mut c1 = NodeCounter::new(cap);
        let o1 = base.solve(&inst.puzzle, &mut c1);
        let mut cand = HallMrv::new(h, guard);
        cand.trace = Some(Vec::new());
        let mut c2 = NodeCounter::new(cap);
        let o2 = cand.solve(&inst.puzzle, &mut c2);
        let (bt, ct) = (
            base.trace.take().unwrap_or_default(),
            cand.trace.take().unwrap_or_default(),
        );
        let res = subtree_removal(&bt, &ct);
        let same_board = match (&o1, &o2) {
            (Outcome::Solved(a), Outcome::Solved(b)) => Some(a == b),
            _ => None,
        };
        let v = json!({
            "arm": arm, "instance": inst.id, "cap": cap,
            "base_nodes": c1.nodes(), "cand_nodes": c2.nodes(),
            "base_solved": matches!(o1, Outcome::Solved(_)), "cand_solved": matches!(o2, Outcome::Solved(_)),
            "same_board": same_board, "subtree_removal_ok": res.is_ok(),
            "skipped": res.as_ref().ok(), "error": res.as_ref().err(),
            "hall_fails": cand.stats.fails,
        });
        eprintln!("{v}");
        writeln!(f, "{v}").map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let res = match args.first().map(String::as_str) {
        Some("run") => run(&args),
        Some("trace-check") => trace_check(&args),
        Some("list") => {
            for (n, d) in ARMS {
                println!("{n:12} {d}");
            }
            Ok(())
        }
        _ => Err("usage: hall_on_mrv run|trace-check|list (see src/main.rs)".into()),
    };
    match res {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
