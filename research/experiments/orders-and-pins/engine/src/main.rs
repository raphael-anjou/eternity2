//! rpp: real-puzzle-profile measurements (see ../PREREG.md and
//! ../PREREG_AMENDMENTS.md). Subcommands:
//!   bw      M1, one run of the Blackwood-style port, one JSON line
//!   smc     M2, SMC repetitions on the official set, one JSON line each
//!   enum    exact widths of a configuration by depth-first enumeration
//!   gate-a  M2 gate (a): the 30 hard-ladder 6x6 instances
//!   gate-gen  secondary check on generated boards (with fixed cells)
//!   paint   M2 null: SMC on instances painted from the official census

mod bw;
mod oap;
mod puzzle;
mod tree;

use puzzle::{load_csv, load_fixed, Puzzle, Rng};
use std::collections::HashMap;
use std::time::Instant;
use tree::{rows_bottom_up, smc, spiral, Tree};

fn args_map() -> (String, HashMap<String, String>) {
    let a: Vec<String> = std::env::args().collect();
    let cmd = a.get(1).cloned().unwrap_or_default();
    let mut m = HashMap::new();
    let mut i = 2;
    while i < a.len() {
        let k = a[i].trim_start_matches("--").to_string();
        if i + 1 < a.len() && !a[i + 1].starts_with("--") {
            m.insert(k, a[i + 1].clone());
            i += 2;
        } else {
            m.insert(k, "1".into());
            i += 1;
        }
    }
    (cmd, m)
}

pub(crate) fn get<T: std::str::FromStr>(m: &HashMap<String, String>, k: &str, d: T) -> T {
    m.get(k).and_then(|v| v.parse().ok()).unwrap_or(d)
}

pub(crate) fn num(m: &HashMap<String, String>, k: &str, d: f64) -> f64 {
    m.get(k)
        .map(|v| v.parse::<f64>().expect("number"))
        .unwrap_or(d)
}

pub(crate) fn fmt_vec<T: std::fmt::Display>(v: &[T]) -> String {
    let s: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    format!("[{}]", s.join(","))
}

pub(crate) fn fmt_f(v: &[f64]) -> String {
    let s: Vec<String> = v.iter().map(|x| format!("{:.6e}", x)).collect();
    format!("[{}]", s.join(","))
}

/// Free cells of `order` after removing fixed ones.
fn free_order(order: Vec<usize>, fixed: &[(usize, u16, u8)]) -> Vec<usize> {
    order
        .into_iter()
        .filter(|p| !fixed.iter().any(|f| f.0 == *p))
        .collect()
}

fn official_config(m: &HashMap<String, String>) -> (Puzzle, Vec<(usize, u16, u8)>, Vec<usize>) {
    let p = load_csv(m.get("csv").expect("--csv"));
    let clues = m.get("clues").map(String::as_str).unwrap_or("five");
    let mut fixed: Vec<(usize, u16, u8)> = match clues {
        "none" => vec![],
        "centre" => p
            .hints
            .iter()
            .copied()
            .filter(|h| h.0 == 8 * 16 + 7)
            .collect(),
        "five" => p.hints.clone(),
        _ => panic!("--clues none|centre|five"),
    };
    if let Some(f) = m.get("fixed") {
        for c in load_fixed(f) {
            if !fixed.iter().any(|x| x.0 == c.0) {
                fixed.push(c);
            }
        }
    }
    let ord = match m.get("order").map(String::as_str).unwrap_or("rows") {
        "rows" => rows_bottom_up(p.n),
        "spiral" => spiral(p.n),
        "ring" => tree::ring(p.n),
        _ => panic!("--order rows|spiral|ring"),
    };
    let order = free_order(ord, &fixed);
    (p, fixed, order)
}

pub(crate) fn smc_json(label: &str, rep: usize, seed: u64, np: usize, r: &tree::SmcRun, secs: f64) -> String {
    let lw: Vec<f64> = r
        .est
        .iter()
        .map(|&x| {
            if x > 0.0 {
                x.log10()
            } else {
                f64::NEG_INFINITY
            }
        })
        .collect();
    let lw_s: Vec<String> = lw
        .iter()
        .map(|x| {
            if x.is_finite() {
                format!("{:.6}", x)
            } else {
                "null".into()
            }
        })
        .collect();
    let total: f64 = r.est[1..].iter().sum();
    let to175: f64 = r.est[1..r.est.len().min(176)].iter().sum();
    let ext = r.est.iter().rposition(|&x| x > 0.0).unwrap_or(0);
    let levels: Vec<String> = r
        .levels
        .iter()
        .map(|l| {
            format!(
                "[{},{},{:.6},{:.6},{:.6}]",
                l.e, l.m, l.zero_frac, l.mean_ext, l.var_ext
            )
        })
        .collect();
    let div: Vec<String> = r
        .diversity
        .iter()
        .map(|d| format!("[{},{},{},{:.6e}]", d.0, d.1, d.2, d.3))
        .collect();
    format!(
        "{{\"label\":\"{label}\",\"rep\":{rep},\"seed\":{seed},\"np\":{np},\"total\":{total:.6e},\"total_to_175\":{to175:.6e},\"extinction_depth\":{ext},\"seconds\":{secs:.2},\"log10_width\":[{}],\"levels_E_M_zero_mean_var\":[{}],\"diversity_depth_anc_distinct_sumsq\":[{}]}}",
        lw_s.join(","),
        levels.join(","),
        div.join(",")
    )
}

pub(crate) fn parse_pieces(line: &str) -> (String, usize, Vec<[u8; 4]>) {
    let id = line
        .split("\"id\":\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap()
        .to_string();
    let n: usize = line
        .split("\"n\":")
        .nth(1)
        .unwrap()
        .split(',')
        .next()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let body = line.split("\"pieces\":[").nth(1).unwrap();
    let body = &body[..body.find("]]").unwrap() + 1];
    let mut pieces = Vec::new();
    for chunk in body.split(']') {
        let nums: Vec<u8> = chunk
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        if nums.len() == 4 {
            pieces.push([nums[0], nums[1], nums[2], nums[3]]);
        }
    }
    assert_eq!(pieces.len(), n * n, "{id}");
    (id, n, pieces)
}

/// Exact widths and SMC per-depth mean/sd for one configuration.
pub(crate) fn check_config(label: &str, t: &Tree, np: usize, reps: usize, cap: u64, seed0: u64) -> String {
    let t0 = Instant::now();
    let (w, complete) = t.enumerate(cap);
    let enum_s = t0.elapsed().as_secs_f64();
    let d = t.nfree + 1;
    let (mut s1, mut s2) = (vec![0f64; d], vec![0f64; d]);
    let mut totals = Vec::with_capacity(reps);
    for r in 0..reps {
        let run = smc(t, np, seed0 + r as u64);
        for k in 0..d {
            s1[k] += run.est[k];
            s2[k] += run.est[k] * run.est[k];
        }
        totals.push(run.est[1..].iter().sum::<f64>());
    }
    let mean: Vec<f64> = s1.iter().map(|x| x / reps as f64).collect();
    let sd: Vec<f64> = (0..d)
        .map(|k| {
            ((s2[k] / reps as f64 - mean[k] * mean[k]).max(0.0) * reps as f64 / (reps as f64 - 1.0))
                .sqrt()
        })
        .collect();
    format!(
        "{{\"label\":\"{label}\",\"np\":{np},\"reps\":{reps},\"exact_complete\":{complete},\"enum_seconds\":{enum_s:.2},\"exact\":{},\"mean\":{},\"sd\":{},\"totals\":{}}}",
        fmt_vec(&w),
        fmt_f(&mean),
        fmt_f(&sd),
        fmt_f(&totals)
    )
}

/// Paint an instance from the official census: ring joints get the official
/// frame-colour joint multiset, seam and inner joints a uniform deal of the
/// official inner-colour joint multiset. Rejects duplicate or
/// rotation-symmetric pieces. Clues: the painted pieces at the official cells
/// with the official rotations' cells (rotation 0 of the painted cell).
pub(crate) fn paint(official: &Puzzle, seed: u64) -> Option<Puzzle> {
    let n = official.n;
    let mut rng = Rng::new(seed ^ 0x9A17_0000);
    // sides of each colour, split into frame-lateral and the rest
    let mut lateral: Vec<u8> = Vec::new();
    let mut inner: Vec<u8> = Vec::new();
    for e in &official.pieces {
        let g = e.iter().filter(|&&c| c == 0).count();
        for s in 0..4 {
            if e[s] == 0 {
                continue;
            }
            let lat = g > 0 && (e[(s + 1) % 4] == 0 || e[(s + 3) % 4] == 0);
            if lat {
                lateral.push(e[s])
            } else {
                inner.push(e[s])
            }
        }
    }
    lateral.sort_unstable();
    inner.sort_unstable();
    let joints = |v: &Vec<u8>| -> Vec<u8> { v.iter().step_by(2).copied().collect() };
    let mut lj = joints(&lateral);
    let mut ij = joints(&inner);
    rng.shuffle(&mut lj);
    rng.shuffle(&mut ij);
    let is_frame = |x: usize, y: usize| x == 0 || y == 0 || x == n - 1 || y == n - 1;
    let mut h = vec![0u8; n * n];
    let mut v = vec![0u8; n * n];
    let (mut li, mut ii) = (0, 0);
    for y in 0..n {
        for x in 0..n {
            if x + 1 < n {
                let ring = (y == 0 || y == n - 1) && is_frame(x + 1, y);
                h[y * n + x] = if ring {
                    li += 1;
                    lj[li - 1]
                } else {
                    ii += 1;
                    ij[ii - 1]
                };
            }
            if y + 1 < n {
                let ring = (x == 0 || x == n - 1) && is_frame(x, y + 1);
                v[y * n + x] = if ring {
                    li += 1;
                    lj[li - 1]
                } else {
                    ii += 1;
                    ij[ii - 1]
                };
            }
        }
    }
    assert_eq!((li, ii), (lj.len(), ij.len()));
    let mut cells = Vec::new();
    for y in 0..n {
        for x in 0..n {
            let t = if y == 0 { 0 } else { v[(y - 1) * n + x] };
            let b = if y == n - 1 { 0 } else { v[y * n + x] };
            let l = if x == 0 { 0 } else { h[y * n + x - 1] };
            let r = if x == n - 1 { 0 } else { h[y * n + x] };
            cells.push([t, r, b, l]);
        }
    }
    let canon = |e: [u8; 4]| (0..4).map(|r| puzzle::rotate(e, r)).min().unwrap();
    let mut seen = std::collections::HashSet::new();
    for &e in &cells {
        if (1..4).any(|r| puzzle::rotate(e, r) == e) || !seen.insert(canon(e)) {
            return None;
        }
    }
    let hints = official
        .hints
        .iter()
        .map(|&(pos, _, _)| (pos, pos as u16, 0u8))
        .collect();
    Some(Puzzle {
        n,
        pieces: cells,
        hints,
    })
}

fn main() {
    let (cmd, m) = args_map();
    match cmd.as_str() {
        "bw" => {
            let p = load_csv(m.get("csv").expect("--csv"));
            let unlock: Vec<usize> = m
                .get("unlock")
                .expect("--unlock")
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            let seed: u64 = get(&m, "seed", 1u64);
            let cap = num(&m, "cap", 1e10) as u64;
            let quota = !m.contains_key("noquota");
            let label = m.get("label").cloned().unwrap_or_default();
            let cps: Vec<u64> = [1e7, 1e8, 1e9, 1e10]
                .iter()
                .map(|&x| x as u64)
                .filter(|&x| x < cap)
                .collect();
            let mut s = bw::Solver::new(&p, &[22, 21, 20], quota, unlock.clone(), 10, cap, cps);
            let t0 = Instant::now();
            let handle = std::thread::Builder::new()
                .stack_size(256 * 1024 * 1024)
                .spawn(move || {
                    s.run(seed);
                    s
                })
                .unwrap();
            let s = handle.join().unwrap();
            let secs = t0.elapsed().as_secs_f64();
            let snaps: Vec<String> = s
                .snapshots
                .iter()
                .map(|(n, h)| format!("[{},{}]", n, fmt_vec(h)))
                .collect();
            println!(
                "{{\"label\":\"{label}\",\"seed\":{seed},\"unlock\":{},\"quota\":{quota},\"cap\":{cap},\"nodes\":{},\"seconds\":{secs:.1},\"deepest\":{},\"breaks_at_deepest\":{},\"completions\":{},\"best_complete_score\":{},\"hist\":{},\"checkpoints\":[{}]}}",
                fmt_vec(&unlock),
                s.nodes,
                s.deepest,
                s.breaks_at_deepest,
                s.completions,
                s.best_score,
                fmt_vec(&s.hist),
                snaps.join(",")
            );
        }
        "bwsmc" | "bwval" => {
            let p = load_csv(m.get("csv").expect("--csv"));
            let unlock: Vec<usize> = m
                .get("unlock")
                .expect("--unlock")
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            let quota = !m.contains_key("noquota");
            let np = num(&m, "np", 1e5) as usize;
            let reps: usize = get(&m, "reps", 5usize);
            let seed0: u64 = get(&m, "seed0", 1u64);
            let label = m.get("label").cloned().unwrap_or_default();
            let mut s = bw::Solver::new(
                &p,
                &[22, 21, 20],
                quota,
                unlock.clone(),
                10,
                u64::MAX,
                vec![],
            );
            let root = s.root_state(1);
            let fmt_est = |e: &[f64]| fmt_f(e);
            if m.contains_key("exact-root") {
                let cap = num(&m, "cap", 1e8) as u64;
                let (h, ok) = s.exact_from(&root, 0, cap);
                println!(
                    "{{\"label\":\"{label}\",\"cap\":{cap},\"complete\":{ok},\"hist\":{}}}",
                    fmt_vec(&h)
                );
                return;
            }
            if cmd == "bwsmc" {
                for r in 0..reps {
                    let t0 = Instant::now();
                    let est = s.smc(&root, 0, np, seed0 + r as u64);
                    println!("{{\"label\":\"{label}\",\"unlock\":{},\"quota\":{quota},\"np\":{np},\"rep\":{r},\"seed\":{},\"seconds\":{:.1},\"entries\":{}}}", fmt_vec(&unlock), seed0 + r as u64, t0.elapsed().as_secs_f64(), fmt_est(&est));
                }
            } else {
                let d0: usize = get(&m, "d0", 190usize);
                let count: usize = get(&m, "count", 10usize);
                let cap = num(&m, "cap", 2e8) as u64;
                let mut rng = Rng::new(seed0 ^ 0xB0B);
                let (mut done, mut tries) = (0, 0);
                let _ = &mut rng;
                let pool = s.capture(
                    &root,
                    d0,
                    num(&m, "pool", 400.0) as usize,
                    num(&m, "stride", 50.0) as u64,
                    5e9 as u64,
                );
                let need: usize = get(&m, "need", 200usize);
                eprintln!("# captured {} states at depth {d0}", pool.len());
                for st in pool {
                    if done >= count {
                        break;
                    }
                    tries += 1;
                    let (h, ok) = s.exact_from(&st, d0, cap);
                    if !ok || h[need..].iter().sum::<u64>() == 0 {
                        continue;
                    }
                    let mut mean = vec![0f64; 257];
                    let mut sq = vec![0f64; 257];
                    for r in 0..reps {
                        let est = s.smc(&st, d0, np, seed0 * 1000 + (done * reps + r) as u64);
                        for k in 0..257 {
                            mean[k] += est[k] / reps as f64;
                            sq[k] += est[k] * est[k] / reps as f64;
                        }
                    }
                    let sd: Vec<f64> = (0..257)
                        .map(|k| {
                            ((sq[k] - mean[k] * mean[k]).max(0.0) * reps as f64
                                / (reps as f64 - 1.0))
                                .sqrt()
                        })
                        .collect();
                    println!("{{\"label\":\"{label}\",\"d0\":{d0},\"prefix\":{done},\"tries\":{tries},\"np\":{np},\"reps\":{reps},\"exact\":{},\"mean\":{},\"sd\":{}}}", fmt_vec(&h), fmt_f(&mean), fmt_f(&sd));
                    done += 1;
                }
            }
        }
        "smc" => {
            let (p, fixed, order) = official_config(&m);
            let np = num(&m, "np", 1e5) as usize;
            let reps: usize = get(&m, "reps", 5usize);
            let seed0: u64 = get(&m, "seed0", 1u64);
            let label = m.get("label").cloned().unwrap_or_default();
            let t = Tree::new(&p, &order, &fixed);
            eprintln!("# {label}: {} free cells, {} fixed", t.nfree, fixed.len());
            for r in 0..reps {
                let t0 = Instant::now();
                let run = smc(&t, np, seed0 + r as u64);
                println!(
                    "{}",
                    smc_json(
                        &label,
                        r,
                        seed0 + r as u64,
                        np,
                        &run,
                        t0.elapsed().as_secs_f64()
                    )
                );
            }
        }
        "enum" => {
            let (p, fixed, order) = official_config(&m);
            let cap = num(&m, "cap", 1e10) as u64;
            let label = m.get("label").cloned().unwrap_or_default();
            let t = Tree::new(&p, &order, &fixed);
            let t0 = Instant::now();
            let (w, complete) = t.enumerate(cap);
            let total: u64 = w[1..].iter().sum();
            println!(
                "{{\"label\":\"{label}\",\"free\":{},\"complete\":{complete},\"total\":{total},\"seconds\":{:.1},\"widths\":{}}}",
                t.nfree,
                t0.elapsed().as_secs_f64(),
                fmt_vec(&w)
            );
        }
        "gate-a" => {
            let path = m.get("jsonl").expect("--jsonl");
            let np = num(&m, "np", 200.0) as usize;
            let reps: usize = get(&m, "reps", 200usize);
            let cap = num(&m, "cap", 2e10) as u64;
            for (li, line) in std::fs::read_to_string(path).unwrap().lines().enumerate() {
                let (id, n, pieces) = parse_pieces(line);
                if n != 6 {
                    continue;
                }
                let p = Puzzle {
                    n,
                    pieces,
                    hints: vec![],
                };
                let t = Tree::new(&p, &rows_bottom_up(n), &[]);
                println!(
                    "{}",
                    check_config(&id, &t, np, reps, cap, 1000 * li as u64 + 1)
                );
            }
        }
        "gate-gen" => {
            let np = num(&m, "np", 1000.0) as usize;
            let reps: usize = get(&m, "reps", 100usize);
            let cap = num(&m, "cap", 2e10) as u64;
            for &(n, cf, ci) in &[(5usize, 3u8, 4u8), (6, 3, 5)] {
                for seed in 1..=10u64 {
                    let (p, planted) = puzzle::generate(n, cf, ci, seed);
                    let mut cfgs: Vec<(String, Vec<(usize, u16, u8)>)> =
                        vec![(format!("gen{n}-s{seed}-none"), vec![])];
                    if n == 6 {
                        cfgs.push((
                            format!("gen{n}-s{seed}-clue"),
                            planted
                                .iter()
                                .copied()
                                .filter(|x| x.0 == 3 * n + 2)
                                .collect(),
                        ));
                        let frame: Vec<_> = planted
                            .iter()
                            .copied()
                            .filter(|x| {
                                let (r, c) = (x.0 / n, x.0 % n);
                                r == 0 || c == 0 || r == n - 1 || c == n - 1
                            })
                            .collect();
                        cfgs.push((format!("gen{n}-s{seed}-frame"), frame));
                    }
                    for (label, fixed) in cfgs {
                        let order = free_order(rows_bottom_up(n), &fixed);
                        let t = Tree::new(&p, &order, &fixed);
                        println!("{}", check_config(&label, &t, np, reps, cap, seed * 7919));
                    }
                }
            }
        }
        "paint" => {
            let official = load_csv(m.get("csv").expect("--csv"));
            let count: usize = get(&m, "count", 20usize);
            let reps: usize = get(&m, "reps", 2usize);
            let np = num(&m, "np", 1e5) as usize;
            let mut seed = 1u64;
            let mut done = 0;
            let mut rejected = 0;
            while done < count {
                if let Some(p) = paint(&official, seed) {
                    let fixed = p.hints.clone();
                    let order = free_order(rows_bottom_up(16), &fixed);
                    let t = Tree::new(&p, &order, &fixed);
                    for r in 0..reps {
                        let t0 = Instant::now();
                        let run = smc(&t, np, seed * 1000 + r as u64);
                        println!(
                            "{}",
                            smc_json(
                                &format!("paint-s{seed}"),
                                r,
                                seed * 1000 + r as u64,
                                np,
                                &run,
                                t0.elapsed().as_secs_f64()
                            )
                        );
                    }
                    done += 1;
                } else {
                    rejected += 1;
                }
                seed += 1;
            }
            eprintln!("# painted {done}, rejected {rejected}");
        }
        "paint-boards" => oap::paint_boards(&m),
        "tsmc" => oap::tsmc(&m),
        "tenum" => oap::tenum(&m),
        "gate6" => oap::gate6(&m),
        "tppd" => oap::tppd(&m),
        "tenumd" => oap::tenumd(&m),
        _ => {
            eprintln!("usage: rpp bw|smc|enum|gate-a|gate-gen|paint|paint-boards|tsmc|tenum|gate6 [--key value ...]");
            std::process::exit(2);
        }
    }
}
