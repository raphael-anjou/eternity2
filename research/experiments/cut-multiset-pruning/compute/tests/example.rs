//! The hand-built example of PREREG.md: a 5x5 state (ring placed, interior
//! empty) that passes class balance, checkerboard parity, gacolor, island,
//! NS-1 multiset equality, the Eulerian ring check and pairwise arc
//! consistency with distinct pieces, has no singleton domain, and is dead:
//! the four interior corner cells need two adjacent `a` faces and only three
//! unused pieces have them. The Hall check rejects it.

use cmp_solver_engine::hall_match;
use eternity2_core::{Edges, Piece, Puzzle};
use eternity2_propagators::border_eulerian::{corner_tile_ring_edge, edge_tile_ring_edge, eulerian_ring_check_full, CornerKind, RingSide};
use eternity2_propagators::{
    class_balance_check, gacolor_check, island_check, multiset_equality_check, parity_check, PlacementInfo,
    PropagatorContext, PropagatorResult,
};

const F: u8 = 1;
const A: u8 = 2;
const B: u8 = 3;
const C: u8 = 4;
const N: usize = 5;

fn ring() -> Vec<(usize, [u8; 4])> {
    let at = |x: usize, y: usize| y * N + x;
    vec![
        (at(0, 0), [0, F, F, 0]),
        (at(1, 0), [0, F, A, F]),
        (at(2, 0), [0, F, B, F]),
        (at(3, 0), [0, F, A, F]),
        (at(4, 0), [0, 0, F, F]),
        (at(4, 1), [F, 0, F, A]),
        (at(4, 2), [F, 0, F, B]),
        (at(4, 3), [F, 0, F, A]),
        (at(4, 4), [F, 0, 0, F]),
        (at(3, 4), [A, F, 0, F]),
        (at(2, 4), [B, F, 0, F]),
        (at(1, 4), [A, F, 0, F]),
        (at(0, 4), [F, F, 0, 0]),
        (at(0, 3), [F, A, F, 0]),
        (at(0, 2), [F, B, F, 0]),
        (at(0, 1), [F, A, F, 0]),
    ]
}

fn interior() -> Vec<[u8; 4]> {
    vec![[A, A, C, C], [A, A, C, C], [A, A, C, C], [B, C, A, C], [B, C, A, C], [B, C, C, C], [B, C, C, C], [A, C, A, C], [C, C, C, C]]
}

fn rot(e: [u8; 4], r: u8) -> [u8; 4] {
    let mut out = e;
    for _ in 0..r {
        out = [out[3], out[0], out[1], out[2]];
    }
    out
}

#[test]
fn hall_prunes_where_existing_propagators_do_not() {
    let ring = ring();
    let mut pieces = Vec::new();
    for (i, (_, e)) in ring.iter().enumerate() {
        pieces.push(Piece::new(i as u16, Edges::new(e[0], e[1], e[2], e[3])));
    }
    for (j, e) in interior().iter().enumerate() {
        pieces.push(Piece::new((ring.len() + j) as u16, Edges::new(e[0], e[1], e[2], e[3])));
    }
    let puzzle = Puzzle::new(N as u32, N as u32, 5, pieces.clone()).unwrap();
    let mut placed: Vec<Option<[u8; 4]>> = vec![None; N * N];
    let mut used = vec![false; N * N];
    for (i, (pos, e)) in ring.iter().enumerate() {
        placed[*pos] = Some(*e);
        used[i] = true;
    }
    // Domains: unused pieces, distinct rotations, rim and placed neighbours.
    let nb = |pos: usize, s: usize| -> Option<usize> {
        let (x, y) = (pos % N, pos / N);
        match s {
            0 if y > 0 => Some(pos - N),
            1 if x + 1 < N => Some(pos + 1),
            2 if y + 1 < N => Some(pos + N),
            3 if x > 0 => Some(pos - 1),
            _ => None,
        }
    };
    let mut dom: Vec<Vec<(usize, u8)>> = vec![Vec::new(); N * N];
    for pos in 0..N * N {
        if placed[pos].is_some() {
            continue;
        }
        for (p, piece) in pieces.iter().enumerate() {
            if used[p] {
                continue;
            }
            let base = piece.edges.as_array();
            for r in 0..4u8 {
                let e = rot(base, r);
                if (0..r).any(|q| rot(base, q) == e) {
                    continue;
                }
                let ok = (0..4).all(|s| match nb(pos, s) {
                    None => e[s] == 0,
                    Some(q) => e[s] != 0 && placed[q].is_none_or(|f| f[(s + 2) % 4] == e[s]),
                });
                if ok {
                    dom[pos].push((p, r));
                }
            }
        }
    }
    // Pairwise arc consistency with distinct pieces, to a fixpoint.
    loop {
        let mut changed = false;
        for pos in 0..N * N {
            let keep: Vec<(usize, u8)> = dom[pos]
                .iter()
                .copied()
                .filter(|&(p, r)| {
                    let e = rot(pieces[p].edges.as_array(), r);
                    (0..4).all(|s| match nb(pos, s) {
                        Some(q) if placed[q].is_none() => dom[q]
                            .iter()
                            .any(|&(p2, r2)| p2 != p && rot(pieces[p2].edges.as_array(), r2)[(s + 2) % 4] == e[s]),
                        _ => true,
                    })
                })
                .collect();
            if keep.len() != dom[pos].len() {
                dom[pos] = keep;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let empty: Vec<usize> = (0..N * N).filter(|&p| placed[p].is_none()).collect();
    assert!(empty.iter().all(|&p| dom[p].len() >= 2), "no empty or singleton domain after AC");

    let words = (N * N * 4).div_ceil(64);
    let mut bits = vec![0u64; N * N * words];
    for &pos in &empty {
        for &(p, r) in &dom[pos] {
            let id = p * 4 + r as usize;
            bits[pos * words + id / 64] |= 1 << (id % 64);
        }
    }
    let infos: Vec<Option<PlacementInfo>> =
        placed.iter().map(|e| e.map(|e| PlacementInfo { edges_after_rotation: e })).collect();
    let ctx = PropagatorContext { puzzle: &puzzle, placed: &infos, used_pieces: &used, domain_bits: &bits, words_per_pos: words };
    for (name, res) in [
        ("class_balance", class_balance_check(&ctx)),
        ("parity", parity_check(&ctx)),
        ("gacolor", gacolor_check(&ctx)),
        ("island", island_check(&ctx)),
        ("multiset_equality (NS-1)", multiset_equality_check(&ctx)),
    ] {
        assert_eq!(res, PropagatorResult::Ok, "{name} should not prune the example");
    }
    // Eulerian ring check on the (closed) placed ring, clockwise from TL.
    let order = [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (4, 1), (4, 2), (4, 3), (4, 4), (3, 4), (2, 4), (1, 4), (0, 4), (0, 3), (0, 2), (0, 1)];
    let mut walk = Vec::new();
    for (x, y) in order {
        let pos = y * N + x;
        let i = ring.iter().position(|(q, _)| *q == pos).unwrap();
        let piece = pieces[i];
        let edge = match (x, y) {
            (0, 0) => corner_tile_ring_edge(&piece, CornerKind::TL),
            (4, 0) => corner_tile_ring_edge(&piece, CornerKind::TR),
            (4, 4) => corner_tile_ring_edge(&piece, CornerKind::BR),
            (0, 4) => corner_tile_ring_edge(&piece, CornerKind::BL),
            (_, 0) => edge_tile_ring_edge(&piece, RingSide::Top),
            (4, _) => edge_tile_ring_edge(&piece, RingSide::Right),
            (_, 4) => edge_tile_ring_edge(&piece, RingSide::Bottom),
            _ => edge_tile_ring_edge(&piece, RingSide::Left),
        };
        walk.push(edge.unwrap());
    }
    assert!(eulerian_ring_check_full(&walk), "Eulerian ring check should not prune the example");

    // Hall: frontier (here every empty cell but the centre) and full scope.
    let mask = |pos: usize| dom[pos].iter().fold(0u128, |m, &(p, _)| m | (1u128 << p));
    let frontier: Vec<usize> = empty.iter().copied().filter(|&p| (0..4).any(|s| nb(p, s).is_some_and(|q| placed[q].is_some()))).collect();
    assert_eq!(frontier.len(), 8);
    let mut fm: Vec<u128> = frontier.iter().map(|&p| mask(p)).collect();
    assert!(!hall_match(&mut fm), "Hall must reject the frontier");
    let mut am: Vec<u128> = empty.iter().map(|&p| mask(p)).collect();
    assert!(!hall_match(&mut am), "Hall must reject the full region");
}

#[test]
fn hall_match_basics() {
    assert!(hall_match(&mut [0b011, 0b110, 0b100]));
    assert!(!hall_match(&mut [0b011, 0b011, 0b011]));
    assert!(hall_match(&mut []));
}
