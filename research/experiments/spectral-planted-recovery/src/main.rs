//! A1 spectral planted recovery (see PREREG.md, which is binding).
//!
//! For each generated framed puzzle: scramble it exactly as `generate_framed`
//! does while tracking the planted solution, build the rarity-weighted
//! compatibility matrix W of the scrambled piece list, and for every non-grey
//! half-edge rank its colour-compatible candidates by several scores. Record
//! the (mid-)rank of the planted neighbour. Methods see only the scrambled
//! pieces; the solution is used for scoring only.

use eternity2_engine::generator::XorShift;
use eternity2_engine::{generate_solved_framed_bc, rotated};
use serde_json::{json, Map, Value};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

type Piece = [u8; 4];

const SCRAMBLE_XOR: u32 = 0xA5A5_A5A5;
const RESCRAMBLE_XOR: u32 = 0x5EED_1234;
const UNIFORM_XOR: u32 = 0x00C0_FFEE;
const K_SWEEP: [usize; 6] = [1, 2, 4, 8, 16, 32];
const STRATA: [&str; 3] = ["all", "deep", "rim"];

struct Config {
    id: &'static str,
    size: usize,
    colors: u8,
    border: u8,
    primary: bool,
}

const CONFIGS: [Config; 4] = [
    Config { id: "n8_c8", size: 8, colors: 8, border: 2, primary: true },
    Config { id: "n10_c12", size: 10, colors: 12, border: 3, primary: true },
    Config { id: "n8_c14", size: 8, colors: 14, border: 2, primary: false },
    Config { id: "n10_c21", size: 10, colors: 21, border: 3, primary: false },
];

/// A piece list as a method sees it, plus (for scoring only) where each piece
/// sits in the planted solution: piece `t` is `rotated(original, rot[t])` and
/// belongs at `cell[t]`.
#[derive(Clone)]
struct Inst {
    n: usize,
    pieces: Vec<Piece>,
    cell: Vec<usize>,
    rot: Vec<u8>,
}

fn solved(size: usize, colors: u8, seed: u32, border: u8) -> Inst {
    let p = generate_solved_framed_bc(size as u8, colors, seed, true, border);
    let n2 = size * size;
    Inst { n: size, pieces: p.pieces, cell: (0..n2).collect(), rot: vec![0; n2] }
}

/// The scramble of `generate_framed` (shuffle, then one `below(4)` rotation
/// per piece, in list order), carrying the solution bookkeeping along.
fn scramble(inst: &Inst, rng_seed: u32) -> Inst {
    let mut items: Vec<(Piece, usize, u8)> = (0..inst.pieces.len())
        .map(|t| (inst.pieces[t], inst.cell[t], inst.rot[t]))
        .collect();
    let mut rng = XorShift::new(rng_seed);
    rng.shuffle(&mut items);
    for it in &mut items {
        let r = rng.below(4) as u8;
        it.0 = rotated(it.0, r);
        it.2 = (it.2 + r) % 4;
    }
    Inst {
        n: inst.n,
        pieces: items.iter().map(|x| x.0).collect(),
        cell: items.iter().map(|x| x.1).collect(),
        rot: items.iter().map(|x| x.2).collect(),
    }
}

fn is_interior(p: &Piece) -> bool {
    p.iter().all(|&c| c != 0)
}

/// Planted neighbour of half-edge (t, k): Some((u, j)) or None for a rim side.
fn true_neighbour(inst: &Inst, inv: &[usize], t: usize, k: usize) -> Option<(usize, usize)> {
    let n = inst.n;
    let s = (k + 4 - inst.rot[t] as usize) % 4; // side in solution orientation
    let (x, y) = (inst.cell[t] % n, inst.cell[t] / n);
    let (nx, ny) = match s {
        0 => (x as isize, y as isize - 1),
        1 => (x as isize + 1, y as isize),
        2 => (x as isize, y as isize + 1),
        _ => (x as isize - 1, y as isize),
    };
    if nx < 0 || ny < 0 || nx >= n as isize || ny >= n as isize {
        return None;
    }
    let u = inv[ny as usize * n + nx as usize];
    let j = ((s + 2) % 4 + inst.rot[u] as usize) % 4;
    Some((u, j))
}

fn inverse_cells(inst: &Inst) -> Vec<usize> {
    let mut inv = vec![usize::MAX; inst.pieces.len()];
    for (t, &c) in inst.cell.iter().enumerate() {
        inv[c] = t;
    }
    inv
}

/// Every planted adjacency joins equal non-grey colours and every rim side is grey.
fn tiling_ok(inst: &Inst) -> bool {
    let inv = inverse_cells(inst);
    if inv.contains(&usize::MAX) {
        return false;
    }
    for t in 0..inst.pieces.len() {
        for k in 0..4 {
            let c = inst.pieces[t][k];
            match true_neighbour(inst, &inv, t, k) {
                None => {
                    if c != 0 {
                        return false;
                    }
                }
                Some((u, j)) => {
                    if c == 0 || inst.pieces[u][j] != c {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn colour_counts(pieces: &[Piece]) -> Vec<usize> {
    let mut cnt = vec![0usize; 256];
    for p in pieces {
        for &c in p {
            cnt[c as usize] += 1;
        }
    }
    cnt
}

/// W[p][q] = sum over side pairs with equal non-grey colour c of 1 / n_c.
fn compat_matrix(pieces: &[Piece], cnt: &[usize]) -> Vec<f64> {
    let nn = pieces.len();
    let mut w = vec![0.0; nn * nn];
    for p in 0..nn {
        for q in 0..nn {
            if p == q {
                continue;
            }
            let mut s = 0.0;
            for &a in &pieces[p] {
                if a == 0 {
                    continue;
                }
                for &b in &pieces[q] {
                    if a == b {
                        s += 1.0 / cnt[a as usize] as f64;
                    }
                }
            }
            w[p * nn + q] = s;
        }
    }
    w
}

/// Cyclic Jacobi eigen-decomposition of a symmetric n x n matrix (row-major).
/// Returns (eigenvalues, eigenvectors) with eigenvector l stored as column l
/// (`vecs[i * n + l]`), sorted by descending eigenvalue.
fn jacobi_eigen(a0: &[f64], n: usize) -> (Vec<f64>, Vec<f64>) {
    let mut a = a0.to_vec();
    let mut v = vec![0.0; n * n];
    for i in 0..n {
        v[i * n + i] = 1.0;
    }
    let frob2: f64 = a0.iter().map(|x| x * x).sum();
    for _sweep in 0..100 {
        let mut off = 0.0;
        for i in 0..n {
            for j in i + 1..n {
                off += a[i * n + j] * a[i * n + j];
            }
        }
        if off <= 1e-26 * frob2.max(1e-300) {
            break;
        }
        for p in 0..n {
            for q in p + 1..n {
                let apq = a[p * n + q];
                if apq == 0.0 {
                    continue;
                }
                let theta = (a[q * n + q] - a[p * n + p]) / (2.0 * apq);
                let sgn = if theta >= 0.0 { 1.0 } else { -1.0 };
                let t = sgn / (theta.abs() + (theta * theta + 1.0).sqrt());
                let c = 1.0 / (t * t + 1.0).sqrt();
                let s = t * c;
                for k in 0..n {
                    let akp = a[k * n + p];
                    let akq = a[k * n + q];
                    a[k * n + p] = c * akp - s * akq;
                    a[k * n + q] = s * akp + c * akq;
                }
                for k in 0..n {
                    let apk = a[p * n + k];
                    let aqk = a[q * n + k];
                    a[p * n + k] = c * apk - s * aqk;
                    a[q * n + k] = s * apk + c * aqk;
                }
                for k in 0..n {
                    let vkp = v[k * n + p];
                    let vkq = v[k * n + q];
                    v[k * n + p] = c * vkp - s * vkq;
                    v[k * n + q] = s * vkp + c * vkq;
                }
            }
        }
    }
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&x, &y| a[y * n + y].partial_cmp(&a[x * n + x]).unwrap().then(x.cmp(&y)));
    let vals: Vec<f64> = order.iter().map(|&l| a[l * n + l]).collect();
    let mut vecs = vec![0.0; n * n];
    for (newl, &l) in order.iter().enumerate() {
        for i in 0..n {
            vecs[i * n + newl] = v[i * n + l];
        }
    }
    (vals, vecs)
}

fn low_rank(vals: &[f64], vecs: &[f64], n: usize, k: usize) -> Vec<f64> {
    let k = k.min(n);
    let mut s = vec![0.0; n * n];
    for p in 0..n {
        for q in 0..n {
            let mut acc = 0.0;
            for l in 0..k {
                acc += vals[l] * vecs[p * n + l] * vecs[q * n + l];
            }
            s[p * n + q] = acc;
        }
    }
    s
}

fn max_abs_residual(w: &[f64], vals: &[f64], vecs: &[f64], n: usize) -> f64 {
    let full = low_rank(vals, vecs, n, n);
    w.iter().zip(&full).map(|(a, b)| (a - b).abs()).fold(0.0, f64::max)
}

/// Mid-rank of the true candidate (index `ti` in `scores`), 1 = best.
/// Returns (rank, strictly_top).
fn mid_rank(scores: &[f64], ti: usize) -> (f64, bool) {
    let maxabs = scores.iter().fold(0.0f64, |m, s| m.max(s.abs()));
    let tol = 1e-9 * maxabs.max(1.0);
    let t = scores[ti];
    let mut higher = 0usize;
    let mut ties = 0usize;
    for (i, &s) in scores.iter().enumerate() {
        if i == ti {
            continue;
        }
        if (s - t).abs() <= tol {
            ties += 1;
        } else if s > t {
            higher += 1;
        }
    }
    (1.0 + higher as f64 + ties as f64 / 2.0, higher == 0 && ties == 0)
}

#[derive(Default, Clone, Copy)]
struct Acc {
    raw: f64,
    norm: f64,
    top1: f64,
    count: usize,
}

impl Acc {
    fn add(&mut self, r: f64, m: usize, top: bool) {
        self.raw += r;
        self.norm += r / m as f64;
        self.top1 += if top { 1.0 } else { 0.0 };
        self.count += 1;
    }
    fn json(&self) -> Value {
        let c = self.count.max(1) as f64;
        json!({"raw": self.raw / c, "norm": self.norm / c, "top1": self.top1 / c, "count": self.count})
    }
}

/// One half-edge's evaluation context.
struct HalfEdge {
    p: usize,
    cands: Vec<(usize, usize)>,
    ti: usize,
    stratum: usize, // 1 = deep, 2 = rim
}

fn half_edges(inst: &Inst) -> Vec<HalfEdge> {
    let inv = inverse_cells(inst);
    let nn = inst.pieces.len();
    let mut out = Vec::new();
    for p in 0..nn {
        for k in 0..4 {
            let c = inst.pieces[p][k];
            if c == 0 {
                continue;
            }
            let (u, j) = true_neighbour(inst, &inv, p, k).expect("non-grey side has a neighbour");
            let mut cands = Vec::new();
            let mut ti = usize::MAX;
            for q in 0..nn {
                if q == p {
                    continue;
                }
                for jj in 0..4 {
                    if inst.pieces[q][jj] == c {
                        if q == u && jj == j {
                            ti = cands.len();
                        }
                        cands.push((q, jj));
                    }
                }
            }
            assert!(ti != usize::MAX, "true neighbour missing from candidate set");
            let deep = is_interior(&inst.pieces[p]) && is_interior(&inst.pieces[u]);
            out.push(HalfEdge { p, cands, ti, stratum: if deep { 1 } else { 2 } });
        }
    }
    out
}

fn eval_matrix(hes: &[HalfEdge], s: &[f64], nn: usize) -> [Acc; 3] {
    let mut acc = [Acc::default(); 3];
    for he in hes {
        let scores: Vec<f64> = he.cands.iter().map(|&(q, _)| s[he.p * nn + q]).collect();
        let (r, top) = mid_rank(&scores, he.ti);
        acc[0].add(r, he.cands.len(), top);
        acc[he.stratum].add(r, he.cands.len(), top);
    }
    acc
}

fn acc_json(acc: &[Acc; 3]) -> Value {
    let mut m = Map::new();
    for (i, s) in STRATA.iter().enumerate() {
        m.insert(s.to_string(), acc[i].json());
    }
    Value::Object(m)
}

/// All piece-level score matrices of the registered methods, by name.
fn method_matrices(inst: &Inst) -> (Vec<(String, Vec<f64>)>, Value) {
    let nn = inst.pieces.len();
    let cnt = colour_counts(&inst.pieces);
    let w = compat_matrix(&inst.pieces, &cnt);
    let (vals, vecs) = jacobi_eigen(&w, nn);
    let kp = (nn as f64).sqrt().ceil() as usize;
    let mut out: Vec<(String, Vec<f64>)> = Vec::new();
    out.push(("rarity".into(), w.clone()));
    let sp = low_rank(&vals, &vecs, nn, kp);
    let spmax = sp.iter().fold(0.0f64, |m, x| m.max(x.abs())).max(1e-300);
    let comb: Vec<f64> = w.iter().zip(&sp).map(|(a, b)| a + 1e-6 * b / spmax).collect();
    out.push(("spectral_primary".into(), sp));
    for &k in &K_SWEEP {
        out.push((format!("spectral_k{k}"), low_rank(&vals, &vecs, nn, k)));
    }
    out.push(("combined".into(), comb));
    let mut cr = vec![0.0; nn * nn];
    for q in 0..nn {
        let r: f64 = inst.pieces[q].iter().filter(|&&c| c != 0).map(|&c| 1.0 / cnt[c as usize] as f64).sum();
        for p in 0..nn {
            cr[p * nn + q] = r;
        }
    }
    out.push(("cand_rarity".into(), cr));
    let mut idd = vec![0.0; nn * nn];
    for p in 0..nn {
        for q in 0..nn {
            idd[p * nn + q] = -((p as f64) - (q as f64)).abs();
        }
    }
    out.push(("id_distance".into(), idd));
    let eig = json!({
        "k_primary": kp,
        "lambda_top12": vals.iter().take(12).cloned().collect::<Vec<f64>>(),
        "gap_at_k_primary": vals[kp - 1] - vals[kp],
        "gap_at_k_sweep": K_SWEEP.iter().map(|&k| vals[k - 1] - vals[k]).collect::<Vec<f64>>(),
        "max_abs_residual": max_abs_residual(&w, &vals, &vecs, nn),
    });
    (out, eig)
}

type MethodResults = Vec<(String, [Acc; 3])>;

fn eval_all(inst: &Inst) -> (MethodResults, Value, Vec<HalfEdge>) {
    let hes = half_edges(inst);
    let nn = inst.pieces.len();
    let (mats, eig) = method_matrices(inst);
    let res = mats.iter().map(|(name, s)| (name.clone(), eval_matrix(&hes, s, nn))).collect();
    (res, eig, hes)
}

fn run_instance(cfg: &Config, seed: u32) -> Value {
    let base = solved(cfg.size, cfg.colors, seed, cfg.border);
    let scr = scramble(&base, seed ^ SCRAMBLE_XOR);
    let rescr = scramble(&scr, seed ^ RESCRAMBLE_XOR);
    let tiling = tiling_ok(&base) && tiling_ok(&scr) && tiling_ok(&rescr);
    assert!(tiling, "planted solution does not tile ({} seed {seed})", cfg.id);

    let (res, eig, hes) = eval_all(&scr);

    // Uniform baselines: analytic expectation and a simulated random ranking.
    let mut ua = [(0.0f64, 0.0f64, 0usize); 3];
    let mut usim = [Acc::default(); 3];
    let mut rng = XorShift::new(seed ^ UNIFORM_XOR);
    let mut msum = 0.0;
    for he in &hes {
        let m = he.cands.len();
        msum += m as f64;
        for s in [0, he.stratum] {
            ua[s].0 += (m as f64 + 1.0) / 2.0;
            ua[s].1 += (m as f64 + 1.0) / (2.0 * m as f64);
            ua[s].2 += 1;
        }
        let r = 1.0 + rng.below(m as u32) as f64;
        usim[0].add(r, m, r == 1.0);
        usim[he.stratum].add(r, m, r == 1.0);
    }
    let mut uaj = Map::new();
    for (i, s) in STRATA.iter().enumerate() {
        let c = ua[i].2.max(1) as f64;
        uaj.insert(s.to_string(), json!({"raw": ua[i].0 / c, "norm": ua[i].1 / c, "count": ua[i].2}));
    }

    let mut methods = Map::new();
    for (name, acc) in &res {
        methods.insert(name.clone(), acc_json(acc));
    }
    methods.insert("uniform_sim".into(), acc_json(&usim));

    // Leakage check 1: rerun on an independent re-scramble.
    let (res2, _, _) = eval_all(&rescr);
    let mut diffs = Map::new();
    let mut id_rescr = Value::Null;
    for ((name, a), (_, b)) in res.iter().zip(&res2) {
        let d = (0..3)
            .map(|i| (a[i].norm / a[i].count.max(1) as f64 - b[i].norm / b[i].count.max(1) as f64).abs())
            .fold(0.0, f64::max);
        if name == "id_distance" {
            id_rescr = acc_json(b);
        } else {
            diffs.insert(name.clone(), json!(d));
        }
    }
    // Leakage check 2: the same methods on the unscrambled solved order.
    let (res_solved, _, _) = eval_all(&base);
    let mut solved_j = Map::new();
    for (name, acc) in &res_solved {
        if name == "rarity" || name == "spectral_primary" || name == "id_distance" {
            solved_j.insert(name.clone(), acc_json(acc));
        }
    }

    let cnt = colour_counts(&scr.pieces);
    let census: Vec<usize> = (1..=cfg.colors as usize).map(|c| cnt[c]).collect();
    json!({
        "config": cfg.id,
        "seed": seed,
        "size": cfg.size,
        "colors": cfg.colors,
        "border_colors": cfg.border,
        "pieces": scr.pieces.len(),
        "half_edges": hes.len(),
        "mean_candidates": msum / hes.len() as f64,
        "colour_census_half_edges": census,
        "tiling_ok": tiling,
        "methods": methods,
        "uniform_analytic": uaj,
        "eig": eig,
        "leak": {
            "rescramble_max_abs_diff_norm": diffs,
            "id_distance_rescrambled": id_rescr,
            "solved_order": solved_j,
        },
    })
}

fn mean_sd(x: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let m = x.iter().sum::<f64>() / n;
    let v = if x.len() > 1 { x.iter().map(|a| (a - m) * (a - m)).sum::<f64>() / (n - 1.0) } else { 0.0 };
    (m, v.sqrt())
}

fn get(v: &Value, path: &[&str]) -> f64 {
    let mut cur = v;
    for p in path {
        cur = &cur[*p];
    }
    cur.as_f64().unwrap_or_else(|| panic!("missing {path:?}"))
}

/// Paired test: d_i = base_i - other_i (positive = `other` ranks the true neighbour higher).
fn paired(rows: &[Value], base: &[&str], other: &[&str]) -> Value {
    let d: Vec<f64> = rows.iter().map(|r| get(r, base) - get(r, other)).collect();
    let (m, sd) = mean_sd(&d);
    let se = sd / (d.len() as f64).sqrt();
    json!({"dbar": m, "se": se, "z": if se > 0.0 { m / se } else { f64::NAN }, "pass": m > 2.0 * se, "n": d.len()})
}

fn summarise(cfg: &Config, rows: &[Value]) -> Value {
    let methods: Vec<String> = rows[0]["methods"].as_object().unwrap().keys().cloned().collect();
    let mut per_method = Map::new();
    for m in &methods {
        let mut st = Map::new();
        for s in STRATA {
            let mut o = Map::new();
            for metric in ["norm", "raw", "top1"] {
                let x: Vec<f64> = rows.iter().map(|r| get(r, &["methods", m, s, metric])).collect();
                let (mu, sd) = mean_sd(&x);
                o.insert(metric.into(), json!({"mean": mu, "sd": sd}));
            }
            st.insert(s.into(), Value::Object(o));
        }
        per_method.insert(m.clone(), Value::Object(st));
    }
    let mut uni = Map::new();
    for s in STRATA {
        let mut o = Map::new();
        for metric in ["norm", "raw"] {
            let x: Vec<f64> = rows.iter().map(|r| get(r, &["uniform_analytic", s, metric])).collect();
            o.insert(metric.into(), json!(mean_sd(&x).0));
        }
        uni.insert(s.into(), Value::Object(o));
    }
    let mut vs_rarity = Map::new();
    for m in &methods {
        if m == "rarity" {
            continue;
        }
        let mut st = Map::new();
        for s in STRATA {
            let mut o = Map::new();
            for metric in ["norm", "raw"] {
                o.insert(metric.into(), paired(rows, &["methods", "rarity", s, metric], &["methods", m, s, metric]));
            }
            st.insert(s.into(), Value::Object(o));
        }
        vs_rarity.insert(m.clone(), Value::Object(st));
    }
    let mut vs_uniform = Map::new();
    for m in ["rarity", "spectral_primary", "combined", "cand_rarity"] {
        let mut st = Map::new();
        for s in STRATA {
            st.insert(s.into(), paired(rows, &["uniform_analytic", s, "norm"], &["methods", m, s, "norm"]));
        }
        vs_uniform.insert(m.into(), Value::Object(st));
    }
    let mut leak_max = Map::new();
    for m in rows[0]["leak"]["rescramble_max_abs_diff_norm"].as_object().unwrap().keys() {
        let x = rows.iter().map(|r| get(r, &["leak", "rescramble_max_abs_diff_norm", m])).fold(0.0, f64::max);
        leak_max.insert(m.clone(), json!(x));
    }
    let mut solved_mean = Map::new();
    for m in ["rarity", "spectral_primary", "id_distance"] {
        let x: Vec<f64> = rows.iter().map(|r| get(r, &["leak", "solved_order", m, "all", "norm"])).collect();
        solved_mean.insert(m.into(), json!(mean_sd(&x).0));
    }
    let idr: Vec<f64> = rows.iter().map(|r| get(r, &["leak", "id_distance_rescrambled", "all", "norm"])).collect();
    let mc: Vec<f64> = rows.iter().map(|r| get(r, &["mean_candidates"])).collect();
    let gaps: Vec<f64> = rows.iter().map(|r| get(r, &["eig", "gap_at_k_primary"])).collect();
    let small_gap_by_k: Map<String, Value> = K_SWEEP
        .iter()
        .enumerate()
        .map(|(i, k)| {
            let c = rows.iter().filter(|r| r["eig"]["gap_at_k_sweep"][i].as_f64().unwrap().abs() < 1e-9).count();
            (format!("k{k}"), json!(c))
        })
        .collect();
    let resid = rows.iter().map(|r| get(r, &["eig", "max_abs_residual"])).fold(0.0, f64::max);
    let kill = &vs_rarity["spectral_primary"]["all"]["norm"];
    json!({
        "config": cfg.id,
        "size": cfg.size,
        "colors": cfg.colors,
        "border_colors": cfg.border,
        "primary": cfg.primary,
        "n_puzzles": rows.len(),
        "half_edges_per_puzzle": rows[0]["half_edges"],
        "mean_candidates": mean_sd(&mc).0,
        "all_tiling_ok": rows.iter().all(|r| r["tiling_ok"] == json!(true)),
        "methods": per_method,
        "uniform_analytic": uni,
        "paired_vs_rarity": vs_rarity,
        "paired_vs_uniform_analytic": vs_uniform,
        "kill_test": {
            "statistic": "dbar = mean_i(NR_i(rarity) - NR_i(spectral_primary)), all half-edges, normalised rank",
            "dbar": kill["dbar"], "se": kill["se"], "z": kill["z"], "pass": kill["pass"],
        },
        "eig": {
            "min_gap_at_k_primary": gaps.iter().cloned().fold(f64::INFINITY, f64::min),
            "puzzles_with_gap_below_1e-9": gaps.iter().filter(|g| g.abs() < 1e-9).count(),
            "puzzles_with_gap_below_1e-9_by_sweep_k": small_gap_by_k,
            "max_abs_residual": resid,
        },
        "leak": {
            "rescramble_max_abs_diff_norm": leak_max,
            "solved_order_mean_norm": solved_mean,
            "id_distance_scrambled_mean_norm": get(&per_method["id_distance"], &["all", "norm", "mean"]),
            "id_distance_rescrambled_mean_norm": mean_sd(&idr).0,
        },
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let flag = |name: &str| args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).cloned();
    let out = PathBuf::from(flag("--out").unwrap_or_else(|| "results".into()));
    let seeds: u32 = flag("--seeds").and_then(|s| s.parse().ok()).unwrap_or(200);
    fs::create_dir_all(out.join("raw")).expect("create out dir");
    let t0 = Instant::now();
    let mut summaries = Map::new();
    for cfg in &CONFIGS {
        let t = Instant::now();
        let rows: Vec<Value> = (1..=seeds).map(|s| run_instance(cfg, s)).collect();
        let mut f = fs::File::create(out.join("raw").join(format!("{}.jsonl", cfg.id))).unwrap();
        for r in &rows {
            writeln!(f, "{}", serde_json::to_string(r).unwrap()).unwrap();
        }
        let s = summarise(cfg, &rows);
        eprintln!(
            "{}: dbar={:.6} se={:.6} z={:.2} pass={} ({:.1}s)",
            cfg.id,
            s["kill_test"]["dbar"].as_f64().unwrap(),
            s["kill_test"]["se"].as_f64().unwrap(),
            s["kill_test"]["z"].as_f64().unwrap_or(f64::NAN),
            s["kill_test"]["pass"],
            t.elapsed().as_secs_f64()
        );
        summaries.insert(cfg.id.into(), s);
    }
    let passes: Vec<bool> = CONFIGS
        .iter()
        .filter(|c| c.primary)
        .map(|c| summaries[c.id]["kill_test"]["pass"].as_bool().unwrap())
        .collect();
    let verdict = match passes.iter().filter(|&&p| p).count() {
        2 => "survived",
        0 => "killed",
        _ => "inconclusive",
    };
    let summary = json!({
        "experiment": "spectral-planted-recovery",
        "prereg": "PREREG.md",
        "seeds": format!("1..={seeds}"),
        "primary_configs": CONFIGS.iter().filter(|c| c.primary).map(|c| c.id).collect::<Vec<_>>(),
        "verdict_rule": "survived if spectral_primary beats rarity by dbar > 2 SE (normalised rank, all half-edges) at both primary configs; killed if at neither; inconclusive if at one",
        "verdict": verdict,
        "configs": summaries,
    });
    fs::write(out.join("summary.json"), serde_json::to_string_pretty(&summary).unwrap() + "\n").unwrap();
    eprintln!("verdict: {verdict} (total {:.1}s)", t0.elapsed().as_secs_f64());
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_engine::generate_framed;
    use eternity2_engine::generator::frame_color_count;

    #[test]
    fn scramble_reproduces_generate_framed() {
        for &(size, colors) in &[(8usize, 8u8), (10, 12), (8, 14), (10, 21)] {
            for seed in 1..=20u32 {
                let bc = frame_color_count(colors);
                let base = solved(size, colors, seed, bc);
                let scr = scramble(&base, seed ^ SCRAMBLE_XOR);
                let eng = generate_framed(size as u8, colors, seed, true);
                assert_eq!(scr.pieces, eng.pieces, "size {size} colors {colors} seed {seed}");
            }
        }
    }

    #[test]
    fn planted_solution_tiles_after_scrambles() {
        for cfg in &CONFIGS {
            for seed in 1..=10u32 {
                let base = solved(cfg.size, cfg.colors, seed, cfg.border);
                let scr = scramble(&base, seed ^ SCRAMBLE_XOR);
                let re = scramble(&scr, seed ^ RESCRAMBLE_XOR);
                assert!(tiling_ok(&base) && tiling_ok(&scr) && tiling_ok(&re));
            }
        }
    }

    #[test]
    fn jacobi_reconstructs() {
        let base = solved(8, 8, 3, 2);
        let cnt = colour_counts(&base.pieces);
        let w = compat_matrix(&base.pieces, &cnt);
        let (vals, vecs) = jacobi_eigen(&w, 64);
        assert!(max_abs_residual(&w, &vals, &vecs, 64) < 1e-10);
        assert!(vals.windows(2).all(|x| x[0] >= x[1]));
        // Orthonormal columns.
        for a in 0..64 {
            for b in 0..64 {
                let d: f64 = (0..64).map(|i| vecs[i * 64 + a] * vecs[i * 64 + b]).sum();
                let e = if a == b { 1.0 } else { 0.0 };
                assert!((d - e).abs() < 1e-9);
            }
        }
    }

    #[test]
    fn mid_rank_handles_ties() {
        assert_eq!(mid_rank(&[0.5, 0.5, 0.5, 0.5], 0), (2.5, false));
        assert_eq!(mid_rank(&[0.9, 0.1, 0.2], 0), (1.0, true));
        assert_eq!(mid_rank(&[0.1, 0.9, 0.2], 0), (3.0, false));
        assert_eq!(mid_rank(&[0.2, 0.9, 0.2, 0.1], 0), (2.5, false));
        assert_eq!(mid_rank(&[0.3], 0), (1.0, true));
    }
}
