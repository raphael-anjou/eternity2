// Houdayer cluster move adapted from spin glasses to E2's permutation setting.
//
// Original (Houdayer 2001, Ising ±J spin glass):
//   - Pair two replicas A, B at the same temperature.
//   - Build the "disagreement graph": sites where A_i != B_i.
//   - Find a connected component C.
//   - Swap A_C ↔ B_C jointly. Energy on the C-boundary is preserved because
//     outside C the two replicas already agree. Internal energy redistributes
//     between the two replicas — globally the joint energy is conserved up
//     to internal contributions, which allows traversal between basins that
//     single-site moves cannot reach.
//
// Adaptation to E2:
//   - Pieces are PERMUTED, not free spins. Naively swapping A's cells in C
//     with B's would give A duplicate pieces. Fix: require the multiset of
//     pieces in {A[r] : r ∈ C} to equal the multiset in {B[r] : r ∈ C}.
//     Then the swap is a pure rearrangement of the *same* piece set within
//     each replica, separately.
//   - Outside C, the cells are identical in A and B by construction, so
//     external edges to C are unchanged on both sides.  The only edges
//     that change are those internal to C and on the C-boundary in BOTH
//     replicas (where the inside-cell changes but the outside-cell is the
//     same).
//
// Cost function: total matched edges across the joint (A, B). A Houdayer
// move is a swap A_C ↔ B_C; we accept if it improves either replica's
// score (or both), or by Metropolis at the shared T.
//
// This file ships the move primitive: enumerate disagreement components,
// filter to those with matching piece multisets, score the swap, return
// the proposal. The PT loop decides when to call it and how often.

use std::collections::BTreeMap;

use eternity2_core::{Board, PieceId, Puzzle, Rotation};

#[derive(Debug, Clone)]
pub struct HoudayerProposal {
    pub component: Vec<u32>,         // cell positions in the disagreement component
    pub a_delta: i32,                // change in A's matched-edge score if applied
    pub b_delta: i32,                // change in B's matched-edge score if applied
    pub joint_delta: i32,            // a_delta + b_delta
}

// Build the disagreement graph as connected components on the grid.
// Two cells are adjacent (share an edge in the disagreement graph) iff
// they are 4-neighbors on the grid AND they both disagree between A and B.
pub fn disagreement_components(puzzle: &Puzzle, a: &Board, b: &Board) -> Vec<Vec<u32>> {
    let w = puzzle.width as i32;
    let h = puzzle.height as i32;
    let n = (w * h) as usize;

    let mut disagree = vec![false; n];
    for pos in 0..n {
        if a.get(pos as u32) != b.get(pos as u32) {
            disagree[pos] = true;
        }
    }

    let mut visited = vec![false; n];
    let mut components: Vec<Vec<u32>> = Vec::new();
    let mut stack: Vec<i32> = Vec::new();

    for start in 0..n {
        if !disagree[start] || visited[start] { continue; }
        visited[start] = true;
        stack.clear();
        stack.push(start as i32);
        let mut comp = Vec::new();
        while let Some(p) = stack.pop() {
            comp.push(p as u32);
            let x = p % w;
            let y = p / w;
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || nx >= w || ny < 0 || ny >= h { continue; }
                let q = (ny * w + nx) as usize;
                if disagree[q] && !visited[q] {
                    visited[q] = true;
                    stack.push(q as i32);
                }
            }
        }
        components.push(comp);
    }
    components
}

// A component is "piece-set-swappable" if the multiset of pieces in
// {A[r] : r ∈ C} equals the multiset in {B[r] : r ∈ C}.  Rotation is
// not part of the multiset key because pieces themselves are the
// conserved quantity; the swap is allowed to change rotations.
pub fn component_is_swappable(a: &Board, b: &Board, comp: &[u32]) -> bool {
    let mut a_count: BTreeMap<PieceId, u32> = BTreeMap::new();
    let mut b_count: BTreeMap<PieceId, u32> = BTreeMap::new();
    for &pos in comp {
        if let Some((pid, _)) = a.get(pos) { *a_count.entry(pid).or_insert(0) += 1; }
        if let Some((pid, _)) = b.get(pos) { *b_count.entry(pid).or_insert(0) += 1; }
    }
    a_count == b_count
}

// Score the internal + boundary edge delta if we replace A[r] with B[r]
// for every r in `comp`. Edges outside the component are unchanged
// (the cells they connect are unchanged); edges on the boundary between
// inside-C and outside-C may change because the inside-cell changes;
// edges fully inside C also change.
//
// Returns (new_minus_old) in matched-edge count for A. Identical
// machinery applies to B by symmetry (with A and B swapped).
pub fn delta_replace_with(puzzle: &Puzzle, a: &Board, b: &Board, comp: &[u32]) -> i32 {
    use std::collections::HashSet;
    let w = puzzle.width as i32;
    let h = puzzle.height as i32;
    let comp_set: HashSet<u32> = comp.iter().copied().collect();

    // Collect each (lower, upper) ordered edge pair touching `comp` exactly
    // once. We score that edge under A (current) and under "A with C
    // replaced by B" (proposed), and accumulate the delta.
    let mut edges: HashSet<(u32, u32)> = HashSet::new();
    for &pos in comp {
        let p = pos as i32;
        let x = p % w;
        let y = p / w;
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || nx >= w || ny < 0 || ny >= h { continue; }
            let q = (ny * w + nx) as u32;
            let key = if pos < q { (pos, q) } else { (q, pos) };
            edges.insert(key);
        }
    }

    let lookup = |id: PieceId| -> Option<&eternity2_core::Piece> {
        puzzle.piece(id)
    };

    let edge_match_in = |get_cell: &dyn Fn(u32) -> Option<(PieceId, Rotation)>, lo: u32, hi: u32| -> u32 {
        let (Some((p1, r1)), Some((p2, r2))) = (get_cell(lo), get_cell(hi)) else { return 0; };
        let (Some(pp1), Some(pp2)) = (lookup(p1), lookup(p2)) else { return 0; };
        let e1 = pp1.edges.rotated(r1).as_array();
        let e2 = pp2.edges.rotated(r2).as_array();
        let xl = (lo as i32) % w;
        let yl = (lo as i32) / w;
        let xh = (hi as i32) % w;
        let yh = (hi as i32) / w;
        if xh == xl + 1 && yh == yl {
            // horizontal: lo's right vs hi's left
            if e1[1] == e2[3] && e1[1] != 0 { 1 } else { 0 }
        } else if yh == yl + 1 && xh == xl {
            // vertical: lo's bottom vs hi's top
            if e1[2] == e2[0] && e1[2] != 0 { 1 } else { 0 }
        } else {
            0
        }
    };

    let a_get = |pos: u32| -> Option<(PieceId, Rotation)> { a.get(pos) };
    let b_get = |pos: u32| -> Option<(PieceId, Rotation)> { b.get(pos) };
    let proposed_get = |pos: u32| -> Option<(PieceId, Rotation)> {
        if comp_set.contains(&pos) { b_get(pos) } else { a_get(pos) }
    };

    let mut delta = 0i32;
    for (lo, hi) in &edges {
        let cur = edge_match_in(&a_get, *lo, *hi) as i32;
        let new = edge_match_in(&proposed_get, *lo, *hi) as i32;
        delta += new - cur;
    }
    delta
}

// Find all swappable components and rank them by joint score delta.
// Returns proposals sorted by joint_delta descending. Callers may
// keep the top-K or accept all positive-delta ones.
pub fn enumerate_proposals(puzzle: &Puzzle, a: &Board, b: &Board) -> Vec<HoudayerProposal> {
    let comps = disagreement_components(puzzle, a, b);
    let mut proposals = Vec::new();
    for comp in comps {
        if comp.len() < 2 { continue; }
        if !component_is_swappable(a, b, &comp) { continue; }
        let a_delta = delta_replace_with(puzzle, a, b, &comp);
        let b_delta = delta_replace_with(puzzle, b, a, &comp);
        proposals.push(HoudayerProposal {
            component: comp,
            a_delta,
            b_delta,
            joint_delta: a_delta + b_delta,
        });
    }
    proposals.sort_by(|p, q| q.joint_delta.cmp(&p.joint_delta));
    proposals
}

// Apply a proposal to (a, b): swap a_C ↔ b_C in place.
pub fn apply_proposal(a: &mut Board, b: &mut Board, proposal: &HoudayerProposal) {
    for &pos in &proposal.component {
        let av = a.get(pos);
        let bv = b.get(pos);
        match (av, bv) {
            (Some((pa, ra)), Some((pb, rb))) => {
                a.place(pos, pb, rb);
                b.place(pos, pa, ra);
            }
            (None, None) => {}
            (Some((pa, ra)), None) => { b.place(pos, pa, ra); a.clear(pos); }
            (None, Some((pb, rb))) => { a.place(pos, pb, rb); b.clear(pos); }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_core::{Edges, Piece, BORDER};

    fn tiny_puzzle() -> Puzzle {
        // 2x2 puzzle, 4 pieces, all with distinct interior colors so we
        // can construct two different board states sharing piece sets.
        let pieces = vec![
            Piece::new(0, Edges::new(BORDER, 1, 2, BORDER)),
            Piece::new(1, Edges::new(BORDER, BORDER, 3, 1)),
            Piece::new(2, Edges::new(2, 4, BORDER, BORDER)),
            Piece::new(3, Edges::new(3, BORDER, BORDER, 4)),
        ];
        Puzzle::new(2, 2, 5, pieces).unwrap()
    }

    #[test]
    fn empty_boards_no_disagreement() {
        let p = tiny_puzzle();
        let a = Board::empty(&p);
        let b = Board::empty(&p);
        assert!(disagreement_components(&p, &a, &b).is_empty());
    }

    #[test]
    fn detects_disagreement_component() {
        let p = tiny_puzzle();
        let mut a = Board::empty(&p);
        let mut b = Board::empty(&p);
        a.place(0, 0, Rotation::R0);
        a.place(1, 1, Rotation::R0);
        b.place(0, 1, Rotation::R0);
        b.place(1, 0, Rotation::R0);
        let comps = disagreement_components(&p, &a, &b);
        assert_eq!(comps.len(), 1);
        assert_eq!(comps[0].len(), 2);
    }

    #[test]
    fn swap_validity_check() {
        let p = tiny_puzzle();
        let mut a = Board::empty(&p);
        let mut b = Board::empty(&p);
        a.place(0, 0, Rotation::R0);
        a.place(1, 1, Rotation::R0);
        b.place(0, 1, Rotation::R0);
        b.place(1, 0, Rotation::R0);
        let comp: Vec<u32> = vec![0, 1];
        assert!(component_is_swappable(&a, &b, &comp));
    }

    #[test]
    fn swap_non_validity_when_piece_sets_differ() {
        let p = tiny_puzzle();
        let mut a = Board::empty(&p);
        let mut b = Board::empty(&p);
        a.place(0, 0, Rotation::R0);
        a.place(1, 1, Rotation::R0);
        b.place(0, 2, Rotation::R0); // different piece — not swappable
        b.place(1, 3, Rotation::R0);
        let comp: Vec<u32> = vec![0, 1];
        assert!(!component_is_swappable(&a, &b, &comp));
    }
}
