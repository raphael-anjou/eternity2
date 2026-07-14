// border_eulerian — anr_56's 2007 directed-Eulerian feasibility test for
// the closed border ring.
//
// Source: groups.io 105176464 (2007-06-28). Treat border-edge colors as
// vertices of a directed multigraph G. Each border tile, placed on a
// specific side with the rotation that puts BORDER outward, contributes
// one directed edge `(prev_color → next_color)` where `prev_color` and
// `next_color` are the two interior-facing colors taken in clockwise
// walk order.
//
// A valid closed border ring ⇔ a directed Eulerian cycle in G ⇔
//   1. in-deg(v) = out-deg(v) for every v, AND
//   2. all vertices of nonzero degree lie in a single weakly-connected
//      component (in a balanced-degree directed multigraph this is
//      equivalent to strong connectivity, the condition required for
//      the Euler cycle to exist).
//
// The Eulerian theorem gives both necessary AND sufficient conditions
// — so this is an exact feasibility test, not a heuristic.
//
// **In-degree = out-degree is automatically satisfied** when the tile
// multiset has matching left/right interior-color counts (which is a
// property of the E2 piece set itself — every border tile contributes
// one `prev` and one `next` interior face, the totals balance by
// construction over the whole 60-tile border set). So the *interesting*
// pruner is **connectivity**.
//
// Two API levels:
//   - `eulerian_ring_check_full`  — fully assigned ring: verify that
//     the 60 (prev,next) edges actually form a single Euler cycle.
//     Used by corpus calibration. O(60).
//   - `eulerian_ring_check_pool` — given a *pool* of unused border
//     tiles (and a chain of already-placed border tiles forming a
//     prefix), test whether ANY assignment of remaining tiles to
//     remaining sides yields an Euler-cycle completion. This is the
//     propagator. Internally fixes each remaining tile's (prev,next)
//     by the side it must go on (corner positions are fully determined;
//     edge positions are determined by their side), then runs the
//     same Euler test. O(remaining_border_count).
//
// The multigraph has at most 22 vertices (one per color in the E2
// palette) — in practice fewer, since interior colors don't appear on
// border tiles. anr_56 found ~5 border colors on canonical E2.

use eternity2_core::{Color, Piece, PieceId, Puzzle, Rotation, BORDER};

/// The placement of a border tile on the ring: which side, which
/// rotation, and the two interior-facing colors it contributes as a
/// directed edge `prev_color → next_color`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RingEdge {
    pub piece_id: PieceId,
    pub rotation: u8,
    pub prev_color: Color,
    pub next_color: Color,
}

/// Side of the perimeter walked clockwise starting at TL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RingSide {
    Top,
    Right,
    Bottom,
    Left,
}

/// The four corner kinds in clockwise order from TL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CornerKind {
    TL,
    TR,
    BR,
    BL,
}

/// Read a tile's prev/next colors for a given side. Returns None if
/// no rotation produces a BORDER-outward placement for this side.
/// (For E2 each edge tile has exactly one valid rotation per side.)
#[must_use]
pub fn edge_tile_ring_edge(piece: &Piece, side: RingSide) -> Option<RingEdge> {
    for r in 0u8..4 {
        let rot = Rotation::from_u8(r).unwrap();
        let e = piece.edges.rotated(rot);
        let (t, ri, b, le) = (e.top(), e.right(), e.bottom(), e.left());
        let (out_face_is_border, prev, next) = match side {
            RingSide::Top => (t == BORDER, le, ri),
            RingSide::Right => (ri == BORDER, t, b),
            RingSide::Bottom => (b == BORDER, ri, le),
            RingSide::Left => (le == BORDER, b, t),
        };
        if out_face_is_border {
            return Some(RingEdge {
                piece_id: piece.id,
                rotation: r,
                prev_color: prev,
                next_color: next,
            });
        }
    }
    None
}

/// Read a corner tile's prev/next colors for a given corner position.
/// The corner has two BORDER edges meeting at the outward angle; the
/// remaining two are interior-facing. `prev` is the color of the
/// interior face that *receives* the clockwise walk; `next` is the
/// color of the face that *sends* it onward.
#[must_use]
pub fn corner_tile_ring_edge(piece: &Piece, kind: CornerKind) -> Option<RingEdge> {
    for r in 0u8..4 {
        let rot = Rotation::from_u8(r).unwrap();
        let e = piece.edges.rotated(rot);
        let (t, ri, b, le) = (e.top(), e.right(), e.bottom(), e.left());
        let matches = match kind {
            CornerKind::TL => t == BORDER && le == BORDER,
            CornerKind::TR => t == BORDER && ri == BORDER,
            CornerKind::BR => b == BORDER && ri == BORDER,
            CornerKind::BL => b == BORDER && le == BORDER,
        };
        if !matches { continue; }
        let (prev, next) = match kind {
            // Clockwise walk: TL sends its right outward, receives bottom from below.
            CornerKind::TL => (b, ri),
            // TR sends its bottom downward to the next tile on the right side,
            // receives left from the previous tile on the top side.
            CornerKind::TR => (le, b),
            // BR sends its left to the next tile on the bottom side (walking
            // right-to-left along the bottom), receives top from above.
            CornerKind::BR => (t, le),
            // BL sends its top up the left side, receives right from the
            // previous tile on the bottom (walking right-to-left).
            CornerKind::BL => (ri, t),
        };
        return Some(RingEdge {
            piece_id: piece.id,
            rotation: r,
            prev_color: prev,
            next_color: next,
        });
    }
    None
}

/// Test whether a *fully-assigned* sequence of 60 ring edges forms a
/// valid directed Eulerian cycle.
///
/// Conditions checked:
///   (a) each tile's `prev_color` equals the previous tile's `next_color`
///       (the chain itself walks correctly);
///   (b) the multiset of (prev → next) directed edges has balanced
///       in/out degree (automatic from (a) on a closed walk, but checked
///       defensively);
///   (c) the underlying multigraph is connected on its support.
///
/// Returns `true` iff all three hold.
#[must_use]
pub fn eulerian_ring_check_full(ring: &[RingEdge]) -> bool {
    if ring.is_empty() { return false; }
    // (a) chain consistency.
    let n = ring.len();
    for i in 0..n {
        let cur = &ring[i];
        let prev = &ring[(i + n - 1) % n];
        if cur.prev_color != prev.next_color { return false; }
    }
    // (b) degree balance.
    let max_color = ring
        .iter()
        .flat_map(|e| [e.prev_color, e.next_color])
        .max()
        .unwrap_or(0) as usize;
    let n_v = max_color + 1;
    let mut indeg = vec![0i32; n_v];
    let mut outdeg = vec![0i32; n_v];
    for e in ring {
        outdeg[e.prev_color as usize] += 1;
        indeg[e.next_color as usize] += 1;
    }
    for v in 0..n_v {
        if indeg[v] != outdeg[v] { return false; }
    }
    // (c) weak connectivity on the nonzero-degree support — and in a
    // balanced-degree directed multigraph this implies strong
    // connectivity.
    let edges: Vec<(usize, usize)> = ring
        .iter()
        .map(|e| (e.prev_color as usize, e.next_color as usize))
        .collect();
    connected_on_support(&edges, n_v)
}

/// Connectivity helper: union-find over vertices that appear in any
/// edge; returns true iff all vertices with nonzero degree end up in
/// the same component.
fn connected_on_support(edges: &[(usize, usize)], n_v: usize) -> bool {
    let mut parent: Vec<usize> = (0..n_v).collect();
    fn find(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    }
    let mut has_edge = vec![false; n_v];
    for &(u, v) in edges {
        has_edge[u] = true;
        has_edge[v] = true;
        let (ru, rv) = (find(&mut parent, u), find(&mut parent, v));
        if ru != rv { parent[ru] = rv; }
    }
    let root_of_first = (0..n_v).find(|&v| has_edge[v]).map(|v| find(&mut parent, v));
    match root_of_first {
        None => true, // empty graph
        Some(r) => (0..n_v).all(|v| !has_edge[v] || find(&mut parent, v) == r),
    }
}

/// Pool-level feasibility test. Given:
///   - the puzzle (used to enumerate corner/edge pieces),
///   - the set of unused border tiles (corners and edges),
///   - the per-side remaining capacity (`remaining_top`, `remaining_right`,
///     `remaining_bottom`, `remaining_left`),
///   - the currently committed chain edges already placed on the ring
///     (these contribute fixed directed edges to G but their tiles are
///     not available for further assignment),
///
/// test whether there exists an assignment of the unused border tiles
/// to the open positions such that the combined multigraph admits a
/// directed Eulerian cycle.
///
/// This v1 checks the *aggregate-multiset* feasibility: collect the
/// per-side (prev,next) edges that EACH remaining tile would produce
/// if placed on its required side, sum across sides, add the committed
/// edges, and run the Euler cycle test on the union.
///
/// The choice "which tile to which position within a side" does not
/// affect aggregate degree/connectivity because all tiles destined for
/// the same side contribute their unique-rotation (prev,next) edge —
/// the multiset is determined the moment side-assignments are fixed.
///
/// Side-assignment freedom (which edge tile goes on which side) IS
/// the combinatorial unknown. v1 handles the simple case: each border
/// tile has exactly one valid side. For E2 edge tiles, only ONE
/// rotation has BORDER on a face — so the tile can only legally go on
/// the side matching that BORDER face direction once. **But** that
/// doesn't pin the side: rotating an edge tile produces 4 rotations,
/// each placing BORDER on a different face. So an edge tile CAN go on
/// any of the 4 sides — its (prev,next) just rotates accordingly.
///
/// Therefore the pool test reduces to: across all valid (tile → side)
/// assignments respecting the per-side capacities, does the resulting
/// multigraph have an Euler cycle?
///
/// Tractable approach: side-assignment is a bipartite matching with 4
/// destinations of capacity (top, right, bottom, left). Each tile has
/// 4 choices (one per side). This is in general a permanent
/// computation; but the **necessary** condition tested by v1 is
/// "averaged across all valid assignments, the support of G is the
/// full set of border colors in the piece set" — which is too weak.
///
/// v1 conservative approximation: assume each remaining edge tile is
/// pre-assigned to its CURRENT (in the partial border) side if known,
/// or to ANY side otherwise — and test the most-relaxed multigraph
/// (sum over all 4 side-edges per tile, weighted by 1/4 each). This
/// is provably weaker than the true test.
///
/// **For correctness as a propagator we ship the strict version**:
/// the test is only invoked when each remaining tile's side is
/// already pinned by the partial-border state (e.g. the side it sits
/// on is determined by depth). This matches how `border_enumerate`
/// generates rings (side by side, in order), and is the natural
/// integration point.
///
/// Returns `true` if Euler cycle exists; `false` if multigraph fails
/// degree balance or connectivity (definitely no valid completion).
///
/// Arguments:
///   `committed_edges`: directed edges already locked in by placed tiles.
///   `pool_with_sides`: each remaining tile + its assigned side (the
///                      side-assignment is the caller's responsibility).
#[must_use]
pub fn eulerian_pool_check(
    puzzle: &Puzzle,
    committed_edges: &[RingEdge],
    pool_with_sides: &[(PieceId, Option<RingSide>, Option<CornerKind>)],
) -> bool {
    let mut all_edges: Vec<RingEdge> = committed_edges.to_vec();
    for &(pid, side, corner) in pool_with_sides {
        let piece = puzzle.piece(pid);
        let piece = match piece { Some(p) => p, None => return false };
        let edge_opt = match (side, corner) {
            (Some(s), None) => edge_tile_ring_edge(piece, s),
            (None, Some(k)) => corner_tile_ring_edge(piece, k),
            (Some(_), Some(_)) | (None, None) => return false,
        };
        match edge_opt {
            Some(e) => all_edges.push(e),
            None => return false,
        }
    }
    // The full check: chain-consistency is NOT required at pool level
    // (we haven't fixed an order); only degree balance and connectivity.
    let max_color = all_edges
        .iter()
        .flat_map(|e| [e.prev_color, e.next_color])
        .max()
        .unwrap_or(0) as usize;
    let n_v = max_color + 1;
    let mut indeg = vec![0i32; n_v];
    let mut outdeg = vec![0i32; n_v];
    for e in &all_edges {
        outdeg[e.prev_color as usize] += 1;
        indeg[e.next_color as usize] += 1;
    }
    for v in 0..n_v {
        if indeg[v] != outdeg[v] { return false; }
    }
    let raw: Vec<(usize, usize)> = all_edges
        .iter()
        .map(|e| (e.prev_color as usize, e.next_color as usize))
        .collect();
    connected_on_support(&raw, n_v)
}

/// The "free-pool" variant: each border tile is allowed any side. The
/// strictest necessary condition this can test is: in the *worst-case*
/// side-assignment, is the multigraph still feasible? Since each tile
/// has 4 possible (prev,next) edges (one per rotation), the conservative
/// pruner builds the **OR-graph**: include all 4 candidate edges per
/// tile. If even this generous graph fails degree-balance + connectivity,
/// no assignment can succeed.
///
/// Note this is much weaker than the side-pinned check above, but
/// has the advantage of being usable BEFORE any side-assignment is made
/// — i.e. as a quick pre-filter at the very root of border search.
#[must_use]
pub fn eulerian_pool_or_check(puzzle: &Puzzle, unused_border_pids: &[PieceId]) -> bool {
    let pieces: Vec<&Piece> = unused_border_pids
        .iter()
        .filter_map(|&pid| puzzle.piece(pid))
        .collect();
    let mut all_edges: Vec<RingEdge> = Vec::new();
    for p in &pieces {
        // For a corner: 4 corner-kinds yield up to 4 distinct (prev,next).
        // For an edge: 4 sides yield 4 (prev,next).
        // (For both, "matches" returns at most 1 placement per side/kind.)
        let candidates = if p.is_corner() {
            [
                corner_tile_ring_edge(p, CornerKind::TL),
                corner_tile_ring_edge(p, CornerKind::TR),
                corner_tile_ring_edge(p, CornerKind::BR),
                corner_tile_ring_edge(p, CornerKind::BL),
            ]
        } else if p.is_edge() {
            [
                edge_tile_ring_edge(p, RingSide::Top),
                edge_tile_ring_edge(p, RingSide::Right),
                edge_tile_ring_edge(p, RingSide::Bottom),
                edge_tile_ring_edge(p, RingSide::Left),
            ]
        } else {
            continue;
        };
        for c in candidates.into_iter().flatten() {
            all_edges.push(c);
        }
    }
    // OR-graph: degree balance over candidate edges is not a valid
    // necessary condition (each tile contributes 4 candidates but only
    // 1 will be chosen). So we only test connectivity here.
    let max_color = all_edges
        .iter()
        .flat_map(|e| [e.prev_color, e.next_color])
        .max()
        .unwrap_or(0) as usize;
    let n_v = max_color + 1;
    let raw: Vec<(usize, usize)> = all_edges
        .iter()
        .map(|e| (e.prev_color as usize, e.next_color as usize))
        .collect();
    connected_on_support(&raw, n_v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_core::{Edges, Piece};

    fn p(id: u16, t: Color, r: Color, b: Color, l: Color) -> Piece {
        Piece::new(id, Edges::new(t, r, b, l))
    }

    #[test]
    fn corner_tl_extracts_correct_colors() {
        // Piece with edges (top=BORDER, right=3, bottom=5, left=BORDER) at r=0
        // is a TL corner. Walk clockwise: incoming from below is bottom=5;
        // outgoing to the right is right=3.
        let piece = p(0, BORDER, 3, 5, BORDER);
        let re = corner_tile_ring_edge(&piece, CornerKind::TL).unwrap();
        assert_eq!(re.rotation, 0);
        assert_eq!(re.prev_color, 5);
        assert_eq!(re.next_color, 3);
    }

    #[test]
    fn corner_tr_clockwise_walk_consistency() {
        // TR corner: walk clockwise enters from previous tile on top (which
        // exposed `right_of_prev` toward this tile's left). So prev = left.
        // Outgoing down right side: next = bottom.
        let piece = p(1, BORDER, BORDER, 7, 9);
        let re = corner_tile_ring_edge(&piece, CornerKind::TR).unwrap();
        assert_eq!(re.prev_color, 9);
        assert_eq!(re.next_color, 7);
    }

    #[test]
    fn edge_tile_top_side_extracts_left_right() {
        // Top-side edge: rotation places BORDER at top.
        // Walking clockwise (left→right), prev = left, next = right.
        let piece = p(2, BORDER, 4, 6, 8);
        let re = edge_tile_ring_edge(&piece, RingSide::Top).unwrap();
        assert_eq!(re.rotation, 0);
        assert_eq!(re.prev_color, 8);
        assert_eq!(re.next_color, 4);
    }

    #[test]
    fn edge_tile_can_rotate_to_any_side() {
        // A piece has 4 distinct rotations placing BORDER on each face.
        // Verify all four sides yield a placement (possibly with rotated
        // prev/next).
        let piece = p(3, BORDER, 1, 2, 3);
        assert!(edge_tile_ring_edge(&piece, RingSide::Top).is_some());
        assert!(edge_tile_ring_edge(&piece, RingSide::Right).is_some());
        assert!(edge_tile_ring_edge(&piece, RingSide::Bottom).is_some());
        assert!(edge_tile_ring_edge(&piece, RingSide::Left).is_some());
    }

    #[test]
    fn full_ring_check_passes_on_tiny_consistent_ring() {
        // Build a 4-edge cycle: A→B→C→D→A.
        let ring = vec![
            RingEdge { piece_id: 0, rotation: 0, prev_color: 1, next_color: 2 },
            RingEdge { piece_id: 1, rotation: 0, prev_color: 2, next_color: 3 },
            RingEdge { piece_id: 2, rotation: 0, prev_color: 3, next_color: 4 },
            RingEdge { piece_id: 3, rotation: 0, prev_color: 4, next_color: 1 },
        ];
        assert!(eulerian_ring_check_full(&ring));
    }

    #[test]
    fn full_ring_check_rejects_chain_break() {
        // Same as above but with a deliberate prev/next mismatch.
        let ring = vec![
            RingEdge { piece_id: 0, rotation: 0, prev_color: 1, next_color: 2 },
            RingEdge { piece_id: 1, rotation: 0, prev_color: 3, next_color: 3 }, // BROKEN: prev should be 2
            RingEdge { piece_id: 2, rotation: 0, prev_color: 3, next_color: 4 },
            RingEdge { piece_id: 3, rotation: 0, prev_color: 4, next_color: 1 },
        ];
        assert!(!eulerian_ring_check_full(&ring));
    }

    #[test]
    fn full_ring_check_rejects_disconnected_two_cycles() {
        // Two disjoint 2-cycles: A→B→A and C→D→C. Balanced degree but
        // disconnected support.
        let ring = vec![
            RingEdge { piece_id: 0, rotation: 0, prev_color: 1, next_color: 2 },
            RingEdge { piece_id: 1, rotation: 0, prev_color: 2, next_color: 1 },
            RingEdge { piece_id: 2, rotation: 0, prev_color: 3, next_color: 4 },
            RingEdge { piece_id: 3, rotation: 0, prev_color: 4, next_color: 3 },
        ];
        // chain-consistency check: ring[2].prev should equal ring[1].next.
        // ring[1].next = 1; ring[2].prev = 3. So chain-consistency fails
        // first — this returns false. The connectivity reason is buried.
        assert!(!eulerian_ring_check_full(&ring));
    }

    #[test]
    fn pool_check_detects_disconnection_without_chain_order() {
        // Build a multigraph from edges of two pieces: piece 0 = 1→2,
        // piece 1 = 2→1 (one 2-cycle); piece 2 = 3→4, piece 3 = 4→3
        // (another 2-cycle). Degree balanced; disconnected → fail.
        let edges = vec![
            RingEdge { piece_id: 0, rotation: 0, prev_color: 1, next_color: 2 },
            RingEdge { piece_id: 1, rotation: 0, prev_color: 2, next_color: 1 },
            RingEdge { piece_id: 2, rotation: 0, prev_color: 3, next_color: 4 },
            RingEdge { piece_id: 3, rotation: 0, prev_color: 4, next_color: 3 },
        ];
        let max_c = 5;
        let n_v = max_c;
        let mut indeg = vec![0i32; n_v];
        let mut outdeg = vec![0i32; n_v];
        for e in &edges {
            outdeg[e.prev_color as usize] += 1;
            indeg[e.next_color as usize] += 1;
        }
        // Balanced
        for v in 0..n_v { assert_eq!(indeg[v], outdeg[v]); }
        let raw: Vec<(usize, usize)> = edges.iter().map(|e| (e.prev_color as usize, e.next_color as usize)).collect();
        assert!(!connected_on_support(&raw, n_v));
    }

    #[test]
    fn connectivity_on_single_self_loop() {
        let edges = vec![(1usize, 1usize)];
        // Single vertex with self-loop is trivially "connected" on its support.
        assert!(connected_on_support(&edges, 3));
    }
}
