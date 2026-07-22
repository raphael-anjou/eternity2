---
# Metadata read by build-index.mjs. id MUST equal this topic's directory name.
id: ledger-prune
title: "LEDGER: a sound global colour-ledger prune for break-budget search"
summary: A per-colour supply/demand ledger with a deficit bound and a parity bound against the total remaining break budget prunes break-allowance DFS soundly, with measured node reductions of 140x to 4330x that compound with suffix depth.
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
  - cd research/topics/ledger-prune/compute && cargo run --release -- --mode soundness
  - cd research/topics/ledger-prune/compute && cargo run --release -- --mode ab
results: []
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
starter kit's canonical instance, scorer and generator. Two modes:

- `--mode soundness` replays a generated board's known perfect tail under a
  zero-slack budget at every prefix depth and asserts the ledger never fires
  along the real line (any fire would be unsound).
- `--mode ab` runs the same suffix exhaustion twice, prune off and prune on,
  over a fixed solution prefix of a generated 16x16 instance, and reports the
  node counts and their ratio at increasing suffix depths.

See `compute/PLAN.md` for the exact claim, the scoring-convention mapping,
the scale-faithfulness argument, and the gap between this reproduction and
the original certificate runs on community record boards.
