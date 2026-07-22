//! ledger-prune reproduction: a break-budget row-major DFS with the LEDGER
//! global colour-ledger prune (deficit + parity vs the remaining break budget).
//!
//! Modes:
//!   cargo run --release -- --mode soundness [--seed S]
//!   cargo run --release -- --mode ab [--seed S] [--suffix K] [--budget R] [--cap-secs T]
//!
//! Soundness replays a generated board's perfect tail at zero slack and asserts
//! the ledger never fires along the real line. AB exhausts a fixed-prefix
//! suffix twice (prune off / prune on), counting ALL completions within the
//! break budget; completions must be identical between arms and the node ratio
//! is the reproduced quantity. See PLAN.md for the claim and its provenance.

#![forbid(unsafe_code)]

use std::time::{Duration, Instant};

use e2_kit::{generator, instance_from_generated, Board, Budget, Instance, SolveOutcome, Solver};

/// Colour ids fit comfortably below 32 on the official shape (22 interior + 5
/// frame colours + grey 0).
const MAXC: usize = 32;

struct Ctx {
    w: usize,
    h: usize,
    n: usize,
    /// Per piece: its URDL edges at each of the 4 rotations.
    rots: Vec<[[u8; 4]; 4]>,
}

impl Ctx {
    fn new(instance: &Instance) -> Self {
        let w = usize::from(instance.width);
        let h = usize::from(instance.height);
        let rots = instance
            .pieces
            .iter()
            .map(|(_, p)| [p.rotated(0), p.rotated(1), p.rotated(2), p.rotated(3)])
            .collect();
        Self { w, h, n: w * h, rots }
    }
}

struct Search<'a> {
    ctx: &'a Ctx,
    /// Placed edge quads, row-major; None = empty.
    cells: Vec<Option<[u8; 4]>>,
    /// Placed (piece, rot), parallel to `cells` (to rebuild a Board).
    assign: Vec<Option<(u16, u8)>>,
    used: Vec<bool>,
    /// S(c): half-edge supply over unused pieces (rotation-invariant multiset).
    supply: [i64; MAXC],
    ledger_on: bool,
    nodes: u64,
    fires: u64,
    fires_deficit: u64,
    fires_parity: u64,
    completions: u64,
    first_completion: Option<Board>,
    deadline: Instant,
    timed_out: bool,
}

impl<'a> Search<'a> {
    /// Build search state from a row-major-contiguous prefix board.
    fn from_prefix(ctx: &'a Ctx, instance: &Instance, start: &Board) -> (Self, usize) {
        let mut cells: Vec<Option<[u8; 4]>> = vec![None; ctx.n];
        let mut assign: Vec<Option<(u16, u8)>> = vec![None; ctx.n];
        let mut used = vec![false; instance.pieces.len()];
        let mut supply = [0i64; MAXC];
        for (pid, p) in instance.pieces.iter() {
            if start_piece_pos(start, ctx.n, pid).is_none() {
                for &c in &p.edges {
                    supply[usize::from(c)] += 1;
                }
            } else {
                used[usize::from(pid)] = true;
            }
        }
        for pos in 0..ctx.n {
            if let Some((pid, r)) = start.piece_at(pos) {
                cells[pos] = Some(ctx.rots[usize::from(pid)][usize::from(r)]);
                assign[pos] = Some((pid, r));
            }
        }
        let first_empty = (0..ctx.n).find(|&p| cells[p].is_none()).unwrap_or(ctx.n);
        assert!(
            (first_empty..ctx.n).all(|p| cells[p].is_none()),
            "start board must be a row-major-contiguous prefix (non-contiguous \
             prefixes are a phase-2 item, see PLAN.md)"
        );
        (
            Self {
                ctx,
                cells,
                assign,
                used,
                supply,
                ledger_on: false,
                nodes: 0,
                fires: 0,
                fires_deficit: 0,
                fires_parity: 0,
                completions: 0,
                first_completion: None,
                deadline: Instant::now() + Duration::from_secs(3600),
                timed_out: false,
            },
            first_empty,
        )
    }

    /// The two LEDGER necessary conditions at the node about to expand `pos`,
    /// with `r` charged breaks still available. Recomputed from scratch: sound
    /// and fire-identical to an incremental ledger, just slower per node.
    /// Hint-free reproduction, so the free-junction term u is 0.
    fn ledger_ok(&self, pos: usize, r: u32) -> bool {
        let (deficit, odd) = self.ledger_eval(pos);
        deficit <= i64::from(r) && odd <= 2 * r
    }

    /// The raw ledger quantities at the node about to expand `pos`: the total
    /// colour deficit sum_c max(0, D(c) - S(c)) and the count of colours with
    /// odd S(c) - D(c).
    fn ledger_eval(&self, pos: usize) -> (i64, u32) {
        let (w, h) = (self.ctx.w, self.ctx.h);
        let mut demand = [0i64; MAXC];
        for e in pos..self.ctx.n {
            let (x, y) = (e % w, e / w);
            if y == 0 {
                demand[0] += 1;
            } else if let Some(q) = self.cells[e - w] {
                demand[usize::from(q[2])] += 1;
            }
            if x == w - 1 {
                demand[0] += 1;
            } else if let Some(q) = self.cells[e + 1] {
                demand[usize::from(q[3])] += 1;
            }
            if y == h - 1 {
                demand[0] += 1;
            } else if let Some(q) = self.cells[e + w] {
                demand[usize::from(q[0])] += 1;
            }
            if x == 0 {
                demand[0] += 1;
            } else if let Some(q) = self.cells[e - 1] {
                demand[usize::from(q[1])] += 1;
            }
        }
        let mut deficit = 0i64;
        let mut odd = 0u32;
        for c in 0..MAXC {
            let f = self.supply[c] - demand[c];
            if f < 0 {
                deficit -= f;
            }
            if f & 1 != 0 {
                odd += 1;
            }
        }
        (deficit, odd)
    }

    /// Exhaustive break-budget DFS from `pos`: counts every completion using at
    /// most `r` further charged breaks. Breaks are charged per edge (top/left
    /// junction mismatches); all four rim edges are hard (proper frame), so
    /// this is the per-edge, no-per-cell-cap convention (see PLAN.md scope
    /// note vs the vault engine's relaxed buckets).
    fn dfs(&mut self, pos: usize, r: u32) {
        if pos == self.ctx.n {
            self.completions += 1;
            if self.first_completion.is_none() {
                let mut b = Board::new();
                for (p, a) in self.assign.iter().enumerate() {
                    if let Some((pid, rot)) = a {
                        b.place(p, *pid, *rot);
                    }
                }
                self.first_completion = Some(b);
            }
            return;
        }
        self.nodes += 1;
        if self.nodes & 0x3FFF == 0 && Instant::now() >= self.deadline {
            self.timed_out = true;
        }
        if self.timed_out {
            return;
        }
        if self.ledger_on {
            let (deficit, odd) = self.ledger_eval(pos);
            let deficit_fail = deficit > i64::from(r);
            let parity_fail = odd > 2 * r;
            if deficit_fail || parity_fail {
                self.fires += 1;
                if deficit_fail {
                    self.fires_deficit += 1;
                }
                if parity_fail {
                    self.fires_parity += 1;
                }
                return;
            }
        }
        let (w, h) = (self.ctx.w, self.ctx.h);
        let (x, y) = (pos % w, pos / w);
        // In row-major order with a contiguous prefix, the up/left neighbours
        // of the frontier cell are always placed (unless on the rim). URDL:
        // the neighbour above exposes its Down edge (q[2]); the neighbour to
        // the left exposes its Right edge (q[1]).
        let up = if y > 0 { self.cells[pos - w].map(|q| q[2]) } else { None };
        let left = if x > 0 { self.cells[pos - 1].map(|q| q[1]) } else { None };
        for pid in 0..self.ctx.rots.len() {
            if self.used[pid] {
                continue;
            }
            for rot in 0..4u8 {
                let e = self.ctx.rots[pid][usize::from(rot)];
                // Proper frame: an edge is grey exactly when it lies on the rim.
                if (y == 0) != (e[0] == 0)
                    || (x == w - 1) != (e[1] == 0)
                    || (y == h - 1) != (e[2] == 0)
                    || (x == 0) != (e[3] == 0)
                {
                    continue;
                }
                let mut b = 0u32;
                if let Some(c) = up {
                    if e[0] != c {
                        b += 1;
                    }
                }
                if let Some(c) = left {
                    if e[3] != c {
                        b += 1;
                    }
                }
                if b > r {
                    continue;
                }
                self.cells[pos] = Some(e);
                self.assign[pos] = Some((pid as u16, rot));
                self.used[pid] = true;
                for &c in &self.ctx.rots[pid][0] {
                    self.supply[usize::from(c)] -= 1;
                }
                self.dfs(pos + 1, r - b);
                for &c in &self.ctx.rots[pid][0] {
                    self.supply[usize::from(c)] += 1;
                }
                self.used[pid] = false;
                self.cells[pos] = None;
                self.assign[pos] = None;
                if self.timed_out {
                    return;
                }
            }
        }
    }
}

/// Where (if anywhere) `pid` sits on `start`.
fn start_piece_pos(start: &Board, n: usize, pid: u16) -> Option<usize> {
    (0..n).find(|&p| start.piece_at(p).map(|(q, _)| q) == Some(pid))
}

/// The kit-facing skeleton: LEDGER break-budget DFS as a [`Solver`]. Handed a
/// row-major-contiguous prefix as the start board, it exhausts the suffix
/// within `breaks` charged mismatches and returns the first completion found
/// (or the untouched start when the space is empty), with the honest outcome
/// kind: `Exhausted` when the search ran to completion, `Improved` on a budget
/// cut-off.
struct LedgerBreakDfs {
    breaks: u32,
    ledger_on: bool,
}

impl Solver for LedgerBreakDfs {
    fn name(&self) -> String {
        if self.ledger_on { "ledger-break-dfs".into() } else { "noprune-break-dfs".into() }
    }

    fn solve(&mut self, instance: &Instance, start: &Board, budget: Budget) -> SolveOutcome {
        let ctx = Ctx::new(instance);
        let (mut s, first_empty) = Search::from_prefix(&ctx, instance, start);
        s.ledger_on = self.ledger_on;
        // Respect the kit budget: poll via a deadline derived from it.
        s.deadline = Instant::now() + Duration::from_secs_f64(budget.remaining_secs());
        s.dfs(first_empty, self.breaks);
        let board = s.first_completion.clone().unwrap_or_else(|| start.clone());
        let out = if s.timed_out {
            SolveOutcome::improved(board)
        } else {
            SolveOutcome::exhausted(board)
        };
        out.with_nodes(s.nodes)
    }
}

/// A generated official-shaped instance plus its known solution as cell codes
/// and the solution's eternity2.dev URL.
fn generated_with_solution(seed: u32) -> (Instance, Vec<i32>, String) {
    let puzzle = generator::generate_framed(16, 22, seed, true);
    let instance = instance_from_generated(&format!("gen-16x16-c22-s{seed}"), &puzzle);
    let solved = generator::generate_solved_framed(16, 22, seed, true);
    let out = instance.match_board(&solved.pieces);
    assert_eq!(out.breaks, 0, "recovered solution must be perfect");
    println!("instance gen-16x16-c22-s{seed}  solution: {}/480", out.score);
    println!("solution board: {}", out.url);
    (instance, out.board, out.url)
}

fn prefix_board(codes: &[i32], keep: usize) -> Board {
    let mut b = Board::new();
    for (pos, &code) in codes.iter().enumerate().take(keep) {
        if code >= 0 {
            b.place(pos, (code / 4) as u16, (code % 4) as u8);
        }
    }
    b
}

/// Gate: replay the perfect tail at zero slack. At every prefix depth the
/// remaining budget along the real line is 0, so any ledger fire is unsound.
fn run_soundness(seed: u32) -> (usize, u32, String) {
    let (instance, codes, url) = generated_with_solution(seed);
    let ctx = Ctx::new(&instance);
    let mut fires = 0u32;
    for depth in 0..=ctx.n {
        let start = prefix_board(&codes, depth);
        let (s, pos) = Search::from_prefix(&ctx, &instance, &start);
        assert_eq!(pos, depth);
        if !s.ledger_ok(depth, 0) {
            fires += 1;
            println!("UNSOUND: ledger fired at depth {depth} with r=0 on the real line");
        }
    }
    println!(
        "soundness replay seed {seed}: {} depths judged, {fires} fires (expected 0) -> {}",
        ctx.n + 1,
        if fires == 0 { "PASS" } else { "FAIL" }
    );
    (ctx.n + 1, fires, url)
}

/// The A/B race: exhaust the suffix twice, prune off and on. Completions must
/// match exactly; the node ratio is the reproduced quantity.
fn run_ab(seed: u32, suffix: usize, breaks: u32, cap_secs: f64) {
    let (instance, codes, _url) = generated_with_solution(seed);
    let ctx = Ctx::new(&instance);
    let keep = ctx.n - suffix;
    let start = prefix_board(&codes, keep);
    println!(
        "A/B suffix exhaustion: prefix D={keep}, suffix {suffix} cells, budget {breaks} breaks, cap {cap_secs}s/arm"
    );
    let mut rows = Vec::new();
    for ledger_on in [false, true] {
        let (mut s, pos) = Search::from_prefix(&ctx, &instance, &start);
        s.ledger_on = ledger_on;
        s.deadline = Instant::now() + Duration::from_secs_f64(cap_secs);
        let t0 = Instant::now();
        s.dfs(pos, breaks);
        let dt = t0.elapsed().as_secs_f64();
        println!(
            "  {}: nodes {}  completions {}  fires {}  {:.3}s{}",
            if ledger_on { "LEDGER " } else { "NoPrune" },
            s.nodes,
            s.completions,
            s.fires,
            dt,
            if s.timed_out { "  TIMEOUT (ratio not comparable)" } else { "" }
        );
        rows.push((s.nodes, s.completions, s.timed_out));
    }
    let (n0, c0, t0) = rows[0];
    let (n1, c1, t1) = rows[1];
    if !t0 && !t1 {
        assert_eq!(c0, c1, "arms disagree on completion count: prune is unsound or engine bug");
        println!(
            "  node ratio NoPrune/LEDGER = {:.2}x  (completions agree: {c0})",
            n0 as f64 / n1.max(1) as f64
        );
    }
}

/// The kit-integration path: the same search driven through the [`Solver`]
/// trait, continuing from a prefix board and finishing through the canonical
/// scorer, the way a sweep or a record re-measurement would consume it.
fn run_solver_demo(seed: u32, suffix: usize, breaks: u32, cap_secs: f64) {
    let (instance, codes, _url) = generated_with_solution(seed);
    let n = usize::from(instance.width) * usize::from(instance.height);
    let start = prefix_board(&codes, n - suffix);
    let mut solver = LedgerBreakDfs { breaks, ledger_on: true };
    let outcome = solver.solve(&instance, &start, Budget::seconds(cap_secs));
    let out = instance.finish(&outcome.board);
    println!("solver:  {}", solver.name());
    println!("score:   {} / {}  breaks {}", out.score, out.max_score, out.breaks);
    println!("outcome: {:?}  nodes {}", outcome.kind, outcome.nodes);
    println!("board:   {}", out.url);
}

/// One arm of an A/B cell. Node counts are only recorded when the arm ran to
/// exhaustion; a time-capped arm is censored (its partial count is
/// hardware-dependent and must not enter the exact-tier record).
struct Arm {
    nodes: u64,
    completions: u64,
    fires: u64,
    fires_deficit: u64,
    fires_parity: u64,
    censored: bool,
}

fn run_arm(
    ctx: &Ctx,
    instance: &Instance,
    start: &Board,
    ledger_on: bool,
    breaks: u32,
    cap_secs: f64,
) -> Arm {
    let (mut s, pos) = Search::from_prefix(ctx, instance, start);
    s.ledger_on = ledger_on;
    s.deadline = Instant::now() + Duration::from_secs_f64(cap_secs);
    s.dfs(pos, breaks);
    Arm {
        nodes: s.nodes,
        completions: s.completions,
        fires: s.fires,
        fires_deficit: s.fires_deficit,
        fires_parity: s.fires_parity,
        censored: s.timed_out,
    }
}

fn arm_json(a: &Arm, ledger_on: bool) -> String {
    if a.censored {
        // No numbers for a censored arm: the cut point is wall-clock-dependent.
        "{\"censored\":true}".to_string()
    } else if ledger_on {
        format!(
            "{{\"censored\":false,\"nodes\":{},\"completions\":{},\"fires\":{},\"fires_deficit\":{},\"fires_parity\":{}}}",
            a.nodes, a.completions, a.fires, a.fires_deficit, a.fires_parity
        )
    } else {
        format!(
            "{{\"censored\":false,\"nodes\":{},\"completions\":{}}}",
            a.nodes, a.completions
        )
    }
}

fn median(sorted: &[f64]) -> f64 {
    let n = sorted.len();
    if n % 2 == 1 { sorted[n / 2] } else { (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0 }
}

/// The PLAN's phase-1 grid: suffix {20,24,28,32} x budget {1,2,3} x seeds
/// 1..=`seeds`, 25 s per-arm cap by default. Writes two deterministic JSON
/// files (no timestamps, no wall-times): the soundness gate and the A/B sweep
/// with per-(K,R) ratio summaries over uncensored pairs.
fn run_sweep(seeds: u32, cap_secs: f64, out_dir: &str) {
    let suffixes = [20usize, 24, 28, 32];
    let budgets = [1u32, 2, 3];

    // Soundness gate over every seed first (cheap, and a hard precondition).
    let mut sound_rows = Vec::new();
    for seed in 1..=seeds {
        let (depths, fires, url) = run_soundness(seed);
        assert_eq!(fires, 0, "soundness gate failed on seed {seed}");
        sound_rows.push(format!(
            "    {{\"seed\":{seed},\"instance\":\"gen-16x16-c22-s{seed}\",\"depths_judged\":{depths},\"fires\":{fires},\"budget_r\":0,\"solution_url\":\"{url}\"}}"
        ));
    }
    let soundness_json = format!
        ("{{\n  \"gate\":\"replay each generated board's known perfect tail at zero slack; any ledger fire is unsound\",\n  \"result\":\"PASS\",\n  \"boards\":[\n{}\n  ]\n}}\n",
        sound_rows.join(",\n"));
    std::fs::write(format!("{out_dir}/soundness.json"), soundness_json).expect("write soundness");

    // The PLAN grid: full cross of suffixes x budgets. Cells run in a fixed
    // order (seed, then grid order) so the record is byte-stable.
    let plan_grid: Vec<(usize, u32)> = suffixes
        .iter()
        .flat_map(|&k| budgets.iter().map(move |&r| (k, r)))
        .collect();
    let (cells, summary) = run_grid(seeds, &plan_grid, cap_secs);
    write_ab_json(
        &format!("{out_dir}/ab-sweep.json"),
        "the PLAN phase-1 grid: suffix {20,24,28,32} x budget {1,2,3}",
        cap_secs,
        &summary,
        &cells,
    );

    // The vault-shaped diagonal: suffix and budget grow together, mirroring
    // the cited certificate rows (24/2, 32/5, 40/6, 48/8). This is where the
    // compounding claim lives; deep NoPrune arms may censor on the cap.
    let diag_grid: Vec<(usize, u32)> = vec![(24, 2), (32, 5), (40, 6), (48, 8)];
    let (cells, summary) = run_grid(seeds, &diag_grid, cap_secs);
    write_ab_json(
        &format!("{out_dir}/ab-diagonal.json"),
        "the vault-row-shaped diagonal: (suffix, budget) in {(24,2),(32,5),(40,6),(48,8)}, suffix and budget growing together as in the cited certificate rows",
        cap_secs,
        &summary,
        &cells,
    );

    // Wrong-regime null: LEDGER arm alone with a generous budget on a short
    // suffix; the reproduced quantity is the fire fraction (expected tiny).
    let mut null_rows = Vec::new();
    for seed in 1..=seeds {
        let (instance, codes, _url) = generated_with_solution(seed);
        let ctx = Ctx::new(&instance);
        let (k, r) = (20usize, 6u32);
        let start = prefix_board(&codes, ctx.n - k);
        let lg = run_arm(&ctx, &instance, &start, true, r, cap_secs);
        println!(
            "null seed {seed} K={k} R={r}: {}",
            if lg.censored { "censored".into() } else { format!("nodes {} fires {}", lg.nodes, lg.fires) }
        );
        let detail = if lg.censored {
            "\"censored\":true".to_string()
        } else {
            format!(
                "\"censored\":false,\"nodes\":{},\"fires\":{},\"fire_fraction\":{:.5}",
                lg.nodes,
                lg.fires,
                lg.fires as f64 / lg.nodes.max(1) as f64
            )
        };
        null_rows.push(format!(
            "    {{\"seed\":{seed},\"suffix_cells\":{k},\"budget_breaks\":{r},{detail}}}"
        ));
    }
    let null_json = format!(
        "{{\n  \"design\":\"wrong-regime null: LEDGER arm alone with a generous break budget (R=6 slack on a 20-cell perfect tail); the prune should almost never fire and its bookkeeping is a pure loss\",\n  \"per_arm_cap_secs\":{cap_secs},\n  \"cells\":[\n{}\n  ]\n}}\n",
        null_rows.join(",\n")
    );
    std::fs::write(format!("{out_dir}/wrong-regime-null.json"), null_json).expect("write null");
    println!("wrote {out_dir}/{{soundness,ab-sweep,ab-diagonal,wrong-regime-null}}.json");
}

/// Run an A/B grid over seeds 1..=`seeds`; returns (cell rows, summary rows)
/// as JSON fragments. Ratios and fire-type tallies aggregate uncensored pairs
/// only.
fn run_grid(seeds: u32, grid: &[(usize, u32)], cap_secs: f64) -> (Vec<String>, Vec<String>) {
    let mut cell_rows = Vec::new();
    let mut ratios: std::collections::BTreeMap<(usize, u32), Vec<f64>> = Default::default();
    let mut censored: std::collections::BTreeMap<(usize, u32), u32> = Default::default();
    let mut fire_split: std::collections::BTreeMap<(usize, u32), (u64, u64)> = Default::default();
    for seed in 1..=seeds {
        let (instance, codes, _url) = generated_with_solution(seed);
        let ctx = Ctx::new(&instance);
        for &(k, r) in grid {
            let keep = ctx.n - k;
            let start = prefix_board(&codes, keep);
            let np = run_arm(&ctx, &instance, &start, false, r, cap_secs);
            let lg = run_arm(&ctx, &instance, &start, true, r, cap_secs);
            if !np.censored && !lg.censored {
                assert_eq!(
                    np.completions, lg.completions,
                    "arms disagree at seed {seed} K={k} R={r}: unsound"
                );
                ratios
                    .entry((k, r))
                    .or_default()
                    .push(np.nodes as f64 / lg.nodes.max(1) as f64);
                let e = fire_split.entry((k, r)).or_default();
                e.0 += lg.fires_deficit;
                e.1 += lg.fires_parity;
            } else {
                *censored.entry((k, r)).or_default() += 1;
            }
            println!(
                "seed {seed} K={k} R={r}: NoPrune {} LEDGER {} {}",
                if np.censored { "censored".into() } else { np.nodes.to_string() },
                if lg.censored { "censored".into() } else { lg.nodes.to_string() },
                if np.censored || lg.censored { "(excluded)" } else { "" }
            );
            cell_rows.push(format!(
                "    {{\"seed\":{seed},\"suffix_cells\":{k},\"prefix_cells\":{keep},\"budget_breaks\":{r},\"noprune\":{},\"ledger\":{}}}",
                arm_json(&np, false),
                arm_json(&lg, true)
            ));
        }
    }
    let mut summary_rows = Vec::new();
    for &(k, r) in grid {
        let mut rs = ratios.get(&(k, r)).cloned().unwrap_or_default();
        rs.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
        let cens = censored.get(&(k, r)).copied().unwrap_or(0);
        let row = if rs.is_empty() {
            format!(
                "    {{\"suffix_cells\":{k},\"budget_breaks\":{r},\"pairs\":0,\"censored_pairs\":{cens}}}"
            )
        } else {
            let (fd, fp) = fire_split.get(&(k, r)).copied().unwrap_or((0, 0));
            format!(
                "    {{\"suffix_cells\":{k},\"budget_breaks\":{r},\"pairs\":{},\"censored_pairs\":{cens},\"ratio_min\":{:.2},\"ratio_median\":{:.2},\"ratio_max\":{:.2},\"fires_deficit\":{fd},\"fires_parity\":{fp}}}",
                rs.len(),
                rs[0],
                median(&rs),
                rs[rs.len() - 1]
            )
        };
        summary_rows.push(row);
    }
    (cell_rows, summary_rows)
}

fn write_ab_json(path: &str, grid_desc: &str, cap_secs: f64, summary: &[String], cells: &[String]) {
    let ab_json = format!(
        "{{\n  \"design\":\"fixed solution prefix, exhaust the suffix twice (prune off / prune on) under a charged-break budget; completions must agree; node ratio NoPrune/LEDGER is the reproduced quantity\",\n  \"grid\":\"{grid_desc}\",\n  \"convention\":\"per-edge charged breaks, no per-cell cap, proper frame, hint-free (u=0); breaks = 480 - matched_edges_score on a full board\",\n  \"per_arm_cap_secs\":{cap_secs},\n  \"censoring\":\"an arm that hit the cap is censored: its node count is hardware-dependent and the pair is excluded from ratios\",\n  \"summary\":[\n{}\n  ],\n  \"cells\":[\n{}\n  ]\n}}\n",
        summary.join(",\n"),
        cells.join(",\n")
    );
    std::fs::write(path, ab_json).expect("write ab json");
}

fn arg<T: std::str::FromStr>(name: &str, default: T) -> T {
    let args: Vec<String> = std::env::args().collect();
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn main() {
    let mode: String = arg("--mode", "all".to_string());
    let seed: u32 = arg("--seed", 1);
    let suffix: usize = arg("--suffix", 20);
    let breaks: u32 = arg("--budget", 1);
    let cap_secs: f64 = arg("--cap-secs", 25.0);
    match mode.as_str() {
        "soundness" => {
            run_soundness(seed);
        }
        "ab" => run_ab(seed, suffix, breaks, cap_secs),
        "solver" => run_solver_demo(seed, suffix, breaks, cap_secs),
        "sweep" => {
            let seeds: u32 = arg("--seeds", 8);
            let out: String = arg("--out", "../results".to_string());
            run_sweep(seeds, cap_secs, &out);
        }
        // The zero-slack diagonal: on a perfect generated tail the analogue of
        // the vault's "budget = the real tail's break count" is R = 0. This is
        // the regime where the vault's compounding lives.
        "zerodiag" => {
            let seeds: u32 = arg("--seeds", 8);
            let out: String = arg("--out", "../results".to_string());
            let grid: Vec<(usize, u32)> = vec![(24, 0), (32, 0), (40, 0), (48, 0)];
            let (cells, summary) = run_grid(seeds, &grid, cap_secs);
            write_ab_json(
                &format!("{out}/ab-zeroslack.json"),
                "the zero-slack diagonal: suffix {24,32,40,48} at budget 0, the perfect-tail analogue of the vault's zero-slack certificate rows",
                cap_secs,
                &summary,
                &cells,
            );
        }
        _ => {
            run_soundness(seed);
            run_ab(seed, suffix, breaks, cap_secs);
        }
    }
}
