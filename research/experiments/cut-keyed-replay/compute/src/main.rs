//! replay run   --ladder L --n N [--role test|calibration] [--seeds a-b] [--threads T] --machine M --out F.jsonl
//! replay check --out F.jsonl     (soundness gate 3: exhaustive check on generated 5x5 boards)
//!
//! See ../PREREG.md. One pass over `bb-mrv`'s tree (census copy of the DFS)
//! carries every arm: cut key, five partition-matched random keys (C1),
//! budgeted POAC on the MRV cell (C2) and unbudgeted POAC (C2u).

use std::fs;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use dead_node_census::tree::{self, Exit, Hook};
use dead_node_census::{planted_options, Ctx, Rng};
use hard_ladder::cli::load_instances;
use hard_ladder::{Ladder, LadderInstance, Role, TEST_CAP};
use sat_backdoors::Model;
use serde_json::{json, Value};

const CONTROL_SEEDS: u64 = 5;

fn arg<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
}

// ------------------------------------------------------------ instrumented propagation

/// Work counters: propagate calls and work units (arc revisions + cells per
/// all-different sweep).
#[derive(Default, Clone, Copy)]
struct Wk {
    calls: u64,
    work: u64,
}

/// Copy of A3 `Model::propagate` (via the census `prop.rs`) returning the
/// same verdict and domains, with work counting. Checked equal to the tree's
/// own propagation at every tree node (`prop_mismatch`).
fn prop(cx: &Ctx, d: &mut [u64], changed: &[usize], wk: &mut Wk) -> bool {
    let m = cx.m;
    let w = m.words;
    wk.calls += 1;
    let mut queue: Vec<usize> = changed.to_vec();
    let mut inq = vec![false; m.cells];
    for &c in changed {
        inq[c] = true;
    }
    let mut allowed = vec![0u64; w];
    loop {
        while let Some(b) = queue.pop() {
            inq[b] = false;
            for db in 0..4 {
                let Some(a) = m.neighbour[b][db] else {
                    continue;
                };
                wk.work += 1;
                let da = (db + 2) & 3;
                let mask = offered(cx, d, b, db);
                allowed.iter_mut().for_each(|x| *x = 0);
                for col in 0..m.num_colors {
                    if mask & (1 << col) != 0 {
                        for (x, y) in allowed.iter_mut().zip(&cx.side_mask[da][col]) {
                            *x |= *y;
                        }
                    }
                }
                let mut changed_a = false;
                let mut empty = true;
                for k in 0..w {
                    let old = d[a * w + k];
                    let new = old & allowed[k];
                    if new != old {
                        changed_a = true;
                        d[a * w + k] = new;
                    }
                    if new != 0 {
                        empty = false;
                    }
                }
                if empty {
                    return false;
                }
                if changed_a && !inq[a] {
                    inq[a] = true;
                    queue.push(a);
                }
            }
        }
        wk.work += m.cells as u64;
        let pieces = m.piece_range.len();
        let mut cnt = vec![0u8; pieces];
        let mut where_ = vec![0usize; pieces];
        let mut single: Vec<Option<usize>> = vec![None; m.cells];
        for c in 0..m.cells {
            let mut first: Option<usize> = None;
            let mut multi = false;
            for (pi, &(s, t)) in m.piece_range.iter().enumerate() {
                let pm = &cx.piece_mask[pi];
                let has = ((s >> 6)..=((t - 1) >> 6)).any(|k| d[c * w + k] & pm[k] != 0);
                if has {
                    if cnt[pi] < 2 {
                        cnt[pi] += 1;
                    }
                    where_[pi] = c;
                    match first {
                        None => first = Some(pi),
                        Some(_) => multi = true,
                    }
                }
            }
            if !multi {
                single[c] = first;
            }
        }
        let mut any = false;
        if cnt.contains(&0) {
            return false;
        }
        for (c, sc) in single.iter().enumerate() {
            if let Some(pi) = *sc {
                let (s, t) = m.piece_range[pi];
                let pm = &cx.piece_mask[pi];
                for c2 in 0..m.cells {
                    if c2 == c {
                        continue;
                    }
                    let mut ch = false;
                    for k in (s >> 6)..=((t - 1) >> 6) {
                        if d[c2 * w + k] & pm[k] != 0 {
                            d[c2 * w + k] &= !pm[k];
                            ch = true;
                        }
                    }
                    if ch {
                        if m.dom(d, c2).iter().all(|&x| x == 0) {
                            return false;
                        }
                        any = true;
                        if !inq[c2] {
                            inq[c2] = true;
                            queue.push(c2);
                        }
                    }
                }
            }
        }
        for pi in 0..pieces {
            if cnt[pi] == 1 {
                let c = where_[pi];
                let pm = &cx.piece_mask[pi];
                let mut ch = false;
                for k in 0..w {
                    let old = d[c * w + k];
                    let new = old & pm[k];
                    if new != old {
                        d[c * w + k] = new;
                        ch = true;
                    }
                }
                if ch {
                    if m.dom(d, c).iter().all(|&x| x == 0) {
                        return false;
                    }
                    any = true;
                    if !inq[c] {
                        inq[c] = true;
                        queue.push(c);
                    }
                }
            }
        }
        if !any {
            return true;
        }
    }
}

fn has_opt(m: &Model, d: &[u64], c: usize, o: usize) -> bool {
    m.dom(d, c)[o >> 6] >> (o & 63) & 1 == 1
}

/// Set cell c to {o} (caller checked o is in the domain) and propagate.
fn assign(cx: &Ctx, d: &mut [u64], c: usize, o: usize, wk: &mut Wk) -> bool {
    let w = cx.m.words;
    d[c * w..(c + 1) * w].iter_mut().for_each(|x| *x = 0);
    d[c * w + (o >> 6)] = 1u64 << (o & 63);
    prop(cx, d, &[c], wk)
}

// ------------------------------------------------------------ keys

/// Undecided edge: (cell a, side da toward b, cell b, colour mask).
type Edge = (usize, usize, usize, u32);

fn offered(cx: &Ctx, d: &[u64], c: usize, side: usize) -> u32 {
    let dc = cx.m.dom(d, c);
    let mut mask = 0u32;
    for col in 0..cx.m.num_colors {
        if dc
            .iter()
            .zip(&cx.side_mask[side][col])
            .any(|(a, b)| a & b != 0)
        {
            mask |= 1 << col;
        }
    }
    mask
}

fn undecided(cx: &Ctx, d: &[u64], assigned: &[bool], a: usize, da: usize) -> Option<Edge> {
    let b = cx.m.neighbour[a][da]?;
    if assigned[a] || assigned[b] {
        return None;
    }
    let mask = offered(cx, d, a, da) & offered(cx, d, b, (da + 2) & 3);
    (mask.count_ones() >= 2).then_some((a, da, b, mask))
}

/// Census D-cut rule: returns (e1, e2, line y).
fn cut_key(cx: &Ctx, d: &[u64], assigned: &[bool]) -> Option<(Edge, Edge, usize)> {
    let n = cx.n;
    let y_low = (0..cx.m.cells)
        .filter(|&c| assigned[c])
        .map(|c| c / n)
        .max()
        .unwrap_or(0);
    let mut lines: Vec<usize> = (0..=y_low.min(n - 2)).rev().collect();
    lines.extend(y_low + 1..n - 1);
    for &y in &lines {
        let mut es: Vec<(u32, usize, Edge)> = (0..n)
            .filter_map(|x| {
                undecided(cx, d, assigned, y * n + x, 2).map(|e| (e.3.count_ones(), y * n + x, e))
            })
            .collect();
        if es.len() >= 2 {
            es.sort_by_key(|e| (e.0, e.1));
            return Some((es[0].2, es[1].2, y));
        }
    }
    None
}

#[derive(Clone)]
enum Rule {
    Cut,
    Rand(Rng),
}

/// Key for an arm; `mismatch` counts C1 draws whose product differs from T.
fn take_key(
    cx: &Ctx,
    d: &[u64],
    assigned: &[bool],
    rule: &mut Rule,
    mismatch: &mut u64,
) -> Option<(Edge, Edge)> {
    let (e1, e2, y) = cut_key(cx, d, assigned)?;
    match rule {
        Rule::Cut => Some((e1, e2)),
        Rule::Rand(rng) => {
            let n = cx.n;
            let t = e1.3.count_ones() * e2.3.count_ones();
            let mut all: Vec<Edge> = Vec::new();
            for c in 0..cx.m.cells {
                for da in [1usize, 2] {
                    if da == 2 && c / n == y {
                        continue; // on the cut line
                    }
                    if let Some(e) = undecided(cx, d, assigned, c, da) {
                        all.push(e);
                    }
                }
            }
            if all.len() < 2 {
                return None;
            }
            let mut best = u32::MAX;
            let mut pairs: Vec<(usize, usize)> = Vec::new();
            for i in 0..all.len() {
                for j in i + 1..all.len() {
                    let p = all[i].3.count_ones() * all[j].3.count_ones();
                    let diff = p.abs_diff(t);
                    if diff < best {
                        best = diff;
                        pairs.clear();
                    }
                    if diff == best {
                        pairs.push((i, j));
                    }
                }
            }
            if best != 0 {
                *mismatch += 1;
            }
            let (i, j) = pairs[rng.below(pairs.len())];
            Some((all[i], all[j]))
        }
    }
}

/// Split a state on a key: one partition per colour pair, restricted and propagated.
fn split(cx: &Ctx, d: &[u64], key: (Edge, Edge), wk: &mut Wk, made: &mut u64) -> Vec<Vec<u64>> {
    let m = cx.m;
    let w = m.words;
    let (e1, e2) = key;
    let mut out = Vec::new();
    for c1 in 0..m.num_colors {
        if e1.3 >> c1 & 1 == 0 {
            continue;
        }
        for c2 in 0..m.num_colors {
            if e2.3 >> c2 & 1 == 0 {
                continue;
            }
            *made += 1;
            let mut nd = d.to_vec();
            let mut empty = false;
            for &(cell, side, col) in &[
                (e1.0, e1.1, c1),
                (e1.2, (e1.1 + 2) & 3, c1),
                (e2.0, e2.1, c2),
                (e2.2, (e2.1 + 2) & 3, c2),
            ] {
                for k in 0..w {
                    nd[cell * w + k] &= cx.side_mask[side][col][k];
                }
                if m.dom(&nd, cell).iter().all(|&x| x == 0) {
                    empty = true;
                }
            }
            if !empty && prop(cx, &mut nd, &[e1.0, e1.2, e2.0, e2.2], wk) {
                out.push(nd);
            }
        }
    }
    out
}

fn join(parts: &[Vec<u64>]) -> Vec<u64> {
    let mut u = parts[0].clone();
    for p in &parts[1..] {
        for (a, b) in u.iter_mut().zip(p) {
            *a |= *b;
        }
    }
    u
}

// ------------------------------------------------------------ arms

/// Arm state at one tree node. `parts` empty = dead.
#[derive(Clone)]
struct PState {
    parts: Vec<Vec<u64>>,
    key: Option<(Edge, Edge)>,
}

#[derive(Default, Clone)]
struct Tally {
    surv: u64,
    kill_dead: u64,
    kill_prune: u64,
    wk: Wk,
    ns: u64,
    made: u64,
    rekeys: u64,
    live_sum: u64,
    mismatch: u64,
    mono: u64,
    /// Kill points (tree node ids) for the exhaustive check.
    kills: Vec<(u64, bool)>,
}

struct PArm {
    rule: Rule,
    stack: Vec<Option<PState>>,
    t: Tally,
}

/// Partition-arm transfer at a surviving node. Returns the child state (None = dead).
#[allow(clippy::too_many_arguments)]
fn p_transfer(
    cx: &Ctx,
    st: &PState,
    c: usize,
    o: usize,
    assigned: &[bool],
    rule: &mut Rule,
    t: &mut Tally,
    wk: &mut Wk,
) -> Option<PState> {
    let mut parts = Vec::new();
    for p in &st.parts {
        if has_opt(cx.m, p, c, o) {
            let mut q = p.clone();
            if assign(cx, &mut q, c, o, wk) {
                parts.push(q);
            }
        }
    }
    if parts.is_empty() {
        return None;
    }
    let touches = st
        .key
        .is_some_and(|(a, b)| [a.0, a.2, b.0, b.2].contains(&c));
    if touches || st.key.is_none() {
        let j = join(&parts);
        match take_key(cx, &j, assigned, rule, &mut t.mismatch) {
            Some(k) => {
                t.rekeys += 1;
                let sp = split(cx, &j, k, wk, &mut t.made);
                if sp.is_empty() {
                    return None;
                }
                return Some(PState {
                    parts: sp,
                    key: Some(k),
                });
            }
            None => {
                return Some(PState {
                    parts: vec![j],
                    key: None,
                })
            }
        }
    }
    Some(PState { parts, key: st.key })
}

fn p_root(cx: &Ctx, rule: &mut Rule, t: &mut Tally, wk: &mut Wk) -> Option<PState> {
    let root = cx.m.root().to_vec();
    let assigned = vec![false; cx.m.cells];
    match take_key(cx, &root, &assigned, rule, &mut t.mismatch) {
        Some(k) => {
            t.rekeys += 1;
            let sp = split(cx, &root, k, wk, &mut t.made);
            (!sp.is_empty()).then_some(PState {
                parts: sp,
                key: Some(k),
            })
        }
        None => Some(PState {
            parts: vec![root],
            key: None,
        }),
    }
}

struct QArm {
    budgeted: bool,
    bank: i64,
    stack: Vec<Option<Vec<u64>>>,
    t: Tally,
    poac_runs: u64,
}

/// POAC arm transfer: assign, then POAC on `next` (the cell bb-mrv branches on next).
fn q_transfer(
    cx: &Ctx,
    st: &[u64],
    c: usize,
    o: usize,
    next: Option<usize>,
    budgeted: bool,
    bank: &mut i64,
    runs: &mut u64,
    wk: &mut Wk,
) -> Option<Vec<u64>> {
    let mut q = st.to_vec();
    let before = wk.calls;
    let ok = assign(cx, &mut q, c, o, wk);
    *bank -= (wk.calls - before) as i64;
    if !ok {
        return None;
    }
    poac(cx, q, next, budgeted, bank, runs, wk)
}

fn poac(
    cx: &Ctx,
    q: Vec<u64>,
    next: Option<usize>,
    budgeted: bool,
    bank: &mut i64,
    runs: &mut u64,
    wk: &mut Wk,
) -> Option<Vec<u64>> {
    let Some(nc) = next else { return Some(q) };
    let opts: Vec<usize> = backdoor_branching::options(cx.m, &q, nc);
    if opts.len() <= 1 || (budgeted && *bank < opts.len() as i64) {
        return Some(q);
    }
    *runs += 1;
    let mut acc: Option<Vec<u64>> = None;
    for o in opts {
        let mut r = q.clone();
        let before = wk.calls;
        let ok = assign(cx, &mut r, nc, o, wk);
        *bank -= (wk.calls - before) as i64;
        if ok {
            match acc.as_mut() {
                None => acc = Some(r),
                Some(a) => a.iter_mut().zip(&r).for_each(|(x, y)| *x |= *y),
            }
        }
    }
    acc
}

// ------------------------------------------------------------ the replay hook

struct Rep<'a> {
    cx: &'a Ctx<'a>,
    parms: Vec<PArm>,
    qarms: Vec<QArm>,
    base: Wk,
    base_ns: u64,
    prop_mismatch: u64,
    /// For each arm, alive at the solution node (None until found).
    found_alive: Vec<Option<bool>>,
    /// Collect kill points for the exhaustive check.
    keep_kills: bool,
    states: Vec<(u64, Vec<u64>, Vec<bool>)>,
}

impl Hook for Rep<'_> {
    fn node(
        &mut self,
        id: u64,
        _: usize,
        ok: bool,
        parent: &[u64],
        c: usize,
        o: usize,
        nd: &[u64],
        assigned: &[bool],
    ) {
        let cx = self.cx;
        let m = cx.m;
        // Base: recompute with the instrumented copy, count bb-mrv's work.
        let t0 = Instant::now();
        let mut q = parent.to_vec();
        let ok2 = assign(cx, &mut q, c, o, &mut self.base);
        self.base_ns += t0.elapsed().as_nanos() as u64;
        if ok2 != ok || (ok && q != nd) {
            self.prop_mismatch += 1;
        }
        if self.keep_kills && ok {
            self.states.push((id, nd.to_vec(), assigned.to_vec()));
        }
        let full = assigned.iter().all(|&a| a);
        let next = (ok && !full).then(|| tree::choose(m, nd, assigned));
        // Partition arms.
        let mut cut_calls = 0u64;
        for (ai, arm) in self.parms.iter_mut().enumerate() {
            let t0 = Instant::now();
            let child = match arm.stack.last().expect("stack") {
                None => None,
                Some(ps) => {
                    if ps.parts.iter().any(|p| has_opt(m, p, c, o)) {
                        arm.t.surv += 1;
                        arm.t.live_sum += ps.parts.len() as u64;
                        let mut wk = arm.t.wk;
                        let before = wk.calls;
                        let ch =
                            p_transfer(cx, ps, c, o, assigned, &mut arm.rule, &mut arm.t, &mut wk);
                        if ai == 0 {
                            cut_calls = wk.calls - before;
                        }
                        arm.t.wk = wk;
                        if ch.is_none() {
                            arm.t.kill_dead += 1;
                            arm.t.kills.push((id, ok));
                        } else if !ok {
                            arm.t.mono += 1;
                        }
                        ch
                    } else {
                        arm.t.kill_prune += 1;
                        arm.t.kills.push((id, ok));
                        None
                    }
                }
            };
            arm.t.ns += t0.elapsed().as_nanos() as u64;
            if ok {
                arm.stack.push(child);
            }
        }
        for arm in &mut self.qarms {
            let t0 = Instant::now();
            if arm.budgeted {
                arm.bank += cut_calls as i64;
            }
            let child = match arm.stack.last().expect("stack") {
                None => None,
                Some(s) => {
                    if has_opt(m, s, c, o) {
                        arm.t.surv += 1;
                        arm.t.live_sum += 1;
                        let mut wk = arm.t.wk;
                        let ch = q_transfer(
                            cx,
                            s,
                            c,
                            o,
                            next,
                            arm.budgeted,
                            &mut arm.bank,
                            &mut arm.poac_runs,
                            &mut wk,
                        );
                        arm.t.wk = wk;
                        if ch.is_none() {
                            arm.t.kill_dead += 1;
                            arm.t.kills.push((id, ok));
                        } else if !ok {
                            arm.t.mono += 1;
                        }
                        ch
                    } else {
                        arm.t.kill_prune += 1;
                        arm.t.kills.push((id, ok));
                        None
                    }
                }
            };
            arm.t.ns += t0.elapsed().as_nanos() as u64;
            if ok {
                arm.stack.push(child);
            }
        }
    }

    fn exit(&mut self, _: u64, _: u64, how: Exit) {
        if how == Exit::Found && self.found_alive[0].is_none() {
            let mut v = Vec::new();
            for a in &self.parms {
                v.push(Some(a.stack.last().expect("s").is_some()));
            }
            for a in &self.qarms {
                v.push(Some(a.stack.last().expect("s").is_some()));
            }
            self.found_alive = v;
        }
        for a in &mut self.parms {
            a.stack.pop();
        }
        for a in &mut self.qarms {
            a.stack.pop();
        }
    }
}

fn arm_names() -> Vec<String> {
    let mut v = vec!["cut".to_string()];
    for s in 1..=CONTROL_SEEDS {
        v.push(format!("rand{s}"));
    }
    v.push("poac".into());
    v.push("poacu".into());
    v
}

fn rand_seed(inst_seed: u64, s: u64) -> u64 {
    0x7431_3232_0000_0000 ^ (inst_seed << 8) ^ s
}

fn build_arms(cx: &Ctx, inst_seed: u64) -> (Vec<PArm>, Vec<QArm>) {
    let mut parms = Vec::new();
    let mut rules = vec![Rule::Cut];
    for s in 1..=CONTROL_SEEDS {
        rules.push(Rule::Rand(Rng(rand_seed(inst_seed, s))));
    }
    for mut rule in rules {
        let mut t = Tally::default();
        let mut wk = Wk::default();
        let r = p_root(cx, &mut rule, &mut t, &mut wk);
        t.wk = wk;
        parms.push(PArm {
            rule,
            stack: vec![r],
            t,
        });
    }
    let root_calls = parms[0].t.wk.calls as i64;
    let mut qarms = Vec::new();
    for budgeted in [true, false] {
        let mut t = Tally::default();
        let mut wk = Wk::default();
        let mut bank = if budgeted { root_calls } else { 0 };
        let mut runs = 0;
        let assigned = vec![false; cx.m.cells];
        let next = Some(tree::choose(cx.m, cx.m.root(), &assigned));
        let r = poac(
            cx,
            cx.m.root().to_vec(),
            next,
            budgeted,
            &mut bank,
            &mut runs,
            &mut wk,
        );
        t.wk = wk;
        qarms.push(QArm {
            budgeted,
            bank,
            stack: vec![r],
            t,
            poac_runs: runs,
        });
    }
    (parms, qarms)
}

/// Soundness gate 2: walk the planted path under bb-mrv's cell rule.
fn planted_path(cx: &Ctx, inst: &LadderInstance) -> Vec<bool> {
    let m = cx.m;
    let popt = planted_options(m, &inst.puzzle, &inst.planted);
    let (mut parms, mut qarms) = build_arms(cx, u64::from(inst.seed));
    let mut ok: Vec<bool> = parms
        .iter()
        .map(|a| a.stack[0].is_some())
        .chain(qarms.iter().map(|a| a.stack[0].is_some()))
        .collect();
    let mut d = m.root().to_vec();
    let mut assigned = vec![false; m.cells];
    let mut base = Wk::default();
    for _ in 0..m.cells {
        let c = tree::choose(m, &d, &assigned);
        let o = popt[c];
        assert!(has_opt(m, &d, c, o), "planted option pruned by bb-mrv");
        assigned[c] = true;
        assert!(assign(cx, &mut d, c, o, &mut base), "planted path failed");
        let full = assigned.iter().all(|&a| a);
        let next = (!full).then(|| tree::choose(m, &d, &assigned));
        let mut cut_calls = 0;
        for (ai, a) in parms.iter_mut().enumerate() {
            let st = a.stack.pop().expect("s");
            let ch = st.and_then(|ps| {
                if !ps.parts.iter().any(|p| has_opt(m, p, c, o)) {
                    return None;
                }
                let mut wk = Wk::default();
                let r = p_transfer(cx, &ps, c, o, &assigned, &mut a.rule, &mut a.t, &mut wk);
                if ai == 0 {
                    cut_calls = wk.calls;
                }
                r
            });
            ok[ai] &= ch.is_some();
            a.stack.push(ch);
        }
        let np = parms.len();
        for (qi, a) in qarms.iter_mut().enumerate() {
            if a.budgeted {
                a.bank += cut_calls as i64;
            }
            let st = a.stack.pop().expect("s");
            let mut wk = Wk::default();
            let ch = st.and_then(|s| {
                if !has_opt(m, &s, c, o) {
                    return None;
                }
                q_transfer(
                    cx,
                    &s,
                    c,
                    o,
                    next,
                    a.budgeted,
                    &mut a.bank,
                    &mut a.poac_runs,
                    &mut wk,
                )
            });
            ok[np + qi] &= ch.is_some();
            a.stack.push(ch);
        }
    }
    ok
}

fn status(how: Exit) -> &'static str {
    match how {
        Exit::Found => "solved",
        Exit::Cap => "censored",
        Exit::Dead => "exhausted",
    }
}

fn tally_json(t: &Tally) -> Value {
    json!({
        "surv": t.surv, "kill_dead": t.kill_dead, "kill_prune": t.kill_prune,
        "calls": t.wk.calls, "work": t.wk.work, "ns": t.ns, "made": t.made,
        "rekeys": t.rekeys, "live_sum": t.live_sum, "mismatch": t.mismatch, "mono": t.mono,
    })
}

fn replay_instance(inst: &LadderInstance, machine: &str) -> Vec<Value> {
    let m = Model::new(&inst.puzzle);
    let cx = Ctx::new(&m);
    let t0 = Instant::now();
    let (parms, qarms) = build_arms(&cx, u64::from(inst.seed));
    let na = parms.len() + qarms.len();
    let hook = Rep {
        cx: &cx,
        parms,
        qarms,
        base: Wk::default(),
        base_ns: 0,
        prop_mismatch: 0,
        found_alive: vec![None; na],
        keep_kills: false,
        states: Vec::new(),
    };
    let r = tree::replay(&m, hook, TEST_CAP);
    let wall_ms = t0.elapsed().as_secs_f64() * 1e3;
    let planted = planted_path(&cx, inst);
    let names = arm_names();
    let h = &r.hook;
    let mut arms = serde_json::Map::new();
    let tallies: Vec<(&Tally, Option<(i64, u64)>)> = h
        .parms
        .iter()
        .map(|a| (&a.t, None))
        .chain(h.qarms.iter().map(|a| (&a.t, Some((a.bank, a.poac_runs)))))
        .collect();
    for (i, (t, q)) in tallies.iter().enumerate() {
        let mut v = tally_json(t);
        let o = v.as_object_mut().expect("obj");
        o.insert("found_alive".into(), json!(h.found_alive[i]));
        o.insert("planted_alive".into(), json!(planted[i]));
        if let Some((bank, runs)) = q {
            o.insert("bank_end".into(), json!(bank));
            o.insert("poac_runs".into(), json!(runs));
        }
        arms.insert(names[i].clone(), v);
    }
    vec![json!({
        "type": "instance", "ladder": inst.ladder, "n": inst.n, "c": inst.c, "seed": inst.seed,
        "role": inst.role, "status": status(r.how), "nodes": r.nodes, "propagation": r.propagation,
        "backtracks": r.backtracks, "hash": format!("{:016x}", r.hash),
        "base_calls": h.base.calls, "base_work": h.base.work, "base_ns": h.base_ns,
        "prop_mismatch": h.prop_mismatch, "wall_ms": wall_ms, "machine": machine,
        "arms": Value::Object(arms),
    })]
}

/// Soundness gate 3: every kill point on generated boards checked by exact search.
fn check(out: &str) -> Result<(), String> {
    let mut f = fs::File::create(out).map_err(|e| e.to_string())?;
    let mut total_v = 0u64;
    // PREREG: 5x5 c=4 and c=5 seeds 1..20; amendment 1 adds 5x5 c=6 and
    // 6x6 c=5 seeds 5001..5020 (not ladder seeds) for more kill points.
    let sets: Vec<(u8, u8, u32, u32)> = vec![
        (5, 4, 1, 20),
        (5, 5, 1, 20),
        (5, 6, 5001, 5020),
        (6, 5, 5001, 5020),
    ];
    for (n, c, s0, s1) in sets {
        for seed in s0..=s1 {
            let (puzzle, _) = hard_ladder::instance::generate(n, c, seed);
            let m = Model::new(&puzzle);
            let cx = Ctx::new(&m);
            let (parms, qarms) = build_arms(&cx, u64::from(seed));
            let na = parms.len() + qarms.len();
            let hook = Rep {
                cx: &cx,
                parms,
                qarms,
                base: Wk::default(),
                base_ns: 0,
                prop_mismatch: 0,
                found_alive: vec![None; na],
                keep_kills: true,
                states: Vec::new(),
            };
            let r = tree::replay(&m, hook, u64::MAX);
            let h = &r.hook;
            let states: std::collections::HashMap<u64, (&Vec<u64>, &Vec<bool>)> =
                h.states.iter().map(|(i, d, a)| (*i, (d, a))).collect();
            let names = arm_names();
            let ts: Vec<&Tally> = h
                .parms
                .iter()
                .map(|a| &a.t)
                .chain(h.qarms.iter().map(|a| &a.t))
                .collect();
            for (i, t) in ts.iter().enumerate() {
                let (mut checked, mut viol) = (0u64, 0u64);
                for &(id, ok) in &t.kills {
                    if !ok {
                        continue; // bb-mrv itself refutes this node
                    }
                    let (d, a) = states[&id];
                    checked += 1;
                    if tree::completes(&m, d, a) {
                        viol += 1;
                    }
                }
                total_v += viol;
                let row = json!({"n": n, "c": c, "seed": seed, "arm": names[i],
                    "status": status(r.how), "nodes": r.nodes, "kills": t.kills.len(),
                    "checked": checked, "violations": viol, "found_alive": h.found_alive[i],
                    "prop_mismatch": h.prop_mismatch});
                writeln!(f, "{row}").map_err(|e| e.to_string())?;
            }
        }
    }
    eprintln!("exhaustive check: {total_v} violations");
    Ok(())
}

// ------------------------------------------------------------ cli

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
                eprintln!("{} done in {:.1} s", insts[i].id, t.elapsed().as_secs_f64());
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
        Some("run") => instances(&args).and_then(|mut insts| {
            // Longest trees first: larger n and order by seed is fine; keep input order.
            insts.sort_by_key(|i| i.seed);
            par(&insts, threads, arg(&args, "--out").expect("--out"), |i| {
                replay_instance(i, &machine)
            })
        }),
        Some("check") => check(arg(&args, "--out").expect("--out")),
        _ => Err("usage: replay run|check (see src/main.rs)".into()),
    };
    if let Err(e) = res {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
