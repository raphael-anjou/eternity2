//! ProbCut on the hard ladder: a forward-checked row-major DFS (`fc-dfs`),
//! its Luby-restart variant (`fc-luby`) and the ProbCut candidates
//! (`probcut-v1`, `probcut-v2`). See `../PREREG.md`.

pub mod search;

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

use eternity2_engine::{rotated, Puzzle};
use hard_ladder::instance::{generate, owen_criterion, FRAME_COLOURS};
use hard_ladder::{LadderInstance, LadderSolver, NodeCounter, Outcome, SolverEntry};
use search::{count_deeper, luby, Board, Budget, CutTest, Dfs, Step, Tables};
use serde::{Deserialize, Serialize};

/// Luby unit (nodes), set from calibration before a run.
pub static LUBY_UNIT: AtomicU64 = AtomicU64::new(1000);

/// Calibrated ProbCut parameters for one rung (keyed by board size).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RungParams {
    pub n: usize,
    /// Probe index j: the probe counts nodes j + 1 cells deeper (v_j).
    pub k: usize,
    /// Filled-cell depths with the planted-prefix mean and sd of v_k.
    pub depths: Vec<usize>,
    pub mu: Vec<f64>,
    pub sigma: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
    pub luby_unit: u64,
    pub rungs: Vec<RungParams>,
}

pub static PARAMS: OnceLock<Params> = OnceLock::new();

/// Per-run ProbCut diagnostics keyed by puzzle name.
#[derive(Debug, Clone, Default, Serialize)]
pub struct PassInfo {
    pub passes_run: usize,
    pub solved_in_pass: Option<usize>,
    pub cuts_fired: u64,
    pub probes: u64,
}

pub static PASS_INFO: Mutex<Option<HashMap<String, PassInfo>>> = Mutex::new(None);

#[must_use]
pub fn z_of(p: f64) -> f64 {
    match p {
        x if (x - 0.2).abs() < 1e-12 => 0.841_621_233_572_914,
        x if (x - 0.05).abs() < 1e-12 => 1.644_853_626_951_472,
        x if (x - 0.01).abs() < 1e-12 => 2.326_347_874_040_841,
        _ => panic!("no z for p = {p}"),
    }
}

#[must_use]
pub fn cut_tests(rp: &RungParams, p: f64) -> Vec<CutTest> {
    if p == 0.0 {
        return Vec::new();
    }
    let z = z_of(p);
    rp.depths
        .iter()
        .zip(rp.mu.iter().zip(&rp.sigma))
        .map(|(&d, (&m, &s))| CutTest { filled: d, probe: rp.k + 1, t: m - z * s })
        .collect()
}

fn finish(st: Step, d: &Dfs) -> Option<Outcome> {
    match st {
        Step::Found => Some(Outcome::Solved(d.b.ladder_board())),
        Step::Cap => Some(Outcome::CapReached),
        Step::Failed | Step::Limit => None,
    }
}

/// Row-major DFS, forward checking on or off.
pub struct FcDfs {
    pub fc: bool,
}

impl LadderSolver for FcDfs {
    fn solve(&mut self, puzzle: &Puzzle, counter: &mut NodeCounter) -> Outcome {
        let t = Tables::new(puzzle);
        let mut d = Dfs { b: Board::new(&t, self.fc), c: counter, cuts: Vec::new(), cut_fired: 0, probes: 0, local_left: u64::MAX, rng: None };
        let st = d.run(0);
        let fit = d.b.fit_tests;
        let out = finish(st, &d).unwrap_or(Outcome::Exhausted);
        counter.fit_tests = fit;
        out
    }
}

/// `fc-dfs` with Luby restarts and a random cyclic start in each candidate list.
pub struct FcLuby {
    pub seed: u64,
}

fn name_hash(s: &str) -> u64 {
    s.bytes().fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ u64::from(b)).wrapping_mul(0x100_0000_01b3))
}

impl LadderSolver for FcLuby {
    fn solve(&mut self, puzzle: &Puzzle, counter: &mut NodeCounter) -> Outcome {
        let t = Tables::new(puzzle);
        let unit = LUBY_UNIT.load(Ordering::Relaxed);
        let mut d = Dfs { b: Board::new(&t, true), c: counter, cuts: Vec::new(), cut_fired: 0, probes: 0, local_left: 0, rng: Some((self.seed ^ name_hash(&puzzle.name)) | 1) };
        let mut i = 1u64;
        let out = loop {
            d.local_left = unit.saturating_mul(luby(i));
            let st = d.run(0);
            match st {
                Step::Limit => i += 1,
                Step::Failed => break Outcome::Exhausted,
                other => break finish(other, &d).expect("terminal"),
            }
        };
        let fit = d.b.fit_tests;
        counter.fit_tests = fit;
        out
    }
}

/// `fc-dfs` plus ProbCut tests, relaxed pass by pass along `schedule`.
pub struct ProbCut {
    pub schedule: &'static [f64],
}

impl LadderSolver for ProbCut {
    fn solve(&mut self, puzzle: &Puzzle, counter: &mut NodeCounter) -> Outcome {
        let t = Tables::new(puzzle);
        let n = puzzle.width as usize;
        let rp = PARAMS.get().expect("params loaded").rungs.iter().find(|r| r.n == n).expect("rung params").clone();
        let mut info = PassInfo::default();
        let mut d = Dfs { b: Board::new(&t, true), c: counter, cuts: Vec::new(), cut_fired: 0, probes: 0, local_left: u64::MAX, rng: None };
        let mut out = Outcome::Exhausted;
        for (pi, &p) in self.schedule.iter().enumerate() {
            d.cuts = cut_tests(&rp, p);
            info.passes_run = pi + 1;
            let st = d.run(0);
            if let Some(o) = finish(st, &d) {
                if matches!(o, Outcome::Solved(_)) {
                    info.solved_in_pass = Some(pi + 1);
                }
                out = o;
                break;
            }
        }
        info.cuts_fired = d.cut_fired;
        info.probes = d.probes;
        let fit = d.b.fit_tests;
        counter.fit_tests = fit;
        counter.propagation = info.probes;
        PASS_INFO.lock().expect("lock").get_or_insert_with(HashMap::new).insert(puzzle.name.clone(), info);
        out
    }
}

pub const V1_SCHEDULE: &[f64] = &[0.2, 0.05, 0.01, 0.0];
pub const V2_SCHEDULE: &[f64] = &[0.05, 0.0];

pub const SOLVERS: &[SolverEntry] = &[
    SolverEntry { name: "fc-dfs", description: "row-major DFS + forward checking", run_seed: 0, make: |_| Box::new(FcDfs { fc: true }) },
    SolverEntry { name: "rm-dfs", description: "row-major DFS, no forward checking (equals hp-dfs)", run_seed: 0, make: |_| Box::new(FcDfs { fc: false }) },
    SolverEntry { name: "fc-luby", description: "fc-dfs + Luby restarts, random value order", run_seed: 1, make: |s| Box::new(FcLuby { seed: s }) },
    SolverEntry { name: "probcut-v1", description: "fc-dfs + ProbCut, p = 0.2, 0.05, 0.01, 0", run_seed: 0, make: |_| Box::new(ProbCut { schedule: V1_SCHEDULE }) },
    SolverEntry { name: "probcut-v2", description: "fc-dfs + ProbCut, p = 0.05, 0", run_seed: 0, make: |_| Box::new(ProbCut { schedule: V2_SCHEDULE }) },
];

/// The R8 instance: the ladder's generator call and scramble at 8x8, c = 10.
#[must_use]
pub fn instance_8c10(role: &str, seed: u32) -> LadderInstance {
    let (n, c) = (8u8, 10u8);
    let (puzzle, planted) = generate(n, c, seed);
    LadderInstance {
        id: format!("n{n}-c{c}-s{seed}"),
        ladder: "8-c10".into(),
        role: role.into(),
        n,
        c,
        seed,
        owen_criterion: owen_criterion(n),
        generator: format!(
            "generate_solved_framed_bc({n}, {}, {seed}, true, {FRAME_COLOURS}) + XorShift({seed} ^ 0xA5A5A5A5) scramble",
            c + FRAME_COLOURS
        ),
        puzzle,
        planted,
    }
}

/// One stage-1 row: a forward-checked child of the planted (d-1)-prefix.
#[derive(Debug, Clone, Serialize)]
pub struct ChildRow {
    pub instance: String,
    pub n: u8,
    pub c: u8,
    pub seed: u32,
    pub d: usize,
    pub planted: bool,
    /// v_j = log2(1 + forward-checked nodes j + 1 cells deeper), j = 0..4.
    pub v: Vec<f64>,
    pub v_capped: Vec<bool>,
    /// log2(1 + nodes 8 cells deeper) and whether its count hit the budget.
    pub l8: Option<f64>,
    pub l8_capped: bool,
}

pub const V_BUDGET: u64 = 20_000_000;
pub const L8_BUDGET: u64 = 2_000_000;

/// Stage-1 rows for one instance at the given depths. `siblings = false`
/// keeps only the planted child and skips the depth-8 count.
#[must_use]
pub fn stage1_rows(inst: &LadderInstance, depths: &[usize], siblings: bool) -> Vec<ChildRow> {
    let p = &inst.puzzle;
    let t = Tables::new(p);
    let planted: Vec<u32> = inst
        .planted
        .iter()
        .map(|&v| {
            let (pc, r) = ((v / 4) as usize, (v % 4) as u8);
            t.oriented(pc, rotated(p.pieces[pc], r))
        })
        .collect();
    let mut rows = Vec::new();
    for &d in depths {
        let mut b = Board::new(&t, true);
        for (pos, &o) in planted.iter().enumerate().take(d - 1) {
            b.place(pos, o);
        }
        let pos = d - 1;
        let mut cands = Vec::new();
        b.candidates(pos, &mut cands);
        let target = t.edges[planted[pos] as usize];
        for o in cands {
            let is_planted = t.edges[o as usize] == target;
            if !siblings && !is_planted {
                continue;
            }
            b.place(pos, o);
            if b.forward_ok(pos) {
                let mut v = Vec::new();
                let mut vc = Vec::new();
                for j in 0..5 {
                    let r = count_deeper(&mut b, d, j + 1, u64::MAX, &mut Budget { left: V_BUDGET });
                    let (cnt, capped) = match r {
                        Ok(x) => (x, false),
                        Err(x) => (x, true),
                    };
                    v.push((1.0 + cnt as f64).log2());
                    vc.push(capped);
                }
                let (l8, l8_capped) = if siblings {
                    match count_deeper(&mut b, d, 8, u64::MAX, &mut Budget { left: L8_BUDGET }) {
                        Ok(x) => (Some((1.0 + x as f64).log2()), false),
                        Err(x) => (Some((1.0 + x as f64).log2()), true),
                    }
                } else {
                    (None, false)
                };
                rows.push(ChildRow {
                    instance: inst.id.clone(),
                    n: inst.n,
                    c: inst.c,
                    seed: inst.seed,
                    d,
                    planted: is_planted,
                    v,
                    v_capped: vc,
                    l8,
                    l8_capped,
                });
            } else {
                assert!(!is_planted, "planted child failed forward checking");
            }
            b.unplace(pos);
        }
    }
    rows
}
