//! beam-width-smc: A/B the beam survivor rule on planted ladder instances.
//!
//! Three rules at fixed width and wall clock (see ../PLAN.md):
//!   plain   deterministic top-k truncation (ties by candidate order)
//!   stoch   top-k with randomised tie-breaking among exactly tied scores
//!   smc     SMC / particle-filter resampling: survivors drawn without
//!           replacement with probability proportional to softmax(score / T)
//!
//!   cargo run --release --features size-12 -- --widths 1024 --rules plain,smc --seeds 1..48 --budget-s 3
//!
//! Emits one JSON row per (rule, width, seed) on stdout. Every board is
//! re-scored through the canonical rim-excluding scorer (`Instance::finish`);
//! the beam's internal edge count is never trusted. The planted optimum of a
//! rung is 2N(N-1) matched interior edges; before any run the solved board is
//! rebuilt and asserted to reach exactly that.
//!
//! Mechanism port, not a producer port: row-major layer beam, break-allowing
//! placement, rim colours hard, matched-edges truncation key. The vol-232
//! comb fill order and the corpus prior are 16x16 producer machinery and are
//! intentionally NOT here (out of scope for the ladder claim; see PLAN.md).

use e2_kit::{
    generator, instance_from_generated, pin_solution_hints, Board, Budget, Instance, SolveOutcome,
    Solver,
};

const HINTS: u32 = 5;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Rule {
    Plain,
    StochTie,
    Smc { temp: f64 },
}

impl Rule {
    fn name(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::StochTie => "stoch",
            Self::Smc { .. } => "smc",
        }
    }
}

/// Row-major layer beam with a pluggable survivor rule.
struct LayerBeam {
    width: usize,
    rule: Rule,
    seed: u64,
}

/// A frontier node: a partial board (filled in row-major order up to the
/// current layer), the used-piece mask, and its matched-interior-edge count.
struct Node {
    board: Board,
    used: Vec<bool>,
    score: u32,
}

/// A child candidate, kept as (parent, placement, score) so only survivors
/// pay for a board clone.
struct Cand {
    parent: u32,
    pid: u16,
    rot: u8,
    score: u32,
}

impl Solver for LayerBeam {
    fn name(&self) -> String {
        format!("beam-{}-w{}", self.rule.name(), self.width)
    }

    fn solve(&mut self, instance: &Instance, start: &Board, budget: Budget) -> SolveOutcome {
        let w = usize::from(instance.width);
        let n = w * usize::from(instance.height);
        let pieces = &instance.pieces;
        let mut rng = XorShift64::new(self.seed ^ 0x51CE_5EED);

        let mut used0 = vec![false; pieces.len()];
        for pos in 0..n {
            if let Some((pid, _)) = start.piece_at(pos) {
                used0[usize::from(pid)] = true;
            }
        }
        let mut frontier = vec![Node { board: start.clone(), used: used0, score: 0 }];
        let mut nodes_expanded = 0u64;

        for pos in 0..n {
            if budget.expired() {
                break;
            }
            // All frontier boards share the start board's occupancy at `pos`:
            // the beam fills strictly in row-major order.
            if !start.is_empty_at(pos) {
                // A pinned hint. Credit its matches against already-processed
                // neighbours so the truncation key stays the true edge count.
                for node in &mut frontier {
                    let (pid, rot) = node.board.piece_at(pos).expect("hint present");
                    let e = pieces.get(pid).expect("hint piece exists").rotated(rot);
                    node.score += matches_before(&node.board, pieces, pos, w, &e);
                }
                continue;
            }

            let mut cands: Vec<Cand> = Vec::new();
            for (i, node) in frontier.iter().enumerate() {
                for (pid, piece) in pieces.iter() {
                    if node.used[usize::from(pid)] {
                        continue;
                    }
                    for rot in 0..4u8 {
                        let e = piece.rotated(rot);
                        if !rim_ok(&e, pos, w, n) {
                            continue;
                        }
                        nodes_expanded += 1;
                        let gained = matches_before(&node.board, pieces, pos, w, &e);
                        cands.push(Cand {
                            parent: u32::try_from(i).expect("frontier fits u32"),
                            pid,
                            rot,
                            score: node.score + gained,
                        });
                    }
                }
            }
            if cands.is_empty() {
                break; // frontier death (piece class depleted); keep best-so-far
            }

            let survivors = self.truncate(&mut cands, &mut rng);
            frontier = survivors
                .into_iter()
                .map(|c| {
                    let parent = &frontier[c.parent as usize];
                    let mut board = parent.board.clone();
                    let mut used = parent.used.clone();
                    board.place(pos, c.pid, c.rot);
                    used[usize::from(c.pid)] = true;
                    Node { board, used, score: c.score }
                })
                .collect();
        }

        let best = frontier
            .into_iter()
            .max_by_key(|node| node.score)
            .expect("frontier starts non-empty");
        let full = (0..n).all(|pos| !best.board.is_empty_at(pos));
        let outcome = if full {
            SolveOutcome::complete(best.board)
        } else {
            SolveOutcome::improved(best.board)
        };
        outcome.with_nodes(nodes_expanded)
    }
}

impl LayerBeam {
    /// Apply the survivor rule: keep at most `self.width` candidates.
    fn truncate(&self, cands: &mut Vec<Cand>, rng: &mut XorShift64) -> Vec<Cand> {
        let width = self.width;
        if cands.len() <= width {
            return std::mem::take(cands);
        }
        let mut keyed: Vec<(u64, usize)> = match self.rule {
            // Deterministic: score desc, then candidate order. The candidate
            // order itself is deterministic (frontier order x piece id x rot),
            // so this is the vol-235 "determinism blindspot" baseline.
            Rule::Plain => (0..cands.len()).map(|i| (i as u64, i)).collect(),
            // Randomise ONLY among exactly tied scores (never widen the score
            // window; vol-246: tol-widening is catastrophic, tie-shuffling is
            // free and positive).
            Rule::StochTie => (0..cands.len()).map(|i| (rng.next(), i)).collect(),
            Rule::Smc { .. } => Vec::new(),
        };
        match self.rule {
            Rule::Plain | Rule::StochTie => {
                keyed.sort_by(|a, b| {
                    cands[b.1]
                        .score
                        .cmp(&cands[a.1].score)
                        .then(a.0.cmp(&b.0))
                });
                keyed.truncate(width);
                take_indices(cands, keyed.into_iter().map(|(_, i)| i))
            }
            Rule::Smc { temp } => {
                // Weighted sampling without replacement, weights softmax(score/T),
                // via Efraimidis-Spirakis keys: key = ln(u) / w, keep the top
                // `width` keys. Duplicates are impossible (each candidate drawn
                // at most once), so the frontier stays deduplicated for free.
                let max = cands.iter().map(|c| c.score).max().unwrap_or(0);
                let mut es: Vec<(f64, usize)> = cands
                    .iter()
                    .enumerate()
                    .map(|(i, c)| {
                        let wgt = ((f64::from(c.score) - f64::from(max)) / temp).exp();
                        let u = rng.uniform01().max(f64::MIN_POSITIVE);
                        (u.ln() / wgt, i)
                    })
                    .collect();
                es.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
                es.truncate(width);
                take_indices(cands, es.into_iter().map(|(_, i)| i))
            }
        }
    }
}

/// Move the candidates at `indices` out of `cands` (order of `indices`).
fn take_indices(cands: &mut Vec<Cand>, indices: impl Iterator<Item = usize>) -> Vec<Cand> {
    let mut picked: Vec<usize> = indices.collect();
    picked.sort_unstable();
    let mut out = Vec::with_capacity(picked.len());
    // Drain from the back so earlier indices stay valid.
    for &i in picked.iter().rev() {
        out.push(cands.swap_remove(i));
    }
    out
}

/// Matched edges the piece with edges `e` at `pos` gains against neighbours at
/// already-processed cells (row-major index < pos). Each interior adjacency is
/// counted exactly once, by the later cell of the pair, so the running sum
/// equals the canonical rim-excluding score of the finished board.
fn matches_before(board: &Board, pieces: &e2_kit::Pieces, pos: usize, w: usize, e: &[u8; 4]) -> u32 {
    let (x, y) = (pos % w, pos / w);
    let mut m = 0;
    if y > 0 {
        if let Some((pid, r)) = board.piece_at(pos - w) {
            let ne = pieces.get(pid).expect("neighbour exists").rotated(r);
            m += u32::from(ne[2] == e[0]); // their down vs our up
        }
    }
    if x > 0 {
        if let Some((pid, r)) = board.piece_at(pos - 1) {
            let ne = pieces.get(pid).expect("neighbour exists").rotated(r);
            m += u32::from(ne[1] == e[3]); // their right vs our left
        }
    }
    m
}

/// Rim edges (colour 0) are HARD: a rim side must carry 0, a non-rim side must
/// not. Interior mismatches are allowed (the beam places breaks); only the
/// grey border is inviolable, matching the producer convention.
fn rim_ok(e: &[u8; 4], pos: usize, w: usize, n: usize) -> bool {
    let h = n / w;
    let (x, y) = (pos % w, pos / w);
    let rim = [y == 0, x == w - 1, y == h - 1, x == 0]; // URDL
    rim.iter().zip(e.iter()).all(|(&is_rim, &c)| (c == 0) == is_rim)
}

struct XorShift64(u64);

impl XorShift64 {
    fn new(seed: u64) -> Self {
        Self(seed.max(1))
    }
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    #[allow(clippy::cast_precision_loss)]
    fn uniform01(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

// ---------------------------------------------------------------------------

struct Args {
    widths: Vec<usize>,
    rules: Vec<String>,
    seeds: Vec<u32>,
    budget_s: f64,
    colors: u8,
    temp: f64,
}

fn parse_args() -> Args {
    let mut widths = vec![1024];
    let mut rules = vec!["plain".to_string(), "smc".to_string()];
    let mut seeds: Vec<u32> = (1..=8).collect();
    let mut budget_s = 3.0;
    let mut colors = 22u8;
    let mut temp = 1.0;
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i + 1 < argv.len() + 1 {
        let Some(flag) = argv.get(i) else { break };
        let val = argv.get(i + 1).cloned().unwrap_or_default();
        match flag.as_str() {
            "--widths" => widths = val.split(',').filter_map(|s| s.parse().ok()).collect(),
            "--rules" => rules = val.split(',').map(str::to_string).collect(),
            "--seeds" => seeds = parse_seeds(&val),
            "--budget-s" => budget_s = val.parse().unwrap_or(budget_s),
            "--colors" => colors = val.parse().unwrap_or(colors),
            "--temp" => temp = val.parse().unwrap_or(temp),
            other => {
                eprintln!("unknown flag {other}");
                std::process::exit(2);
            }
        }
        i += 2;
    }
    Args { widths, rules, seeds, budget_s, colors, temp }
}

/// `1..48` (inclusive range) or `1,2,3`.
fn parse_seeds(s: &str) -> Vec<u32> {
    if let Some((a, b)) = s.split_once("..") {
        let (a, b) = (a.parse().unwrap_or(1), b.parse().unwrap_or(1));
        (a..=b).collect()
    } else {
        s.split(',').filter_map(|t| t.parse().ok()).collect()
    }
}

/// Build one ladder rung: a planted framed W x W instance with 5 solution
/// cells pinned, plus the proven ceiling 2N(N-1), asserted by re-scoring the
/// planted solved board through the canonical scorer.
fn build_rung(size: u8, colors: u8, seed: u32) -> (Instance, u32) {
    let colors = colors.min(generator::max_colors(size));
    let name = format!("smc-n{size}-c{colors}-s{seed}");
    let puzzle = generator::generate_framed(size, colors, seed, true);
    let instance = instance_from_generated(&name, &puzzle);
    let instance = pin_solution_hints(instance, size, colors, seed, true, HINTS);

    let n = u32::from(size);
    let ceiling = 2 * n * (n - 1);
    let solved = generator::generate_solved_framed(size, colors, seed, true);
    let solved_instance = instance_from_generated(&name, &solved);
    let mut solved_board = Board::new();
    for pos in 0..usize::from(size) * usize::from(size) {
        solved_board.place(pos, u16::try_from(pos).expect("pos fits u16"), 0);
    }
    let out = solved_instance.finish(&solved_board);
    assert_eq!(
        out.score, ceiling,
        "planted board must score exactly 2N(N-1) (vol-240 planted-optimality lemma)"
    );
    (instance, ceiling)
}

fn main() {
    let args = parse_args();
    let size = u8::try_from(e2_core::W).expect("board size fits u8");

    for &width in &args.widths {
        for rule_name in &args.rules {
            let rule = match rule_name.as_str() {
                "plain" => Rule::Plain,
                "stoch" => Rule::StochTie,
                "smc" => Rule::Smc { temp: args.temp },
                other => {
                    eprintln!("unknown rule {other} (use plain|stoch|smc)");
                    std::process::exit(2);
                }
            };
            for &seed in &args.seeds {
                let (instance, ceiling) = build_rung(size, args.colors, seed);
                let mut solver = LayerBeam { width, rule, seed: u64::from(seed) };
                let start = instance.seed_board();
                let outcome = solver.solve(&instance, &start, Budget::seconds(args.budget_s));
                let out = instance.finish(&outcome.board);
                let row = serde_json::json!({
                    "solver": solver.name(),
                    "rule": rule.name(),
                    "width": width,
                    "n": size,
                    "seed": seed,
                    "budget_s": args.budget_s,
                    "score": out.score,
                    "opt": ceiling,
                    "ratio": f64::from(out.score) / f64::from(ceiling),
                    "nodes": outcome.nodes,
                    "outcome": outcome.kind,
                    "url": out.url,
                });
                println!("{row}");
            }
        }
    }
}
