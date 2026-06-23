---
id: experiments-log
title: Experiments log (every attempt, with its outcome)
summary: A regenerable roll-up of every experiment in the research notebook — its outcome and a one-line note — surfaced on the site as a searchable log.
status: published
created: 2026-06-23
updated: 2026-06-23
contributors:
  - name: Raphaël Anjou
    role: author
tags:
  - index
  - reference
reproduce:
  - cd research/topics/experiments-log && python3 extract.py > ../../../web/src/data/experiments.json
results:
  - label: Experiments data (JSON, generated)
    path: ../../../web/src/data/experiments.json
site:
  render: false
  dataFile: null
---

# Experiments log

> **Created:** 2026-06-23 · **Updated:** 2026-06-23

## Summary

The research notebook holds hundreds of attempts at Eternity II. This topic rolls
them all up into a single browsable record on the site at
[`/research/lab/experiments`](https://eternity2.dev/research/lab/experiments): every
experiment, its outcome (worked, partial, refuted, idea, or won't-do), a one-line
note, and — for the instructive refuted and partial ones — a short "why" pulled
from its write-up. The point is breadth: the dead ends and half-built ideas are as
useful to publish as the wins, so others can build on them and skip walls already
hit.

## Method

`extract.py` reads the concept notes in the research vault. Each note carries
frontmatter with a `status:` and a one-line `description:`; for refuted and partial
notes it also looks for a verdict, conclusion, or because-sentence to use as the
"why". Nothing is fabricated: a "why" is added only when a confident match is
found. The output is the data file the site's log page renders.

## Reproduce

```sh
cd research/topics/experiments-log
python3 extract.py > ../../../web/src/data/experiments.json
```

Requires the research vault checked out alongside the site repo (the script
resolves it relative to the repo root). The result is deterministic for a given
vault state.

## Notes

- Summaries and why-lines are the original lab notes, kept terse; some use internal
  shorthand. They are faithful to what was recorded rather than rewritten for a
  general audience.
- This is an index topic (`site.render: false`): the page consumes the generated
  data file directly rather than this article.
