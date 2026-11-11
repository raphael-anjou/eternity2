use hard_ladder::instance::generate;
use hard_ladder::{Role, LadderInstance};
use ktam::driver::{instances, r8_instance, run_method, EVENT_BUDGET};
use ktam::{grid, Rule};

fn small(seed: u32) -> LadderInstance {
    let mut i = r8_instance(Role::Calibration, 1001);
    let (puzzle, planted) = generate(5, 3, seed);
    i.n = 5;
    i.c = 3;
    i.seed = seed;
    i.id = format!("n5-c3-s{seed}");
    i.puzzle = puzzle;
    i.planted = planted;
    i
}

#[test]
fn grids_and_names_round_trip() {
    assert_eq!(grid("B3").len(), 9);
    assert_eq!(grid("V1").len(), 36);
    assert_eq!(grid("V2").len(), 9);
    for arm in ["B3", "V1", "V2"] {
        for r in grid(arm) {
            assert_eq!(Rule::parse(&r.name()), Some(r), "{}", r.name());
        }
    }
}

#[test]
fn every_arm_solves_small_boards_with_exact_event_accounting() {
    let rules = [grid("B3")[8], grid("V1")[24], grid("V2")[1]];
    for seed in 1..=3 {
        let inst = small(seed);
        for rule in rules {
            let row = run_method("T", rule, &inst, "T5", 200_000);
            assert_eq!(row.status, "solved", "{} {}", rule.name(), inst.id);
            let rem = row.removals.unwrap();
            assert_eq!(row.events, Some(row.nodes + rem));
            assert_eq!(row.events, Some(2 * row.nodes - 25));
            assert_eq!(row.backtracks, rem);
            assert!(row.within_e);
        }
    }
}

#[test]
fn deterministic_given_seeds() {
    let inst = &instances("R6", Role::Calibration).unwrap()[0];
    for rule in [grid("B3")[7], grid("V1")[24], grid("V2")[1]] {
        let a = run_method("T", rule, inst, "R6", 20_000);
        let b = run_method("T", rule, inst, "R6", 20_000);
        assert_eq!((a.status.clone(), a.nodes, a.events, a.removals, a.removals_planted), (b.status, b.nodes, b.events, b.removals, b.removals_planted));
    }
}

#[test]
fn cap_is_respected_and_censored() {
    let inst = &instances("R7", Role::Calibration).unwrap()[0];
    for rule in [grid("B3")[7], grid("V1")[24], grid("V2")[1]] {
        let row = run_method("T", rule, inst, "R7", 500);
        assert_eq!(row.status, "censored");
        assert_eq!(row.nodes, 500);
        assert!(!row.within_e);
        assert!(row.events.unwrap() < EVENT_BUDGET);
    }
}

#[test]
fn ladder_rungs_load() {
    for (rung, n, c) in [("R6", 6, 5), ("R7", 7, 6), ("R8", 8, 9)] {
        let t = instances(rung, Role::Test).unwrap();
        assert_eq!(t.len(), 30);
        assert!(t.iter().all(|i| i.n == n && i.c == c));
        assert_eq!(t.iter().map(|i| i.seed).collect::<Vec<_>>(), (1..=30).collect::<Vec<_>>());
        let cal = instances(rung, Role::Calibration).unwrap();
        assert_eq!(cal.iter().map(|i| i.seed).collect::<Vec<_>>(), (1001..=1010).collect::<Vec<_>>());
    }
}
