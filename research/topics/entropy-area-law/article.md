---
id: entropy-area-law
title: Entropy and the area law
summary: The matching rules alone are rich (positive entropy density), so the hardness lives entirely in the use-each-piece-once rule, whose cost grows with area; the exact in-repo block counts through 3x3 fit an area law with exponent alpha near 0.044.
status: published
created: 2026-06-22
updated: 2026-07-22
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
results:
  - label: Entropy densities, horizontal bound, and exact A(n)/B(n) block counts (JSON, computed here)
    path: results/entropy.json
site:
  render: true
  dataFile: results/entropy.json
---

# Entropy and the area law

> **Created:** 2026-06-22 · **Updated:** 2026-07-22

## Summary

Eternity II has two rules: touching edges must match colors, and each of the 256
pieces is used exactly once. This finding separates them and asks which one is the
source of the hardness. The matching rule alone turns out to be generous: the
number of valid all-matched patches grows with a healthy, positive entropy
density. The hardness is entirely in the second rule. Its cost grows with the
*area* of a patch. We count that cost exactly, in-repo, for interior blocks up to
3x3, and fit an area law whose exponent runs near 0.044 over the computed range.

## The matching grammar is rich

Forget the use-each-piece-once rule for a moment and treat the 196 interior pieces
as reusable tiles. Count the valid all-matched patches and you find they grow
exponentially with the number of cells. The growth rate per cell is an entropy
density, and it is comfortably positive: there is no shortage of locally-valid
ways to tile.

We measure it width by width. For an n-wide strip, the per-cell entropy is the
log of the dominant eigenvalue of a row-transfer matrix, and the sequence
h(1), h(2), h(3), … decreases to the true two-dimensional density h_infinity:

| width n | h(n) (log₁₀ per cell) |
|---:|---:|
| 1 | 1.6645 |
| 2 | 1.0497 |
| 3 | 0.8449 |
| 4 | 0.7425 |
| limit | ≈ 0.67 |

The first two rows are computed exactly here; the rest come from a heavier offline
sweep. The density is positive, so the matching grammar is "good": rich, not
restrictive.

## The area law, counted exactly

Now put the use-each-piece-once rule back. Take an interior n×n block with a free
outer border and count two things exactly: A(n), the number of color-valid
fillings when pieces may repeat (the matching grammar), and B(n), the number that
use distinct pieces (true Eternity II). Their ratio ρ(n) = B(n)/A(n) is the
realizable fraction. A(n) is a dense broken-profile transfer count; B(n) is a
parallel distinct-counting depth-first search whose cost is bounded by A(n). Both
are computed in this crate.

| n | cells | A(n) reusable | B(n) distinct | ρ(n) = B/A |
|---:|---:|---:|---:|---:|
| 1 | 1 | 784 | 784 | 1.0000 |
| 2 | 4 | 4,550,669 | 4,059,952 | 0.8922 |
| 3 | 9 | 195,941,569,440 | 126,907,480,216 | 0.6477 |
| 4 | 16 | 62,607,390,876,341,976 | *over budget* | n/a |

The n=2 distinct count B(2) = 4,059,952 matches the in-repo
[sub-grid placement counts](../subgrid-placement-counts/) reference table exactly.
B(3) is a roughly 1.3×10¹¹-node search that runs in about seven minutes on 8
cores. At n=4 the reusable count A(4) is already 6.3×10¹⁶, so the distinct DFS is
out of reach in reasonable time; **that is the wall.** A(4) is still reported
because the transfer count is cheap.

ρ(n) decays in the *area* n², not the perimeter. Fitting ρ(n) ≈ exp(−α n²) by
least squares through the origin over the exact range gives

$$ \rho(n) \;\approx\; \exp(-\alpha\, n^2), \qquad \alpha \approx 0.0445. $$

The per-n exponent α(n) = −ln ρ(n)/n² is 0.0285 at n=2 and 0.0483 at n=3: it is
still rising with n over this small exact range, so 0.0445 is a lower estimate of
the large-patch exponent. Extrapolating the fit, ρ(n) drops below one in a
thousand near 155 cells. An area-law decay is brutal, because area grows
quadratically: past that scale almost no color-valid patch can be realized with
distinct pieces.

(An earlier off-site single-instance study reported α ≈ 0.085 and a collapse near
80 cells. The exact in-repo counts over n ≤ 3 give the smaller α ≈ 0.044 and a
later collapse near 155 cells; the rising α(n) is consistent with the exponent
climbing toward the off-site figure at larger blocks than we can enumerate.)

## Why it matters

A collapse in the low hundreds of cells is on the scale of the moves that separate
the best known boards (see
[why basin-hopping is impossible](sigma-cycles), where the cycles run to 80 and
154 cells). The two facts line up: the matching grammar stays rich, then the
distinctness rule collapses it over the area. So the wall everyone hits isn't in
the part of the puzzle that looks hard (matching colors); it's in the quiet
bookkeeping rule that you only use each piece once. That rule has an entropy cost
that grows with area, and the board is just large enough for it to bite.

## The theorem, briefly

The per-width entropy is well-defined and the limit exists. Concatenating an
$n_1$-wide and an $n_2$-wide strip side by side adds a seam constraint, so the
counts satisfy $\lambda_{n_1+n_2} \le \lambda_{n_1}\lambda_{n_2}$; taking logs,
$\log\lambda_n$ is subadditive, and Fekete's lemma gives

$$ h_\infty \;=\; \lim_{n\to\infty}\frac{\log_{10}\lambda_n}{n} \;=\; \inf_n \frac{\log_{10}\lambda_n}{n}. $$

That the infimum is reached from above is exactly why the table of h(n) decreases.
The upper bound is the 1-D horizontal compatibility rate: ignoring vertical
constraints only adds patches, so $h_\infty \le \log_{10}\lambda_H$ where
$\lambda_H = 46.18$ is the spectral radius of the horizontal color-compatibility
matrix, giving $h_\infty \le 1.6645$. Positivity holds because the grammar
supports exponentially many chains. So $0 < h_\infty \le 1.6645$, measured at
about 0.67.

## Reproduce

```sh
cd research/topics/entropy-area-law/compute
cargo run --release > ../results/entropy.json
```

Deterministic. h(1), h(2) and $\lambda_H$ are recomputed exactly, as are the block
counts A(n) for n ≤ 4 and B(n) for n ≤ 3 and the α fit over that range. h(3), h(4)
and h_infinity are carried from the offline width-n sweep and labelled as such; the
ρ(n) curve and the ~155-cell collapse point beyond n=3 are extrapolations of the
in-repo fit.

## Notes

- "Entropy density" here is the per-cell growth rate of the count of valid patches,
  in log base ten. Positive means the count grows exponentially with area.
- A(n) and B(n) are exact integer counts. The area-law exponent α and the collapse
  cell count are stated as consequences of the in-repo fit, with the part beyond
  the computed range labelled as an extrapolation.
