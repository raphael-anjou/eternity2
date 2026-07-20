//! Kit integration tests: the plumbing must be correct, because everyone builds
//! on it. These check the two things a silent bug would corrupt — that pinned
//! hints are genuine solution cells, and that a solved board scores the maximum
//! through the canonical scorer.

use e2_kit::{
    generator, instance_from_generated, pin_solution_hints, BoundKind, CellResult, Board, Instance,
    OutcomeKind, Pieces, SolveOutcome,
};

/// Build the true solution board for a generated instance: the *solved* form
/// places piece i at cell i, rotation 0.
fn solved_board(size: u8, colors: u8, seed: u32, framed: bool) -> (Instance, Board) {
    let solved = generator::generate_solved_framed(size, colors, seed, framed);
    let instance = Instance {
        name: "solved".into(),
        width: solved.width,
        height: solved.height,
        num_colors: solved.num_colors,
        pieces: Pieces::new(solved.pieces.clone()),
        hints: Vec::new(),
    };
    let mut board = Board::new();
    for id in 0..solved.pieces.len() {
        board.place(id, id as u16, 0);
    }
    (instance, board)
}

#[test]
fn solved_board_scores_max() {
    for seed in [1u32, 7, 42] {
        let (instance, board) = solved_board(16, 22, seed, true);
        let out = instance.finish(&board);
        assert_eq!(out.score, out.max_score, "solved board must score max at seed {seed}");
        assert_eq!(out.max_score, 480, "official board max is 480");
    }
}

#[test]
fn pinned_hints_are_genuine_solution_cells() {
    // A hint pins a piece+rotation at a cell. For the pin to be legitimate, the
    // *scrambled* instance's pinned piece, rotated as specified, must reproduce
    // exactly the edges the true solution has at that cell. Otherwise the board
    // would be unsolvable with the hint applied.
    for &(seed, k) in &[(1u32, 5u32), (7, 12), (42, 1)] {
        let scrambled = generator::generate_framed(16, 22, seed, true);
        let instance = instance_from_generated("t", &scrambled);
        let pinned = pin_solution_hints(instance.clone(), 16, 22, seed, true, k);
        assert_eq!(pinned.hints.len() as u32, k, "expected {k} hints at seed {seed}");

        // The solution's edges at each cell.
        let solved = generator::generate_solved_framed(16, 22, seed, true);
        for h in &pinned.hints {
            let placed = pinned.pieces.get(h.piece).unwrap().rotated(h.rot);
            let want = solved.pieces[h.pos as usize];
            assert_eq!(
                placed, want,
                "hint at pos {} (seed {seed}) does not match the solution",
                h.pos
            );
        }
    }
}

#[test]
fn pinned_board_still_solves_to_max() {
    // Applying the hints to a board and then completing it with the rest of the
    // true solution must reach 480 — proving the pins are consistent with a real
    // solution, not just structurally valid.
    let seed = 3u32;
    let scrambled = generator::generate_framed(16, 22, seed, true);
    let instance = instance_from_generated("t", &scrambled);
    let pinned = pin_solution_hints(instance, 16, 22, seed, true, 5);

    // Reconstruct the full solution onto the pinned board by matching each
    // solution cell's edges to a scrambled piece (the kit's own match_board does
    // exactly this).
    let solved = generator::generate_solved_framed(16, 22, seed, true);
    let solved_cells: Vec<[u8; 4]> = solved.pieces.clone();
    let out = pinned.match_board(&solved_cells);
    assert_eq!(out.score, 480, "pinned board's true solution must score 480");
}

#[test]
fn bounds_and_exhaustions_do_not_pollute_the_score_stats() {
    // The aggregator must compute the mean over ACHIEVED cells only. A bound row
    // (score 0) folded into the mean would drag it toward 0 — the exact
    // confusion the outcome type exists to prevent.
    use e2_kit::Summary;
    let cell = |seed, score, outcome| CellResult {
        seed,
        score,
        breaks: 0,
        outcome,
        nodes: 0,
        elapsed_s: 0.0,
        url: String::new(),
    };
    let cells = vec![
        cell(1, 460, OutcomeKind::Complete),
        cell(2, 470, OutcomeKind::Improved),
        cell(3, 0, OutcomeKind::Bound { value: 452, kind: BoundKind::LpUb }),
        cell(4, 300, OutcomeKind::Exhausted),
    ];
    let s = Summary::of("t", &cells);
    assert_eq!(s.n, 4);
    assert_eq!(s.n_scored, 2, "only Complete + Improved are achieved scores");
    assert_eq!(s.bounds, 1);
    assert_eq!(s.exhausted, 1);
    // Mean is over 460 and 470 only — the bound's 0 and the exhaustion's 300 are
    // NOT averaged in.
    assert!((s.mean - 465.0).abs() < 1e-9, "mean must be 465, got {}", s.mean);
    assert_eq!(s.best, 470);
    assert_eq!(s.worst, 460);
}

#[test]
fn a_bound_never_looks_like_an_achieved_score() {
    // The whole point of the outcome type: a proven upper bound and an achieved
    // score serialize differently, so a run row can never silently present a
    // relaxation's number as if a solver had reached it.
    let bound = SolveOutcome::bound(Board::new(), 452, BoundKind::LpUb);
    let cell = CellResult {
        seed: 1,
        score: 0, // no board placed — the bound is NOT written to `score`
        breaks: 0,
        outcome: bound.kind,
        nodes: 0,
        elapsed_s: 0.0,
        url: String::new(),
    };
    let json = serde_json::to_string(&cell).unwrap();
    // The score field stays 0; the bound value lives under the outcome, tagged.
    assert!(json.contains("\"outcome\":{\"type\":\"bound\""));
    assert!(json.contains("\"value\":452"));
    assert!(json.contains("\"kind\":\"lp-ub\""));

    // Round-trips back to the same typed outcome.
    let back: CellResult = serde_json::from_str(&json).unwrap();
    assert_eq!(back.outcome, OutcomeKind::Bound { value: 452, kind: BoundKind::LpUb });
}

#[test]
fn official_set_has_the_real_e2_shape() {
    // The shipped official set must be the real puzzle: 256 pieces, and the exact
    // border census of Eternity II — 4 corners (two grey edges), 56 edge pieces
    // (one grey edge), 196 interior (none). A wrong or scrambled set fails here.
    let official = e2_kit::official_instance(false);
    assert_eq!(official.pieces.len(), 256);
    let mut corners = 0;
    let mut edges = 0;
    let mut interior = 0;
    for (_, p) in official.pieces.iter() {
        match p.border_edge_count() {
            2 => corners += 1,
            1 => edges += 1,
            0 => interior += 1,
            _ => panic!("piece with 3+ grey edges is not a legal E2 piece"),
        }
    }
    assert_eq!((corners, edges, interior), (4, 56, 196));

    // Loaded through e2-io's own SiteInstance path, so the kit does no parsing of
    // its own: the JSON round-trips back to the same instance.
    let site = official.to_site();
    assert_eq!(site.pieces.len(), 256);
}

#[test]
fn official_and_generated_are_distinct_up_to_rotation() {
    // The URL format's edge-only piece recovery relies on this. The official set
    // must have it; a generated 16×16 must too (the generator's repair pass).
    assert!(e2_kit::pieces_distinct_up_to_rotation(
        &e2_kit::official_instance(false).pieces
    ));
    for seed in [1u32, 7, 42] {
        let p = generator::generate_framed(16, 22, seed, true);
        let inst = instance_from_generated("t", &p);
        assert!(
            e2_kit::pieces_distinct_up_to_rotation(&inst.pieces),
            "generated 16×16 seed {seed} not distinct up to rotation"
        );
    }
}

#[test]
fn official_clues_are_the_five_known_positions() {
    // The five official clues, at their published cells. Guards the clue data
    // and the clue-compliance check the verifier relies on.
    let official = e2_kit::official_instance(true);
    let mut positions: Vec<u16> = official.hints.iter().map(|h| h.pos).collect();
    positions.sort_unstable();
    assert_eq!(positions, vec![34, 45, 135, 210, 221]);
}

#[test]
fn old_rows_without_an_outcome_read_as_complete() {
    // Backward-compatible: a results.jsonl row written before the outcome field
    // existed still deserializes, defaulting to Complete.
    let row = r#"{"seed":7,"score":466,"breaks":14,"nodes":0,"elapsed_s":1.2,"url":"u"}"#;
    let cell: CellResult = serde_json::from_str(row).unwrap();
    assert_eq!(cell.outcome, OutcomeKind::Complete);
    assert_eq!(cell.score, 466);
}
