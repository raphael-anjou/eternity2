// 2x2 sub-tiling enumeration for Verhaard's set-tilability metric.
//
// Per groups.io 105190116 (louis.verhaard, 2008-04-11), the surrogate
// metric correlating R² = 0.65 with log(total full tilings) is the
// count of 2x2 sub-tilings exhausted on a given inner-piece set.
//
// A 2x2 sub-tiling is a placement of 4 inner pieces (each rotated to
// some Rotation r ∈ {R0,R90,R180,R270}) into the 2x2 grid:
//
//     TL  TR
//     BL  BR
//
// such that the 4 *internal* edges match colors:
//   TL.right  == TR.left
//   TL.bottom == BL.top
//   TR.bottom == BR.top
//   BL.right  == BR.left
//
// The 8 *external* edges are unconstrained. All four pieces must be
// distinct (no piece used twice).
//
// We count tilings — each distinct (piece, rotation) tuple per corner
// is a separate tiling. So a single set of 4 pieces can contribute up
// to 4 tilings if multiple rotations also satisfy the constraints.
//
// Performance: for an inner set of N=190 pieces, we enumerate via a
// pre-indexed lookup table:
//   by_left[c]  : list of (piece_id, rot, top, right, bottom) where
//                 piece's rotated `left` == c.
//   by_top[c]   : list of (piece_id, rot, top, right, bottom, left)
//                 where piece's rotated `top` == c.
//   by_lt[(l,t)]: list of (piece_id, rot, right, bottom) where left==l
//                 and top==t (used for BR lookup).
//
// Total count algorithm:
//   for each piece P in set:
//     for each rotation r:
//       let TL = (P, r), with (top, right, bottom, left) after rotation
//       for each TR in by_left[TL.right] excluding P:
//         for each BL in by_top[TL.bottom] excluding {P, TR}:
//           for each BR in by_lt[(BL.right, TR.bottom)] excluding {P, TR, BL}:
//             count += 1

#![allow(clippy::similar_names)]

use eternity2_core::{Color, Piece, PieceId, Rotation};

/// One rotated piece in the 2x2 index. We store the rotated edge
/// values directly rather than recomputing per lookup.
#[derive(Debug, Clone, Copy)]
pub struct RotatedPiece {
    pub piece_id: PieceId,
    pub rotation: u8,
    pub top: Color,
    pub right: Color,
    pub bottom: Color,
    pub left: Color,
}

/// Index over rotated inner pieces for fast 2x2 enumeration.
pub struct Tile2x2Index {
    /// All rotated entries (no dedup; the 4 rotations of each piece
    /// may produce distinct entries).
    pub all: Vec<RotatedPiece>,
    /// Indices into `all` grouped by rotated `left` color.
    by_left: Vec<Vec<u32>>,
    /// Indices into `all` grouped by rotated `top` color.
    by_top: Vec<Vec<u32>>,
    /// Indices into `all` grouped by (rotated `left`, rotated `top`).
    /// Flat 2D layout: [left * max_color + top].
    by_lt: Vec<Vec<u32>>,
    max_color: usize,
}

impl Tile2x2Index {
    /// Build the index from the inner-piece subset of a puzzle. The
    /// `pieces` slice may include non-inner pieces; those are filtered
    /// out internally.
    #[must_use]
    pub fn build(pieces: &[Piece], max_color: usize) -> Self {
        let mut all: Vec<RotatedPiece> = Vec::new();
        for p in pieces {
            if !p.is_inner() {
                continue;
            }
            for r in 0u8..4 {
                let rot = Rotation::from_u8(r).unwrap();
                let e = p.edges.rotated(rot);
                all.push(RotatedPiece {
                    piece_id: p.id,
                    rotation: r,
                    top: e.top(),
                    right: e.right(),
                    bottom: e.bottom(),
                    left: e.left(),
                });
            }
        }
        let mc = max_color + 1;
        let mut by_left: Vec<Vec<u32>> = vec![Vec::new(); mc];
        let mut by_top: Vec<Vec<u32>> = vec![Vec::new(); mc];
        let mut by_lt: Vec<Vec<u32>> = vec![Vec::new(); mc * mc];
        for (i, rp) in all.iter().enumerate() {
            by_left[rp.left as usize].push(i as u32);
            by_top[rp.top as usize].push(i as u32);
            by_lt[(rp.left as usize) * mc + (rp.top as usize)].push(i as u32);
        }
        Self {
            all,
            by_left,
            by_top,
            by_lt,
            max_color: mc,
        }
    }

    /// Count 2x2 sub-tilings whose four pieces are all in the
    /// `in_set` mask (indexed by piece id). Inclusive: each distinct
    /// (TL rot, TR rot, BL rot, BR rot) ordered tuple counts once.
    pub fn count_total(&self, in_set: &[bool]) -> u64 {
        let mut total: u64 = 0;
        for tl_idx in 0..self.all.len() {
            let tl = &self.all[tl_idx];
            if !in_set[tl.piece_id as usize] {
                continue;
            }
            total += self.count_with_tl(tl_idx, in_set);
        }
        total
    }

    /// Count 2x2 sub-tilings anchored with piece-rotation `tl_idx`
    /// (a position in `self.all`) at the TL corner, with the other 3
    /// drawn from `in_set`.
    #[inline]
    fn count_with_tl(&self, tl_idx: usize, in_set: &[bool]) -> u64 {
        let tl = &self.all[tl_idx];
        let pid_tl = tl.piece_id;
        let mut count: u64 = 0;
        // TR candidates: rotated.left == tl.right.
        for &tr_i in &self.by_left[tl.right as usize] {
            let tr = &self.all[tr_i as usize];
            if tr.piece_id == pid_tl || !in_set[tr.piece_id as usize] {
                continue;
            }
            // BL candidates: rotated.top == tl.bottom.
            for &bl_i in &self.by_top[tl.bottom as usize] {
                let bl = &self.all[bl_i as usize];
                if bl.piece_id == pid_tl
                    || bl.piece_id == tr.piece_id
                    || !in_set[bl.piece_id as usize]
                {
                    continue;
                }
                // BR candidates: rotated.left == bl.right AND rotated.top == tr.bottom.
                let key = (bl.right as usize) * self.max_color + (tr.bottom as usize);
                if key >= self.by_lt.len() {
                    continue;
                }
                for &br_i in &self.by_lt[key] {
                    let br = &self.all[br_i as usize];
                    if br.piece_id == pid_tl
                        || br.piece_id == tr.piece_id
                        || br.piece_id == bl.piece_id
                        || !in_set[br.piece_id as usize]
                    {
                        continue;
                    }
                    count += 1;
                }
            }
        }
        count
    }

    /// Count 2x2 sub-tilings involving piece `pid` somewhere (anywhere
    /// among the 4 corners), with all four pieces from `in_set`. This
    /// is `participates(pid, in_set)`.
    ///
    /// Implementation: anchor `pid` at each of TL/TR/BL/BR in turn.
    /// Sum the counts. Since a single 2x2 tiling has 4 *distinct*
    /// pieces (one at each corner), no over-counting: each 2x2 instance
    /// contributes exactly one anchor-position to pid's count (the
    /// position where pid actually sits).
    pub fn participates(&self, pid: PieceId, in_set: &[bool]) -> u64 {
        let mut count: u64 = 0;
        // For each rotation of `pid` (only those in self.all; non-inner
        // pieces aren't in the index).
        for (rp_idx, rp) in self.all.iter().enumerate() {
            if rp.piece_id != pid {
                continue;
            }
            // anchor as TL
            count += self.count_with_tl(rp_idx, in_set);
            // anchor as TR: TR has left=tl.right. TL is whatever
            // sits to the left. Iterate all candidate TLs whose
            // right == rp.left. (Symmetric search.)
            //
            // Implementing TR/BL/BR anchoring directly is symmetric
            // to TL anchoring. We do it inline for clarity.
            count += self.count_with_tr(rp_idx, in_set);
            count += self.count_with_bl(rp_idx, in_set);
            count += self.count_with_br(rp_idx, in_set);
        }
        count
    }

    #[inline]
    fn count_with_tr(&self, tr_idx: usize, in_set: &[bool]) -> u64 {
        let tr = &self.all[tr_idx];
        let pid_tr = tr.piece_id;
        let mut count: u64 = 0;
        // TL candidates: rotated.right == tr.left.
        // We don't have a by_right index; iterate all and filter, or
        // build a by_right at construction time. Build on demand:
        // for now, iterate by_left and check (slower); switch to
        // dedicated by_right if needed.
        //
        // OPTIMIZATION: build by_right and by_bottom at construction
        // for full symmetry.
        for tl_idx in 0..self.all.len() {
            let tl = &self.all[tl_idx];
            if tl.right != tr.left {
                continue;
            }
            if tl.piece_id == pid_tr || !in_set[tl.piece_id as usize] {
                continue;
            }
            // BL candidates: top == tl.bottom AND right such that
            // BR exists with left == bl.right AND top == tr.bottom.
            for &bl_i in &self.by_top[tl.bottom as usize] {
                let bl = &self.all[bl_i as usize];
                if bl.piece_id == pid_tr
                    || bl.piece_id == tl.piece_id
                    || !in_set[bl.piece_id as usize]
                {
                    continue;
                }
                let key = (bl.right as usize) * self.max_color + (tr.bottom as usize);
                if key >= self.by_lt.len() {
                    continue;
                }
                for &br_i in &self.by_lt[key] {
                    let br = &self.all[br_i as usize];
                    if br.piece_id == pid_tr
                        || br.piece_id == tl.piece_id
                        || br.piece_id == bl.piece_id
                        || !in_set[br.piece_id as usize]
                    {
                        continue;
                    }
                    count += 1;
                }
            }
        }
        count
    }

    #[inline]
    fn count_with_bl(&self, bl_idx: usize, in_set: &[bool]) -> u64 {
        let bl = &self.all[bl_idx];
        let pid_bl = bl.piece_id;
        let mut count: u64 = 0;
        // TL candidates: bottom == bl.top.
        for tl_idx in 0..self.all.len() {
            let tl = &self.all[tl_idx];
            if tl.bottom != bl.top {
                continue;
            }
            if tl.piece_id == pid_bl || !in_set[tl.piece_id as usize] {
                continue;
            }
            // TR candidates: left == tl.right.
            for &tr_i in &self.by_left[tl.right as usize] {
                let tr = &self.all[tr_i as usize];
                if tr.piece_id == pid_bl
                    || tr.piece_id == tl.piece_id
                    || !in_set[tr.piece_id as usize]
                {
                    continue;
                }
                // BR: left == bl.right AND top == tr.bottom.
                let key = (bl.right as usize) * self.max_color + (tr.bottom as usize);
                if key >= self.by_lt.len() {
                    continue;
                }
                for &br_i in &self.by_lt[key] {
                    let br = &self.all[br_i as usize];
                    if br.piece_id == pid_bl
                        || br.piece_id == tl.piece_id
                        || br.piece_id == tr.piece_id
                        || !in_set[br.piece_id as usize]
                    {
                        continue;
                    }
                    count += 1;
                }
            }
        }
        count
    }

    #[inline]
    fn count_with_br(&self, br_idx: usize, in_set: &[bool]) -> u64 {
        let br = &self.all[br_idx];
        let pid_br = br.piece_id;
        let mut count: u64 = 0;
        // TR candidates: bottom == br.top.
        for tr_idx in 0..self.all.len() {
            let tr = &self.all[tr_idx];
            if tr.bottom != br.top {
                continue;
            }
            if tr.piece_id == pid_br || !in_set[tr.piece_id as usize] {
                continue;
            }
            // BL candidates: right == br.left.
            for bl_idx in 0..self.all.len() {
                let bl = &self.all[bl_idx];
                if bl.right != br.left {
                    continue;
                }
                if bl.piece_id == pid_br
                    || bl.piece_id == tr.piece_id
                    || !in_set[bl.piece_id as usize]
                {
                    continue;
                }
                // TL: right == tr.left AND bottom == bl.top.
                // Need by_right_bottom lookup; iterate by_left[tr.left]
                // and filter on bottom == bl.top.
                for &tl_i in &self.by_left[tr.left as usize] {
                    let tl = &self.all[tl_i as usize];
                    if tl.bottom != bl.top {
                        continue;
                    }
                    if tl.piece_id == pid_br
                        || tl.piece_id == tr.piece_id
                        || tl.piece_id == bl.piece_id
                        || !in_set[tl.piece_id as usize]
                    {
                        continue;
                    }
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_core::{Edges, Piece};

    fn p(id: u16, t: Color, r: Color, b: Color, l: Color) -> Piece {
        Piece::new(id, Edges::new(t, r, b, l))
    }

    #[test]
    fn empty_set_has_zero_tilings() {
        let pieces = vec![p(0, 1, 2, 3, 4)];
        let idx = Tile2x2Index::build(&pieces, 5);
        let in_set = vec![false; 10];
        assert_eq!(idx.count_total(&in_set), 0);
    }

    #[test]
    fn four_pieces_that_tile_count_one_or_more() {
        // Build a known 2x2 with all matching internal edges. Internal
        // edges in 2x2: 4 edges (TL-TR, TL-BL, TR-BR, BL-BR).
        // Let's set them all to color 5; external edges arbitrary.
        //
        //   TL=(t=1, r=5, b=5, l=2)
        //   TR=(t=3, r=4, b=5, l=5)
        //   BL=(t=5, r=5, b=6, l=7)
        //   BR=(t=5, r=8, b=9, l=5)
        let pieces = vec![
            p(0, 1, 5, 5, 2),
            p(1, 3, 4, 5, 5),
            p(2, 5, 5, 6, 7),
            p(3, 5, 8, 9, 5),
        ];
        let idx = Tile2x2Index::build(&pieces, 10);
        let mut in_set = vec![false; 10];
        for i in 0..4 { in_set[i] = true; }
        let total = idx.count_total(&in_set);
        // At least one 2x2 (the canonical orientation). May find more
        // because the same 4 pieces might also tile in other rotations.
        assert!(total >= 1, "expected ≥1 tiling, got {total}");
    }

    #[test]
    fn participates_matches_total_count() {
        // For any set, sum_p participates(p, set) == 4 * total(set).
        let pieces = vec![
            p(0, 1, 5, 5, 2),
            p(1, 3, 4, 5, 5),
            p(2, 5, 5, 6, 7),
            p(3, 5, 8, 9, 5),
            // a 5th piece off-pattern
            p(4, 1, 1, 1, 1),
        ];
        let idx = Tile2x2Index::build(&pieces, 10);
        let mut in_set = vec![false; 10];
        for i in 0..5 { in_set[i] = true; }
        let total = idx.count_total(&in_set);
        let part_sum: u64 = (0..5u16).map(|pid| idx.participates(pid, &in_set)).sum();
        assert_eq!(part_sum, 4 * total, "sum participates = 4 * total invariant violated");
    }
}
