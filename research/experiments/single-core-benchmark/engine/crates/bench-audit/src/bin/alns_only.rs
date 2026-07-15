// replay ALNS on a saved Blackwood CP board.
//
// Saves the 5 minutes of CP each experiment, lets us iterate ALNS
// configs at 2× throughput. Loads a CP-board JSON (the existing
// `blackwood_raw_cp_board.json` format) and runs the same ops set
// as run_e2_blackwood.
//
// CLI:
//   alns_only --cp-board <path> --alns-budget-ms <N> --seed <N> [--ops <preset>]
//             [--chains N]
//
// --chains N (default 1) runs N independent ALNS chains from the same start
// board in parallel and keeps the best. ALNS is one time-budgeted chain with no
// restart loop, so best-of-N-chains is the parallelism available to it; each
// chain gets the full --alns-budget-ms, so N chains cost ~1 chain's wall time on
// N cores.
//
// Ops presets:
//   --ops minimal   = RandomRegion{4}, WorstWindow{5}, ConflictDriven{30}, MwpmDefectPair{12}
//   --ops basic     = + WorstBand{4} (the 455-record set)
//   --ops full      = + ConflictDriven{80}, WorstBand{6}, ComponentDestroy,
//                       ComponentPlusHaloDestroy, WorstRow, HingeDestroy
//   --ops wbonly    = WorstBand{4} only
//   --ops cdonly    = ConflictDriven{80} only

#![forbid(unsafe_code)]

use std::path::PathBuf;
use std::time::Instant;

use eternity2_bench_audit::{placed_count, score_board_dense as score_board};
use eternity2_benchmark::loader::load_puzzle_with_hints;
use eternity2_benchmark::report::bucas_url;
use eternity2_core::{Board, Rotation};
use eternity2_localsearch::{
    piece_swap_hillclimb, polish_rotations, run_alns, run_alns_portfolio, Acceptance, AlnsConfig,
    BottomBandDestroy, ComponentClusterDestroy, ComponentDestroy, ComponentPlusHaloDestroy, ConflictDriven, DestroyOp,
    HalfBoardDestroy, HingeDestroy, LkhChainDestroy, MegaBand, MwpmDefectPair, PriorDestroy, RandomRegion, RandomScatter,
    RepairKind, SigmaCycleDestroy, WorstBand, WorstColumn, WorstColumnBand, WorstRow, WorstWindow,
};

/// Load V155 prior matrix from JSON: `{ "matrix": [[u16; N_pos]; N_pieces], ... }`.
fn load_prior_matrix(path: &std::path::Path) -> Vec<Vec<u16>> {
    let raw = std::fs::read_to_string(path).expect("read prior matrix");
    let v: serde_json::Value = serde_json::from_str(&raw).expect("parse prior matrix");
    let arr = v.get("matrix").and_then(|x| x.as_array()).expect("prior.matrix array");
    arr.iter()
        .map(|row| {
            row.as_array()
                .expect("prior row array")
                .iter()
                .map(|x| x.as_u64().expect("prior cell u64") as u16)
                .collect::<Vec<u16>>()
        })
        .collect()
}

/// V169 — build PriorDestroy ops augmenting an existing preset.
///
/// Two complementary directions:
/// - ESCAPE (β>0): destroy corpus-anchored cells to break out of corpus
///   basin. For the 460 plateau on V155→ALNS boards. Highest leverage.
/// - ATTRACT (β<0): destroy corpus-unsupported (weak) cells to pull
///   board back to corpus. For diversified boards that should refine
///   within the basin.
///
/// Variants returned:
///   prior_escape_b0.5_k16   — gentle escape pull, larger region.
///   prior_escape_b2.0_k12   — sharp escape pull.
///   prior_attract_b-1.0_k8  — gentle attract.
fn build_prior_ops(
    prior: &[Vec<u16>],
    pinned: &std::collections::BTreeSet<eternity2_core::Position>,
) -> Vec<Box<dyn DestroyOp>> {
    vec![
        Box::new(PriorDestroy {
            support: prior.to_vec(),
            beta: 0.5,
            max_size: 16,
            pinned: pinned.clone(),
            variant: "prior_escape_b0.5_k16",
        }),
        Box::new(PriorDestroy {
            support: prior.to_vec(),
            beta: 2.0,
            max_size: 12,
            pinned: pinned.clone(),
            variant: "prior_escape_b2.0_k12",
        }),
        Box::new(PriorDestroy {
            support: prior.to_vec(),
            beta: -1.0,
            max_size: 8,
            pinned: pinned.clone(),
            variant: "prior_attract_b-1.0_k8",
        }),
        // V179 LARGE-K variants — escape iso-460 basin via huge destroy.
        Box::new(PriorDestroy {
            support: prior.to_vec(),
            beta: 1.0,
            max_size: 32,
            pinned: pinned.clone(),
            variant: "prior_huge_b1.0_k32",
        }),
        Box::new(PriorDestroy {
            support: prior.to_vec(),
            beta: 4.0,
            max_size: 48,
            pinned: pinned.clone(),
            variant: "prior_huge_b4.0_k48",
        }),
    ]
}

fn build_ops_with_oracle(preset: &str, oracle: Option<&Board>) -> Vec<Box<dyn DestroyOp>> {
    match preset {
        "sigma_only" => {
            let oracle = oracle.expect("sigma_only preset requires --oracle");
            vec![
                Box::new(SigmaCycleDestroy {
                    oracle: oracle.clone(),
                    min_size: 8,
                    max_cells: 60,
                    halo: 0,
                }),
            ]
        }
        "sigma_halo" => {
 // T1.b — try σ-cycle + 1-cell halo to unlock the
            // surrounding pieces' rotations.
            let oracle = oracle.expect("sigma_halo preset requires --oracle");
            vec![
                Box::new(SigmaCycleDestroy {
                    oracle: oracle.clone(),
                    min_size: 8,
                    max_cells: 60,
                    halo: 1,
                }),
            ]
        }
        "winning5_sigma" => {
            let oracle = oracle.expect("winning5_sigma preset requires --oracle");
            vec![
                Box::new(RandomRegion { k: 4 }),
                Box::new(WorstWindow { k: 5 }),
                Box::new(ConflictDriven { max_size: 30 }),
                Box::new(ConflictDriven { max_size: 80 }),
                Box::new(MwpmDefectPair { max_pairs: 12 }),
                Box::new(WorstBand { k_rows: 4 }),
                Box::new(SigmaCycleDestroy {
                    oracle: oracle.clone(),
                    min_size: 10,
                    max_cells: 80,
                    halo: 0,
                }),
                Box::new(SigmaCycleDestroy {
                    oracle: oracle.clone(),
                    min_size: 20,
                    max_cells: 40,
                    halo: 1,
                }),
            ]
        }
        _ => build_ops(preset),
    }
}

fn build_ops(preset: &str) -> Vec<Box<dyn DestroyOp>> {
    match preset {
        "minimal" => vec![
            Box::new(RandomRegion { k: 4 }),
            Box::new(WorstWindow { k: 5 }),
            Box::new(ConflictDriven { max_size: 30 }),
            Box::new(MwpmDefectPair { max_pairs: 12 }),
        ],
        "basic" => vec![
            Box::new(RandomRegion { k: 4 }),
            Box::new(WorstWindow { k: 5 }),
            Box::new(ConflictDriven { max_size: 30 }),
            Box::new(MwpmDefectPair { max_pairs: 12 }),
            Box::new(WorstBand { k_rows: 4 }),
        ],
        "basic_lkh" => vec![
 // W4 — basic + LKH-chain operator (adaptive K=10-15).
            Box::new(RandomRegion { k: 4 }),
            Box::new(WorstWindow { k: 5 }),
            Box::new(ConflictDriven { max_size: 30 }),
            Box::new(MwpmDefectPair { max_pairs: 12 }),
            Box::new(WorstBand { k_rows: 4 }),
            Box::new(LkhChainDestroy { max_size: 12, seed_worst: false }),
            Box::new(LkhChainDestroy { max_size: 16, seed_worst: true }),
        ],
        "lkh_only" => vec![
            Box::new(LkhChainDestroy { max_size: 12, seed_worst: false }),
            Box::new(LkhChainDestroy { max_size: 16, seed_worst: true }),
            Box::new(LkhChainDestroy { max_size: 20, seed_worst: true }),
        ],
        "winning5" => vec![
            // The 5-op set with ConflictDriven{80} added + WB — this is
            // the configuration that hit 455 on seed 1 with SA-primary.
            Box::new(RandomRegion { k: 4 }),
            Box::new(WorstWindow { k: 5 }),
            Box::new(ConflictDriven { max_size: 30 }),
            Box::new(ConflictDriven { max_size: 80 }),
            Box::new(MwpmDefectPair { max_pairs: 12 }),
            Box::new(WorstBand { k_rows: 4 }),
        ],
        "diverse" => vec![
 // winning5 + BottomBandDestroy to inject diversity in
            // the perfect zone. Tests H15: forcing bottom-row disturbance
            // breaks past 455 local optimum.
            Box::new(RandomRegion { k: 4 }),
            Box::new(WorstWindow { k: 5 }),
            Box::new(ConflictDriven { max_size: 30 }),
            Box::new(ConflictDriven { max_size: 80 }),
            Box::new(MwpmDefectPair { max_pairs: 12 }),
            Box::new(WorstBand { k_rows: 4 }),
            Box::new(BottomBandDestroy { k_rows: 3, first_row: 8 }),
            Box::new(BottomBandDestroy { k_rows: 4, first_row: 8 }),
        ],
        "full" => vec![
            Box::new(RandomRegion { k: 4 }),
            Box::new(WorstWindow { k: 5 }),
            Box::new(ConflictDriven { max_size: 30 }),
            Box::new(ConflictDriven { max_size: 80 }),
            Box::new(MwpmDefectPair { max_pairs: 12 }),
            Box::new(WorstBand { k_rows: 4 }),
            Box::new(WorstBand { k_rows: 6 }),
            Box::new(ComponentDestroy { max_size: 100, min_size: 6 }),
            Box::new(ComponentPlusHaloDestroy { max_size: 100, min_size: 6 }),
            Box::new(WorstRow),
            Box::new(HingeDestroy { halo: 1 }),
        ],
        "wbonly" => vec![Box::new(WorstBand { k_rows: 4 })],
        "cdonly" => vec![Box::new(ConflictDriven { max_size: 80 })],
        "componentonly" => vec![Box::new(ComponentDestroy { max_size: 100, min_size: 4 })],
        "componentonly2" => vec![
            Box::new(ComponentDestroy { max_size: 100, min_size: 2 }),
            Box::new(ComponentPlusHaloDestroy { max_size: 100, min_size: 2 }),
        ],
        "winning5_smallcomp" => vec![
            Box::new(RandomRegion { k: 4 }),
            Box::new(WorstWindow { k: 5 }),
            Box::new(ConflictDriven { max_size: 30 }),
            Box::new(ConflictDriven { max_size: 80 }),
            Box::new(MwpmDefectPair { max_pairs: 12 }),
            Box::new(WorstBand { k_rows: 4 }),
            Box::new(ComponentDestroy { max_size: 100, min_size: 2 }),
            Box::new(ComponentPlusHaloDestroy { max_size: 100, min_size: 2 }),
        ],
        "cluster_only" => vec![
            Box::new(ComponentClusterDestroy { cluster_radius: 4, halo: 1 }),
        ],
        "winning5_cluster" => vec![
 // invented op: ComponentClusterDestroy joins many small
            // co-located defect components into one destroy region. Tested
            // against winning5 on local 459 board.
            Box::new(RandomRegion { k: 4 }),
            Box::new(WorstWindow { k: 5 }),
            Box::new(ConflictDriven { max_size: 30 }),
            Box::new(ConflictDriven { max_size: 80 }),
            Box::new(MwpmDefectPair { max_pairs: 12 }),
            Box::new(WorstBand { k_rows: 4 }),
            Box::new(ComponentClusterDestroy { cluster_radius: 3, halo: 1 }),
            Box::new(ComponentClusterDestroy { cluster_radius: 5, halo: 1 }),
        ],
        "hingeonly" => vec![Box::new(HingeDestroy { halo: 1 })],
 // escape 457 operator-lock with fundamentally different
        // proposals. MegaBand{8,10,12} destroys 128-192 cells; WorstColumn
        // and column-band sample orthogonal to row-bands; HalfBoardDestroy
        // is brutal; RandomScatter explores non-contiguous patterns.
        "mega" => vec![
            Box::new(MegaBand { k_rows: 8 }),
            Box::new(MegaBand { k_rows: 10 }),
            Box::new(MegaBand { k_rows: 12 }),
            Box::new(WorstColumn),
            Box::new(WorstColumnBand { k_cols: 4 }),
            Box::new(RandomScatter { k: 60 }),
        ],
        "mega_mix" => vec![
 // combine winning5's reliable ops with mega-escape ops.
            // Gives ALNS adaptive weights the choice: small ops for
            // refinement, big ops for basin-escape.
            Box::new(RandomRegion { k: 4 }),
            Box::new(WorstWindow { k: 5 }),
            Box::new(ConflictDriven { max_size: 30 }),
            Box::new(ConflictDriven { max_size: 80 }),
            Box::new(MwpmDefectPair { max_pairs: 12 }),
            Box::new(WorstBand { k_rows: 4 }),
            Box::new(MegaBand { k_rows: 8 }),
            Box::new(MegaBand { k_rows: 12 }),
            Box::new(WorstColumn),
            Box::new(WorstColumnBand { k_cols: 4 }),
            Box::new(RandomScatter { k: 60 }),
            Box::new(HalfBoardDestroy { which: 0 }),
        ],
        "halfboard" => vec![
            Box::new(HalfBoardDestroy { which: 0 }),
            Box::new(HalfBoardDestroy { which: 1 }),
            Box::new(HalfBoardDestroy { which: 2 }),
            Box::new(HalfBoardDestroy { which: 3 }),
        ],
        other => panic!("unknown --ops preset {other}; want minimal|basic|winning5|full|mega|mega_mix|halfboard|wbonly|cdonly|componentonly|componentonly2|winning5_smallcomp|cluster_only|winning5_cluster|hingeonly"),
    }
}

fn load_cp_board(path: &std::path::Path) -> Board {
    let raw = std::fs::read_to_string(path).expect("read cp board");
    let v: serde_json::Value = serde_json::from_str(&raw).expect("parse cp board");
    let puzzle_path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/puzzles/size_16_official_eternity.csv"));
    let (puzzle, _) = load_puzzle_with_hints(&puzzle_path).expect("load puzzle");
    let mut b = Board::empty(&puzzle);
    if let Some(arr) = v.get("placement").and_then(|x| x.as_array()) {
        // Two supported formats:
        // (a) positional: [{piece_id,rotation} | null, ...] — pos = array index
        // (b) sparse:     [{pos,piece_id,rotation}, ...] — explicit pos field
        for (idx, p) in arr.iter().enumerate() {
            if p.is_null() { continue; }
            let pos = match p.get("pos").and_then(|x| x.as_u64()) {
                Some(v) => v as u32,
                None => idx as u32, // positional format
            };
            let pid = p["piece_id"].as_u64().unwrap() as u16;
            let rot_u = p["rotation"].as_u64().unwrap() as u8;
            let rot = Rotation::from_u8(rot_u).unwrap();
            b.place(pos, pid, rot);
        }
    }
    b
}

fn main() {
    let mut cp_board_path = PathBuf::new();
    let mut alns_ms: u64 = 300_000;
    let mut seed: u64 = 1;
    let mut ops_preset = "winning5".to_string();
    let mut repair_budget_ms: u64 = 1500;
    let mut repair_kind = "sa".to_string();
    let mut t: f64 = 1.0;
    let mut lex = false;
    let mut lex_intaglio = false;
    let mut repair_step_budget: u64 = 0;
    let mut cp_repair_parallel = true;
 // extra-hint support (mirrors vanilla_fast). These are
    // additional pinned positions beyond canonical hints. Format:
    // POS:PID:ROT per --extra-hint. Used by the corner-sweep experiment
    // to pin corner pieces during ALNS so the sweep's distinct
    // permutations are preserved through recovery.
    let mut extra_pins: Vec<u32> = Vec::new();
 // T1 — oracle board path for SigmaCycleDestroy.
    let mut oracle_path: Option<PathBuf> = None;
 // K11 — allow custom puzzle path (e.g. for hint-relaxation experiments).
    let mut custom_puzzle: Option<PathBuf> = None;
    // V169 — load a prior matrix to enable PriorDestroy ops appended to
    // the preset's destroy portfolio.
    let mut prior_path: Option<PathBuf> = None;
    // >1 runs that many independent ALNS chains from the same start
    // board (in parallel) and keeps the best. 1 = the historical single chain.
    let mut chains: usize = 1;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--cp-board" => cp_board_path = PathBuf::from(args.next().unwrap()),
            "--puzzle" => custom_puzzle = Some(PathBuf::from(args.next().unwrap())),
            "--alns-budget-ms" => alns_ms = args.next().unwrap().parse().unwrap(),
            "--seed" => seed = args.next().unwrap().parse().unwrap(),
            "--chains" => chains = args.next().unwrap().parse().unwrap(),
            "--ops" => ops_preset = args.next().unwrap(),
            "--oracle" => oracle_path = Some(PathBuf::from(args.next().unwrap())),
            "--repair-budget-ms" => repair_budget_ms = args.next().unwrap().parse().unwrap(),
            "--repair-kind" => repair_kind = args.next().unwrap(),
            "--t" => t = args.next().unwrap().parse().unwrap(),
            "--lex" => lex = true,
 // OPTIMIZATION_REPORT phase 0 — determinism knobs.
            "--repair-step-budget" => repair_step_budget = args.next().unwrap().parse().unwrap(),
            "--cp-repair-single" => cp_repair_parallel = false,
 // pin additional position(s). Format POS:PID:ROT
            // (PID:ROT are ignored — the partial's existing placement
            // at that position is honored; this just adds the position
            // to the pinned set so ALNS doesn't move that piece).
            "--extra-hint" => {
                let val = args.next().unwrap();
                let parts: Vec<&str> = val.split(':').collect();
                assert!(parts.len() >= 1, "--extra-hint format: POS[:PID:ROT]");
                let pos: u32 = parts[0].parse().expect("--extra-hint pos");
                extra_pins.push(pos);
            }
            "--prior-destroy" => prior_path = Some(PathBuf::from(args.next().unwrap())),
            "--lex-intaglio" => lex_intaglio = true,
            other => panic!("unknown arg {other}"),
        }
    }
    if cp_board_path.as_os_str().is_empty() {
        eprintln!("--cp-board PATH required");
        std::process::exit(1);
    }
    let puzzle_path = custom_puzzle.clone().unwrap_or_else(|| PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/puzzles/size_16_official_eternity.csv")));
    let (puzzle, hints) = load_puzzle_with_hints(&puzzle_path).expect("load");
    let cp_board = load_cp_board(&cp_board_path);
    let (cp_m, _) = score_board(&puzzle, &cp_board);
    let cp_p = placed_count(&cp_board, &puzzle);
    eprintln!(
        "loaded CP board: {} placed, {}/480 matched",
        cp_p, cp_m
    );
    eprintln!(
        "ALNS config: ops={ops_preset} repair_kind={repair_kind} repair_budget_ms={repair_budget_ms} t={t} budget={alns_ms}ms seed={seed}"
    );

 // T1 — load oracle board if --oracle was given.
    let oracle_board = oracle_path.as_ref().map(|p| {
        eprintln!("loading oracle from {}", p.display());
        load_cp_board(p)
    });
    // V169 — the prior matrix (if any) is loaded ONCE and shared; build_ops is a
    // factory because a portfolio needs one independent op set per chain
    // (DestroyOps carry mutable per-chain state, so chains cannot share them).
    let prior_matrix = prior_path.as_ref().map(|p| {
        eprintln!("loading prior matrix from {}", p.display());
        load_prior_matrix(p)
    });
    let prior_pinned: std::collections::BTreeSet<u32> = hints
        .hints
        .iter()
        .map(|h| h.position)
        .chain(extra_pins.iter().copied())
        .collect();
    // Named build_chain_ops, not build_ops: a free `build_ops(preset)` already
    // exists in this file and shadowing it here would be a trap for the next reader.
    let build_chain_ops = || {
        let mut ops = build_ops_with_oracle(&ops_preset, oracle_board.as_ref());
        // V169 — append PriorDestroy variants to the preset if a prior was given.
        if let Some(prior) = prior_matrix.as_ref() {
            for op in build_prior_ops(prior, &prior_pinned) {
                ops.push(op);
            }
        }
        ops
    };

    let mut ops = build_chain_ops();
    if prior_matrix.is_some() {
        eprintln!("appended PriorDestroy ops to preset (total {} ops)", ops.len());
    }

    let cfg = AlnsConfig {
        time_budget_ms: alns_ms,
        repair_budget_ms,
        acceptance: Acceptance::SimulatedAnnealing { t },
        segment_iters: 50,
        seed,
        verbose: false,
        repair: match repair_kind.as_str() {
            "sa" => RepairKind::Sa,
            "cp" => RepairKind::Cp,
            "ot" => RepairKind::IterativeOt,
            "jv" => RepairKind::IterativeJv,
            "jv_joint" | "jvjoint" | "jv-joint" => RepairKind::IterativeJvJoint,
            "filament" => RepairKind::Filament,
            other => panic!("--repair-kind want sa|cp|ot|jv|jv_joint|filament, got {other}"),
        },
        cp_fallback_to_sa: true,
        pinned_positions: hints.hints.iter().map(|h| h.position)
            .chain(extra_pins.iter().copied())
            .collect(),
        iter_budget: 0,
        lex_break_isoscore: lex,
        lex_break_intaglio: lex_intaglio,
        checkpoint_path: None,
        checkpoint_every_ms: 60_000,
        repair_step_budget,
        cp_repair_parallel,
    };

    let t0 = Instant::now();
    // ALNS is a single time-budgeted chain -- there is no restart loop
    // to fan out, and one chain cannot be split across cores without changing the
    // search. The available parallelism is INDEPENDENT CHAINS from the same start
    // board, keeping the best: --chains N. Each chain gets the same wall budget,
    // so N chains cost the same wall time as 1 on N cores and sample the
    // acceptance landscape N times instead of once.
    //
    // run_alns_portfolio (localsearch) already implements exactly this and is
    // already rayon-parallel; it simply had no caller here. --chains 1 (default)
    // keeps the direct single-chain call, unchanged.
    let (alns_board, stats) = if chains > 1 {
        eprintln!("# --chains {chains}: independent ALNS chains from the same start board, best-of. Each gets the full --alns-budget-ms.");
        let (b, per_chain) = run_alns_portfolio(
            &puzzle,
            &cp_board,
            |_i| build_chain_ops(),
            &cfg,
            chains,
            |_i| cfg.acceptance,
        );
        // Report the best chain's stats (the portfolio returns the best board).
        let best = per_chain
            .iter()
            .max_by_key(|(score, _)| *score)
            .expect("portfolio returns one entry per chain");
        eprintln!(
            "# chain scores: {:?} (best {})",
            per_chain.iter().map(|(s, _)| *s).collect::<Vec<_>>(),
            best.0
        );
        (b, best.1.clone())
    } else {
        run_alns(&puzzle, &cp_board, ops.as_mut_slice(), &cfg)
    };
    let elapsed = t0.elapsed();
    let pinned_set: std::collections::BTreeSet<u32> = hints.hints.iter().map(|h| h.position).collect();
    let (alns_board, rg) = polish_rotations(&puzzle, &alns_board, &pinned_set);
    let (alns_board, sg) = piece_swap_hillclimb(&puzzle, &alns_board, &pinned_set);

    let (am, _) = score_board(&puzzle, &alns_board);
    let ap = placed_count(&alns_board, &puzzle);
    let url = bucas_url(&puzzle, &alns_board, "v17_alns_only");

    eprintln!(
        "ALNS: elapsed={:.1}s iters={} placed={ap}/256 matched={am}/480 polish_rot=+{rg} polish_swap=+{sg}",
        elapsed.as_secs_f64(), stats.iters
    );
 // show when new bests were found.
    if !stats.best_score_history.is_empty() {
        eprintln!("  best_score_history: {} entries", stats.best_score_history.len());
        for (iter, score) in &stats.best_score_history {
            eprintln!("    iter={iter:>4}  new_best={score}");
        }
    }
    for (i, name) in stats.op_names.iter().enumerate() {
        let inv = stats.per_op_invocations[i];
        let acc = stats.per_op_accepts[i];
        let rate = if inv > 0 { 100.0 * acc as f64 / inv as f64 } else { 0.0 };
        eprintln!("  {name:<24}  inv={inv:>4}  acc={acc:>4}  rate={rate:>5.1}%");
    }
    eprintln!("bucas: {url}");
    let json = serde_json::json!({
        "matched": am, "placed": ap,
        "ops_preset": ops_preset, "repair_kind": repair_kind,
        "repair_budget_ms": repair_budget_ms, "t": t, "alns_ms": alns_ms,
        "polish_rot_gain": rg, "polish_swap_gain": sg,
        "bucas_url": url,
        "placement": (0..puzzle.cell_count()).map(|p| {
            alns_board.get(p).map(|(pid, rot)| serde_json::json!({
                "pos": p, "piece_id": u32::from(pid), "rotation": rot.as_u8(),
            }))
        }).collect::<Vec<_>>(),
    });
 // fix — include nanos + pid in run_id so concurrent ALNS
    // runs with the same (ops, repair, t, seed) don't overwrite each
    // other's output files. The earlier secs-only naming collided
    // when 8 parallel processes finished in the same second.
    let run_id_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let run_id_nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).map(|d| d.subsec_nanos()).unwrap_or(0);
    let pid = std::process::id();
    let out_dir = PathBuf::from("output/v17_alns_only");
    let _ = std::fs::create_dir_all(&out_dir);
    let p = out_dir.join(format!(
        "{ops_preset}_{repair_kind}_t{t}_s{seed}_{run_id_secs}_{run_id_nanos}_p{pid}.json"
    ));
    let _ = std::fs::write(&p, serde_json::to_string_pretty(&json).unwrap());
    eprintln!("saved: {}", p.display());
}
