---
# Metadata read by build-index.mjs. id MUST equal this topic's directory name.
id: ledger-prune
title: "LEDGER: a sound global colour-ledger prune for break-budget search"
summary: A sound per-colour supply/demand ledger prunes a break-budget DFS with zero unsound fires (replayed at zero slack over 8 boards x 257 depths); on generated boards this phase measures node ratios of 1.5x to 5x that grow with the budget, bounding from below the 140x to 4330x the source measured on a community 464 board's break-carrying tail (phase 2).
status: draft
created: 2026-07-22
updated: 2026-07-22
contributors:
  - name: Raphael Anjou
    role: author
tags:
  - search
  - pruning
  - structure
sources:
  - label: "Prune beats speed (this site): why branching-factor cuts dominate raw speed"
    url: /research/prune-vs-speed
reproduce:
  - cd research/topics/ledger-prune/compute && cargo run --release -- --mode sweep --seeds 8 --cap-secs 25 --out ../results
  - cd research/topics/ledger-prune/compute && cargo run --release -- --mode zerodiag --seeds 8 --cap-secs 25 --out ../results
results:
  - label: Soundness gate (8 boards, zero-slack replay, 0 fires)
    path: results/soundness.json
  - label: A/B node-count grid, suffix {20,24,28,32} x budget {1,2,3} x 8 seeds
    path: results/ab-sweep.json
  - label: Vault-row-shaped diagonal (24,2) (32,5) (40,6) (48,8)
    path: results/ab-diagonal.json
  - label: Zero-slack diagonal, budget 0 at suffix {24,32,40,48}
    path: results/ab-zeroslack.json
  - label: Wrong-regime probe (generous budget, fire fraction)
    path: results/wrong-regime-null.json
site:
  render: false
  dataFile: null
---

# LEDGER: a sound global colour-ledger prune for break-budget search

> **Status:** draft · **Created:** 2026-07-22 · **Updated:** 2026-07-22
> **Authors:** Raphael Anjou

## Summary

High-score search on Eternity II is usually run with a break allowance: the
DFS may place a piece that mismatches a neighbour, as long as the total number
of charged mismatches stays within a budget. Local look-ahead pruning is
unsound in this setting, because the break budget is a global resource; a
branch that looks locally dead can be rescued by spending a break somewhere
else entirely. LEDGER replaces the local judgment with a global counting
argument. At every node it maintains, per colour, the supply of half-edges
over all unused pieces and the demand from the exposed frontier plus the grey
rim. Two necessary conditions follow: the total colour deficit, summed as
max(0, demand minus supply) over colours, can never exceed the remaining break
budget, and the number of colours whose supply/demand difference is odd can
never exceed twice the remaining budget plus the number of uncharged hint
junctions. If either fails, no completion within budget exists below the node
and the whole subtree is skipped.

Both conditions are proved sound by an invariance argument: a fully matched
placement changes every per-colour balance by an even amount, so parity moves
only at charged breaks (two colour flips each) and at uncharged hint junctions
(at most one flip). The deficit condition is the budget-aware form of the
classic "ran out of colour c" failure that plain DFS discovers cell by cell.

Measured on suffix-exhaustion races over a fixed 232 to 208 cell prefix of a
high-score board, with the real tail's break count as the budget, the prune
reduced node counts by 1.5x at a 24 cell suffix, 140x at 32 cells, 995x at 40
cells and 4330x at 48 cells. The multiplier compounds with depth, which is the
signature of a branching-factor reduction rather than a constant-factor
speedup. A soundness gate passed on four independent high-score boards
replayed under zero-slack budgets: zero prune fires over 251 judged cells per
board. The prune is honest about its regime: with a large remaining budget it
fires on roughly 0.1 percent of nodes and its bookkeeping is a pure loss; its
value is confined to the small-budget deep-suffix regime, which is exactly
where record searches spend their time.

## Reproduction

The `compute/` crate contains a self-contained break-budget DFS with the
LEDGER conditions implemented from the definitions above, on top of the
starter kit's canonical instance, scorer and generator (per-edge charged
breaks, no per-cell cap, proper frame, hint-free so u = 0). Phase 1 ran on
generated official-shaped 16x16/22 instances with known solutions, seeds 1
through 8, 25 s per-arm cap; hardware Apple Silicon, single core. Node counts
are seed-deterministic; a full re-run reproduces the committed JSON
byte-for-byte (checked), and any arm that hits the cap is recorded as
censored and excluded from ratios. Every instance's solution board ships as
an eternity2.dev URL in `results/soundness.json`.

**Verdict: partial.** The soundness core reproduces exactly; the headline
magnitudes (140x to 4330x) are tied to a community record board this phase
cannot access, and the generated-board proxies bound them from below without
reaching them.

| Claim (expected, vault) | Measured here | Status |
|---|---|---|
| Soundness gate: 0 fires replaying real tails at zero slack (4 boards, 251 cells each) | 0 fires on 8 boards x 257 depths | reproduced |
| Prune never changes the answer | completions identical in all 144 uncensored A/B pairs | reproduced |
| Node ratio > 1 under small budgets | all 96 grid pairs > 1 (min 1.48x, medians 1.8x to 4.1x) | reproduced |
| Ratio compounds with suffix depth (1.5x, 140x, 995x, 4330x at 24/32/40/48 cells) | diagonal medians 1.9x at (24,2), 5.0x at (32,5); (40,6) and (48,8) censored at 25 s | untested at magnitude, see below |
| Parity fires dominate deficit fires (5-10:1 small-r) | parity is the dominant trigger everywhere (non-exclusive tallies 1.1-2.5:1, e.g. 98,213 vs 48,363 at K=32 R=3) | direction reproduced, accounting convention differs |
| Wrong-regime null: ~0.1 percent fires with a big budget | probe (6 spare breaks on a 20-cell tail) still fires on 76-82 percent of nodes | untested, proxy stayed in the active regime |

Grid medians of the node ratio NoPrune/LEDGER (8 seeds each, no censoring):

| suffix \ budget | R=1 | R=2 | R=3 |
|---|---|---|---|
| K=20 | 2.01x | 2.06x | 4.07x |
| K=24 | 1.97x | 1.91x | 3.81x |
| K=28 | 1.90x | 1.77x | 3.28x |
| K=32 | 2.13x | 1.82x | 3.21x |

The reduction grows with the budget, not with the suffix depth at fixed small
budget: more slack means deeper subtrees for the global counting argument to
amputate. The vault's compounding rows grew suffix and budget together on a
real record board, and phase 1 cannot enter that regime from either side. On
a perfect generated tail the zero-slack analogue (budget 0) is degenerate:
the tail is nearly forced (24 to 102 nodes for 24 to 48 cells), both arms
coincide and the ratio is exactly 1.0. With the vault's own (suffix, budget)
pairs the slack is real and the ratio reaches only 5.0x median at (32,5)
before both arms blow past the 25 s cap. The 140x to 4330x certificates live
on the 464 board's break-carrying tail, where zero slack still leaves an
enormous space; reproducing them needs that board (phase 2 in
`compute/PLAN.md`, blocked on community board data in the kit).

The wrong-regime probe fell on the instructive side of the null: 6 spare
breaks on a 20-cell perfect tail sounds generous but depletes near the
leaves, where most nodes live, so the ledger stayed active (and useful)
instead of going quiet. The vault's 0.1 percent figure describes a full-board
search whose budget is large relative to every remaining tail; a 20-cell
suffix cannot be configured into that regime with an exhaustible budget.

See `compute/PLAN.md` for the exact claim, the scoring-convention mapping,
the scale-faithfulness argument, and the gap between this reproduction and
the original certificate runs on community record boards.
