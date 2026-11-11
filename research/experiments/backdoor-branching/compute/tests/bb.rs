use std::time::Duration;

use backdoor_branching::{zscore, BbSolver, Rule, IMPACT, N_FEATURES};
use hard_ladder::instance::{colours, generate, make_instance, Ladder, Role};
use hard_ladder::{run_one, LadderInstance, LadderSolver, SolverEntry};

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

fn entry(make: fn(u64) -> Box<dyn LadderSolver>) -> SolverEntry {
    SolverEntry { name: "t", description: "", run_seed: 0, make }
}

const RULES: [fn(u64) -> Box<dyn LadderSolver>; 4] = [
    |_| Box::new(BbSolver { rule: Rule::Mrv, guard: Duration::from_secs(60) }),
    |_| Box::new(BbSolver { rule: Rule::Impact, guard: Duration::from_secs(60) }),
    |_| {
        let mut w = [0.0; N_FEATURES];
        w[IMPACT] = 1.0;
        w[0] = 0.5;
        Box::new(BbSolver { rule: Rule::Learned(w), guard: Duration::from_secs(60) })
    },
    |_| Box::new(BbSolver { rule: Rule::Single(4, 1.0), guard: Duration::from_secs(60) }),
];

#[test]
fn every_rule_solves_verified_and_counts_every_assignment() {
    for make in RULES {
        for seed in 1..=3 {
            let inst = small(5, 4, seed);
            let r = run_one(&entry(make), &inst, 1_000_000);
            assert_eq!(r.status, "solved", "{:?}", r.error);
            assert!(r.nodes >= 25, "at least one assignment per cell");
            assert_eq!(r.nodes, r.nodes_counted);
            assert!(r.propagation > 0);
        }
    }
}

#[test]
fn deterministic_node_counts() {
    for make in RULES {
        let inst = small(6, 5, 7);
        let a = run_one(&entry(make), &inst, 200_000);
        let b = run_one(&entry(make), &inst, 200_000);
        assert_eq!((a.status.clone(), a.nodes_counted, a.backtracks), (b.status, b.nodes_counted, b.backtracks));
    }
}

#[test]
fn node_cap_is_respected() {
    let inst = small(7, 6, 1001);
    let r = run_one(&entry(RULES[0]), &inst, 50);
    assert_eq!(r.status, "censored");
    assert_eq!(r.nodes_counted, 50);
}

#[test]
fn time_guard_returns_cap_reached() {
    let e = entry(|_| Box::new(BbSolver { rule: Rule::Mrv, guard: Duration::ZERO }));
    let r = run_one(&e, &small(6, 5, 1), 1_000_000);
    assert_eq!(r.status, "censored");
    assert!(r.nodes_counted < 1_000_000);
}

#[test]
fn r8_instances_are_the_ladder_generator_at_c10() {
    assert_eq!(colours(Ladder::EightC10, 8), 10);
    for seed in [1, 30, 1001] {
        let role = if seed > 1000 { Role::Calibration } else { Role::Test };
        let inst = make_instance(Ladder::EightC10, role, 8, seed);
        let (p, planted) = generate(8, 10, seed);
        assert_eq!(inst.puzzle.pieces, p.pieces);
        assert_eq!(inst.planted, planted);
        assert_eq!(inst.ladder, "8-c10");
    }
}

#[test]
fn zscore_zero_sd_gives_zero() {
    let mut a = [0.0; N_FEATURES];
    let mut b = [0.0; N_FEATURES];
    a[1] = 1.0;
    b[1] = 3.0;
    let z = zscore(&[a, b]);
    assert!((z[0][1] + 1.0).abs() < 1e-12 && (z[1][1] - 1.0).abs() < 1e-12);
    assert_eq!(z[0][0], 0.0);
}
