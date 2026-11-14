//! psx: prefix statistics (see ../PREREG.md). Subcommands:
//!   smc      Track 10's SMC on a CSV puzzle (smoke check of the import)
//!   measure  one planted 16x16 board: true prefix, decoy and sampled prefixes
//!            at each target depth and scan direction, one JSON line each
//!   real     the official pieces: sampled prefixes only (null in reverse)
//!   gate     small planted boards: SMC means and sds of every statistic
//!            against exact enumeration of all nodes at the target depths

mod puzzle;
mod stats;
mod tree;

use puzzle::{load_csv, rotate, Edges, Puzzle, Rng};
use stats::{Ctx, NAMES, NSTAT};
use std::collections::HashMap;
use std::time::Instant;
use tree::{rows_bottom_up, scan, smc, smc_with, State, Tree};

/// Official clue cells, pos = row * 16 + col (CSV hints).
pub const CLUES16: [usize; 5] = [135, 210, 34, 221, 45];

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

fn get<T: std::str::FromStr>(m: &HashMap<String, String>, k: &str, d: T) -> T {
    m.get(k).and_then(|v| v.parse().ok()).unwrap_or(d)
}

fn list(m: &HashMap<String, String>, k: &str, d: &str) -> Vec<usize> {
    m.get(k)
        .map(String::as_str)
        .unwrap_or(d)
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

/// Planted board from the repo's framed generator, tile ids and rotations
/// scrambled with a seeded permutation. Returns the puzzle and the planted
/// placement (pos, pid, rot) for every cell.
fn planted(n: usize, colors: u8, border: u8, seed: u32) -> (Puzzle, Vec<(usize, u16, u8)>) {
    let g = eternity2_engine::generator::generate_solved_framed_bc(n as u8, colors, seed, true, border);
    let cells: Vec<Edges> = g.pieces.iter().map(|p| [p[0], p[1], p[2], p[3]]).collect();
    assert_eq!(cells.len(), n * n);
    let mut rng = Rng::new(u64::from(seed) ^ 0xD4D4_0000_0000_0000);
    let mut perm: Vec<usize> = (0..n * n).collect();
    rng.shuffle(&mut perm);
    let mut pieces = vec![[0u8; 4]; n * n];
    let mut plant = Vec::with_capacity(n * n);
    for (pos, &pid) in perm.iter().enumerate() {
        let k = rng.below(4) as u8;
        pieces[pid] = rotate(cells[pos], k);
        plant.push((pos, pid as u16, (4 - k) % 4));
    }
    for &(pos, pid, r) in &plant {
        assert_eq!(rotate(pieces[pid as usize], r), cells[pos]);
    }
    (
        Puzzle {
            n,
            pieces,
            hints: Vec::new(),
        },
        plant,
    )
}

/// State reached by applying `placement` (pos -> (pid, rot)) along the scan
/// of `t` for k steps. Panics if a step is not a valid child: this is the
/// pipeline check that the true prefix is a node of the tree.
fn replay(t: &Tree, p: &Puzzle, placement: &HashMap<usize, (u16, Edges)>, k: usize) -> State {
    let mut st = t.root();
    for j in 0..k {
        let pos = t.pos(j);
        let &(pid, e) = placement.get(&pos).expect("cell in placement");
        let i = t.find(pid, e).expect("orientation exists");
        let mut ok = false;
        t.for_each_child(&st, j, |c| ok |= c == i);
        assert!(ok, "replayed placement is not a child at step {j}");
        t.apply(&mut st, j, i);
        let _ = p;
    }
    st
}

fn summary(vals: &[[f64; NSTAT]]) -> (Vec<f64>, Vec<f64>) {
    let m = vals.len() as f64;
    let mut mean = vec![0f64; NSTAT];
    let mut sd = vec![0f64; NSTAT];
    for v in vals {
        for s in 0..NSTAT {
            mean[s] += v[s] / m;
        }
    }
    for v in vals {
        for s in 0..NSTAT {
            sd[s] += (v[s] - mean[s]).powi(2) / m;
        }
    }
    for s in 0..NSTAT {
        sd[s] = sd[s].sqrt();
    }
    (mean, sd)
}

fn rank(vals: &[[f64; NSTAT]], x: &[f64; NSTAT]) -> (Vec<f64>, Vec<f64>) {
    let m = vals.len() as f64;
    let mut below = vec![0f64; NSTAT];
    let mut equal = vec![0f64; NSTAT];
    for v in vals {
        for s in 0..NSTAT {
            if v[s] < x[s] {
                below[s] += 1.0 / m;
            } else if v[s] == x[s] {
                equal[s] += 1.0 / m;
            }
        }
    }
    (below, equal)
}

fn jf(v: &[f64]) -> String {
    let s: Vec<String> = v
        .iter()
        .map(|x| {
            if x.is_finite() {
                format!("{:.6e}", x)
            } else {
                "null".into()
            }
        })
        .collect();
    format!("[{}]", s.join(","))
}

/// Captured population at one target depth: population size, the uniform
/// subsample, a decoy from the rest, and each subsample particle's work
/// weight (estimated number of nodes in its subtree, from its SMC
/// descendants at every later depth, up to a common factor).
/// A4 additions: the family diagnostic of the whole population (distinct
/// ancestors at depth k - 32 and 1 / sum of squared family shares) and, at
/// the survivor depths, the population members with a descendant one row
/// ahead (and two rows ahead).
pub struct Cap {
    pub popn: usize,
    pub sample: Vec<State>,
    pub decoy: Option<State>,
    pub w: Vec<f64>,
    payload: Vec<i32>,
    pub fam_distinct: usize,
    pub fam_ess: f64,
    pub surv: Option<Surv>,
}

pub struct Surv {
    pub scanned: usize,
    pub states: Vec<State>,
    pub r2_tested: usize,
    pub r2_capped: usize,
    pub r2_states: Vec<State>,
}

pub const SURV_DEPTHS: [usize; 3] = [128, 144, 160];

fn survivors(t: &Tree, pop: &[(State, [u32; 4])], k: usize, seed: u64, want: usize) -> Surv {
    let mut rng = Rng::new(seed ^ ((k as u64) << 32) ^ 0x5u64.wrapping_mul(0x9E37_79B9));
    let mut idx: Vec<usize> = (0..pop.len()).collect();
    let mut states = Vec::new();
    let mut scanned = 0;
    for i in 0..idx.len() {
        let j = i + rng.below(idx.len() - i);
        idx.swap(i, j);
        let mut st = pop[idx[i]].0.clone();
        scanned += 1;
        if t.exists_descendant(&mut st, k, 16, 100_000) == Some(true) {
            states.push(st);
            if states.len() >= want {
                break;
            }
        }
    }
    let (mut r2_tested, mut r2_capped) = (0, 0);
    let mut r2_states = Vec::new();
    for st in &states {
        let mut s2 = st.clone();
        r2_tested += 1;
        match t.exists_descendant(&mut s2, k, 32, 1_000_000) {
            Some(true) => r2_states.push(st.clone()),
            None => r2_capped += 1,
            Some(false) => {}
        }
    }
    Surv { scanned, states, r2_tested, r2_capped, r2_states }
}

/// One SMC repetition on tree `t`; at each target depth keep `m` uniformly
/// chosen particles and one decoy chosen from the rest.
fn capture(
    t: &Tree,
    np: usize,
    seed: u64,
    depths: &[usize],
    m: usize,
    surv_depths: &[usize],
) -> (HashMap<usize, Cap>, tree::SmcRun) {
    let mut out: HashMap<usize, Cap> = HashMap::new();
    // family tags started at depth k - 32 for each target depth k
    let mut fam: HashMap<usize, Vec<u32>> = HashMap::new();
    let run = smc_with(t, np, seed, |k, pop, parent, est_k| {
        if pop.is_empty() {
            return;
        }
        let unit = est_k / pop.len() as f64;
        // carry payloads of earlier captures down one level, add work
        for c in out.values_mut() {
            let p: Vec<i32> = parent.iter().map(|&j| c.payload[j as usize]).collect();
            for &id in &p {
                if id >= 0 {
                    c.w[id as usize] += unit;
                }
            }
            c.payload = p;
        }
        for tags in fam.values_mut() {
            let p: Vec<u32> = parent.iter().map(|&j| tags[j as usize]).collect();
            *tags = p;
        }
        if depths.contains(&(k + 32)) {
            fam.insert(k + 32, (0..pop.len() as u32).collect());
        }
        if !depths.contains(&k) {
            return;
        }
        let mut rng = Rng::new(seed ^ ((k as u64) << 40) ^ 0xC0FFEE);
        let mut idx: Vec<usize> = (0..pop.len()).collect();
        // partial Fisher-Yates: first m+1 entries uniform without replacement
        let take = (m + 1).min(pop.len());
        for i in 0..take {
            let j = i + rng.below(pop.len() - i);
            idx.swap(i, j);
        }
        let (sample, decoy) = if pop.len() > m {
            (&idx[..m], Some(idx[m]))
        } else if pop.len() >= 2 {
            (&idx[..pop.len() - 1], Some(idx[pop.len() - 1]))
        } else {
            (&idx[..pop.len()], None)
        };
        let mut payload = vec![-1i32; pop.len()];
        for (id, &i) in sample.iter().enumerate() {
            payload[i] = id as i32;
        }
        let (mut fam_distinct, mut fam_ess) = (0usize, f64::NAN);
        if let Some(tags) = fam.remove(&k) {
            let mut tg = tags.clone();
            tg.sort_unstable();
            let tot = tg.len() as f64;
            let (mut ss, mut run_) = (0f64, 0usize);
            for i in 0..tg.len() {
                run_ += 1;
                if i + 1 == tg.len() || tg[i + 1] != tg[i] {
                    fam_distinct += 1;
                    ss += (run_ as f64 / tot).powi(2);
                    run_ = 0;
                }
            }
            fam_ess = 1.0 / ss;
        }
        let surv = if surv_depths.contains(&k) { Some(survivors(t, pop, k, seed, 1000)) } else { None };
        let c = Cap {
            popn: pop.len(),
            sample: sample.iter().map(|&i| pop[i].0.clone()).collect(),
            decoy: decoy.map(|i| pop[i].0.clone()),
            w: vec![unit; sample.len()],
            payload,
            fam_distinct,
            fam_ess,
            surv,
        };
        out.insert(k, c);
    });
    for c in out.values_mut() {
        c.payload = Vec::new();
    }
    (out, run)
}

/// Weighted mean, sd, fraction strictly below x, fraction equal to x, and
/// the effective sample size of the weights.
fn wsummary(vals: &[[f64; NSTAT]], w: &[f64], x: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, f64) {
    let sw: f64 = w.iter().sum();
    let sw2: f64 = w.iter().map(|a| a * a).sum();
    let mut mean = vec![0f64; NSTAT];
    let mut sd = vec![0f64; NSTAT];
    let mut below = vec![0f64; NSTAT];
    let mut equal = vec![0f64; NSTAT];
    for (v, &wi) in vals.iter().zip(w) {
        for s in 0..NSTAT {
            mean[s] += wi * v[s] / sw;
            if v[s] < x[s] {
                below[s] += wi / sw;
            } else if v[s] == x[s] {
                equal[s] += wi / sw;
            }
        }
    }
    for (v, &wi) in vals.iter().zip(w) {
        for s in 0..NSTAT {
            sd[s] += wi * (v[s] - mean[s]).powi(2) / sw;
        }
    }
    for s in 0..NSTAT {
        sd[s] = sd[s].sqrt();
    }
    (mean, sd, below, equal, sw * sw / sw2)
}

fn placement_of(st: &State, p: &Puzzle) -> HashMap<usize, (u16, Edges)> {
    let mut h = HashMap::new();
    for pos in 0..p.n * p.n {
        let t = st.cell[pos];
        if t != u16::MAX {
            let e = [
                st.board[pos * 4],
                st.board[pos * 4 + 1],
                st.board[pos * 4 + 2],
                st.board[pos * 4 + 3],
            ];
            h.insert(pos, (t, e));
        }
    }
    h
}

fn cmd_measure(m: &HashMap<String, String>) {
    let n: usize = get(m, "n", 16);
    let colors: u8 = get(m, "colors", 22);
    let border: u8 = get(m, "border", 5);
    // A5: a board seed that does not parse is an error, not board 1
    let seed: u32 = m.get("board").map(|v| v.parse().expect("--board must be an integer")).unwrap_or(1);
    let np: usize = get(m, "np", 100_000);
    let ms: usize = get(m, "m", 1000);
    let role = m.get("role").cloned().unwrap_or_else(|| "calib".into());
    let dirs = list(m, "dirs", "0,1,2,3");
    let depths = list(m, "depths", "32,64,96,128,144,160,176,192,208,224");
    let smc_seed0: u64 = get(m, "smc-seed", 1);
    let t_all = Instant::now();
    let (p, plant) = planted(n, colors, border, seed);
    let clue_pos: Vec<usize> = if n == 16 { CLUES16.to_vec() } else { vec![] };
    let fixed: Vec<(usize, u16, u8)> = plant.iter().copied().filter(|x| clue_pos.contains(&x.0)).collect();
    let mut full: HashMap<usize, (u16, Edges)> = HashMap::new();
    for &(pos, pid, r) in &plant {
        full.insert(pos, (pid, rotate(p.pieces[pid as usize], r)));
    }
    let t0 = Instant::now();
    let ctx = Ctx::new(n, &p.pieces, &clue_pos);
    let ctx_s = t0.elapsed().as_secs_f64();
    for &dir in &dirs {
        let order: Vec<usize> = scan(n, dir).into_iter().filter(|q| !clue_pos.contains(q)).collect();
        let t = Tree::new(&p, &order, &fixed);
        let smc_seed = smc_seed0 + 1000 * u64::from(seed) + dir as u64;
        let ts = Instant::now();
        let (cap, run) = capture(&t, np, smc_seed, &depths, ms, &SURV_DEPTHS);
        let smc_s = ts.elapsed().as_secs_f64();
        let ext = run.est.iter().rposition(|&x| x > 0.0).unwrap_or(0);
        for &k in &depths {
            if k > t.nfree {
                continue;
            }
            let lw = run.est.get(k).copied().unwrap_or(0.0);
            let truth = replay(&t, &p, &full, k);
            let (tv, _) = ctx.all(&t, &truth, k);
            let Some(c) = cap.get(&k) else {
                println!(
                    "{{\"role\":\"{role}\",\"board\":{seed},\"dir\":{dir},\"depth\":{k},\"extinct\":true,\"extinction_depth\":{ext},\"true\":{}}}",
                    jf(&tv)
                );
                continue;
            };
            let (popn, sample, decoy) = (&c.popn, &c.sample, &c.decoy);
            let tq = Instant::now();
            let mut vals = Vec::with_capacity(sample.len());
            let mut nsum = [0u64; NSTAT];
            for st in sample {
                let (v, ns) = ctx.all(&t, st, k);
                for s in 0..NSTAT {
                    nsum[s] += ns[s];
                }
                vals.push(v);
            }
            let (mean, sd) = summary(&vals);
            let (tb, te) = rank(&vals, &tv);
            let (dv, db, de) = match decoy {
                Some(d) => {
                    let st = replay(&t, &p, &placement_of(d, &p), k);
                    let (v, _) = ctx.all(&t, &st, k);
                    let (b, e) = rank(&vals, &v);
                    (v.to_vec(), b, e)
                }
                None => (vec![f64::NAN; NSTAT], vec![f64::NAN; NSTAT], vec![f64::NAN; NSTAT]),
            };
            let ns_per: Vec<f64> = nsum.iter().map(|&x| x as f64 / sample.len() as f64).collect();
            let (wmean, wsd, wb, we, ess) = wsummary(&vals, &c.w, &tv);
            // A4 change 2: census-dead sampled prefixes, feasible-only reference
            assert!(!ctx.census_dead(&truth), "true prefix census-dead: board {seed} dir {dir} depth {k}");
            let dead: Vec<bool> = sample.iter().map(|st| ctx.census_dead(st)).collect();
            let n_dead = dead.iter().filter(|&&x| x).count();
            let fv: Vec<[f64; NSTAT]> = vals.iter().zip(&dead).filter(|(_, &d)| !d).map(|(v, _)| *v).collect();
            let fw: Vec<f64> = c.w.iter().zip(&dead).filter(|(_, &d)| !d).map(|(w, _)| *w).collect();
            let wtot: f64 = c.w.iter().sum();
            let wdead: f64 = c.w.iter().zip(&dead).filter(|(_, &d)| d).map(|(w, _)| *w).sum();
            let (fmean, fsd) = summary(&fv);
            let (ftb, fte) = rank(&fv, &tv);
            let (fwmean, fwsd, fwb, fwe, fess) = wsummary(&fv, &fw, &tv);
            // S6 percentile with census-dead prefixes counted as lowest (value reading)
            let (mut s6n, mut s6w) = (0f64, 0f64);
            for ((v, &d), &w) in vals.iter().zip(&dead).zip(&c.w) {
                let x = if d || v[5] < tv[5] { 1.0 } else if v[5] == tv[5] { 0.5 } else { 0.0 };
                s6n += x / vals.len() as f64;
                s6w += x * w / wtot;
            }
            // A4 change 3: survivors one and two rows ahead
            let surv_json = match &c.surv {
                None => String::new(),
                Some(sv) => {
                    let summ = |sts: &[State]| -> (usize, usize, Vec<f64>, Vec<f64>) {
                        let mut v = Vec::new();
                        let mut nd = 0;
                        for st in sts {
                            if ctx.census_dead(st) {
                                nd += 1;
                            } else {
                                v.push(ctx.all(&t, st, k).0);
                            }
                        }
                        let (mu, sd) = if v.is_empty() { (vec![f64::NAN; NSTAT], vec![f64::NAN; NSTAT]) } else { summary(&v) };
                        (v.len(), nd, mu, sd)
                    };
                    let (n1, d1, m1, s1) = summ(&sv.states);
                    let (n2, d2, m2, s2) = summ(&sv.r2_states);
                    format!(
                        ",\"surv_scanned\":{},\"surv_found\":{},\"surv_frac\":{:.6e},\"surv_n\":{n1},\"surv_dead\":{d1},\"surv_mean\":{},\"surv_sd\":{},\"surv2_tested\":{},\"surv2_pass\":{},\"surv2_capped\":{},\"surv2_n\":{n2},\"surv2_dead\":{d2},\"surv2_mean\":{},\"surv2_sd\":{}",
                        sv.scanned,
                        sv.states.len(),
                        sv.states.len() as f64 / sv.scanned.max(1) as f64,
                        jf(&m1),
                        jf(&s1),
                        sv.r2_tested,
                        sv.r2_states.len(),
                        sv.r2_capped,
                        jf(&m2),
                        jf(&s2)
                    )
                }
            };
            let a4_json = format!(
                ",\"fam_distinct\":{},\"fam_ess\":{:.3},\"n_dead\":{n_dead},\"work_dead_share\":{:.6e},\"mean_feas\":{},\"sd_feas\":{},\"true_below_feas\":{},\"true_equal_feas\":{},\"work_mean_feas\":{},\"work_sd_feas\":{},\"work_true_below_feas\":{},\"work_true_equal_feas\":{},\"work_ess_feas\":{:.3},\"s6_node_pct_deadlow\":{:.6},\"s6_work_pct_deadlow\":{:.6}{surv_json}",
                c.fam_distinct,
                c.fam_ess,
                wdead / wtot,
                jf(&fmean),
                jf(&fsd),
                jf(&ftb),
                jf(&fte),
                jf(&fwmean),
                jf(&fwsd),
                jf(&fwb),
                jf(&fwe),
                fess,
                s6n,
                s6w
            );
            println!(
                "{{\"role\":\"{role}\",\"board\":{seed},\"dir\":{dir},\"depth\":{k},\"extinct\":false,\"extinction_depth\":{ext},\"np\":{np},\"pop\":{popn},\"m\":{},\"log10_width\":{:.6},\"true\":{},\"decoy\":{},\"mean\":{},\"sd\":{},\"true_below\":{},\"true_equal\":{},\"decoy_below\":{},\"decoy_equal\":{},\"ns_per_eval\":{},\"work_mean\":{},\"work_sd\":{},\"work_true_below\":{},\"work_true_equal\":{},\"work_ess\":{:.3},\"stat_seconds\":{:.3},\"smc_seconds\":{:.3},\"ctx_seconds\":{:.3}{a4_json}}}",
                sample.len(),
                if lw > 0.0 { lw.log10() } else { -1.0 },
                jf(&tv),
                jf(&dv),
                jf(&mean),
                jf(&sd),
                jf(&tb),
                jf(&te),
                jf(&db),
                jf(&de),
                jf(&ns_per),
                jf(&wmean),
                jf(&wsd),
                jf(&wb),
                jf(&we),
                ess,
                tq.elapsed().as_secs_f64(),
                smc_s,
                ctx_s
            );
        }
        let prof: Vec<f64> = run.est.iter().map(|&x| if x > 0.0 { x.log10() } else { f64::NEG_INFINITY }).collect();
        println!("{{\"role\":\"{role}\",\"board\":{seed},\"dir\":{dir},\"profile_log10_width\":{}}}", jf(&prof));
    }
    println!(
        "{{\"role\":\"{role}\",\"board\":{seed},\"done\":true,\"seconds\":{:.2},\"names\":[{}]}}",
        t_all.elapsed().as_secs_f64(),
        NAMES.iter().map(|s| format!("\"{s}\"")).collect::<Vec<_>>().join(",")
    );
}

fn cmd_real(m: &HashMap<String, String>) {
    let p = load_csv(m.get("csv").expect("--csv"));
    let np: usize = get(m, "np", 100_000);
    let ms: usize = get(m, "m", 1000);
    let dirs = list(m, "dirs", "0,1,2,3");
    let depths = list(m, "depths", "32,64,96,128,144,160,176,192,208,224");
    let seed0: u64 = get(m, "smc-seed", 7);
    let fixed = p.hints.clone();
    let clue_pos: Vec<usize> = fixed.iter().map(|h| h.0).collect();
    let ctx = Ctx::new(p.n, &p.pieces, &clue_pos);
    for &dir in &dirs {
        let order: Vec<usize> = scan(p.n, dir).into_iter().filter(|q| !clue_pos.contains(q)).collect();
        let t = Tree::new(&p, &order, &fixed);
        let (cap, run) = capture(&t, np, seed0 + dir as u64, &depths, ms, &[]);
        let ext = run.est.iter().rposition(|&x| x > 0.0).unwrap_or(0);
        for &k in &depths {
            let Some(Cap { popn, sample, .. }) = cap.get(&k) else {
                println!("{{\"role\":\"real\",\"dir\":{dir},\"depth\":{k},\"extinct\":true,\"extinction_depth\":{ext}}}");
                continue;
            };
            let vals: Vec<[f64; NSTAT]> = sample.iter().map(|st| ctx.all(&t, st, k).0).collect();
            let (mean, sd) = summary(&vals);
            println!(
                "{{\"role\":\"real\",\"dir\":{dir},\"depth\":{k},\"extinct\":false,\"extinction_depth\":{ext},\"pop\":{popn},\"m\":{},\"log10_width\":{:.6},\"mean\":{},\"sd\":{}}}",
                sample.len(),
                run.est[k].log10(),
                jf(&mean),
                jf(&sd)
            );
        }
    }
}

/// Exact moments of every statistic at the target depths, by enumeration of
/// the whole tree (small boards only): node-uniform (mean, sd) and
/// work-weighted (mean, sd), the weight of a node being the size of its
/// subtree including itself. Returns depth -> (nodes, mean, sd, wmean, wsd).
type Mom = (f64, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>);
fn exact_moments(t: &Tree, ctx: &Ctx, depths: &[usize], cap: u64) -> Option<HashMap<usize, Mom>> {
    // acc: n, s1, s2, W, ws1, ws2
    let mut acc: HashMap<usize, (f64, [f64; NSTAT], [f64; NSTAT], f64, [f64; NSTAT], [f64; NSTAT])> = HashMap::new();
    for &d in depths {
        acc.insert(d, (0.0, [0.0; NSTAT], [0.0; NSTAT], 0.0, [0.0; NSTAT], [0.0; NSTAT]));
    }
    let mut st = t.root();
    let mut nodes = 0u64;
    #[allow(clippy::type_complexity)]
    fn rec(
        t: &Tree,
        ctx: &Ctx,
        st: &mut State,
        k: usize,
        depths: &[usize],
        acc: &mut HashMap<usize, (f64, [f64; NSTAT], [f64; NSTAT], f64, [f64; NSTAT], [f64; NSTAT])>,
        nodes: &mut u64,
        cap: u64,
    ) -> Option<f64> {
        let v = if depths.contains(&k) { Some(ctx.all(t, st, k).0) } else { None };
        let mut size = 1.0;
        if k < t.nfree {
            let mut kids = Vec::new();
            t.for_each_child(st, k, |i| kids.push(i));
            for i in kids {
                *nodes += 1;
                if *nodes > cap {
                    return None;
                }
                t.apply(st, k, i);
                let r = rec(t, ctx, st, k + 1, depths, acc, nodes, cap);
                t.undo(st, k, i);
                size += r?;
            }
        }
        if let Some(v) = v {
            let a = acc.get_mut(&k).unwrap();
            a.0 += 1.0;
            a.3 += size;
            for s in 0..NSTAT {
                a.1[s] += v[s];
                a.2[s] += v[s] * v[s];
                a.4[s] += size * v[s];
                a.5[s] += size * v[s] * v[s];
            }
        }
        Some(size)
    }
    rec(t, ctx, &mut st, 0, depths, &mut acc, &mut nodes, cap)?;
    let mut out = HashMap::new();
    for (d, a) in acc {
        let mut mu = vec![f64::NAN; NSTAT];
        let mut sd = vec![f64::NAN; NSTAT];
        let mut wmu = vec![f64::NAN; NSTAT];
        let mut wsd = vec![f64::NAN; NSTAT];
        if a.0 > 0.0 {
            for s in 0..NSTAT {
                mu[s] = a.1[s] / a.0;
                sd[s] = (a.2[s] / a.0 - mu[s] * mu[s]).max(0.0).sqrt();
                wmu[s] = a.4[s] / a.3;
                wsd[s] = (a.5[s] / a.3 - wmu[s] * wmu[s]).max(0.0).sqrt();
            }
        }
        out.insert(d, (a.0, mu, sd, wmu, wsd));
    }
    Some(out)
}

fn cmd_gate(m: &HashMap<String, String>) {
    let n: usize = get(m, "n", 6);
    let colors: u8 = get(m, "colors", 9);
    let border: u8 = get(m, "border", 2);
    let boards = list(m, "boards", "1,2,3");
    let np: usize = get(m, "np", 200);
    let reps: usize = get(m, "reps", 50);
    let ms: usize = get(m, "m", 200);
    let cap: u64 = get(m, "cap", 50_000_000);
    let fr = list(m, "fracs", "125,250,375,500,625,750");
    for &b in &boards {
        let (p, plant) = planted(n, colors, border, b as u32);
        let order = rows_bottom_up(n);
        let t = Tree::new(&p, &order, &[]);
        let mut full: HashMap<usize, (u16, Edges)> = HashMap::new();
        for &(pos, pid, r) in &plant {
            full.insert(pos, (pid, rotate(p.pieces[pid as usize], r)));
        }
        let mut ctx = Ctx::new(n, &p.pieces, &[]);
        ctx.s10_cap = u64::MAX;
        let depths: Vec<usize> = fr.iter().map(|&f| (t.nfree * f + 500) / 1000).collect();
        let t0 = Instant::now();
        let Some(ex) = exact_moments(&t, &ctx, &depths, cap) else {
            println!("{{\"board\":{b},\"n\":{n},\"skipped\":\"cap\"}}");
            continue;
        };
        let ex_s = t0.elapsed().as_secs_f64();
        for rep in 0..reps {
            let (capd, _) = capture(&t, np, 10_000 * b as u64 + rep as u64, &depths, ms, &[]);
            for &k in &depths {
                let (cnt, emu, esd, ewmu, ewsd) = &ex[&k];
                let tv = ctx.all(&t, &replay(&t, &p, &full, k), k).0;
                let line_base = format!(
                    "\"board\":{b},\"n\":{n},\"colors\":{colors},\"rep\":{rep},\"depth\":{k},\"exact_nodes\":{cnt},\"exact_mean\":{},\"exact_sd\":{},\"exact_work_mean\":{},\"exact_work_sd\":{},\"exact_seconds\":{ex_s:.2},\"true\":{}",
                    jf(emu),
                    jf(esd),
                    jf(ewmu),
                    jf(ewsd),
                    jf(&tv)
                );
                match capd.get(&k) {
                    Some(c) => {
                        let vals: Vec<[f64; NSTAT]> = c.sample.iter().map(|st| ctx.all(&t, st, k).0).collect();
                        let (mean, sd) = summary(&vals);
                        let (wmean, wsd, _, _, ess) = wsummary(&vals, &c.w, &[0.0; NSTAT]);
                        println!(
                            "{{{line_base},\"pop\":{},\"m\":{},\"smc_mean\":{},\"smc_sd\":{},\"smc_work_mean\":{},\"smc_work_sd\":{},\"work_ess\":{:.3}}}",
                            c.popn,
                            c.sample.len(),
                            jf(&mean),
                            jf(&sd),
                            jf(&wmean),
                            jf(&wsd),
                            ess
                        );
                    }
                    None => println!("{{{line_base},\"extinct\":true}}"),
                }
            }
        }
    }
}

/// A4 change 8, painter control: on small boards with several solutions,
/// exact census statistics (S4, S6, S9) on the prefixes of the planted
/// solution and of the other solutions, against exact node-uniform moments.
fn cmd_painter(m: &HashMap<String, String>) {
    let n: usize = get(m, "n", 6);
    let colors: u8 = get(m, "colors", 7);
    let border: u8 = get(m, "border", 2);
    let seed0: u32 = get(m, "seed0", 9101);
    let count: u32 = get(m, "count", 10);
    let cap: u64 = get(m, "cap", 200_000_000);
    let fr = list(m, "fracs", "125,250,375,500,625,750");
    for b in seed0..seed0 + count {
        let (p, plant) = planted(n, colors, border, b);
        let order = rows_bottom_up(n);
        let t = Tree::new(&p, &order, &[]);
        let ctx = Ctx::new(n, &p.pieces, &[]);
        let depths: Vec<usize> = fr.iter().map(|&f| (t.nfree * f + 500) / 1000).collect();
        let mut acc: HashMap<usize, (f64, [f64; 3], [f64; 3])> = HashMap::new();
        let mut sols: Vec<(Vec<u16>, Vec<u8>)> = Vec::new();
        let mut nodes = 0u64;
        #[allow(clippy::too_many_arguments)]
        fn rec(
            t: &Tree,
            ctx: &Ctx,
            st: &mut State,
            k: usize,
            depths: &[usize],
            acc: &mut HashMap<usize, (f64, [f64; 3], [f64; 3])>,
            sols: &mut Vec<(Vec<u16>, Vec<u8>)>,
            nodes: &mut u64,
            cap: u64,
        ) -> bool {
            if depths.contains(&k) {
                let v = ctx.census3(st);
                let a = acc.entry(k).or_insert((0.0, [0.0; 3], [0.0; 3]));
                a.0 += 1.0;
                for s in 0..3 {
                    a.1[s] += v[s];
                    a.2[s] += v[s] * v[s];
                }
            }
            if k == t.nfree {
                sols.push((st.cell.clone(), st.board.clone()));
                return true;
            }
            let mut kids = Vec::new();
            t.for_each_child(st, k, |i| kids.push(i));
            for i in kids {
                *nodes += 1;
                if *nodes > cap {
                    return false;
                }
                t.apply(st, k, i);
                let ok = rec(t, ctx, st, k + 1, depths, acc, sols, nodes, cap);
                t.undo(st, k, i);
                if !ok {
                    return false;
                }
            }
            true
        }
        let t0 = Instant::now();
        let mut st = t.root();
        if !rec(&t, &ctx, &mut st, 0, &depths, &mut acc, &mut sols, &mut nodes, cap) {
            println!("{{\"board\":{b},\"skipped\":\"cap\"}}");
            continue;
        }
        let mut pl = vec![0u16; n * n];
        let mut plb = vec![0u8; 4 * n * n];
        for &(pos, pid, r) in &plant {
            pl[pos] = pid;
            plb[pos * 4..pos * 4 + 4].copy_from_slice(&rotate(p.pieces[pid as usize], r));
        }
        let rot = |pos: usize| -> usize { let (r, c) = (pos / n, pos % n); c * n + (n - 1 - r) };
        let kind = |s: &Vec<u16>| -> u8 {
            if *s == pl {
                return 0;
            }
            for q in 1..4 {
                let mut ok = true;
                for pos in 0..n * n {
                    let mut x = pos;
                    for _ in 0..q {
                        x = rot(x);
                    }
                    if s[x] != pl[pos] {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    return 1;
                }
            }
            2
        };
        // census values along a solution's row-scan prefixes
        let along = |s: &Vec<u16>, sb: &Vec<u8>| -> Vec<[f64; 3]> {
            let mut full: HashMap<usize, (u16, Edges)> = HashMap::new();
            // recover orientation by replaying: find the oriented piece matching at each step
            let mut st = t.root();
            let mut out = Vec::new();
            for j in 0..t.nfree {
                if depths.contains(&j) {
                    out.push(ctx.census3(&st));
                }
                let pos = t.pos(j);
                let mut chosen = None;
                t.for_each_child(&st, j, |i| {
                    let (pid, e) = t.oriented(i);
                    if pid == s[pos] && e[..] == sb[pos * 4..pos * 4 + 4] && chosen.is_none() {
                        chosen = Some(i);
                    }
                });
                let Some(i) = chosen else {
                    panic!("solution step is not a child: step {j} pos {pos} tile {} sol {:?}", s[pos], s);
                };
                full.insert(pos, t.oriented(i));
                t.apply(&mut st, j, i);
            }
            out
        };
        let kinds: Vec<u8> = sols.iter().map(|x| kind(&x.0)).collect();
        let planted_vals = along(&pl, &plb);
        let others: Vec<Vec<[f64; 3]>> = sols.iter().zip(&kinds).filter(|(_, &k)| k == 2).map(|(s, _)| along(&s.0, &s.1)).collect();
        let mut rows = Vec::new();
        for (di, &k) in depths.iter().enumerate() {
            let a = acc[&k];
            let mu: Vec<f64> = (0..3).map(|s| a.1[s] / a.0).collect();
            let sd: Vec<f64> = (0..3).map(|s| (a.2[s] / a.0 - mu[s] * mu[s]).max(0.0).sqrt()).collect();
            let om: Vec<f64> = (0..3)
                .map(|s| if others.is_empty() { f64::NAN } else { others.iter().map(|o| o[di][s]).sum::<f64>() / others.len() as f64 })
                .collect();
            rows.push(format!(
                "{{\"depth\":{k},\"nodes\":{},\"mean\":{},\"sd\":{},\"planted\":{},\"others_mean\":{}}}",
                a.0,
                jf(&mu),
                jf(&sd),
                jf(&planted_vals[di]),
                jf(&om)
            ));
        }
        println!(
            "{{\"board\":{b},\"n\":{n},\"colors\":{colors},\"border\":{border},\"tree_nodes\":{nodes},\"solutions\":{},\"planted_rotations\":{},\"others\":{},\"seconds\":{:.2},\"depths\":[{}]}}",
            sols.len(),
            kinds.iter().filter(|&&k| k == 1).count(),
            others.len(),
            t0.elapsed().as_secs_f64(),
            rows.join(",")
        );
    }
}

fn cmd_smc(m: &HashMap<String, String>) {
    let p = load_csv(m.get("csv").expect("--csv"));
    let np: usize = get(m, "np", 100_000);
    let reps: usize = get(m, "reps", 1);
    let seed0: u64 = get(m, "seed0", 1);
    let fixed = p.hints.clone();
    let order: Vec<usize> = rows_bottom_up(p.n).into_iter().filter(|q| !fixed.iter().any(|f| f.0 == *q)).collect();
    let t = Tree::new(&p, &order, &fixed);
    for r in 0..reps {
        let seed = seed0 + r as u64;
        let t0 = Instant::now();
        let run = smc(&t, np, seed);
        let total: f64 = run.est[1..].iter().sum();
        let ext = run.est.iter().rposition(|&x| x > 0.0).unwrap_or(0);
        println!(
            "{{\"seed\":{seed},\"np\":{np},\"total\":{total:.6e},\"log10_total\":{:.6},\"extinction_depth\":{ext},\"seconds\":{:.2}}}",
            total.log10(),
            t0.elapsed().as_secs_f64()
        );
    }
}

/// Unplanned follow-up: share of sampled prefixes that arc consistency does
/// not refute, with the test stage's sampler, seeds, subsample and work
/// weights (capture() with no survivor depths; survivors use their own RNG,
/// so the population and subsample are those of the test stage).
fn cmd_acshare(m: &HashMap<String, String>) {
    let seed: u32 = m.get("board").map(|v| v.parse().expect("--board must be an integer")).expect("--board");
    let np: usize = get(m, "np", 500_000);
    let ms: usize = get(m, "m", 1000);
    let dirs = list(m, "dirs", "0,1,2,3");
    let depths = list(m, "depths", "96,112,128,136,144,148,152,156,160");
    let (p, plant) = planted(16, 22, 5, seed);
    let clue_pos: Vec<usize> = CLUES16.to_vec();
    let fixed: Vec<(usize, u16, u8)> = plant.iter().copied().filter(|x| clue_pos.contains(&x.0)).collect();
    let mut full: HashMap<usize, (u16, Edges)> = HashMap::new();
    for &(pos, pid, r) in &plant {
        full.insert(pos, (pid, rotate(p.pieces[pid as usize], r)));
    }
    let ctx = Ctx::new(16, &p.pieces, &clue_pos);
    for &dir in &dirs {
        let order: Vec<usize> = scan(16, dir).into_iter().filter(|q| !clue_pos.contains(q)).collect();
        let t = Tree::new(&p, &order, &fixed);
        let smc_seed = 1 + 1000 * u64::from(seed) + dir as u64;
        let ts = Instant::now();
        let (cap, run) = capture(&t, np, smc_seed, &depths, ms, &[]);
        let smc_s = ts.elapsed().as_secs_f64();
        for &k in &depths {
            let truth = replay(&t, &p, &full, k);
            let true_ref = ctx.ac_refuted(&truth);
            let Some(c) = cap.get(&k) else {
                println!("{{\"board\":{seed},\"dir\":{dir},\"depth\":{k},\"extinct\":true,\"true_refuted\":{true_ref}}}");
                continue;
            };
            let tq = Instant::now();
            let (mut ok, mut wok, mut wtot) = (0usize, 0f64, 0f64);
            for (st, &w) in c.sample.iter().zip(&c.w) {
                let r = ctx.ac_refuted(st);
                wtot += w;
                if !r {
                    ok += 1;
                    wok += w;
                }
            }
            let ns = tq.elapsed().as_nanos() as f64 / c.sample.len() as f64;
            let sw2: f64 = c.w.iter().map(|a| a * a).sum();
            let lw = run.est.get(k).copied().unwrap_or(0.0);
            println!(
                "{{\"board\":{seed},\"dir\":{dir},\"depth\":{k},\"extinct\":false,\"pop\":{},\"m\":{},\"log10_width\":{:.6},\"consistent\":{ok},\"share\":{:.6e},\"work_share\":{:.6e},\"work_ess\":{:.3},\"ns_per_check\":{:.0},\"true_refuted\":{true_ref},\"smc_seconds\":{smc_s:.3}}}",
                c.popn,
                c.sample.len(),
                if lw > 0.0 { lw.log10() } else { -1.0 },
                ok as f64 / c.sample.len() as f64,
                wok / wtot,
                wtot * wtot / sw2,
                ns
            );
        }
    }
}

fn main() {
    let (cmd, m) = args_map();
    match cmd.as_str() {
        "smc" => cmd_smc(&m),
        "measure" => cmd_measure(&m),
        "real" => cmd_real(&m),
        "gate" => cmd_gate(&m),
        "painter" => cmd_painter(&m),
        "acshare" => cmd_acshare(&m),
        _ => {
            eprintln!("usage: psx smc|measure|real|gate [--key value]...");
            std::process::exit(2);
        }
    }
}
