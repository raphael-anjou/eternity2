//! Engine correctness tests. The load-bearing ones prove the two things a
//! repair loop can silently get wrong: that its *incrementally* maintained score
//! always equals the canonical scorer's (so the fast inner loop is honest), and
//! that the run's output is never worse than where it started and always keeps
//! the pinned hints.

use e2_core::{score_cells, Board, Pieces};
use e2_io::Instance;

use crate::registry::all_specs;
use crate::state::{Rng, State};
use crate::{find, run, RunConfig};

const VARIANT_00: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../variants/variant_00.json");

fn load() -> Instance {
    Instance::from_site_json(VARIANT_00).expect("load variant 00")
}

/// The canonical score of a state's current board, via the one true scorer.
fn canonical(st: &State, pieces: &Pieces) -> u32 {
    let board = Board::from_cell_codes(&st.to_codes());
    score_cells(&board.to_edge_cells(pieces))
}

#[test]
fn incremental_score_tracks_canonical_under_random_edits() {
    let inst = load();
    // Start from a greedy build so there is real structure to perturb.
    let mut rng = Rng::new(7);
    let codes = crate::StartBoard::GreedyConstruct.build(&inst, &mut rng, 7, 60000);
    let mut st = State::from_codes(&inst.pieces, &codes);
    assert_eq!(st.score(), canonical(&st, &inst.pieces), "score wrong at start");

    // Random place/clear churn: clear a cell, then place its piece back (possibly
    // rotated), asserting the incremental score matches the canonical each time.
    for _ in 0..2000 {
        let pos = rng.below(256);
        if let Some((pid, _)) = st.cell(pos) {
            if is_hint(&inst, pos) {
                continue;
            }
            st.clear(pos);
            let r = (rng.next_u64() % 4) as u8;
            st.place(pos, pid, r);
            assert_eq!(
                st.score(),
                canonical(&st, &inst.pieces),
                "incremental score diverged from canonical after edit at {pos}"
            );
        }
    }
}

#[test]
fn incremental_conflict_map_tracks_recompute_under_multi_cell_churn() {
    // The churn that actually exercises the cross-cell asymmetry: destroy a SET of
    // cells and refill them with a PERMUTED pool, exactly as the real loop does,
    // so a seam is placed from one cell and later cleared from the other. A
    // non-symmetric conflict rule leaks a counter here; this asserts the whole
    // incremental map (not just the score) stays equal to a from-scratch recompute.
    let inst = load();
    let mut rng = Rng::new(11);
    let codes = crate::StartBoard::GreedyConstruct.build(&inst, &mut rng, 7, 60000);
    let mut st = State::from_codes(&inst.pieces, &codes);
    assert!(st.matches_recompute(), "map wrong at start");

    for _ in 0..3000 {
        // Pick a handful of non-hint filled cells, lift them, refill with their
        // own pieces in a random permutation and rotation.
        let k = 2 + rng.below(10);
        let mut cells: Vec<usize> = Vec::new();
        while cells.len() < k {
            let p = rng.below(256);
            if !is_hint(&inst, p) && !st.is_empty_at(p) && !cells.contains(&p) {
                cells.push(p);
            }
        }
        let mut pool: Vec<u16> = cells.iter().filter_map(|&p| st.clear(p).map(|(pid, _)| pid)).collect();
        rng.shuffle(&mut pool);
        for (&p, &pid) in cells.iter().zip(pool.iter()) {
            st.place(p, pid, (rng.next_u64() % 4) as u8);
        }
        assert!(
            st.matches_recompute(),
            "incremental score/conflict map diverged from recompute after multi-cell churn"
        );
    }
}

fn is_hint(inst: &Instance, pos: usize) -> bool {
    inst.hints.iter().any(|h| h.pos as usize == pos)
}

#[test]
fn every_variant_runs_and_never_regresses_below_start() {
    let inst = load();
    for spec in all_specs() {
        let res = run(&inst, &spec, RunConfig::timed(300, 1));
        // The published score is the canonical re-score of the best board.
        let out = res.output(&inst);
        assert_eq!(out.score, res.best_score, "{}: output disagrees with best_score", spec.name);
        assert!(
            res.best_score >= res.stats.start_score,
            "{}: best {} fell below start {}",
            spec.name,
            res.best_score,
            res.stats.start_score
        );
    }
}

#[test]
fn every_output_board_is_a_valid_256_piece_permutation() {
    // The load-bearing integrity check: over a full run of destroy, repair,
    // accept and revert, the output board must still place each of the 256 pieces
    // exactly once with no empty cell. A double-placed piece would score fine on
    // edges yet be an invalid board, so this is what rules out the "impossible
    // score from a corrupted board" failure the accept/revert path could hide.
    let inst = load();
    for spec in all_specs() {
        let res = run(&inst, &spec, RunConfig::timed(400, 1));
        let mut seen = vec![false; inst.pieces.len()];
        let mut empties = 0;
        for &code in &res.best_codes {
            if code < 0 {
                empties += 1;
                continue;
            }
            let pid = (code / 4) as usize;
            assert!(!seen[pid], "{}: piece {pid} placed twice (invalid board)", spec.name);
            seen[pid] = true;
        }
        assert_eq!(empties, 0, "{}: {empties} empty cells in output board", spec.name);
        assert_eq!(
            seen.iter().filter(|&&b| b).count(),
            256,
            "{}: output board is not a full 256-piece permutation",
            spec.name
        );
    }
}

#[test]
fn hints_are_preserved_across_a_run() {
    let inst = load();
    for name in ["greedy-mismatch", "restart-kick", "band-destroy", "start-random"] {
        let spec = find(name).unwrap();
        let res = run(&inst, &spec, RunConfig::timed(300, 3));
        for h in &inst.hints {
            let code = res.best_codes[h.pos as usize];
            assert!(code >= 0, "{name}: hint cell {} became empty", h.pos);
            assert_eq!(
                (code / 4) as u16,
                h.piece,
                "{name}: hint piece at {} changed",
                h.pos
            );
        }
    }
}

#[test]
fn registry_names_are_unique_and_parents_resolve() {
    let specs = all_specs();
    let mut names: Vec<&str> = specs.iter().map(|s| s.name).collect();
    names.sort_unstable();
    let mut dedup = names.clone();
    dedup.dedup();
    assert_eq!(names.len(), dedup.len(), "duplicate variant name in registry");
    for s in &specs {
        if let Some(p) = s.parent {
            assert!(specs.iter().any(|q| q.name == p), "{}: parent {p} not in registry", s.name);
        }
    }
}

#[test]
fn greedy_start_beats_random_start_on_average() {
    // A sanity floor, not a study claim: a greedy construction should out-score a
    // random one at t=0. If this ever fails the greedy builder is broken.
    let inst = load();
    let mut rng = Rng::new(1);
    let g = State::from_codes(&inst.pieces, &crate::StartBoard::GreedyConstruct.build(&inst, &mut rng, 7, 60000));
    let mut rng2 = Rng::new(1);
    let r = State::from_codes(&inst.pieces, &crate::StartBoard::Random.build(&inst, &mut rng2, 1, 60000));
    assert!(
        g.score() > r.score(),
        "greedy start {} did not beat random start {}",
        g.score(),
        r.score()
    );
}
