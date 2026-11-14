//! Soundness checks on small boards, against exhaustive search.
//!
//! For random generated boards (n = 4, 5) and random prefixes (structurally
//! valid, pieces distinct, edges matched or not at random), with 1 or 2 rows
//! remaining, checks:
//! 1. oracle optimum == brute-force optimum, and the witness scores it;
//! 2. dd_bound(W) >= brute-force optimum for W in WIDTHS;
//! 3. dd_bound with W large enough to never merge == brute-force optimum
//!    (the unrelaxed diagram is exact);
//! 4. dd_bound(W) <= trivial bound.

use crate::dd::{dd_bound, trivial_bound};
use crate::model::{Model, Opt, Prefix};
use crate::oracle::{best_completion, brute_best};
use eternity2_engine::generate_solved_framed_bc;
use eternity2_engine::generator::XorShift;

pub const WIDTHS: [usize; 7] = [1, 2, 3, 8, 64, 512, 4096];

/// A random structurally valid prefix of d cells, using distinct pieces. With
/// probability `p_follow` per cell the piece is chosen among those matching
/// the placed neighbours when one exists.
pub fn random_prefix(m: &Model, d: usize, rng: &mut XorShift, p_follow: u32) -> Option<Prefix> {
    let n = m.n;
    let mut cells: Vec<Opt> = Vec::new();
    let mut used = 0u64;
    for pos in 0..d {
        let cand: Vec<Opt> = m.opts[pos]
            .iter()
            .copied()
            .filter(|o| used >> o.p & 1 == 0)
            .collect();
        if cand.is_empty() {
            return None;
        }
        let (x, y) = (pos % n, pos / n);
        let fits: Vec<Opt> = cand
            .iter()
            .copied()
            .filter(|o| {
                (x == 0 || cells[pos - 1].e[1] == o.e[3])
                    && (y == 0 || cells[pos - n].e[2] == o.e[0])
            })
            .collect();
        let pool = if !fits.is_empty() && rng.next_u32() % 100 < p_follow {
            fits
        } else {
            cand
        };
        let o = pool[(rng.next_u32() as usize) % pool.len()];
        used |= 1 << o.p;
        cells.push(o);
    }
    Some(Prefix { cells })
}

pub fn witness_score(m: &Model, pre: &Prefix, w: &[(u8, [u8; 4])]) -> u32 {
    let mut cells: Vec<[u8; 4]> = pre.cells.iter().map(|o| o.e).collect();
    let mut used = pre.used();
    for &(p, e) in w {
        assert_eq!(used >> p & 1, 0, "witness reuses a piece");
        used |= 1 << p;
        cells.push(e);
    }
    let n = m.n;
    assert_eq!(cells.len(), n * n);
    let mut s = 0;
    for pos in 0..n * n {
        assert!(crate::model::structurally_valid(n, pos, cells[pos]));
        let (x, y) = (pos % n, pos / n);
        if x > 0 && cells[pos - 1][1] == cells[pos][3] {
            s += 1;
        }
        if y > 0 && cells[pos - n][2] == cells[pos][0] {
            s += 1;
        }
    }
    s
}

/// Returns the number of cases checked; panics on any failure.
pub fn run(cases_per_config: u32) -> u32 {
    let mut checked = 0;
    let mut loose_triv = 0;
    let mut loose_dd = [0u32; 7];
    let configs: [(u8, u8, usize); 6] = [
        (4, 6, 1),
        (4, 7, 2),
        (4, 9, 2),
        (5, 7, 1),
        (5, 9, 1),
        (5, 10, 2),
    ];
    for &(n, colours, r) in &configs {
        for seed in 1..=cases_per_config {
            let p = generate_solved_framed_bc(n, colours, seed, true, 3);
            let m = Model::new(&p);
            let nn = n as usize;
            let d = (nn - r) * nn;
            let mut rng = XorShift::new(seed * 7919 + n as u32 * 31 + r as u32);
            let follow = [0u32, 60, 95][(seed % 3) as usize];
            let Some(pre) = random_prefix(&m, d, &mut rng, follow) else {
                continue;
            };
            let brute = brute_best(&m, &pre);
            let triv = trivial_bound(&m, &pre);
            let or = best_completion(&m, &pre, u64::MAX);
            let k = or.kstar.expect("uncapped oracle finishes");
            let oracle_best = pre.score(&m) + m.remaining_edges(d) - k;
            assert_eq!(
                oracle_best, brute,
                "oracle != brute (n={n} c={colours} r={r} seed={seed})"
            );
            assert_eq!(witness_score(&m, &pre, &or.witness), brute, "witness score");
            if triv > brute {
                loose_triv += 1;
            }
            for &w in &WIDTHS {
                let b = dd_bound(&m, &pre, w);
                if b.bound > brute {
                    loose_dd[WIDTHS.iter().position(|&x| x == w).unwrap()] += 1;
                }
                assert!(
                    b.bound >= brute,
                    "DD W={w} bound {} < truth {brute} (n={n} c={colours} r={r} seed={seed})",
                    b.bound
                );
                assert!(b.bound <= triv, "DD above trivial");
            }
            let exact = dd_bound(&m, &pre, usize::MAX / 2);
            assert_eq!(exact.absorbed_frontier + exact.absorbed_pieces_only, 0);
            assert_eq!(
                exact.bound, brute,
                "unmerged DD not exact (n={n} c={colours} r={r} seed={seed})"
            );
            checked += 1;
        }
    }
    eprintln!("cases with gap > 0: trivial {loose_triv}, DD per W {WIDTHS:?}: {loose_dd:?}");
    checked
}

#[cfg(test)]
mod tests {
    #[test]
    fn small_boards_against_brute_force() {
        let k = super::run(12);
        assert!(k >= 40, "only {k} cases");
    }
}
