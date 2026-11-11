//! Exact count of frame placements with the corner constraint (agenda A5).
//!
//! M1: transfer matrix over multisets of edge-arc types (meet in the middle).
//! M3: brute-force enumeration of labelled placements (reduced rings only).
//! See PREREG.md for the normalisation and the kill criterion.

use eternity2_engine::{generate_framed, official_puzzle, Puzzle, BORDER};
use std::io::Write;
use std::time::Instant;

pub const PRIMES: [u64; 3] = [
    2305843009213693951,
    2305843009213693921,
    2305843009213693907,
];

// ---------------------------------------------------------------- ring model

/// Directed ring multigraph: one arc (from, to) per border piece, colours
/// remapped to 0..nv in increasing order of engine colour id.
#[derive(Clone, Debug)]
pub struct Ring {
    pub nv: usize,
    pub side: usize,
    pub corners: Vec<(u8, u8)>,
    pub edges: Vec<(u8, u8)>,
    pub engine_colours: Vec<u8>,
}

/// Same convention as research/topics/ring-purity/compute/ring_count.py:
/// edge with gray at z: from p[z+3] to p[z+1]; corner with gray at a, a+1:
/// from p[a+3] to p[a+2] (indices mod 4, URDL, clockwise ring).
pub fn ring_of(p: &Puzzle) -> Ring {
    assert_eq!(p.width, p.height);
    let n = p.width as usize;
    let mut raw_c = Vec::new();
    let mut raw_e = Vec::new();
    for e in &p.pieces {
        let g: Vec<usize> = (0..4).filter(|&i| e[i] == BORDER).collect();
        match g.len() {
            0 => {}
            1 => {
                let z = g[0];
                raw_e.push((e[(z + 3) % 4], e[(z + 1) % 4]));
            }
            2 => {
                let a = if (g[0] + 1) % 4 == g[1] { g[0] } else { g[1] };
                assert_eq!(e[(a + 1) % 4], BORDER);
                raw_c.push((e[(a + 3) % 4], e[(a + 2) % 4]));
            }
            _ => panic!("piece with more than two gray sides"),
        }
    }
    assert_eq!(raw_c.len(), 4);
    assert_eq!(raw_e.len(), 4 * (n - 2));
    let mut cols: Vec<u8> = raw_c
        .iter()
        .chain(raw_e.iter())
        .flat_map(|&(a, b)| [a, b])
        .collect();
    cols.sort_unstable();
    cols.dedup();
    let id = |c: u8| cols.iter().position(|&x| x == c).unwrap() as u8;
    Ring {
        nv: cols.len(),
        side: n - 2,
        corners: raw_c.iter().map(|&(a, b)| (id(a), id(b))).collect(),
        edges: raw_e.iter().map(|&(a, b)| (id(a), id(b))).collect(),
        engine_colours: cols.clone(),
    }
}

/// Edge-arc types with multiplicities and their bit fields in a u64 key.
#[derive(Clone, Debug)]
pub struct Types {
    pub arcs: Vec<(u8, u8)>,
    pub mult: Vec<u64>,
    pub off: Vec<u32>,
    pub mask: Vec<u64>,
    pub full_key: u64,
}

pub fn types_of(r: &Ring) -> Types {
    let mut arcs: Vec<(u8, u8)> = r.edges.clone();
    arcs.sort_unstable();
    arcs.dedup();
    let mult: Vec<u64> = arcs
        .iter()
        .map(|t| r.edges.iter().filter(|e| *e == t).count() as u64)
        .collect();
    let mut off = Vec::new();
    let mut mask = Vec::new();
    let mut bits = 0u32;
    let mut full = 0u64;
    for &m in &mult {
        let w = 64 - m.leading_zeros();
        off.push(bits);
        mask.push((1u64 << w) - 1);
        full |= m << bits;
        bits += w;
    }
    assert!(bits <= 64, "multiset key does not fit in 64 bits");
    Types { arcs, mult, off, mask, full_key: full }
}

// --------------------------------------------------------------- arithmetic

pub trait Num: Copy + Send + Sync + 'static {
    fn zero() -> Self;
    fn one() -> Self;
    fn add(self, o: Self) -> Self;
    fn mul(self, o: Self) -> Self;
    fn is_zero(self) -> bool;
    fn dec(self) -> String;
}

impl Num for u128 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
    fn add(self, o: Self) -> Self {
        self.checked_add(o).expect("u128 overflow")
    }
    fn mul(self, o: Self) -> Self {
        self.checked_mul(o).expect("u128 overflow")
    }
    fn is_zero(self) -> bool {
        self == 0
    }
    fn dec(self) -> String {
        self.to_string()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModP<const P: u64>(pub u64);

impl<const P: u64> Num for ModP<P> {
    fn zero() -> Self {
        ModP(0)
    }
    fn one() -> Self {
        ModP(1)
    }
    fn add(self, o: Self) -> Self {
        let s = self.0 + o.0;
        ModP(if s >= P { s - P } else { s })
    }
    fn mul(self, o: Self) -> Self {
        ModP(((self.0 as u128 * o.0 as u128) % P as u128) as u64)
    }
    fn is_zero(self) -> bool {
        self.0 == 0
    }
    fn dec(self) -> String {
        self.0.to_string()
    }
}

// ------------------------------------------------------- M1 transfer matrix

#[derive(Clone)]
pub struct Sub<V> {
    pub keys: Vec<u64>,
    pub vals: Vec<V>,
}

/// One incoming stream for a target vertex: states at `src` extended by type.
#[derive(Clone, Copy)]
struct Stream {
    src: usize,
    inc: u64,
    off: u32,
    mask: u64,
    m: u64,
}

/// in_streams[w] = arcs ending at w (forward) or starting at w (reversed).
fn streams(t: &Types, nv: usize, reversed: bool) -> Vec<Vec<Stream>> {
    let mut s = vec![Vec::new(); nv];
    for (i, &(a, b)) in t.arcs.iter().enumerate() {
        let (src, dst) = if reversed { (b, a) } else { (a, b) };
        s[dst as usize].push(Stream {
            src: src as usize,
            inc: 1u64 << t.off[i],
            off: t.off[i],
            mask: t.mask[i],
            m: t.mult[i],
        });
    }
    s
}

/// k-way merge of the incoming streams of one target; `emit(key, val)` is
/// called once per distinct key in increasing order.
fn merge_into<V: Num>(layer: &[Sub<V>], ss: &[Stream], mut emit: impl FnMut(u64, V)) {
    let k = ss.len();
    let mut pos = vec![0usize; k];
    let valid = |j: usize, p: usize| -> bool {
        let key = layer[ss[j].src].keys[p];
        (key >> ss[j].off) & ss[j].mask < ss[j].m
    };
    let len: Vec<usize> = ss.iter().map(|s| layer[s.src].keys.len()).collect();
    for j in 0..k {
        while pos[j] < len[j] && !valid(j, pos[j]) {
            pos[j] += 1;
        }
    }
    loop {
        let mut best = u64::MAX;
        let mut any = false;
        for j in 0..k {
            if pos[j] < len[j] {
                let c = layer[ss[j].src].keys[pos[j]] + ss[j].inc;
                if !any || c < best {
                    best = c;
                    any = true;
                }
            }
        }
        if !any {
            return;
        }
        let mut acc = V::zero();
        for j in 0..k {
            if pos[j] < len[j] && layer[ss[j].src].keys[pos[j]] + ss[j].inc == best {
                acc = acc.add(layer[ss[j].src].vals[pos[j]]);
                pos[j] += 1;
                while pos[j] < len[j] && !valid(j, pos[j]) {
                    pos[j] += 1;
                }
            }
        }
        if !acc.is_zero() {
            emit(best, acc);
        }
    }
}

fn step<V: Num>(layer: &[Sub<V>], ins: &[Vec<Stream>], par: bool) -> Vec<Sub<V>> {
    let one = |w: usize| -> Sub<V> {
        let mut n = 0usize;
        merge_into(layer, &ins[w], |_, _| n += 1);
        let mut out = Sub { keys: Vec::with_capacity(n), vals: Vec::with_capacity(n) };
        merge_into(layer, &ins[w], |k, v| {
            out.keys.push(k);
            out.vals.push(v);
        });
        out
    };
    if par {
        std::thread::scope(|sc| {
            let hs: Vec<_> = (0..ins.len()).map(|w| sc.spawn(move || one(w))).collect();
            hs.into_iter().map(|h| h.join().unwrap()).collect()
        })
    } else {
        (0..ins.len()).map(one).collect()
    }
}

pub struct Stats {
    pub max_layer: usize,
    pub max_join_side: usize,
}

/// Run `side` steps from a single start vertex; return the states that end
/// at `end`.
#[allow(clippy::too_many_arguments)]
fn walk<V: Num>(
    nv: usize,
    ins: &[Vec<Stream>],
    start: Sub<V>,
    from: usize,
    end: usize,
    side: usize,
    par: bool,
    st: &mut Stats,
) -> Sub<V> {
    let mut layer: Vec<Sub<V>> = (0..nv).map(|_| Sub { keys: vec![], vals: vec![] }).collect();
    layer[from] = start;
    for _ in 0..side {
        layer = step(&layer, ins, par);
        st.max_layer = st.max_layer.max(layer.iter().map(|s| s.keys.len()).sum());
    }
    std::mem::replace(&mut layer[end], Sub { keys: vec![], vals: vec![] })
}

/// W(sigma): number of 4-tuples of side type-words with corner pieces
/// sigma[0..4] on corner cells 0..4, using the edge-type multiset exactly.
pub fn w_sigma<V: Num>(r: &Ring, t: &Types, sigma: [usize; 4], par: bool, st: &mut Stats) -> V {
    let c: Vec<(usize, usize)> =
        sigma.iter().map(|&i| (r.corners[i].0 as usize, r.corners[i].1 as usize)).collect();
    let (tail, head) = (|j: usize| c[j].0, |j: usize| c[j].1);
    let fw = streams(t, r.nv, false);
    let bw = streams(t, r.nv, true);
    let unit = || Sub { keys: vec![0u64], vals: vec![V::one()] };
    let f0 = walk(r.nv, &fw, unit(), head(0), tail(1), r.side, par, st);
    let f = walk(r.nv, &fw, f0, head(1), tail(2), r.side, par, st);
    let b0 = walk(r.nv, &bw, unit(), tail(0), head(3), r.side, par, st);
    let b = walk(r.nv, &bw, b0, tail(3), head(2), r.side, par, st);
    st.max_join_side = st.max_join_side.max(f.keys.len()).max(b.keys.len());
    // F ascending in U  <=>  complement M-U descending (no borrows: U <= M fieldwise).
    let mut acc = V::zero();
    let mut j = 0usize;
    for i in (0..f.keys.len()).rev() {
        let want = t.full_key - f.keys[i];
        while j < b.keys.len() && b.keys[j] < want {
            j += 1;
        }
        if j < b.keys.len() && b.keys[j] == want {
            acc = acc.add(f.vals[i].mul(b.vals[j]));
        }
    }
    acc
}

pub fn perms4() -> Vec<[usize; 4]> {
    let mut v = Vec::new();
    for a in 0..4 {
        for b in 0..4 {
            for c in 0..4 {
                for d in 0..4 {
                    let s = [a, b, c, d];
                    if (0..4).all(|i| (0..4).filter(|&j| s[j] == i).count() == 1) {
                        v.push(s);
                    }
                }
            }
        }
    }
    v
}

pub fn prod_fact(t: &Types) -> u128 {
    t.mult.iter().map(|&m| (1..=m as u128).product::<u128>()).product()
}

// ------------------------------------------------------------ M3 brute force

pub struct Brute {
    corners: Vec<(u8, u8)>,
    edges_by_tail: Vec<Vec<(usize, u8)>>,
    corners_by_tail: Vec<Vec<(usize, u8)>>,
    side: usize,
    cells: usize,
    pub nodes: u64,
    cap: u64,
    pub count: u128,
    pub aborted: bool,
}

impl Brute {
    pub fn new(r: &Ring, cap: u64) -> Self {
        let mut eb = vec![Vec::new(); r.nv];
        for (i, &(a, b)) in r.edges.iter().enumerate() {
            eb[a as usize].push((i, b));
        }
        let mut cb = vec![Vec::new(); r.nv];
        for (i, &(a, b)) in r.corners.iter().enumerate() {
            cb[a as usize].push((i, b));
        }
        Brute {
            corners: r.corners.clone(),
            edges_by_tail: eb,
            corners_by_tail: cb,
            side: r.side,
            cells: 4 * (r.side + 1),
            nodes: 0,
            cap,
            count: 0,
            aborted: false,
        }
    }

    /// Cell 0 is a corner cell; cells i with i % (side+1) == 0 are corners.
    pub fn run(&mut self) {
        for c in 0..self.corners.len() {
            let (a, b) = self.corners[c];
            self.nodes += 1;
            self.dfs(1, b, a, 1u64 << c, 0);
            if self.aborted {
                return;
            }
        }
    }

    fn dfs(&mut self, cell: usize, cur: u8, start: u8, uc: u64, ue: u64) {
        if self.aborted {
            return;
        }
        if cell == self.cells {
            if cur == start {
                self.count += 1;
            }
            return;
        }
        let corner = cell.is_multiple_of(self.side + 1);
        let used = if corner { uc } else { ue };
        let len = if corner {
            self.corners_by_tail[cur as usize].len()
        } else {
            self.edges_by_tail[cur as usize].len()
        };
        for k in 0..len {
            let (i, h) = if corner {
                self.corners_by_tail[cur as usize][k]
            } else {
                self.edges_by_tail[cur as usize][k]
            };
            if used & (1u64 << i) != 0 {
                continue;
            }
            self.nodes += 1;
            if self.nodes > self.cap {
                self.aborted = true;
                return;
            }
            if corner {
                self.dfs(cell + 1, h, start, uc | 1u64 << i, ue);
            } else {
                self.dfs(cell + 1, h, start, uc, ue | 1u64 << i);
            }
        }
    }
}

// ---------------------------------------------------------------- reporting

fn arcs_json(v: &[(u8, u8)]) -> String {
    let s: Vec<String> = v.iter().map(|(a, b)| format!("[{a},{b}]")).collect();
    format!("[{}]", s.join(","))
}

fn ring_json(r: &Ring, t: &Types) -> String {
    format!(
        "\"nv\":{},\"side\":{},\"engine_colours\":{:?},\"corners\":{},\"edges\":{},\"types\":{},\"mult\":{:?}",
        r.nv,
        r.side,
        r.engine_colours,
        arcs_json(&r.corners),
        arcs_json(&r.edges),
        arcs_json(&t.arcs),
        t.mult
    )
}

fn w_residues<const P: u64>(r: &Ring, t: &Types, sig: &[[usize; 4]], par: bool, st: &mut Stats) -> Vec<u64> {
    sig.iter().map(|&s| w_sigma::<ModP<P>>(r, t, s, par, st).0).collect()
}

/// Reduced instance: M1 on all 24 assignments (exact u128) and residues of the
/// 6-way cyclic sum, then M3 brute force.
fn reduced(n: u8, colors: u8, seed: u32, cap: u64) -> String {
    let p = generate_framed(n, colors, seed, true);
    let r = ring_of(&p);
    let t = types_of(&r);
    let mut st = Stats { max_layer: 0, max_join_side: 0 };
    let t0 = Instant::now();
    let ps = perms4();
    let w: Vec<u128> = ps.iter().map(|&s| w_sigma::<u128>(&r, &t, s, false, &mut st)).collect();
    let pf = prod_fact(&t);
    let w24: u128 = w.iter().sum();
    let w6: u128 = ps.iter().zip(&w).filter(|(s, _)| s[0] == 0).map(|(_, x)| *x).sum();
    let n24 = pf.checked_mul(w24).expect("overflow");
    let n6x4 = pf.checked_mul(w6).and_then(|x| x.checked_mul(4)).expect("overflow");
    let cyc: Vec<[usize; 4]> = ps.iter().copied().filter(|s| s[0] == 0).collect();
    let r1: u64 = w_residues::<{ PRIMES[0] }>(&r, &t, &cyc, false, &mut st).iter().fold(0u128, |a, &x| (a + x as u128) % PRIMES[0] as u128) as u64;
    let r2: u64 = w_residues::<{ PRIMES[1] }>(&r, &t, &cyc, false, &mut st).iter().fold(0u128, |a, &x| (a + x as u128) % PRIMES[1] as u128) as u64;
    let r3: u64 = w_residues::<{ PRIMES[2] }>(&r, &t, &cyc, false, &mut st).iter().fold(0u128, |a, &x| (a + x as u128) % PRIMES[2] as u128) as u64;
    let m1_s = t0.elapsed().as_secs_f64();
    let t1 = Instant::now();
    let mut b = Brute::new(&r, cap);
    b.run();
    let m3_s = t1.elapsed().as_secs_f64();
    let m3 = if b.aborted { "null".to_string() } else { format!("\"{}\"", b.count) };
    let ws: Vec<String> = w.iter().map(|x| format!("\"{x}\"")).collect();
    let sg: Vec<String> = ps.iter().map(|s| format!("{s:?}")).collect();
    format!(
        "{{\"n\":{n},\"colors\":{colors},\"seed\":{seed},{},\"prod_mult_fact\":\"{pf}\",\"sigmas\":[{}],\"W_sigma\":[{}],\"N_M1\":\"{n24}\",\"N_M1_cyc6x4\":\"{n6x4}\",\"W6_residues\":[{r1},{r2},{r3}],\"N_M3\":{m3},\"M3_nodes\":{},\"M3_capped\":{},\"M3_cap\":{cap},\"max_layer_states\":{},\"max_join_states\":{},\"M1_seconds\":{m1_s:.4},\"M3_seconds\":{m3_s:.4}}}",
        ring_json(&r, &t),
        sg.join(","),
        ws.join(","),
        b.nodes,
        b.aborted,
        st.max_layer,
        st.max_join_side
    )
}

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).map(|i| args[i + 1].clone())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("");
    match cmd {
        "reduced" => {
            // reduced --cells 5x3,5x4 --seeds 1,2,3 --cap N --threads T --out file
            let cells: Vec<(u8, u8)> = arg(&args, "--cells")
                .unwrap()
                .split(',')
                .map(|c| {
                    let (a, b) = c.split_once('x').unwrap();
                    (a.parse().unwrap(), b.parse().unwrap())
                })
                .collect();
            let seeds: Vec<u32> =
                arg(&args, "--seeds").unwrap().split(',').map(|s| s.parse().unwrap()).collect();
            let cap: u64 = arg(&args, "--cap").map(|s| s.parse::<f64>().unwrap() as u64).unwrap();
            let threads: usize = arg(&args, "--threads").map(|s| s.parse().unwrap()).unwrap_or(8);
            let out = arg(&args, "--out").unwrap();
            let jobs: Vec<(u8, u8, u32)> =
                cells.iter().flat_map(|&(n, c)| seeds.iter().map(move |&s| (n, c, s))).collect();
            let next = std::sync::atomic::AtomicUsize::new(0);
            let res = std::sync::Mutex::new(vec![String::new(); jobs.len()]);
            std::thread::scope(|sc| {
                for _ in 0..threads {
                    sc.spawn(|| loop {
                        let i = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        if i >= jobs.len() {
                            break;
                        }
                        let (n, c, s) = jobs[i];
                        let line = reduced(n, c, s, cap);
                        eprintln!("done {n}x{n} colors={c} seed={s}");
                        res.lock().unwrap()[i] = line;
                    });
                }
            });
            let mut f = std::fs::File::create(out).unwrap();
            for l in res.into_inner().unwrap() {
                writeln!(f, "{l}").unwrap();
            }
        }
        "official" => {
            // official --prime-index i  (6 assignments with corner piece 0 on cell 0)
            let pi: usize = arg(&args, "--prime-index").unwrap().parse().unwrap();
            let r = ring_of(&official_puzzle());
            let t = types_of(&r);
            let cyc: Vec<[usize; 4]> = perms4().into_iter().filter(|s| s[0] == 0).collect();
            let mut st = Stats { max_layer: 0, max_join_side: 0 };
            let t0 = Instant::now();
            let mut ws = Vec::new();
            let mut secs = Vec::new();
            for s in &cyc {
                let ts = Instant::now();
                let one = [*s];
                let v = match pi {
                    0 => w_residues::<{ PRIMES[0] }>(&r, &t, &one, true, &mut st),
                    1 => w_residues::<{ PRIMES[1] }>(&r, &t, &one, true, &mut st),
                    2 => w_residues::<{ PRIMES[2] }>(&r, &t, &one, true, &mut st),
                    _ => panic!("prime index 0..3"),
                };
                ws.push(v[0]);
                secs.push(ts.elapsed().as_secs_f64());
                eprintln!("sigma {s:?}: W mod p = {} ({:.1}s, max layer {})", v[0], secs[secs.len() - 1], st.max_layer);
            }
            let sg: Vec<String> = cyc.iter().map(|s| format!("{s:?}")).collect();
            println!(
                "{{\"prime_index\":{pi},\"prime\":{},{},\"prod_mult_fact\":\"{}\",\"sigmas\":[{}],\"W_sigma_mod_p\":{:?},\"sigma_seconds\":{:?},\"max_layer_states\":{},\"max_join_states\":{},\"seconds\":{:.2}}}",
                PRIMES[pi],
                ring_json(&r, &t),
                prod_fact_dec(&t),
                sg.join(","),
                ws,
                secs.iter().map(|x| (x * 100.0).round() / 100.0).collect::<Vec<_>>(),
                st.max_layer,
                st.max_join_side,
                t0.elapsed().as_secs_f64()
            );
        }
        _ => {
            eprintln!("usage: frame_count reduced --cells 5x3,... --seeds 1,2 --cap 2e10 --out f.jsonl | official --prime-index 0|1|2");
            std::process::exit(2);
        }
    }
}

/// prod m! as a decimal string (may exceed u128 on the official ring).
fn prod_fact_dec(t: &Types) -> String {
    let mut digits: Vec<u32> = vec![1]; // little-endian base 10^9
    for &m in &t.mult {
        for k in 2..=m {
            let mut carry = 0u64;
            for d in digits.iter_mut() {
                let x = *d as u64 * k + carry;
                *d = (x % 1_000_000_000) as u32;
                carry = x / 1_000_000_000;
            }
            while carry > 0 {
                digits.push((carry % 1_000_000_000) as u32);
                carry /= 1_000_000_000;
            }
        }
    }
    let mut s = digits.last().unwrap().to_string();
    for d in digits.iter().rev().skip(1) {
        s.push_str(&format!("{d:09}"));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tiny() -> Ring {
        // 2 colours, side 2: corners and edges chosen so several frames exist.
        Ring {
            nv: 2,
            side: 2,
            corners: vec![(0, 1), (1, 0), (0, 0), (1, 1)],
            edges: vec![(0, 1), (1, 0), (0, 0), (1, 1), (0, 1), (1, 0), (1, 1), (0, 0)],
            engine_colours: vec![1, 2],
        }
    }

    #[test]
    fn m1_matches_brute_on_tiny_ring() {
        let r = tiny();
        let t = types_of(&r);
        let mut st = Stats { max_layer: 0, max_join_side: 0 };
        let w: u128 = perms4().iter().map(|&s| w_sigma::<u128>(&r, &t, s, false, &mut st)).sum();
        let mut b = Brute::new(&r, u64::MAX);
        b.run();
        assert!(b.count > 0);
        assert_eq!(w * prod_fact(&t), b.count);
    }

    #[test]
    fn m1_matches_brute_on_generated_rings() {
        for (n, c, s) in [(5u8, 3u8, 1u32), (5, 4, 2), (6, 4, 3)] {
            let r = ring_of(&generate_framed(n, c, s, true));
            let t = types_of(&r);
            let mut st = Stats { max_layer: 0, max_join_side: 0 };
            let w: u128 = perms4().iter().map(|&x| w_sigma::<u128>(&r, &t, x, false, &mut st)).sum();
            let mut b = Brute::new(&r, u64::MAX);
            b.run();
            assert_eq!(w * prod_fact(&t), b.count, "{n}x{n} c{c} s{s}");
        }
    }

    #[test]
    fn residues_match_exact() {
        let r = ring_of(&generate_framed(6, 4, 1, true));
        let t = types_of(&r);
        let mut st = Stats { max_layer: 0, max_join_side: 0 };
        for s in perms4() {
            let e = w_sigma::<u128>(&r, &t, s, false, &mut st);
            let m = w_sigma::<ModP<{ PRIMES[1] }>>(&r, &t, s, true, &mut st).0;
            assert_eq!((e % PRIMES[1] as u128) as u64, m);
        }
    }

    #[test]
    fn official_ring_shape() {
        let r = ring_of(&official_puzzle());
        assert_eq!((r.nv, r.side, r.corners.len(), r.edges.len()), (5, 14, 4, 56));
        let t = types_of(&r);
        assert_eq!(t.mult.iter().sum::<u64>(), 56);
        assert_eq!(prod_fact_dec(&Types { arcs: vec![], mult: vec![5, 3], off: vec![], mask: vec![], full_key: 0 }), "720");
    }
}
