# research/

Reproducible research notes for the Eternity II community site. Each **topic**
is a self-contained directory holding an article (with metadata in its YAML
frontmatter), the exact code that produced its numbers, and the committed
results. The website links to these articles ("computed from …") so every figure
on the site is traceable to runnable source.

This directory is designed to scale to many topics without the website or the
index ever drifting from what is on disk.

## Layout

```
research/
├── README.md            ← you are here
├── index.json           ← GENERATED registry of every topic (built from frontmatter)
├── build-index.mjs      ← scans topics/*/article.md frontmatter → index.json
├── TEMPLATE/            ← copy this to start a new topic
│   └── article.md       ← frontmatter + article skeleton
├── topics/
│   └── <topic-id>/
│       ├── article.md   ← the article; YAML frontmatter holds all metadata
│       ├── compute/     ← the exact code that produces the results (+ how to run)
│       └── results/     ← committed output files (JSON/CSV) the article and site reference
└── experiments/
    └── <author>/
        ├── justfile     ← this author's recipes; `just <author>` lists them,
        │                  `just <author> <experiment>` runs one
        └── <experiment>/ ← a self-contained author experiment (see below)
```

## Author experiments (`experiments/<author>/`)

Some experiments are one researcher's own runs rather than shared theory, and a
few carry the code that produced them. Those live under
`experiments/<author>/<experiment>/`, each self-contained:

```
experiments/raphael-anjou/single-core-benchmark/
├── engine/     ← a standalone cargo workspace (lifted from the vault), the
│                 runnable solver family; `run_algo` is the entry point
├── variants/   ← the puzzle inputs
├── results/    ← committed results (jsonl/csv + per-run bucas urls); results/rerun/
│                 (gitignored) is where a reproduction writes
├── scripts/    ← the grid runner + reporter (repo-relative), plus any archived
│                 wrappers for private-vault engines that cannot run here
└── README.md   ← what is runnable here vs archived, and how to reproduce
```

Recipes are per-author `just` modules: the root justfile does
`mod <author> 'research/experiments/<author>/justfile'`, so `just <author>` lists
that author's experiments and `just <author> <experiment>` runs one. The site
page for the experiment lives in `web/content/research/lab/experiments/<author>/`
and sets `repro.cmd` to that command.

## Conventions

- **The article's YAML frontmatter is the single source of truth** for a topic's
  metadata (id, title, dates, contributors, status, tags, sources, reproduce
  command, result files, and whether the site renders its data). The website and
  `index.json` both derive from it — never hardcode a topic list anywhere else.
  (Same discipline as `web/sitemap.config.ts` for pages.)
- **`status`** is `draft` or `published`. Drafts may live in the tree; the
  website only surfaces `published` topics. Work-in-progress can land without
  leaking to the public site.
- **Results are committed.** The `results/` files are the canonical numbers the
  site displays. Regenerating must be deterministic and reproduce them
  byte-for-byte — no timestamps inside result files (run metadata belongs in the
  frontmatter).
- **`reproduce`** in the frontmatter is the literal command(s) a reader runs to
  regenerate `results/`. Keep it copy-pasteable.
- **One topic = one directory.** No shared mutable state between topics.

## Frontmatter schema

```yaml
---
id: <must equal the directory name>
title: ...
summary: one sentence
status: draft | published
created: YYYY-MM-DD
updated: YYYY-MM-DD
contributors:
  - { name: ..., role: author }
credits: [ ... ]            # who posed the question / supplied data
tags: [ ... ]
sources:
  - { label: ..., url: ... }
reproduce: [ "cmd ...", ... ]
results:
  - { label: ..., path: results/file.json }
site:
  render: true | false      # does the site render this topic's data directly?
  dataFile: results/file.json | null
---
```

`build-index.mjs` parses this with a tiny dependency-free reader (no YAML library
needed, so the static site stays dependency-free — the site reads `index.json`,
not the frontmatter). The parser supports the subset above: scalars, one level of
nested maps, block sequences of scalars, and block sequences of maps.

## Adding a topic

```sh
cp -r research/TEMPLATE research/topics/my-new-topic
# edit article.md frontmatter (id MUST equal the directory name), write the body,
# add compute/ + results/
node research/build-index.mjs        # regenerate + validate index.json
```

`build-index.mjs` fails (non-zero exit) if any `id` mismatches its directory, a
required field is missing, or a declared result/data file is absent — so it
doubles as a CI gate.

## How the website consumes this

The site imports `research/index.json` (filtering to `published`) to render the
research hub, and deep-links each card to the topic's `article.md` on GitHub.
Topics that set `site.render: true` ship a `results/` JSON the site renders
directly (e.g. the reference-number tables).
