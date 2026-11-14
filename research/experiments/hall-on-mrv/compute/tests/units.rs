//! Unit checks for the Hall routine, the trace comparison and the off arm.

use std::time::Duration;

use backdoor_branching::{BbSolver, Rule};
use hall_on_mrv::{hall_match, subtree_removal, Hall, HallMrv, TraceEntry, OFF};
use hard_ladder::cli::load_instances;
use hard_ladder::{Ladder, LadderSolver, NodeCounter, Role};

/// Three cells sharing two pieces: no single-cell or single-piece rule of A3
/// fires (no cell has one piece, each piece sits in three cells), and the
/// Hall check rejects it.
#[test]
fn hall_rejects_three_cells_two_pieces() {
    let (p, q, r, s) = (1u128, 2u128, 4u128, 8u128);
    assert!(!hall_match(&mut [p | q, p | q, p | q, r | s]));
    assert!(hall_match(&mut [p | q, p | r, q | r, r | s]));
}

fn e(depth: u32, cell: u32, opt: u32, hall_pruned: bool) -> TraceEntry {
    TraceEntry {
        depth,
        cell,
        opt,
        hall_pruned,
    }
}

#[test]
fn subtree_removal_detects_reorder() {
    let base = [
        e(0, 0, 1, false),
        e(1, 2, 5, false),
        e(2, 3, 7, false),
        e(1, 2, 6, false),
    ];
    let cand = [e(0, 0, 1, false), e(1, 2, 5, true), e(1, 2, 6, false)];
    assert_eq!(subtree_removal(&base, &cand), Ok(1));
    let reordered = [e(0, 0, 1, false), e(1, 2, 6, false)];
    assert!(subtree_removal(&base, &reordered).is_err());
}

/// The off arm counts node for node like A16's bb-mrv (R6 calibration).
#[test]
fn off_equals_bb_mrv() {
    let g = Duration::from_secs(600);
    for inst in load_instances(Ladder::Primary, Role::Calibration)
        .unwrap()
        .iter()
        .filter(|i| i.n == 6)
    {
        let mut c1 = NodeCounter::new(1_000_000);
        let o1 = BbSolver {
            rule: Rule::Mrv,
            guard: g,
        }
        .solve(&inst.puzzle, &mut c1);
        for h in [
            OFF,
            Hall {
                scope: 1,
                period: 1,
                prune: false,
            },
            Hall {
                scope: 2,
                period: 4,
                prune: false,
            },
        ] {
            let mut c2 = NodeCounter::new(1_000_000);
            let o2 = HallMrv::new(h, g).solve(&inst.puzzle, &mut c2);
            assert_eq!(o1, o2, "{}", inst.id);
            assert_eq!(
                (c1.nodes(), c1.backtracks, c1.propagation),
                (c2.nodes(), c2.backtracks, c2.propagation)
            );
        }
    }
}
