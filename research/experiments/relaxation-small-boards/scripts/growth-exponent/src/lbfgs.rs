//! L-BFGS (two-loop recursion, H0 = s'y / y'y) with a strong Wolfe line search (bracketing and
//! zoom with safeguarded cubic interpolation, Nocedal and Wright algorithms 3.5 and 3.6).
//! Stopping as scipy L-BFGS-B without bounds: max_iter, max|g| <= pgtol, relative decrease
//! (f_k - f_k+1) / max(|f_k|, |f_k+1|, 1) <= ftol, max_fun evaluations.

pub struct Opts {
    pub m: usize,
    pub max_iter: usize,
    pub pgtol: f64,
    pub ftol: f64,
    pub max_fun: usize,
    pub c1: f64,
    pub c2: f64,
    pub max_ls: usize,
}

impl Default for Opts {
    fn default() -> Self {
        // scipy L-BFGS-B line search: ftol 1e-3, gtol 0.9, at most 20 evaluations
        Opts { m: 10, max_iter: 300, pgtol: 1e-9, ftol: 1e-14, max_fun: 15000, c1: 1e-3, c2: 0.9, max_ls: 20 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Out {
    pub iters: usize,
    pub evals: usize,
    pub f: f64,
    pub stop: &'static str,
}

fn dotv(a: &[f64], b: &[f64]) -> f64 {
    crate::al::dot(a, b)
}

/// Minimise from x (overwritten). `fg(x, g) -> f`. `on_iter(x, f) -> true` stops the run.
pub fn minimize<F, C>(x: &mut [f64], mut fg: F, o: &Opts, mut on_iter: C) -> Out
where
    F: FnMut(&[f64], &mut [f64]) -> f64,
    C: FnMut(&[f64], f64) -> bool,
{
    let n = x.len();
    let mut g = vec![0.0; n];
    let mut f = fg(x, &mut g);
    let mut evals = 1usize;
    let mut s_hist: Vec<Vec<f64>> = Vec::with_capacity(o.m);
    let mut y_hist: Vec<Vec<f64>> = Vec::with_capacity(o.m);
    let mut rho_hist: Vec<f64> = Vec::with_capacity(o.m);
    let mut d = vec![0.0; n];
    let mut xn = vec![0.0; n];
    let mut gn = vec![0.0; n];
    let mut alpha = vec![0.0; o.m];
    let mut iters = 0;
    let mut stop = "max_iter";
    while iters < o.max_iter {
        if g.iter().fold(0.0f64, |a, &b| a.max(b.abs())) <= o.pgtol {
            stop = "pgtol";
            break;
        }
        // two-loop recursion
        d.copy_from_slice(&g);
        let k = s_hist.len();
        for i in (0..k).rev() {
            alpha[i] = rho_hist[i] * dotv(&s_hist[i], &d);
            crate::al::axpy(&mut d, -alpha[i], &y_hist[i]);
        }
        if k > 0 {
            let gam = dotv(&s_hist[k - 1], &y_hist[k - 1]) / dotv(&y_hist[k - 1], &y_hist[k - 1]);
            d.iter_mut().for_each(|a| *a *= gam);
        }
        for i in 0..k {
            let beta = rho_hist[i] * dotv(&y_hist[i], &d);
            crate::al::axpy(&mut d, alpha[i] - beta, &s_hist[i]);
        }
        d.iter_mut().for_each(|a| *a = -*a);
        let mut dg0 = dotv(&g, &d);
        if !(dg0 < 0.0) {
            s_hist.clear();
            y_hist.clear();
            rho_hist.clear();
            for i in 0..n {
                d[i] = -g[i];
            }
            dg0 = dotv(&g, &d);
        }
        let step0 = if s_hist.is_empty() { 1.0 / dotv(&d, &d).sqrt() } else { 1.0 };
        let (ok, fnew, ne) = line_search(x, f, dg0, &d, step0, &mut fg, o, &mut xn, &mut gn);
        evals += ne;
        if !ok {
            stop = "line_search";
            break;
        }
        // history update
        let mut s = if s_hist.len() == o.m { s_hist.remove(0) } else { vec![0.0; n] };
        let mut y = if y_hist.len() == o.m { y_hist.remove(0) } else { vec![0.0; n] };
        if rho_hist.len() == o.m {
            rho_hist.remove(0);
        }
        for i in 0..n {
            s[i] = xn[i] - x[i];
            y[i] = gn[i] - g[i];
        }
        let sy = dotv(&s, &y);
        let yy = dotv(&y, &y);
        if sy > 2.2e-16 * yy {
            s_hist.push(s);
            y_hist.push(y);
            rho_hist.push(1.0 / sy);
        }
        let rel = (f - fnew) / f.abs().max(fnew.abs()).max(1.0);
        x.copy_from_slice(&xn);
        g.copy_from_slice(&gn);
        f = fnew;
        iters += 1;
        if on_iter(x, f) {
            stop = "callback";
            break;
        }
        if rel <= o.ftol {
            stop = "ftol";
            break;
        }
        if evals >= o.max_fun {
            stop = "max_fun";
            break;
        }
    }
    Out { iters, evals, f, stop }
}

fn cubic_min(a: f64, fa: f64, da: f64, b: f64, fb: f64, db: f64) -> f64 {
    // minimiser of the cubic through (a, fa, da), (b, fb, db); midpoint on failure
    let d1 = da + db - 3.0 * (fa - fb) / (a - b);
    let disc = d1 * d1 - da * db;
    if disc < 0.0 || !disc.is_finite() {
        return 0.5 * (a + b);
    }
    let d2 = (b - a).signum() * disc.sqrt();
    let t = b - (b - a) * (db + d2 - d1) / (db - da + 2.0 * d2);
    if t.is_finite() {
        t
    } else {
        0.5 * (a + b)
    }
}

#[allow(clippy::too_many_arguments)]
fn line_search<F>(
    x: &[f64],
    f0: f64,
    dg0: f64,
    d: &[f64],
    step0: f64,
    fg: &mut F,
    o: &Opts,
    xn: &mut [f64],
    gn: &mut [f64],
) -> (bool, f64, usize)
where
    F: FnMut(&[f64], &mut [f64]) -> f64,
{
    let n = x.len();
    let mut ne = 0;
    let mut eval = |a: f64, xn: &mut [f64], gn: &mut [f64], ne: &mut usize| -> (f64, f64) {
        for i in 0..n {
            xn[i] = x[i] + a * d[i];
        }
        let f = fg(xn, gn);
        *ne += 1;
        (f, dotv(gn, d))
    };
    let (mut a_prev, mut f_prev, mut dg_prev) = (0.0, f0, dg0);
    let mut a = step0;
    let mut lo;
    let mut hi;
    let mut i = 0;
    loop {
        let (f, dg) = eval(a, xn, gn, &mut ne);
        if !f.is_finite() {
            // step too long: shrink
            a *= 0.1;
            i += 1;
            if i >= o.max_ls {
                return (false, f0, ne);
            }
            continue;
        }
        if f > f0 + o.c1 * a * dg0 || (i > 0 && f >= f_prev) {
            lo = (a_prev, f_prev, dg_prev);
            hi = (a, f, dg);
            break;
        }
        if dg.abs() <= -o.c2 * dg0 {
            return (true, f, ne);
        }
        if dg >= 0.0 {
            lo = (a, f, dg);
            hi = (a_prev, f_prev, dg_prev);
            break;
        }
        a_prev = a;
        f_prev = f;
        dg_prev = dg;
        a *= 4.0;
        i += 1;
        if i >= o.max_ls {
            return (true, f, ne);
        }
    }
    // zoom
    while ne < o.max_ls {
        let (alo, flo, dlo) = lo;
        let (ahi, fhi, dhi) = hi;
        let w = ahi - alo;
        let mut t = cubic_min(alo, flo, dlo, ahi, fhi, dhi);
        let (l, h) = if w > 0.0 { (alo + 0.1 * w, ahi - 0.1 * w) } else { (ahi - 0.1 * w, alo + 0.1 * w) };
        let (l, h) = if l <= h { (l, h) } else { (h, l) };
        if !(t >= l && t <= h) {
            t = 0.5 * (alo + ahi);
        }
        let (f, dg) = eval(t, xn, gn, &mut ne);
        if f > f0 + o.c1 * t * dg0 || f >= flo {
            hi = (t, f, dg);
        } else {
            if dg.abs() <= -o.c2 * dg0 {
                return (true, f, ne);
            }
            if dg * (ahi - alo) >= 0.0 {
                hi = lo;
            }
            lo = (t, f, dg);
        }
        if (hi.0 - lo.0).abs() < 1e-16 * lo.0.abs().max(1e-300) {
            break;
        }
    }
    // no strong Wolfe point within the budget: take the best point seen if it decreased f
    if lo.0 > 0.0 && lo.1 < f0 {
        let (f, _) = eval(lo.0, xn, gn, &mut ne);
        return (true, f, ne);
    }
    (false, f0, ne)
}
