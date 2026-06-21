---
# Metadata lives in this YAML frontmatter — it is the single source of truth that
# `build-index.mjs` reads to produce research/index.json (consumed by the site).
# `id` MUST equal this topic's directory name. status: draft | published.
id: TEMPLATE
title: Short human title of the result
summary: One sentence describing what this topic establishes.
status: draft
created: YYYY-MM-DD
updated: YYYY-MM-DD
contributors:
  - name: Full Name
    role: author
credits:
  - Who originally posed the question or supplied data, e.g. a forum thread.
tags:
  - enumeration
sources:
  - label: Reference or thread
    url: https://...
reproduce:
  - cd engine && cargo run --release --bin <your-bin> > ../research/topics/TEMPLATE/results/example.json
results:
  - label: Results JSON
    path: results/example.json
# Set render: true and dataFile to a results/*.json path if the site should
# render this topic's data directly (e.g. a numbers table).
site:
  render: false
  dataFile: null
---

# <Title of the result>

> **Status:** draft · **Created:** YYYY-MM-DD · **Updated:** YYYY-MM-DD
> **Authors:** Full Name · **Credits:** who posed the question / supplied data

## Summary

One short paragraph: what question this answers and the headline result.

## Background

Why this matters, the context, links to the originating discussion or prior
work. Cite sources (also listed in the frontmatter `sources`).

## Method

Precisely define what is computed/proved. State every convention and constraint
unambiguously — a reader must be able to reimplement from this section alone.
Use LaTeX where it helps: $inline$ and $$display$$.

## Results

The numbers/figures. If the topic ships a machine-readable file, reference it:
`results/<file>.json`. Tables here should match that file exactly.

## Reproduce

The exact commands (also in the frontmatter `reproduce`):

```sh
cd engine && cargo run --release --bin <your-bin> > ../research/topics/<id>/results/<file>.json
```

State the environment if it matters (toolchain version, cores, expected
runtime) and confirm the output reproduces the committed `results/` byte-for-byte.

## Notes / caveats

Known limitations, what was *not* covered, open follow-ups.
