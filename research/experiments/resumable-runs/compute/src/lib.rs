//! bb-mrv-fast with progress records, checkpoint and resume (see `../PREREG.md`).
//!
//! [`ResumableSolver`] is a copy of Track 0's `FastSolver`
//! (`mrv-node-cost/compute/src/lib.rs`): `offered`, `presence`, `push`,
//! `propagate` and `choose` are copied verbatim; `dfs` is split into `dfs`
//! and `branch` with three additions, none of which changes which node is
//! visited next:
//!
//! - per-level bookkeeping: the chosen cell and the index of the option in
//!   progress at each level (two stores per node);
//! - an event check before each `try_assign` (one compare per node) that
//!   writes a progress record or a periodic checkpoint;
//! - a resume entry (`replay` then `dfs_resume`) that rebuilds the stack from
//!   a checkpoint and continues with the remaining options of each level.
//!
//! The gate (`../PREREG.md`) compares it node for node, through the FNV-1a
//! node-sequence hash, with the unmodified `FastSolver`.

use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use backdoor_branching::options;
use eternity2_engine::Puzzle;
use hard_ladder::{LadderSolver, NodeCounter, Outcome};
use mrv_node_cost::{HashHook, NoHook, NodeHook};
use sat_backdoors::Model;
use serde::{Deserialize, Serialize};

pub const FORMAT: &str = "e2-bb-mrv-fast-checkpoint";
/// Version 2 (2026-11-12, PREREG_AMENDMENTS.md A1): adds `cap` and the
/// content `digest`, and every load-time consistency check. Version 1 files
/// carry no digest and are refused.
pub const VERSION: u32 = 2;

/// A node hook whose running state can be saved and restored.
pub trait ResumeHook: NodeHook {
    fn get(&self) -> Option<(u64, u64)>;
    fn set(&mut self, h: u64, n: u64);
}

impl ResumeHook for HashHook {
    fn get(&self) -> Option<(u64, u64)> {
        Some((self.h, self.n))
    }
    fn set(&mut self, h: u64, n: u64) {
        self.h = h;
        self.n = n;
    }
}

impl ResumeHook for NoHook {
    fn get(&self) -> Option<(u64, u64)> {
        None
    }
    fn set(&mut self, _: u64, _: u64) {}
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InstanceMeta {
    pub id: String,
    pub ladder: String,
    pub role: String,
    pub n: u8,
    pub c: u8,
    pub seed: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Totals {
    pub nodes: u64,
    pub backtracks: u64,
    pub propagation: u64,
    pub elapsed_s: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Level {
    pub cell: usize,
    /// Every option of the level in search order, as [piece, rotation].
    pub options: Vec<[u16; 2]>,
    /// For every level but the last: the option in progress (counted). For
    /// the last level: the next option to try (not counted).
    pub index: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Checkpoint {
    pub format: String,
    pub version: u32,
    pub solver: String,
    pub instance: InstanceMeta,
    pub model_fnv: String,
    pub segments: u32,
    pub totals: Totals,
    /// Running node-sequence hash (16 hex digits) and hashed node count,
    /// null when the run had no hash hook.
    pub hash: Option<String>,
    pub hashed: Option<u64>,
    pub deepest: usize,
    pub closed_share: f64,
    pub stack: Vec<Level>,
    pub state_fnv: String,
    /// Total node cap of the segment that wrote the checkpoint.
    pub cap: u64,
    /// FNV-1a 64 (16 hex digits) over the compact JSON of this checkpoint
    /// with `digest` set to "". Detects any change of content; it is not a
    /// signature (anyone can recompute it).
    pub digest: String,
}

fn is_hex16(s: &str) -> bool {
    s.len() == 16
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

impl Checkpoint {
    /// Digest of the content: FNV-1a 64 over the compact JSON with an empty
    /// `digest` field.
    pub fn content_digest(&self) -> String {
        let mut c = self.clone();
        c.digest = String::new();
        let text = serde_json::to_string(&c).expect("checkpoint serializes");
        let mut h = FNV0;
        fnv(&mut h, text.as_bytes());
        format!("{h:016x}")
    }

    /// Set `digest` from the content.
    pub fn seal(&mut self) {
        self.digest = self.content_digest();
    }

    /// Parse and verify a checkpoint: format and version first (a version 1
    /// file gets its own message), then the digest, then every check that
    /// needs no puzzle. Replay checks the rest.
    pub fn parse(text: &str) -> Result<Self, String> {
        let v: serde_json::Value =
            serde_json::from_str(text).map_err(|e| format!("not a JSON checkpoint: {e}"))?;
        let format = v.get("format").and_then(|x| x.as_str());
        if format != Some(FORMAT) {
            return Err(format!("format {format:?} is not {FORMAT}"));
        }
        match v.get("version").and_then(serde_json::Value::as_u64) {
            Some(v) if v == u64::from(VERSION) => {}
            Some(1) => {
                return Err(format!(
                    "checkpoint version 1 cannot be resumed: it was written before the \
                     integrity fix and carries no content digest, so an altered index, \
                     total or hash in it cannot be detected (rerun from the start; this \
                     build reads version {VERSION})"
                ))
            }
            other => {
                return Err(format!(
                    "checkpoint version {other:?} not supported (want {VERSION})"
                ))
            }
        }
        let ck: Self = serde_json::from_value(v).map_err(|e| format!("bad checkpoint: {e}"))?;
        let want = ck.content_digest();
        if ck.digest != want {
            return Err(format!(
                "checkpoint digest mismatch (stored {}, content gives {want}): the file was altered or corrupted",
                ck.digest
            ));
        }
        ck.check()?;
        Ok(ck)
    }

    pub fn load(p: &std::path::Path) -> Result<Self, String> {
        let text = fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))?;
        Self::parse(&text).map_err(|e| format!("{}: {e}", p.display()))
    }

    /// Invariants that hold for every checkpoint the solver writes and need
    /// no puzzle. Independent of the digest.
    pub fn check(&self) -> Result<(), String> {
        let t = &self.totals;
        let cells = usize::from(self.instance.n) * usize::from(self.instance.n);
        let fail = |m: &str| Err(format!("inconsistent checkpoint: {m}"));
        if self.solver != "bb-mrv-fast" {
            return fail("solver is not bb-mrv-fast");
        }
        if self.segments == 0 {
            return fail("segments is 0");
        }
        if t.nodes > self.cap {
            return fail("nodes above cap");
        }
        // One propagate call per node plus the root's.
        if t.propagation != t.nodes + 1 {
            return fail("propagation is not nodes + 1");
        }
        if t.backtracks > t.nodes {
            return fail("backtracks above nodes");
        }
        if !(t.elapsed_s.is_finite() && t.elapsed_s >= 0.0) {
            return fail("elapsed_s");
        }
        match (&self.hash, self.hashed) {
            (None, None) => {}
            (Some(h), Some(n)) => {
                if !is_hex16(h) {
                    return fail("hash is not 16 hex digits");
                }
                if n != t.nodes {
                    return fail("hashed is not nodes");
                }
            }
            _ => return fail("hash and hashed must both be set or both null"),
        }
        if !is_hex16(&self.model_fnv) || !is_hex16(&self.state_fnv) {
            return fail("model_fnv / state_fnv are not 16 hex digits");
        }
        if self.stack.is_empty() || self.stack.len() > cells {
            return fail("stack length");
        }
        if self.deepest + 1 < self.stack.len() || self.deepest > cells {
            return fail("deepest");
        }
        if !(0.0..1.0).contains(&self.closed_share) {
            return fail("closed_share outside [0, 1)");
        }
        for (i, l) in self.stack.iter().enumerate() {
            if l.cell >= cells || l.index >= l.options.len() {
                return fail(&format!("level {i}: cell or index out of range"));
            }
        }
        Ok(())
    }
}

/// What one segment should do besides searching.
#[derive(Default)]
pub struct Config {
    pub meta: InstanceMeta,
    pub progress: Option<PathBuf>,
    /// 0 = no progress records.
    pub progress_every: u64,
    /// Written at the cap; also every `checkpoint_every` nodes when > 0.
    pub checkpoint: Option<PathBuf>,
    pub checkpoint_every: u64,
    pub resume: Option<Checkpoint>,
    /// Total cap of this segment, stored in the checkpoints it writes.
    pub cap_total: u64,
}

/// One checkpoint write: (depth, milliseconds, bytes).
#[derive(Serialize, Clone, Debug, Default)]
pub struct CkptWrite {
    pub nodes: u64,
    pub depth: usize,
    pub ms: f64,
    pub bytes: u64,
}

/// Filled by `solve`.
#[derive(Serialize, Clone, Debug, Default)]
pub struct Report {
    pub base: Totals,
    pub segment: u32,
    pub closed_share: f64,
    pub depth: usize,
    pub deepest: usize,
    pub guard_fired: bool,
    pub replay_ms: f64,
    pub model_fnv: String,
    pub progress_lines: u64,
    pub writes: Vec<CkptWrite>,
    /// Path of the checkpoint written at the cap, if any.
    pub checkpoint: Option<String>,
    pub segment_elapsed_s: f64,
    /// Failed progress or periodic checkpoint writes (the row becomes an error).
    pub io_errors: Vec<String>,
}

pub struct ResumableSolver<H: ResumeHook> {
    pub guard: Duration,
    pub hook: H,
    pub cfg: Config,
    pub report: Report,
    pub error: Option<String>,
}

impl<H: ResumeHook> ResumableSolver<H> {
    pub fn new(guard: Duration, hook: H, cfg: Config) -> Self {
        Self {
            guard,
            hook,
            cfg,
            report: Report::default(),
            error: None,
        }
    }
}

enum Step {
    Found,
    Failed,
    Cap,
}

const NONE: u32 = u32::MAX;
/// Domains with at most this many options compute offered colours by walking
/// their set bits; larger ones test each colour mask.
const SMALL: u32 = 12;

const FNV0: u64 = 0xcbf2_9ce4_8422_2325;

fn fnv(h: &mut u64, bytes: &[u8]) {
    for &b in bytes {
        *h ^= u64::from(b);
        *h = h.wrapping_mul(0x0100_0000_01b3);
    }
}

/// Gather the low bit of each nibble of `x` (already folded) into 16 bits.
#[inline(always)]
fn nib(x: u64) -> u64 {
    let mut y = x | (x >> 1);
    y |= y >> 2;
    y &= 0x1111_1111_1111_1111;
    y = (y | (y >> 3)) & 0x0303_0303_0303_0303;
    y = (y | (y >> 6)) & 0x000F_000F_000F_000F;
    y = (y | (y >> 12)) & 0x0000_00FF_0000_00FF;
    (y | (y >> 24)) & 0xFFFF
}

/// Clear the lowest `j` set bits of `dom`.
fn skip<const W: usize>(mut dom: [u64; W], mut j: usize) -> [u64; W] {
    for w in &mut dom {
        while j > 0 && *w != 0 {
            *w &= *w - 1;
            j -= 1;
        }
    }
    dom
}

struct Fast<'a, const W: usize, H: ResumeHook> {
    cells: usize,
    nc: usize,
    side_mask: Vec<[u64; W]>,
    bit_col: Vec<[u8; 4]>,
    bit_piece: Vec<u16>,
    bit_rot: Vec<u8>,
    nb: Vec<[u32; 4]>,
    all_pieces: u128,
    states: Vec<[u64; W]>,
    assigned: u128,
    queue: Vec<u32>,
    pres: Vec<u128>,
    propagations: u64,
    start: Instant,
    guard: Duration,
    board_level: usize,
    hook: &'a mut H,
    // Additions for progress / checkpoint / resume.
    lvl_cell: Vec<u32>,
    lvl_j: Vec<u32>,
    deepest: usize,
    stop_lv: usize,
    guard_fired: bool,
    /// Segment-local node count at which `event` runs next.
    next_event: u64,
    cfg: &'a Config,
    rep: &'a mut Report,
    progress_file: Option<File>,
    model_fnv: u64,
}

impl<const W: usize, H: ResumeHook> Fast<'_, W, H> {
    // ---- verbatim from mrv-node-cost FastSolver ---------------------------

    #[inline(always)]
    fn count(x: &[u64; W]) -> u32 {
        x.iter().map(|w| w.count_ones()).sum()
    }

    #[inline(always)]
    fn offered(&self, dom: &[u64; W], side: usize) -> u32 {
        let mut mask = 0u32;
        if Self::count(dom) <= SMALL {
            for (k, &w) in dom.iter().enumerate() {
                let mut x = w;
                while x != 0 {
                    let b = k * 64 + x.trailing_zeros() as usize;
                    mask |= 1 << self.bit_col[b][side];
                    x &= x - 1;
                }
            }
        } else {
            let base = side * self.nc;
            for col in 0..self.nc {
                let sm = &self.side_mask[base + col];
                let mut hit = 0u64;
                for k in 0..W {
                    hit |= dom[k] & sm[k];
                }
                if hit != 0 {
                    mask |= 1 << col;
                }
            }
        }
        mask
    }

    #[inline(always)]
    fn presence(dom: &[u64; W]) -> u128 {
        let mut p = 0u128;
        for (k, &w) in dom.iter().enumerate() {
            if w != 0 {
                p |= u128::from(nib(w)) << (16 * k);
            }
        }
        p
    }

    #[inline(always)]
    fn push(&mut self, inq: &mut u128, c: usize) {
        if *inq >> c & 1 == 0 {
            *inq |= 1u128 << c;
            self.queue.push(c as u32);
        }
    }

    /// Propagate level `lv` to the common fixpoint of A3's rules starting
    /// from cell `start`. Returns false on contradiction.
    fn propagate(&mut self, lv: usize, start: usize) -> bool {
        self.propagations += 1;
        let cells = self.cells;
        let base = lv * cells;
        self.queue.clear();
        let mut inq = 0u128;
        self.push(&mut inq, start);
        loop {
            while let Some(b) = self.queue.pop() {
                let b = b as usize;
                inq &= !(1u128 << b);
                let db_dom = self.states[base + b];
                for db in 0..4 {
                    let a = self.nb[b][db];
                    if a == NONE {
                        continue;
                    }
                    let a = a as usize;
                    let da = (db + 2) & 3;
                    let mut mask = self.offered(&db_dom, db);
                    let mut allowed = [0u64; W];
                    let sbase = da * self.nc;
                    while mask != 0 {
                        let col = mask.trailing_zeros() as usize;
                        mask &= mask - 1;
                        let sm = &self.side_mask[sbase + col];
                        for k in 0..W {
                            allowed[k] |= sm[k];
                        }
                    }
                    let da_dom = &mut self.states[base + a];
                    let mut changed = false;
                    let mut nz = 0u64;
                    for k in 0..W {
                        let old = da_dom[k];
                        let new = old & allowed[k];
                        changed |= new != old;
                        da_dom[k] = new;
                        nz |= new;
                    }
                    if nz == 0 {
                        return false;
                    }
                    if changed {
                        self.push(&mut inq, a);
                    }
                }
            }
            // All-different pass (piece singles, hidden singles, piece with no cell).
            let (mut once, mut twice) = (0u128, 0u128);
            for c in 0..cells {
                let p = Self::presence(&self.states[base + c]);
                self.pres[c] = p;
                twice |= once & p;
                once |= p;
            }
            if once != self.all_pieces {
                return false;
            }
            let mut any = false;
            // Piece singles whose piece still appears in another cell.
            for c in 0..cells {
                let p = self.pres[c];
                if p.count_ones() != 1 || p & twice == 0 {
                    continue;
                }
                let pi = p.trailing_zeros() as usize;
                let (k, m) = (pi >> 4, 0xFu64 << ((pi & 15) * 4));
                for c2 in 0..cells {
                    if c2 == c {
                        continue;
                    }
                    let dom = &mut self.states[base + c2];
                    if dom[k] & m != 0 {
                        dom[k] &= !m;
                        if dom.iter().all(|&x| x == 0) {
                            return false;
                        }
                        any = true;
                        self.push(&mut inq, c2);
                    }
                }
            }
            // Hidden singles: pieces present in exactly one cell.
            let hs = once & !twice;
            if hs != 0 {
                for c in 0..cells {
                    let h = self.pres[c] & hs;
                    if h == 0 {
                        continue;
                    }
                    if h.count_ones() > 1 {
                        return false;
                    }
                    let pi = h.trailing_zeros() as usize;
                    let (k, m) = (pi >> 4, 0xFu64 << ((pi & 15) * 4));
                    let dom = &mut self.states[base + c];
                    let mut changed = false;
                    for (j, x) in dom.iter_mut().enumerate() {
                        let new = if j == k { *x & m } else { 0 };
                        changed |= new != *x;
                        *x = new;
                    }
                    if changed {
                        if dom.iter().all(|&x| x == 0) {
                            return false;
                        }
                        any = true;
                        self.push(&mut inq, c);
                    }
                }
            }
            if !any {
                return true;
            }
        }
    }

    /// Free cell minimising (|D|, index).
    #[inline]
    fn choose(&self, lv: usize) -> usize {
        let base = lv * self.cells;
        let mut best = usize::MAX;
        let mut best_cnt = u32::MAX;
        for c in 0..self.cells {
            if self.assigned >> c & 1 != 0 {
                continue;
            }
            let k = Self::count(&self.states[base + c]);
            if k < best_cnt {
                best_cnt = k;
                best = c;
                if k == 1 {
                    break;
                }
            }
        }
        best
    }

    // ---- search with bookkeeping ------------------------------------------

    fn dfs(&mut self, lv: usize, left: usize, counter: &mut NodeCounter) -> Step {
        if lv > self.deepest {
            self.deepest = lv;
        }
        if left == 0 {
            self.board_level = lv;
            self.stop_lv = lv;
            return Step::Found;
        }
        let c = self.choose(lv);
        self.lvl_cell[lv] = c as u32;
        let dom = self.states[lv * self.cells + c];
        self.branch(lv, c, dom, 0, left, counter)
    }

    /// Try the options of `dom` (already stripped of the first `j0` options
    /// of the level) at cell `c` of level `lv`. The node body is the original
    /// `FastSolver::dfs` loop body.
    #[inline(always)]
    fn branch(
        &mut self,
        lv: usize,
        c: usize,
        dom: [u64; W],
        j0: u32,
        left: usize,
        counter: &mut NodeCounter,
    ) -> Step {
        let cells = self.cells;
        let mut j = j0;
        for (k, &w) in dom.iter().enumerate() {
            let mut x = w;
            while x != 0 {
                let b = k * 64 + x.trailing_zeros() as usize;
                x &= x - 1;
                self.lvl_j[lv] = j;
                j += 1;
                if counter.nodes() == self.next_event {
                    self.event(lv, counter);
                }
                if !counter.try_assign() {
                    self.stop_lv = lv;
                    return Step::Cap;
                }
                if self.start.elapsed() > self.guard {
                    self.guard_fired = true;
                    self.stop_lv = lv;
                    return Step::Cap;
                }
                self.hook.node(c, self.bit_piece[b], self.bit_rot[b]);
                let (lo, hi) = self.states.split_at_mut((lv + 1) * cells);
                hi[..cells].copy_from_slice(&lo[lv * cells..]);
                self.assigned |= 1u128 << c;
                let mut single = [0u64; W];
                single[k] = 1u64 << (b & 63);
                hi[c] = single;
                if self.propagate(lv + 1, c) {
                    match self.dfs(lv + 1, left - 1, counter) {
                        Step::Failed => {}
                        done => return done,
                    }
                }
                self.assigned &= !(1u128 << c);
                counter.backtracks += 1;
            }
        }
        Step::Failed
    }

    /// Continue a replayed stack whose deepest level is `last`.
    fn dfs_resume(
        &mut self,
        lv: usize,
        last: usize,
        left: usize,
        counter: &mut NodeCounter,
    ) -> Step {
        let c = self.lvl_cell[lv] as usize;
        let dom = self.states[lv * self.cells + c];
        let j = self.lvl_j[lv];
        if lv == last {
            return self.branch(lv, c, skip(dom, j as usize), j, left, counter);
        }
        match self.dfs_resume(lv + 1, last, left - 1, counter) {
            Step::Failed => {}
            done => return done,
        }
        self.assigned &= !(1u128 << c);
        counter.backtracks += 1;
        self.branch(lv, c, skip(dom, j as usize + 1), j + 1, left, counter)
    }

    // ---- progress, checkpoint, replay -------------------------------------

    /// Closed share with levels 0..=last (Horner from the deepest level).
    fn closed_share(&self, last: usize) -> f64 {
        let mut s = 0.0f64;
        for i in (0..=last).rev() {
            let c = self.lvl_cell[i] as usize;
            let k = Self::count(&self.states[i * self.cells + c]);
            s = (f64::from(self.lvl_j[i]) + s) / f64::from(k);
        }
        s
    }

    fn opts_of(&self, dom: &[u64; W]) -> Vec<[u16; 2]> {
        let mut v = Vec::new();
        for (k, &w) in dom.iter().enumerate() {
            let mut x = w;
            while x != 0 {
                let b = k * 64 + x.trailing_zeros() as usize;
                x &= x - 1;
                v.push([self.bit_piece[b], u16::from(self.bit_rot[b])]);
            }
        }
        v
    }

    fn state_fnv(&self, lv: usize) -> u64 {
        let mut h = FNV0;
        for d in &self.states[lv * self.cells..(lv + 1) * self.cells] {
            for w in d {
                fnv(&mut h, &w.to_le_bytes());
            }
        }
        h
    }

    fn totals(&self, counter: &NodeCounter) -> Totals {
        let b = &self.rep.base;
        Totals {
            nodes: b.nodes + counter.nodes(),
            backtracks: b.backtracks + counter.backtracks,
            propagation: b.propagation + self.propagations,
            elapsed_s: b.elapsed_s + self.start.elapsed().as_secs_f64(),
        }
    }

    fn checkpoint(&self, last: usize, counter: &NodeCounter) -> Checkpoint {
        let stack = (0..=last)
            .map(|i| {
                let c = self.lvl_cell[i] as usize;
                Level {
                    cell: c,
                    options: self.opts_of(&self.states[i * self.cells + c]),
                    index: self.lvl_j[i] as usize,
                }
            })
            .collect();
        let hs = self.hook.get();
        Checkpoint {
            format: FORMAT.into(),
            version: VERSION,
            solver: "bb-mrv-fast".into(),
            instance: self.cfg.meta.clone(),
            model_fnv: format!("{:016x}", self.model_fnv),
            segments: self.rep.segment,
            totals: self.totals(counter),
            hash: hs.map(|(h, _)| format!("{h:016x}")),
            hashed: hs.map(|(_, n)| n),
            deepest: self.deepest,
            closed_share: self.closed_share(last),
            stack,
            state_fnv: format!("{:016x}", self.state_fnv(last)),
            cap: self.cfg.cap_total,
            digest: String::new(),
        }
    }

    fn write_checkpoint(&mut self, last: usize, counter: &NodeCounter) -> Result<String, String> {
        let Some(p) = self.cfg.checkpoint.as_ref() else {
            return Err("no checkpoint path".into());
        };
        let t = Instant::now();
        let mut ck = self.checkpoint(last, counter);
        ck.seal();
        let text = serde_json::to_string(&ck).map_err(|e| e.to_string())?;
        if let Some(d) = p.parent() {
            fs::create_dir_all(d).map_err(|e| e.to_string())?;
        }
        let tmp = p.with_extension("tmp");
        fs::write(&tmp, &text).map_err(|e| e.to_string())?;
        fs::rename(&tmp, p).map_err(|e| e.to_string())?;
        self.rep.writes.push(CkptWrite {
            nodes: ck.totals.nodes,
            depth: last,
            ms: t.elapsed().as_secs_f64() * 1000.0,
            bytes: text.len() as u64,
        });
        Ok(p.display().to_string())
    }

    /// Runs just before the `try_assign` that would make total node count
    /// `next_event + base + 1`.
    #[cold]
    #[inline(never)]
    fn event(&mut self, lv: usize, counter: &NodeCounter) {
        let total = self.rep.base.nodes + counter.nodes();
        let pe = self.cfg.progress_every;
        let ce = if self.cfg.checkpoint.is_some() {
            self.cfg.checkpoint_every
        } else {
            0
        };
        if pe > 0 && total % pe == 0 {
            if let Some(p) = &self.cfg.progress {
                let t = self.totals(counter);
                let line = serde_json::json!({
                    "instance": self.cfg.meta.id, "segment": self.rep.segment,
                    "nodes": t.nodes, "backtracks": t.backtracks, "propagation": t.propagation,
                    "depth": lv, "deepest": self.deepest, "elapsed_s": t.elapsed_s,
                    "closed_share": self.closed_share(lv),
                });
                if self.progress_file.is_none() {
                    self.progress_file = OpenOptions::new().create(true).append(true).open(p).ok();
                }
                let r = match self.progress_file.as_mut() {
                    Some(f) => writeln!(f, "{line}").and_then(|()| f.flush()),
                    None => Err(std::io::Error::other("progress file not open")),
                };
                if let Err(e) = r {
                    self.rep
                        .io_errors
                        .push(format!("progress {}: {e}", p.display()));
                }
                self.rep.progress_lines += 1;
            }
        }
        if ce > 0 && total % ce == 0 {
            if let Err(e) = self.write_checkpoint(lv, counter) {
                self.rep.io_errors.push(format!("periodic checkpoint: {e}"));
            }
        }
        self.next_event = next_event(self.rep.base.nodes, total, pe, ce);
    }

    /// Rebuild the stack of `ck` by replaying its decisions from the root.
    fn replay(&mut self, ck: &Checkpoint) -> Result<usize, String> {
        if ck.stack.is_empty() {
            return Err("empty stack".into());
        }
        let last = ck.stack.len() - 1;
        let cells = self.cells;
        for (i, l) in ck.stack.iter().enumerate() {
            let c = self.choose(i);
            if c != l.cell {
                return Err(format!(
                    "level {i}: choose gave cell {c}, checkpoint {}",
                    l.cell
                ));
            }
            let dom = self.states[i * cells + c];
            if self.opts_of(&dom) != l.options {
                return Err(format!("level {i}: domain differs from checkpoint"));
            }
            if l.index >= l.options.len() {
                return Err(format!("level {i}: index out of range"));
            }
            self.lvl_cell[i] = c as u32;
            self.lvl_j[i] = l.index as u32;
            if i < last {
                let one = skip(dom, l.index);
                let k = one.iter().position(|&x| x != 0).expect("option");
                let b = k * 64 + one[k].trailing_zeros() as usize;
                let (lo, hi) = self.states.split_at_mut((i + 1) * cells);
                hi[..cells].copy_from_slice(&lo[i * cells..]);
                self.assigned |= 1u128 << c;
                let mut single = [0u64; W];
                single[k] = 1u64 << (b & 63);
                hi[c] = single;
                if !self.propagate(i + 1, c) {
                    return Err(format!("level {i}: propagation failed in replay"));
                }
            }
        }
        if format!("{:016x}", self.state_fnv(last)) != ck.state_fnv {
            return Err("replayed state digest differs from checkpoint".into());
        }
        // Recomputed from the replayed stack, independently of the digest.
        let share = self.closed_share(last);
        if share.to_bits() != ck.closed_share.to_bits() {
            return Err(format!(
                "replayed closed share {share:e} differs from checkpoint {:e}",
                ck.closed_share
            ));
        }
        if ck.deepest < last {
            return Err("checkpoint deepest is below its stack depth".into());
        }
        Ok(last)
    }
}

/// Smallest total node count above `total` that is a multiple of `pe` or
/// `ce` (0 = off), as a segment-local count; `u64::MAX` if none.
fn next_event(base: u64, total: u64, pe: u64, ce: u64) -> u64 {
    let nxt = |p: u64| {
        if p == 0 {
            u64::MAX
        } else {
            (total / p + 1) * p
        }
    };
    let t = nxt(pe).min(nxt(ce));
    if t == u64::MAX {
        u64::MAX
    } else {
        t - base
    }
}

fn solve_w<const W: usize, H: ResumeHook>(
    m: &Model,
    guard: Duration,
    start: Instant,
    hook: &mut H,
    cfg: &Config,
    rep: &mut Report,
    counter: &mut NodeCounter,
) -> Result<Outcome, String> {
    let cells = m.cells;
    let pieces = m.piece_range.len();
    let nc = m.num_colors;
    let nbits = W * 64;
    let bit_of: Vec<usize> = (0..m.opt_piece.len())
        .map(|o| {
            let p = m.opt_piece[o] as usize;
            4 * p + (o - m.piece_range[p].0)
        })
        .collect();
    let mut side_mask = vec![[0u64; W]; 4 * nc];
    let mut bit_col = vec![[0u8; 4]; nbits];
    let mut bit_piece = vec![0u16; nbits];
    let mut bit_rot = vec![0u8; nbits];
    for (o, &b) in bit_of.iter().enumerate() {
        let e = m.opt_edges[o];
        for s in 0..4 {
            side_mask[s * nc + e[s] as usize][b >> 6] |= 1u64 << (b & 63);
        }
        bit_col[b] = e;
        bit_piece[b] = m.opt_piece[o];
        bit_rot[b] = m.opt_rot[o];
    }
    let nb: Vec<[u32; 4]> = m
        .neighbour
        .iter()
        .map(|r| r.map(|x| x.map_or(NONE, |v| v as u32)))
        .collect();
    let mut states = vec![[0u64; W]; (cells + 1) * cells];
    let root = m.root();
    for c in 0..cells {
        for o in options(m, root, c) {
            let b = bit_of[o];
            states[c][b >> 6] |= 1u64 << (b & 63);
        }
    }
    let all_pieces = if pieces == 128 {
        u128::MAX
    } else {
        (1u128 << pieces) - 1
    };
    // Model digest: everything the search reads, plus the root domains.
    let mut mf = FNV0;
    fnv(&mut mf, &(cells as u64).to_le_bytes());
    fnv(&mut mf, &(nc as u64).to_le_bytes());
    fnv(&mut mf, &(W as u64).to_le_bytes());
    for b in 0..nbits {
        fnv(&mut mf, &bit_piece[b].to_le_bytes());
        fnv(&mut mf, &[bit_rot[b]]);
        fnv(&mut mf, &bit_col[b]);
    }
    for r in &nb {
        for x in r {
            fnv(&mut mf, &x.to_le_bytes());
        }
    }
    for d in &states[..cells] {
        for w in d {
            fnv(&mut mf, &w.to_le_bytes());
        }
    }
    rep.model_fnv = format!("{mf:016x}");
    // Open the progress file before searching: a path that cannot be opened
    // fails the segment at once instead of dropping records.
    let progress_file = match (&cfg.progress, cfg.progress_every) {
        (Some(p), pe) if pe > 0 => {
            if let Some(d) = p.parent().filter(|d| !d.as_os_str().is_empty()) {
                fs::create_dir_all(d).map_err(|e| format!("progress dir {}: {e}", d.display()))?;
            }
            Some(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(p)
                    .map_err(|e| format!("progress file {}: {e}", p.display()))?,
            )
        }
        _ => None,
    };
    let mut f = Fast::<W, H> {
        cells,
        nc,
        side_mask,
        bit_col,
        bit_piece,
        bit_rot,
        nb,
        all_pieces,
        states,
        assigned: 0,
        queue: Vec::with_capacity(4 * cells + 8),
        pres: vec![0; cells],
        propagations: 1,
        start,
        guard,
        board_level: 0,
        hook,
        lvl_cell: vec![0; cells + 1],
        lvl_j: vec![0; cells + 1],
        deepest: 0,
        stop_lv: 0,
        guard_fired: false,
        next_event: u64::MAX,
        cfg,
        rep,
        progress_file,
        model_fnv: mf,
    };
    let out = if let Some(ck) = &cfg.resume {
        if ck.model_fnv != f.rep.model_fnv {
            return Err("checkpoint is for a different model (model_fnv differs)".into());
        }
        if ck.instance != cfg.meta {
            return Err("checkpoint is for a different instance".into());
        }
        if ck.stack.len() > cells {
            return Err("checkpoint stack is deeper than the board".into());
        }
        let t = Instant::now();
        let last = f.replay(ck)?;
        f.rep.replay_ms = t.elapsed().as_secs_f64() * 1000.0;
        f.propagations = 0;
        f.deepest = ck.deepest;
        if let (Some(h), Some(n)) = (&ck.hash, ck.hashed) {
            let h = u64::from_str_radix(h, 16).map_err(|e| e.to_string())?;
            f.hook.set(h, n);
        }
        f.next_event = next_event(
            f.rep.base.nodes,
            f.rep.base.nodes,
            cfg.progress_every,
            if cfg.checkpoint.is_some() {
                cfg.checkpoint_every
            } else {
                0
            },
        );
        f.dfs_resume(0, last, cells, counter)
    } else {
        f.next_event = next_event(
            0,
            0,
            cfg.progress_every,
            if cfg.checkpoint.is_some() {
                cfg.checkpoint_every
            } else {
                0
            },
        );
        f.dfs(0, cells, counter)
    };
    counter.propagation = f.propagations;
    f.rep.deepest = f.deepest;
    f.rep.guard_fired = f.guard_fired;
    f.rep.segment_elapsed_s = f.start.elapsed().as_secs_f64();
    Ok(match out {
        Step::Found => {
            f.rep.depth = cells;
            f.rep.closed_share = f.closed_share(cells - 1);
            let base = f.board_level * cells;
            let board = (0..cells)
                .map(|c| {
                    let d = &f.states[base + c];
                    let k = d.iter().position(|&x| x != 0).expect("assigned cell");
                    let b = k * 64 + d[k].trailing_zeros() as usize;
                    i32::from(f.bit_piece[b]) * 4 + i32::from(f.bit_rot[b])
                })
                .collect();
            Outcome::Solved(board)
        }
        Step::Failed => {
            f.rep.depth = 0;
            f.rep.closed_share = 1.0;
            Outcome::Exhausted
        }
        Step::Cap => {
            let lv = f.stop_lv;
            f.rep.depth = lv;
            f.rep.closed_share = f.closed_share(lv);
            if !f.guard_fired && cfg.checkpoint.is_some() {
                let p = f.write_checkpoint(lv, counter)?;
                f.rep.checkpoint = Some(p);
            }
            Outcome::CapReached
        }
    })
}

impl<H: ResumeHook> LadderSolver for ResumableSolver<H> {
    fn solve(&mut self, puzzle: &Puzzle, counter: &mut NodeCounter) -> Outcome {
        let start = Instant::now();
        self.report = Report::default();
        self.error = None;
        if let Some(ck) = &self.cfg.resume {
            self.report.base = ck.totals.clone();
            self.report.segment = ck.segments + 1;
        } else {
            self.report.segment = 1;
        }
        let m = Model::new(puzzle);
        if !m.root_ok {
            // Refuted at the root: the whole tree is closed with 0 nodes.
            self.report.closed_share = 1.0;
            self.report.depth = 0;
            return Outcome::Exhausted;
        }
        assert!(m.cells <= 128, "supports at most 128 cells (n <= 11)");
        let words = (4 * m.cells).div_ceil(64);
        let (g, h, cfg, rep) = (self.guard, &mut self.hook, &self.cfg, &mut self.report);
        let r = match words {
            1 => solve_w::<1, H>(&m, g, start, h, cfg, rep, counter),
            2 => solve_w::<2, H>(&m, g, start, h, cfg, rep, counter),
            3 => solve_w::<3, H>(&m, g, start, h, cfg, rep, counter),
            4 => solve_w::<4, H>(&m, g, start, h, cfg, rep, counter),
            5 => solve_w::<5, H>(&m, g, start, h, cfg, rep, counter),
            6 => solve_w::<6, H>(&m, g, start, h, cfg, rep, counter),
            7 => solve_w::<7, H>(&m, g, start, h, cfg, rep, counter),
            8 => solve_w::<8, H>(&m, g, start, h, cfg, rep, counter),
            _ => unreachable!(),
        };
        let r = match r {
            Ok(_) if !self.report.io_errors.is_empty() => Err(self.report.io_errors.join("; ")),
            r => r,
        };
        match r {
            Ok(o) => o,
            Err(e) => {
                self.error = Some(e);
                Outcome::Exhausted
            }
        }
    }
}
