//! Eternity II starter kit — the plumbing, so you write only your idea.
//!
//! `use e2_kit::*;` gives you:
//!
//! * the shared substrate, re-exported so you never reach past this crate:
//!   [`Board`], [`Pieces`], [`Instance`], [`SolveOutput`], the canonical
//!   [`score_cells`] scorer, and the seeded [`generator`] with real
//!   Eternity-II colour balance;
//! * the kit's own helpers: [`instance_from_generated`] (turn a generated board
//!   into a solvable [`Instance`]), [`pin_solution_hints`] (pin K solution cells
//!   as hints, à la the official clues), and [`instance_from_url`] /
//!   [`score_url`] (read a board out of an eternity2.dev / bucas URL);
//! * the iteration loop: the [`Solver`] trait, the [`sweep`] runner, and the
//!   [`RunDir`] / [`Summary`] run-directory types.
//!
//! Nothing here re-implements scoring, formats, or generation — those come from
//! the blog's `e2-core` / `e2-io` crates, so a score from the kit is the same
//! score the site and every other engine produce.

#![forbid(unsafe_code)]

pub mod run;
pub mod runner;
pub mod solver;

// Re-export the shared substrate so kit users depend only on `e2_kit`.
pub use e2_core::{
    generator, score_board, score_cells, Board, GeneratedPuzzle, Piece, Pieces, MAX_SCORE_16,
};
pub use e2_io::{
    board_to_bucas_url, bucas_url, parse_board_edges, viewer_url, BoardDoc, Hint, Instance,
    SiteInstance, SolveOutput,
};

pub use run::{CellResult, RunConfig, RunDir, Summary};
pub use runner::{sweep, SweepConfig};
pub use solver::{Budget, Solver};

/// The official puzzle's side length and its interior-colour count. Handy
/// defaults for a sweep aimed at the real 16×16 puzzle.
pub const OFFICIAL_SIZE: u8 = 16;
/// The official interior-colour count (22 renderable motifs).
pub const OFFICIAL_COLORS: u8 = 22;

/// Turn a [`GeneratedPuzzle`] into a solvable [`Instance`] with no hints.
///
/// The generated puzzle already carries all-distinct, colour-balanced pieces;
/// this just wraps them in the IO layer's [`Instance`] so the scorer, the URL
/// encoders, and the sweep runner all work on it. Use the *scrambled* forms
/// (`generator::generate` / `generate_framed`) for a genuine puzzle; a *solved*
/// form would hand the solver the answer.
#[must_use]
pub fn instance_from_generated(name: &str, p: &GeneratedPuzzle) -> Instance {
    Instance {
        name: name.to_string(),
        width: p.width,
        height: p.height,
        num_colors: p.num_colors,
        pieces: Pieces::new(p.pieces.clone()),
        hints: Vec::new(),
    }
}

/// Pin `k` of the board's *solution* cells as hints, mimicking the way the
/// official puzzle ships five fixed clue pieces.
///
/// The solution is the identity placement of the matching *solved* puzzle, so
/// this takes the `(size, colors, seed, framed)` the instance was generated from
/// and re-derives which piece sits at each cell in the solution. The pinned
/// cells are chosen deterministically from `seed`, so a given `(seed, k)` always
/// pins the same cells — runs stay reproducible.
///
/// Returns a new [`Instance`] with the hints applied. Panics if `k` exceeds the
/// cell count.
#[must_use]
pub fn pin_solution_hints(
    mut instance: Instance,
    size: u8,
    colors: u8,
    seed: u32,
    framed: bool,
    k: u32,
) -> Instance {
    use e2_core::generator::XorShift;

    let n_cells = usize::from(size) * usize::from(size);
    let k = k as usize;
    assert!(k <= n_cells, "cannot pin {k} cells on a {n_cells}-cell board");

    // The solved board: piece i sits at cell i, rotation 0. Its edge quad at
    // each cell tells us which piece (by edge pattern) to pin there. We map that
    // quad back to a piece id in the *scrambled* instance's piece set.
    let solved = generator::generate_solved_framed(size, colors, seed, framed);

    // Build an edge-pattern -> piece-id lookup over the scrambled set. Pieces are
    // unique up to rotation, so the canonical key is a unique identifier.
    let mut by_key: std::collections::HashMap<[u8; 4], (u16, [u8; 4])> =
        std::collections::HashMap::new();
    for (pid, piece) in instance.pieces.iter() {
        by_key.insert(canonical(piece.edges), (pid, piece.edges));
    }

    // Choose k distinct cells deterministically from the seed.
    let mut order: Vec<usize> = (0..n_cells).collect();
    let mut rng = XorShift::new(seed ^ 0x00C1_0E50_u32);
    rng.shuffle(&mut order);
    order.truncate(k);

    let mut hints = Vec::with_capacity(k);
    for &cell in &order {
        let want = solved.pieces[cell];
        let key = canonical(want);
        if let Some(&(pid, base)) = by_key.get(&key) {
            // Find the rotation of the scrambled piece that reproduces the
            // solution's orientation at this cell.
            let rot = (0..4).find(|&r| e2_core::rotated(base, r) == want).unwrap_or(0);
            hints.push(Hint {
                pos: cell as u16,
                piece: pid,
                rot,
            });
        }
    }
    instance.hints = hints;
    instance
}

/// Read a board out of an `eternity2.dev` / `e2.bucas.name` URL and score it
/// canonically, returning the full [`SolveOutput`] (score, breaks, URL, JSON).
///
/// The URL only carries edge colours, so we rebuild placements by matching each
/// cell's edge quad against `instance`'s piece set — pass the instance the board
/// belongs to. For a bare "just tell me the score of these edges" check, use
/// [`score_url`].
#[must_use]
pub fn output_from_url(instance: &Instance, url: &str) -> Option<SolveOutput> {
    let cells = parse_board_edges(url)?;
    Some(instance.match_board(&cells))
}

/// Score the raw edge grid encoded in a board URL, with no piece set required.
///
/// This is the format-agnostic path: it scores whatever edges the URL carries
/// through the one canonical rim-excluding scorer. Returns `None` if the URL has
/// no parseable `board_edges`.
#[must_use]
pub fn score_url(url: &str) -> Option<u32> {
    let cells = parse_board_edges(url)?;
    Some(score_cells(&cells))
}

/// Lexicographically minimal rotation of a piece's URDL edges — the
/// rotation-independent identity of a piece, matching the generator's key.
#[must_use]
fn canonical(e: [u8; 4]) -> [u8; 4] {
    let mut best = e;
    for r in 1..4 {
        let c = e2_core::rotated(e, r);
        if c < best {
            best = c;
        }
    }
    best
}
