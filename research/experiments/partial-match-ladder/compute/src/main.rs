//! pml: partial-match ladder harness.
//!
//!   pml gen   --role R --n N --c C --seeds A..B --out FILE
//!   pml check FILE...
//!   pml run   --entrant E --instances FILE --budget-evals W --budget-placements P
//!             --run-seed S --threads T --machine M --out FILE.jsonl
//!
//! `run` appends one JSON row per instance as it finishes (resumable: rows
//! already in FILE for the same entrant, instance, budget and seed are skipped).

#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use pml::entrants::{self, unit_of};
use pml::instance::{self, Instance};
use pml::verify::verify;
use pml::work::Unit;
use serde_json::json;

fn arg<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
}

fn req<T: std::str::FromStr>(args: &[String], name: &str) -> T {
    arg(args, name)
        .unwrap_or_else(|| panic!("missing {name}"))
        .parse()
        .unwrap_or_else(|_| panic!("bad {name}"))
}

fn cmd_gen(args: &[String]) {
    let role: String = req(args, "--role");
    let n: u8 = req(args, "--n");
    let c: u8 = req(args, "--c");
    let seeds: String = req(args, "--seeds");
    let (a, b) = seeds.split_once("..").expect("--seeds A..B");
    let (a, b): (u32, u32) = (a.parse().unwrap(), b.parse().unwrap());
    let out = PathBuf::from(arg(args, "--out").expect("--out"));
    let mut lines = String::new();
    for seed in a..=b {
        let inst = instance::make(&role, n, c, seed);
        lines.push_str(&serde_json::to_string(&inst).unwrap());
        lines.push('\n');
    }
    std::fs::write(&out, lines).unwrap();
    let rows = instance::load(&out).unwrap();
    instance::check(&rows).unwrap();
    eprintln!("wrote {} instances to {}", rows.len(), out.display());
}

fn cmd_check(args: &[String]) -> bool {
    let mut ok = true;
    for f in args {
        let rows = instance::load(f.as_ref()).unwrap();
        match instance::check(&rows) {
            Ok(()) => println!("ok {} ({} instances)", f, rows.len()),
            Err(e) => {
                println!("FAIL {f}: {e}");
                ok = false;
            }
        }
    }
    ok
}

fn key(entrant: &str, id: &str, budget: u64, seed: u64) -> String {
    format!("{entrant}|{id}|{budget}|{seed}")
}

fn run_one(
    entrant: &str,
    inst: &Instance,
    budget: u64,
    seed: u64,
    machine: &str,
) -> serde_json::Value {
    let t0 = Instant::now();
    let res = entrants::run(entrant, &inst.puzzle, budget, seed);
    let wall_ms = t0.elapsed().as_secs_f64() * 1000.0;
    let base = json!({
        "entrant": entrant, "instance": inst.id, "role": inst.role, "n": inst.n, "c": inst.c,
        "seed": inst.seed, "run_seed": seed, "optimum": inst.optimum,
        "unit": unit_of(entrant), "budget": budget, "machine": machine, "wall_ms": wall_ms,
    });
    let mut row = base.as_object().unwrap().clone();
    match res {
        Err(e) => {
            row.insert("status".into(), json!("error"));
            row.insert("error".into(), json!(e));
            row.insert("best_score".into(), json!(null));
        }
        Ok(out) => {
            // Independent re-verification of the returned board (the adapter
            // verified too; this is the harness's own call).
            let (status, score, err) = match &out.board {
                None => ("no_complete_board", None, None),
                Some(b) => match verify(&inst.puzzle, b) {
                    Ok(s) => ("ok", Some(s), None),
                    Err(e) => ("error", None, Some(e)),
                },
            };
            let traj_end = out.trajectory.last().map(|t| t.1);
            let (status, err) = if status == "ok" && traj_end != score {
                (
                    "error",
                    Some(format!("trajectory end {traj_end:?} != verified {score:?}")),
                )
            } else {
                (status, err)
            };
            row.insert("status".into(), json!(status));
            if let Some(e) = err {
                row.insert("error".into(), json!(e));
            }
            row.insert("best_score".into(), json!(score));
            row.insert("work_used".into(), json!(out.work.used));
            row.insert("trajectory".into(), json!(out.trajectory));
            row.insert("trajectory_verified".into(), json!(out.trajectory_verified));
            row.insert("extra".into(), out.extra);
            row.insert("board".into(), json!(out.board));
        }
    }
    serde_json::Value::Object(row)
}

fn cmd_run(args: &[String]) {
    let entrant: String = req(args, "--entrant");
    let insts = instance::load(arg(args, "--instances").expect("--instances").as_ref()).unwrap();
    instance::check(&insts).unwrap();
    let budget_evals: u64 = req(args, "--budget-evals");
    let budget_place: u64 = req(args, "--budget-placements");
    let seed: u64 = req(args, "--run-seed");
    let threads: usize = req(args, "--threads");
    let machine: String = req(args, "--machine");
    let out = PathBuf::from(arg(args, "--out").expect("--out"));
    let budget = match unit_of(&entrant) {
        Unit::Placement => budget_place,
        Unit::MoveEval => budget_evals,
    };
    let done: HashSet<String> = std::fs::read_to_string(&out)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .map(|v| {
            key(
                v["entrant"].as_str().unwrap_or(""),
                v["instance"].as_str().unwrap_or(""),
                v["budget"].as_u64().unwrap_or(0),
                v["run_seed"].as_u64().unwrap_or(0),
            )
        })
        .collect();
    let todo: Vec<&Instance> = insts
        .iter()
        .filter(|i| !done.contains(&key(&entrant, &i.id, budget, seed)))
        .collect();
    eprintln!(
        "{entrant}: {} to run, {} already in {}",
        todo.len(),
        insts.len() - todo.len(),
        out.display()
    );
    let next = AtomicUsize::new(0);
    let file = Mutex::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&out)
            .unwrap(),
    );
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                let Some(inst) = todo.get(i) else { break };
                let row = run_one(&entrant, inst, budget, seed, &machine);
                eprintln!(
                    "{entrant} {} -> {} {} ({:.0} ms)",
                    inst.id,
                    row["status"],
                    row["best_score"],
                    row["wall_ms"].as_f64().unwrap_or(0.0)
                );
                let mut f = file.lock().unwrap();
                writeln!(f, "{}", serde_json::to_string(&row).unwrap()).unwrap();
                f.flush().unwrap();
            });
        }
    });
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("gen") => cmd_gen(&args[2..]),
        Some("check") => {
            if !cmd_check(&args[2..]) {
                std::process::exit(1);
            }
        }
        Some("run") => cmd_run(&args[2..]),
        _ => {
            eprintln!("usage: pml gen|check|run ... (see src/main.rs)");
            std::process::exit(2);
        }
    }
}
