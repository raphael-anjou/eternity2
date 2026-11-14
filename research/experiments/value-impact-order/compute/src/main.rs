//! A21 runner (see `../PREREG.md`). Arms are interleaved per instance: a job
//! is (instance, repetition) and the worker runs every requested arm on it back
//! to back, in an order rotated by (instance index + repetition).
//!
//! value_impact run --rung r6|r7|r8 --role test|calibration --arms a,b,..
//!                  [--reps 1] [--threads 2] --out F.jsonl     (appends, resumes)
//! value_impact list

use std::cell::Cell;
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Duration;

use backdoor_branching::{BbSolver, Rule};
use hard_ladder::cli::load_instances;
use hard_ladder::{run_one, Ladder, LadderInstance, LadderSolver, Role, SolverEntry, TEST_CAP};
use mrv_baseline::scb::GuardScb;
use mrv_baseline::{MrvSolver, Order};
use value_impact_order::{ProbeSolver, ValueOrder, PROBES, SEQ_HASH};

/// Per-run wall-clock guard (PREREG.md: 600 s, never the binding stop).
const GUARD: Duration = Duration::from_secs(600);

fn scb_orig() -> Box<dyn eternity2_solver_trait::Solver> {
    Box::new(eternity2_solver_engine::EngineSolver::gacolor_ac3_random())
}

macro_rules! entry {
    ($name:expr, $desc:expr, $seed:expr, $make:expr) => {
        SolverEntry {
            name: $name,
            description: $desc,
            run_seed: $seed,
            make: $make,
        }
    };
}

const ARMS: [SolverEntry; 10] = [
    entry!(
        "bb-mrv",
        "parent and decision baseline: A16 BbSolver Rule::Mrv",
        0,
        |_| -> Box<dyn LadderSolver> {
            Box::new(BbSolver {
                rule: Rule::Mrv,
                guard: GUARD,
            })
        }
    ),
    entry!(
        "vi-impact",
        "candidate: probe options, least options removed first",
        0,
        |_| -> Box<dyn LadderSolver> {
            Box::new(ProbeSolver {
                order: ValueOrder::Impact,
                guard: GUARD,
            })
        }
    ),
    entry!(
        "vi-index",
        "check: probing code with bb-mrv's index order",
        0,
        |_| -> Box<dyn LadderSolver> {
            Box::new(ProbeSolver {
                order: ValueOrder::Index,
                guard: GUARD,
            })
        }
    ),
    entry!(
        "vi-probe-rand-s1",
        "control: probes, failed last, rest random",
        1,
        |s| -> Box<dyn LadderSolver> {
            Box::new(ProbeSolver {
                order: ValueOrder::ProbeRand(s),
                guard: GUARD,
            })
        }
    ),
    entry!(
        "vi-probe-rand-s2",
        "control: probes, failed last, rest random",
        2,
        |s| -> Box<dyn LadderSolver> {
            Box::new(ProbeSolver {
                order: ValueOrder::ProbeRand(s),
                guard: GUARD,
            })
        }
    ),
    entry!(
        "vi-probe-rand-s3",
        "control: probes, failed last, rest random",
        3,
        |s| -> Box<dyn LadderSolver> {
            Box::new(ProbeSolver {
                order: ValueOrder::ProbeRand(s),
                guard: GUARD,
            })
        }
    ),
    entry!("mrv-rand-s1", "control: A22 mrv-rand", 1, |s| -> Box<
        dyn LadderSolver,
    > {
        Box::new(MrvSolver {
            order: Order::Free,
            random: Some(s),
            hs: true,
            guard: GUARD,
        })
    }),
    entry!("mrv-rand-s2", "control: A22 mrv-rand", 2, |s| -> Box<
        dyn LadderSolver,
    > {
        Box::new(MrvSolver {
            order: Order::Free,
            random: Some(s),
            hs: true,
            guard: GUARD,
        })
    }),
    entry!("mrv-rand-s3", "control: A22 mrv-rand", 3, |s| -> Box<
        dyn LadderSolver,
    > {
        Box::new(MrvSolver {
            order: Order::Free,
            random: Some(s),
            hs: true,
            guard: GUARD,
        })
    }),
    entry!(
        "scb-gacolor_ac3_random",
        "R6/R7 baseline: A22 GuardScb, run seed 1",
        1,
        |s| -> Box<dyn LadderSolver> {
            Box::new(GuardScb {
                make: scb_orig,
                seed: s,
                guard: GUARD,
            })
        }
    ),
];

fn arg<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
}

fn instances(rung: &str, role: Role) -> Result<Vec<LadderInstance>, String> {
    let mut v = match rung {
        "r6" | "r7" => {
            let n = if rung == "r6" { 6 } else { 7 };
            let mut v = load_instances(Ladder::Primary, role)?;
            v.retain(|i| i.n == n);
            v
        }
        "r8" => match role {
            Role::Test => {
                let mut a = load_instances(Ladder::EightC10, role)?;
                a.retain(|i| (21..=30).contains(&i.seed));
                a.extend(load_instances(Ladder::EightC10Fresh, role)?);
                a
            }
            Role::Calibration => load_instances(Ladder::EightC10, role)?,
        },
        _ => return Err(format!("bad --rung {rung}")),
    };
    v.sort_by_key(|i| i.seed);
    Ok(v)
}

fn run(args: &[String]) -> Result<(), String> {
    let rung = arg(args, "--rung").ok_or("--rung required")?;
    let role = Role::parse(arg(args, "--role").unwrap_or("test")).ok_or("bad --role")?;
    let arms: Vec<&SolverEntry> = arg(args, "--arms")
        .ok_or("--arms required")?
        .split(',')
        .map(|a| {
            ARMS.iter()
                .find(|e| e.name == a)
                .ok_or_else(|| format!("unknown arm {a}"))
        })
        .collect::<Result<_, _>>()?;
    let reps: usize = arg(args, "--reps")
        .unwrap_or("1")
        .parse()
        .map_err(|_| "bad --reps")?;
    let threads: usize = arg(args, "--threads")
        .unwrap_or("2")
        .parse()
        .map_err(|_| "bad --threads")?;
    let out = arg(args, "--out").ok_or("--out required")?;
    let insts = instances(rung, role)?;
    if let Some(d) = Path::new(out).parent() {
        fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    let mut done: HashSet<(String, String, usize)> = HashSet::new();
    if let Ok(text) = fs::read_to_string(out) {
        for l in text.lines().filter(|l| !l.trim().is_empty()) {
            let v: serde_json::Value = serde_json::from_str(l).map_err(|e| e.to_string())?;
            done.insert((
                v["solver"].as_str().unwrap_or("").into(),
                v["instance"].as_str().unwrap_or("").into(),
                v["rep"].as_u64().unwrap_or(0) as usize,
            ));
        }
    }
    let jobs: Vec<(usize, usize)> = (0..insts.len())
        .flat_map(|i| (0..reps).map(move |r| (i, r)))
        .collect();
    eprintln!(
        "{rung} {} instances x {reps} reps, arms {}, {threads} threads, {} rows already done",
        insts.len(),
        arms.len(),
        done.len()
    );
    let file = Mutex::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(out)
            .map_err(|e| e.to_string())?,
    );
    let next = AtomicUsize::new(0);
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| loop {
                let k = next.fetch_add(1, Ordering::Relaxed);
                let Some(&(i, rep)) = jobs.get(k) else { break };
                let inst = &insts[i];
                let a = arms.len();
                for pos in 0..a {
                    let e = arms[(pos + i + rep) % a];
                    if done.contains(&(e.name.to_string(), inst.id.clone(), rep)) {
                        continue;
                    }
                    SEQ_HASH.with(|h| h.set(0));
                    PROBES.with(|p| p.set(0));
                    let rec = run_one(e, inst, TEST_CAP);
                    let probing = e.name.starts_with("vi-");
                    let mut v = serde_json::to_value(&rec).expect("json");
                    v["rung"] = rung.into();
                    v["rep"] = rep.into();
                    v["pos"] = pos.into();
                    v["seq_hash"] = if probing {
                        format!("{:016x}", SEQ_HASH.with(Cell::get)).into()
                    } else {
                        serde_json::Value::Null
                    };
                    v["probes"] = if probing {
                        PROBES.with(Cell::get).into()
                    } else {
                        serde_json::Value::Null
                    };
                    eprintln!(
                        "{} {} r{rep} {} nodes={} wall_ms={:.0}",
                        rec.solver, rec.instance, rec.status, rec.nodes, rec.wall_ms
                    );
                    let mut f = file.lock().expect("lock");
                    writeln!(f, "{v}").expect("write");
                    f.flush().expect("flush");
                }
            });
        }
    });
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let res = match args.first().map(String::as_str) {
        Some("run") => run(&args),
        Some("list") => {
            for e in &ARMS {
                println!("{:24} {}", e.name, e.description);
            }
            Ok(())
        }
        _ => Err("usage: value_impact run|list (see src/main.rs)".into()),
    };
    match res {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
