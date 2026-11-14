//! Divide-and-concur difference map for edge matching with exact band projections.
//!
//!   dmb run --instances F.jsonl --k K --runs 1-3 --cap ITERS [--beta 0.25]
//!           [--seeds lo-hi] [--sizes 6,7] --out OUT.jsonl
//!
//! State x: one real vector over colours for each side of each cell, per replica
//! (cell, side N/E/S/W, colour). Sets, all with exact projections:
//!   A    layouts: every cell one oriented piece, every piece once (Hungarian);
//!   R_k  row bands of k rows, each a fully matched tiling, repeats allowed
//!        (max-plus broken-profile DP, state = k lane colours + one vertical colour);
//!   K_k  the same for column bands;
//!   B    seams: facing sides averaged, rim grey (linear subspace).
//! k = 0 runs A against B only. Update (relaxed difference map, form "a" of the
//! memo's dm.py): pc = P_B(mean X); X_r += beta (P_r(2 pc - X_r) - pc). Stop when
//! the layout produced by P_A has zero mismatched seams. Single thread.
//!
//! Boards are row-major `piece * 4 + rot`, rot = clockwise quarter turns, sides
//! URDL, the hard-ladder convention (`eternity2_engine::rotated`).

use serde_json::{json, Value};
use std::io::{BufRead, BufWriter, Write};
use std::time::Instant;

const N_: usize = 0;
const E_: usize = 1;
const S_: usize = 2;
const W_: usize = 3;
const NEG: f64 = -1e18;

fn rotated(e: [u8; 4], r: usize) -> [u8; 4] {
    [
        e[(4 - r) & 3],
        e[(5 - r) & 3],
        e[(6 - r) & 3],
        e[(7 - r) & 3],
    ]
}

struct Inst {
    id: String,
    n: usize,
    c: i64,
    seed: i64,
    pieces: Vec<[u8; 4]>,
    planted: Option<Vec<i64>>,
}

struct SplitMix(u64);
impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// One band: cells in processing order (position along the band outer, lane
/// inner), each with its (left, right, up, down) sides in board terms.
struct Band {
    h: usize,
    steps: Vec<(usize, usize, [usize; 4])>, // (cell, lane, sides)
}

struct Model {
    n: usize,
    p: usize,
    t: usize,
    c: usize,
    col: Vec<[u8; 4]>,
    valid: Vec<Vec<u16>>,
    bands: Vec<Band>,
    // DP scratch, sized for the tallest band
    bases: Vec<Vec<Vec<usize>>>, // bases[h][lane]
    m: Vec<f64>,
    m2: Vec<f64>,
    red: Vec<f64>,
    redarg: Vec<u32>,
    bpt: Vec<u16>,
    bpo: Vec<u32>,
    pub transitions: u64,
}

impl Model {
    fn new(inst: &Inst, k: usize) -> Model {
        let n = inst.n;
        let p = n * n;
        let t = 4 * p;
        let mut c = 0usize;
        let mut col = Vec::with_capacity(t);
        for pc in &inst.pieces {
            for r in 0..4 {
                let o = rotated(*pc, r);
                for &x in &o {
                    c = c.max(x as usize + 1);
                }
                col.push(o);
            }
        }
        let mut valid = vec![Vec::new(); p];
        for r in 0..n {
            for j in 0..n {
                let rim = [r == 0, j == n - 1, r == n - 1, j == 0];
                for s in 0..t {
                    if (0..4).all(|q| (col[s][q] == 0) == rim[q]) {
                        valid[r * n + j].push(s as u16);
                    }
                }
            }
        }
        let mut bands = Vec::new();
        if k > 0 {
            // row bands: rows [b, b+h); position = column j; lane = row offset
            let mut b = 0;
            while b < n {
                let h = k.min(n - b);
                let mut steps = Vec::new();
                for j in 0..n {
                    for i in 0..h {
                        steps.push(((b + i) * n + j, i, [W_, E_, N_, S_]));
                    }
                }
                bands.push(Band { h, steps });
                b += h;
            }
            // column bands: columns [b, b+h); position = row r; lane = column offset
            let mut b = 0;
            while b < n {
                let h = k.min(n - b);
                let mut steps = Vec::new();
                for r in 0..n {
                    for i in 0..h {
                        steps.push((r * n + b + i, i, [N_, S_, W_, E_]));
                    }
                }
                bands.push(Band { h, steps });
                b += h;
            }
        }
        let hmax = bands.iter().map(|b| b.h).max().unwrap_or(0);
        let mut bases = vec![Vec::new(); hmax + 1];
        for h in 1..=hmax {
            for lane in 0..h {
                // all indices with e_lane = 0 and s = 0
                let others = h - 1;
                let mut v = Vec::new();
                for combo in 0..c.pow(others as u32) {
                    let mut idx = 0usize;
                    let mut rem = combo;
                    for j in 0..h {
                        if j == lane {
                            continue;
                        }
                        let e = rem % c;
                        rem /= c;
                        idx += e * c.pow(j as u32 + 1);
                    }
                    v.push(idx);
                }
                bases[h].push(v);
            }
        }
        let smax = if hmax > 0 { c.pow(hmax as u32 + 1) } else { 0 };
        let stepmax = bands.iter().map(|b| b.steps.len()).max().unwrap_or(0);
        Model {
            n,
            p,
            t,
            c,
            col,
            valid,
            bands,
            bases,
            m: vec![NEG; smax],
            m2: vec![NEG; smax],
            red: vec![NEG; smax],
            redarg: vec![0; smax],
            bpt: vec![0; smax * stepmax],
            bpo: vec![0; smax * stepmax],
            transitions: 0,
        }
    }

    fn dim(&self) -> usize {
        self.p * 4 * self.c
    }

    fn unary(&self, x: &[f64], u: &mut [f64]) {
        let c = self.c;
        for cell in 0..self.p {
            for &s in &self.valid[cell] {
                let s = s as usize;
                let o = self.col[s];
                let b = cell * 4 * c;
                u[cell * self.t + s] = x[b + o[0] as usize]
                    + x[b + c + o[1] as usize]
                    + x[b + 2 * c + o[2] as usize]
                    + x[b + 3 * c + o[3] as usize];
            }
        }
    }

    fn proj_a(&self, u: &[f64], out: &mut [usize]) {
        let p = self.p;
        let mut score = vec![-1e9; p * p];
        let mut rot = vec![0usize; p * p];
        for cell in 0..p {
            for &s in &self.valid[cell] {
                let s = s as usize;
                let (pc, r) = (s / 4, s % 4);
                let v = u[cell * self.t + s];
                if v > score[cell * p + pc] {
                    score[cell * p + pc] = v;
                    rot[cell * p + pc] = r;
                }
            }
        }
        let asg = hungarian_max(&score, p);
        for cell in 0..p {
            out[cell] = asg[cell] * 4 + rot[cell * p + asg[cell]];
        }
    }

    fn proj_band(&mut self, bi: usize, u: &[f64], out: &mut [usize]) {
        let c = self.c;
        let h = self.bands[bi].h;
        let ssz = c.pow(h as u32 + 1);
        let nsteps = self.bands[bi].steps.len();
        let m = &mut self.m;
        let m2 = &mut self.m2;
        let red = &mut self.red;
        let redarg = &mut self.redarg;
        let bpt = &mut self.bpt;
        let bpo = &mut self.bpo;
        m[..ssz].fill(NEG);
        m[0] = 0.0;
        let mut trans = 0u64;
        for st in 0..nsteps {
            let (cell, lane, sides) = self.bands[bi].steps[st];
            let [sl, sr, su, sd] = sides;
            let stride = c.pow(lane as u32 + 1);
            m2[..ssz].fill(NEG);
            let bases = &self.bases[h][lane];
            let bp_off = st * ssz;
            if lane == 0 {
                // the vertical colour carried from the previous position is free
                for e in (0..ssz).step_by(c) {
                    let mut best = NEG;
                    let mut arg = 0u32;
                    for s in 0..c {
                        if m[e + s] > best {
                            best = m[e + s];
                            arg = s as u32;
                        }
                    }
                    red[e] = best;
                    redarg[e] = arg;
                }
            }
            for &tt in &self.valid[cell] {
                let t = tt as usize;
                let o = self.col[t];
                let w = u[cell * self.t + t];
                let (l, r, d) = (o[sl] as usize, o[sr] as usize, o[sd] as usize);
                if lane == 0 {
                    for &b in bases {
                        trans += 1;
                        let old = b + l * stride;
                        let v = red[old];
                        if v <= NEG * 0.5 {
                            continue;
                        }
                        let nv = v + w;
                        let new = b + r * stride + d;
                        if nv > m2[new] {
                            m2[new] = nv;
                            bpt[bp_off + new] = tt;
                            bpo[bp_off + new] = (old + redarg[old] as usize) as u32;
                        }
                    }
                } else {
                    let up = o[su] as usize;
                    for &b in bases {
                        trans += 1;
                        let old = b + l * stride + up;
                        let v = m[old];
                        if v <= NEG * 0.5 {
                            continue;
                        }
                        let nv = v + w;
                        let new = b + r * stride + d;
                        if nv > m2[new] {
                            m2[new] = nv;
                            bpt[bp_off + new] = tt;
                            bpo[bp_off + new] = old as u32;
                        }
                    }
                }
            }
            std::mem::swap(m, m2);
        }
        let mut idx = 0usize;
        let mut best = NEG;
        for (i, &v) in m[..ssz].iter().enumerate() {
            if v > best {
                best = v;
                idx = i;
            }
        }
        assert!(best > NEG * 0.5, "band {bi} has no tiling");
        for st in (0..nsteps).rev() {
            let cell = self.bands[bi].steps[st].0;
            out[cell] = bpt[st * ssz + idx] as usize;
            idx = bpo[st * ssz + idx] as usize;
        }
        self.transitions += trans;
    }

    /// Projection onto R_k (`which` = 0) or K_k (`which` = 1).
    fn proj_bands(&mut self, which: usize, u: &[f64], out: &mut [usize]) {
        let nb = self.bands.len() / 2;
        for bi in which * nb..(which + 1) * nb {
            self.proj_band(bi, u, out);
        }
    }

    fn embed(&self, states: &[usize], out: &mut [f64]) {
        out.fill(0.0);
        let c = self.c;
        for cell in 0..self.p {
            let o = self.col[states[cell]];
            for s in 0..4 {
                out[(cell * 4 + s) * c + o[s] as usize] = 1.0;
            }
        }
    }

    fn proj_b(&self, y: &mut [f64]) {
        let (n, c) = (self.n, self.c);
        let at = |cell: usize, s: usize| (cell * 4 + s) * c;
        for r in 0..n {
            for j in 0..n {
                let cell = r * n + j;
                if j + 1 < n {
                    let (a, b) = (at(cell, E_), at(cell + 1, W_));
                    for q in 0..c {
                        let v = 0.5 * (y[a + q] + y[b + q]);
                        y[a + q] = v;
                        y[b + q] = v;
                    }
                }
                if r + 1 < n {
                    let (a, b) = (at(cell, S_), at(cell + n, N_));
                    for q in 0..c {
                        let v = 0.5 * (y[a + q] + y[b + q]);
                        y[a + q] = v;
                        y[b + q] = v;
                    }
                }
                let rim = [r == 0, j == n - 1, r == n - 1, j == 0];
                for s in 0..4 {
                    if rim[s] {
                        let a = at(cell, s);
                        for q in 0..c {
                            y[a + q] = 0.0;
                        }
                        y[a] = 1.0;
                    }
                }
            }
        }
    }

    fn mismatches(&self, states: &[usize]) -> usize {
        let n = self.n;
        let mut mm = 0;
        for r in 0..n {
            for j in 0..n {
                let o = self.col[states[r * n + j]];
                if j + 1 < n && o[E_] != self.col[states[r * n + j + 1]][W_] {
                    mm += 1;
                }
                if r + 1 < n && o[S_] != self.col[states[(r + 1) * n + j]][N_] {
                    mm += 1;
                }
            }
        }
        mm
    }
}

/// Maximum-weight perfect assignment (shortest augmenting path, O(n^3)).
/// Returns, for each row, its column.
fn hungarian_max(score: &[f64], n: usize) -> Vec<usize> {
    let inf = f64::INFINITY;
    let mut u = vec![0.0; n + 1];
    let mut v = vec![0.0; n + 1];
    let mut p = vec![0usize; n + 1];
    let mut way = vec![0usize; n + 1];
    let mut minv = vec![inf; n + 1];
    let mut used = vec![false; n + 1];
    for i in 1..=n {
        p[0] = i;
        let mut j0 = 0usize;
        minv.fill(inf);
        used.fill(false);
        loop {
            used[j0] = true;
            let i0 = p[j0];
            let mut delta = inf;
            let mut j1 = 0usize;
            for j in 1..=n {
                if !used[j] {
                    let cur = -score[(i0 - 1) * n + (j - 1)] - u[i0] - v[j];
                    if cur < minv[j] {
                        minv[j] = cur;
                        way[j] = j0;
                    }
                    if minv[j] < delta {
                        delta = minv[j];
                        j1 = j;
                    }
                }
            }
            for j in 0..=n {
                if used[j] {
                    u[p[j]] += delta;
                    v[j] -= delta;
                } else {
                    minv[j] -= delta;
                }
            }
            j0 = j1;
            if p[j0] == 0 {
                break;
            }
        }
        loop {
            let j1 = way[j0];
            p[j0] = p[j1];
            j0 = j1;
            if j0 == 0 {
                break;
            }
        }
    }
    let mut ans = vec![0usize; n];
    for j in 1..=n {
        ans[p[j] - 1] = j - 1;
    }
    ans
}

/// Independent full-board check (same rules as hard-ladder `verify_board`).
fn verify(inst: &Inst, board: &[usize]) -> Result<(), String> {
    let n = inst.n;
    let mut used = vec![false; n * n];
    let mut e = Vec::new();
    for &v in board {
        let (p, r) = (v / 4, v % 4);
        if p >= n * n || used[p] {
            return Err(format!("piece {p} reused"));
        }
        used[p] = true;
        e.push(rotated(inst.pieces[p], r));
    }
    for y in 0..n {
        for x in 0..n {
            let [t, r, b, l] = e[y * n + x];
            if (y == 0) != (t == 0)
                || (y == n - 1) != (b == 0)
                || (x == 0) != (l == 0)
                || (x == n - 1) != (r == 0)
            {
                return Err(format!("rim at ({x},{y})"));
            }
            if x + 1 < n && r != e[y * n + x + 1][3] {
                return Err(format!("seam right of ({x},{y})"));
            }
            if y + 1 < n && b != e[(y + 1) * n + x][0] {
                return Err(format!("seam below ({x},{y})"));
            }
        }
    }
    Ok(())
}

struct RunOut {
    period: Option<u64>,
    distinct: Option<usize>,
    last_mm: usize,
    iters: Option<u64>,
    best_mm: usize,
    board: Vec<usize>,
    wall_ms: f64,
    transitions: u64,
    iters_done: u64,
}

fn run_dm(inst: &Inst, k: usize, run_seed: u64, cap: u64, beta: f64) -> RunOut {
    let t0 = Instant::now();
    let mut md = Model::new(inst, k);
    let dim = md.dim();
    let nrep = if k == 0 { 1 } else { 3 };
    let mut rng = SplitMix(
        (inst.seed as u64).wrapping_mul(0x100000001B3)
            ^ run_seed.wrapping_mul(0x9E3779B97F4A7C15)
            ^ 0xD1B54A32D192ED03,
    );
    // replica 0 (A) is drawn first, so it starts identically for every k
    let mut x: Vec<Vec<f64>> = (0..nrep)
        .map(|_| (0..dim).map(|_| rng.unit()).collect())
        .collect();
    let mut pc = vec![0.0; dim];
    let mut y = vec![0.0; dim];
    let mut d = vec![0.0; dim];
    let mut u = vec![NEG; md.p * md.t];
    let mut lay = vec![0usize; md.p];
    let mut band = vec![0usize; md.p];
    let mut best = usize::MAX;
    let mut it = 0u64;
    let mut solved = None;
    let mut last_mm = usize::MAX;
    // ring of the last HW layout hashes, for cycle detection on censored runs
    const HW: usize = 2000;
    let mut hist = vec![0u64; HW];
    while it < cap {
        it += 1;
        // concur
        for i in 0..dim {
            let mut s = 0.0;
            for r in 0..nrep {
                s += x[r][i];
            }
            pc[i] = s / nrep as f64;
        }
        md.proj_b(&mut pc);
        // divide on 2 pc - x
        for r in 0..nrep {
            for i in 0..dim {
                y[i] = 2.0 * pc[i] - x[r][i];
            }
            md.unary(&y, &mut u);
            if r == 0 {
                md.proj_a(&u, &mut lay);
                md.embed(&lay, &mut d);
            } else {
                md.proj_bands(r - 1, &u, &mut band);
                md.embed(&band, &mut d);
            }
            for i in 0..dim {
                x[r][i] += beta * (d[i] - pc[i]);
            }
        }
        let mm = md.mismatches(&lay);
        best = best.min(mm);
        last_mm = mm;
        let mut hsh = 0xcbf29ce484222325u64;
        for &v in &lay {
            hsh = (hsh ^ v as u64).wrapping_mul(0x100000001B3);
        }
        hist[(it as usize) % HW] = hsh;
        if mm == 0 {
            solved = Some(it);
            break;
        }
    }
    // cycle detection: smallest period p <= 50 with layout(i) == layout(i - p)
    // for every i in the last 200 iterations; distinct layouts in the last HW
    let mut period = None;
    let mut distinct = None;
    if solved.is_none() && it as usize >= HW {
        let at = |i: u64| hist[(i as usize) % HW];
        for p in 1..=50u64 {
            if (it - 199..=it).all(|i| at(i) == at(i - p)) {
                period = Some(p);
                break;
            }
        }
        let mut v = hist.clone();
        v.sort_unstable();
        v.dedup();
        distinct = Some(v.len());
    }
    RunOut {
        period,
        distinct,
        last_mm,
        iters: solved,
        best_mm: best,
        board: lay,
        wall_ms: t0.elapsed().as_secs_f64() * 1e3,
        transitions: md.transitions,
        iters_done: it,
    }
}

fn load(path: &str) -> Vec<Inst> {
    let f = std::fs::File::open(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let mut out = Vec::new();
    for line in std::io::BufReader::new(f).lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        let v: Value = serde_json::from_str(&line).unwrap();
        let pz = &v["puzzle"];
        let n = pz["width"].as_u64().unwrap() as usize;
        let pieces: Vec<[u8; 4]> = pz["pieces"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| {
                let a = p.as_array().unwrap();
                [0, 1, 2, 3].map(|i| a[i].as_u64().unwrap() as u8)
            })
            .collect();
        out.push(Inst {
            id: v["id"].as_str().unwrap().to_string(),
            n,
            c: v["c"].as_i64().unwrap_or(-1),
            seed: v["seed"].as_i64().unwrap(),
            pieces,
            planted: v["planted"]
                .as_array()
                .map(|a| a.iter().map(|x| x.as_i64().unwrap()).collect()),
        });
    }
    out
}

fn arg<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
}

fn range(s: &str) -> (i64, i64) {
    let (a, b) = s.split_once('-').unwrap_or((s, s));
    (a.parse().unwrap(), b.parse().unwrap())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) != Some("run") {
        eprintln!("usage: dmb run --instances F --k K --runs 1-3 --cap N [--beta 0.25] [--seeds a-b] [--sizes 6,7] --out F");
        std::process::exit(2);
    }
    let insts = load(arg(&args, "--instances").expect("--instances"));
    let k: usize = arg(&args, "--k").expect("--k").parse().unwrap();
    let (r0, r1) = range(arg(&args, "--runs").unwrap_or("1-1"));
    let cap: u64 = arg(&args, "--cap").expect("--cap").parse().unwrap();
    let beta: f64 = arg(&args, "--beta").unwrap_or("0.25").parse().unwrap();
    let seeds = arg(&args, "--seeds").map(range);
    let sizes: Option<Vec<usize>> =
        arg(&args, "--sizes").map(|s| s.split(',').map(|x| x.parse().unwrap()).collect());
    let out_path = arg(&args, "--out").expect("--out");
    let mut out = BufWriter::new(
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(out_path)
            .unwrap(),
    );
    for inst in &insts {
        if let Some((a, b)) = seeds {
            if inst.seed < a || inst.seed > b {
                continue;
            }
        }
        if let Some(sz) = &sizes {
            if !sz.contains(&inst.n) {
                continue;
            }
        }
        for rs in r0..=r1 {
            let r = run_dm(inst, k, rs as u64, cap, beta);
            let ok = r.iters.is_some();
            let check = if ok {
                verify(inst, &r.board).err()
            } else {
                None
            };
            let board: Vec<i64> = r.board.iter().map(|&v| v as i64).collect();
            let eq = inst.planted.as_ref().map(|p| ok && *p == board);
            let row = json!({
                "arm": format!("dm-k{k}"), "k": k, "instance": inst.id, "n": inst.n, "c": inst.c,
                "seed": inst.seed, "run_seed": rs, "cap": cap, "beta": beta,
                "status": if ok { "solved" } else { "censored" },
                "iterations": r.iters.unwrap_or(cap), "iterations_done": r.iters_done,
                "best_mismatches": r.best_mm, "terminal_mismatches": r.last_mm,
                "cycle_period": r.period, "distinct_layouts_last2000": r.distinct, "wall_ms": r.wall_ms,
                "us_per_iter": r.wall_ms * 1e3 / r.iters_done.max(1) as f64,
                "dp_transitions": r.transitions,
                "verify_error": check, "equals_planted": eq,
                "board": if ok { json!(board) } else { Value::Null },
            });
            writeln!(out, "{row}").unwrap();
            out.flush().unwrap();
            if let Some(e) = check {
                eprintln!("VERIFY FAILED {} k={k} run={rs}: {e}", inst.id);
            }
        }
    }
}
