//! Regression tests for `derive_stream_seed`.
//!
//! Sub-stream seeds used to be derived as `(seed + index) * K`, which aliases:
//! `seed + index` is one number, so (seed=7, index=1) and (seed=8, index=0)
//! produce the same stream. Consecutive seeds re-ran each other's chains and an
//! "N seeds x M chains" sweep covered far fewer than N*M distinct streams. The
//! same formula had been copied into blackwood_bt and verhaard's restart
//! portfolios.
//!
//! These tests pin the property that broke, not the implementation: distinct
//! (seed, index) pairs must give distinct streams.

use eternity2_localsearch::derive_stream_seed;
use std::collections::HashSet;

/// The exact historical collision: seed 7 / chain 1 vs seed 8 / chain 0.
#[test]
fn adjacent_seed_and_index_do_not_collide() {
    assert_ne!(
        derive_stream_seed(7, 1),
        derive_stream_seed(8, 0),
        "(seed+index) aliasing is back: seed 7 chain 1 must not equal seed 8 chain 0"
    );
}

/// The general form: over a seed x index grid, every pair must be distinct.
/// The old formula produced only `n_seeds + n_index - 1` distinct values here
/// (the distinct sums) instead of `n_seeds * n_index`.
#[test]
fn seed_index_grid_is_collision_free() {
    let (n_seeds, n_index) = (64u64, 64usize);
    let mut seen = HashSet::new();
    for s in 1..=n_seeds {
        for i in 0..n_index {
            assert!(
                seen.insert(derive_stream_seed(s, i)),
                "collision at seed={s} index={i}: {} distinct so far",
                seen.len()
            );
        }
    }
    assert_eq!(seen.len(), (n_seeds as usize) * n_index);
}

/// Deterministic: the same (seed, index) always gives the same stream. This is
/// what lets a parallel portfolio be reproducible regardless of thread count.
#[test]
fn is_deterministic() {
    for (s, i) in [(1u64, 0usize), (7, 3), (99, 41)] {
        assert_eq!(derive_stream_seed(s, i), derive_stream_seed(s, i));
    }
}

/// Nearby inputs must not produce nearby outputs -- consecutive indices are the
/// common case (chain 0, 1, 2, ...), and an RNG seeded from correlated values
/// can emit correlated streams. Check the derived seeds differ in a healthy
/// number of bits rather than trending together.
#[test]
fn consecutive_indices_decorrelate() {
    for i in 0..32usize {
        let a = derive_stream_seed(12345, i);
        let b = derive_stream_seed(12345, i + 1);
        let hamming = (a ^ b).count_ones();
        assert!(
            (12..=52).contains(&hamming),
            "seeds for index {i} and {} differ in only {hamming} bits (expect ~32)",
            i + 1
        );
    }
}
