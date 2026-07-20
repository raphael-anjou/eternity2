//! Kit integration tests: the plumbing must be correct, because everyone builds
//! on it. These check the two things a silent bug would corrupt — that pinned
//! hints are genuine solution cells, and that a solved board scores the maximum
//! through the canonical scorer.

use e2_kit::{
    generator, instance_from_generated, pin_solution_hints, score_url, Board, Instance, Pieces,
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
fn known_469_scores_469() {
    // The kit's bundled sample board is a verified 469; scoring it through the
    // kit must agree, proving score_url routes to the canonical scorer.
    let edges = include_str!("../examples/data/known-469.edges");
    assert_eq!(score_url(edges), Some(469));
}
