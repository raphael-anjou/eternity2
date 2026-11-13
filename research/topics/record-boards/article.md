---
id: record-boards
title: Record boards, verified
summary: The high-scoring boards from this project's algorithms, plus the public five-clue 466, with their matched-edge scores recomputed from the board itself.
status: published
created: 2026-06-22
updated: 2026-11-13
contributors:
  - name: Raphaël Anjou
    role: author
tags:
  - records
  - verification
sources:
  - label: "Eternity II board viewer (e2.bucas.name)"
    url: https://e2.bucas.name
  - label: "Eternity@Home public board library: board FiveClue_466_2c037e70f7e9"
    url: https://stats.eternityathome.org/library/board/2c037e70f7e93518a48733c7aacd096226b7f23728efabd6289088b91c0945cd
  - label: "Eternity@Home board library, list of 466 boards with dates and discoverers"
    url: https://stats.eternityathome.org/library/static/s/466/0.json
reproduce:
  - cd research/topics/record-boards/compute && cargo run --release > ../results/verified-scores.json
  - just research-record-boards-pieces
results:
  - label: Verified scores (JSON, computed here)
    path: results/verified-scores.json
  - label: Community boards verified from pieces and rotations (JSON, computed here)
    path: results/community-verified.json
site:
  render: true
  dataFile: results/verified-scores.json
---

# Record boards, verified

> **Created:** 2026-06-22 · **Updated:** 2026-11-13

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
matched-edge board this project has produced. The community's records on the same
puzzle are higher; they are kept in one place, the records page
(`/research/records`).

## A community five-clue board, stored and verified

The strict five-clue record board `FiveClue_466_2c037e70f7e9` is stored in
[`community/fiveclue-466-2c037e70f7e9.json`](community/fiveclue-466-2c037e70f7e9.json)
as 256 pairs of official piece number and clockwise rotation, with its
e2.bucas.name edge string.

- **Source.** The Eternity@Home public board library
  (`stats.eternityathome.org/library`), retrieved 2026-11-13. The library shows it
  as "Discovered by Jef" with the timestamp 2026-09-19T11:27:35Z; it is the
  earliest of the 14 boards the library listed at 466 on 2026-11-13 (their
  timestamps run from 2026-09-19 to 2026-11-12). The attribution is
  the library's metadata, not an independent check of identity. The same document
  is mirrored in the public repository `gearashi/EternityIISolver`
  (`data/record466.json`, committed 2026-10-08).
- **Verification** (`compute/verify_pieces.py`, standard library only, against
  `engine/data/official_eternity2.csv`): 256 distinct official pieces; the five
  official clue pieces (139 at cell 135, 181 at 210, 208 at 34, 249 at 221, 255
  at 45, cells row-major from 0) at their official rotations; no grey side off
  the frame; 466 of 480 internal edges matched; the stored edge string equals the
  edges recomputed from the pieces.
- **Scoring convention.** Matched internal edges out of 480. Because the board
  honours all five clues it counts in both regimes: 466 under the strict
  five-clue convention, and 466 under the starter-only convention, where it sits
  below the open record.
- Only this one board was verified. The other 466 boards in the library were not
  downloaded or checked.

## Reproduce

```sh
cd research/topics/record-boards/compute
cargo run --release > ../results/verified-scores.json
```

Deterministic; reproduces the committed `results/verified-scores.json`
byte-for-byte.

```sh
just research-record-boards-pieces
```

Re-verifies the stored community board(s) from pieces and rotations and rewrites
`results/community-verified.json` byte-for-byte. This is artifact verification of
a stored board, not a search. Each board can also be opened directly in the viewer from its
invention page.

## Notes

- This verifies the *board*, not the *search*. The searches that produced these
  boards are stochastic; their write-ups say so and link the board here so the
  result stands on its own.
- "Matched-edge" is the scoring used on the community leaderboard. Some of these
  boards do not obey all five official clue positions; where that matters, the
  invention's page says so.
