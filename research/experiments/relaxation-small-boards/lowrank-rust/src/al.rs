//! TOY BOARDS ONLY. The augmented Lagrangian of bm.py (SphereAL, vec=True) on V (P x r, row
//! major), its gradient, the sphere map, max_violation and the multiplier update.
//!
//! Multipliers: lc (N x r), lt (T x r) for the vector cell and tile sums, and one dense symmetric
//! P x P matrix `pl` holding lz on conflict pairs (E6) and mu >= 0 on free pairs (I2), 0 on the
//! diagonal. bm.py keeps lz as a list and mu as a sparse matrix; the values are the same.

use crate::board::{Board, CONFLICT, FREE};

#[derive(Clone)]
pub struct Mult {
    pub rho: f64,
    /// weight of the objective ||x||^2 (1 in bm.py)
    pub obj_w: f64,
    pub r: usize,
    pub lc: Vec<f64>,
    pub lt: Vec<f64>,
    pub pl: Vec<f64>,
}

impl Mult {
    pub fn new(b: &Board, r: usize, rho: f64) -> Mult {
        Mult { rho, obj_w: 1.0, r, lc: vec![0.0; b.nc * r], lt: vec![0.0; b.nt * r], pl: vec![0.0; b.p * b.p] }
    }
}

#[inline(always)]
pub fn dot(a: &[f64], b: &[f64]) -> f64 {
    // four independent accumulators so the loop vectorises; r is small (2 to 64)
    let mut s = [0.0f64; 4];
    let n4 = a.len() / 4 * 4;
    let (a4, ar) = a.split_at(n4);
    let (b4, br) = b.split_at(n4);
    for (x, y) in a4.chunks_exact(4).zip(b4.chunks_exact(4)) {
        s[0] += x[0] * y[0];
        s[1] += x[1] * y[1];
        s[2] += x[2] * y[2];
        s[3] += x[3] * y[3];
    }
    let mut t = (s[0] + s[2]) + (s[1] + s[3]);
    for (x, y) in ar.iter().zip(br) {
        t += x * y;
    }
    t
}

#[inline(always)]
pub fn axpy(y: &mut [f64], a: f64, x: &[f64]) {
    for (u, v) in y.iter_mut().zip(x) {
        *u += a * v;
    }
}

/// The linear part (objective ||x||^2 and the vector cell and tile sums), shared by every kernel.
fn linear_part(b: &Board, m: &Mult, v: &[f64], g: &mut [f64]) -> f64 {
    let r = m.r;
    let rho = m.rho;
    let mut f = 0.0;
    for p in 0..b.p {
        let x = v[p * r];
        f += m.obj_w * x * x;
        g[p * r] += 2.0 * m.obj_w * x;
    }
    let mut h = vec![0.0; r];
    for (groups, lam) in [(&b.cells, &m.lc), (&b.tiles, &m.lt)] {
        for (c, grp) in groups.iter().enumerate() {
            h.iter_mut().for_each(|a| *a = 0.0);
            for &p in grp {
                axpy(&mut h, 1.0, &v[p * r..p * r + r]);
            }
            h[0] -= 1.0;
            let l = &lam[c * r..c * r + r];
            for k in 0..r {
                f += l[k] * h[k] + 0.5 * rho * h[k] * h[k];
                h[k] = l[k] + rho * h[k];
            }
            for &p in grp {
                axpy(&mut g[p * r..p * r + r], 1.0, &h);
            }
        }
    }
    f
}

/// Literal port of SphereAL.fV: E6 over the conflict list with the I2 correction, then I2 over
/// every ordered pair (diagonal included), then the constant -sum(mu^2)/(2 rho).
pub fn fv_ref(b: &Board, m: &Mult, v: &[f64], g: &mut [f64]) -> f64 {
    let (p, r, rho) = (b.p, m.r, m.rho);
    g.iter_mut().for_each(|a| *a = 0.0);
    let mut f = linear_part(b, m, v, g);
    for &(i, j) in &b.conf {
        let hz = dot(&v[i * r..i * r + r], &v[j * r..j * r + r]);
        let lz = m.pl[i * p + j];
        let wz = (-rho * hz).max(0.0);
        f += lz * hz + 0.5 * rho * hz * hz - wz * wz / (2.0 * rho);
        let c = lz + rho * hz + wz;
        for k in 0..r {
            g[i * r + k] += c * v[j * r + k];
            g[j * r + k] += c * v[i * r + k];
        }
    }
    let mut mu2 = 0.0;
    for i in 0..p {
        for j in 0..p {
            let mu = if b.kind[i * p + j] == FREE { m.pl[i * p + j] } else { 0.0 };
            mu2 += mu * mu;
        }
    }
    f -= 0.5 * mu2 / (2.0 * rho);
    let mut fw = 0.0;
    for i in 0..p {
        for j in 0..p {
            let mu = if b.kind[i * p + j] == FREE { m.pl[i * p + j] } else { 0.0 };
            let gij = dot(&v[i * r..i * r + r], &v[j * r..j * r + r]);
            let w = (mu - rho * gij).max(0.0);
            fw += w * w;
            for k in 0..r {
                g[i * r + k] -= w * v[j * r + k];
            }
        }
    }
    f + 0.5 * fw / (2.0 * rho)
}

/// One pass over row i: the pair terms of every j != i, folded. Returns twice row i's share of f
/// (each unordered pair is seen from both ends) and adds row i's gradient into gi.
#[inline(always)]
fn pair_row(b: &Board, m: &Mult, v: &[f64], i: usize, gi: &mut [f64]) -> f64 {
    let (p, r, rho) = (b.p, m.r, m.rho);
    let inv = 1.0 / (2.0 * rho);
    let vi = &v[i * r..i * r + r];
    let kind = &b.kind[i * p..i * p + p];
    let pl = &m.pl[i * p..i * p + p];
    let mut f = 0.0;
    for j in 0..p {
        let k = kind[j];
        if k == 0 {
            continue;
        }
        let vj = &v[j * r..j * r + r];
        let d = dot(vi, vj);
        let l = pl[j];
        let c = if k == CONFLICT {
            f += l * d + 0.5 * rho * d * d;
            l + rho * d
        } else {
            let w = (l - rho * d).max(0.0);
            f += (w * w - l * l) * inv;
            -w
        };
        if c != 0.0 {
            axpy(gi, c, vj);
        }
    }
    f
}

/// Same function, one pass over all pairs (row by row, rows independent, so deterministic in any
/// order and safe to split across threads).
pub fn fv_rows(b: &Board, m: &Mult, v: &[f64], g: &mut [f64]) -> f64 {
    let r = m.r;
    g.iter_mut().for_each(|a| *a = 0.0);
    let mut f = linear_part(b, m, v, g);
    let mut fp = 0.0;
    for i in 0..b.p {
        fp += pair_row(b, m, v, i, &mut g[i * r..i * r + r]);
    }
    f += 0.5 * fp;
    f
}

/// Rows split across the current rayon pool in fixed chunks; per-chunk sums are added in chunk
/// order, so the result does not depend on scheduling.
pub fn fv_rows_par(b: &Board, m: &Mult, v: &[f64], g: &mut [f64], chunk: usize) -> f64 {
    use rayon::prelude::*;
    let r = m.r;
    g.iter_mut().for_each(|a| *a = 0.0);
    let f = linear_part(b, m, v, g);
    let parts: Vec<f64> = g
        .par_chunks_mut(chunk * r)
        .enumerate()
        .map(|(c, gc)| {
            let mut s = 0.0;
            for (k, gi) in gc.chunks_mut(r).enumerate() {
                s += pair_row(b, m, v, c * chunk + k, gi);
            }
            s
        })
        .collect();
    f + 0.5 * parts.iter().sum::<f64>()
}

/// Row block of the blocked kernel: four rows of V share every load of V^T.
const RB: usize = 4;

/// Pair terms for rows i0..i0+nb (nb <= RB), branch free, on the transposed factor vt (r x P).
/// Phase 1: D = V_blk V^T as r axpys over P-long rows of vt; phase 2: the coefficient of every
/// pair from the conflict and free masks; phase 3: g_blk = C V as r dot products over P.
#[inline(always)]
fn pair_block(b: &Board, m: &Mult, v: &[f64], vt: &[f64], i0: usize, nb: usize, gb: &mut [f64], d: &mut [f64]) -> f64 {
    let (p, r, rho) = (b.p, m.r, m.rho);
    let inv = 1.0 / (2.0 * rho);
    let hr = 0.5 * rho;
    d[..RB * p].iter_mut().for_each(|x| *x = 0.0);
    {
        let (d0, rest) = d[..RB * p].split_at_mut(p);
        let (d1, rest) = rest.split_at_mut(p);
        let (d2, d3) = rest.split_at_mut(p);
        for k in 0..r {
            let s: [f64; RB] = std::array::from_fn(|a| if a < nb { v[(i0 + a) * r + k] } else { 0.0 });
            let row = &vt[k * p..k * p + p];
            for j in 0..p {
                let y = row[j];
                d0[j] += s[0] * y;
                d1[j] += s[1] * y;
                d2[j] += s[2] * y;
                d3[j] += s[3] * y;
            }
        }
    }
    let mut f = 0.0;
    for a in 0..nb {
        let i = i0 + a;
        let cm = &b.confm[i * p..i * p + p];
        let fm = &b.freem[i * p..i * p + p];
        let pl = &m.pl[i * p..i * p + p];
        let da = &mut d[a * p..a * p + p];
        let mut fs = [0.0f64; 4];
        for (((x, &c), &fr), &l) in da.iter_mut().zip(cm).zip(fm).zip(pl) {
            let dd = *x;
            let w = (l - rho * dd).max(0.0);
            let fc = l * dd + hr * dd * dd;
            let ff = (w * w - l * l) * inv;
            fs[0] += c * fc + fr * ff;
            *x = c * (l + rho * dd) - fr * w;
        }
        f += fs[0];
    }
    let n4 = p / 4 * 4;
    let (c0, rest) = d[..RB * p].split_at(p);
    let (c1, rest) = rest.split_at(p);
    let (c2, c3) = rest.split_at(p);
    for k in 0..r {
        let row = &vt[k * p..k * p + p];
        let mut s = [[0.0f64; 4]; RB];
        let mut j = 0;
        while j < n4 {
            for t in 0..4 {
                let y = row[j + t];
                s[0][t] += c0[j + t] * y;
                s[1][t] += c1[j + t] * y;
                s[2][t] += c2[j + t] * y;
                s[3][t] += c3[j + t] * y;
            }
            j += 4;
        }
        let mut tail = [0.0f64; RB];
        for j in n4..p {
            let y = row[j];
            tail[0] += c0[j] * y;
            tail[1] += c1[j] * y;
            tail[2] += c2[j] * y;
            tail[3] += c3[j] * y;
        }
        for a in 0..nb {
            gb[a * r + k] += ((s[a][0] + s[a][2]) + (s[a][1] + s[a][3])) + tail[a];
        }
    }
    f
}

pub fn transpose(v: &[f64], p: usize, r: usize, vt: &mut Vec<f64>) {
    vt.resize(p * r, 0.0);
    for i in 0..p {
        for k in 0..r {
            vt[k * p + i] = v[i * r + k];
        }
    }
}

/// Blocked, branch-free version of fv_rows (same pairs, same terms, different summation order).
pub fn fv_blk(b: &Board, m: &Mult, v: &[f64], g: &mut [f64]) -> f64 {
    let (p, r) = (b.p, m.r);
    g.iter_mut().for_each(|a| *a = 0.0);
    let f = linear_part(b, m, v, g);
    let mut vt = Vec::new();
    transpose(v, p, r, &mut vt);
    let mut d = vec![0.0; RB * p];
    let mut fp = 0.0;
    let mut i0 = 0;
    while i0 < p {
        let nb = RB.min(p - i0);
        fp += pair_block(b, m, v, &vt, i0, nb, &mut g[i0 * r..(i0 + nb) * r], &mut d);
        i0 += nb;
    }
    f + 0.5 * fp
}

/// fv_blk with fixed chunks of `chunk` rows across the rayon pool; chunk sums added in order.
pub fn fv_blk_par(b: &Board, m: &Mult, v: &[f64], g: &mut [f64], chunk: usize) -> f64 {
    use rayon::prelude::*;
    let (p, r) = (b.p, m.r);
    let chunk = chunk.div_ceil(RB) * RB;
    g.iter_mut().for_each(|a| *a = 0.0);
    let f = linear_part(b, m, v, g);
    let mut vt = Vec::new();
    transpose(v, p, r, &mut vt);
    let parts: Vec<f64> = g
        .par_chunks_mut(chunk * r)
        .enumerate()
        .map_init(
            || vec![0.0; RB * p],
            |d, (c, gc)| {
                let rows = gc.len() / r;
                let mut s = 0.0;
                let mut a = 0;
                while a < rows {
                    let nb = RB.min(rows - a);
                    s += pair_block(b, m, v, &vt, c * chunk + a, nb, &mut gc[a * r..(a + nb) * r], d);
                    a += nb;
                }
                s
            },
        )
        .collect();
    f + 0.5 * parts.iter().sum::<f64>()
}

/// BLAS form: G = V V^T (dgemm), one branch-free pass turning G into the coefficient matrix C in
/// place, then gV += C V (dgemm). `buf` is a P x P scratch.
pub fn fv_blas(b: &Board, m: &Mult, v: &[f64], g: &mut [f64], buf: &mut Vec<f64>) -> f64 {
    let (p, r, rho) = (b.p, m.r, m.rho);
    g.iter_mut().for_each(|a| *a = 0.0);
    let f = linear_part(b, m, v, g);
    buf.resize(p * p, 0.0);
    crate::blas::dgemm(false, true, p, p, r, 1.0, v, v, 0.0, buf);
    let inv = 1.0 / (2.0 * rho);
    let hr = 0.5 * rho;
    let mut fs = 0.0;
    for (((x, &c), &fr), &l) in buf.iter_mut().zip(&b.confm).zip(&b.freem).zip(&m.pl) {
        let dd = *x;
        let w = (l - rho * dd).max(0.0);
        fs += c * (l * dd + hr * dd * dd) + fr * ((w * w - l * l) * inv);
        *x = c * (l + rho * dd) - fr * w;
    }
    crate::blas::dgemm(false, false, p, r, p, 1.0, buf, v, 1.0, g);
    f + 0.5 * fs
}

/// fv_blas reading the pair kind as one byte (P^2 bytes) instead of two f64 masks (16 P^2 bytes),
/// so the elementwise pass streams 17 P^2 instead of 32 P^2 bytes.
pub fn fv_blas2(b: &Board, m: &Mult, v: &[f64], g: &mut [f64], buf: &mut Vec<f64>) -> f64 {
    let (p, r, rho) = (b.p, m.r, m.rho);
    g.iter_mut().for_each(|a| *a = 0.0);
    let f = linear_part(b, m, v, g);
    buf.resize(p * p, 0.0);
    crate::blas::dgemm(false, true, p, p, r, 1.0, v, v, 0.0, buf);
    let inv = 1.0 / (2.0 * rho);
    let hr = 0.5 * rho;
    let mut fs = [0.0f64; 2];
    for ((x, &k), &l) in buf.iter_mut().zip(&b.kind).zip(&m.pl) {
        let dd = *x;
        let c = (k == CONFLICT) as u8 as f64;
        let fr = (k == FREE) as u8 as f64;
        let w = (l - rho * dd).max(0.0);
        fs[0] += c * (l * dd + hr * dd * dd);
        fs[1] += fr * ((w * w - l * l) * inv);
        *x = c * (l + rho * dd) - fr * w;
    }
    crate::blas::dgemm(false, false, p, r, p, 1.0, buf, v, 1.0, g);
    f + 0.5 * (fs[0] + fs[1])
}

/// V = (U + e1) / 2 with U = W / ||W|| row-wise. Returns the row norms.
pub fn sphere(w: &[f64], r: usize, u: &mut [f64], v: &mut [f64], nrm: &mut [f64]) {
    for (pi, wr) in w.chunks_exact(r).enumerate() {
        let n = dot(wr, wr).sqrt();
        nrm[pi] = n;
        for k in 0..r {
            u[pi * r + k] = wr[k] / n;
            v[pi * r + k] = 0.5 * u[pi * r + k];
        }
        v[pi * r] += 0.5;
    }
}

/// gW = (gU - <gU, u> u) / ||w||, gU = gV / 2, in place on g.
pub fn pullback(u: &[f64], nrm: &[f64], r: usize, g: &mut [f64]) {
    for (pi, gr) in g.chunks_exact_mut(r).enumerate() {
        let ur = &u[pi * r..pi * r + r];
        let s = 0.5 * dot(gr, ur);
        let inv = 1.0 / nrm[pi];
        for k in 0..r {
            gr[k] = (0.5 * gr[k] - s * ur[k]) * inv;
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Viol {
    pub cell_sum: f64,
    pub tile_sum: f64,
    pub diag: f64,
    pub row_cell: f64,
    pub row_tile: f64,
    pub zero: f64,
    pub x_nonneg: f64,
    pub y_nonneg: f64,
    pub max: f64,
}

/// bm.max_violation: every constraint of lift.py's relaxation, explicitly.
pub fn max_violation(b: &Board, v: &[f64], r: usize) -> Viol {
    let p = b.p;
    let x: Vec<f64> = (0..p).map(|q| v[q * r]).collect();
    let mut o = Viol::default();
    let sums = |groups: &Vec<Vec<usize>>| -> Vec<f64> {
        let mut s = vec![0.0; groups.len() * r];
        for (c, grp) in groups.iter().enumerate() {
            for &q in grp {
                axpy(&mut s[c * r..c * r + r], 1.0, &v[q * r..q * r + r]);
            }
        }
        s
    };
    let sc = sums(&b.cells);
    let st = sums(&b.tiles);
    for c in 0..b.nc {
        o.cell_sum = o.cell_sum.max((sc[c * r] - 1.0).abs());
    }
    for t in 0..b.nt {
        o.tile_sum = o.tile_sum.max((st[t * r] - 1.0).abs());
    }
    for q in 0..p {
        let vq = &v[q * r..q * r + r];
        o.diag = o.diag.max((dot(vq, vq) - x[q]).abs());
        for c in 0..b.nc {
            if c != b.cell[q] {
                o.row_cell = o.row_cell.max((dot(vq, &sc[c * r..c * r + r]) - x[q]).abs());
            }
        }
        for t in 0..b.nt {
            if t != b.tile[q] {
                o.row_tile = o.row_tile.max((dot(vq, &st[t * r..t * r + r]) - x[q]).abs());
            }
        }
        o.x_nonneg = o.x_nonneg.max(-x[q]);
    }
    let mut gmin = f64::INFINITY;
    for i in 0..p {
        for j in i + 1..p {
            let d = dot(&v[i * r..i * r + r], &v[j * r..j * r + r]);
            if b.kind[i * p + j] == CONFLICT {
                o.zero = o.zero.max(d.abs());
            } else {
                gmin = gmin.min(d);
            }
        }
    }
    o.y_nonneg = (-gmin).max(0.0);
    o.max = [o.cell_sum, o.tile_sum, o.diag, o.row_cell, o.row_tile, o.zero, o.x_nonneg, o.y_nonneg]
        .iter()
        .fold(0.0f64, |a, &c| a.max(c));
    o
}

/// SphereAL.update_multipliers.
pub fn update_multipliers(b: &Board, m: &mut Mult, v: &[f64]) {
    let (p, r, rho) = (b.p, m.r, m.rho);
    for (groups, lam) in [(&b.cells, &mut m.lc), (&b.tiles, &mut m.lt)] {
        for (c, grp) in groups.iter().enumerate() {
            let mut h = vec![0.0; r];
            for &q in grp {
                axpy(&mut h, 1.0, &v[q * r..q * r + r]);
            }
            h[0] -= 1.0;
            axpy(&mut lam[c * r..c * r + r], rho, &h);
        }
    }
    for i in 0..p {
        for j in i + 1..p {
            let d = dot(&v[i * r..i * r + r], &v[j * r..j * r + r]);
            let k = b.kind[i * p + j];
            let nv = if k == CONFLICT { m.pl[i * p + j] + rho * d } else { (m.pl[i * p + j] - rho * d).max(0.0) };
            m.pl[i * p + j] = nv;
            m.pl[j * p + i] = nv;
        }
    }
}

pub fn active_nonneg(b: &Board, m: &Mult) -> usize {
    let p = b.p;
    let mut a = 0;
    for i in 0..p {
        for j in i + 1..p {
            if b.kind[i * p + j] == FREE && m.pl[i * p + j] > 0.0 {
                a += 1;
            }
        }
    }
    a
}

/// max_violation from a precomputed Gram matrix G = V V^T (buf, P x P): same quantities. The row
/// sums <v_p, S_c> are sums of row p of G over cell c's placements.
pub fn max_violation_gram(b: &Board, v: &[f64], r: usize, gram: &[f64]) -> Viol {
    let p = b.p;
    let mut o = Viol::default();
    let mut sc = vec![0.0; b.nc];
    let mut st = vec![0.0; b.nt];
    for q in 0..p {
        sc[b.cell[q]] += v[q * r];
        st[b.tile[q]] += v[q * r];
    }
    o.cell_sum = sc.iter().fold(0.0f64, |a, &x| a.max((x - 1.0).abs()));
    o.tile_sum = st.iter().fold(0.0f64, |a, &x| a.max((x - 1.0).abs()));
    let mut gmin = f64::INFINITY;
    let mut rc = vec![0.0; b.nc];
    let mut rt = vec![0.0; b.nt];
    for i in 0..p {
        let row = &gram[i * p..i * p + p];
        let kind = &b.kind[i * p..i * p + p];
        let xi = v[i * r];
        o.diag = o.diag.max((row[i] - xi).abs());
        o.x_nonneg = o.x_nonneg.max(-xi);
        rc.iter_mut().for_each(|a| *a = 0.0);
        rt.iter_mut().for_each(|a| *a = 0.0);
        for j in 0..p {
            let d = row[j];
            rc[b.cell[j]] += d;
            rt[b.tile[j]] += d;
            if j > i {
                if kind[j] == CONFLICT {
                    o.zero = o.zero.max(d.abs());
                } else {
                    gmin = gmin.min(d);
                }
            }
        }
        for c in 0..b.nc {
            if c != b.cell[i] {
                o.row_cell = o.row_cell.max((rc[c] - xi).abs());
            }
        }
        for t in 0..b.nt {
            if t != b.tile[i] {
                o.row_tile = o.row_tile.max((rt[t] - xi).abs());
            }
        }
    }
    o.y_nonneg = (-gmin).max(0.0);
    o.max = [o.cell_sum, o.tile_sum, o.diag, o.row_cell, o.row_tile, o.zero, o.x_nonneg, o.y_nonneg]
        .iter()
        .fold(0.0f64, |a, &c| a.max(c));
    o
}

/// update_multipliers from a precomputed Gram matrix (same values).
pub fn update_multipliers_gram(b: &Board, m: &mut Mult, v: &[f64], gram: &[f64]) {
    update_multipliers_gram_mode(b, m, v, gram, 0)
}

/// mode 0: every multiplier (bm.py); 1: no nonnegativity multipliers (I2 stays a pure penalty,
/// so nothing of size F is stored); 2: no multipliers at all (quadratic penalty method).
pub fn update_multipliers_gram_mode(b: &Board, m: &mut Mult, v: &[f64], gram: &[f64], mode: u8) {
    if mode == 2 {
        return;
    }
    let (p, r, rho) = (b.p, m.r, m.rho);
    for (groups, lam) in [(&b.cells, &mut m.lc), (&b.tiles, &mut m.lt)] {
        for (c, grp) in groups.iter().enumerate() {
            let mut h = vec![0.0; r];
            for &q in grp {
                axpy(&mut h, 1.0, &v[q * r..q * r + r]);
            }
            h[0] -= 1.0;
            axpy(&mut lam[c * r..c * r + r], rho, &h);
        }
    }
    for ((l, &k), &d) in m.pl.iter_mut().zip(&b.kind).zip(gram) {
        if k == CONFLICT {
            *l += rho * d;
        } else if k == FREE && mode == 0 {
            *l = (*l - rho * d).max(0.0);
        }
    }
}

/// Row-compressed pair lists for the structured kernel: conflict pairs with their lz, and the
/// ACTIVE nonnegativity multipliers (mu > 0) only. Both rows of a symmetric pair are stored.
pub struct Sparse {
    pub rp: Vec<usize>,
    pub cj: Vec<u32>,
    pub val: Vec<f64>,
}

/// Builds (conflicts, active nonnegativity) lists from the multiplier state. At 5x5 the state is
/// held densely between updates; the kernel below reads only these lists.
pub fn build_sparse(b: &Board, m: &Mult) -> (Sparse, Sparse) {
    let p = b.p;
    let mut c = Sparse { rp: vec![0], cj: vec![], val: vec![] };
    let mut a = Sparse { rp: vec![0], cj: vec![], val: vec![] };
    for i in 0..p {
        for j in 0..p {
            let k = b.kind[i * p + j];
            if k == CONFLICT {
                c.cj.push(j as u32);
                c.val.push(m.pl[i * p + j]);
            } else if k == FREE && m.pl[i * p + j] > 0.0 {
                a.cj.push(j as u32);
                a.val.push(m.pl[i * p + j]);
            }
        }
        c.rp.push(c.cj.len());
        a.rp.push(a.cj.len());
    }
    (c, a)
}

pub const SB: usize = 64;

/// Structured kernel: no P x P object. Per block of SB rows: G_B = V_B V^T (dgemm, SB x P), the
/// generic free-pair term max(0, -rho G) on every entry, then the conflict and active entries
/// patched from their lists (their G read from the block, not gathered), then g_B += C_B V (dgemm).
/// Memory: SB P doubles of scratch + 2 K + 2 A list entries. Flops: 4 P^2 r + O(P^2 + K + A).
pub fn fv_struct(b: &Board, m: &Mult, cs: &Sparse, act: &Sparse, v: &[f64], g: &mut [f64], buf: &mut Vec<f64>) -> f64 {
    let (p, r, rho) = (b.p, m.r, m.rho);
    g.iter_mut().for_each(|a| *a = 0.0);
    let f = linear_part(b, m, v, g);
    let inv = 1.0 / (2.0 * rho);
    let hr = 0.5 * rho;
    buf.resize(SB * p, 0.0);
    let mut fs = 0.0;
    let mut patch: Vec<(usize, f64)> = Vec::new();
    let mut i0 = 0;
    while i0 < p {
        let nb = SB.min(p - i0);
        crate::blas::dgemm(false, true, nb, p, r, 1.0, &v[i0 * r..(i0 + nb) * r], v, 0.0, &mut buf[..nb * p]);
        for a in 0..nb {
            let i = i0 + a;
            let row = &mut buf[a * p..a * p + p];
            patch.clear();
            for q in cs.rp[i]..cs.rp[i + 1] {
                let j = cs.cj[q] as usize;
                let d = row[j];
                let l = cs.val[q];
                let w0 = (-rho * d).max(0.0);
                fs += l * d + hr * d * d - w0 * w0 * inv;
                patch.push((j, l + rho * d));
            }
            for q in act.rp[i]..act.rp[i + 1] {
                let j = act.cj[q] as usize;
                let d = row[j];
                let l = act.val[q];
                let w0 = (-rho * d).max(0.0);
                let w = (l - rho * d).max(0.0);
                fs += (w * w - l * l) * inv - w0 * w0 * inv;
                patch.push((j, -w));
            }
            let mut s = 0.0;
            for x in row.iter_mut() {
                let w = (-rho * *x).max(0.0);
                s += w * w;
                *x = -w;
            }
            fs += s * inv;
            row[i] = 0.0;
            for &(j, c) in &patch {
                row[j] = c;
            }
        }
        crate::blas::dgemm(false, false, nb, r, p, 1.0, &buf[..nb * p], v, 1.0, &mut g[i0 * r..(i0 + nb) * r]);
        i0 += nb;
    }
    f + 0.5 * fs
}
