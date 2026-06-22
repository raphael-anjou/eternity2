---
id: record-boards
title: Record boards, verified
summary: The high-scoring boards from this project's algorithms, with their matched-edge scores recomputed from the board itself.
status: published
created: 2026-06-22
updated: 2026-06-22
contributors:
  - name: Raphaël Anjou
    role: author
tags:
  - records
  - verification
sources:
  - label: "Eternity II board viewer (e2.bucas.name)"
    url: https://e2.bucas.name
reproduce:
  - cd research/topics/record-boards/compute && cargo run --release > ../results/verified-scores.json
results:
  - label: Verified scores (JSON, computed here)
    path: results/verified-scores.json
site:
  render: true
  dataFile: results/verified-scores.json
---

# Record boards, verified

> **Created:** 2026-06-22 · **Updated:** 2026-06-22

## Summary

The algorithms in the lab notebook produced a handful of high-scoring boards. The
searches that found them lean on randomness, so they won't repeat the exact same
board run to run. The boards themselves, though, are fixed and checkable. This
topic bundles each one and recomputes its matched-edge score straight from the
board, so the number on its page is verified, not just asserted.

## Method

Each board is stored as its e2.bucas.name parameters, the same format the viewer
reads. Those include a string of four letters per cell giving the colors on its
four sides. The matched-edge score is simply the number of internal board edges
where the two touching sides carry the same color. We recompute it from the
letters and compare it to the score we claim. You can do the same by hand, or just
open the board in the viewer.

## Results

| Board | Algorithm | Claimed | Recomputed |
|---|---|---:|---:|
| PALIMPSEST | basin-targeted search | 463 | 463 |
| KEYRING | corpus-guided construction | 460 | 460 |
| GAUNTLET | multi-direction beam | 458 | 458 |

For reference, a complete board scores 480. The best of these, 463, is the best
matched-edge board this project has produced; the community's best on the same
puzzle is 469.

## Reproduce

```sh
cd research/topics/record-boards/compute
cargo run --release > ../results/verified-scores.json
```

Deterministic; reproduces the committed `results/verified-scores.json`
byte-for-byte. Each board can also be opened directly in the viewer from its
invention page.

## Notes

- This verifies the *board*, not the *search*. The searches that produced these
  boards are stochastic; their write-ups say so and link the board here so the
  result stands on its own.
- "Matched-edge" is the scoring used on the community leaderboard. Some of these
  boards do not obey all five official clue positions; where that matters, the
  invention's page says so.
