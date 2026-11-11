//! Equivalence checks for the A22 arms (PREREG.md).

use std::time::Duration;

use backdoor_branching::{BbSolver, Rule};
use hard_ladder::cli::load_instances;
use hard_ladder::{verify_board, Ladder, LadderSolver, NodeCounter, Outcome, Role};
use mrv_baseline::scb::GuardScb;
use mrv_baseline::{MrvSolver, Order, Prop};
use sat_backdoors::{Model, Stats};

const G: Duration = Duration::from_secs(600);

fn nodes(s: &mut dyn LadderSolver, p: &hard_ladder::Puzzle, cap: u64) -> (u64, bool) {
    let mut c = NodeCounter::new(cap);
    let out = s.solve(p, &mut c);
    let solved = match out {
        Outcome::Solved(b) => {
            verify_board(p, &b).expect("valid board");
            true
        }
        _ => false,
    };
    (c.nodes().min(cap), solved)
}

#[test]
fn prop_with_hs_equals_model_propagate() {
    let insts = load_instances(Ladder::Primary, Role::Calibration).unwrap();
    for inst in insts.iter().filter(|i| i.n <= 7).take(10) {
        let m = Model::new(&inst.puzzle);
        let p = Prop::new(&m, true);
        let mut d0 = p.unary_root();
        let all: Vec<usize> = (0..m.cells).collect();
        let ok = p.propagate(&mut d0, &all, &mut Stats { propagations: 0 });
        assert_eq!(ok, m.root_ok);
        assert_eq!(d0, m.root());
        // assign planted options one by one from a scrambled cell order
        let (mut a, mut b) = (d0.clone(), d0.clone());
        for k in 0..m.cells {
            let c = (k * 7 + inst.seed as usize) % m.cells;
            let o = sat_backdoors_option(&m, &a, c);
            let Some(o) = o else { break };
            for d in [&mut a, &mut b] {
                let w = m.words;
                d[c * w..(c + 1) * w].iter_mut().for_each(|x| *x = 0);
                d[c * w + (o >> 6)] = 1u64 << (o & 63);
            }
            let ra = m.propagate(&mut a, &[c], &mut Stats { propagations: 0 });
            let rb = p.propagate(&mut b, &[c], &mut Stats { propagations: 0 });
            assert_eq!(ra, rb);
            if !ra {
                break;
            }
            assert_eq!(a, b);
        }
    }
}

fn sat_backdoors_option(m: &Model, d: &[u64], c: usize) -> Option<usize> {
    backdoor_branching::options(m, d, c).into_iter().next()
}

#[test]
fn mrv_free_index_hs_equals_bb_mrv() {
    let insts = load_instances(Ladder::Primary, Role::Calibration).unwrap();
    for inst in insts.iter().filter(|i| i.n <= 7) {
        let a = nodes(&mut BbSolver { rule: Rule::Mrv, guard: G }, &inst.puzzle, 200_000);
        let b = nodes(
            &mut MrvSolver { order: Order::Free, random: None, hs: true, guard: G },
            &inst.puzzle,
            200_000,
        );
        assert_eq!(a, b, "{}", inst.id);
    }
}

#[test]
fn ablations_solve_small_boards() {
    let insts = load_instances(Ladder::Primary, Role::Calibration).unwrap();
    for inst in insts.iter().filter(|i| i.n == 6).take(5) {
        for mut s in [
            MrvSolver { order: Order::Free, random: None, hs: false, guard: G },
            MrvSolver { order: Order::BorderFirst, random: None, hs: true, guard: G },
            MrvSolver { order: Order::Free, random: Some(1), hs: true, guard: G },
        ] {
            let (_, solved) = nodes(&mut s, &inst.puzzle, 2_000_000);
            assert!(solved, "{}", inst.id);
        }
    }
}

fn orig() -> Box<dyn eternity2_solver_trait::Solver> {
    Box::new(eternity2_solver_engine::EngineSolver::gacolor_ac3_random())
}
fn off() -> Box<dyn eternity2_solver_trait::Solver> {
    Box::new(mrvb_solver_engine::EngineSolver::gacolor_ac3_random())
}
fn ad() -> Box<dyn eternity2_solver_trait::Solver> {
    Box::new(mrvb_solver_engine::EngineSolver::gacolor_ac3_random().with_alldiff(true))
}

#[test]
fn scb_wrapper_and_fork_off_equal_hard_ladder_scb() {
    let insts = load_instances(Ladder::Primary, Role::Calibration).unwrap();
    for inst in insts.iter().filter(|i| i.n <= 7) {
        let mut base = hard_ladder::solvers::scb::Scb { make: orig, seed: 1 };
        let a = nodes(&mut base, &inst.puzzle, 200_000);
        let b = nodes(&mut GuardScb { make: orig, seed: 1, guard: G }, &inst.puzzle, 200_000);
        let c = nodes(&mut GuardScb { make: off, seed: 1, guard: G }, &inst.puzzle, 200_000);
        assert_eq!(a, b, "{}", inst.id);
        assert_eq!(a, c, "{}", inst.id);
    }
}

#[test]
fn scb_ad_solves_and_prunes() {
    let insts = load_instances(Ladder::Primary, Role::Calibration).unwrap();
    for inst in insts.iter().filter(|i| i.n == 6).take(5) {
        let (_, solved) = nodes(&mut GuardScb { make: ad, seed: 1, guard: G }, &inst.puzzle, 2_000_000);
        assert!(solved, "{}", inst.id);
    }
}
