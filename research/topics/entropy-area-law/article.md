---
id: entropy-area-law
title: Entropy and the area law
summary: The matching rules alone are rich (positive entropy density), so the hardness lives entirely in the use-each-piece-once rule, whose cost grows with area and collapses distinctness past ~80 cells.
status: published
created: 2026-06-22
updated: 2026-06-22
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
  - label: Entropy densities and the horizontal bound (JSON, computed here)
    path: results/entropy.json
site:
  render: true
  dataFile: results/entropy.json
---

# Entropy and the area law

> **Created:** 2026-06-22 · **Updated:** 2026-06-22

## Summary

Eternity II has two rules: touching edges must match colors, and each of the 256
pieces is used exactly once. This finding separates them and asks which one is the
source of the hardness. The matching rule alone turns out to be generous: the
number of valid all-matched patches grows with a healthy, positive entropy
density. The hardness is entirely in the second rule. Its cost grows with the
*area* of a patch, and that cost wipes out the matching rule's richness at around
eighty cells, which is exactly the scale of the moves that separate the best known
boards.

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

## The area law

Now put the use-each-piece-once rule back. Ask: among random color-valid n×n
patches, what fraction actually use distinct pieces? Call it ρ(n). It falls off
fast, and the way it falls is the key: it decays in the *area* n², not the
perimeter,

$$ \rho(n) \;\approx\; \exp(-\alpha\, n^2), \qquad \alpha \approx 0.085. $$

An area-law decay is brutal, because area grows quadratically. Plug in numbers and
ρ(n) drops below one in a thousand at roughly eighty cells. Past that size, almost
no color-valid patch can be realized with distinct pieces.

## Why it matters

Eighty cells is not an arbitrary number. It is the size of the smallest moves that
separate the best known boards from each other (see
[why basin-hopping is impossible](sigma-cycles), where the cycles run to 80 and
154 cells). The two facts line up: the matching grammar stays rich up to about
that scale, then the distinctness rule collapses it. So the wall everyone hits
isn't in the part of the puzzle that looks hard (matching colors); it's in the
quiet bookkeeping rule that you only use each piece once. That rule has an entropy
cost that grows with area, and the board is just large enough for it to bite.

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

Deterministic. h(1) and h(2) and $\lambda_H$ are recomputed exactly; h(3), h(4)
and h_infinity are carried from the offline width-n sweep and labelled as such.

## Notes

- "Entropy density" here is the per-cell growth rate of the count of valid patches,
  in log base ten. Positive means the count grows exponentially with area.
- The area-law constant α and the ~80-cell collapse come from the distinctness
  sampling in the original study; they are stated here as the consequence, with the
  exactly-recomputed grammar entropy as the on-site reproducible part.
