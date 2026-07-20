//! A worked example of a solver that returns a **bound**, not a placed board.
//!
//!   cargo run --release --example bound
//!
//! Not every useful result is a board. Sometimes you compute a number that
//! *bounds* the optimum from above — an LP relaxation, a MIP upper bound, or (as
//! here) a cheap combinatorial relaxation. Such a number must never be recorded
//! as if a solver had achieved it. The kit's outcome type makes that a type-level
//! distinction: this solver returns `SolveOutcome::bound(...)`, so the sweep
//! runner records it as a bound (not a score), and `Summary` / `compare` keep it
//! out of the achieved-score statistics.
//!
//! THE BOUND: on any placement, each interior colour `c` appears on some number
//! of piece edges `N_c`. A matched interior seam consumes two same-colour edges,
//! so at most `floor(N_c / 2)` seams of colour `c` can ever match. Summing over
//! colours gives an upper bound on the matched-edge score that holds for EVERY
//! arrangement — no search required. It is loose (real boards fall well short),
//! but it is a genuine, honest ceiling, and it is `BoundKind::GreedyRelaxed`.

use e2_kit::{official_instance, BoundKind, Board, Budget, Instance, SolveOutcome, Solver};

/// A colour-count relaxation: sum over colours of `floor(edge_count / 2)`.
struct ColorCountBound;

impl Solver for ColorCountBound {
    fn name(&self) -> String {
        "color-count-bound".into()
    }

    fn solve(&mut self, instance: &Instance, _start: &Board, _budget: Budget) -> SolveOutcome {
        // Tally every interior-colour edge across the whole piece set.
        let mut counts = [0u32; 256];
        for (_, piece) in instance.pieces.iter() {
            for &c in &piece.edges {
                if c != 0 {
                    counts[c as usize] += 1;
                }
            }
        }
        let bound: u32 = counts.iter().map(|&n| n / 2).sum();

        // Return the bound. The board is empty: this is a ceiling, not a
        // placement, and saying so is the whole point.
        SolveOutcome::bound(Board::new(), bound, BoundKind::GreedyRelaxed)
    }
}

fn main() {
    let instance = official_instance(false);
    let mut solver = ColorCountBound;
    let outcome = solver.solve(&instance, &Board::new(), Budget::seconds(1.0));

    let max = instance.max_score();
    println!("solver:  {}", solver.name());
    match outcome.kind {
        e2_kit::OutcomeKind::Bound { value, kind } => {
            println!("result:  UPPER BOUND {value} / {max}  ({kind:?})");
            println!("meaning: no arrangement of the official set can match more than {value} of");
            println!("         the {max} interior seams. This is a ceiling, not an achieved score.");
            if value == max {
                println!(
                    "\nHere the bound equals the maximum: the official set is perfectly colour-\n\
                     balanced (every colour appears an even number of times), so this particular\n\
                     relaxation is trivially tight at {max}. A stronger relaxation (LP/MIP) would\n\
                     return a lower, more useful ceiling — and it would still be a Bound, never a\n\
                     score."
                );
            }
            println!(
                "\nThe empty board it carries scores 0 — which is exactly why a bound must never\n\
                 be averaged as a score. The kit keeps them separate by type: the sweep runner\n\
                 records this as a bound, and Summary / compare exclude it from the score stats."
            );
        }
        other => println!("unexpected outcome: {other:?}"),
    }
}
