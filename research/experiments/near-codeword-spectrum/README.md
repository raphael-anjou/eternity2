# Near-codeword spectrum: fewest breaks at each overlap with a planted solution

Pre-registered round-2 test. Binding statement: [PREREG.md](PREREG.md)
(commit bf2fecb, pushed before any run), with amendments
[PREREG_AMENDMENTS.md](PREREG_AMENDMENTS.md) (A1 to A4 made before any
reported-seed solve; A5 records the early stop). **The run is partial:**
stage 1 and the curves finished, stage 2 stopped after 6 of 80 solves, and
stage 3 was not run (A5).

## What was measured

On `generate_solved_framed(n, C, seed, true)` boards, with the planted corner
piece pinned at cell 0, CP-SAT minimises internal breaks over full gray-legal
boards whose colour-tuple distance a from the planted board lies in a band:
near [1, ceil(N/4)-1], mid [ceil(N/4), floor(3N/4)], far [floor(3N/4)+1,
N-1], K2 band [ceil(N/3), floor(3N/4)] (N = n^2). H_band = band minimum,
barrier B = H_mid - H_far. Every witness board was rechecked in Python
(legality, permutation, breaks, distance). Values are [LB, UB] intervals when
the solve did not close.

## Commands

```
just experiments near-codeword-spectrum     # = near-codeword-spectrum/scripts/run_all.sh
```

The script builds the Rust crate (instances, the annealed baseline, the
stage-3 samplers), creates `.venv` with `requirements.txt` (OR-Tools
9.15.6755, SciPy 1.18.1, NumPy 2.5.4) via uv, and runs
`scripts/cpsat_spectrum.py --stage {stage1,curve,stage2}` (resumable), then
stage 3 and `scripts/analyze.py`. To re-evaluate the committed raw data only:
`.venv/bin/python scripts/analyze.py`. Rust checks: `cargo test --release`
(5 tests, including an exact brute-force check of the annealed DP on 4x4) and
`cargo clippy --release --all-targets` (clean).

Hardware and wall clock: VM, 8 vCPU Intel Xeon, 31 GB RAM, no AWS. Run
13:17 to 14:55 UTC on 2026-11-11: stage 1 2519 s, curves 3001 s, stage 2
about 6 min before the stop (about 1.6 h wall, about 13 core-hours). CP-SAT
witnesses and bounds of time-limited solves are not bit-for-bit
reproducible; closed optima are.

## Results (stage 1, all 30 + 10 instances per size)

Median [LB, UB] over instances; "closed" = solves at OPTIMAL.

| palette | near | mid | far | K2 band | median B | closed mid / K2 |
|---|---|---|---|---|---|---|
| 5x5 C=13 (primary) | 2 | 4 | 2 | 5 | 1.5 | 30/30, 30/30 |
| 6x6 C=14 (primary) | 2 | [4, 4.5] | 2 | [5, 7] | 2 | 23/30, 13/30 |
| 5x5 C=9 (contrast) | 2 | 2 | 2 | 2 | 0 | 10/10, 10/10 |
| 6x6 C=10 (contrast) | 2 | 2 | 2 | 2 | 0 | 10/10, 10/10 |

- Share of instances with mid certified strictly above both near and far:
  5x5 C=13 0.77, 6x6 C=14 0.83, contrasts 0.10.
- Annealed baseline b_ann (exact first-moment DP; p_f, p_i are
  seed-independent per palette as the prereg assumed): 5x5 C=13 near 2, mid 5,
  far 7; 6x6 C=14 near 2, mid 5, far 8, K2 6.
- Curves (A2, seeds 1..10): closed for every instance up to a = 7 (5x5) and
  a = 6 (6x6); medians 2, 2, 3.5, 2, 4, 4, 4 (5x5) and 2, 2, 3, 3, 4, 4
  (6x6). Most 6x6 points with a >= 9 do not close in 30 s (median LB 0), so
  the 6x6 curve UBs past a = 8 are weak. Median near radius r(b) at
  b = H_mid - 1: 4 cells at 5x5 (all 10 certain), 7 at 6x6 (5 of 10
  certain). Band-versus-curve consistency checks: 18, 0 failures.
- Stage 2 (partial, 7x7 C=15): near = 2 on seeds 1, 2, 3; seed 1 mid
  [3, 15], far 2 (closed, 16 s); seed 2 mid [3, 15].
- Zero rotation-duplicate pieces over all 140 generated instances.

## Diagnostic not in the prereg: the far floor is the board's own symmetry

For every band witness we also measured the distance to the planted board
turned by 1, 2 or 3 quarter turns (ignoring the pin). At the primary
palettes, 29 of 30 (5x5) and 28 of 30 (6x6) far-band optima lie within 4
cells of a turned copy of the planted solution (median 2 cells), and so
does the one 7x7 far optimum. The near, mid and K2 witnesses are never that
close (median 20 to 36). At the contrast palettes only 2 of 10 and 1 of 10
are. So at mu-matched palettes the pin does not remove the symmetry from the
spectrum: H_far = 2 is mostly "a turned copy of the same solution with the
pinned corner swapped in", not an orthogonal solution. B therefore measures
the mid band against the symmetric image of the planted solution. Read with
the kill criteria below, this means: a barrier exists between two copies of
the same solution at 5x5 and 6x6; whether a gap separates distinct
solutions is not measured here. (`witness_symmetry_diagnostic` in
`results/summary.json`.)

## Kill criteria (self-assessment; a separate judge decides)

| criterion | numbers | verdict |
|---|---|---|
| K1a, 5x5 | median B = 1.5 (all exact) | decided, does not fire |
| K1a, 6x6 | median B = 2 at both interval ends | decided, does not fire |
| K1b | Wilcoxon one-sided p = 0.021 (unfavourable end), 0.00068 (favourable end); mean B 1.37 -> 2.77 (UB side); secondary Mann-Whitney p = 0.003 | decided, does not fire |
| K1c, 7x7 | 1 of 10 instances usable (seed 1: mid UB 15 > far 2) | undetermined (A5) |
| K1c, 8x8 | 0 of 10 | undetermined (A5) |
| K2 | median m2 over 30 6x6 instances in [5, 7] > 2 | decided, does not fire |
| K3 | b_ann(mid) = 5; median (H_mid - b_ann) in [-1, -0.5] | decided, does not fire |

**Self-assessed outcome: inconclusive.** None of K1a, K1b, K2 or K3 fires,
and K1c is undetermined because stage 2 was stopped (A5). Even with K1c
complete, the symmetry diagnostic above limits what a survival would mean:
the measured far floor at these palettes is the turned copy of the planted
solution.

## Files

- `src/lib.rs`, `src/bin/ncs.rs`: instances, annealed DP, stage-3 samplers.
- `scripts/cpsat_spectrum.py`: CP-SAT model and job schedule.
- `scripts/analyze.py`: kill criteria and summary.
- `results/instances.jsonl`: every reported instance (pieces, p_f, p_i, mu).
- `results/raw/{stage1,curve,stage2,annealed}.jsonl`: per-solve records
  (status, UB, LB, wall time, witness board as piece*4+rotation).
- `results/raw/wall.txt`, `results/summary.json`.
