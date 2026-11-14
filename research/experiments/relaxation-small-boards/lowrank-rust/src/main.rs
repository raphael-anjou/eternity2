//! TOY BOARDS ONLY (generated 5x5 and 6x6 boards; nothing here is Eternity II progress).
//! Rust port of ../lowrank-relaxation/bm.py.
//!
//!   lowrank-rust equiv <board>...                      step 1: objective and gradient vs NumPy
//!   lowrank-rust run boards=a,b starts=0-9 out=f.jsonl tag=t jobs=3 [cfg key=value ...]
//!   lowrank-rust bench board=b r=32 kernel=rows reps=200 [threads=3]

mod al;
mod blas;
mod cputime;
mod board;
mod lbfgs;
mod solver;

use board::{read_f64, Board};
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn kv(args: &[String]) -> HashMap<String, String> {
    args.iter()
        .map(|a| {
            let (k, v) = a.split_once('=').unwrap_or_else(|| panic!("expected key=value: {a}"));
            (k.to_string(), v.to_string())
        })
        .collect()
}

fn relerr(a: &[f64], b: &[f64]) -> (f64, f64) {
    // (max |a - b| / max |b|, max over entries with |b| >= 1e-6 max|b| of |a - b| / |b|)
    let mb = b.iter().fold(0.0f64, |s, &x| s.max(x.abs()));
    let mut nw = 0.0f64;
    let mut ew = 0.0f64;
    for (x, y) in a.iter().zip(b) {
        nw = nw.max((x - y).abs());
        if y.abs() >= 1e-6 * mb {
            ew = ew.max((x - y).abs() / y.abs());
        }
    }
    (nw / mb, ew)
}

fn equiv(names: &[String]) {
    let mut worst = 0.0f64;
    println!("board k r rho | f_ref f_rows | gV_ref gV_rows (normwise) | gW_ref gW_rows (normwise) | gW_rows entrywise");
    for name in names {
        let b = Board::load(&root().join("boards").join(name));
        let ed = b.dir.join("equiv");
        for k in 0..5 {
            let head = std::fs::read_to_string(ed.join(format!("{k}.txt"))).unwrap();
            let h: Vec<&str> = head.split_whitespace().collect();
            let r: usize = h[0].parse().unwrap();
            let rho: f64 = h[1].parse().unwrap();
            let f_np: f64 = h[3].parse().unwrap();
            let rd = |s: &str| read_f64(&ed.join(format!("{k}_{s}.bin")));
            let w = rd("W");
            let mut m = al::Mult::new(&b, r, rho);
            m.lc = rd("lc");
            m.lt = rd("lt");
            let lz = rd("lz");
            for (q, &(i, j)) in b.conf.iter().enumerate() {
                m.pl[i * b.p + j] = lz[q];
                m.pl[j * b.p + i] = lz[q];
            }
            for t in rd("mu").chunks_exact(3) {
                let (i, j) = (t[0] as usize, t[1] as usize);
                assert_eq!(b.kind[i * b.p + j], board::FREE);
                m.pl[i * b.p + j] = t[2];
                m.pl[j * b.p + i] = t[2];
            }
            let gv_np = rd("gV");
            let gw_np = rd("gW");
            let (p, n) = (b.p, b.p * r);
            let mut u = vec![0.0; n];
            let mut v = vec![0.0; n];
            let mut nrm = vec![0.0; p];
            al::sphere(&w, r, &mut u, &mut v, &mut nrm);
            let mut out = Vec::new();
            let mut buf = Vec::new();
            let (cs, act) = al::build_sparse(&b, &m);
            for kern in ["ref", "rows", "par", "blk", "blkpar", "blas", "blas2", "struct"] {
                let mut g = vec![0.0; n];
                let f = match kern {
                    "ref" => al::fv_ref(&b, &m, &v, &mut g),
                    "rows" => al::fv_rows(&b, &m, &v, &mut g),
                    "par" => al::fv_rows_par(&b, &m, &v, &mut g, 32),
                    "blk" => al::fv_blk(&b, &m, &v, &mut g),
                    "blkpar" => al::fv_blk_par(&b, &m, &v, &mut g, 32),
                    "blas" => al::fv_blas(&b, &m, &v, &mut g, &mut buf),
                    "blas2" => al::fv_blas2(&b, &m, &v, &mut g, &mut buf),
                    _ => al::fv_struct(&b, &m, &cs, &act, &v, &mut g, &mut buf),
                };
                let ev = relerr(&g, &gv_np);
                al::pullback(&u, &nrm, r, &mut g);
                let ew = relerr(&g, &gw_np);
                let fe = (f - f_np).abs() / f_np.abs();
                worst = worst.max(fe).max(ev.0).max(ew.0);
                out.push((fe, ev, ew));
            }
            println!(
                "{name} {k} {r} {rho:.4e} | {:.1e} {:.1e} | {:.1e} {:.1e} | {:.1e} {:.1e} | {:.1e} (par: f {:.1e} gW {:.1e}; blk: f {:.1e} gW {:.1e}; blkpar: f {:.1e} gW {:.1e}; blas: f {:.1e} gW {:.1e}; blas2: f {:.1e} gW {:.1e}; struct: f {:.1e} gW {:.1e})",
                out[0].0, out[1].0, out[0].1 .0, out[1].1 .0, out[0].2 .0, out[1].2 .0, out[1].2 .1, out[2].0, out[2].2 .0, out[3].0, out[3].2 .0, out[4].0, out[4].2 .0, out[5].0, out[5].2 .0, out[6].0, out[6].2 .0, out[7].0, out[7].2 .0
            );
        }
    }
    println!("worst relative error over all boards, states and kernels: {worst:.2e} (target 1e-9): {}", if worst <= 1e-9 { "PASS" } else { "FAIL" });
}

fn parse_range(s: &str) -> Vec<usize> {
    let mut v = Vec::new();
    for part in s.split(',') {
        if let Some((a, b)) = part.split_once('-') {
            v.extend(a.parse::<usize>().unwrap()..=b.parse::<usize>().unwrap());
        } else {
            v.push(part.parse().unwrap());
        }
    }
    v
}

fn run(args: &[String]) {
    let a = kv(args);
    let mut cfg = solver::Cfg::default();
    for (k, v) in &a {
        if !matches!(k.as_str(), "boards" | "starts" | "out" | "tag" | "jobs" | "hist") && !cfg.set(k, v) {
            panic!("unknown key {k}");
        }
    }
    let boards: Vec<Board> = a["boards"].split(',').map(|n| Board::load(&root().join("boards").join(n))).collect();
    let starts = parse_range(&a["starts"]);
    let jobs: usize = a.get("jobs").map(|s| s.parse().unwrap()).unwrap_or(3);
    let tag = a.get("tag").cloned().unwrap_or_default();
    let with_hist = a.get("hist").map(|s| s == "1").unwrap_or(false);
    let out = Mutex::new(std::fs::OpenOptions::new().create(true).append(true).open(root().join(&a["out"])).unwrap());
    let list: Vec<(usize, usize)> = (0..boards.len()).flat_map(|bi| starts.iter().map(move |&s| (bi, s))).collect();
    let next = AtomicUsize::new(0);
    let pool = rayon::ThreadPoolBuilder::new().num_threads(cfg.threads.max(1)).build().unwrap();
    std::thread::scope(|sc| {
        for _ in 0..jobs {
            sc.spawn(|| loop {
                let q = next.fetch_add(1, Ordering::SeqCst);
                if q >= list.len() {
                    break;
                }
                let (bi, s) = list[q];
                let b = &boards[bi];
                let u0 = read_f64(&b.dir.join("init").join(format!("r{}_s{}.bin", cfg.r, s)));
                assert_eq!(u0.len(), b.p * cfg.r);
                // restart seed from the board name (FNV-1a), the rank and the start: independent of list order
                let seed = b.name.bytes().fold(0xcbf29ce484222325u64, |h, c| (h ^ c as u64).wrapping_mul(0x100000001b3))
                    ^ ((cfg.r as u64) << 32 | s as u64);
                // threads == 1: run on this job thread (a shared one-thread pool would serialise the jobs)
                let o = if cfg.threads <= 1 { solver::solve_seeded(b, &cfg, &u0, seed) } else { pool.install(|| solver::solve_seeded(b, &cfg, &u0, seed)) };
                let v = &o.viol;
                let hist = if with_hist {
                    let h: Vec<String> = o.hist.iter().map(|h| format!("[{},{:.4e},{:.4e},{:.5},{},{:.3}]", h.0, h.1, h.2, h.3, h.4, h.5)).collect();
                    format!(",\"hist\":[{}]", h.join(","))
                } else {
                    String::new()
                };
                let opt = |x: Option<f64>| x.map(|t| format!("{t:.4}")).unwrap_or("null".into());
                let opu = |x: Option<usize>| x.map(|t| t.to_string()).unwrap_or("null".into());
                let line = format!(
                    "{{\"tag\":\"{tag}\",\"board\":\"{}\",\"start\":{s},\"P\":{},\"nsols\":{},\"status\":\"{}\",\"solved\":{},\"end_is_solution\":{},\"seconds\":{:.4},\"wall_seconds\":{:.2},\"clock\":\"{}\",\"t_solution\":{},\"evals_solution\":{},\"outer_solution\":{},\"outer\":{},\"evals\":{},\"inner_iters\":{},\"final_rho\":{:.4e},\"viol_max\":{:.4e},\"viol\":{{\"cell_sum\":{:.3e},\"tile_sum\":{:.3e},\"diag\":{:.3e},\"row_cell\":{:.3e},\"row_tile\":{:.3e},\"zero\":{:.3e},\"x_nonneg\":{:.3e},\"y_nonneg\":{:.3e}}},\"end_mismatches\":{},\"solution_in_enumeration\":{},\"t_eval\":{:.4},\"t_round\":{:.4},\"t_viol\":{:.4},\"t_mult\":{:.4},\"n_rounds\":{},\"restarts\":{},\"active_nonneg\":{},\"free_pairs\":{},\"mass_off_per_cell\":{},\"rank_1e2\":{},\"rank_1e3\":{},\"cfg\":{}{hist}}}\n",
                    b.name, b.p, b.sols.len(), o.status, o.solved, o.end_is_solution, o.seconds, o.wall_seconds, if cfg.threads <= 1 { "thread_cpu" } else { "wall" }, opt(o.t_solution), opu(o.evals_solution), opu(o.outer_solution),
                    o.outer, o.evals, o.inner_iters, o.final_rho, v.max, v.cell_sum, v.tile_sum, v.diag, v.row_cell, v.row_tile, v.zero, v.x_nonneg, v.y_nonneg,
                    if o.end_mismatches == usize::MAX { "null".to_string() } else { o.end_mismatches.to_string() },
                    o.solution_in_enumeration.map(|x| x.to_string()).unwrap_or("null".into()),
                    o.t_eval, o.t_round, o.t_viol, o.t_mult, o.n_rounds, o.restarts, o.active_nonneg, b.nfree, o.mass_off.map(|x| format!("{x:.5}")).unwrap_or("null".into()), o.rank_1e2, o.rank_1e3, cfg.json()
                );
                eprintln!("{tag} {} s{s} {} solved={} t_sol={} evals={} outer={} viol={:.2e} {:.1}s", b.name, o.status, o.solved, opt(o.t_solution), o.evals, o.outer, v.max, o.seconds);
                let mut f = out.lock().unwrap();
                f.write_all(line.as_bytes()).unwrap();
                f.flush().unwrap();
            });
        }
    });
}

fn bench(args: &[String]) {
    let a = kv(args);
    let b = Board::load(&root().join("boards").join(&a["board"]));
    let r: usize = a["r"].parse().unwrap();
    let reps: usize = a.get("reps").map(|s| s.parse().unwrap()).unwrap_or(100);
    let threads: usize = a.get("threads").map(|s| s.parse().unwrap()).unwrap_or(1);
    let chunk: usize = a.get("chunk").map(|s| s.parse().unwrap()).unwrap_or(32);
    let kern = a.get("kernel").cloned().unwrap_or("rows".into());
    let u0 = read_f64(&b.dir.join("init").join(format!("r{r}_s0.bin")));
    let mut m = al::Mult::new(&b, r, 100.0);
    // a realistic multiplier state: a few updates from the start point
    let (p, n) = (b.p, b.p * r);
    let mut u = vec![0.0; n];
    let mut v = vec![0.0; n];
    let mut nrm = vec![0.0; p];
    al::sphere(&u0, r, &mut u, &mut v, &mut nrm);
    al::update_multipliers(&b, &mut m, &v);
    let mut g = vec![0.0; n];
    let (cs, act) = al::build_sparse(&b, &m);
    let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
    let mut best = f64::INFINITY;
    let mut bestc = f64::INFINITY;
    let mut buf = Vec::new();
    let mut fsum = 0.0;
    pool.install(|| {
        for _ in 0..3 {
            let t = Instant::now();
            let c0 = cputime::thread_cpu();
            for _ in 0..reps {
                fsum += match kern.as_str() {
                    "ref" => al::fv_ref(&b, &m, &v, &mut g),
                    "rows" => al::fv_rows(&b, &m, &v, &mut g),
                    "par" => al::fv_rows_par(&b, &m, &v, &mut g, chunk),
                    "blk" => al::fv_blk(&b, &m, &v, &mut g),
                    "blkpar" => al::fv_blk_par(&b, &m, &v, &mut g, chunk),
                    "blas" => al::fv_blas(&b, &m, &v, &mut g, &mut buf),
                    "blas2" => al::fv_blas2(&b, &m, &v, &mut g, &mut buf),
                    "struct" => al::fv_struct(&b, &m, &cs, &act, &v, &mut g, &mut buf),
                    k => panic!("{k}"),
                };
            }
            best = best.min(t.elapsed().as_secs_f64() / reps as f64);
            bestc = bestc.min((cputime::thread_cpu() - c0) / reps as f64);
        }
    });
    let t = Instant::now();
    let vv = al::max_violation(&b, &v, r);
    let tv = t.elapsed().as_secs_f64();
    let t = Instant::now();
    let mut gram = vec![0.0; p * p];
    blas::dgemm(false, true, p, p, r, 1.0, &v, &v, 0.0, &mut gram);
    let vg = al::max_violation_gram(&b, &v, r, &gram);
    let mut m2 = m.clone();
    al::update_multipliers_gram(&b, &mut m2, &v, &gram);
    let tg = t.elapsed().as_secs_f64();
    let mut m3 = m.clone();
    al::update_multipliers(&b, &mut m3, &v);
    let dm = m2.pl.iter().zip(&m3.pl).fold(0.0f64, |a, (x, y)| a.max((x - y).abs()));
    eprintln!("gram path: viol {:.6e} vs {:.6e} (row_cell {:.3e} vs {:.3e}), max multiplier diff {:.2e}, {:.3} ms for both", vg.max, vv.max, vg.row_cell, vv.row_cell, dm, tg * 1e3);
    let t = Instant::now();
    al::update_multipliers(&b, &mut m, &v);
    let tm = t.elapsed().as_secs_f64();
    println!(
        "{{\"board\":\"{}\",\"P\":{p},\"r\":{r},\"kernel\":\"{kern}\",\"threads\":{threads},\"chunk\":{chunk},\"ms_per_eval_wall\":{:.4},\"ms_per_eval_cpu\":{:.4},\"ms_max_violation\":{:.4},\"ms_update_multipliers\":{:.4},\"check\":{:.6e},\"v\":{:.3e}}}",
        b.name, best * 1e3, bestc * 1e3, tv * 1e3, tm * 1e3, fsum, vv.max
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("equiv") => equiv(&args[2..]),
        Some("run") => run(&args[2..]),
        Some("bench") => bench(&args[2..]),
        _ => eprintln!("usage: equiv | run | bench"),
    }
}
