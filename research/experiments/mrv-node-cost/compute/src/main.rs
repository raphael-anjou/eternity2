//! mrv_node_cost: gate, timing and plain runs (see `../PREREG.md`).
//!
//! mrv_node_cost run  --solver S --ladder L --role R [--sizes 6,7] [--seeds 21-30]
//!                    [--cap 1000000] [--guard 600] [--threads 1] --out F.jsonl
//! mrv_node_cost gate --ladder L --role R [--sizes ..] [--seeds ..] [--cap 1000000] --out F.jsonl
//! mrv_node_cost time --ladder L --role R [--sizes ..] [--seeds ..] [--cap 1000000] --out F.jsonl
//! mrv_node_cost list

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;
use std::sync::atomic::Ordering;
use std::time::Instant;

use backdoor_branching::{BbSolver, Rule};
use hard_ladder::cli::load_instances;
use hard_ladder::{
    run_all, run_one, verify_board, Ladder, LadderInstance, LadderSolver, NodeCounter, Outcome,
    Role, SolverEntry,
};
use mrv_node_cost::{guard, FastSolver, HashHook, NoHook, RefSolver, GUARD_SECS};
use serde_json::json;

const ARMS: [SolverEntry; 2] = [
    SolverEntry {
        name: "bb-mrv",
        description: "original: A16 BbSolver Rule::Mrv, unchanged code (as run by A22)",
        run_seed: 0,
        make: |_| -> Box<dyn LadderSolver> {
            Box::new(BbSolver {
                rule: Rule::Mrv,
                guard: guard(),
            })
        },
    },
    SolverEntry {
        name: "bb-mrv-fast",
        description: "optimization: same search tree as bb-mrv, cheaper node",
        run_seed: 0,
        make: |_| -> Box<dyn LadderSolver> {
            Box::new(FastSolver::<NoHook> {
                guard: guard(),
                hook: NoHook,
            })
        },
    },
];

fn arg<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
}

fn instances(args: &[String]) -> Result<Vec<LadderInstance>, String> {
    let ladder =
        Ladder::parse(arg(args, "--ladder").ok_or("--ladder required")?).ok_or("bad --ladder")?;
    let role = Role::parse(arg(args, "--role").unwrap_or("test")).ok_or("bad --role")?;
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
    Ok(insts)
}

fn setup(args: &[String]) -> Result<(u64, fs::File), String> {
    let cap: u64 = arg(args, "--cap")
        .unwrap_or("1000000")
        .parse()
        .map_err(|_| "bad --cap")?;
    let g: u64 = arg(args, "--guard")
        .unwrap_or("600")
        .parse()
        .map_err(|_| "bad --guard")?;
    GUARD_SECS.store(g, Ordering::Relaxed);
    let out = arg(args, "--out").ok_or("--out required")?;
    if let Some(d) = Path::new(out).parent() {
        fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    Ok((cap, fs::File::create(out).map_err(|e| e.to_string())?))
}

fn run(args: &[String]) -> Result<(), String> {
    let name = arg(args, "--solver").ok_or("--solver required")?;
    let entry =
        hard_ladder::registry::find(&ARMS, name).ok_or_else(|| format!("unknown solver {name}"))?;
    let insts = instances(args)?;
    let threads: usize = arg(args, "--threads")
        .unwrap_or("1")
        .parse()
        .map_err(|_| "bad --threads")?;
    let (cap, mut f) = setup(args)?;
    eprintln!(
        "{} on {} instances, cap {cap}, {threads} threads",
        entry.name,
        insts.len()
    );
    for r in &run_all(entry, &insts, cap, threads) {
        writeln!(
            f,
            "{}",
            serde_json::to_string(r).map_err(|e| e.to_string())?
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// One gate arm: (status, nodes, backtracks, propagation, board, hash, hashed nodes, ms).
fn gate_arm(s: &mut dyn LadderSolver, inst: &LadderInstance, cap: u64) -> serde_json::Value {
    let mut counter = NodeCounter::new(cap);
    let t = Instant::now();
    let out = s.solve(&inst.puzzle, &mut counter);
    let ms = t.elapsed().as_secs_f64() * 1000.0;
    let (status, board) = match out {
        Outcome::Solved(b) => match verify_board(&inst.puzzle, &b) {
            Ok(()) => ("solved", Some(b)),
            Err(_) => ("error", Some(b)),
        },
        Outcome::CapReached => ("censored", None),
        Outcome::Exhausted => ("error", None),
    };
    json!({
        "status": status, "nodes": counter.nodes(), "backtracks": counter.backtracks,
        "propagation": counter.propagation, "board": board, "ms": ms,
    })
}

fn gate(args: &[String]) -> Result<(), String> {
    let insts = instances(args)?;
    let (cap, mut f) = setup(args)?;
    let mut bad = 0;
    for inst in &insts {
        let orig = gate_arm(
            &mut BbSolver {
                rule: Rule::Mrv,
                guard: guard(),
            },
            inst,
            cap,
        );
        let mut rs = RefSolver::<HashHook> {
            guard: guard(),
            hook: HashHook::default(),
        };
        let mut refr = gate_arm(&mut rs, inst, cap);
        refr["hash"] = json!(format!("{:016x}", rs.hook.h));
        refr["hashed"] = json!(rs.hook.n);
        let mut fs_ = FastSolver::<HashHook> {
            guard: guard(),
            hook: HashHook::default(),
        };
        let mut fast = gate_arm(&mut fs_, inst, cap);
        fast["hash"] = json!(format!("{:016x}", fs_.hook.h));
        fast["hashed"] = json!(fs_.hook.n);
        let keys = ["status", "nodes", "backtracks", "propagation", "board"];
        let ref_eq_orig =
            keys.iter().all(|k| orig[k] == refr[k]) && refr["hashed"] == refr["nodes"];
        let fast_eq_ref = keys
            .iter()
            .chain(["hash", "hashed"].iter())
            .all(|k| fast[k] == refr[k]);
        if !(ref_eq_orig && fast_eq_ref) {
            bad += 1;
        }
        eprintln!(
            "{} {} nodes={} hash={} ref==orig {} fast==ref {} ms orig {:.0} ref {:.0} fast {:.0}",
            inst.id,
            orig["status"],
            orig["nodes"],
            refr["hash"],
            ref_eq_orig,
            fast_eq_ref,
            orig["ms"].as_f64().unwrap_or(0.0),
            refr["ms"].as_f64().unwrap_or(0.0),
            fast["ms"].as_f64().unwrap_or(0.0)
        );
        let row = json!({
            "instance": inst.id, "ladder": inst.ladder, "role": inst.role, "n": inst.n, "c": inst.c,
            "seed": inst.seed, "cap": cap, "orig": orig, "ref": refr, "fast": fast,
            "ref_eq_orig": ref_eq_orig, "fast_eq_ref": fast_eq_ref,
        });
        writeln!(f, "{row}").map_err(|e| e.to_string())?;
    }
    eprintln!("gate: {} instances, {bad} mismatches", insts.len());
    Ok(())
}

/// Per instance, 10 runs in the order A B B A A B B A A B (A = bb-mrv,
/// B = bb-mrv-fast), each through hard-ladder's `run_one`.
fn time(args: &[String]) -> Result<(), String> {
    let insts = instances(args)?;
    let (cap, mut f) = setup(args)?;
    const PATTERN: [usize; 10] = [0, 1, 1, 0, 0, 1, 1, 0, 0, 1];
    for inst in &insts {
        let mut rep = [0usize; 2];
        for (slot, &arm) in PATTERN.iter().enumerate() {
            let rec = run_one(&ARMS[arm], inst, cap);
            let mut v = serde_json::to_value(&rec).map_err(|e| e.to_string())?;
            v["slot"] = json!(slot);
            v["rep"] = json!(rep[arm]);
            rep[arm] += 1;
            writeln!(f, "{v}").map_err(|e| e.to_string())?;
            eprintln!(
                "{} {} slot {slot} {} nodes={} ms={:.1}",
                inst.id, rec.solver, rec.status, rec.nodes, rec.wall_ms
            );
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let res = match args.first().map(String::as_str) {
        Some("run") => run(&args),
        Some("gate") => gate(&args),
        Some("time") => time(&args),
        Some("list") => {
            for e in &ARMS {
                println!("{:16} {}", e.name, e.description);
            }
            Ok(())
        }
        _ => Err("usage: mrv_node_cost run|gate|time|list (see src/main.rs)".into()),
    };
    match res {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
