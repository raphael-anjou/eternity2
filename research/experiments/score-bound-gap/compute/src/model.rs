//! Board model shared by the bound, the exact oracle and the brute force.
//!
//! Scoring convention: matched-edges, as in `engine::score_board` and
//! `BreakSolver`. A score counts matched interior edges (2n(n-1) at most). The
//! rim is structural and never scored: a piece may sit at a cell only in an
//! orientation whose grey sides face exactly the rim (the same hard rule as
//! `BreakSolver::break_cost`). Cells are filled in row-major order.

use eternity2_engine::{rotated, Puzzle, BORDER};

/// One structurally valid (piece, orientation) for a cell. `e` is URDL.
#[derive(Clone, Copy, Debug)]
pub struct Opt {
    pub p: u8,
    pub e: [u8; 4],
}

pub struct Model {
    pub n: usize,
    pub ncol: usize,
    pub pieces: Vec<[u8; 4]>,
    /// Per cell, every valid (piece, distinct orientation).
    pub opts: Vec<Vec<Opt>>,
}

pub fn structurally_valid(n: usize, pos: usize, e: [u8; 4]) -> bool {
    let (x, y) = (pos % n, pos / n);
    (y == 0) == (e[0] == BORDER)
        && (x == n - 1) == (e[1] == BORDER)
        && (y == n - 1) == (e[2] == BORDER)
        && (x == 0) == (e[3] == BORDER)
}

impl Model {
    pub fn new(p: &Puzzle) -> Model {
        let n = p.width as usize;
        assert_eq!(p.width, p.height, "square boards only");
        assert!(n * n <= 64, "piece sets are u64 bitsets");
        let ncol = p.num_colors as usize + 1;
        assert!(ncol <= 16, "colour masks are u16");
        let pieces: Vec<[u8; 4]> = p.pieces.clone();
        let mut opts = vec![Vec::new(); n * n];
        for pos in 0..n * n {
            for (pid, &pe) in pieces.iter().enumerate() {
                let mut seen: Vec<[u8; 4]> = Vec::new();
                for r in 0..4u8 {
                    let e = rotated(pe, r);
                    if seen.contains(&e) {
                        continue;
                    }
                    seen.push(e);
                    if structurally_valid(n, pos, e) {
                        opts[pos].push(Opt { p: pid as u8, e });
                    }
                }
            }
        }
        Model {
            n,
            ncol,
            pieces,
            opts,
        }
    }

    pub fn n_cells(&self) -> usize {
        self.n * self.n
    }

    /// Interior edges with at least one endpoint at index >= d (row-major).
    pub fn remaining_edges(&self, d: usize) -> u32 {
        let n = self.n;
        let mut k = 0;
        for pos in 0..n * n {
            let (x, y) = (pos % n, pos / n);
            if x + 1 < n && pos + 1 >= d {
                k += 1;
            }
            if y + 1 < n && pos + n >= d {
                k += 1;
            }
        }
        k
    }

    pub fn max_score(&self) -> u32 {
        (2 * self.n * (self.n - 1)) as u32
    }
}

/// A prefix: cells 0..d-1 filled, as (piece, URDL colours).
#[derive(Clone, Debug)]
pub struct Prefix {
    pub cells: Vec<Opt>,
}

impl Prefix {
    /// From `piece*4 + rot` codes (original piece ids).
    pub fn from_codes(m: &Model, codes: &[i32]) -> Prefix {
        let cells = codes
            .iter()
            .enumerate()
            .map(|(pos, &c)| {
                assert!(c >= 0);
                let p = (c / 4) as usize;
                let e = rotated(m.pieces[p], (c % 4) as u8);
                assert!(
                    structurally_valid(m.n, pos, e),
                    "prefix cell {pos} not structural"
                );
                Opt { p: p as u8, e }
            })
            .collect();
        Prefix { cells }
    }

    pub fn d(&self) -> usize {
        self.cells.len()
    }

    pub fn used(&self) -> u64 {
        self.cells.iter().fold(0u64, |a, o| a | (1u64 << o.p))
    }

    /// Matched interior edges between two prefix cells.
    pub fn score(&self, m: &Model) -> u32 {
        let n = m.n;
        let d = self.d();
        let mut s = 0;
        for pos in 0..d {
            let (x, y) = (pos % n, pos / n);
            if x > 0 && self.cells[pos - 1].e[1] == self.cells[pos].e[3] {
                s += 1;
            }
            if y > 0 && self.cells[pos - n].e[2] == self.cells[pos].e[0] {
                s += 1;
            }
        }
        s
    }

    /// Interior edges between two prefix cells.
    pub fn decided_edges(&self, m: &Model) -> u32 {
        m.max_score() - m.remaining_edges(self.d())
    }
}
