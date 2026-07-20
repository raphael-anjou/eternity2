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
    board_to_bucas_url, bucas_url, hints_to_param, parse_board_edges, parse_hints, viewer_url,
    BoardDoc, Hint, Instance, SiteInstance, SolveOutput,
};

pub use run::{CellResult, RunConfig, RunDir, Summary};
pub use runner::{sweep, SweepConfig};
pub use solver::{BoundKind, Budget, OutcomeKind, SolveOutcome, Solver};

// The official puzzle is available via the `official_instance` / `OFFICIAL_JSON`
// items defined below, and `pieces_distinct_up_to_rotation` for the invariant
// the URL format relies on.

/// The official puzzle's side length and its interior-colour count. Handy
/// defaults for a sweep aimed at the real 16×16 puzzle.
pub const OFFICIAL_SIZE: u8 = 16;
/// The official interior-colour count (22 renderable motifs).
pub const OFFICIAL_COLORS: u8 = 22;

/// The official Eternity II puzzle, in the site's canonical `Puzzle`/[`SiteInstance`]
/// JSON: all 256 pieces in id order, plus the five official clues. This is the
/// same schema the site's viewer and every algorithm read (`--puzzle …json`), so
/// the kit publishes the real set in *our* format, not a foreign one.
pub const OFFICIAL_JSON: &str = include_str!("../data/official.json");

/// The official Eternity II instance: all 256 real pieces, and — when
/// `with_clues` — the five official clue pieces pinned at their published cells.
///
/// This is the instance a record attempt or a re-measurement actually runs on
/// (one canonical board, not a generated seed). It is loaded through `e2-io`'s
/// own [`SiteInstance`] deserialiser and `From<SiteInstance>` conversion — the
/// same path `Instance::from_site_json` uses — so no parsing lives in the kit.
/// `official_instance(true).finish(board)` re-scores any candidate board against
/// the true set through the canonical scorer, the check a "new record" claim has
/// to survive.
#[must_use]
pub fn official_instance(with_clues: bool) -> Instance {
    let site: SiteInstance =
        serde_json::from_str(OFFICIAL_JSON).expect("bundled official.json is valid SiteInstance");
    let mut instance: Instance = site.into();
    // The official set must be distinct up to rotation — it is what makes
    // edge-only piece recovery unambiguous. Cheap to check; a corrupt bundled
    // file would trip it in tests/debug rather than mis-decoding silently.
    debug_assert!(
        pieces_distinct_up_to_rotation(&instance.pieces),
        "official set is not distinct up to rotation — bundled data is corrupt"
    );
    if !with_clues {
        instance.hints.clear();
    }
    instance
}

/// Turn a [`GeneratedPuzzle`] into a solvable [`Instance`] with no hints.
///
/// This wraps the pieces in the IO layer's [`Instance`] so the scorer, the URL
/// encoders, and the sweep runner all work on it. Use the *scrambled* forms
/// (`generator::generate` / `generate_framed`) for a genuine puzzle; a *solved*
/// form would hand the solver the answer.
///
/// The generator aims for colour-balanced pieces that are **distinct up to
/// rotation** — the property edge-only piece recovery relies on — but for a full
/// 16×16 board it does so **best-effort** (a bounded repair pass). At the
/// official 16×16/22 shape this always succeeds in practice, and a debug build
/// asserts it here. For unusual small shapes with very few colours distinctness
/// may be impossible; there, check [`pieces_distinct_up_to_rotation`] yourself
/// before relying on edge-only recovery.
#[must_use]
pub fn instance_from_generated(name: &str, p: &GeneratedPuzzle) -> Instance {
    let pieces = Pieces::new(p.pieces.clone());
    debug_assert!(
        p.width != 16 || p.height != 16 || pieces_distinct_up_to_rotation(&pieces),
        "generated 16×16 board is not distinct up to rotation — edge-only recovery is unsafe"
    );
    Instance {
        name: name.to_string(),
        width: p.width,
        height: p.height,
        num_colors: p.num_colors,
        pieces,
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

/// True when every piece is **distinct up to rotation** — no two pieces share a
/// canonical (rotation-minimal) edge pattern.
///
/// This is the property that lets a board recover piece identity and rotation
/// from its edges alone (and so lets a board-in-a-URL carry no piece channel).
/// The official set has it; a generated puzzle is built to have it but only
/// best-effort. If it fails, edge-only piece recovery ([`Instance::match_board`])
/// can silently mis-assign, so check it before relying on that path.
#[must_use]
pub fn pieces_distinct_up_to_rotation(pieces: &Pieces) -> bool {
    let mut seen = std::collections::HashSet::with_capacity(pieces.len());
    pieces.iter().all(|(_, p)| seen.insert(canonical(p.edges)))
}
