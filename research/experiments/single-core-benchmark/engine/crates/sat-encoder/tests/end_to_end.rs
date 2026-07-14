// End-to-end SAT-encoder validation.
//
// For a generated puzzle, the encoder must produce a CNF that:
//   1. Is satisfiable (because the generator guarantees a perfect solution).
//   2. Has a satisfying model that decodes back to a fully-placed board
//      with 100% edge match.
//
// We use splr (pure-Rust CDCL) as the back-end SAT solver.

use eternity2_core::{Board, BORDER, Hints, Piece, PieceId, Puzzle};
use eternity2_generator::{generate, GeneratorConfig};
use eternity2_sat_encoder::{encode, decode_model, EncodeOptions, VarMap};
use splr::{Certificate, Config, SolveIF, Solver};

fn score_board(puzzle: &Puzzle, board: &Board) -> (u32, u32) {
    let w = puzzle.width;
    let h = puzzle.height;
    let total = (w - 1) * h + w * (h - 1);
    let mut matches = 0u32;
    let lookup = |id: PieceId| -> Option<&Piece> {
        puzzle.piece(id)
    };
    for y in 0..h {
        for x in 0..w {
            let pos = y * w + x;
            let Some((pid, rot)) = board.get(pos) else { continue };
            let Some(piece) = lookup(pid) else { continue };
            let e = piece.edges.rotated(rot).as_array();
            if x + 1 < w {
                if let Some((rpid, rrot)) = board.get(y * w + (x + 1)) {
                    if let Some(rp) = lookup(rpid) {
                        let re = rp.edges.rotated(rrot).as_array();
                        if e[1] == re[3] && e[1] != BORDER && e[1] != 0 { matches += 1; }
                    }
                }
            }
            if y + 1 < h {
                if let Some((bpid, brot)) = board.get((y + 1) * w + x) {
                    if let Some(bp) = lookup(bpid) {
                        let be = bp.edges.rotated(brot).as_array();
                        if e[2] == be[0] && e[2] != BORDER && e[2] != 0 { matches += 1; }
                    }
                }
            }
        }
    }
    (matches, total)
}

fn solve_and_decode(puzzle: &Puzzle, hints: &Hints) -> Option<Board> {
    let vmap = VarMap::build(puzzle);
    let cnf = encode(puzzle, hints, &vmap, &EncodeOptions { soft_edge_match: false });
    let n_vars = cnf.n_vars() as usize;

    let clauses: Vec<Vec<i32>> = cnf.clauses.clone();
    let mut solver: Solver = Solver::try_from((Config::default(), clauses.as_slice()))
        .expect("splr init");

    match solver.solve() {
        Ok(Certificate::SAT(model)) => {
            let mut assignment = vec![false; n_vars];
            for &lit in &model {
                let v = lit.unsigned_abs() as usize;
                if v >= 1 && v <= n_vars { assignment[v - 1] = lit > 0; }
            }
            Some(decode_model(puzzle, &vmap, &assignment))
        }
        Ok(Certificate::UNSAT) => None,
        Err(e) => panic!("splr error: {:?}", e),
    }
}

#[test]
fn solves_3x3_generated_puzzle() {
    let puzzle = generate(GeneratorConfig {
        size: 3,
        interior_colors: 4,
        seed: 0xDEADBEEF,
    }).expect("generate 3x3");
    let board = solve_and_decode(&puzzle, &Hints::default())
        .expect("3x3 should be satisfiable (generator guarantees a perfect solution)");
    let (matches, total) = score_board(&puzzle, &board);
    assert_eq!(matches, total, "3x3 SAT solution must achieve 100% edge match");
    // Verify every cell is placed and every piece used exactly once.
    let mut piece_seen = vec![false; puzzle.pieces().len()];
    for pos in 0..puzzle.cell_count() {
        let (pid, _) = board.get(pos).expect("every cell placed");
        piece_seen[pid as usize] = true;
    }
    assert!(piece_seen.iter().all(|&b| b), "every piece must be placed exactly once");
}

#[test]
fn solves_4x4_generated_puzzle() {
    let puzzle = generate(GeneratorConfig {
        size: 4,
        interior_colors: 5,
        seed: 0xC0FFEE,
    }).expect("generate 4x4");
    let board = solve_and_decode(&puzzle, &Hints::default())
        .expect("4x4 should be satisfiable");
    let (matches, total) = score_board(&puzzle, &board);
    assert_eq!(matches, total, "4x4 SAT solution must achieve 100% edge match");
}

#[test]
fn solves_5x5_generated_puzzle() {
    let puzzle = generate(GeneratorConfig {
        size: 5,
        interior_colors: 6,
        seed: 0xBADF00D,
    }).expect("generate 5x5");
    let board = solve_and_decode(&puzzle, &Hints::default())
        .expect("5x5 should be satisfiable");
    let (matches, total) = score_board(&puzzle, &board);
    assert_eq!(matches, total, "5x5 SAT solution must achieve 100% edge match");
}
