//! Micro-benchmark of the two kernels that dominate one function-and-gradient evaluation of the
//! factored (low-rank) solver. TOY sizes and a synthetic one; nothing here solves a board.
//!
//! V is P x r, row-major. Kernel A ("pairs"): for K index pairs (i, j), h = <v_i, v_j>,
//! w = lam_k + rho h, f += lam_k h + rho h^2 / 2, g_i += w v_j, g_j += w v_i (the zero
//! constraints). Kernel B ("allpairs"): for every ordered pair, w = max(0, c - rho <v_i, v_j>),
//! f += w^2, g_i -= sum_j w v_j (the entrywise nonnegativity, with a scalar shift c in place of
//! the multiplier matrix so that P = 20,000 fits in memory).
//!
//! Inputs are closed-form so that kernel_numpy.py builds the same arrays and the checksums can be
//! compared across languages.
//!
//! Usage: kernel-bench P K r reps    (threads from RAYON_NUM_THREADS)
use rayon::prelude::*;
use std::time::Instant;

trait Real:
    Copy + Send + Sync + PartialOrd + std::ops::Add<Output = Self> + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self> + std::ops::AddAssign + 'static
{
    const ZERO: Self;
    const NAME: &'static str;
    fn from_f64(x: f64) -> Self;
    fn to_f64(self) -> f64;
    unsafe fn gemm(ta: i32, tb: i32, m: i32, n: i32, k: i32, a: *const Self, lda: i32, b: *const Self, ldb: i32, c: *mut Self, ldc: i32);
}

#[link(name = "Accelerate", kind = "framework")]
extern "C" {
    fn cblas_dgemm(order: i32, ta: i32, tb: i32, m: i32, n: i32, k: i32, alpha: f64, a: *const f64, lda: i32, b: *const f64, ldb: i32, beta: f64, c: *mut f64, ldc: i32);
    fn cblas_sgemm(order: i32, ta: i32, tb: i32, m: i32, n: i32, k: i32, alpha: f32, a: *const f32, lda: i32, b: *const f32, ldb: i32, beta: f32, c: *mut f32, ldc: i32);
}

impl Real for f64 {
    const ZERO: f64 = 0.0;
    const NAME: &'static str = "f64";
    fn from_f64(x: f64) -> f64 { x }
    fn to_f64(self) -> f64 { self }
    unsafe fn gemm(ta: i32, tb: i32, m: i32, n: i32, k: i32, a: *const f64, lda: i32, b: *const f64, ldb: i32, c: *mut f64, ldc: i32) {
        cblas_dgemm(101, ta, tb, m, n, k, 1.0, a, lda, b, ldb, 0.0, c, ldc)
    }
}
impl Real for f32 {
    const ZERO: f32 = 0.0;
    const NAME: &'static str = "f32";
    fn from_f64(x: f64) -> f32 { x as f32 }
    fn to_f64(self) -> f64 { self as f64 }
    unsafe fn gemm(ta: i32, tb: i32, m: i32, n: i32, k: i32, a: *const f32, lda: i32, b: *const f32, ldb: i32, c: *mut f32, ldc: i32) {
        cblas_sgemm(101, ta, tb, m, n, k, 1.0, a, lda, b, ldb, 0.0, c, ldc)
    }
}

/// Dot product with 8 independent accumulators (Rust does not reassociate floating sums, so a
/// plain loop would stay scalar).
#[inline(always)]
fn dot<T: Real>(a: &[T], b: &[T]) -> T {
    let mut acc = [T::ZERO; 8];
    let ca = a.chunks_exact(8);
    let cb = b.chunks_exact(8);
    let (ra, rb) = (ca.remainder(), cb.remainder());
    for (x, y) in ca.zip(cb) {
        for t in 0..8 {
            acc[t] += x[t] * y[t];
        }
    }
    let mut s = (acc[0] + acc[1]) + (acc[2] + acc[3]) + ((acc[4] + acc[5]) + (acc[6] + acc[7]));
    for (x, y) in ra.iter().zip(rb) {
        s += *x * *y;
    }
    s
}

#[inline(always)]
fn axpy<T: Real>(w: T, x: &[T], y: &mut [T]) {
    for (yy, xx) in y.iter_mut().zip(x) {
        *yy += w * *xx;
    }
}

/// Kernel A, each pair once, serial. Returns f.
fn pairs_half<T: Real>(v: &[T], r: usize, pi: &[u32], pj: &[u32], lam: &[T], rho: T, g: &mut [T]) -> f64 {
    let half = T::from_f64(0.5);
    let mut f = 0.0f64;
    let gp = g.as_mut_ptr();
    for k in 0..pi.len() {
        let (i, j) = (pi[k] as usize, pj[k] as usize);
        let (vi, vj) = (&v[i * r..i * r + r], &v[j * r..j * r + r]);
        let h = dot(vi, vj);
        let w = lam[k] + rho * h;
        f += (lam[k] * h + half * rho * h * h).to_f64();
        // i != j by construction, so the two rows of g do not overlap
        unsafe {
            axpy(w, vj, std::slice::from_raw_parts_mut(gp.add(i * r), r));
            axpy(w, vi, std::slice::from_raw_parts_mut(gp.add(j * r), r));
        }
    }
    f
}

/// Kernel A over the symmetric adjacency (each pair stored at both ends): every row of g is
/// written by one task only, so rows run in parallel without locks. Twice the dot products of
/// pairs_half.
fn pairs_rows<T: Real>(v: &[T], r: usize, rowptr: &[usize], col: &[u32], slot: &[u32], lam: &[T], rho: T, g: &mut [T], par: bool) -> f64 {
    let half = T::from_f64(0.5);
    let body = |(i, gi): (usize, &mut [T])| -> f64 {
        let vi = &v[i * r..i * r + r];
        let mut f = 0.0f64;
        for e in rowptr[i]..rowptr[i + 1] {
            let j = col[e] as usize;
            let l = lam[slot[e] as usize];
            let vj = &v[j * r..j * r + r];
            let h = dot(vi, vj);
            f += (l * h + half * rho * h * h).to_f64();
            axpy(l + rho * h, vj, gi);
        }
        f
    };
    let s: f64 = if par { g.par_chunks_mut(r).enumerate().map(body).sum() } else { g.chunks_mut(r).enumerate().map(body).sum() };
    0.5 * s
}

/// Kernel B, fused loops, no library: row i against every row j.
fn allpairs_fused<T: Real>(v: &[T], r: usize, c: T, rho: T, g: &mut [T], par: bool) -> f64 {
    let body = |(i, gi): (usize, &mut [T])| -> f64 {
        let vi = &v[i * r..i * r + r];
        let mut f = 0.0f64;
        for vj in v.chunks_exact(r) {
            let w = c - rho * dot(vi, vj);
            if w > T::ZERO {
                f += (w * w).to_f64();
                for (y, x) in gi.iter_mut().zip(vj) {
                    *y = *y - w * *x;
                }
            }
        }
        f
    };
    if par { g.par_chunks_mut(r).enumerate().map(body).sum() } else { g.chunks_mut(r).enumerate().map(body).sum() }
}

/// Kernel B with the two products done by the system BLAS (Accelerate) on blocks of `bs` rows,
/// the max and the sum of squares fused in one pass over the block.
fn allpairs_blas<T: Real>(v: &[T], p: usize, r: usize, c: T, rho: T, g: &mut [T], bs: usize, par: bool) -> f64 {
    let body = |(b, gb): (usize, &mut [T])| -> f64 {
        let rows = gb.len() / r;
        let mut blk = vec![T::ZERO; rows * p];
        let mut f = 0.0f64;
        unsafe {
            T::gemm(111, 112, rows as i32, p as i32, r as i32, v.as_ptr().add(b * bs * r), r as i32, v.as_ptr(), r as i32, blk.as_mut_ptr(), p as i32);
        }
        for row in blk.chunks_exact_mut(p) {
            let mut acc = [T::ZERO; 8];
            let mut it = row.chunks_exact_mut(8);
            for ch in &mut it {
                for t in 0..8 {
                    let w = rho * ch[t] - c;                 // -(c - rho G)
                    let w = if w < T::ZERO { w } else { T::ZERO }; // = -max(0, c - rho G)
                    ch[t] = w;
                    acc[t] += w * w;
                }
            }
            let mut s = T::ZERO;
            for t in 0..8 { s += acc[t]; }
            for x in it.into_remainder() {
                let w = rho * *x - c;
                let w = if w < T::ZERO { w } else { T::ZERO };
                *x = w;
                s += w * w;
            }
            f += s.to_f64();
        }
        unsafe {
            T::gemm(111, 111, rows as i32, r as i32, p as i32, blk.as_ptr(), p as i32, v.as_ptr(), r as i32, gb.as_mut_ptr(), r as i32);
        }
        f
    };
    if par { g.par_chunks_mut(bs * r).enumerate().map(body).sum() } else { g.chunks_mut(bs * r).enumerate().map(body).sum() }
}

/// Kernels A and B in one pass (the design a production solver would use while P^2 fits): the
/// Gram block is computed once by BLAS, the pair values are read out of it (K scalars, no row
/// gathers), the block is turned into the weights, the pair weights are added in place, and one
/// BLAS product gives the gradient rows. Returns f_A + f_B; g = g_A + g_B.
fn onepass_blas<T: Real>(v: &[T], p: usize, r: usize, rowptr: &[usize], col: &[u32], slot: &[u32], lam: &[T], c: T, rho: T, g: &mut [T], bs: usize, par: bool) -> f64 {
    let half = T::from_f64(0.5);
    let body = |(b, gb): (usize, &mut [T])| -> f64 {
        let rows = gb.len() / r;
        let i0 = b * bs;
        let mut blk = vec![T::ZERO; rows * p];
        let mut f = 0.0f64;
        unsafe {
            T::gemm(111, 112, rows as i32, p as i32, r as i32, v.as_ptr().add(i0 * r), r as i32, v.as_ptr(), r as i32, blk.as_mut_ptr(), p as i32);
        }
        let (e0, e1) = (rowptr[i0], rowptr[i0 + rows]);
        let mut hs = vec![T::ZERO; e1 - e0];
        for li in 0..rows {
            let row = &blk[li * p..(li + 1) * p];
            for e in rowptr[i0 + li]..rowptr[i0 + li + 1] {
                hs[e - e0] = row[col[e] as usize];
            }
        }
        let mut fb = 0.0f64;
        for x in blk.iter_mut() {
            let w = rho * *x - c;
            let w = if w < T::ZERO { w } else { T::ZERO };
            *x = w;
            fb += (w * w).to_f64();
        }
        f += fb;
        let mut fa = 0.0f64;
        for li in 0..rows {
            let row = &mut blk[li * p..(li + 1) * p];
            for e in rowptr[i0 + li]..rowptr[i0 + li + 1] {
                let h = hs[e - e0];
                let l = lam[slot[e] as usize];
                fa += (l * h + half * rho * h * h).to_f64();
                row[col[e] as usize] += l + rho * h;
            }
        }
        f += 0.5 * fa;
        unsafe {
            T::gemm(111, 111, rows as i32, r as i32, p as i32, blk.as_ptr(), p as i32, v.as_ptr(), r as i32, gb.as_mut_ptr(), r as i32);
        }
        f
    };
    if par { g.par_chunks_mut(bs * r).enumerate().map(body).sum() } else { g.chunks_mut(bs * r).enumerate().map(body).sum() }
}

fn best<F: FnMut() -> f64>(reps: usize, mut f: F) -> (f64, f64) {
    let mut t = f64::INFINITY;
    let mut val = 0.0;
    for _ in 0..reps {
        let t0 = Instant::now();
        val = f();
        t = t.min(t0.elapsed().as_secs_f64());
    }
    (t, val)
}

fn sumsq<T: Real>(g: &[T]) -> f64 {
    g.iter().map(|x| x.to_f64() * x.to_f64()).sum()
}

fn run<T: Real>(p: usize, k: usize, r: usize, reps: usize, threads: usize) {
    // closed-form inputs, mirrored in kernel_numpy.py
    let scale = 1.0 / (r as f64).sqrt();
    let v: Vec<T> = (0..p * r).map(|x| {
        let (q, t) = (x / r, x % r);
        T::from_f64((((q * 31 + t * 17 + q * t * 7) % 101) as f64 / 101.0 - 0.5) * scale)
    }).collect();
    let pi: Vec<u32> = (0..k as u64).map(|x| ((x * 2654435761) % p as u64) as u32).collect();
    let pj: Vec<u32> = (0..k as u64).map(|x| ((pi[x as usize] as u64 + 1 + (x * 40503) % (p as u64 - 1)) % p as u64) as u32).collect();
    let lam: Vec<T> = (0..k).map(|x| T::from_f64(((x % 13) as f64 - 6.0) * 0.01)).collect();
    let (rho, c) = (T::from_f64(10.0), T::from_f64(0.02));
    // symmetric adjacency by counting sort
    let mut rowptr = vec![0usize; p + 1];
    for x in 0..k { rowptr[pi[x] as usize + 1] += 1; rowptr[pj[x] as usize + 1] += 1; }
    for i in 0..p { rowptr[i + 1] += rowptr[i]; }
    let mut fill = rowptr.clone();
    let mut col = vec![0u32; 2 * k];
    let mut slot = vec![0u32; 2 * k];
    for x in 0..k {
        let (i, j) = (pi[x] as usize, pj[x] as usize);
        col[fill[i]] = j as u32; slot[fill[i]] = x as u32; fill[i] += 1;
        col[fill[j]] = i as u32; slot[fill[j]] = x as u32; fill[j] += 1;
    }
    let mut g = vec![T::ZERO; p * r];
    // Gram blocks of 1 MB, but never fewer than `min_rows` rows (KB_MIN_ROWS, default 1: the first
    // full run used 1, which leaves 6-row blocks at P = 20,000 and starves the BLAS).
    let min_rows: usize = std::env::var("KB_MIN_ROWS").ok().and_then(|x| x.parse().ok()).unwrap_or(1);
    let bs = ((1usize << 20) / (std::mem::size_of::<T>() * p)).max(min_rows);
    let only_blas = std::env::var("KB_ONLY_BLAS").is_ok();
    let out = |name: &str, th: usize, t: f64, f: f64, g: &[T]| {
        println!("{{\"lang\":\"rust\",\"kernel\":\"{}\",\"prec\":\"{}\",\"threads\":{},\"P\":{},\"K\":{},\"r\":{},\"seconds\":{:.6e},\"f\":{:.12e},\"g_sumsq\":{:.12e}}}", name, T::NAME, th, p, k, r, t, f, sumsq(g));
    };
    if only_blas {
        let (t, f) = best(reps, || allpairs_blas(&v, p, r, c, rho, &mut g, bs, false));
        out(&format!("allpairs_blas_min{}", min_rows), 1, t, f, &g);
        let (t, f) = best(reps, || allpairs_blas(&v, p, r, c, rho, &mut g, bs, true));
        out(&format!("allpairs_blas_min{}", min_rows), threads, t, f, &g);
        let (t, f) = best(reps, || onepass_blas(&v, p, r, &rowptr, &col, &slot, &lam, c, rho, &mut g, bs, false));
        out(&format!("onepass_blas_min{}", min_rows), 1, t, f, &g);
        let (t, f) = best(reps, || onepass_blas(&v, p, r, &rowptr, &col, &slot, &lam, c, rho, &mut g, bs, true));
        out(&format!("onepass_blas_min{}", min_rows), threads, t, f, &g);
        return;
    }
    let (t, f) = best(reps, || { g.iter_mut().for_each(|x| *x = T::ZERO); pairs_half(&v, r, &pi, &pj, &lam, rho, &mut g) });
    out("pairs_half", 1, t, f, &g);
    let (t, f) = best(reps, || { g.iter_mut().for_each(|x| *x = T::ZERO); pairs_rows(&v, r, &rowptr, &col, &slot, &lam, rho, &mut g, false) });
    out("pairs_rows", 1, t, f, &g);
    let (t, f) = best(reps, || { g.iter_mut().for_each(|x| *x = T::ZERO); pairs_rows(&v, r, &rowptr, &col, &slot, &lam, rho, &mut g, true) });
    out("pairs_rows", threads, t, f, &g);
    let (t, f) = best(reps, || { g.iter_mut().for_each(|x| *x = T::ZERO); allpairs_fused(&v, r, c, rho, &mut g, false) });
    out("allpairs_fused", 1, t, f, &g);
    let (t, f) = best(reps, || { g.iter_mut().for_each(|x| *x = T::ZERO); allpairs_fused(&v, r, c, rho, &mut g, true) });
    out("allpairs_fused", threads, t, f, &g);
    let (t, f) = best(reps, || allpairs_blas(&v, p, r, c, rho, &mut g, bs, false));
    out("allpairs_blas", 1, t, f, &g);
    let (t, f) = best(reps, || allpairs_blas(&v, p, r, c, rho, &mut g, bs, true));
    out("allpairs_blas", threads, t, f, &g);
}

fn main() {
    let a: Vec<usize> = std::env::args().skip(1).map(|s| s.parse().expect("P K r reps")).collect();
    let (p, k, r, reps) = (a[0], a[1], a[2], a[3]);
    let threads = rayon::current_num_threads();
    run::<f64>(p, k, r, reps, threads);
    run::<f32>(p, k, r, reps, threads);
}
