// Vol-125 T37 — property test: lsap (JV) and pathfinding kuhn_munkres
// should produce the same OPTIMAL score (up to ties) on small LAPs.

use pathfinding::kuhn_munkres::kuhn_munkres;
use pathfinding::matrix::Matrix;

fn lcg_next(state: u64) -> u64 {
    state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407)
}

fn gen_matrix(k: usize, seed: u64) -> Vec<i64> {
    let mut state = seed.wrapping_mul(0x9E3779B97F4A7C15);
    let mut v = Vec::with_capacity(k * k);
    for _ in 0..k * k {
        state = lcg_next(state);
        v.push(((state >> 32) as i32 & 0xFF) as i64); // 0..255
    }
    v
}

fn km_score(k: usize, m: &[i64]) -> i64 {
    let cost_matrix = Matrix::from_vec(k, k, m.to_vec()).unwrap();
    let (s, _) = kuhn_munkres(&cost_matrix);
    s
}

fn jv_score(k: usize, m: &[i64]) -> i64 {
    let costs: Vec<f64> = m.iter().map(|&x| x as f64).collect();
    let (_row_ind, col_ind) = lsap::solve(k, k, &costs, true).expect("LAP solvable");
    (0..k).map(|i| m[i * k + col_ind[i] as usize]).sum()
}

#[test]
fn jv_matches_km_small_k() {
    for k in [3, 4, 5, 8, 10] {
        for seed in 0..20 {
            let m = gen_matrix(k, seed);
            let s_km = km_score(k, &m);
            let s_jv = jv_score(k, &m);
            assert_eq!(s_km, s_jv,
                "JV ≠ KM at k={}, seed={}: km={}, jv={}",
                k, seed, s_km, s_jv);
        }
    }
}

#[test]
fn jv_matches_km_medium_k() {
    for k in [16, 32, 50] {
        for seed in 0..10 {
            let m = gen_matrix(k, seed);
            let s_km = km_score(k, &m);
            let s_jv = jv_score(k, &m);
            assert_eq!(s_km, s_jv,
                "JV ≠ KM at k={}, seed={}: km={}, jv={}",
                k, seed, s_km, s_jv);
        }
    }
}

#[test]
fn jv_matches_km_with_negatives() {
    // KM requires non-negative weights for some implementations. Test that
    // lsap also handles negative costs (since matchres can be subtracted).
    let k = 5;
    let m: Vec<i64> = vec![
        -1, 2, 0, 1, 4,
        3, -2, 1, 0, 2,
        0, 1, 5, -3, 2,
        2, 3, -1, 4, 0,
        1, 0, 2, 3, -2,
    ];
    let s_km = km_score(k, &m);
    let s_jv = jv_score(k, &m);
    assert_eq!(s_km, s_jv, "JV ≠ KM with negatives: km={}, jv={}", s_km, s_jv);
}
