//! A22 runner: hard-ladder instances, `NodeCounter` and `run_all`, with the
//! A22 arms, a seed filter, a cap override and the wall-clock guard.
//!
//! mrv_baseline run --solver S --ladder L --role R [--sizes 6,7] [--seeds 21-30]
//!                  [--cap 1000000] [--guard 600] [--threads 8] --out F.jsonl
//! mrv_baseline list

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;
use std::sync::atomic::Ordering;

use backdoor_branching::{BbSolver, Rule};
use hard_ladder::cli::load_instances;
use hard_ladder::{run_all, Ladder, LadderSolver, Role, SolverEntry};
use mrv_baseline::scb::GuardScb;
use mrv_baseline::{guard, MrvSolver, Order, GUARD_SECS};

fn scb_orig() -> Box<dyn eternity2_solver_trait::Solver> {
    Box::new(eternity2_solver_engine::EngineSolver::gacolor_ac3_random())
}
fn scb_fork_off() -> Box<dyn eternity2_solver_trait::Solver> {
    Box::new(mrvb_solver_engine::EngineSolver::gacolor_ac3_random())
}
fn scb_fork_ad() -> Box<dyn eternity2_solver_trait::Solver> {
    Box::new(mrvb_solver_engine::EngineSolver::gacolor_ac3_random().with_alldiff(true))
}

const ARMS: [SolverEntry; 7] = [
    SolverEntry {
        name: "bb-mrv",
        description: "binding: A16 BbSolver Rule::Mrv (A3 propagation, forced cells, free MRV)",
        run_seed: 0,
        make: |_| -> Box<dyn LadderSolver> { Box::new(BbSolver { rule: Rule::Mrv, guard: guard() }) },
    },
    SolverEntry {
        name: "scb-gacolor_ac3_random",
        description: "binding: hard-ladder scb gacolor_ac3_random, run seed 1, with wall-clock guard",
        run_seed: 1,
        make: |s| -> Box<dyn LadderSolver> { Box::new(GuardScb { make: scb_orig, seed: s, guard: guard() }) },
    },
    SolverEntry {
        name: "mrv-nohs",
        description: "exploratory: bb-mrv without piece-side all-different (hidden singles)",
        run_seed: 0,
        make: |_| -> Box<dyn LadderSolver> {
            Box::new(MrvSolver { order: Order::Free, random: None, hs: false, guard: guard() })
        },
    },
    SolverEntry {
        name: "mrv-bf",
        description: "exploratory: bb-mrv with border-first cell order",
        run_seed: 0,
        make: |_| -> Box<dyn LadderSolver> {
            Box::new(MrvSolver { order: Order::BorderFirst, random: None, hs: true, guard: guard() })
        },
    },
    SolverEntry {
        name: "mrv-rand",
        description: "exploratory: bb-mrv with scb's random value order, seed 1",
        run_seed: 1,
        make: |s| -> Box<dyn LadderSolver> {
            Box::new(MrvSolver { order: Order::Free, random: Some(s), hs: true, guard: guard() })
        },
    },
    SolverEntry {
        name: "scb-fork-off",
        description: "check: forked engine, all-different hook off (must equal scb)",
        run_seed: 1,
        make: |s| -> Box<dyn LadderSolver> { Box::new(GuardScb { make: scb_fork_off, seed: s, guard: guard() }) },
    },
    SolverEntry {
        name: "scb-ad",
        description: "exploratory: scb with A3 piece all-different after AC-3",
        run_seed: 1,
        make: |s| -> Box<dyn LadderSolver> { Box::new(GuardScb { make: scb_fork_ad, seed: s, guard: guard() }) },
    },
];

fn arg<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.iter().position(|a| a == key).and_then(|i| args.get(i + 1)).map(String::as_str)
}

fn run(args: &[String]) -> Result<(), String> {
    let name = arg(args, "--solver").ok_or("--solver required")?;
    let entry = hard_ladder::registry::find(&ARMS, name).ok_or_else(|| format!("unknown solver {name}"))?;
    let ladder = Ladder::parse(arg(args, "--ladder").ok_or("--ladder required")?).ok_or("bad --ladder")?;
    let role = Role::parse(arg(args, "--role").unwrap_or("test")).ok_or("bad --role")?;
    let cap: u64 = arg(args, "--cap").unwrap_or("1000000").parse().map_err(|_| "bad --cap")?;
    let g: u64 = arg(args, "--guard").unwrap_or("600").parse().map_err(|_| "bad --guard")?;
    GUARD_SECS.store(g, Ordering::Relaxed);
    let threads: usize = arg(args, "--threads").unwrap_or("8").parse().map_err(|_| "bad --threads")?;
    let out = arg(args, "--out").ok_or("--out required")?;
    let mut insts = load_instances(ladder, role)?;
    if let Some(s) = arg(args, "--sizes") {
        let sizes: Vec<u8> = s.split(',').map(|x| x.parse().map_err(|_| "bad --sizes")).collect::<Result<_, _>>()?;
        insts.retain(|i| sizes.contains(&i.n));
    }
    if let Some(s) = arg(args, "--seeds") {
        let (a, b) = s.split_once('-').ok_or("--seeds lo-hi")?;
        let (a, b): (u32, u32) = (a.parse().map_err(|_| "bad seed")?, b.parse().map_err(|_| "bad seed")?);
        insts.retain(|i| (a..=b).contains(&i.seed));
    }
    eprintln!("{} on {} instances, cap {cap}, guard {g} s, {threads} threads", entry.name, insts.len());
    let recs = run_all(entry, &insts, cap, threads);
    if let Some(d) = Path::new(out).parent() {
        fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    let mut f = fs::File::create(out).map_err(|e| e.to_string())?;
    for r in &recs {
        writeln!(f, "{}", serde_json::to_string(r).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    }
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
        _ => Err("usage: mrv_baseline run|list (see src/main.rs)".into()),
    };
    match res {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
