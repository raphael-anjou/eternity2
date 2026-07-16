//! The working state a repair run mutates: a full board, its resolved-edge
//! cache, and — the piece that makes conflict-driven destroy cheap — an
//! incrementally-maintained map of which interior edges are currently broken.
//!
//! A repair loop, unlike a backtracker, always holds a *complete* board (every
//! cell filled). Its quality is the matched-edge score; the cells worth
//! attacking are the ones touching a mismatch. Recomputing the mismatch set from
//! scratch each iteration would dominate the loop, so this state keeps it live:
//! placing or clearing a cell updates only the edges incident to it. That is
//! what lets a destroy operator ask "which cells touch a broken edge?" in `O(1)`
//! per query instead of an `O(N)` rescan.

use e2_core::{rotated, Pieces, Rot, BORDER, EMPTY, H, N, W};

/// splitmix64 → xoshiro256**: the same fast, deterministic PRNG the DFS engine
/// uses, so the two studies share their randomness discipline. Seeded per run.
pub struct Rng {
    s: [u64; 4],
}

impl Rng {
    #[must_use]
    pub fn new(seed: u64) -> Self {
        let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut next = || {
            z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut x = z;
            x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            x ^ (x >> 31)
        };
        Self { s: [next(), next(), next(), next()] }
    }

    #[inline]
    pub const fn next_u64(&mut self) -> u64 {
        let r = self.s[0].wrapping_add(self.s[3]).rotate_left(23).wrapping_add(self.s[0]);
        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);
        r
    }

    /// A uniform index in `0..n` (n > 0).
    #[inline]
    pub fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }

    /// `true` with probability `p` (clamped to `[0,1]`), drawn to 24 bits.
    #[inline]
    pub fn chance(&mut self, p: f64) -> bool {
        let p = p.clamp(0.0, 1.0);
        let scale = f64::from(1u32 << 24);
        (self.next_u64() & 0x00FF_FFFF) < (p * scale) as u64
    }

    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = (self.next_u64() % (i as u64 + 1)) as usize;
            v.swap(i, j);
        }
    }
}

/// A placement: piece id and rotation. `None` is an empty cell.
pub type Cell = Option<(u16, Rot)>;

/// The mutable board state a repair run works on. Holds the placement grid, the
/// resolved URDL edges per cell (so neighbour colors are array reads, never a
/// rotation recompute on the hot path), and the live count of broken interior
/// edges incident to each cell.
pub struct State<'a> {
    pub pieces: &'a Pieces,
    /// Piece id per cell, [`EMPTY`] when unplaced.
    piece: [u16; N],
    rot: [Rot; N],
    /// Resolved URDL edges per cell; `[BORDER;4]` where empty.
    edges: [[u8; 4]; N],
    /// `used[piece_id]` — is this piece currently on the board?
    used: Vec<bool>,
    /// Number of broken (mismatched, non-border) interior edges incident to each
    /// cell. Maintained incrementally by [`Self::place`] / [`Self::clear`].
    conflicts: [u8; N],
    /// The current matched-edge score, kept in lockstep so it is never rescanned.
    score: u32,
}

impl<'a> State<'a> {
    /// Build a state from a full board of cell codes (`piece*4 + rot`, `-1`
    /// empty). Computes the edge cache, conflict map and score once up front.
    #[must_use]
    pub fn from_codes(pieces: &'a Pieces, codes: &[i32]) -> Self {
        let mut st = Self {
            pieces,
            piece: [EMPTY; N],
            rot: [0; N],
            edges: [[BORDER; 4]; N],
            used: vec![false; pieces.len()],
            conflicts: [0; N],
            score: 0,
        };
        for (pos, &v) in codes.iter().enumerate().take(N) {
            if v >= 0 {
                let (pid, r) = ((v / 4) as u16, (v % 4) as u8);
                st.piece[pos] = pid;
                st.rot[pos] = r;
                st.used[pid as usize] = true;
                st.edges[pos] = pieces.get(pid).map_or([BORDER; 4], |p| rotated(p.edges, r));
            }
        }
        st.recompute_derived();
        st
    }

    /// Full rescan of score + conflict map. Called once at construction; the
    /// incremental path keeps them current thereafter. It classifies every filled
    /// interior seam with the *same* [`seam_kind`] the incremental path uses, so a
    /// from-scratch recompute and the incremental map are identical by
    /// construction (the property the tests assert).
    fn recompute_derived(&mut self) {
        self.conflicts = [0; N];
        let mut score = 0u32;
        for y in 0..H {
            for x in 0..W {
                let pos = y * W + x;
                let a = self.edges[pos];
                if x + 1 < W && self.piece[pos] != EMPTY && self.piece[pos + 1] != EMPTY {
                    match seam_kind(a[1], self.edges[pos + 1][3]) {
                        SeamKind::Match => score += 1,
                        SeamKind::Conflict => {
                            self.conflicts[pos] += 1;
                            self.conflicts[pos + 1] += 1;
                        }
                        SeamKind::Neither => {}
                    }
                }
                if y + 1 < H && self.piece[pos] != EMPTY && self.piece[pos + W] != EMPTY {
                    match seam_kind(a[2], self.edges[pos + W][0]) {
                        SeamKind::Match => score += 1,
                        SeamKind::Conflict => {
                            self.conflicts[pos] += 1;
                            self.conflicts[pos + W] += 1;
                        }
                        SeamKind::Neither => {}
                    }
                }
            }
        }
        self.score = score;
    }

    #[inline]
    #[must_use]
    pub const fn score(&self) -> u32 {
        self.score
    }

    #[inline]
    #[must_use]
    pub const fn is_empty_at(&self, pos: usize) -> bool {
        self.piece[pos] == EMPTY
    }

    #[inline]
    #[must_use]
    pub const fn cell(&self, pos: usize) -> Cell {
        if self.piece[pos] == EMPTY {
            None
        } else {
            Some((self.piece[pos], self.rot[pos]))
        }
    }

    #[inline]
    #[must_use]
    pub fn is_used(&self, pid: u16) -> bool {
        self.used[pid as usize]
    }

    #[inline]
    #[must_use]
    pub const fn conflicts_at(&self, pos: usize) -> u8 {
        self.conflicts[pos]
    }

    /// The change in score `place(pos, pid, rot)` would produce, without
    /// mutating anything. Only the up/right/down/left seams of `pos` can change,
    /// so this is `O(1)`. Used by greedy repair to pick the best placement.
    #[must_use]
    pub fn delta_if_placed(&self, pos: usize, pid: u16, r: Rot) -> i32 {
        debug_assert!(self.piece[pos] == EMPTY);
        let e = self.pieces.get(pid).map_or([BORDER; 4], |p| rotated(p.edges, r));
        let mut delta = 0i32;
        for (dir, npos) in self.neighbours(pos) {
            let Some(npos) = npos else { continue };
            if self.piece[npos] == EMPTY {
                continue;
            }
            let (mine, theirs) = (e[dir], self.edges[npos][opposite(dir)]);
            if mine != BORDER && mine == theirs {
                delta += 1;
            }
        }
        delta
    }

    /// Place a piece into an empty cell, updating the edge cache, the conflict
    /// map on both endpoints of every incident seam, and the running score.
    pub fn place(&mut self, pos: usize, pid: u16, r: Rot) {
        debug_assert!(self.piece[pos] == EMPTY);
        debug_assert!(!self.used[pid as usize]);
        let e = self.pieces.get(pid).map_or([BORDER; 4], |p| rotated(p.edges, r));
        self.piece[pos] = pid;
        self.rot[pos] = r;
        self.used[pid as usize] = true;
        self.edges[pos] = e;
        for (dir, npos) in self.neighbours(pos) {
            let Some(npos) = npos else { continue };
            if self.piece[npos] == EMPTY {
                continue;
            }
            let (mine, theirs) = (e[dir], self.edges[npos][opposite(dir)]);
            match seam_kind(mine, theirs) {
                SeamKind::Match => self.score += 1,
                SeamKind::Conflict => {
                    self.conflicts[pos] += 1;
                    self.conflicts[npos] += 1;
                }
                SeamKind::Neither => {}
            }
        }
    }

    /// Lift the piece out of a cell, reversing every effect of [`Self::place`].
    /// Returns the `(piece, rot)` removed so a caller can restore or re-pool it.
    pub fn clear(&mut self, pos: usize) -> Option<(u16, Rot)> {
        if self.piece[pos] == EMPTY {
            return None;
        }
        let e = self.edges[pos];
        for (dir, npos) in self.neighbours(pos) {
            let Some(npos) = npos else { continue };
            if self.piece[npos] == EMPTY {
                continue;
            }
            let (mine, theirs) = (e[dir], self.edges[npos][opposite(dir)]);
            match seam_kind(mine, theirs) {
                SeamKind::Match => self.score -= 1,
                SeamKind::Conflict => {
                    self.conflicts[pos] -= 1;
                    self.conflicts[npos] -= 1;
                }
                SeamKind::Neither => {}
            }
        }
        let removed = (self.piece[pos], self.rot[pos]);
        self.used[removed.0 as usize] = false;
        self.piece[pos] = EMPTY;
        self.edges[pos] = [BORDER; 4];
        Some(removed)
    }

    /// The four neighbours of `pos` as `(my-edge-direction, Option<neighbour>)`,
    /// where the direction indexes URDL on *this* cell. Off-board neighbours are
    /// `None`.
    #[inline]
    const fn neighbours(&self, pos: usize) -> [(usize, Option<usize>); 4] {
        let (y, x) = (pos / W, pos % W);
        [
            (0, if y > 0 { Some(pos - W) } else { None }),     // up
            (1, if x + 1 < W { Some(pos + 1) } else { None }), // right
            (2, if y + 1 < H { Some(pos + W) } else { None }), // down
            (3, if x > 0 { Some(pos - 1) } else { None }),     // left
        ]
    }

    /// The board as row-major cell codes (`piece*4 + rot`, `-1` empty), for the
    /// canonical scorer and the bucas URL.
    #[must_use]
    pub fn to_codes(&self) -> Vec<i32> {
        (0..N)
            .map(|pos| {
                if self.piece[pos] == EMPTY {
                    -1
                } else {
                    i32::from(self.piece[pos]) * 4 + i32::from(self.rot[pos])
                }
            })
            .collect()
    }

    /// Cells currently incident to at least one broken interior edge — the
    /// candidate pool every conflict-driven destroy operator draws from.
    #[must_use]
    pub fn conflicted_cells(&self) -> Vec<usize> {
        (0..N).filter(|&p| self.conflicts[p] > 0).collect()
    }

    /// Test-only integrity check: does the incrementally-maintained score and
    /// conflict map exactly equal a from-scratch recompute of the current board?
    /// This is what guards against the incremental path leaking a counter over a
    /// long sequence of place/clear edits.
    #[cfg(test)]
    #[must_use]
    pub fn matches_recompute(&self) -> bool {
        let fresh = Self::from_codes(self.pieces, &self.to_codes());
        self.score == fresh.score && self.conflicts == fresh.conflicts
    }
}

/// The URDL index of the edge facing back from a neighbour reached in direction
/// `dir` (up↔down, right↔left).
#[inline]
const fn opposite(dir: usize) -> usize {
    match dir {
        0 => 2,
        1 => 3,
        2 => 0,
        _ => 1,
    }
}

/// How one interior seam between two filled cells counts. The classification is
/// **symmetric in `(a, b)`**, which is what makes the incremental conflict map
/// safe: `place` and `clear` visit a seam from opposite cells at different times,
/// so a non-symmetric rule (one that gated on only one side's edge) would
/// increment on `place` and fail to decrement on `clear`, leaking the counter.
///
/// * `Match`   — both edges are the same non-border color (this is exactly the
///   canonical scorer's rule, so the running score stays byte-identical to it).
/// * `Conflict` — both edges are real (non-border) colors that disagree: a
///   genuine interior color clash, the thing a repair loop wants to attack.
/// * `Neither` — at least one edge is the border color: a frame edge facing the
///   interior, which neither scores nor is a repairable clash.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SeamKind {
    Match,
    Conflict,
    Neither,
}

#[inline]
fn seam_kind(a: u8, b: u8) -> SeamKind {
    if a == BORDER || b == BORDER {
        SeamKind::Neither
    } else if a == b {
        SeamKind::Match
    } else {
        SeamKind::Conflict
    }
}
