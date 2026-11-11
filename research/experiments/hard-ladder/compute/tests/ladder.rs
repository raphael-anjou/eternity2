use eternity2_engine::generator::XorShift;
use eternity2_engine::{generate_solved_framed_bc, rotated};
use hard_ladder::instance::{colours, generate, make_instance, owen_criterion, verify_board, Ladder, Role};
use hard_ladder::registry::{find, BUILTIN, CALIBRATION_CANDIDATES};
use hard_ladder::{run_one, LadderInstance, LadderSolver, NodeCounter, Outcome, Puzzle, SolverEntry};

fn small(n: u8, c: u8, seed: u32) -> LadderInstance {
    let (puzzle, planted) = generate(n, c, seed);
    LadderInstance {
        id: format!("t{n}-{c}-{seed}"),
        ladder: "test".into(),
        role: "test".into(),
        n,
        c,
        seed,
        owen_criterion: 0.0,
        generator: String::new(),
        puzzle,
        planted,
    }
}

fn entry(name: &str) -> &'static SolverEntry {
    find(&[], name).expect("registered")
}

#[test]
fn colour_table_matches_prereg() {
    let crit: Vec<f64> = [6, 7, 8, 9, 10].iter().map(|&n| owen_criterion(n)).collect();
    for (got, want) in crit.iter().zip([5.2155, 6.3803, 7.5579, 8.7435, 9.9346]) {
        assert!((got - want).abs() < 1e-3, "{got} vs {want}");
    }
    let p: Vec<u8> = [6, 7, 8, 9, 10].iter().map(|&n| colours(Ladder::Primary, n)).collect();
    let s: Vec<u8> = [6, 7, 8, 9, 10].iter().map(|&n| colours(Ladder::Secondary, n)).collect();
    assert_eq!(p, [5, 6, 8, 9, 10]);
    assert_eq!(s, [6, 7, 7, 8, 9]);
}

#[test]
fn scramble_matches_hardness_peak_and_planted_verifies() {
    for (n, c, seed) in [(6, 5, 1), (8, 8, 3), (10, 10, 30), (9, 8, 1001)] {
        let (puzzle, planted) = generate(n, c, seed);
        // hardness_peak's own code path, verbatim.
        let mut hp = generate_solved_framed_bc(n, c + 5, seed, true, 5);
        let mut rng = XorShift::new(seed ^ 0xA5A5_A5A5);
        rng.shuffle(&mut hp.pieces);
        for p in &mut hp.pieces {
            *p = rotated(*p, rng.below(4) as u8);
        }
        assert_eq!(puzzle.pieces, hp.pieces);
        assert!(puzzle.hints.is_empty());
        verify_board(&puzzle, &planted).expect("planted solution verifies");
        assert_eq!(puzzle.num_colors, c + 5);
    }
    let inst = make_instance(Ladder::Primary, Role::Test, 8, 1);
    assert_eq!((inst.c, inst.id.as_str()), (8, "n8-c8-s1"));
}

#[test]
fn verifier_rejects_broken_boards() {
    let (puzzle, mut planted) = generate(6, 5, 2);
    planted.swap(7, 8);
    assert!(verify_board(&puzzle, &planted).is_err());
}

#[test]
fn example_solver_counts_exactly_like_hp_dfs() {
    for (n, c, seed) in [(4, 3, 1), (4, 3, 2), (5, 4, 1), (5, 4, 2), (5, 4, 3), (6, 5, 1001)] {
        let inst = small(n, c, seed);
        let a = run_one(entry("hp-dfs"), &inst, 300_000);
        let b = run_one(entry("example-dfs"), &inst, 300_000);
        assert_eq!((a.status.as_str(), a.nodes), (b.status.as_str(), b.nodes), "{}", inst.id);
        assert_eq!(a.backtracks, b.backtracks);
        assert_eq!(a.fit_tests, b.fit_tests);
    }
}

#[test]
fn caps_are_respected() {
    let inst = small(8, 8, 1);
    for name in ["hp-dfs", "example-dfs", "scb-naive_rowmajor", "scb-gacolor_ac3"] {
        let r = run_one(entry(name), &inst, 500);
        assert_eq!(r.status, "censored", "{name}");
        assert_eq!(r.nodes, 500);
        // solver-naive polls should_continue less often; the sink panics past cap + 1e6.
        assert!(r.nodes_counted <= 500 + 1_000_001, "{name} counted {}", r.nodes_counted);
    }
}

#[test]
fn registry_sink_count_equals_native_nodes() {
    let names = ["scb-naive_rowmajor", "scb-naive_spiral", "scb-border_first_lcv", "scb-gacolor_ac3", "scb-gacolor_ac3_ns1"];
    for name in names {
        for (n, c, seed) in [(4, 3, 1), (5, 4, 1), (5, 4, 2)] {
            let r = run_one(entry(name), &small(n, c, seed), 50_000_000);
            assert_eq!(r.status, "solved", "{name} {n} {seed}: {:?}", r.error);
            assert_eq!(Some(r.nodes_counted), r.native_nodes, "{name}");
            assert_eq!(r.inconsistent, 0, "{name}");
        }
    }
}

#[test]
fn every_candidate_is_registered() {
    for c in CALIBRATION_CANDIDATES {
        assert!(BUILTIN.iter().any(|e| e.name == *c), "{c}");
    }
}

/// The README's plug-in example, written exactly as an external crate would.
struct FirstFit;
impl LadderSolver for FirstFit {
    fn solve(&mut self, puzzle: &Puzzle, counter: &mut NodeCounter) -> Outcome {
        hard_ladder::solvers::example::ExampleDfs.solve(puzzle, counter)
    }
}

#[test]
fn plug_in_entry_runs_through_the_harness() {
    let e = SolverEntry {
        name: "first-fit",
        description: "test plug-in",
        run_seed: 0,
        make: |_| -> Box<dyn LadderSolver> { Box::new(FirstFit) },
    };
    let r = run_one(&e, &small(5, 4, 1), 1_000_000);
    assert_eq!(r.status, "solved");
    assert!(r.nodes > 0);
}

/// hp-dfs reproduces research/topics/hardness-peak/results/hardness-peak-8x8.json
/// (8x8, 5 frame colours, same generator and scramble): node = placement.
#[test]
fn hp_dfs_matches_committed_hardness_peak_rows() {
    for (c, seed, placements) in [(10, 12, 146_785), (10, 16, 43_383), (10, 28, 58_119), (9, 16, 912_765)] {
        let r = run_one(entry("hp-dfs"), &small(8, c, seed), 2_000_000);
        assert_eq!((r.status.as_str(), r.nodes), ("solved", placements), "c={c} seed={seed}");
    }
}
