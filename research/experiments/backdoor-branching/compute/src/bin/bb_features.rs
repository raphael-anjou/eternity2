//! Stage 1: root features of every cell on A3's 300 boards, with A3's
//! committed backdoor labels. Writes one jsonl row per instance, sorted.
//!
//! bb_features [--threads 8] [--out results/stage1/features.jsonl]

use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use backdoor_branching::{features, log_size, Static, FEATURES};
use eternity2_engine::generate_solved_framed_bc;
use sat_backdoors::{Model, Stats};
use serde_json::{json, Value};

/// A3's colour table (sat-backdoors/src/main.rs).
fn colours(regime: &str, n: usize) -> (u8, u8) {
    let border = if n <= 8 { 2 } else { 3 };
    let a = [5u8, 6, 8, 9, 10][n - 6];
    let b = [8u8, 10, 11, 13, 15][n - 6];
    (border, if regime == "A" { a } else { b })
}

fn row(a3: &Value) -> Value {
    let n = a3["n"].as_u64().expect("n") as usize;
    let seed = a3["seed"].as_u64().expect("seed") as u32;
    let regime = a3["regime"].as_str().expect("regime");
    let (border, interior) = colours(regime, n);
    assert_eq!(a3["border_colors"].as_u64(), Some(u64::from(border)));
    assert_eq!(a3["interior_colors"].as_u64(), Some(u64::from(interior)));
    let p = generate_solved_framed_bc(n as u8, border + interior, seed, true, border);
    let m = Model::new(&p);
    assert!(m.root_ok);
    let s = Static::new(&m, &p);
    let d = m.root().to_vec();
    let l = log_size(&m, &d);
    let assigned = vec![false; m.cells];
    let bd: Vec<u64> = a3["backdoor_cells"].as_array().expect("bd").iter().map(|x| x.as_u64().expect("c")).collect();
    let mut st = Stats { propagations: 0 };
    let mut cells = Vec::new();
    for c in 0..m.cells {
        if m.count(&d, c) < 2 {
            continue;
        }
        let (f, _) = features(&m, &s, &d, &assigned, c, l, true, &mut st);
        cells.push(json!({"cell": c, "label": bd.contains(&(c as u64)), "f": f.to_vec()}));
    }
    json!({"regime": regime, "n": n, "seed": seed, "k": a3["k"], "k_source": a3["k_source"],
           "features": FEATURES, "cells": cells})
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
    let threads: usize = arg("--threads").map_or(8, |t| t.parse().expect("threads"));
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let out = arg("--out").map_or_else(|| root.join("results/stage1/features.jsonl"), Into::into);
    let a3_path = root.join("../sat-backdoors/results/results.jsonl");
    let a3: Vec<Value> = std::fs::read_to_string(&a3_path)
        .expect("A3 results")
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("json"))
        .collect();
    let next = AtomicUsize::new(0);
    let rows: Mutex<Vec<Option<Value>>> = Mutex::new(vec![None; a3.len()]);
    std::thread::scope(|sc| {
        for _ in 0..threads.max(1) {
            sc.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                let Some(r) = a3.get(i) else { break };
                let v = row(r);
                rows.lock().expect("lock")[i] = Some(v);
            });
        }
    });
    let mut rows: Vec<Value> = rows.into_inner().expect("lock").into_iter().map(|r| r.expect("row")).collect();
    rows.sort_by_key(|r| (r["regime"].as_str().unwrap_or("").to_string(), r["n"].as_u64(), r["seed"].as_u64()));
    std::fs::create_dir_all(out.parent().expect("dir")).expect("mkdir");
    let mut f = std::fs::File::create(&out).expect("create");
    for r in &rows {
        writeln!(f, "{r}").expect("write");
    }
    eprintln!("wrote {} rows to {}", rows.len(), out.display());
}
