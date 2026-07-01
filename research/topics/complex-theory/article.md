---
id: complex-theory
title: Complex theory, exactly as published
summary: The site's live search-width estimator is a validated byte-faithful port of Peter McGavin's reference C implementation of Brendan Owen's complex theory.
status: published
created: 2026-06-30
updated: 2026-07-02
contributors:
  - name: Brendan Owen
    role: theory
  - name: Peter McGavin
    role: reference implementation
  - name: Raphaël Anjou
    role: port
tags:
  - structure
  - complexity
sources:
  - label: "Peter McGavin's post with the reference C source and published depth table (groups.io message 11197)"
    url: https://groups.io/g/eternity2/message/11197
  - label: "Brendan Owen's original complex-theory posts (groups.io message 5209)"
    url: https://groups.io/g/eternity2/message/5209
reproduce:
  - "compare web/src/engine-ts/complexity.ts against reference/complex_theory.c — the port is line-faithful and validated against McGavin's published depth table (14,702 expected solutions with the centre hint)"
results:
  - label: McGavin's reference C source (provenance copy, (c) Peter McGavin 2024)
    path: reference/complex_theory.c
---

# Complex theory, exactly as published

This topic holds the provenance for the site's live complex-theory estimator
(`web/src/engine-ts/complexity.ts`): Brendan Owen's theory, in Peter McGavin's
reference C implementation, ported line-faithfully to TypeScript and validated
against the depth table McGavin published alongside the source (notably the
expected ~14,702 full solutions for the official puzzle with the centre hint
placed).

There is no `compute/` crate here: the "result" is the estimator itself, which
runs live in the browser on `/research/why/complex-theory`. The reference
source is kept under `reference/` for provenance; see `README.md` for the full
derivation notes.
