//! Single-core apples-to-apples benchmark harness for Eternity II solvers.
//!
//! # The unified interface
//!
//! Every algorithm in the grid — ours and the community's — consumes the SAME
//! wire format and emits the SAME output, so scores are directly comparable and
//! every solver is later liftable to the public site (`eternity2/engine`):
//!
//! * **Input**: a [`SiteInstance`] — the site-schema `Puzzle` JSON (`name`,
//!   `width`, `height`, `numColors`, `pieces` as `[u8;4]` URDL, `hints` as
//!   `{pos, piece, rot}`), byte-compatible with `eternity2-engine::Puzzle`, plus
//!   run params (`seed`, `budget_ms`). Corner-fixing is expressed purely as
//!   extra `hints` — no engine-specific flags.
//! * **Output**: a [`SolveOutput`] carrying the best board (row-major
//!   `piece*4 + rot`, `-1` empty), its canonical score, and a bucas `.url`.
//! * **Scoring**: the ONE scorer [`score_cells`] (identical formula to the
//!   site's `score_board` and our `verify_bucas` raw count). Engines' internal
//!   scores are never trusted — every board is re-scored here.
//!
//! # Conventions (shared by the site and v2 core, verified identical)
//!
//! Edges are URDL = (top, right, bottom, left). Rotation is clockwise quarter
//! turns. A cell holds `piece*4 + rot`, row-major, `-1` when empty. Border color
//! is 0; bucas letter = `'a' + color`.

use std::path::Path;

use eternity2_core::{Board, Edges, Piece, Puzzle, Rotation};
use serde::{Deserialize, Serialize};

pub mod algos;

pub const BORDER: u8 = 0;

/// The puzzle instance wire schema is the *one* shared definition in `e2-io`
/// (`SiteInstance` / `SiteHint`), so the studies and this benchmark cannot drift
/// on the format. The benchmark-specific behaviour — loading the official CSV
/// and converting to this workspace's `eternity2_core` types — lives in the
/// [`SiteInstanceExt`] trait below, which is why `e2-io` can stay free of any
/// engine dependency.
pub use e2_io::{SiteHint, SiteInstance};

/// What every solver returns. `board` is row-major `piece*4 + rot` (`-1` empty).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolveOutput {
    pub board: Vec<i32>,
    /// Canonical matched-edge count via [`score_cells`] — the single source of
    /// truth. Engines never self-report here.
    pub score: u32,
    /// The canonical `https://eternity2.dev/viewer?…` URL for this board.
    pub url: String,
}

/// Benchmark-local behaviour on the shared [`SiteInstance`]: everything that
/// touches this workspace's `eternity2_core` types (which `e2-io` deliberately
/// knows nothing about). Bring it into scope with `use bench_grid::SiteInstanceExt`.
pub trait SiteInstanceExt {
    /// Load the official puzzle + its 5 clue hints from the canonical CSV,
    /// re-expressed in the site schema. The base every variant pins corners onto.
    fn official<P: AsRef<Path>>(csv: P) -> Result<SiteInstance, String>;
    /// Convert to the `eternity2_core::Puzzle` every native engine consumes.
    fn to_core_puzzle(&self) -> Puzzle;
    /// Convert the hints to the core `Hints`.
    fn to_core_hints(&self) -> eternity2_core::Hints;
    /// Build a [`SolveOutput`] from a row-major `piece*4 + rot` board (`-1`
    /// empty): canonically scored, carrying the eternity2.dev viewer URL.
    fn finish(&self, board: Vec<i32>) -> SolveOutput;
    /// Build the canonical [`e2_io::BoardDoc`] for a `piece*4 + rot` board.
    fn to_doc(&self, board: &[i32], board_hash: u64) -> e2_io::BoardDoc;
}

impl SiteInstanceExt for SiteInstance {
    fn official<P: AsRef<Path>>(csv: P) -> Result<Self, String> {
        let (puzzle, hints) =
            eternity2_puzzle_io::load_puzzle_with_hints(csv).map_err(|e| format!("{e:?}"))?;
        let mut pieces = vec![[0u8; 4]; puzzle.pieces().len()];
        let mut num_colors = 0u8;
        for p in puzzle.pieces() {
            let e = p.edges.as_array();
            pieces[p.id as usize] = e;
            for &c in &e {
                num_colors = num_colors.max(c);
            }
        }
        let hints = hints
            .hints
            .iter()
            .map(|h| SiteHint {
                pos: h.position as u16,
                piece: h.piece_id,
                rot: h.rotation.as_u8(),
            })
            .collect();
        Ok(SiteInstance {
            name: "size_16_official_eternity".into(),
            width: puzzle.width as u8,
            height: puzzle.height as u8,
            num_colors,
            pieces,
            hints,
        })
    }

    /// Convert to the v2 `eternity2_core::Puzzle` every native engine consumes.
    fn to_core_puzzle(&self) -> Puzzle {
        let pieces = self
            .pieces
            .iter()
            .enumerate()
            .map(|(id, e)| Piece::new(id as u16, Edges::new(e[0], e[1], e[2], e[3])))
            .collect();
        // Site `numColors` counts interior colors (border 0 excluded); core
        // `color_count` is max_color + 1 = interior colors + 1 for the border.
        Puzzle::new(
            self.width as u32,
            self.height as u32,
            self.num_colors as u32 + 1,
            pieces,
        )
        .expect("valid site instance -> core puzzle")
    }

    /// Convert the hints to the v2 core `Hints`.
    fn to_core_hints(&self) -> eternity2_core::Hints {
        let hints = self
            .hints
            .iter()
            .map(|h| eternity2_core::Hint {
                position: h.pos as u32,
                piece_id: h.piece,
                rotation: Rotation::from_u8(h.rot).expect("rot < 4"),
            })
            .collect();
        eternity2_core::Hints::new(hints)
    }

    /// Build a [`SolveOutput`] from a row-major `piece*4 + rot` board (`-1`
    /// empty). Scores it canonically and renders the bucas URL — the uniform
    /// output contract for every solver.
    fn finish(&self, board: Vec<i32>) -> SolveOutput {
        let cells = board_to_cells(self, &board);
        let score = score_cells(self, &cells);
        let url = cells_to_bucas_url(self, &cells, &board);
        SolveOutput { board, score, url }
    }

    /// Build the canonical [`e2_io::BoardDoc`] for a row-major `piece*4 + rot`
    /// board (`-1` empty). Scores it canonically and carries the eternity2.dev
    /// viewer URL. `board_hash` is a caller-supplied stable fingerprint.
    fn to_doc(&self, board: &[i32], board_hash: u64) -> e2_io::BoardDoc {
        let cells = board_to_cells(self, board);
        let score = score_cells(self, &cells);
        let hints: Vec<(u16, u8)> = self.hints.iter().map(|h| (h.pos, h.rot)).collect();
        e2_io::BoardDoc::new(
            &self.name,
            self.width,
            score,
            self.max_score(),
            &cells,
            board,
            board_hash,
            &hints,
        )
    }
}

/// Decode a `piece*4 + rot` board into per-cell URDL edge arrays (empty ->
/// `[0;4]`, i.e. all-border, which never scores).
fn board_to_cells(inst: &SiteInstance, board: &[i32]) -> Vec<[u8; 4]> {
    let n = inst.cell_count();
    let mut cells = vec![[0u8; 4]; n];
    for (pos, &v) in board.iter().enumerate().take(n) {
        if v < 0 {
            continue;
        }
        let pid = (v / 4) as usize;
        let r = (v % 4) as u8;
        if let Some(e) = inst.pieces.get(pid) {
            cells[pos] = rotated(*e, r);
        }
    }
    cells
}

/// Rotate URDL edges clockwise by `r` (matches `Edges::rotated`).
pub fn rotated(e: [u8; 4], r: u8) -> [u8; 4] {
    let [t, ri, b, l] = e;
    match r & 3 {
        0 => [t, ri, b, l],
        1 => [l, t, ri, b],
        2 => [b, l, t, ri],
        _ => [ri, b, l, t],
    }
}

/// THE canonical scorer: matched non-border adjacencies (right + down per
/// cell). Identical to `eternity2-engine::score_board` and `verify_bucas`.
pub fn score_cells(inst: &SiteInstance, cells: &[[u8; 4]]) -> u32 {
    let (w, h) = (inst.width as usize, inst.height as usize);
    let mut score = 0;
    for y in 0..h {
        for x in 0..w {
            let a = cells[y * w + x];
            if x + 1 < w {
                let b = cells[y * w + x + 1];
                if a[1] == b[3] && a[1] != BORDER {
                    score += 1;
                }
            }
            if y + 1 < h {
                let b = cells[(y + 1) * w + x];
                if a[2] == b[0] && a[2] != BORDER {
                    score += 1;
                }
            }
        }
    }
    score
}

/// Render a board as the canonical eternity2.dev viewer URL. `cells` are the
/// per-cell URDL edge quads (row-major); the board rides entirely in
/// `board_edges`, and any pinned clues in `hints`. Delegates to the shared
/// `e2-io` encoder so every engine emits byte-identical URLs.
pub fn cells_to_bucas_url(inst: &SiteInstance, cells: &[[u8; 4]], _codes: &[i32]) -> String {
    let hints: Vec<(u16, u8)> = inst.hints.iter().map(|h| (h.pos, h.rot)).collect();
    e2_io::viewer_url(&inst.name, inst.width, cells, &hints)
}

/// Convert a v2 `eternity2_core::Board` to the row-major `piece*4 + rot` board.
pub fn core_board_to_vec(board: &Board) -> Vec<i32> {
    board
        .cells()
        .iter()
        .map(|c| match c {
            Some((pid, rot)) => *pid as i32 * 4 + rot.as_u8() as i32,
            None => -1,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Resolved relative to this crate so the test runs from any checkout.
    const OFFICIAL_CSV: &str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/puzzles/size_16_official_eternity.csv");

    #[test]
    fn official_loads_256_pieces_5_hints() {
        let inst = SiteInstance::official(OFFICIAL_CSV).unwrap();
        assert_eq!(inst.pieces.len(), 256);
        assert_eq!(inst.hints.len(), 5);
        assert_eq!(inst.max_score(), 480);
    }

    #[test]
    fn empty_board_scores_zero() {
        let inst = SiteInstance::official(OFFICIAL_CSV).unwrap();
        let out = inst.finish(vec![-1; 256]);
        assert_eq!(out.score, 0);
        assert!(out.url.starts_with("https://eternity2.dev/viewer?"));
        assert!(out.url.matches('a').count() >= 1024);
    }

    // McGavin/Blackwood 469/480 board, bucas board_edges (from the site engine's
    // interop test). Used to cross-validate our scorer against the known 469.
    const MCGAVIN_469_EDGES: &str = "adcaaendadweadhdafwdafgfaemfacqeaencafqeadofaepdaeueaeseadteaacdcnfansnnwuwshlsuwvvlgsgvmtrsqvgtntqvkkntokvkpllkulplsnhltovncacofqfangwqwwvgstgwvphtgnnprqungqmqqkwhuwokvwlwlrqwpnqrhsknvpnscadpfvcawkrvvrnkglorhuqlnvhuusuvmonpwnspouqnlkvuqmokqhwmkrphlslrdadscpcarqvpntmqopstqrqphmorulwmnkolsntkqoqnuvgoosnqwsoqphssoggkdadgcrfavmkrmupmsthuqqwtoriqwmmrorrmtrgrqrtrgtmrnlktogllsquggtnqdaftfoeaknvopgtnhhlgwushiowumnlorkmngtrktlqtmnwlkounllwougolnkpgfackelfavhnltrwhluqrsrtuwvlrlgvvmkigrwpkqlhwwokluisowquiooiqpprocaepfufanhhuwkuhqtjktpjtloupvmjoijjmpppjhhrpkprhskppuvtkitvvrvmteabvfhcahwjhuhmwjnthjronuhhrjqphjskqpgmsrusgrhouptwhtkmtvrtkmhmrbafhcsbajwgsmsowtjisoqtjhmkqpjnmkgrjmpqgsuvpoujuwovummsotlgmmtplfabtbteagiwtomniiwpmtwvwksmwnplsrilpqwoivkvwjhukvrkhsjprgwvjppvwbaepeibawilinqwiphiqvjshmlljlkillqikovjqvijvuksikgikpnigvjnnvomjeabobjfalijjwgiiisjgsksslqjkiigqisoijuhsjosuslgoirilijurnvijmqlvbaeqfjbajrijingrjminsutmjtrugigtommihjumsiujggjiimhgujvmijpjlthjeaetbdaaidadgcadidactcadrbacgbabmdabufadubafjbabhcabvbacpcabhbaceaab";

    #[test]
    fn scorer_agrees_with_known_469() {
        // Reconstruct the 469 board by matching each bucas cell to an unused
        // piece+rotation, then confirm our canonical scorer returns 469.
        let inst = SiteInstance::official(OFFICIAL_CSV).unwrap();
        let cells: Vec<[u8; 4]> = MCGAVIN_469_EDGES
            .as_bytes()
            .chunks(4)
            .map(|c| [c[0] - b'a', c[1] - b'a', c[2] - b'a', c[3] - b'a'])
            .collect();
        assert_eq!(cells.len(), 256);
        let mut used = vec![false; inst.pieces.len()];
        let mut board = vec![-1i32; 256];
        for (pos, &cell) in cells.iter().enumerate() {
            'find: for (pid, &e) in inst.pieces.iter().enumerate() {
                if used[pid] {
                    continue;
                }
                for r in 0..4u8 {
                    if rotated(e, r) == cell {
                        used[pid] = true;
                        board[pos] = pid as i32 * 4 + i32::from(r);
                        break 'find;
                    }
                }
            }
        }
        let out = inst.finish(board);
        assert_eq!(out.score, 469, "canonical scorer must reproduce the 469");
    }
}
