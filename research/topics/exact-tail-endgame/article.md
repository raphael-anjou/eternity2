---
id: exact-tail-endgame
title: The bottom-band endgame as its own exact problem
summary: PLACEHOLDER
status: draft
created: 2026-07-23
updated: 2026-07-23
contributors:
  - name: Raphael Anjou
    role: author
credits:
  - Vol-228 TAILFORGE and vol-234 R3 comb-squeeze research notes, which posed and measured the exact-tail endgame question.
tags:
  - exact-methods
  - constraint-programming
  - endgame
  - finishability
sources: []
reproduce:
  - cd research/topics/exact-tail-endgame/compute && uv run tailforge.py --seeds 24 --rows 1,2 --caps 1,2 --band-seeds 1:24,2:4 --time-limit 45 --workers 8 --out ../results/tailforge.json
results:
  - label: Exact-tail vs producer-tail, per (seed, band, cap) with delta distribution (JSON)
    path: results/tailforge.json
site:
  render: false
  dataFile: null
---

# The bottom-band endgame as its own exact problem

> **Status:** draft · **Created:** 2026-07-23 · **Updated:** 2026-07-23
> **Authors:** Raphael Anjou

## Summary

PLACEHOLDER

## Reproduction

PLACEHOLDER

### Measured results

PLACEHOLDER
