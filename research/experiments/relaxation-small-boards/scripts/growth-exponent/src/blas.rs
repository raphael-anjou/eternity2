//! Accelerate's dgemm, behind one safe wrapper. Unsafe is used here only, after measurement:
//! the blocked safe-Rust pair kernel ran at 5.6 ms per evaluation at P = 478, r = 32 (results/
//! kernels.txt), the BLAS form at the figure in the same file.

#[cfg(target_os = "macos")]
#[link(name = "Accelerate", kind = "framework")]
extern "C" {
    fn cblas_dgemm(
        order: i32, ta: i32, tb: i32, m: i32, n: i32, k: i32, alpha: f64, a: *const f64, lda: i32,
        b: *const f64, ldb: i32, beta: f64, c: *mut f64, ldc: i32,
    );
}

#[cfg(target_os = "macos")]
const ROW_MAJOR: i32 = 101;
#[cfg(target_os = "macos")]
const NO_TRANS: i32 = 111;
#[cfg(target_os = "macos")]
const TRANS: i32 = 112;

/// Pure-Rust fallback off macOS (the Linux sweep machine has no BLAS installed): plain loops,
/// same contract. Only the once-per-outer Gram uses it there; the evaluations use kernel=blk.
#[cfg(not(target_os = "macos"))]
#[allow(clippy::too_many_arguments)]
pub fn dgemm(ta: bool, tb: bool, m: usize, n: usize, k: usize, alpha: f64, a: &[f64], b: &[f64], beta: f64, c: &mut [f64]) {
    let ga = |i: usize, l: usize| if ta { a[l * m + i] } else { a[i * k + l] };
    let gb = |l: usize, j: usize| if tb { b[j * k + l] } else { b[l * n + j] };
    for i in 0..m {
        for j in 0..n {
            let mut s = 0.0;
            for l in 0..k {
                s += ga(i, l) * gb(l, j);
            }
            c[i * n + j] = alpha * s + if beta == 0.0 { 0.0 } else { beta * c[i * n + j] };
        }
    }
}

/// C (m x n) = alpha op(A) op(B) + beta C, row major, with the dimensions checked against the
/// slice lengths before the call.
#[cfg(target_os = "macos")]
#[allow(clippy::too_many_arguments)]
pub fn dgemm(ta: bool, tb: bool, m: usize, n: usize, k: usize, alpha: f64, a: &[f64], b: &[f64], beta: f64, c: &mut [f64]) {
    let (ar, ac) = if ta { (k, m) } else { (m, k) };
    let (br, bc) = if tb { (n, k) } else { (k, n) };
    assert!(a.len() >= ar * ac && b.len() >= br * bc && c.len() >= m * n);
    // SAFETY: the asserts above bound every access the routine makes (row major, leading
    // dimensions equal to the column counts); the slices are valid for those lengths.
    unsafe {
        cblas_dgemm(
            ROW_MAJOR, if ta { TRANS } else { NO_TRANS }, if tb { TRANS } else { NO_TRANS }, m as i32, n as i32, k as i32,
            alpha, a.as_ptr(), ac as i32, b.as_ptr(), bc as i32, beta, c.as_mut_ptr(), n as i32,
        )
    }
}
