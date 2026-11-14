//! census run  --ladder L --n N [--seeds a-b] --oracles B,A,E,D,C [--threads T] --machine M --out F.jsonl
//! census gate --ladder L --n N [--seeds a-b] [--threads T] --machine M --out F.jsonl   (planted and solution paths)
//! census fuzz --out F.jsonl                                                            (brute-force checks)
//!
//! See ../PREREG.md. Node rows carry `type: "node"`, instance rows `type: "instance"`.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use dead_node_census::oracles::{self, View};
use dead_node_census::tree::{self, Exit, Hook};
use dead_node_census::{planted_options, prop, Ctx, Rng, State};
use hard_ladder::cli::load_instances;
use hard_ladder::{Ladder, LadderInstance, Role, TEST_CAP};
use sat_backdoors::{Model, Stats};
use serde_json::{json, Value};

const SAMPLE: usize = 200;
const SAMPLE_SEED: u64 = 20_261_112;

fn arg<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
}

// ------------------------------------------------------------------ pass 1

#[derive(Default)]
struct P1 {
    depth: Vec<u8>,
    /// 0 propagation failed, 1 dead, 2 found path, 3 open at cap / pending.
    kind: Vec<u8>,
    size: Vec<u32>,
}
impl Hook for P1 {
    fn node(
        &mut self,
        _: u64,
        depth: usize,
        ok: bool,
        _: &[u64],
        _: usize,
        _: usize,
        _: &[u64],
        _: &[bool],
    ) {
        self.depth.push(depth as u8);
        self.kind.push(if ok { 3 } else { 0 });
        self.size.push(1);
    }
    fn exit(&mut self, id: u64, size: u64, how: Exit) {
        let i = (id - 1) as usize;
        self.size[i] = size as u32;
        self.kind[i] = match how {
            Exit::Dead => 1,
            Exit::Found => 2,
            Exit::Cap => 3,
        };
    }
}

/// Depth-stratified sample (PREREG.md "Sample"): (id, design weight).
fn sample(p1: &P1, seed: u64) -> (Vec<(u64, f64)>, BTreeMap<u8, usize>, BTreeMap<u8, usize>) {
    let mut strata: BTreeMap<u8, Vec<u64>> = BTreeMap::new();
    for (i, &k) in p1.kind.iter().enumerate() {
        if k == 1 {
            strata.entry(p1.depth[i]).or_default().push(i as u64 + 1);
        }
    }
    let counts: BTreeMap<u8, usize> = strata.iter().map(|(&d, v)| (d, v.len())).collect();
    let total: usize = counts.values().sum();
    let mut alloc: BTreeMap<u8, usize> = BTreeMap::new();
    if total <= SAMPLE {
        alloc = counts.clone();
    } else {
        let mut open: Vec<u8> = counts.keys().copied().collect();
        let mut rem = SAMPLE;
        loop {
            let q = rem / open.len();
            let small: Vec<u8> = open.iter().copied().filter(|d| counts[d] <= q).collect();
            if small.is_empty() {
                for &d in &open {
                    alloc.insert(d, q);
                }
                let left = rem - q * open.len();
                for &d in open.iter().rev().take(left) {
                    *alloc.get_mut(&d).expect("open") += 1;
                }
                break;
            }
            for d in small {
                alloc.insert(d, counts[&d]);
                rem -= counts[&d];
                open.retain(|&x| x != d);
            }
            if open.is_empty() {
                break;
            }
        }
    }
    let mut rng = Rng(seed);
    let mut out = Vec::new();
    for (d, ids) in &strata {
        let a = alloc[d];
        let mut v = ids.clone();
        for i in 0..a {
            let j = i + rng.below(v.len() - i);
            v.swap(i, j);
        }
        let w = ids.len() as f64 / a as f64;
        for &id in &v[..a] {
            out.push((id, w));
        }
    }
    out.sort_unstable_by_key(|x| x.0);
    (out, counts, alloc)
}

// ------------------------------------------------------------------ oracles at a state

#[derive(Clone, Copy)]
struct Which {
    b: bool,
    a: bool,
    e: bool,
    d: bool,
    c: bool,
}

fn ns(t: Instant) -> u64 {
    t.elapsed().as_nanos() as u64
}

fn run_oracles(cx: &Ctx, s: &State, wipes: &[u64], which: Which, rng_seed: u64) -> (Value, bool) {
    let v = View::new(cx, s);
    let mut row = serde_json::Map::new();
    let mut any_dead = false;
    if which.b {
        for (name, pool) in [("b_pool", true), ("b_dom", false)] {
            let t = Instant::now();
            let r = oracles::cavity(&v, pool);
            any_dead |= r.r.is_some();
            row.insert(
                name.into(),
                json!({"r": r.r, "work": r.work, "windows": r.windows, "budget_outs": r.budget_outs,
                       "at": r.at, "sig_raw": r.key_raw.map(|k| format!("{k:016x}")),
                       "sig_can": r.key_can.map(|k| format!("{k:016x}")), "ns": ns(t)}),
            );
        }
    }
    if which.a {
        for k in 1..=3usize {
            let t = Instant::now();
            let r = oracles::band(&v, k);
            any_dead |= r.dead;
            row.insert(
                format!("a{k}"),
                json!({"dead": r.dead, "dead_rows": r.dead_rows, "work": r.work, "removed": r.removed,
                       "values": r.values, "ns": ns(t)}),
            );
        }
    }
    if which.e {
        let t = Instant::now();
        let r = oracles::colour_only(&v);
        any_dead |= r.dead;
        row.insert(
            "e".into(),
            json!({"dead": r.dead, "overflow": r.overflow, "work": r.work, "max_states": r.max_states, "ns": ns(t)}),
        );
    }
    if which.d {
        let t = Instant::now();
        let r = oracles::partition(&v, rng_seed);
        let part = |p: &oracles::Part| json!({"key": p.key, "dead": p.dead, "calls": p.calls, "parts": p.parts, "dead_parts": p.dead_parts});
        any_dead |= r.cut.dead || r.rand.dead || r.poac.dead || r.poac_mrv.dead;
        row.insert(
            "d".into(),
            json!({"cut": part(&r.cut), "rand": part(&r.rand), "poac": part(&r.poac), "poac_mrv": part(&r.poac_mrv), "ns": ns(t)}),
        );
    }
    if which.c {
        let t = Instant::now();
        let r = oracles::pocket(&v, wipes);
        any_dead |= r.dead;
        row.insert(
            "pocket".into(),
            json!({"dead": r.dead, "size": r.size, "work": r.work, "overflow": r.overflow,
                   "sig_can": format!("{:016x}", r.key_can), "ns": ns(t)}),
        );
    }
    row.insert("empties".into(), json!(s.empties()));
    (Value::Object(row), any_dead)
}

// ------------------------------------------------------------------ pass 2

struct Snap {
    id: u64,
    depth: usize,
    state: State,
    wipes: Vec<u64>,
}

struct P2<'a> {
    cx: &'a Ctx<'a>,
    weight: HashMap<u64, f64>,
    size: &'a [u32],
    active: Vec<Snap>,
    which: Which,
    seed: u64,
    rows: Vec<Value>,
    prop_mismatch: u64,
}

impl Hook for P2<'_> {
    fn node(
        &mut self,
        id: u64,
        depth: usize,
        ok: bool,
        parent: &[u64],
        c: usize,
        o: usize,
        nd: &[u64],
        assigned: &[bool],
    ) {
        if !ok && self.which.c && !self.active.is_empty() {
            let w = self.cx.m.words;
            let mut x = parent.to_vec();
            x[c * w..(c + 1) * w].iter_mut().for_each(|y| *y = 0);
            x[c * w + (o >> 6)] = 1u64 << (o & 63);
            match prop::propagate(self.cx, &mut x, &[c]) {
                Ok(()) => self.prop_mismatch += 1,
                Err(Some(cell)) => {
                    for s in &mut self.active {
                        s.wipes[cell] += 1;
                    }
                }
                Err(None) => {}
            }
        }
        if ok && self.weight.contains_key(&id) {
            self.active.push(Snap {
                id,
                depth,
                state: State {
                    d: nd.to_vec(),
                    assigned: assigned.to_vec(),
                },
                wipes: vec![0; self.cx.m.cells],
            });
        }
    }
    fn exit(&mut self, id: u64, size: u64, how: Exit) {
        if self.active.last().is_some_and(|s| s.id == id) {
            let s = self.active.pop().expect("snap");
            assert_eq!(how, Exit::Dead, "sampled node {id} is not dead");
            assert_eq!(size, u64::from(self.size[(id - 1) as usize]));
            let (mut row, _) = run_oracles(
                self.cx,
                &s.state,
                &s.wipes,
                self.which,
                self.seed ^ id.wrapping_mul(0x9E37),
            );
            let o = row.as_object_mut().expect("obj");
            o.insert("id".into(), json!(id));
            o.insert("depth".into(), json!(s.depth));
            o.insert("size".into(), json!(size));
            o.insert("w".into(), json!(self.weight[&id]));
            o.insert(
                "used".into(),
                json!(format!("{:x}", s.state.used(self.cx.m))),
            );
            self.rows.push(row);
        }
    }
}

// ------------------------------------------------------------------ pass 3 (B-pool key recurrence)

struct Target {
    id: u64,
    size: u64,
    used: u128,
}

struct P3<'a> {
    cx: &'a Ctx<'a>,
    size: &'a [u32],
    targets: Vec<Target>,
    raw: HashMap<u64, Vec<usize>>,
    raw_pos: Vec<(usize, usize, usize)>,
    can: HashMap<u64, Vec<usize>>,
    can_k: Vec<usize>,
    raw_rec: Vec<u64>,
    raw_sound: Vec<u64>,
    can_rec: Vec<u64>,
    can_sound: Vec<u64>,
    last_can: Vec<u64>,
}

impl P3<'_> {
    fn related(&self, u: u64, us: u64, t: &Target) -> bool {
        (t.id <= u && u < t.id + t.size) || (u <= t.id && t.id < u + us)
    }
}

impl Hook for P3<'_> {
    fn node(
        &mut self,
        id: u64,
        _: usize,
        ok: bool,
        _: &[u64],
        _: usize,
        _: usize,
        nd: &[u64],
        assigned: &[bool],
    ) {
        if !ok {
            return;
        }
        let s = State {
            d: nd.to_vec(),
            assigned: assigned.to_vec(),
        };
        let v = View::new(self.cx, &s);
        let us = u64::from(self.size[(id - 1) as usize]);
        let n = self.cx.n;
        let has_empty = |x0: usize, y0: usize, k: usize| {
            (y0..y0 + k).any(|y| (x0..x0 + k).any(|x| !assigned[y * n + x]))
        };
        for &(k, x0, y0) in &self.raw_pos {
            if !has_empty(x0, y0, k) {
                continue;
            }
            let key = oracles::raw_key(&v, x0, y0, k);
            if let Some(ts) = self.raw.get(&key) {
                for &ti in ts {
                    let t = &self.targets[ti];
                    if t.id == id || self.related(id, us, t) {
                        continue;
                    }
                    self.raw_rec[ti] += 1;
                    if t.used & !v.used == 0 {
                        self.raw_sound[ti] += 1;
                    }
                }
            }
        }
        for &k in &self.can_k {
            for y0 in 0..=n - k {
                for x0 in 0..=n - k {
                    if !has_empty(x0, y0, k) {
                        continue;
                    }
                    let key = oracles::canonical_key(&v, x0, y0, k);
                    if let Some(ts) = self.can.get(&key) {
                        for &ti in ts {
                            let t = &self.targets[ti];
                            if self.last_can[ti] == id || t.id == id || self.related(id, us, t) {
                                continue;
                            }
                            self.last_can[ti] = id;
                            self.can_rec[ti] += 1;
                            if t.used & !v.used == 0 {
                                self.can_sound[ti] += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    fn exit(&mut self, _: u64, _: u64, _: Exit) {}
}

// ------------------------------------------------------------------ one instance

fn parse_which(s: &str) -> Which {
    let has = |c: char| s.split(',').any(|x| x.trim().starts_with(c));
    Which {
        b: has('B'),
        a: has('A'),
        e: has('E'),
        d: has('D'),
        c: has('C'),
    }
}

fn status(how: Exit) -> &'static str {
    match how {
        Exit::Found => "solved",
        Exit::Cap => "censored",
        Exit::Dead => "exhausted",
    }
}

fn census_instance(inst: &LadderInstance, which: Which, machine: &str) -> Vec<Value> {
    let m = Model::new(&inst.puzzle);
    let cx = Ctx::new(&m);
    let t0 = Instant::now();
    let r1 = tree::replay(&m, P1::default(), TEST_CAP);
    let p1_ms = t0.elapsed().as_secs_f64() * 1e3;
    let seed =
        SAMPLE_SEED ^ (u64::from(inst.n) << 40) ^ (u64::from(inst.c) << 32) ^ u64::from(inst.seed);
    let (smp, counts, alloc) = sample(&r1.hook, seed);
    let weight: HashMap<u64, f64> = smp.iter().copied().collect();
    let t1 = Instant::now();
    let r2 = tree::replay(
        &m,
        P2 {
            cx: &cx,
            weight,
            size: &r1.hook.size,
            active: Vec::new(),
            which,
            seed,
            rows: Vec::new(),
            prop_mismatch: 0,
        },
        TEST_CAP,
    );
    let p2_ms = t1.elapsed().as_secs_f64() * 1e3;
    let mut rows = r2.hook.rows;
    let mut p3_hash = None;
    let mut p3_ms = 0.0;
    if which.b {
        // B-pool keys of refuted sampled nodes.
        let mut p3 = P3 {
            cx: &cx,
            size: &r1.hook.size,
            targets: Vec::new(),
            raw: HashMap::new(),
            raw_pos: Vec::new(),
            can: HashMap::new(),
            can_k: Vec::new(),
            raw_rec: Vec::new(),
            raw_sound: Vec::new(),
            can_rec: Vec::new(),
            can_sound: Vec::new(),
            last_can: Vec::new(),
        };
        let mut row_of = Vec::new();
        for (ri, row) in rows.iter().enumerate() {
            let b = &row["b_pool"];
            if b["r"].is_null() {
                continue;
            }
            let at = b["at"].as_array().expect("at");
            let (k, x0, y0) = (
                at[0].as_u64().expect("k") as usize,
                at[1].as_u64().expect("x") as usize,
                at[2].as_u64().expect("y") as usize,
            );
            let kr = u64::from_str_radix(b["sig_raw"].as_str().expect("raw"), 16).expect("hex");
            let kc = u64::from_str_radix(b["sig_can"].as_str().expect("can"), 16).expect("hex");
            let ti = p3.targets.len();
            p3.targets.push(Target {
                id: row["id"].as_u64().expect("id"),
                size: row["size"].as_u64().expect("size"),
                used: u128::from_str_radix(row["used"].as_str().expect("used"), 16).expect("hex"),
            });
            p3.raw.entry(kr).or_default().push(ti);
            p3.can.entry(kc).or_default().push(ti);
            if !p3.raw_pos.contains(&(k, x0, y0)) {
                p3.raw_pos.push((k, x0, y0));
            }
            if !p3.can_k.contains(&k) {
                p3.can_k.push(k);
            }
            row_of.push(ri);
        }
        let nt = p3.targets.len();
        p3.raw_rec = vec![0; nt];
        p3.raw_sound = vec![0; nt];
        p3.can_rec = vec![0; nt];
        p3.can_sound = vec![0; nt];
        p3.last_can = vec![0; nt];
        if nt > 0 {
            let t2 = Instant::now();
            let r3 = tree::replay(&m, p3, TEST_CAP);
            p3_ms = t2.elapsed().as_secs_f64() * 1e3;
            p3_hash = Some(format!("{:016x}", r3.hash));
            let h = r3.hook;
            for (ti, &ri) in row_of.iter().enumerate() {
                let o = rows[ri]["b_pool"].as_object_mut().expect("obj");
                o.insert("raw_rec".into(), json!(h.raw_rec[ti]));
                o.insert("raw_sound".into(), json!(h.raw_sound[ti]));
                o.insert("can_rec".into(), json!(h.can_rec[ti]));
                o.insert("can_sound".into(), json!(h.can_sound[ti]));
            }
        }
    }
    // C canonical recurrence among sampled nodes (keys of refuted pockets).
    if which.c {
        let mut by_key: HashMap<String, Vec<usize>> = HashMap::new();
        for (ri, row) in rows.iter().enumerate() {
            if row["pocket"]["dead"].as_bool() == Some(true) {
                by_key
                    .entry(row["pocket"]["sig_can"].as_str().expect("key").to_string())
                    .or_default()
                    .push(ri);
            }
        }
        let spans: Vec<(u64, u64)> = rows
            .iter()
            .map(|r| {
                (
                    r["id"].as_u64().expect("id"),
                    r["size"].as_u64().expect("size"),
                )
            })
            .collect();
        for ris in by_key.values() {
            for &a in ris {
                let (ia, sa) = spans[a];
                let rec = ris
                    .iter()
                    .filter(|&&b| {
                        let (ib, sb) = spans[b];
                        b != a && !(ia <= ib && ib < ia + sa) && !(ib <= ia && ia < ib + sb)
                    })
                    .count();
                rows[a]["pocket"]
                    .as_object_mut()
                    .expect("obj")
                    .insert("rec_sampled".into(), json!(rec));
            }
        }
    }
    let meta = json!({"ladder": inst.ladder, "n": inst.n, "c": inst.c, "seed": inst.seed, "machine": machine});
    for row in &mut rows {
        let o = row.as_object_mut().expect("obj");
        o.insert("type".into(), json!("node"));
        for (k, v) in meta.as_object().expect("obj") {
            o.insert(k.clone(), v.clone());
        }
    }
    let dead_total: usize = counts.values().sum();
    let prop_fail = r1.hook.kind.iter().filter(|&&k| k == 0).count();
    let mut inst_row = json!({
        "type": "instance", "status": status(r1.how), "nodes": r1.nodes, "propagation": r1.propagation,
        "backtracks": r1.backtracks, "hash": format!("{:016x}", r1.hash),
        "hash_p2": format!("{:016x}", r2.hash), "hash_p3": p3_hash,
        "p1_ms": p1_ms, "p2_ms": p2_ms, "p3_ms": p3_ms, "ns_per_node": p1_ms * 1e6 / r1.nodes.max(1) as f64,
        "dead_total": dead_total, "prop_fail": prop_fail, "sampled": smp.len(),
        "dead_by_depth": counts.iter().map(|(d, c)| (d.to_string(), json!(c))).collect::<serde_json::Map<_, _>>(),
        "sampled_by_depth": alloc.iter().map(|(d, c)| (d.to_string(), json!(c))).collect::<serde_json::Map<_, _>>(),
        "prop_mismatch": r2.hook.prop_mismatch,
    });
    for (k, v) in meta.as_object().expect("obj") {
        inst_row
            .as_object_mut()
            .expect("obj")
            .insert(k.clone(), v.clone());
    }
    rows.insert(0, inst_row);
    rows
}

// ------------------------------------------------------------------ gate

/// Keeps the states of the current path; evaluates them when the solution is found.
struct PathHook<'a> {
    cx: &'a Ctx<'a>,
    stack: Vec<(u64, State)>,
    rows: Vec<Value>,
}
impl Hook for PathHook<'_> {
    fn node(
        &mut self,
        id: u64,
        _: usize,
        ok: bool,
        _: &[u64],
        _: usize,
        _: usize,
        nd: &[u64],
        assigned: &[bool],
    ) {
        if ok {
            self.stack.push((
                id,
                State {
                    d: nd.to_vec(),
                    assigned: assigned.to_vec(),
                },
            ));
        }
    }
    fn exit(&mut self, id: u64, _: u64, how: Exit) {
        let (sid, s) = self.stack.pop().expect("stack");
        assert_eq!(sid, id);
        if how == Exit::Found && s.empties() > 0 {
            let all = Which {
                b: true,
                a: true,
                e: true,
                d: true,
                c: true,
            };
            let (row, dead) = run_oracles(self.cx, &s, &vec![0; self.cx.m.cells], all, id);
            self.rows
                .push(json!({"path": "solution", "id": id, "violation": dead, "oracles": row}));
        }
    }
}

fn gate_instance(inst: &LadderInstance, machine: &str) -> Vec<Value> {
    let m = Model::new(&inst.puzzle);
    let cx = Ctx::new(&m);
    let all = Which {
        b: true,
        a: true,
        e: true,
        d: true,
        c: true,
    };
    let popt = planted_options(&m, &inst.puzzle, &inst.planted);
    let mut rows = Vec::new();
    // Planted path under bb-mrv's cell rule.
    let mut d = m.root().to_vec();
    let mut assigned = vec![false; m.cells];
    let mut st = Stats { propagations: 0 };
    for step in 0..m.cells {
        let c = tree::choose(&m, &d, &assigned);
        let o = popt[c];
        assert!(
            m.dom(&d, c)[o >> 6] >> (o & 63) & 1 == 1,
            "planted option pruned: propagation unsound"
        );
        assigned[c] = true;
        assert!(
            backdoor_branching::assign(&m, &mut d, c, o, &mut st),
            "planted path propagation failed"
        );
        if step + 1 == m.cells {
            break;
        }
        let s = State {
            d: d.clone(),
            assigned: assigned.clone(),
        };
        let (row, dead) = run_oracles(&cx, &s, &vec![0; m.cells], all, step as u64);
        rows.push(json!({"path": "planted", "step": step + 1, "violation": dead, "oracles": row}));
    }
    let r = tree::replay(
        &m,
        PathHook {
            cx: &cx,
            stack: Vec::new(),
            rows: Vec::new(),
        },
        TEST_CAP,
    );
    rows.extend(r.hook.rows);
    for row in &mut rows {
        let o = row.as_object_mut().expect("obj");
        o.insert("ladder".into(), json!(inst.ladder));
        o.insert("n".into(), json!(inst.n));
        o.insert("c".into(), json!(inst.c));
        o.insert("seed".into(), json!(inst.seed));
        o.insert("machine".into(), json!(machine));
    }
    rows
}

// ------------------------------------------------------------------ fuzz

fn fuzz(out: &str) -> Result<(), String> {
    let mut f = fs::File::create(out).map_err(|e| e.to_string())?;
    let mut tally: BTreeMap<&str, (u64, u64)> = BTreeMap::new();
    let mut bump = |k: &'static str, bad: bool| {
        let e = tally.entry(k).or_insert((0, 0));
        e.0 += 1;
        e.1 += u64::from(bad);
    };
    for (n, c, seeds) in [(4u8, 3u8, 1..=20u32), (5, 4, 1..=20)] {
        for seed in seeds {
            let (puzzle, _) = hard_ladder::instance::generate(n, c, seed);
            let m = Model::new(&puzzle);
            if !m.root_ok {
                continue;
            }
            let cx = Ctx::new(&m);
            let mut rng = Rng(0xF022 ^ u64::from(seed) ^ (u64::from(n) << 20));
            for _walk in 0..60 {
                let mut d = m.root().to_vec();
                let mut assigned = vec![false; m.cells];
                let mut st = Stats { propagations: 0 };
                loop {
                    let free: Vec<usize> = (0..m.cells).filter(|&x| !assigned[x]).collect();
                    if free.len() <= 1 {
                        break;
                    }
                    let cell = free[rng.below(free.len())];
                    let opts = backdoor_branching::options(&m, &d, cell);
                    let o = opts[rng.below(opts.len())];
                    // propagator equality on this step
                    let mut a = d.clone();
                    let mut b = d.clone();
                    let w = m.words;
                    for x in [&mut a, &mut b] {
                        x[cell * w..(cell + 1) * w].iter_mut().for_each(|y| *y = 0);
                        x[cell * w + (o >> 6)] = 1u64 << (o & 63);
                    }
                    let ra = m.propagate(&mut a, &[cell], &mut st);
                    let rb = prop::propagate(&cx, &mut b, &[cell]);
                    bump("prop_equal", ra != rb.is_ok() || (ra && a != b));
                    if !ra {
                        break;
                    }
                    d = a;
                    assigned[cell] = true;
                    let s = State {
                        d: d.clone(),
                        assigned: assigned.clone(),
                    };
                    let v = View::new(&cx, &s);
                    // Cavity solver against naive enumeration on 2x2, 2x3, 3x2 windows.
                    let nn = n as usize;
                    for (kw, kh) in [(2usize, 2usize), (3, 2), (2, 3)] {
                        let (x0, y0) = (rng.below(nn - kw + 1), rng.below(nn - kh + 1));
                        let cells: Vec<usize> = (y0..y0 + kh)
                            .flat_map(|y| (x0..x0 + kw).map(move |x| y * nn + x))
                            .filter(|&x| !assigned[x])
                            .collect();
                        if cells.is_empty() {
                            continue;
                        }
                        for pool in [true, false] {
                            let cands: Vec<Vec<u32>> = cells
                                .iter()
                                .map(|&x| {
                                    if pool {
                                        v.pool_cands(x)
                                    } else {
                                        v.dom_cands(x)
                                    }
                                })
                                .collect();
                            let mut t = 0;
                            let ex = oracles::exact(&m, &cells, &cands, u64::MAX, &mut t);
                            let nv = oracles::naive(&m, &cells, &cands, true);
                            bump("cavity_vs_naive", ex != Some(nv));
                        }
                    }
                    for k in 1..=3usize.min(nn) {
                        if n == 4 || k <= 2 {
                            bump(
                                "band_vs_naive",
                                oracles::band(&v, k).dead != oracles::band_naive(&v, k),
                            );
                        }
                    }
                    if n == 4 {
                        bump(
                            "colour_vs_naive",
                            oracles::colour_only(&v).dead != oracles::colour_naive(&v),
                        );
                    }
                    // Every oracle against the complete exact search.
                    let all = Which {
                        b: true,
                        a: true,
                        e: true,
                        d: true,
                        c: true,
                    };
                    let wipes: Vec<u64> = (0..m.cells).map(|_| rng.below(4) as u64).collect();
                    let (row, dead) = run_oracles(&cx, &s, &wipes, all, rng.next());
                    let live = tree::completes(&m, &d, &assigned);
                    bump("oracles_vs_exact", dead && live);
                    if dead && live {
                        writeln!(f, "{}", json!({"violation": "dead but completes", "n": n, "seed": seed, "oracles": row}))
                            .map_err(|e| e.to_string())?;
                    }
                    bump(if live { "states_live" } else { "states_dead" }, false);
                }
            }
        }
    }
    for (k, (cnt, bad)) in &tally {
        writeln!(
            f,
            "{}",
            json!({"check": k, "cases": cnt, "violations": bad})
        )
        .map_err(|e| e.to_string())?;
        eprintln!("{k}: {cnt} cases, {bad} violations");
    }
    Ok(())
}

// ------------------------------------------------------------------ cli

fn instances(args: &[String]) -> Result<Vec<LadderInstance>, String> {
    let ladder =
        Ladder::parse(arg(args, "--ladder").ok_or("--ladder required")?).ok_or("bad --ladder")?;
    let n: u8 = arg(args, "--n")
        .ok_or("--n required")?
        .parse()
        .map_err(|_| "bad --n")?;
    let role = Role::parse(arg(args, "--role").unwrap_or("test")).ok_or("bad --role")?;
    let mut insts = load_instances(ladder, role)?;
    insts.retain(|i| i.n == n);
    if let Some(s) = arg(args, "--seeds") {
        let (a, b) = s.split_once('-').ok_or("--seeds lo-hi")?;
        let (a, b): (u32, u32) = (
            a.parse().map_err(|_| "bad seed")?,
            b.parse().map_err(|_| "bad seed")?,
        );
        insts.retain(|i| (a..=b).contains(&i.seed));
    }
    Ok(insts)
}

fn par<F: Fn(&LadderInstance) -> Vec<Value> + Sync>(
    insts: &[LadderInstance],
    threads: usize,
    out: &str,
    f: F,
) -> Result<(), String> {
    let next = AtomicUsize::new(0);
    let file = Mutex::new(fs::File::create(out).map_err(|e| e.to_string())?);
    // largest seeds first does not matter; keep input order of work items
    std::thread::scope(|sc| {
        for _ in 0..threads {
            sc.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                if i >= insts.len() {
                    break;
                }
                let t = Instant::now();
                let rows = f(&insts[i]);
                let mut fh = file.lock().expect("lock");
                for r in &rows {
                    writeln!(fh, "{r}").expect("write");
                }
                fh.flush().expect("flush");
                eprintln!(
                    "{} done in {:.1} s ({} rows)",
                    insts[i].id,
                    t.elapsed().as_secs_f64(),
                    rows.len()
                );
            });
        }
    });
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let threads: usize = arg(&args, "--threads")
        .unwrap_or("1")
        .parse()
        .expect("--threads");
    let machine = arg(&args, "--machine").unwrap_or("unknown").to_string();
    let res = match args.first().map(String::as_str) {
        Some("run") => instances(&args).and_then(|insts| {
            let which = parse_which(arg(&args, "--oracles").unwrap_or("B,A,E,D,C"));
            par(&insts, threads, arg(&args, "--out").expect("--out"), |i| {
                census_instance(i, which, &machine)
            })
        }),
        Some("gate") => instances(&args).and_then(|insts| {
            par(&insts, threads, arg(&args, "--out").expect("--out"), |i| {
                gate_instance(i, &machine)
            })
        }),
        Some("fuzz") => fuzz(arg(&args, "--out").expect("--out")),
        _ => Err("usage: census run|gate|fuzz (see src/main.rs)".into()),
    };
    if let Err(e) = res {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
