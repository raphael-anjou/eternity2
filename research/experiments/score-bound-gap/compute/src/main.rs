//! sbg: sample prefixes, measure bound gaps, run the soundness self-test.
//!
//!   sbg selftest [cases]
//!   sbg sample  --instances F --n N --source schedule|strict --runs 1,2,3,4
//!               --steps S --k K --seed X --out F
//!   sbg measure --instances F --prefixes F --widths 1,64,512,4096 --cap C
//!               --threads T --machine NAME --out F

use eternity2_engine::Puzzle;
use sbg::dd::{dd_bound, trivial_bound};
use sbg::model::{Model, Prefix};
use sbg::oracle::best_completion;
use sbg::sample::{collect, draw};
use sbg::selftest::witness_score;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

fn arg(args: &[String], k: &str) -> Option<String> {
    args.iter()
        .position(|a| a == k)
        .map(|i| args[i + 1].clone())
}
fn req(args: &[String], k: &str) -> String {
    arg(args, k).unwrap_or_else(|| panic!("missing {k}"))
}
fn list<T: std::str::FromStr>(s: &str) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    s.split(',').map(|x| x.trim().parse().unwrap()).collect()
}

fn read_jsonl(path: &str) -> Vec<Value> {
    let f = std::fs::File::open(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    std::io::BufReader::new(f)
        .lines()
        .filter_map(|l| {
            let l = l.unwrap();
            if l.trim().is_empty() {
                None
            } else {
                Some(serde_json::from_str(&l).unwrap())
            }
        })
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("selftest") => {
            let cases = args.get(2).map(|s| s.parse().unwrap()).unwrap_or(12);
            let k = sbg::selftest::run(cases);
            println!("selftest ok: {k} cases");
        }
        Some("sample") => sample(&args),
        Some("measure") => measure(&args),
        _ => {
            eprintln!("usage: sbg selftest|sample|measure ...");
            std::process::exit(2);
        }
    }
}

fn sample(args: &[String]) {
    let inst = read_jsonl(&req(args, "--instances"));
    let n: u64 = req(args, "--n").parse().unwrap();
    let runs: Vec<u32> = list(&req(args, "--runs"));
    let steps: u64 = req(args, "--steps").parse().unwrap();
    let k: usize = req(args, "--k").parse().unwrap();
    let seed: u32 = req(args, "--seed").parse().unwrap();
    let out_path = req(args, "--out");
    let source = req(args, "--source");
    let with_breaks = match source.as_str() {
        "schedule" => true,
        "strict" => false,
        _ => panic!("--source schedule|strict"),
    };
    let mut out = std::fs::File::create(&out_path).unwrap();
    for row in inst.iter().filter(|r| r["n"].as_u64() == Some(n)) {
        let p: Puzzle = serde_json::from_value(row["puzzle"].clone()).unwrap();
        let iseed = row["seed"].as_u64().unwrap() as u32;
        let pools = collect(&p, &runs, steps, with_breaks);
        let nn = n as usize;
        for r in 1..nn {
            let pool = &pools.pools[r];
            let s = seed
                ^ iseed.wrapping_mul(1_000_003)
                ^ (r as u32).wrapping_mul(7919)
                ^ (with_breaks as u32);
            for (j, idx) in draw(pool, k, s).into_iter().enumerate() {
                let rec = json!({
                    "id": row["id"], "ladder": row["ladder"], "n": n, "c": row["c"],
                    "seed": iseed, "source": source, "r": r, "draw": j, "pool_size": pool.len(),
                    "visits": pools.visits[r], "codes": pool[idx],
                });
                writeln!(out, "{rec}").unwrap();
            }
        }
        eprintln!(
            "{} pools {:?}",
            row["id"],
            pools.pools.iter().map(|p| p.len()).collect::<Vec<_>>()
        );
    }
}

fn key_of(v: &Value) -> String {
    format!(
        "{}|{}|{}|{}",
        v["id"].as_str().unwrap(),
        v["source"],
        v["r"],
        v["draw"]
    )
}

fn measure(args: &[String]) {
    let inst = read_jsonl(&req(args, "--instances"));
    let puzzles: HashMap<String, Puzzle> = inst
        .iter()
        .map(|r| {
            (
                r["id"].as_str().unwrap().to_string(),
                serde_json::from_value(r["puzzle"].clone()).unwrap(),
            )
        })
        .collect();
    let prefixes = read_jsonl(&req(args, "--prefixes"));
    let widths: Vec<usize> = list(&req(args, "--widths"));
    let cap: u64 = req(args, "--cap").parse().unwrap();
    let threads: usize = req(args, "--threads").parse().unwrap();
    let machine = req(args, "--machine");
    let out_path = req(args, "--out");
    let only_r: Option<Vec<u64>> = arg(args, "--r").map(|s| list(&s));
    let only_src: Option<String> = arg(args, "--source");
    let done: HashSet<String> = if std::path::Path::new(&out_path).exists() {
        read_jsonl(&out_path).iter().map(key_of).collect()
    } else {
        HashSet::new()
    };
    let todo: Vec<Value> = prefixes
        .into_iter()
        .filter(|v| !done.contains(&key_of(v)))
        .filter(|v| {
            only_r
                .as_ref()
                .map_or(true, |rs| rs.contains(&v["r"].as_u64().unwrap()))
        })
        .filter(|v| {
            only_src
                .as_ref()
                .map_or(true, |s| v["source"].as_str() == Some(s.as_str()))
        })
        .collect();
    eprintln!(
        "{} prefixes to measure ({} already done)",
        todo.len(),
        done.len()
    );
    let out = Arc::new(Mutex::new(
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&out_path)
            .unwrap(),
    ));
    let next = Arc::new(AtomicUsize::new(0));
    let todo = Arc::new(todo);
    let puzzles = Arc::new(puzzles);
    let widths = Arc::new(widths);
    let violation = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let mut hs = Vec::new();
    for _ in 0..threads {
        let (out, next, todo, puzzles, widths, machine, violation) = (
            out.clone(),
            next.clone(),
            todo.clone(),
            puzzles.clone(),
            widths.clone(),
            machine.clone(),
            violation.clone(),
        );
        hs.push(std::thread::spawn(move || loop {
            if violation.load(Ordering::SeqCst) {
                return;
            }
            let i = next.fetch_add(1, Ordering::SeqCst);
            if i >= todo.len() {
                return;
            }
            let v = &todo[i];
            let p = &puzzles[v["id"].as_str().unwrap()];
            let m = Model::new(p);
            let codes: Vec<i32> = v["codes"].as_array().unwrap().iter().map(|c| c.as_i64().unwrap() as i32).collect();
            let pre = Prefix::from_codes(&m, &codes);
            let d = pre.d();
            let pscore = pre.score(&m);
            let rem = m.remaining_edges(d);
            let triv = trivial_bound(&m, &pre);
            let or = best_completion(&m, &pre, cap);
            let truth = or.kstar.map(|k| pscore + rem - k);
            if let Some(t) = truth {
                let ws = witness_score(&m, &pre, &or.witness);
                assert_eq!(ws, t, "witness score mismatch on {}", key_of(v));
            }
            // Upper bound on the truth proven by the oracle (k_infeasible).
            let truth_ub = pscore + rem - (or.k_infeasible + 1).max(0) as u32;
            let mut dds = Vec::new();
            let mut bad = false;
            for &w in widths.iter() {
                let b = dd_bound(&m, &pre, w);
                if let Some(t) = truth {
                    if b.bound < t {
                        bad = true;
                    }
                }
                if b.bound > triv {
                    bad = true;
                }
                dds.push(b);
            }
            let rec = json!({
                "id": v["id"], "ladder": v["ladder"], "n": v["n"], "c": v["c"], "seed": v["seed"],
                "source": v["source"], "r": v["r"], "draw": v["draw"], "pool_size": v["pool_size"], "d": d,
                "prefix_score": pscore, "prefix_mismatches": pre.decided_edges(&m) - pscore,
                "remaining_edges": rem, "max_score": m.max_score(), "trivial": triv,
                "truth": truth, "truth_ub": truth_ub, "oracle_nodes": or.nodes,
                "oracle_wall_us": or.wall_us, "oracle_cap": cap, "k_infeasible": or.k_infeasible,
                "dd": dds, "soundness_violation": bad, "machine": machine,
            });
            writeln!(out.lock().unwrap(), "{rec}").unwrap();
            if bad {
                eprintln!("SOUNDNESS VIOLATION on {}", key_of(v));
                violation.store(true, Ordering::SeqCst);
            }
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
    if violation.load(Ordering::SeqCst) {
        eprintln!("stopped: soundness violation");
        std::process::exit(3);
    }
}
