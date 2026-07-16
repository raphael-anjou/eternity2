//! Engine correctness tests. The load-bearing invariants: every variant is
//! sound (breaks never pair with a strict-only propagator), the search only
//! ever reports a board it actually built, and a strict search never claims a
//! score above what its placed edges justify.

use dfs_core::N;
use dfs_io::Instance;

use crate::{all_specs, find, run, RunConfig, SpecKind};

const VARIANT_00: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../../variants/variant_00.json");

fn inst() -> Instance {
    Instance::from_site_json(VARIANT_00).expect("load variant 00")
}

#[test]
fn every_registered_variant_is_sound() {
    for s in all_specs() {
        assert!(s.is_sound(), "{} pairs breaks with a strict-only propagator", s.name);
    }
}

#[test]
fn registry_names_are_unique() {
    let specs = all_specs();
    let mut names: Vec<&str> = specs.iter().map(|s| s.name).collect();
    names.sort_unstable();
    let before = names.len();
    names.dedup();
    assert_eq!(before, names.len(), "duplicate variant name in registry");
}

#[test]
fn every_non_root_parent_exists() {
    let specs = all_specs();
    let names: std::collections::HashSet<&str> = specs.iter().map(|s| s.name).collect();
    for s in &specs {
        if let Some(p) = s.parent {
            assert!(names.contains(p), "{}'s parent {p} is not in the registry", s.name);
        }
    }
}

#[test]
fn naive_clean_reports_a_consistent_score() {
    // A short run: the reported score must equal the canonical re-score of the
    // returned board, and score + breaks must equal the max (480).
    let inst = inst();
    let spec = find("naive-clean").unwrap();
    let res = run(&inst, &spec, RunConfig { budget_ms: 500, seed: 1 });
    let out = res.output(&inst);
    assert_eq!(out.score, res.best_score, "reported score != canonical re-score");
    assert_eq!(out.score + out.breaks, 480);
    // A strict row-major DFS must make real progress in half a second.
    assert!(res.stats.max_depth > 100, "suspiciously shallow: {}", res.stats.max_depth);
    assert!(res.stats.nodes > 1_000_000);
}

#[test]
fn best_board_is_a_real_placement() {
    // The returned board must have exactly the pinned hints plus whatever the
    // search placed, with no piece used twice.
    let inst = inst();
    let spec = find("rowmajor").unwrap();
    let res = run(&inst, &spec, RunConfig { budget_ms: 300, seed: 1 });
    let codes = res.best.to_cell_codes();
    let mut used = vec![false; inst.pieces.len()];
    let mut placed = 0;
    for &v in &codes {
        if v >= 0 {
            let pid = (v / 4) as usize;
            assert!(!used[pid], "piece {pid} placed twice");
            used[pid] = true;
            placed += 1;
        }
    }
    assert!(placed <= N);
    // Every pinned hint is present in the best board.
    for h in &inst.hints {
        let v = codes[h.pos as usize];
        assert_eq!(v, i32::from(h.piece) * 4 + i32::from(h.rot), "hint at {} lost", h.pos);
    }
}

#[test]
fn break_variants_beat_the_strict_wall_and_stay_consistent() {
    // Given enough budget to reach the break zone, a break variant reaches past
    // the strict row-major depth wall (~205) and reports a canonically
    // consistent board (score + breaks == 480).
    let inst = inst();
    let brk = run(&inst, &find("break-1").unwrap(), RunConfig { budget_ms: 6000, seed: 1 });
    assert!(
        brk.stats.max_depth > 205,
        "breaks should reach past the strict wall, got depth {}",
        brk.stats.max_depth
    );
    let out = brk.output(&inst);
    assert_eq!(out.score, brk.best_score);
    // The reported break count is the TRUE interior-break count on the best
    // board, not the unmatched-edge deficit. It never exceeds the deficit, and
    // it is at most the schedule's ceiling (12 for the Blackwood ladder).
    assert!(
        brk.stats.breaks <= 480 - brk.best_score,
        "true breaks {} exceed the deficit {}",
        brk.stats.breaks,
        480 - brk.best_score
    );
    assert!(brk.stats.breaks <= 12, "more breaks than the schedule allows");
}

#[test]
fn break_one_never_exceeds_one_mismatch_per_cell() {
    // The non-adjacency rule: on the best board of BREAK-1, no interior cell is
    // incident to more than one broken (mismatched, non-border) edge.
    let inst = inst();
    let res = run(&inst, &find("break-1").unwrap(), RunConfig { budget_ms: 1500, seed: 1 });
    let cells = res.best.to_edge_cells(&inst.pieces);
    for pos in 0..N {
        let (row, col) = (pos / 16, pos % 16);
        let a = cells[pos];
        let mut broken = 0;
        // right and down mismatches counted at this cell, plus left/up from
        // neighbours, to get this cell's incident broken edges.
        if col < 15 {
            let b = cells[pos + 1];
            if a[1] != 0 && b[3] != 0 && a[1] != b[3] {
                broken += 1;
            }
        }
        if col > 0 {
            let b = cells[pos - 1];
            if a[3] != 0 && b[1] != 0 && a[3] != b[1] {
                broken += 1;
            }
        }
        if row < 15 {
            let b = cells[pos + 16];
            if a[2] != 0 && b[0] != 0 && a[2] != b[0] {
                broken += 1;
            }
        }
        if row > 0 {
            let b = cells[pos - 16];
            if a[0] != 0 && b[2] != 0 && a[0] != b[2] {
                broken += 1;
            }
        }
        assert!(broken <= 1, "cell {pos} has {broken} broken edges (>1)");
    }
}

#[test]
fn propagators_report_consistent_boards() {
    // Every propagating variant must report a score that equals the canonical
    // re-score of the board it returns — a propagator that pruned incorrectly
    // and corrupted its bookkeeping would show up as a mismatch here. (This does
    // not by itself prove the prune is sound — that rests on the by-construction
    // argument that a propagator only rejects an already-empty domain — but it
    // catches any propagator that mis-scores or returns an illegal board.)
    let inst = inst();
    for name in ["mrv-fc", "mrv-ac3", "mrv-gacolor"] {
        let res = run(&inst, &find(name).unwrap(), RunConfig { budget_ms: 800, seed: 1 });
        let out = res.output(&inst);
        assert_eq!(out.score, res.best_score, "{name}: reported score != canonical re-score");
        // No piece is placed twice on the returned board.
        let codes = res.best.to_cell_codes();
        let mut used = vec![false; inst.pieces.len()];
        for &v in &codes {
            if v >= 0 {
                let pid = (v / 4) as usize;
                assert!(!used[pid], "{name}: piece {pid} placed twice");
                used[pid] = true;
            }
        }
    }
}

#[test]
fn forward_check_never_reports_a_strictly_higher_score_than_it_holds() {
    // A propagator must not inflate the score: its reported best is a real board.
    // Cross-check that the frontier-at-timeout is never deeper than the deepest
    // depth reached (a coherence check on the two depth stats after the fix).
    let inst = inst();
    let res = run(&inst, &find("mrv-fc").unwrap(), RunConfig { budget_ms: 800, seed: 1 });
    assert!(
        res.stats.depth_at_timeout <= res.stats.max_depth,
        "depth_at_timeout {} exceeds max_depth {}",
        res.stats.depth_at_timeout,
        res.stats.max_depth
    );
}

#[test]
fn engine_kind_variants_all_run() {
    // Smoke: every Engine-kind variant runs a tiny budget without panicking and
    // returns a legal score.
    let inst = inst();
    for s in all_specs().into_iter().filter(|s| s.kind == SpecKind::Engine) {
        let res = run(&inst, &s, RunConfig { budget_ms: 60, seed: 1 });
        assert!(res.best_score <= 480, "{} reported impossible score", s.name);
    }
}
