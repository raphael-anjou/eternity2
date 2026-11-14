//! A20 runner. Arms are interleaved per instance (rotating arm order) on a
//! shared job queue, so no arm runs entirely before another.
//!
//! blocker_backjump run --arms a,b,.. --ladder L --role R [--sizes 6,7]
//!     [--seeds 1-30] [--cap 1000000] [--guard 900] [--threads 2]
//!     [--tabu 16] [--run-seed 1] --outdir DIR
//! blocker_backjump list
//!
//! Writes DIR/<arm>.jsonl (hard-ladder `RunRecord` rows) and
//! DIR/<arm>.side.jsonl (bj arms: per-run dead-end, eviction, revisit stats).

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Duration;

use backdoor_branching::{BbSolver, Rule};
use blocker_backjump::{BjSolver, Mode, SIDECAR};
use hard_ladder::cli::load_instances;
use hard_ladder::{run_one, Ladder, LadderSolver, Role, RunRecord, SolverEntry};
use mrv_baseline::scb::GuardScb;
use mrv_baseline::GUARD_SECS;

static TABU: AtomicUsize = AtomicUsize::new(16);

fn guard() -> Duration {
    Duration::from_secs(GUARD_SECS.load(Ordering::Relaxed))
}
fn tabu() -> u64 {
    TABU.load(Ordering::Relaxed) as u64
}
fn scb_orig() -> Box<dyn eternity2_solver_trait::Solver> {
    Box::new(eternity2_solver_engine::EngineSolver::gacolor_ac3_random())
}

fn arms(run_seed: u64) -> Vec<SolverEntry> {
    vec![
        SolverEntry {
            name: "bb-mrv",
            description: "parent and 8x8 c=10 baseline: A16 BbSolver Rule::Mrv, chronological",
            run_seed: 0,
            make: |_| -> Box<dyn LadderSolver> { Box::new(BbSolver { rule: Rule::Mrv, guard: guard() }) },
        },
        SolverEntry {
            name: "scb-gacolor_ac3_random",
            description: "R6/R7 baseline: hard-ladder scb gacolor_ac3_random, run seed 1 (A22 guarded wrapper)",
            run_seed: 1,
            make: |s| -> Box<dyn LadderSolver> { Box::new(GuardScb { make: scb_orig, seed: s, guard: guard() }) },
        },
        SolverEntry {
            name: "bj-blocker",
            description: "candidate: bb-mrv propagation, evict the IMS blocker set of the failed cell",
            run_seed,
            make: |s| -> Box<dyn LadderSolver> {
                Box::new(BjSolver { mode: Mode::Blocker, tabu: tabu(), run_seed: s, guard: guard(), label: "bj-blocker" })
            },
        },
        SolverEntry {
            name: "bj-random",
            description: "matched twin: same bookkeeping, evicts |B| uniformly random decisions",
            run_seed,
            make: |s| -> Box<dyn LadderSolver> {
                Box::new(BjSolver { mode: Mode::Random, tabu: tabu(), run_seed: s, guard: guard(), label: "bj-random" })
            },
        },
    ]
}

fn arg<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
}

fn run(args: &[String]) -> Result<(), String> {
    let run_seed: u64 = arg(args, "--run-seed")
        .unwrap_or("1")
        .parse()
        .map_err(|_| "bad --run-seed")?;
    let all = arms(run_seed);
    let names: Vec<&str> = arg(args, "--arms")
        .ok_or("--arms required")?
        .split(',')
        .collect();
    let entries: Vec<&SolverEntry> = names
        .iter()
        .map(|n| {
            all.iter()
                .find(|e| e.name == *n)
                .ok_or_else(|| format!("unknown arm {n}"))
        })
        .collect::<Result<_, _>>()?;
    let ladder =
        Ladder::parse(arg(args, "--ladder").ok_or("--ladder required")?).ok_or("bad --ladder")?;
    let role = Role::parse(arg(args, "--role").unwrap_or("test")).ok_or("bad --role")?;
    let cap: u64 = arg(args, "--cap")
        .unwrap_or("1000000")
        .parse()
        .map_err(|_| "bad --cap")?;
    let g: u64 = arg(args, "--guard")
        .unwrap_or("900")
        .parse()
        .map_err(|_| "bad --guard")?;
    GUARD_SECS.store(g, Ordering::Relaxed);
    let t: usize = arg(args, "--tabu")
        .unwrap_or("16")
        .parse()
        .map_err(|_| "bad --tabu")?;
    TABU.store(t, Ordering::Relaxed);
    let threads: usize = arg(args, "--threads")
        .unwrap_or("2")
        .parse()
        .map_err(|_| "bad --threads")?;
    let outdir = arg(args, "--outdir").ok_or("--outdir required")?;
    let mut insts = load_instances(ladder, role)?;
    if let Some(s) = arg(args, "--sizes") {
        let sizes: Vec<u8> = s
            .split(',')
            .map(|x| x.parse().map_err(|_| "bad --sizes"))
            .collect::<Result<_, _>>()?;
        insts.retain(|i| sizes.contains(&i.n));
    }
    if let Some(s) = arg(args, "--seeds") {
        let (a, b) = s.split_once('-').ok_or("--seeds lo-hi")?;
        let (a, b): (u32, u32) = (
            a.parse().map_err(|_| "bad seed")?,
            b.parse().map_err(|_| "bad seed")?,
        );
        insts.retain(|i| (a..=b).contains(&i.seed));
    }
    // Interleaved job list: instance i runs its arms in an order rotated by i.
    let k = entries.len();
    let mut jobs = Vec::new();
    for i in 0..insts.len() {
        for j in 0..k {
            jobs.push((i, (i + j) % k));
        }
    }
    eprintln!("arms {names:?} on {} instances, cap {cap}, guard {g} s, tabu {t}, run seed {run_seed}, {threads} threads", insts.len());
    let next = AtomicUsize::new(0);
    let out: Mutex<HashMap<usize, Vec<Option<RunRecord>>>> =
        Mutex::new((0..k).map(|a| (a, vec![None; insts.len()])).collect());
    std::thread::scope(|sc| {
        for _ in 0..threads.max(1) {
            sc.spawn(|| loop {
                let q = next.fetch_add(1, Ordering::Relaxed);
                let Some(&(i, a)) = jobs.get(q) else { break };
                let rec = run_one(entries[a], &insts[i], cap);
                eprintln!(
                    "{} {} {} nodes={} wall_ms={:.0}",
                    rec.solver, rec.instance, rec.status, rec.nodes, rec.wall_ms
                );
                out.lock().expect("lock").get_mut(&a).expect("arm")[i] = Some(rec);
            });
        }
    });
    fs::create_dir_all(outdir).map_err(|e| e.to_string())?;
    let out = out.into_inner().expect("lock");
    let side = SIDECAR.lock().expect("sidecar").clone();
    for (a, e) in entries.iter().enumerate() {
        let mut f =
            fs::File::create(format!("{outdir}/{}.jsonl", e.name)).map_err(|e| e.to_string())?;
        for r in out[&a].iter().map(|r| r.as_ref().expect("ran")) {
            writeln!(
                f,
                "{}",
                serde_json::to_string(r).map_err(|e| e.to_string())?
            )
            .map_err(|e| e.to_string())?;
        }
        let rows: Vec<&String> = side
            .iter()
            .filter(|l| l.contains(&format!("\"solver\":\"{}\"", e.name)))
            .collect();
        if !rows.is_empty() {
            let mut f = fs::File::create(format!("{outdir}/{}.side.jsonl", e.name))
                .map_err(|e| e.to_string())?;
            for l in rows {
                writeln!(f, "{l}").map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let res = match args.first().map(String::as_str) {
        Some("run") => run(&args),
        Some("list") => {
            for e in arms(1) {
                println!("{:24} {}", e.name, e.description);
            }
            Ok(())
        }
        _ => Err("usage: blocker_backjump run|list (see src/main.rs)".into()),
    };
    match res {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
