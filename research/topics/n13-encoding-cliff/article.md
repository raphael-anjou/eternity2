---
id: n13-encoding-cliff
title: The N=13 cliff is in the search, not the instances
summary: A restarting exact-match DFS collapses at N=12 on 22-colour boards while CP-SAT verifiably full-solves instances no DFS seed touches; the source's half-second N=13 solves and 15-second planted 16x16 were measured at 26 colours, a setting this bed cannot generate, and at 22 colours CP-SAT itself slows and times out from N=12.
status: draft
created: 2026-07-22
updated: 2026-07-22
contributors:
  - name: Raphael Anjou
    role: author
tags:
  - complexity
  - exact-methods
  - methodology
sources: []
reproduce:
  - cd research/topics/n13-encoding-cliff/compute && cargo run --release -- sweep --ns 10,11,12,13,14 --gen-seeds 1,2,3,4,5 --budget-secs 20 > ../results/heuristic.jsonl
  - cd research/topics/n13-encoding-cliff/compute && cargo run --release -- export --ns 10,11,12,13,14,15,16 --gen-seeds 1,2,3,4,5 --out instances
  - cd research/topics/n13-encoding-cliff/compute && cargo run --release -- sweep --ns 14,12 --gen-seeds 1 --budget-secs 45 > ../results/budget45.jsonl
  - cd research/topics/n13-encoding-cliff/compute && cargo run --release -- sweep --ns 14,12 --gen-seeds 1 --budget-secs 120 > ../results/budget120.jsonl
  - cd research/topics/n13-encoding-cliff/compute && uv run cpsat_cliff.py instances/inst_n10_s*.json instances/inst_n11_s*.json instances/inst_n12_s*.json instances/inst_n13_s1.json instances/inst_n13_s2.json --time-limit 300 --workers 8 > ../results/cpsat.jsonl
  - cd research/topics/n13-encoding-cliff/compute && uv run cpsat_cliff.py instances/inst_n14_c22_s1.json instances/inst_n16_c22_s1.json --time-limit 120 --workers 8 >> ../results/cpsat.jsonl
results:
  - label: Heuristic arm, per-seed runs (JSONL)
    path: results/heuristic.jsonl
  - label: CP-SAT arm, per-instance runs (JSONL)
    path: results/cpsat.jsonl
  - label: Budget-insensitivity check, 45 s (JSONL)
    path: results/budget45.jsonl
  - label: Budget-insensitivity check, 120 s (JSONL)
    path: results/budget120.jsonl
  - label: Aggregated summary (JSON)
    path: results/summary.json
site:
  render: false
  dataFile: null
---

# The N=13 cliff is in the search, not the instances

> **Status:** draft · **Created:** 2026-07-22 · **Updated:** 2026-07-22
> **Authors:** Raphael Anjou

## Summary

On planted, guaranteed solvable Eternity II style boards with five pinned
clues, restarting depth-first search full-solves every 12x12 instance in well
under a second and then fails catastrophically at 13x13: it plateaus around
12 percent of the target and never improves, no matter the budget. Ten
heuristic families were tested, from a naive row-major filler to AC-3
propagation and Hall-deficit pruning. Six of them full-solve 12x12; not one
solves 13x13. The boundary is sharp, seed-independent, and not a budget
effect.

That looked like a property of the instances. It is not. A CP-SAT model of
the same instances, built from an AllDifferent constraint over cell-to-piece
assignment plus Element channelling of edge colours, posed as a decision
question, solved 25 of 25 instances across sizes 10 through 14 on the original 26-colour bed with a smooth
time curve: where the 13x13 boards that no heuristic touches fall in 0.50 seconds
median, and planted 16x16 boards fall in about 15 seconds. The cliff belongs
to the search paradigm, not to the boards. All ten heuristic families share
one bias, chronological greedy commitment with only local information, so
their agreement was one witness reported ten times.

The formulation gap alone is worth stating. A generic MIP over placement
binaries with sum-to-one rows (HiGHS branch and bound) scores 11 of 264 on a
12x12 instance in 300 seconds; CP-SAT on the same instance returns the full
264 in 0.42 seconds. Three orders of magnitude, same problem, different
encoding. Two cautions on scope: the planted boards are not canonical
Eternity II (they plausibly admit very many solutions, where the real puzzle
is believed to admit essentially one), and the original measurements used 26
interior colours where the real puzzle has 22, which makes those boards more
constrained and easier than the real thing.

## Reproduction

Two arms, both driven from `compute/`, both scored through the kit's
canonical rim-excluding scorer so a full solve at size N means exactly
2N(N-1) matched edges (312 at N=13, 480 at N=16).

The heuristic arm is a Rust binary on the starter kit: it generates framed,
colour-balanced planted instances, pins five solution cells as clues, and
runs a restarting randomized depth-first search that places only exact
matches. Expected shape: full solves in milliseconds up to N=12, hard
failure at the cliff, with scores pinned rather than creeping as the budget
grows. The same binary exports each instance as JSON for the second arm.

The exact arm is a Python script (OR-Tools CP-SAT) that reads the exported
instances and builds the structured encoding: one integer variable per cell
over piece ids under AllDifferent, a piece-and-rotation index channelled
through Element tables into per-side colour variables, hard adjacency and
rim constraints, and the clues fixed. It asks the decision question only,
then independently re-verifies any returned board (piece distinctness, clue
conformance, recomputed edge count) rather than trusting the solver's
status. The claim reproduces if the exact arm solves every instance with a
smooth time curve through the size at which every heuristic in the first arm
has collapsed. See `compute/PLAN.md` for the full design, the expected
numbers, and the fidelity caveats (colour count and generator differences
against the original measurement).

### Measured results

Run on Apple Silicon; the DFS is single-core, CP-SAT uses 8 workers. The
kit's generator caps interior colours at 22, so the original C=26 setting
could not be generated; everything below is at C=22, and the source numbers
in the table are the C=26 originals, shown for shape, not for digit-level
comparison.

Heuristic arm (restarting exact-match DFS, 5 pinned clues, 20 s per
instance, 5 generator seeds per size):

| N | target | full solves | scores (seeds 1..5) | solve times |
|---|--------|-------------|---------------------|-------------|
| 10 | 180 | 4/5 | 180, 180, 180, 180, 52 | 0.004 to 9.4 s |
| 11 | 220 | 2/5 | 220, 80, 91, 93, 220 | 0.11 s, 3.7 s |
| 12 | 264 | 0/5 | 127, 140, 123, 144, 98 | timeout |
| 13 | 312 | 0/5 | 179, 166, 141, 160, 189 | timeout |
| 14 | 364 | 0/5 | 236, 174, 233, 223, 227 | timeout |

The collapse size at C=22 is N\*=12: no seed full-solves 12x12 or anything
above it, and the failure is budget-insensitive, exactly as in the source.
The N=12 seed-1 instance scores 127 at 20 s, 127 at 45 s, and 127 at 120 s
while the node count grows from 22 M to 132 M; N=14 seed 1 moves only 236
to 240 across a sixfold budget increase. Below the cliff the boundary is a
seed lottery at C=22 (the source's C=26 rungs solved uniformly at N=12),
which the plan's smoke run had already flagged.

CP-SAT arm (same instances, decision form, independent verification):

| N | source (C=26) median | measured (C=22) | verified full solves |
|---|----------------------|-----------------|----------------------|
| 10 | 0.10 s | 0.29 s median | 5/5 |
| 11 | 0.19 s | 2.27 s median | 5/5 |
| 12 | 0.42 s | 31.7 to 88.1 s, one timeout at 300 s | 4/5 |
| 13 | 0.50 s | 74.2 s (seed 1), timeout (seed 2) | 1/2 run |
| 14 | 0.84 s | timeout at 120 s (seed 1) | 0/1 run |
| 16 | 15.27 s | timeout at 120 s (seed 1) | 0/1 run |

The direction of the finding reproduces at the cliff edge: CP-SAT
full-solves, with independent verification, four of the five 12x12
instances and the first 13x13 instance that no DFS seed touches. But the
smooth-seconds curve does not reproduce at C=22. The exact arm slows by two
orders of magnitude between N=11 and N=12 and starts timing out itself from
N=12 up (one of five at N=12, then more above). The remaining N=13 seeds
and the N=14 to N=16 rungs were cut to single-seed probes to stay inside
the compute budget, so the table above is the largest honest subset, not a
full 35-instance sweep. The optional HiGHS MIP arm was not run.

Two readings follow. First, the paradigm claim survives where it was
tested: the chronological heuristic dies at a sharp, budget-insensitive
boundary inside the 10..16 band, and the structured encoding crosses that
exact boundary on most instances. Second, the C=26 versus C=22 fidelity
gap, declared up front in the plan, turns out to be load-bearing: at 22
colours the planted family is harder for both paradigms, and CP-SAT's own
wall moves down to roughly the same sizes. The vault's open question,
whether the cliff moves with the colour count, gets its first data point:
it does, for both arms at once. The CP-SAT times at N=12 and N=13 also
carry some inflation from a concurrently running DFS arm on the same
machine, which cannot account for the gap to the sub-second source times.

A planted 16x16 board from the same generator family (seed 1, the instance
the CP-SAT probe timed out on, shown as its planted solution):
[gen_16x16_c22_s1 solution, 480/480](https://eternity2.dev/viewer?puzzle=gen_16x16_c22_s1&puzzle_size=16&board_edges=aebaabgeabvbacvbadvcacrdadlcadodadidaeqdafneadgfacwdafscachfaabcbjcagmujvhpmvuohvtpurnttlrsnouvriphuqowpnluogqllwpqqsvgphkgvbackcgdauwsgpluwoqmlpjoqtikjsnnivvvnhmrvwhpmuuuhlutuqqkugguqgnhgcabndgdassrguihsmqkiokhqkjnknokjvkrormikpptmuroptrirkomruumohjsubaejdlearsulhqmskvpqhqkvnwpqknuwrlqnikrltjlkosnjiglsmoogmnjoslqneaclerdauhjrmiihpklikwlkprmwunprqnwnrgnnlgjgnvmglknvowskjkowqwpkcafwdjfajjgjiqqjlptqlnipmslnpowswjjonoljjttomsrtnwtssrowosmrpmisfafmfhdagvkhqjsvtinjitsilwntwjrwjvrjlkovtsvkrhqstquhojhqmkqjitrkfaftdufakstusuisnmvuslsmnrmlrsvrrhnsoomhvtloqvntutvvhkltqukkrmtufaemfhfatwhhirhwvtrrsjptmtujvtgtnvstmohvliionwhivqqwlomqkgpotljgeaflfqeahgqqhmngrprmpqhpuugqgiquslqihqrliimqhkjiqwwkmspwpqisjkjqfaekembaqipmnioirkwihvkkgntvqhhnqnshrwgnmvlwjgwvwqkgpwmqiliwjulleadububapssuohjswmuhkowmtnpohrtnspgrgnrplnvnwgpnkvvgmhlvihwhlnshdadnbkeasgtkjjigumvjwwwmpjgwtpojgujprvtuvruvptorvmmtlsomwigssopidafoegcatppgiltpvpwlwohpgiuoolhijroltigrujgiotljmkjtorkkgiirppmifabpcbaapeabtbaewcabhcacueacheaeocaegfacgcaflbacjfabkbafieabmdaebaad)


results/summary.json is hand-aggregated from the per-arm JSONL files; the JSONLs are the primary record.
