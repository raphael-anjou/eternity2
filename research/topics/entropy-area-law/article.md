---
id: entropy-area-law
title: Entropy and the use-once rule
summary: With pieces reusable, colour-valid patches grow exponentially (the strip entropy h(n) is 0.640 per cell at width 6, an upper bound on the limit); the share of those patches that use distinct pieces is counted exactly to 3x3 (0.892, 0.648) and sampled beyond (0.33, 0.11, 0.024, 0.0004 at 16, 24, 32, 48 cells), crossing one in a thousand near 44 cells.
status: published
created: 2026-06-22
updated: 2026-11-13
contributors:
  - name: Raphaël Anjou
    role: author
tags:
  - structure
  - entropy
  - theorem
sources:
  - label: "Eternity II board viewer (e2.bucas.name)"
    url: https://e2.bucas.name
reproduce:
  - cd research/topics/entropy-area-law/compute && cargo run --release > ../results/entropy.json
  - just research-entropy-area-law-scripts
results:
  - label: Entropy densities h(1) to h(6), horizontal bound, exact A(n)/B(n) block counts, sampled distinct-piece fractions, and the withdrawn values with their reasons (JSON)
    path: results/entropy.json
site:
  render: true
  dataFile: results/entropy.json
---

# Entropy and the use-once rule

> **Created:** 2026-06-22 · **Updated:** 2026-11-13

## Summary

Eternity II has two rules: touching edges must match colours, and each of the
256 pieces is used exactly once. This topic separates them. With the second
rule dropped, the number of colour-valid patches grows exponentially with the
number of cells. With it, the share of colour-valid patches that use distinct
pieces falls fast as patches grow: exactly 0.892 at 2x2 and 0.648 at 3x3, and,
sampled for patches four cells wide, 0.33 at 16 cells, 0.11 at 24, 0.024 at 32
and 0.0004 at 48. It crosses one in a thousand near 44 cells.

An earlier version of this topic reported an entropy limit of 0.67, an
area-law exponent near 0.044 fitted to the three exact points, and a collapse
near 155 cells extrapolated from that fit. All three are withdrawn (see
"What was withdrawn" below); the results file keeps them under `withdrawn`
with the reason.

## The matching grammar is rich

Treat the 196 interior pieces as reusable tiles. For an n-wide strip, the
per-cell entropy h(n) is log10 of the dominant eigenvalue of the row-transfer
operator, divided by n:

| width n | lambda_n | h(n) (log10 per cell) | source |
|---:|---:|---:|---|
| 1 | 46.182 | 1.6645 | this crate |
| 2 | 125.745 | 1.0497 | this crate |
| 3 | 342.502 | 0.8449 | `strip_entropy.js` |
| 4 | 932.949 | 0.7425 | `strip_entropy.js` |
| 5 | 2,541.305 | 0.6810 | `strip_entropy.js` |
| 6 | 6,922.392 | 0.6400 | `strip_entropy.js` |

`strip_entropy.js` is the stdlib script in `compute/scripts/`; it agrees with this
crate at widths 1 and 2. The sequence decreases to the two-dimensional density
h_infinity, which is its infimum (below), so h(6) = 0.640 bounds h_infinity
from above. Each added row multiplies lambda by about 10^0.435 (the increment
log10(lambda_n / lambda_(n-1)) is 0.4350 at width 2 and 0.4352 at widths 3 to
6), so h(n) is close to 0.435 + 1.23/n over the computed widths. That suggests
a limit close to 0.435, next to the mean-field value log10(784/289) = 0.433;
it is a reading of six widths, not a computed limit, and the results file
states only the upper bound.

## The distinct-piece fraction

Now put the use-each-piece-once rule back. Take an interior n×n block with a
free outer border and count exactly A(n), the colour-valid fillings when
pieces may repeat, and B(n), those that use distinct pieces. Their ratio
rho(n) = B(n)/A(n) is the realizable fraction. A(n) is a dense broken-profile
transfer count; B(n) is a parallel distinct-counting depth-first search whose
cost is bounded by A(n). Both are computed in this crate.

| n | cells | A(n) reusable | B(n) distinct | rho(n) = B/A |
|---:|---:|---:|---:|---:|
| 1 | 1 | 784 | 784 | 1.0000 |
| 2 | 4 | 4,550,669 | 4,059,952 | 0.8922 |
| 3 | 9 | 195,941,569,440 | 126,907,480,216 | 0.6477 |
| 4 | 16 | 62,607,390,876,341,976 | *over budget* | n/a |

B(2) = 4,059,952 matches the in-repo
[sub-grid placement counts](../subgrid-placement-counts/) reference table. B(3)
is a roughly 1.3×10¹¹-node search that runs in about seven minutes on 8 cores.
At n=4 the reusable count is already 6.3×10¹⁶, out of reach for the distinct
search.

Beyond 3x3 the fraction is sampled: `distinct_fraction.js` (same folder)
draws colour-valid patches uniformly at random and checks whether they use
distinct pieces. It reproduces the exact 2x2 and 3x3 values within sampling
error.

| patch | cells | rho (sampled) | standard error | draws | -ln(rho) per cell |
|---|---:|---:|---:|---:|---:|
| 4x4 | 16 | 0.3263 | 0.00074 | 400,000 | 0.070 |
| 4x6 | 24 | 0.1060 | 0.00049 | 400,000 | 0.094 |
| 4x8 | 32 | 0.02383 | 0.00024 | 400,000 | 0.117 |
| 4x12 | 48 | 0.000375 | 0.000019 | 1,000,000 | 0.164 |

-ln(rho) per cell keeps rising, so no single area-law exponent describes the
decay: it is faster than exp(-alpha × cells) for any fixed alpha over this
range. Log-linear interpolation between 32 and 48 cells crosses one in a
thousand at 44.2 cells (`entropy_fields.py`). These patches are four cells
wide; square patches of the same area were not sampled.

## What was withdrawn

- **h_infinity = 0.67.** Not computed by this crate: a constant carried from
  an off-repo width-n sweep. Since h_infinity is the infimum of h(n) and
  h(6) = 0.640, the value is impossible.
- **Area-law exponent 0.0445.** A least-squares fit of ln rho = -alpha n² over
  the three exact points, then used to draw rho beyond n = 3. The sampled
  fraction at 48 cells is about 315 times below the fit's value there
  (0.000375 against 0.118).
- **Collapse near 155 cells.** ln(1000)/0.0445, from the withdrawn exponent.
  The sampled fraction crosses one in a thousand near 44 cells.

The off-site study that reported an exponent near 0.085 and a collapse near
80 cells was cited only for comparison with the withdrawn fit, and that
comparison was dropped with it.

## The theorem, briefly

Concatenating an $n_1$-wide and an $n_2$-wide strip side by side adds a seam
constraint, so $\lambda_{n_1+n_2} \le \lambda_{n_1}\lambda_{n_2}$; taking logs,
$\log\lambda_n$ is subadditive, and Fekete's lemma gives

$$ h_\infty \;=\; \lim_{n\to\infty}\frac{\log_{10}\lambda_n}{n} \;=\; \inf_n \frac{\log_{10}\lambda_n}{n}. $$

So every computed width is an upper bound. Ignoring vertical constraints only
adds patches, so $h_\infty \le \log_{10}\lambda_H = 1.6645$ with
$\lambda_H = 46.18$ the spectral radius of the horizontal colour-compatibility
matrix; h(6) tightens that to $h_\infty \le 0.640$. Strict positivity of the
limit does not follow from this argument and is not claimed here.

## Reproduce

```sh
cd research/topics/entropy-area-law/compute
cargo run --release > ../results/entropy.json
```

Deterministic. The crate recomputes lambda_H, h(1), h(2), the width-2
increment, A(n) for n ≤ 4, B(n) for n ≤ 3 and the per-n -ln(rho(n))/n². It
carries, verbatim and with their source named in the output, h(3) to h(6),
the sampled fractions, the 44-cell crossing and the withdrawn values. Those
come from the stdlib scripts in `compute/scripts/` (`strip_entropy.js`,
`distinct_fraction.js`, `entropy_fields.py`), each with its committed `.out`,
rerun by `just research-entropy-area-law-scripts`. The strip eigenvalues are
exact; the distinct-piece fractions are seeded samples. Two runs with
different seeds (1 to 6 and 101 to 106) agree within their combined standard
error (largest gap 1.8 combined standard errors, at 32 cells) and put the
1e-3 crossing at 44.2 and 44.4 cells; the 44-cell figure therefore rests on
sampled values, not on exact counts.

## Notes

- "Entropy density" is the per-cell growth rate of the count of valid
  patches, in log base ten.
- A(n) and B(n) are exact integer counts; the fractions beyond 3x3 are
  sampled estimates with their standard errors.
