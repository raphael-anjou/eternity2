// Vol-17 idea D — Zobrist hash for E2 boards.
//
// A Zobrist hash maintains an O(1)-updatable 64-bit fingerprint of a
// fully-placed board as `XOR_{pos} table[pos][piece_id][rotation]`. On
// a swap of two cells (a, b), the fingerprint update is
//   h' = h
//        ^ table[a][old_a_pid][old_a_rot]
//        ^ table[a][new_a_pid][new_a_rot]
//        ^ table[b][old_b_pid][old_b_rot]
//        ^ table[b][new_b_pid][new_b_rot]
// independent of board size. So an LRU "tabu" set keyed on the
// fingerprint can detect chain stagnation in PT (vol-14 finding:
// pt_e2 lacks any explicit tabu mechanism; cold chains can drift in
// iso-score plateaus undetected).
//
// Storage: table[pos * MAX_PIECES * 4 + piece_id * 4 + rotation].
// MAX_PIECES = 256, board cells = 256 → ~262144 u64 = 2 MiB. Fine.
//
// Determinism: the table is generated from a deterministic 64-bit
// SplitMix seed so two runs with the same seed produce identical
// fingerprints. By default the seed is a constant baked in here.
//
// API contract:
//   - `BoardZobrist::new(puzzle, seed)` builds the random table.
//   - `fingerprint(&board)` computes h from scratch (O(n_cells)).
//   - `xor_place(pos, pid, rot)` returns the diff to XOR in/out for
//     placing/removing a piece at `pos`. Caller maintains its own
//     running h via h ^= xor_place(...) on every change.

#![allow(clippy::cast_possible_truncation)]

use eternity2_core::{Board, PieceId, Position, Puzzle, Rotation};

/// Maximum supported `piece_id` (canonical E2 has 256). Trades 4× more
/// table memory for fewer branches; safe to bump.
pub const ZOBRIST_MAX_PIECES: usize = 1024;

#[derive(Debug, Clone)]
pub struct BoardZobrist {
    n_cells: usize,
    table: Vec<u64>,
}

impl BoardZobrist {
    /// Construct a Zobrist table sized for `puzzle.cell_count()` cells
    /// and `ZOBRIST_MAX_PIECES` piece-ids, seeded by `seed`. Same seed
    /// produces same table.
    #[must_use]
    pub fn new(puzzle: &Puzzle, seed: u64) -> Self {
        let n_cells = puzzle.cell_count() as usize;
        let size = n_cells * ZOBRIST_MAX_PIECES * 4;
        let mut table = Vec::with_capacity(size);
        let mut state: u64 = seed.wrapping_add(0x9E3779B97F4A7C15);
        // SplitMix64 — small, deterministic, no_std friendly.
        for _ in 0..size {
            let mut z = state;
            state = state.wrapping_add(0x9E3779B97F4A7C15);
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
            z ^= z >> 31;
            table.push(z);
        }
        Self { n_cells, table }
    }

    /// XOR contribution of placing `(piece_id, rotation)` at `pos`.
    /// The same value is XORed when removing the placement.
    #[inline(always)]
    #[must_use]
    pub fn xor_place(&self, pos: Position, piece_id: PieceId, rotation: Rotation) -> u64 {
        let pid = usize::from(piece_id);
        debug_assert!(pid < ZOBRIST_MAX_PIECES, "piece_id exceeds ZOBRIST_MAX_PIECES");
        let idx = (pos as usize) * ZOBRIST_MAX_PIECES * 4
            + pid * 4
            + usize::from(rotation.as_u8());
        self.table[idx]
    }

    /// Compute the fingerprint of `board` from scratch.
    #[must_use]
    pub fn fingerprint(&self, board: &Board) -> u64 {
        let mut h = 0u64;
        for pos in 0..self.n_cells as u32 {
            if let Some((pid, rot)) = board.get(pos) {
                h ^= self.xor_place(pos, pid, rot);
            }
        }
        h
    }

    /// Diff for swapping the contents of two cells. `a_pid_rot` is the
    /// (piece_id, rotation) currently at position `a` (before the
    /// swap), likewise for `b`. After the swap, `a` will hold
    /// `b_pid_rot` and vice versa. The returned XOR diff captures all
    /// four contributions in one call.
    #[inline]
    #[must_use]
    pub fn xor_swap(
        &self,
        a: Position,
        a_pid_rot: (PieceId, Rotation),
        b: Position,
        b_pid_rot: (PieceId, Rotation),
    ) -> u64 {
        let (apid, arot) = a_pid_rot;
        let (bpid, brot) = b_pid_rot;
        // Removing a from pos a, removing b from pos b, adding b at a,
        // adding a at b. Each xor_place is its own self-inverse.
        self.xor_place(a, apid, arot)
            ^ self.xor_place(b, bpid, brot)
            ^ self.xor_place(a, bpid, brot)
            ^ self.xor_place(b, apid, arot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_core::{Edges, Piece};

    fn tiny_puzzle() -> Puzzle {
        // 3 × 3 puzzle, 9 pieces, all edges 0 (placeholder). We only
        // need the dimensions for the Zobrist table; piece edges don't
        // affect the hash.
        let pieces: Vec<Piece> = (0..9u16)
            .map(|i| Piece::new(i, Edges::new(0, 0, 0, 0)))
            .collect();
        Puzzle::new(3, 3, 8, pieces).unwrap()
    }

    #[test]
    fn empty_board_has_zero_fingerprint() {
        let p = tiny_puzzle();
        let z = BoardZobrist::new(&p, 0xDEAD_BEEF);
        let b = Board::empty(&p);
        assert_eq!(z.fingerprint(&b), 0);
    }

    #[test]
    fn placement_diff_is_self_inverse() {
        let p = tiny_puzzle();
        let z = BoardZobrist::new(&p, 0xDEAD_BEEF);
        let mut b = Board::empty(&p);
        b.place(0, 0, Rotation::R0);
        let h1 = z.fingerprint(&b);
        // Re-derive incrementally.
        let h_inc = z.xor_place(0, 0, Rotation::R0);
        assert_eq!(h1, h_inc);
        // Place then "remove" via xor again should return to zero.
        let h_back = h1 ^ h_inc;
        assert_eq!(h_back, 0);
    }

    #[test]
    fn swap_diff_matches_full_recompute() {
        let p = tiny_puzzle();
        let z = BoardZobrist::new(&p, 0xDEAD_BEEF);
        let mut b = Board::empty(&p);
        b.place(0, 1, Rotation::R0);
        b.place(4, 5, Rotation::R180);
        let h_before = z.fingerprint(&b);

        // Apply swap (0, 4) using xor_swap.
        let a_old = (1u16, Rotation::R0);
        let b_old = (5u16, Rotation::R180);
        let diff = z.xor_swap(0, a_old, 4, b_old);
        let h_after_incr = h_before ^ diff;

        // Actually swap on the board.
        b.place(0, 5, Rotation::R180);
        b.place(4, 1, Rotation::R0);
        let h_after_full = z.fingerprint(&b);

        assert_eq!(h_after_incr, h_after_full);
    }

    #[test]
    fn deterministic_under_seed() {
        let p = tiny_puzzle();
        let z1 = BoardZobrist::new(&p, 42);
        let z2 = BoardZobrist::new(&p, 42);
        assert_eq!(z1.table, z2.table);
    }

    #[test]
    fn different_seed_different_table() {
        let p = tiny_puzzle();
        let z1 = BoardZobrist::new(&p, 1);
        let z2 = BoardZobrist::new(&p, 2);
        assert_ne!(z1.table, z2.table);
    }
}
