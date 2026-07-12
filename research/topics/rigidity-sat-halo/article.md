---
id: rigidity-sat-halo
title: The rigidity wall, checked with a SAT solver
summary: A second, independent way to show record boards are locally frozen. Free the cells around each mismatch, ask a SAT solver whether those pieces can be rearranged so every local edge matches, and get UNSAT on every record board tested.
status: published
created: 2026-07-11
updated: 2026-07-11
contributors:
  - name: William Millilaw
    role: independent SAT-residual and freeze-test result
  - name: Raphaël Anjou
    role: reproduction and write-up
tags:
  - structure
  - local-search
  - exact-methods
sources:
  - label: "Eternity II board viewer (e2.bucas.name)"
    url: https://e2.bucas.name
  - label: "kissat SAT solver (Biere et al.)"
    url: https://github.com/arminbiere/kissat
reproduce:
  - cd research/topics/rigidity-sat-halo/compute && cargo run --release -- --selftest
  - cd research/topics/rigidity-sat-halo && ./run.sh
results:
  - label: Halo-SAT verdicts (JSON, computed here)
    path: results/halo-sat.json
site:
  render: true
  dataFile: results/halo-sat.json
---

# The rigidity wall, checked with a SAT solver

The [rigidity wall](/research/why/rigidity-wall) shows that record boards are
frozen in place: free a region around the last few mismatches, search for the
best legal refill, and the best arrangement is the one already there. That page
proves it with integer programming. This page asks the same question with a
different tool, a Boolean SAT solver, and gets the same answer.

The two methods are worth having side by side. Integer programming *optimizes*:
it finds the best refill and bounds how good any refill could be. A SAT solver
*decides*: it answers a yes-or-no question and, when the answer is no, that no
is a proof. Here the question is sharp. Take a record board, free every cell
within a small radius of a mismatched edge, and ask: can these freed pieces be
placed back, in any rotation, so that every edge inside the freed region
matches? A *yes* would be a strictly better board. A *no* is a certificate that
the board cannot be improved by rearranging that local patch of its own pieces.

## Where this came from

The approach is William Millilaw's. Working independently, he attacked the same
ceiling two ways: a replica freeze test, which found that the roots of his best
boards return to the same cells 93 to 100 percent of the time when perturbed and
re-optimized, and a halo SAT-residual test, which freed the cells around the
mismatches of a record board and asked a SAT solver whether the gap could be
closed. His solver returned UNSAT. We reproduce that halo test here on public
boards, with our own encoder, so the result can be checked end to end.

## What we ran

For each board we recover its pieces straight from the published edge string,
confirm the matched-edge score equals the claimed record, mark every internal
edge whose two sides disagree, and free every cell within Chebyshev radius $R$
of such an edge. The freed cells keep exactly their own pieces: nothing enters
or leaves the region, so this is a strictly *local* rearrangement. We then write
a SAT instance whose variables place one piece and rotation per freed cell, with
clauses for one piece per cell, each freed piece used once, the border color
staying on the rim, and every freed edge matching its neighbor. We hand that to
kissat.

## The result

Every record board tested is UNSAT: no local rearrangement of its own pieces
closes any of its mismatches. The mismatch count is exactly $480 -$ score, as it
must be, and the freed region grows with the radius.

The full table is computed live from `results/halo-sat.json`.

## Guardrails

A reproduction that always prints UNSAT proves nothing, so the encoder ships
with a positive control. On a fully matched region the original arrangement is
itself a valid refill, so the instance must be satisfiable; our `--selftest`
builds that instance, proves in code that the known solution satisfies every
clause, and the solver returns SAT. The score of each board is recomputed from
its edges before anything else runs. Only after those checks pass do the UNSAT
verdicts mean what they say.

## What this does and does not show

It shows that these specific record boards sit at the bottom of their own local
valley: you cannot improve them by shuffling a halo of their own pieces. That is
the rigidity wall, reached from the decision side instead of the optimization
side, and it matches both the integer-programming proofs and the annealing
evidence on the [main page](/research/why/rigidity-wall). It does not show that
480 is unreachable. The halo is local by construction; a solution, if one
exists, requires rearranging a large region at once or starting somewhere else
entirely.
