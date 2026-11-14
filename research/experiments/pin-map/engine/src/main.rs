//! pinmap: exact search-tree sizes of small planted boards with pinned cells
//! (see ../PREREG.md). `puzzle.rs` and `tree.rs` are copied from
//! research/experiments/orders-and-pins/engine/src at commit ba0b77b1 (Track
//! D3's branch); `tree.rs` gains `count_fast` and `count_multi`. The bb-mrv search below is
//! the `Rule::Mrv` DFS of research/experiments/backdoor-branching/compute/src/
//! lib.rs (A3 propagation from sat-backdoors, forced cell first, then min
//! (|D|, index), options in index order), changed to run to exhaustion.
//!
//! Subcommands:
//!   gen    --n --cf --ci --seed --out    planted board with distinct tiles
//!   rs     --board --order fwd|rev [--cap] [--verify]
//!   mrv    --board [--cap]
//!   multi  --board --order fwd|rev [--cap]   (see `multi`)
//! `rs` and `mrv` read one job per stdin line, `label<TAB>cells` (cells a
//! comma list of pinned positions pos = row*n+col, or `-` for none), and print
//! one JSON line per job.

mod puzzle;
mod tree;

use puzzle::{generate, rotate, Edges, Puzzle};
use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::time::Instant;
use tree::Tree;

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

fn num(m: &HashMap<String, String>, k: &str, d: f64) -> f64 {
    m.get(k).map(|v| v.parse::<f64>().expect("number")).unwrap_or(d)
}

struct Board {
    n: usize,
    pieces: Vec<Edges>,
    /// planted[pos] = (pid, rot)
    planted: Vec<(u16, u8)>,
}

/// Board file (same format as orders-and-pins): line 1 `n`; n*n lines
/// `t r b l`; n*n lines `pid rot` by cell.
fn load_board(path: &str) -> Board {
    let raw = std::fs::read_to_string(path).expect("read board");
    let mut it = raw.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#'));
    let n: usize = it.next().unwrap().trim().parse().unwrap();
    let mut rd = |k: usize| -> Vec<Vec<usize>> {
        (0..k)
            .map(|_| it.next().unwrap().split_whitespace().map(|t| t.parse().unwrap()).collect())
            .collect()
    };
    let pieces = rd(n * n).iter().map(|v| [v[0] as u8, v[1] as u8, v[2] as u8, v[3] as u8]).collect();
    let planted = rd(n * n).iter().map(|v| (v[0] as u16, v[1] as u8)).collect();
    let b = Board { n, pieces, planted };
    verify(&b);
    b
}

fn verify(b: &Board) {
    let n = b.n;
    let e: Vec<Edges> = b.planted.iter().map(|&(p, r)| rotate(b.pieces[p as usize], r)).collect();
    let mut used = vec![false; n * n];
    for (pos, &(pid, _)) in b.planted.iter().enumerate() {
        assert!(!used[pid as usize], "piece used twice");
        used[pid as usize] = true;
        let (r, c) = (pos / n, pos % n);
        assert_eq!(e[pos][0] == 0, r == 0);
        assert_eq!(e[pos][2] == 0, r == n - 1);
        assert_eq!(e[pos][3] == 0, c == 0);
        assert_eq!(e[pos][1] == 0, c == n - 1);
        if c + 1 < n {
            assert_eq!(e[pos][1], e[pos + 1][3]);
        }
        if r + 1 < n {
            assert_eq!(e[pos][2], e[pos + n][0]);
        }
    }
}

fn canon(e: Edges) -> Edges {
    (0..4).map(|r| rotate(e, r)).min().unwrap()
}

fn gen(m: &HashMap<String, String>) {
    let n = num(m, "n", 8.0) as usize;
    let cf = num(m, "cf", 4.0) as u8;
    let ci = num(m, "ci", 8.0) as u8;
    let seed = num(m, "seed", 1.0) as u64;
    let out = m.get("out").expect("--out");
    // attempt a uses generator seed seed*1000+a; the first board whose tiles
    // are pairwise distinct up to rotation is kept.
    for a in 0..1000u64 {
        let (p, planted) = generate(n, cf, ci, seed * 1000 + a);
        let mut cs: Vec<Edges> = p.pieces.iter().map(|&e| canon(e)).collect();
        cs.sort_unstable();
        cs.dedup();
        if cs.len() < n * n {
            continue;
        }
        let mut s = format!("# pin-map gen n={n} cf={cf} ci={ci} seed={seed} attempt={a}\n{n}\n");
        for e in &p.pieces {
            s += &format!("{} {} {} {}\n", e[0], e[1], e[2], e[3]);
        }
        let mut by_pos = vec![(0u16, 0u8); n * n];
        for &(pos, pid, r) in &planted {
            by_pos[pos] = (pid, r);
        }
        for (pid, r) in by_pos {
            s += &format!("{pid} {r}\n");
        }
        std::fs::write(out, s).expect("write board");
        verify(&load_board(out));
        println!("{{\"seed\":{seed},\"attempt\":{a},\"path\":\"{out}\"}}");
        return;
    }
    panic!("no distinct-tile board in 1000 attempts");
}

fn parse_cells(s: &str) -> Vec<usize> {
    if s.trim() == "-" || s.trim().is_empty() {
        return vec![];
    }
    s.split(',').map(|t| t.trim().parse().unwrap()).collect()
}

fn jobs() -> Vec<(String, Vec<usize>)> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let mut it = l.splitn(2, '\t');
            let label = it.next().unwrap().to_string();
            (label, parse_cells(it.next().unwrap_or("-")))
        })
        .collect()
}

fn order(n: usize, kind: &str) -> Vec<usize> {
    let fwd: Vec<usize> = (0..n * n).collect();
    match kind {
        "fwd" => fwd,
        "rev" => fwd.into_iter().rev().collect(),
        _ => panic!("--order fwd|rev"),
    }
}

fn rs(m: &HashMap<String, String>) {
    let b = load_board(m.get("board").expect("--board"));
    let kind = m.get("order").map(String::as_str).unwrap_or("fwd");
    let cap = num(m, "cap", 2e10) as u64;
    let check = m.contains_key("verify");
    let p = Puzzle { n: b.n, pieces: b.pieces.clone(), hints: vec![] };
    let full = order(b.n, kind);
    let out = std::io::stdout();
    for (label, cells) in jobs() {
        let fixed: Vec<(usize, u16, u8)> = cells.iter().map(|&q| (q, b.planted[q].0, b.planted[q].1)).collect();
        let ord: Vec<usize> = full.iter().copied().filter(|q| !cells.contains(q)).collect();
        let t = Tree::new(&p, &ord, &fixed);
        let t0 = Instant::now();
        let (total, sols, first, complete) = t.count_fast(cap);
        let secs = t0.elapsed().as_secs_f64();
        if check {
            let (w, c2, f2) = t.enumerate_first(cap);
            let tot2: u64 = w[1..].iter().sum();
            assert!(c2 == complete && f2 == first && (!complete || (tot2 == total && w[t.nfree] == sols)));
        }
        let mut o = out.lock();
        writeln!(
            o,
            "{{\"label\":\"{label}\",\"solver\":\"rs-{kind}\",\"pins\":{:?},\"complete\":{complete},\"nodes\":{total},\"solutions\":{sols},\"first\":{},\"seconds\":{secs:.3}}}",
            cells,
            first.map(|x| x.to_string()).unwrap_or("null".into())
        )
        .unwrap();
        o.flush().unwrap();
    }
}

// ---- bb-mrv to exhaustion ----

use eternity2_engine::Puzzle as EPuzzle;
use sat_backdoors::{Model, Stats};

fn options(m: &Model, d: &[u64], c: usize) -> Vec<usize> {
    let mut v = Vec::new();
    for (k, &w) in m.dom(d, c).iter().enumerate() {
        let mut x = w;
        while x != 0 {
            v.push(k * 64 + x.trailing_zeros() as usize);
            x &= x - 1;
        }
    }
    v
}

fn assign(m: &Model, d: &mut [u64], c: usize, o: usize, st: &mut Stats) -> bool {
    let w = m.words;
    d[c * w..(c + 1) * w].iter_mut().for_each(|x| *x = 0);
    d[c * w + (o >> 6)] = 1u64 << (o & 63);
    m.propagate(d, &[c], st)
}

struct Mrv<'a> {
    m: &'a Model,
    st: Stats,
    nodes: u64,
    sols: u64,
    first: Option<u64>,
    cap: u64,
}

impl Mrv<'_> {
    fn choose(&self, d: &[u64], assigned: &[bool]) -> usize {
        let m = self.m;
        let free: Vec<usize> = (0..m.cells).filter(|&c| !assigned[c]).collect();
        if let Some(&c) = free.iter().find(|&&c| m.count(d, c) == 1) {
            return c;
        }
        *free.iter().min_by_key(|&&c| (m.count(d, c), c)).expect("free cell")
    }

    /// false = cap reached
    fn dfs(&mut self, d: &[u64], assigned: &mut Vec<bool>, left: usize) -> bool {
        if left == 0 {
            self.sols += 1;
            if self.first.is_none() {
                self.first = Some(self.nodes);
            }
            return true;
        }
        let c = self.choose(d, assigned);
        let mut nd = d.to_vec();
        for o in options(self.m, d, c) {
            if self.nodes >= self.cap {
                return false;
            }
            self.nodes += 1;
            nd.copy_from_slice(d);
            assigned[c] = true;
            if assign(self.m, &mut nd, c, o, &mut self.st) && !self.dfs(&nd, assigned, left - 1) {
                assigned[c] = false;
                return false;
            }
            assigned[c] = false;
        }
        true
    }
}

fn mrv(m_: &HashMap<String, String>) {
    let b = load_board(m_.get("board").expect("--board"));
    let cap = num(m_, "cap", 1e8) as u64;
    let ep = EPuzzle {
        name: "pin-map".into(),
        width: b.n as u8,
        height: b.n as u8,
        num_colors: b.pieces.iter().flat_map(|e| e.iter().copied()).max().unwrap(),
        pieces: b.pieces.clone(),
        hints: vec![],
    };
    let model = Model::new(&ep);
    assert!(model.root_ok);
    let out = std::io::stdout();
    for (label, cells) in jobs() {
        let t0 = Instant::now();
        let mut d = model.root().to_vec();
        let mut st = Stats { propagations: 0 };
        let mut assigned = vec![false; model.cells];
        let w = model.words;
        for &q in &cells {
            let (pid, r) = b.planted[q];
            let want = rotate(b.pieces[pid as usize], r);
            let (s, e) = model.piece_range[pid as usize];
            let o = (s..e).find(|&o| model.opt_edges[o] == want).expect("planted option");
            d[q * w..(q + 1) * w].iter_mut().for_each(|x| *x = 0);
            d[q * w + (o >> 6)] = 1u64 << (o & 63);
            assigned[q] = true;
        }
        let ok = model.propagate(&mut d, &cells, &mut st);
        assert!(ok, "pins contradict: propagation bug");
        let mut s = Mrv { m: &model, st, nodes: 0, sols: 0, first: None, cap };
        let complete = s.dfs(&d, &mut assigned, model.cells - cells.len());
        let secs = t0.elapsed().as_secs_f64();
        let mut o = out.lock();
        writeln!(
            o,
            "{{\"label\":\"{label}\",\"solver\":\"bb-mrv-exh\",\"pins\":{:?},\"complete\":{complete},\"nodes\":{},\"solutions\":{},\"first\":{},\"propagations\":{},\"seconds\":{secs:.3}}}",
            cells,
            s.nodes,
            s.sols,
            s.first.map(|x| x.to_string()).unwrap_or("null".into()),
            s.st.propagations
        )
        .unwrap();
        o.flush().unwrap();
    }
}


/// One pass per job: job line `label<TAB>base cells<TAB>candidate cells` (`all`
/// = every cell not in base). Prints the base tree and, per candidate p, the
/// exact tree with base + {p} fixed.
fn multi(m: &HashMap<String, String>) {
    let b = load_board(m.get("board").expect("--board"));
    let kind = m.get("order").map(String::as_str).unwrap_or("fwd");
    let cap = num(m, "cap", 2e10) as u64;
    let n = b.n;
    let p = Puzzle { n, pieces: b.pieces.clone(), hints: vec![] };
    let full = order(n, kind);
    let pe: Vec<Edges> = b.planted.iter().map(|&(pid, r)| rotate(b.pieces[pid as usize], r)).collect();
    let mut owner = vec![0usize; n * n];
    for (pos, &(pid, _)) in b.planted.iter().enumerate() {
        owner[pid as usize] = pos;
    }
    let stdin = std::io::stdin();
    let out = std::io::stdout();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        let label = f[0];
        let base = parse_cells(f.get(1).copied().unwrap_or("-"));
        let cand: Vec<usize> = match f.get(2).copied().unwrap_or("all") {
            "all" => (0..n * n).filter(|q| !base.contains(q)).collect(),
            s => parse_cells(s),
        };
        let fixed: Vec<(usize, u16, u8)> = base.iter().map(|&q| (q, b.planted[q].0, b.planted[q].1)).collect();
        let ord: Vec<usize> = full.iter().copied().filter(|q| !base.contains(q)).collect();
        let t = Tree::new(&p, &ord, &fixed);
        let t0 = Instant::now();
        let r = t.count_multi(n, &pe, &owner, &cand, cap);
        let secs = t0.elapsed().as_secs_f64();
        let cnt: Vec<String> = cand.iter().map(|&q| r.counts[q].to_string()).collect();
        let sol: Vec<String> = cand.iter().map(|&q| r.sols[q].to_string()).collect();
        let fst: Vec<String> =
            cand.iter().map(|&q| r.first[q].map(|x| x.to_string()).unwrap_or("null".into())).collect();
        let mut o = out.lock();
        writeln!(
            o,
            "{{\"label\":\"{label}\",\"solver\":\"rs-{kind}\",\"base\":{:?},\"complete\":{},\"base_nodes\":{},\"cand\":{:?},\"nodes\":[{}],\"solutions\":[{}],\"first\":[{}],\"seconds\":{secs:.3}}}",
            base,
            r.complete,
            r.base_nodes,
            cand,
            cnt.join(","),
            sol.join(","),
            fst.join(",")
        )
        .unwrap();
        o.flush().unwrap();
    }
}

fn main() {
    let (cmd, m) = args_map();
    match cmd.as_str() {
        "gen" => gen(&m),
        "rs" => rs(&m),
        "mrv" => mrv(&m),
        "multi" => multi(&m),
        _ => {
            eprintln!("usage: pinmap gen|rs|mrv ...");
            std::process::exit(2);
        }
    }
}
