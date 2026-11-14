//! Dead-node refutation census (see `../PREREG.md`).
//!
//! `tree` replays `bb-mrv` (A16 `BbSolver { rule: Rule::Mrv }`) with hooks;
//! `oracles` holds the region checks B, A, E, D, C run at sampled dead nodes.

pub mod oracles;
pub mod prop;
pub mod tree;

use eternity2_engine::{rotated, Puzzle, BORDER};
use sat_backdoors::Model;

pub fn set_bit(v: &mut [u64], i: usize) {
    v[i >> 6] |= 1u64 << (i & 63);
}

/// Iterate the set bits of a bitset slice.
pub fn bits(v: &[u64]) -> impl Iterator<Item = usize> + '_ {
    v.iter().enumerate().flat_map(|(k, &w)| {
        let mut x = w;
        std::iter::from_fn(move || {
            if x == 0 {
                None
            } else {
                let b = x.trailing_zeros() as usize;
                x &= x - 1;
                Some(k * 64 + b)
            }
        })
    })
}

/// splitmix64 stream.
#[derive(Clone)]
pub struct Rng(pub u64);
impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    pub fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// Static data the oracles share, rebuilt from the model's public fields.
pub struct Ctx<'a> {
    pub m: &'a Model,
    pub n: usize,
    /// side_mask[d][col]: options with colour col on side d.
    pub side_mask: Vec<Vec<Vec<u64>>>,
    /// piece_mask[p]: options of piece p.
    pub piece_mask: Vec<Vec<u64>>,
    /// Unary rim mask per cell (cells * words).
    pub unary: Vec<u64>,
}

impl<'a> Ctx<'a> {
    #[must_use]
    pub fn new(m: &'a Model) -> Self {
        let w = m.words;
        let mut side_mask = vec![vec![vec![0u64; w]; m.num_colors]; 4];
        for (o, e) in m.opt_edges.iter().enumerate() {
            for (d, sm) in side_mask.iter_mut().enumerate() {
                set_bit(&mut sm[e[d] as usize], o);
            }
        }
        let mut piece_mask = vec![vec![0u64; w]; m.piece_range.len()];
        for (pi, &(s, t)) in m.piece_range.iter().enumerate() {
            for o in s..t {
                set_bit(&mut piece_mask[pi], o);
            }
        }
        let n_opt = m.opt_edges.len();
        let mut unary = vec![0u64; m.cells * w];
        for c in 0..m.cells {
            for k in 0..w {
                let mut v = !0u64;
                for d in 0..4 {
                    let grey = side_mask[d][BORDER as usize][k];
                    v &= if m.neighbour[c][d].is_none() {
                        grey
                    } else {
                        !grey
                    };
                }
                if k == w - 1 && n_opt % 64 != 0 {
                    v &= (1u64 << (n_opt % 64)) - 1;
                }
                unary[c * w + k] = v;
            }
        }
        Self {
            m,
            n: m.n,
            side_mask,
            piece_mask,
            unary,
        }
    }
}

/// A search state: propagated domains plus which cells the search assigned.
#[derive(Clone)]
pub struct State {
    pub d: Vec<u64>,
    pub assigned: Vec<bool>,
}

impl State {
    /// Option at an assigned cell (its domain is a singleton).
    #[must_use]
    pub fn opt_at(&self, m: &Model, c: usize) -> usize {
        bits(m.dom(&self.d, c))
            .next()
            .expect("assigned cell has an option")
    }
    /// Bitmask of pieces on assigned cells.
    #[must_use]
    pub fn used(&self, m: &Model) -> u128 {
        let mut u = 0u128;
        for c in 0..m.cells {
            if self.assigned[c] {
                u |= 1u128 << m.opt_piece[self.opt_at(m, c)];
            }
        }
        u
    }
    #[must_use]
    pub fn empties(&self) -> usize {
        self.assigned.iter().filter(|&&a| !a).count()
    }
}

/// Option index of the planted (piece, rotation) at each cell.
#[must_use]
pub fn planted_options(m: &Model, p: &Puzzle, planted: &[i32]) -> Vec<usize> {
    planted
        .iter()
        .map(|&v| {
            let (pi, r) = ((v / 4) as usize, (v % 4) as u8);
            let e = rotated(p.pieces[pi], r);
            let (s, t) = m.piece_range[pi];
            (s..t)
                .find(|&o| m.opt_edges[o] == e)
                .expect("planted option")
        })
        .collect()
}
