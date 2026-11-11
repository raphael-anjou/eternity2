//! Runner for agenda A3 (see PREREG.md). Writes one jsonl line per instance,
//! sorted by (regime, n, seed), plus a separate timing file.

use eternity2_engine::generate_solved_framed_bc;
use sat_backdoors::*;
use serde_json::json;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

/// (border, interior) colours per (regime, n), fixed in PREREG.md.
fn colours(regime: usize, n: usize) -> (u8, u8) {
    let border = match n {
        6..=8 => 2,
        9 | 10 => 3,
        _ => panic!("size outside 6..10"),
    };
    let interior_a = [5u8, 6, 8, 9, 10][n - 6];
    let interior_b = [8u8, 10, 11, 13, 15][n - 6];
    (border, if regime == 0 { interior_a } else { interior_b })
}

struct Cfg {
    perms: u32,
    restarts: u32,
    ls_cap: u64,
}

fn run_instance(regime: usize, n: usize, seed: u32, cfg: &Cfg) -> (serde_json::Value, f64) {
    let t0 = Instant::now();
    let (border, interior) = colours(regime, n);
    let total = border + interior;
    let p = generate_solved_framed_bc(n as u8, total, seed, true, border);
    let m = Model::new(&p);
    assert!(m.root_ok, "root contradiction");
    let mut st = Stats { propagations: 0 };
    let cells = m.cells;
    let root_singletons = m.singletons(m.root());
    let root_complete = m.is_complete(m.root());

    let fwd = forward_greedy(&m, &mut st);
    let mut best = fwd.clone();
    let mut best_restart = usize::MAX;
    let all: Vec<usize> = (0..cells).collect();
    let rid = regime as u32;
    for r in 0..cfg.restarts {
        let s = 1_000_003u32
            .wrapping_mul(seed)
            .wrapping_add(101 * n as u32 + rid)
            .wrapping_add(r);
        let order = permutation(cells, s);
        let cand = removal_pass(&m, &all, &order, &mut st);
        best_restart = best_restart.min(cand.len());
        if cand.len() < best.len() {
            best = cand;
        }
    }
    let before_ls = best.len();
    let p0 = st.propagations;
    let best = local_search(&m, best, cfg.ls_cap, &mut st);
    let ls_props = st.propagations - p0;
    let ub = best.len();

    let cap = if n <= 7 { 3_000_000 } else { 500_000 };
    let ex = exact_min(&m, ub, cap, &mut st);
    let (k, source) = if ex.certified { (ex.value, "exact") } else { (ub, "greedy_ub") };

    let (dbest, ok) = m.state_for(&best, &mut st);
    assert!(ok && m.is_complete(&dbest));
    let (valid, planted) = m.check_board(&dbest);

    let mut thresholds = Vec::with_capacity(cfg.perms as usize);
    for r in 0..cfg.perms {
        let s = 7_000_001u32
            .wrapping_mul(seed)
            .wrapping_add(101 * n as u32 + rid)
            .wrapping_add(r);
        let perm = permutation(cells, s);
        thresholds.push(threshold(&m, &perm, &mut st));
    }
    let mut sorted = thresholds.clone();
    sorted.sort_unstable();
    let half = (cfg.perms as usize).div_ceil(2);
    let t50 = sorted[half - 1];
    let mean_t = thresholds.iter().sum::<usize>() as f64 / thresholds.len() as f64;
    let p_rand_k = thresholds.iter().filter(|&&t| t <= k).count() as f64 / thresholds.len() as f64;
    let p_rand_ub = thresholds.iter().filter(|&&t| t <= ub).count() as f64 / thresholds.len() as f64;

    let v = json!({
        "regime": if regime == 0 { "A" } else { "B" },
        "n": n,
        "cells": cells,
        "seed": seed,
        "border_colors": border,
        "interior_colors": interior,
        "colors_requested": total,
        "colors_returned": p.num_colors,
        "options_total": m.opt_piece.len(),
        "root_singletons": root_singletons,
        "root_complete": root_complete,
        "forward_greedy": fwd.len(),
        "best_restart": best_restart,
        "before_local_search": before_ls,
        "upper_bound": ub,
        "local_search_propagations": ls_props,
        "backdoor_cells": best,
        "exact_certified": ex.certified,
        "exact_value": ex.value,
        "exact_nodes": ex.nodes,
        "k": k,
        "k_source": source,
        "completed_valid": valid,
        "completed_is_planted": planted,
        "random_thresholds": thresholds,
        "t50": t50,
        "mean_t": mean_t,
        "p_rand_at_k": p_rand_k,
        "p_rand_at_ub": p_rand_ub,
        "propagations": st.propagations,
    });
    (v, t0.elapsed().as_secs_f64())
}

fn parse_list(s: &str) -> Vec<u32> {
    if let Some((a, b)) = s.split_once("..") {
        let (a, b): (u32, u32) = (a.parse().unwrap(), b.parse().unwrap());
        (a..=b).collect()
    } else {
        s.split(',').map(|x| x.parse().unwrap()).collect()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let get = |k: &str, d: &str| -> String {
        args.iter()
            .position(|a| a == k)
            .map(|i| args[i + 1].clone())
            .unwrap_or_else(|| d.to_string())
    };
    let sizes = parse_list(&get("--sizes", "6..10"));
    let seeds = parse_list(&get("--seeds", "1..30"));
    let regimes: Vec<usize> = get("--regimes", "A,B")
        .split(',')
        .map(|r| if r == "A" { 0 } else { 1 })
        .collect();
    let threads: usize = get("--threads", "8").parse().unwrap();
    let out = get("--out", "results/results.jsonl");
    let timing = get("--timing", "results/timing.jsonl");
    let cfg = Cfg {
        perms: get("--perms", "200").parse().unwrap(),
        restarts: get("--restarts", "32").parse().unwrap(),
        ls_cap: get("--ls-cap", "100000").parse().unwrap(),
    };

    let mut jobs = Vec::new();
    for &r in &regimes {
        for &n in &sizes {
            for &s in &seeds {
                jobs.push((r, n as usize, s));
            }
        }
    }
    // Largest boards first for load balance; output is re-sorted.
    let mut order: Vec<usize> = (0..jobs.len()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(jobs[i].1));
    let next = AtomicUsize::new(0);
    let results: Mutex<Vec<Option<(serde_json::Value, f64)>>> = Mutex::new(vec![None; jobs.len()]);
    let t0 = Instant::now();
    std::thread::scope(|sc| {
        for _ in 0..threads {
            sc.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                if i >= order.len() {
                    break;
                }
                let j = order[i];
                let (r, n, s) = jobs[j];
                let res = run_instance(r, n, s, &cfg);
                eprintln!(
                    "[{:7.1}s] regime={} n={} seed={} k={} ub={} t50={} ({:.1}s)",
                    t0.elapsed().as_secs_f64(),
                    if r == 0 { "A" } else { "B" },
                    n,
                    s,
                    res.0["k"],
                    res.0["upper_bound"],
                    res.0["t50"],
                    res.1
                );
                results.lock().unwrap()[j] = Some(res);
            });
        }
    });
    if let Some(dir) = std::path::Path::new(&out).parent() {
        std::fs::create_dir_all(dir).unwrap();
    }
    if let Some(dir) = std::path::Path::new(&timing).parent() {
        std::fs::create_dir_all(dir).unwrap();
    }
    let mut f = std::fs::File::create(&out).unwrap();
    let mut tf = std::fs::File::create(&timing).unwrap();
    for (j, r) in results.into_inner().unwrap().into_iter().enumerate() {
        let (v, secs) = r.unwrap();
        writeln!(f, "{}", serde_json::to_string(&v).unwrap()).unwrap();
        let (rg, n, s) = jobs[j];
        writeln!(
            tf,
            "{}",
            json!({"regime": if rg == 0 {"A"} else {"B"}, "n": n, "seed": s, "wall_s": (secs * 1000.0).round() / 1000.0})
        )
        .unwrap();
    }
    eprintln!("done: {} instances in {:.1}s", jobs.len(), t0.elapsed().as_secs_f64());
}
