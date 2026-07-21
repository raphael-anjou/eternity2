# /research/why — section structure & consolidation review

Scope: the 16 EN pages under `web/content/research/why/` (plus `index.mdx`),
their frontmatter (`tier`, `order`, `kind`, `contribution`, `rigor`, `topics`),
the index's argumentative order, inbound prose links from the rest of
`web/content`, and the render mechanics in `web/src`. Review only — no edits.

## How the section actually renders (mechanics that constrain the judgment)

- **Sidebar / reading order** is driven purely by the `order` frontmatter field
  (`src/lib/research/nav.ts` `sectionReadingOrder`, sort key `order ?? 1e9`).
  Inbound links do NOT gate visibility: every page shows in the sidebar and in
  prev/next regardless of whether anything links to it.
- **The section landing (`index.mdx`, `kind: page`) auto-appends a card grid of
  ALL 16 pages** below its prose (`DocsShell.tsx` `HubCards`, invoked at
  render for the section root; cards sorted by `order`). So the index's prose
  need only be an intro — the enumeration is generated.
- **`tier`** renders as a `TierChip` badge (`DocsShell.tsx:539`) and is
  documented as `1 = flagship, 2 = finding, 3 = supporting`
  (`types.ts:240`). It is a per-page credibility/weight badge, not a grouping
  key — nothing sorts or sections by it.
- **"Referenced by" backlinks** come from a compile-time inbound-prose-link map
  (`nav.ts` `backlinkItems`). A page with zero inbound prose links renders an
  empty backlink rail.

Net: reachability is guaranteed by the sidebar + hub grid, so no page is
"lost." But contextual integration (curated intro placement, in-prose inbound
links, the tier badge) is where the real structural quality lives, and that is
uneven.

## The order / tier / kind map (sorted by `order`)

| order | tier | kind | contribution | rigor | page | inbound (total / within-why / outside) |
|---|---|---|---|---|---|---|
| — | — | page | — | — | index | (landing) |
| 10 | — | finding | negative | measured | prune-vs-speed | 19 / 7 / 12 |
| 20 | — | finding | exposition | conjectured | walls-and-methods | 9 / 4 / 5 |
| 25 | — | concept | exposition | — | how-hard-is-this-instance | **0 / 0 / 0** |
| 30 | — | finding | exposition | conjectured | complex-theory | 19 / 2 / 17 |
| 40 | **1** | finding | measurement | measured | phase-transition | 12 / 7 / 5 |
| 45 | **2** | finding | reconstruction | conjectured | design-recipe | 2 / 2 / 0 |
| 50 | **1** | finding | theory | proven | rigidity-wall | 23 / 5 / 18 |
| 60 | **1** | finding | theory | conjectured | sigma-cycles | 10 / 3 / 7 |
| 70 | — | finding | measurement | measured | mismatch-geometry | 4 / 2 / 2 |
| 80 | **1** | finding | theory | proven | forbidden-patterns | 6 / 3 / 3 |
| 90 | **2** | finding | measurement | proven | no-forced-moves | 14 / 8 / 6 |
| 95 | — | finding | measurement | measured | hint-geometry | 3 / 0 / 3 |
| 100 | **3** | finding | analysis | measured | piece-theft | 6 / 1 / 5 |
| 110 | **2** | finding | measurement | proven | rare-color-geography | 7 / 2 / 5 |
| 120 | **3** | finding | theory | measured | border-balance | 5 / 1 / 4 |
| 130 | **2** | finding | theory | measured | entropy-area-law | 9 / 5 / 4 |

## 1. Reading-order coherence — order metadata vs the index's argument

The index has two coexisting orderings that disagree, and the `order` field has
gaps in its own logic.

**(a) The "Where to start" curation contradicts `order`.** The index manually
promotes four pages, in this sequence: walls-and-methods (20) → rigidity-wall
(50) → phase-transition (40) → no-forced-moves (90). But the auto card grid
below renders strictly by `order`, so the *first* card a reader sees is
**prune-vs-speed (order 10)**, which the "Where to start" list never mentions,
followed by walls-and-methods, then the orphan how-hard (25), then complex-theory
(30). The curated on-ramp and the generated grid tell two different "start here"
stories on the same screen. prune-vs-speed being `order: 10` (literally first)
while being absent from the hand-picked intro is the sharpest instance.

**(b) The `order` values encode a rough "framing → walls → mechanisms →
appendix" arc, but tier and order are inconsistent about it.** Orders 10–30 are
the four un-tiered *exposition/negative* framing pages (prune-vs-speed,
walls-and-methods, how-hard, complex-theory). Orders 40–130 are the walls and
mechanisms. That's a defensible spine. But within it: rigidity-wall (the index's
own "wall almost every method ends against," a Tier-1 flagship) sits at order 50,
*after* design-recipe (45), a Tier-2 reconstruction — so the single most
load-bearing wall page is not near the front of the walls block. And
entropy-area-law, named in the index description as one of the three headline
walls ("rigidity, entropy, forbidden patterns"), is dead last at order 130,
behind border-balance and piece-theft. The index's own thesis sentence lists
three walls; two of them (entropy 130, forbidden 80) are ordered behind
supporting-tier material.

**(c) The "structural walls" H2 is a lead-in to nothing visible in the prose.**
The index's final section reads "…the puzzle has measurable structure… Each of
these is published with the exact computation behind it." and then the MDX ends
(all three languages, verified). It works only because `HubCards` injects the
card grid after the body. A reader of the raw prose (or anyone auditing the MDX)
sees a colon-style promise with no list. This is a load-bearing dependence on an
implicit render behavior; the intro sentence should either name the walls it's
promising (rigidity / entropy / forbidden, matching the description) or be
reworded so it doesn't read as a dropped list.

## 2. The tier system — coverage and correctness

**Coverage is partial and non-principled.** 10 of 16 pages carry a `tier`; 6 do
not (prune-vs-speed, walls-and-methods, how-hard, complex-theory,
mismatch-geometry, hint-geometry). The six untiered pages are not a natural
class — four are the framing/exposition front matter (orders 10–30), but two
(mismatch-geometry order 70, hint-geometry order 95) sit in the middle of the
walls block with no tier while their neighbours have one. A reader scanning
TierChips sees badges appear and vanish with no pattern. Either the framing
pages deserve an explicit tier (walls-and-methods and prune-vs-speed are clearly
flagship-weight by inbound links — 9 and 19) or the section should state that
exposition pages are intentionally untiered. Right now it's silent.

**Tier vs actual weight — the mismatches:**

- **walls-and-methods is untiered but is the section's synthesis spine.** It is
  the index's #1 "start here," it is the bridge to the whole /build half, it
  carries the master method→wall table, and 9 pages/pages link it. Leaving it
  with no tier badge while border-balance (a late pruner) carries a Tier-3 badge
  understates it. By weight it is flagship (Tier 1).
- **prune-vs-speed is untiered, `contribution: negative`, but is a top-3 hub by
  inbound links (19 total, 12 from outside why/).** It is the conceptual root
  ("shrinking beats speeding up") that half the section leans on. Flagship-weight,
  no badge.
- **complex-theory is `rigor: conjectured` yet is the second most-linked page in
  the whole set (19 inbound, 17 from outside why/).** The community regards it as
  "the single most important thing to understand" (its own description). It is
  untiered. Whether or not one promotes it, a page this central being both
  untiered and badged "conjectured" is a mismatch between its structural role and
  its credibility signalling — worth an explicit editorial note in the page, not
  a silent gap. (The `conjectured` rigor is itself defensible: it's Owen's
  estimator, not a theorem. The issue is only that the section leans on it as if
  it were settled while badging it as not.)
- **The Tier-1 set is internally sound.** rigidity-wall (proven, 23 inbound —
  the most-linked page, correctly flagship), forbidden-patterns (proven),
  phase-transition (measured, the hardness peak), sigma-cycles (conjectured) are
  the right flagships. One caveat: sigma-cycles is Tier-1 flagship but
  `rigor: conjectured` and `repro: heavy` with only 10 inbound; it is more a
  companion to rigidity-wall than a standalone flagship. Consider Tier-2.
- **design-recipe is Tier-2 but has only 2 inbound links, both from within why/
  (phase-transition, rare-color-geography), zero from outside.** It is a strong,
  well-sourced reconstruction page, but structurally it is a leaf that only the
  walls point back into. Its tier is fine; its isolation is the issue (see §4).

## 3. Is walls-and-methods the closing synthesis, or is one still needed?

**walls-and-methods already IS the section's synthesis, and a good one.** It
opens by naming the two halves of the research section, restates the four walls
in one line each, gives the master method→wall table with the score each method
stalled at, a per-method "where it stopped and why," a "shape of the gap"
close, and the Millilaw cross-check. It bridges outward to /build/approaches-map,
/build/dead-ends, and /open-problems. That is exactly what a closing synthesis
should do. **The section does not need a second synthesis page.**

The one structural mismatch: it is placed at `order: 20` (near the *front*), and
the index correctly frames it as "orient first" — i.e. it is used as an
*opening* map, not a *closing* synthesis. That dual role is fine, but it means
the section has a strong front-map and no deliberate *ending*: the last page by
order is entropy-area-law (130), which just stops. If any closing gesture is
wanted, it is a one-paragraph "where to go next → /build, /open-problems" coda,
which walls-and-methods already contains — so the cleanest fix is a single
"continue to the methods" pointer at the natural end of the reading order, not a
new page. Recommendation: **keep walls-and-methods as the synthesis; do not add
a closing page.**

## 4. Fold candidates — and why the bar is (mostly) not met

The fold bar is high: a fold means merging content plus a redirect, and it's
only justified when a page has thin unique evidence, low inbound demand, AND no
distinct audience. Weighed against that:

- **how-hard-is-this-instance (order 25) — NOT a fold, but the section's one
  true orphan.** Zero inbound links anywhere in `content` (verified across all
  languages) — nothing in why/, nothing in /build, not even the index prose
  links it; it is reachable only via the sidebar and the auto card grid. Yet it
  is substantial and has a genuinely distinct audience: it is written to answer
  the "is this NP-complete?" category error that "tops the dispute list on
  Wikipedia's Talk page" (its own opening), and it carries three worked solver
  encodings (SAT/CNF, exact-cover/DLX, ILP) that live nowhere else in why/. It
  is also the only `kind: concept` page in the section. The right move is the
  opposite of a fold: **link it in.** complex-theory ("counting the search")
  and prune-vs-speed both have natural hooks to "what complexity theory does and
  doesn't say about one instance," and it should appear in the index's framing
  cluster. Folding it into complex-theory would destroy a distinct-audience
  explainer to fix an integration problem that a few inbound links solve.

- **border-balance (order 120, Tier-3, 5 inbound) — NOT a fold.** Tempting
  because it's a "late pruner" leaf, but it carries unique, well-sourced evidence
  (five distinct groups.io citations: 2073, 2098, 422, 10754, 10757 — all
  present in the archive) and is linked from four *outside*-why pages
  (people, build/known-facts, build/analysis/parity-arguments,
  community/hunt-part-2). That outside demand plus unique sourcing clears the
  bar to stand alone. Its natural parent would be rare-color-geography (both are
  border-colour structure), and they cross-link, but the parity/pruning content
  is distinct enough and externally-referenced enough to keep separate.

- **hint-geometry (order 95, untiered, 3 inbound, 0 from within why/) — NOT a
  fold, but weakly integrated.** All three inbound links come from one
  experiment cluster (raphael-anjou/hint-study), none from within the why/
  section itself despite it sitting mid-walls. It has a distinct finding
  (position beats count, McGavin's 18-hint solve, msg 11725/11746 both in the
  archive) and a distinct audience (the clue-placement question). Keep it, but it
  should earn at least one within-why inbound link (no-forced-moves and
  phase-transition are the natural hooks; it already lists them as `related`, but
  the *prose* of those pages doesn't link back).

- **piece-theft (order 100, Tier-3, 6 inbound, 5 from outside) — NOT a fold.**
  It is the mechanism page five method write-ups (prior, mosaic, midden,
  lodestone, beam-search) point to for "where solvers die." That outside demand
  is exactly what makes it a keeper: it's a shared explainer for the /build
  half. Its within-why integration is thin (1 link, from no-forced-moves), but
  the outside role is load-bearing.

- **mismatch-geometry (order 70, untiered, 4 inbound) — NOT a fold, but it is
  the closest structural sibling to sigma-cycles / rigidity-wall** (all three
  are "where the errors/rigidity live on a record board," all cite McGavin's 469,
  msg 10045). It could be argued as a section *within* rigidity-wall, but it
  carries a distinct measurement (the error-band lives in one 5-row band set by
  fill direction, mirrored on real records) and links out to the staged
  pipeline. Keep separate; the three form a natural "record-board anatomy"
  cluster that could be grouped by adjacent `order` (see below) rather than
  folded.

**No page in the section clears the fold bar.** The section's problem is not too
many pages; it is uneven *linking and tiering* of the pages it has.

## 5. Consolidation without folding — the cheap structural wins

These change metadata / a few prose links, not page count:

1. **Fix the orphan.** Add inbound prose links to how-hard-is-this-instance from
   complex-theory and prune-vs-speed (and ideally name it in the index's framing
   cluster). One page with zero inbound links in a 16-page section is the single
   clearest defect.
2. **Tier the framing pages, or declare them untiered.** walls-and-methods and
   prune-vs-speed are flagship by weight; give them a tier or add a one-line
   convention ("exposition pages carry no tier") so the badge pattern stops
   looking arbitrary.
3. **Reconcile the two "start here" orderings.** Either add prune-vs-speed to the
   index's "Where to start," or reorder so the first auto-card matches the
   curated on-ramp. Right now card #1 (prune-vs-speed) and curated #1
   (walls-and-methods) disagree.
4. **Repair the dropped-list intro** in the index's "structural walls" section
   (all three languages): name the walls the sentence promises, or reword so it
   doesn't read as a lead-in to a missing list.
5. **Order the record-board anatomy cluster together.** rigidity-wall (50),
   sigma-cycles (60), mismatch-geometry (70) already are adjacent and thematically
   one cluster — good. But entropy-area-law (130) and forbidden-patterns (80),
   both named in the index thesis as headline walls, are ordered behind
   supporting material; consider pulling them forward so the "three headline
   walls" the index names actually read first.

## Bottom line

The section is well-populated and needs no folds and no new synthesis page —
walls-and-methods already carries the synthesis. Its real structural debt is
integration and signalling: one true orphan (how-hard, 0 inbound), a tier system
applied to 10 of 16 pages with the two heaviest hub pages (walls-and-methods,
prune-vs-speed) left un-badged, two "start here" orderings that disagree on the
same screen, and an index whose thesis names three headline walls (rigidity /
entropy / forbidden) that the `order` field then buries behind supporting pages —
plus a "structural walls" intro that reads as a dropped list in the raw MDX.
