//! TOY BOARDS ONLY. bm.solve (augmented Lagrangian outer loop, L-BFGS inner) with the stopping
//! rule of step 2a: round and verify exactly, stop on the first valid board.

use crate::al::{self, Mult};
use crate::board::Board;
use crate::lbfgs;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Cfg {
    pub r: usize,
    pub rho0: f64,
    pub rho_up: f64,
    pub rho_max: f64,
    pub inner_iters: usize,
    pub max_outer: usize,
    pub time_limit: f64,
    /// hard wall-clock cap per run (the brief: no run over 15 minutes)
    pub wall_limit: f64,
    pub tol: f64,
    /// round every k outer iterations (0: only at the end, as bm.py)
    pub round_every: usize,
    /// round after every inner L-BFGS iteration too
    pub round_inner: usize,
    /// "ref", "rows", "par"
    pub kernel: String,
    pub threads: usize,
    pub chunk: usize,
    /// rho schedule: "halve" (bm.py: x rho_up when the violation did not halve)
    pub sched: String,
    /// multiply the non-first coordinates of the start by this (then renormalise)
    pub init_scale: f64,
    /// inner L-BFGS memory
    pub mem: usize,
    /// L-BFGS line search c2
    pub c2: f64,
    /// pgtol of the inner solve (bm.py 1e-9); "adaptive" inner tolerance when > 0: inner stops
    /// once max|g| <= inner_tol_fac * current violation
    pub inner_tol_fac: f64,
    /// multiplier update skipped while the violation is above this (0: always update)
    pub mult_gate: f64,
    /// stop at the first valid rounding (false: record it and run on, as bm.py, for step 0)
    pub stop_on_solution: bool,
    /// objective weight (1: bm.py; 0: feasibility only)
    pub obj_w: f64,
    /// restart from a fresh random point (Rust RNG, seeded by start and restart number) when no
    /// valid rounding after this many outer iterations (0: never)
    pub restart_outer: usize,
    /// violation and multiplier update from one dgemm Gram matrix (same values)
    pub fast_outer: bool,
    /// cap on evaluations (machine-independent budget for the sweeps; 0: none)
    pub max_evals: usize,
    /// 0 all multipliers, 1 none on nonnegativity, 2 none at all
    pub mult_mode: u8,
}

impl Default for Cfg {
    fn default() -> Self {
        Cfg {
            r: 32,
            rho0: 10.0,
            rho_up: 1.3,
            rho_max: 1e5,
            inner_iters: 300,
            max_outer: 200,
            time_limit: 480.0,
            wall_limit: 840.0,
            tol: 1e-5,
            round_every: 0,
            round_inner: 0,
            kernel: "rows".into(),
            threads: 1,
            chunk: 32,
            sched: "halve".into(),
            init_scale: 1.0,
            mem: 10,
            c2: 0.9,
            inner_tol_fac: 0.0,
            mult_gate: 0.0,
            stop_on_solution: true,
            obj_w: 1.0,
            restart_outer: 0,
            fast_outer: false,
            max_evals: 0,
            mult_mode: 0,
        }
    }
}

impl Cfg {
    pub fn set(&mut self, k: &str, v: &str) -> bool {
        match k {
            "r" => self.r = v.parse().unwrap(),
            "rho0" => self.rho0 = v.parse().unwrap(),
            "rho_up" => self.rho_up = v.parse().unwrap(),
            "rho_max" => self.rho_max = v.parse().unwrap(),
            "inner_iters" => self.inner_iters = v.parse().unwrap(),
            "max_outer" => self.max_outer = v.parse().unwrap(),
            "time_limit" => self.time_limit = v.parse().unwrap(),
            "wall_limit" => self.wall_limit = v.parse().unwrap(),
            "tol" => self.tol = v.parse().unwrap(),
            "round_every" => self.round_every = v.parse().unwrap(),
            "round_inner" => self.round_inner = v.parse().unwrap(),
            "kernel" => self.kernel = v.into(),
            "threads" => self.threads = v.parse().unwrap(),
            "chunk" => self.chunk = v.parse().unwrap(),
            "sched" => self.sched = v.into(),
            "init_scale" => self.init_scale = v.parse().unwrap(),
            "mem" => self.mem = v.parse().unwrap(),
            "c2" => self.c2 = v.parse().unwrap(),
            "inner_tol_fac" => self.inner_tol_fac = v.parse().unwrap(),
            "stop_on_solution" => self.stop_on_solution = v == "1",
            "obj_w" => self.obj_w = v.parse().unwrap(),
            "mult_mode" => self.mult_mode = v.parse().unwrap(),
            "max_evals" => self.max_evals = v.parse().unwrap(),
            "fast_outer" => self.fast_outer = v == "1",
            "restart_outer" => self.restart_outer = v.parse().unwrap(),
            "mult_gate" => self.mult_gate = v.parse().unwrap(),
            _ => return false,
        }
        true
    }
    pub fn json(&self) -> String {
        format!(
            "{{\"r\":{},\"rho0\":{},\"rho_up\":{},\"rho_max\":{},\"inner_iters\":{},\"max_outer\":{},\"time_limit\":{},\"tol\":{},\"round_every\":{},\"round_inner\":{},\"kernel\":\"{}\",\"threads\":{},\"sched\":\"{}\",\"init_scale\":{},\"mem\":{},\"c2\":{},\"inner_tol_fac\":{},\"mult_gate\":{},\"stop_on_solution\":{},\"obj_w\":{},\"restart_outer\":{},\"fast_outer\":{},\"max_evals\":{},\"mult_mode\":{}}}",
            self.r, self.rho0, self.rho_up, self.rho_max, self.inner_iters, self.max_outer, self.time_limit, self.tol,
            self.round_every, self.round_inner, self.kernel, self.threads, self.sched, self.init_scale, self.mem, self.c2,
            self.inner_tol_fac, self.mult_gate, self.stop_on_solution, self.obj_w, self.restart_outer, self.fast_outer, self.max_evals, self.mult_mode
        )
    }
}

pub struct RunOut {
    pub status: &'static str,
    pub seconds: f64,
    pub wall_seconds: f64,
    pub t_solution: Option<f64>,
    pub evals_solution: Option<usize>,
    pub outer_solution: Option<usize>,
    pub outer: usize,
    pub evals: usize,
    pub inner_iters: usize,
    pub viol: al::Viol,
    pub final_rho: f64,
    pub solved: bool,
    pub end_is_solution: bool,
    pub end_mismatches: usize,
    pub solution_in_enumeration: Option<bool>,
    pub t_eval: f64,
    pub t_round: f64,
    pub t_viol: f64,
    pub t_mult: f64,
    pub n_rounds: usize,
    pub restarts: usize,
    pub active_nonneg: usize,
    pub mass_off: Option<f64>,
    pub rank_1e2: usize,
    pub rank_1e3: usize,
    pub hist: Vec<(usize, f64, f64, f64, usize, f64)>,
}

fn wall() -> f64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64()
}

pub fn eval_v(b: &Board, m: &Mult, sp: &(al::Sparse, al::Sparse), cfg: &Cfg, v: &[f64], g: &mut [f64], buf: &mut Vec<f64>) -> f64 {
    match cfg.kernel.as_str() {
        "ref" => al::fv_ref(b, m, v, g),
        "rows" => al::fv_rows(b, m, v, g),
        "par" => al::fv_rows_par(b, m, v, g, cfg.chunk),
        "blk" => al::fv_blk(b, m, v, g),
        "blkpar" => al::fv_blk_par(b, m, v, g, cfg.chunk),
        "blas" => al::fv_blas(b, m, v, g, buf),
        "blas2" => al::fv_blas2(b, m, v, g, buf),
        "struct" => al::fv_struct(b, m, &sp.0, &sp.1, v, g, buf),
        k => panic!("kernel {k}"),
    }
}

/// SplitMix64, and normals by Box-Muller: the restart starts only (the first start of every run
/// is NumPy's, read from boards/<b>/init).
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn unif(&mut self) -> f64 {
        ((self.next() >> 11) as f64 + 0.5) / (1u64 << 53) as f64
    }
    fn normal(&mut self) -> f64 {
        (-2.0 * self.unif().ln()).sqrt() * (std::f64::consts::TAU * self.unif()).cos()
    }
}

/// bm.init_U with this crate's RNG.
pub fn init_u(b: &Board, r: usize, seed: u64) -> Vec<f64> {
    let mut rng = Rng(seed);
    let x0 = b.x0();
    let mut u = vec![0.0; b.p * r];
    for q in 0..b.p {
        let x = (x0[q] * (1.0 + 0.2 * rng.normal())).clamp(1e-3, 1.0 - 1e-3);
        u[q * r] = 2.0 * x - 1.0;
    }
    if r > 1 {
        for q in 0..b.p {
            let z: Vec<f64> = (1..r).map(|_| rng.normal()).collect();
            let n = al::dot(&z, &z).sqrt();
            let s = (1.0 - u[q * r] * u[q * r]).sqrt();
            for k in 1..r {
                u[q * r + k] = s * z[k - 1] / n;
            }
        }
    }
    u
}

pub fn solve(b: &Board, cfg: &Cfg, u0: &[f64]) -> RunOut {
    solve_seeded(b, cfg, u0, 0)
}

pub fn solve_seeded(b: &Board, cfg: &Cfg, u0: &[f64], seed: u64) -> RunOut {
    let (p, r) = (b.p, cfg.r);
    let now: fn() -> f64 = if cfg.threads <= 1 { crate::cputime::thread_cpu } else { wall };
    let wall0 = Instant::now();
    let t0 = now();
    let mut m = Mult::new(b, r, cfg.rho0);
    m.obj_w = cfg.obj_w;
    let mut restarts = 0usize;
    let mut since = 0usize;
    let mut w = u0.to_vec();
    if cfg.init_scale != 1.0 {
        for row in w.chunks_exact_mut(r) {
            for k in 1..r {
                row[k] *= cfg.init_scale;
            }
        }
    }
    let mut u = vec![0.0; p * r];
    let mut v = vec![0.0; p * r];
    let mut nrm = vec![0.0; p];
    let mut buf: Vec<f64> = Vec::new();
    let mut gram: Vec<f64> = Vec::new();
    let mut evals = 0usize;
    let mut inner_total = 0usize;
    let mut t_eval = 0.0;
    let mut t_round = 0.0;
    let mut t_viol = 0.0;
    let mut t_mult = 0.0;
    let mut n_rounds = 0usize;
    let mut order = Vec::with_capacity(p);
    let mut found: Option<(f64, usize, usize, Vec<usize>)> = None;
    let mut hist = Vec::new();
    let mut prev_v = f64::INFINITY;
    let mut prev_obj = f64::INFINITY;
    let mut status = "max_outer";
    let mut outer = 0;
    let mut viol = al::Viol::default();
    // rounding of the point w (any scale): x_p = (1 + w_p0 / ||w_p||) / 2
    let round = |w: &[f64], order: &mut Vec<usize>| -> Option<Vec<usize>> {
        let x: Vec<f64> = w.chunks_exact(r).map(|row| 0.5 * (1.0 + row[0] / al::dot(row, row).sqrt())).collect();
        let js = b.greedy_round(&x, order)?;
        if b.is_valid(&js) {
            Some(js)
        } else {
            None
        }
    };
    for it in 0..cfg.max_outer {
        outer = it + 1;
        if cfg.restart_outer > 0 && since == cfg.restart_outer {
            restarts += 1;
            since = 0;
            w = init_u(b, r, seed.wrapping_mul(1000003).wrapping_add(restarts as u64));
            m = Mult::new(b, r, cfg.rho0);
            m.obj_w = cfg.obj_w;
            prev_v = f64::INFINITY;
            prev_obj = f64::INFINITY;
        }
        since += 1;
        let opts = lbfgs::Opts {
            m: cfg.mem,
            max_iter: cfg.inner_iters,
            pgtol: if cfg.inner_tol_fac > 0.0 && prev_v.is_finite() { (cfg.inner_tol_fac * prev_v).max(1e-9) } else { 1e-9 },
            c2: cfg.c2,
            ..Default::default()
        };
        let mut inner_it = 0usize;
        let res = {
            let m_ref = &m;
            let sp = if cfg.kernel == "struct" { al::build_sparse(b, &m) } else { (al::Sparse { rp: vec![], cj: vec![], val: vec![] }, al::Sparse { rp: vec![], cj: vec![], val: vec![] }) };
            let evals_ref = &mut evals;
            let te = &mut t_eval;
            let fg = |wx: &[f64], g: &mut [f64]| -> f64 {
                let ts = now();
                al::sphere(wx, r, &mut u, &mut v, &mut nrm);
                let f = eval_v(b, m_ref, &sp, cfg, &v, g, &mut buf);
                al::pullback(&u, &nrm, r, g);
                *evals_ref += 1;
                *te += (now() - ts);
                f
            };
            let tr = &mut t_round;
            let nr = &mut n_rounds;
            let fd = &mut found;
            let order_ref = &mut order;
            let cb = |wx: &[f64], _f: f64| -> bool {
                inner_it += 1;
                if cfg.round_inner > 0 && inner_it % cfg.round_inner == 0 && fd.is_none() {
                    let ts = now();
                    *nr += 1;
                    let res = round(wx, order_ref);
                    *tr += (now() - ts);
                    if let Some(js) = res {
                        *fd = Some(((now() - t0), usize::MAX, it + 1, js));
                        return cfg.stop_on_solution;
                    }
                }
                false
            };
            lbfgs::minimize(&mut w, fg, &opts, cb)
        };
        inner_total += res.iters;
        if let Some((t, e, o, js)) = found.take() {
            found = Some((t, if e == usize::MAX { evals } else { e }, o, js));
            if cfg.stop_on_solution {
                status = "solved";
                break;
            }
        }
        // renormalise
        for row in w.chunks_exact_mut(r) {
            let n = al::dot(row, row).sqrt();
            row.iter_mut().for_each(|a| *a /= n);
        }
        al::sphere(&w, r, &mut u, &mut v, &mut nrm);
        let ts = now();
        if cfg.fast_outer {
            gram.resize(p * p, 0.0);
            crate::blas::dgemm(false, true, p, p, r, 1.0, &v, &v, 0.0, &mut gram);
            viol = al::max_violation_gram(b, &v, r, &gram);
        } else {
            viol = al::max_violation(b, &v, r);
        }
        t_viol += (now() - ts);
        let obj: f64 = (0..p).map(|q| v[q * r] * v[q * r]).sum();
        hist.push((it, m.rho, viol.max, obj, res.iters, (now() - t0)));
        if cfg.round_every > 0 && (it + 1) % cfg.round_every == 0 && found.is_none() {
            let ts = now();
            n_rounds += 1;
            let res = round(&w, &mut order);
            t_round += (now() - ts);
            if let Some(js) = res {
                found = Some(((now() - t0), evals, it + 1, js));
                if cfg.stop_on_solution {
                    status = "solved";
                    break;
                }
            }
        }
        if viol.max <= cfg.tol && (obj - prev_obj).abs() <= 1e-6 * obj.max(1.0) {
            status = "converged";
            break;
        }
        prev_obj = obj;
        let ts = now();
        if !(cfg.mult_gate > 0.0 && viol.max > cfg.mult_gate) {
            if cfg.fast_outer {
                al::update_multipliers_gram_mode(b, &mut m, &v, &gram, cfg.mult_mode);
            } else {
                assert_eq!(cfg.mult_mode, 0, "mult_mode needs fast_outer=1");
                al::update_multipliers(b, &mut m, &v);
            }
        }
        t_mult += (now() - ts);
        match cfg.sched.as_str() {
            "halve" => {
                if viol.max > 0.5 * prev_v {
                    m.rho = (m.rho * cfg.rho_up).min(cfg.rho_max);
                }
            }
            "always" => m.rho = (m.rho * cfg.rho_up).min(cfg.rho_max),
            "quarter" => {
                if viol.max > 0.25 * prev_v {
                    m.rho = (m.rho * cfg.rho_up).min(cfg.rho_max);
                }
            }
            s => panic!("sched {s}"),
        }
        prev_v = viol.max;
        if cfg.max_evals > 0 && evals >= cfg.max_evals {
            status = "max_evals";
            break;
        }
        if (now() - t0) > cfg.time_limit || wall0.elapsed().as_secs_f64() > cfg.wall_limit {
            status = "time_limit";
            break;
        }
    }
    let seconds = now() - t0;
    let wall_seconds = wall0.elapsed().as_secs_f64();
    // final rounding of the end point (as run_unit.py does)
    al::sphere(&w, r, &mut u, &mut v, &mut nrm);
    let x: Vec<f64> = (0..p).map(|q| v[q * r]).collect();
    let js_end = b.greedy_round(&x, &mut order);
    let (end_ok, end_mm) = match &js_end {
        Some(js) => (b.is_valid(js), b.mismatches(js)),
        None => (false, usize::MAX),
    };
    if viol.max == 0.0 {
        viol = al::max_violation(b, &v, r);
    }
    // mass off every enumerated solution, per cell (run_unit.py's measure), and numerical rank of
    // Z = [e1; V][e1; V]^T from the eigenvalues of the r x r matrix e1 e1^T + V^T V
    let mass_off = if b.sols.is_empty() {
        None
    } else {
        let mut on = vec![false; p];
        for sl in &b.sols {
            for &q in sl {
                on[q] = true;
            }
        }
        Some((0..p).filter(|&q| !on[q]).map(|q| x[q].max(0.0)).sum::<f64>() / b.nc as f64)
    };
    let mut gm = vec![0.0; r * r];
    for q in 0..p {
        for i in 0..r {
            for j in 0..r {
                gm[i * r + j] += v[q * r + i] * v[q * r + j];
            }
        }
    }
    gm[0] += 1.0;
    let mut ev = jacobi_eigs(gm, r);
    ev.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let rank_1e2 = ev.iter().filter(|&&e| e > 1e-2 * ev[0]).count();
    let rank_1e3 = ev.iter().filter(|&&e| e > 1e-3 * ev[0]).count();
    let sols: HashMap<&Vec<usize>, ()> = b.sols.iter().map(|s| (s, ())).collect();
    let in_enum = found.as_ref().map(|f| sols.contains_key(&f.3)).or_else(|| if end_ok { js_end.as_ref().map(|j| sols.contains_key(j)) } else { None });
    RunOut {
        status,
        seconds,
        wall_seconds,
        t_solution: found.as_ref().map(|f| f.0).or(if end_ok { Some(seconds) } else { None }),
        evals_solution: found.as_ref().map(|f| f.1).or(if end_ok { Some(evals) } else { None }),
        outer_solution: found.as_ref().map(|f| f.2).or(if end_ok { Some(outer) } else { None }),
        outer,
        evals,
        inner_iters: inner_total,
        viol,
        final_rho: m.rho,
        solved: end_ok || found.is_some(),
        end_is_solution: end_ok || (found.is_some() && cfg.stop_on_solution),
        end_mismatches: if found.is_some() && cfg.stop_on_solution { 0 } else { end_mm },
        solution_in_enumeration: in_enum,
        t_eval,
        t_round,
        t_viol,
        t_mult,
        n_rounds,
        restarts,
        active_nonneg: al::active_nonneg(b, &m),
        mass_off,
        rank_1e2,
        rank_1e3,
        hist,
    }
}

/// Eigenvalues of a small symmetric matrix by cyclic Jacobi rotations.
fn jacobi_eigs(mut a: Vec<f64>, n: usize) -> Vec<f64> {
    for _ in 0..100 {
        let mut off = 0.0;
        for i in 0..n {
            for j in i + 1..n {
                off += a[i * n + j] * a[i * n + j];
            }
        }
        if off < 1e-22 {
            break;
        }
        for pi in 0..n {
            for qi in pi + 1..n {
                let apq = a[pi * n + qi];
                if apq.abs() < 1e-300 {
                    continue;
                }
                let theta = (a[qi * n + qi] - a[pi * n + pi]) / (2.0 * apq);
                let t = theta.signum() / (theta.abs() + (theta * theta + 1.0).sqrt());
                let t = if theta == 0.0 { 1.0 } else { t };
                let c = 1.0 / (t * t + 1.0).sqrt();
                let sn = t * c;
                for k in 0..n {
                    let akp = a[k * n + pi];
                    let akq = a[k * n + qi];
                    a[k * n + pi] = c * akp - sn * akq;
                    a[k * n + qi] = sn * akp + c * akq;
                }
                for k in 0..n {
                    let apk = a[pi * n + k];
                    let aqk = a[qi * n + k];
                    a[pi * n + k] = c * apk - sn * aqk;
                    a[qi * n + k] = sn * apk + c * aqk;
                }
            }
        }
    }
    (0..n).map(|i| a[i * n + i]).collect()
}
