//! Prefixes reached by the repo's break-tolerant DFS (`BreakSolver`).
//!
//! Path: row-major. Break schedule (preregistered): one licensed break cell per
//! row y = 1..n-1, at column (3y) mod n. Each run relabels the pieces with a
//! seeded shuffle (so the DFS visits a different part of the tree) and calls
//! `step(1)` up to `steps` times. Every time the DFS places a piece that
//! completes a full row with r rows left (r = 1..n-1), the prefix (in original
//! piece ids) enters the pool of stratum r. Pools are deduplicated; K prefixes
//! per stratum are then drawn uniformly without replacement with a seeded RNG.
//! Source "strict" runs the same solver with no break cell (perfect-so-far
//! prefixes only).

use eternity2_engine::generator::XorShift;
use eternity2_engine::{build_path, BreakSolver, Puzzle, Status};
use std::collections::HashSet;

pub fn break_cells(n: usize) -> Vec<u16> {
    (1..n).map(|y| (y * n + (3 * y) % n) as u16).collect()
}

fn shuffle<T>(v: &mut [T], rng: &mut XorShift) {
    for i in (1..v.len()).rev() {
        let j = (rng.next_u32() as usize) % (i + 1);
        v.swap(i, j);
    }
}

pub struct Pools {
    /// pools[r] = distinct prefixes (original codes) with r rows remaining.
    pub pools: Vec<Vec<Vec<i32>>>,
    pub visits: Vec<u64>,
}

pub fn collect(p: &Puzzle, run_seeds: &[u32], steps: u64, with_breaks: bool) -> Pools {
    let n = p.width as usize;
    let path = build_path("row-major", n as u8, n as u8, 0).expect("row-major path");
    let breaks = if with_breaks {
        break_cells(n)
    } else {
        Vec::new()
    };
    let mut pools: Vec<Vec<Vec<i32>>> = vec![Vec::new(); n];
    let mut seen: Vec<HashSet<Vec<i32>>> = vec![HashSet::new(); n];
    let mut visits = vec![0u64; n];
    for &rs in run_seeds {
        let mut perm: Vec<usize> = (0..p.pieces.len()).collect();
        let mut rng = XorShift::new(rs.wrapping_mul(2654435761) ^ 0x5eed);
        shuffle(&mut perm, &mut rng);
        let mut q = p.clone();
        q.pieces = perm.iter().map(|&i| p.pieces[i]).collect();
        q.hints.clear();
        let mut s = BreakSolver::new(&q, &path, false, &breaks).expect("solver");
        let mut last = s.placed();
        for _ in 0..steps {
            let rep = s.step(1);
            let placed = rep.placed;
            if placed > last && placed as usize % n == 0 {
                let r = n - placed as usize / n;
                if (1..n).contains(&r) {
                    visits[r] += 1;
                    let codes: Vec<i32> = s.board()[..placed as usize]
                        .iter()
                        .map(|&c| (perm[(c / 4) as usize] * 4) as i32 + c % 4)
                        .collect();
                    if seen[r].insert(codes.clone()) {
                        pools[r].push(codes);
                    }
                }
            }
            last = placed;
            if rep.status != Status::Running {
                break;
            }
        }
    }
    Pools { pools, visits }
}

/// Draw up to k items uniformly without replacement (partial Fisher-Yates).
pub fn draw(pool: &[Vec<i32>], k: usize, seed: u32) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..pool.len()).collect();
    let mut rng = XorShift::new(seed);
    let k = k.min(idx.len());
    for i in 0..k {
        let j = i + (rng.next_u32() as usize) % (idx.len() - i);
        idx.swap(i, j);
    }
    idx.truncate(k);
    idx
}
