---
description: Scientific-rigor check of a research page before publication. Null models, title vs body, current hardware and literature, numbers against committed results, and cross-page consistency.
argument-hint: "[page path, e.g. web/content/research/why/entropy-area-law.mdx, or empty for the current diff]"
---

# Rigor check

Check the research page(s) in `$ARGUMENTS` (or every research page touched by
the current diff) against the questions below. The site's earlier self-audits
checked internal consistency and polish and missed most of these, so treat
each question as a separate pass. Report findings as
`file:lines | severity | claim | problem | fix`, verify each one against the
data before editing, and fix EN, FR and ES together.

## 1. Baseline (null model)

- Would a random puzzle of the same size and colour counts show the same
  property? Anything painted from a solved board has exact colour-side parity
  and zero total slack automatically; that is not evidence of design.
- If a property is presented as "extreme", "designed", "no slack", or a
  "law", the page must show the same statistic on random instances
  (`generate_solved` / `generate_solved_framed`) or say it has not been
  compared.
- A decay, threshold or scaling law fitted to two or three points is a
  conjecture. Say how many points and do not extrapolate beyond them.

## 2. Title, description and tier against the body

- The title, `description`, callouts and "finding" headline may not claim more
  than the body measures. One engine is not "solvers"; one size is not "in
  general"; one producer's ceiling is not "the puzzle's".
- A plateau one method family reaches is a property of that method unless
  other methods are shown to stop there too. Community boards already above it
  (see `/research/records`) refute "a property of the puzzle".
- Borrowed physics vocabulary (area law, phase transition, entropy, glass)
  must match its technical meaning or be defined as an analogy.

## 3. Exact search versus record hunting

- Constant-factor speedups cannot make the full 16x16 exhaustive search
  feasible. In stochastic record hunting more throughput means more attempts
  and can raise the best score. Never let the first argument stand in for the
  second.

## 4. Currency

- Hardware figures (memory per core, cache sizes, GPU or CPU throughput) must
  carry their year. A 2011 argument is history, not a standing law. Check the
  newest archive messages and the benchmarks page for later measurements.
- Literature: search for work after the newest paper the page cites
  (Crossref, arXiv, Google Scholar) and either cite it or say the page covers
  up to a given year.
- Throughput comparisons need the same puzzle, the same counting convention,
  and the same unit (per socket, per watt or per dollar; not a whole GPU
  against one CPU core).

## 5. Numbers and sources

- Every number matches `research/topics/*/results` or the cited message. Recompute
  derived arithmetic (ratios, exponents, rack-years).
- Quantities with several bases on the site (tree width, nodes to first
  solution, total work) state which one they use. Expected solution count:
  about 14,702 with the starter clue and about one with all five, a
  first-moment ensemble estimate (canonical page:
  `/research/build/analysis/solution-counting`).
- `repro.cmd` regenerates the headline numbers, not just part of them.

## 6. Cross-page consistency

- `grep` the site for the same claim. If another page states a newer or
  contradicting figure, reconcile both or link one canonical page.
- Current records are linked to `/research/records`, never restated.

For a deeper multi-reviewer pass use `/adversarial-review`.
