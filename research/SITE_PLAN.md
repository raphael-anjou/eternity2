# Research section — master content plan

This is the build map for the site's research section: every experiment and
finding from the research vault, mapped to where it belongs on the site and why.
It is the companion to `LEDGER.md` (what's been built, oldest-first). The goal is
that **nothing is lost**: everything has a home here, including a "not published"
section for what stays internal and why.

Source vault: `../../v2/vault/` — 384 concept files, 17 basin case studies, 179
session journals, plus curated PAPER / MATH_NOTES / SYNTHESIS docs.

Legend for each entry:
- **repro**: `exact` (deterministic, byte-for-byte) · `seeded` · `stochastic`
 (ship the board, verifiable in viewer) · `heavy` (MIP/long search, script +
 result) · `prose`.
- **viz**: the animation / diagram / interactive demo it should get (the site
 must feel alive).
- **tier**: ★★★ flagship · ★★ finding · ★ supporting · — internal.

---

## The three doors (recap)

- **/research** — chooser + community infrastructure.
- **/research/why** — why the puzzle is hard (the science).
- **/research/build** — how to write a solver (tools, papers, records, dead ends, run-it-yourself).
- **/research/lab** — the open notebook (findings, inventions, boards).

Folders mirror routes at any depth; every level has a `Hub.tsx`.

---

## DOOR 1 — /research/why (the science of hardness)

The user's note: "we had soooo much other proof in many different ways." This
door should carry the whole stack of hardness evidence, not just one argument.

| Page | Source | Claim / key number | repro | viz | tier | status |
|---|---|---|---|---|---|---|
| phase-transition | Mateu 2012 + color-split | 17 interior / 5 frame colors sits on the SAT/CSP hardness peak | exact (split) + measured (wall) | difficulty-wall chart (built) + color swatches (built) | ★★★ | LIVE |
| forbidden-patterns | INTAGLIO vols 138-142 | 99.72% of 2x2 placements impossible; 39%→83%→99.7% climb | exact | feasible-vs-forbidden board demo (built); add: board forbidden-patch heatmap | ★★★ | LIVE |
| rigidity-wall | PAPER rigidity; vols 65/83-101/187/188 | every top board is locally frozen; 13+ MIP-proven regions, halo-4 | heavy (MIP) | animated halo region + "no better neighbour" board overlay (built) | ★★★ | LIVE |
| sigma-cycles | vols 65/99/101/188 | basin-hopping impossible: every proper subset of the 154-cell swap scores worse | heavy | interactive apply-part-of-the-loop diagram (built) | ★★★ | LIVE |
| entropy-area-law | ISENTROPE; MATH_NOTES | distinctness collapses at ~80 cells; grammar entropy h∞≈0.67>0; ρ(n)≈exp(-0.085 n²) | exact (h1,h2,λ_H) + carried (h3,h4,h∞) | h(n) decay chart + ρ(n) log-collapse + Fekete theorem in KaTeX (built) | ★★ | LIVE |
| no-forced-moves | exact recompute | zero forced placements; every interior piece has 73-137 partners (recomputed exact) | exact | partner-count histogram (built) | ★★ | LIVE |
| rare-color-geography | vols 5/7/13 + exact recompute | 5 rare colors only on the border ring, each on exactly 24 edges, 0 interior | exact | board-ring diagram + rare swatches + stacked frame/interior bar (built) | ★★ | LIVE |
| watershed-piece-theft | WATERSHED | 90% of solver deaths are local piece-theft, not global infeasibility | exact | death-cell map; (N,W)-pair scarcity heatmap | ★ | TODO |
| depth-40-wall | | block-search plateaus at depth 40/64 invariantly | seeded | depth histogram by seed; trajectory curves | ★ | TODO |
| ns1-deficit | (Hopfer) | Δ∈{0,1,2,4} for canonical partials; correlates with score | exact | score-vs-Δ scatter | ★ | TODO |

Supporting evidence (fold into the pages above, not standalone): piece-side
polytope (rotation absorbs 307→480), Fiedler spectral frame/interior split,
47-component basin isolation, two-cluster 459 decomposition.

---

## DOOR 2 — /research/build (write a solver)

| Page | Source | Content | tier | status |
|---|---|---|---|---|
| reference | subgrid topic | exact placement counts to validate code | ★★ | LIVE |
| papers | academic-references.md | 22-paper ranked bibliography | ★★ | LIVE |
| records | community-e2-history.md | community record timeline + 480 false positives | ★★ | LIVE |
| dead-ends | dead-ends.md + refutations | 8 approaches that don't work, proven/measured | ★★ | LIVE |
| run-it-yourself | repo + justfile | clone → build → run → reproduce | ★★ | LIVE |
| solvers/ (catalogue) | blackwood-*, engine profiles | Blackwood schedule + break-index; engine heuristic profiles; live solver demo | ★★ | TODO |

`solvers/` is a sub-hub that will grow (Blackwood, McGavin engine, our own).
Each solver: how it searches, with a live in-browser demo (Watch/Paths already
prove the pattern).

---

## DOOR 3 — /research/lab (the open notebook)

### /research/lab/inventions (LIVE hub; many TODO pages)

Record-setting (have verified boards):
| Invention | vol | result | repro | viz/demo | status |
|---|---|---|---|---|---|
| PALIMPSEST | 129 | 463 (project best, matched-edges) | seeded | trap-density heatmap; corner-perm grid | LIVE |
| KEYRING | 181 | 460, new corner-perm | stochastic | patch-prior heatmap; 3-signal vote | LIVE |
| GAUNTLET | 175 | 458, new corner-perm | stochastic | construction-order path animation | LIVE |
| LADDER | 214 | 451 strict, no witness | seeded | rung progression; prefix-depth dist | TODO |
| REPLAY | 213 | 460×8 exact witness replay; double-break discovery | seeded | break-schedule anim; double-break cells | TODO |

Constructive-from-scratch:
| PRIOR | 155 | 460 from scratch | seeded | position-prior heatmap | TODO |
| STAGED | 217 | 436 frame-free, emergent border | seeded | stage-by-stage build animation | TODO |
| STIGMA (pheromone) | 178 | KEYRING component | exact | adjacency network | fold into KEYRING |

Exact / endgame:
| BANDSAW | 218 | 437 frame-free; meet-in-middle endgame | exact | MITM tree anim; tropical-suffix heatmap | TODO |
| CLOISTER-II | 212 | 453 unhinted standalone interior | exact | frame geometry; rim-target overlay | TODO |
| MIDDEN | 215 | perfect wall 153→174 via damage geometry | seeded | break-mask shapes; wall-progression | TODO |

Each invention page uses the shared `InventionLayout` (idea / how / board /
result / open questions / repro) and gets its viz where listed.

### /research/lab/findings (TODO hub)

The structural results from Door 1's research, written as standalone findings
with their proofs. Some overlap with /why is intentional and cross-linked:
/why explains the consequence to a newcomer; /lab/findings gives the full result
+ method + numbers. Candidates: rigidity theorem, sigma-cycles, entropy/area-law,
forbidden patterns, no-forced-moves, NS-1 invariant. (Decide: keep these in /why
only, or mirror richer versions here. Leaning: flagship proofs live in /why;
/lab/findings holds the longer technical write-ups + certificates.)

### /research/lab/basins (TODO hub)

17 board case studies. Gallery with viewer previews (native eternity2.dev
params) + per-board annotation. Avoid duplicating the records timeline; frame as
case studies. Boards: 463 (cp2301), 461 (cp1203), 460 (cp0312), 469 McGavin,
459 (p06), 458 (cp3012), 470 Blackwood, + 10 more.

---

## NOT PUBLISHED (kept internal, with reason)

These stay out of the public site (or only as brief mentions). Listed so they're
accounted for, not lost — full detail remains in the vault.

| What | Where in vault | Why not published |
|---|---|---|
| Design-only / unbuilt inventions (bidirectional A5, DLX A4, RL self-play D4, FSMC J6, per-color Lagrangian J4, M-series cross-domain M1-M15) | INVENTIONS_BACKLOG.md | not built / no result yet; would read as speculation. Revisit if built. |
| Raw session journals | sessions/ | lab minutiae; the findings/inventions distil them. Link the vault, don't republish. |
| Early refuted micro-experiments | concepts/ status:refuted (non-general) | too specific to be instructive publicly; the Dead Ends page covers the general lessons. |
| Internal tooling notes (engine profiles, code-debt, audits) | concepts/, AUDIT_*, CONCEPT_TRIAGE_* | maintenance detail, not research content. |
| χ-truncation / TIDEMARK conditional-marginal internals | tidemark-* | honest negative but math-heavy; mention as a dead end, don't dedicate a page. |

---

## Full vault accounting (every experiment has a home)

The vault holds **219 volumes, 384 concept files, 17 basins, 179 sessions**. The
rule: every one maps to exactly one of three fates below. Detail stays in the
vault regardless; this is about what surfaces publicly.

- **PUBLISH** → becomes (or feeds) a public page. Listed in the door tables above.
- **SUPPORT** → not its own page, but cited as evidence inside a published page.
- **INTERNAL** → stays in the vault only, with a reason. Listed in "Not published".

### Concept themes → fate (all 384 files)

| Theme (file count) | Fate | Where it lands / why |
|---|---|---|
| Record boards & basins (19) | PUBLISH | /research/lab/basins gallery + per-basin notes |
| σ-cycle indecomposability (12) | PUBLISH | /research/why/sigma-cycles (flagship) + SUPPORT in findings |
| McGavin MIP rigidity proofs (18, umbrella) | PUBLISH | /research/why/rigidity-wall (flagship); proofs as SUPPORT/certificates |
| Local-459 halo proofs (6) | SUPPORT | evidence inside rigidity-wall |
| Three-basin iso-plateau (6) | PUBLISH | finding under rigidity-wall or its own /lab/findings page |
| Entropy / ISENTROPE (4) | PUBLISH | /research/why/entropy-area-law |
| Forbidden patterns / INTAGLIO (4) | PUBLISH | /research/why/forbidden-patterns (LIVE) |
| Rare-color / piece-set design (7) | PUBLISH | /research/why/rare-color-geography |
| No-forced-moves / lattice (3) | PUBLISH | /research/why/no-forced-moves |
| Watershed piece-theft (3) | PUBLISH (★) | /research/why/watershed-piece-theft |
| Depth-40 wall (3) | PUBLISH (★) | /research/why/depth-40-wall |
| NS-1 deficit (1) | PUBLISH (★) | /research/why/ns1-deficit |
| Piece spectral / Fiedler (5) | SUPPORT | inside rare-color or a structure page |
| Propagators: AC family, gacolor, alldiff (19) | SUPPORT | /research/build/solvers (how solvers prune) |
| ALNS & local search (16) | SUPPORT | feeds invention pages (REPLAY, MIDDEN) + solvers |
| Bounds LP/MIP (25) | SUPPORT | rigidity-wall + dead-ends (relaxation bounds) |
| Search strategy & pathfinding (18) | MIXED | scan-order/prefix → solvers; the rest SUPPORT |
| Strategy inventions V155+ (60+) | PUBLISH | /research/lab/inventions (one page per named algo) |
| Novel solver architectures: J1, CLOISTER, DLX, A5 (16) | MIXED | CLOISTER→inventions; J1/DLX/A5 → INTERNAL (design/unbuilt) |
| Cross-domain brainstorms K/W/M-series (25+) | INTERNAL | speculative/unbuilt; the good refutations feed dead-ends |
| Refuted / dead-ends (35+) | MIXED | general lessons → /research/build/dead-ends; specific ones INTERNAL |
| Vol-specific deep-dives (45+) | INTERNAL | lab minutiae; distilled into the published findings/inventions |
| Infrastructure & tooling (15+) | INTERNAL | engine profiles, perf, audits — not research content |
| Partial/unbuilt design docs (15+) | INTERNAL | no result yet; revisit if built |

### Named inventions → fate (all found)

PUBLISH (own page): PALIMPSEST(463,LIVE), KEYRING(460,LIVE), GAUNTLET(458,LIVE),
LADDER(451-strict), REPLAY(460-witness-replay+double-break), PRIOR(456-from-scratch),
STAGED(436 frame-free), BANDSAW(437 exact endgame), CLOISTER-II(453 standalone),
MIDDEN(damage geometry). STIGMA/pheromone → fold into KEYRING. MURMURATION,
OPHIDIA, ENGRAVE, SEMAPHORE, LIGHTHOUSE, CHIASMUS → SUPPORT/mention (refuted or
component). INTAGLIO → already in forbidden-patterns + dead-ends (inert as a
pruner).

INTERNAL (design-only / unbuilt): bidirectional A5, DLX A4, RL self-play D4,
FSMC J6, per-color Lagrangian J4, the M-series cross-domain (M1-M17), Q-learning,
streamlining, CVC, SB-BLE. Reason: proposed, no result; revisit if built.

### Eras → public narrative

The 219 volumes compress into a public "how the record climbed" timeline (this
is the Records page's job, already LIVE; can be enriched): 449 baseline → 454
(border-diversity) → 457 (Blackwood schedule + PT) → 458
→ 459 (cross-machine) → 461 (A1 pipeline) → 463 (PALIMPSEST).
Strict-canonical: → 458. Plus the frame-free track as a separate
"clean-slate construction" story.

### Sessions (179) and synthesis docs

INTERNAL. The session journals are the raw lab record; the published
findings/inventions distil them. Link the GitHub vault for provenance, don't
republish. Same for SYNTHESIS_VOL_188, the PAPER_*, MATH_NOTES_* — these are the
sources the public pages are written from; optionally link them as "the full
technical write-up" from the matching finding page.

---

## Cross-cutting: make it alive

Every Door 1 finding and every invention should get the viz noted above. Reusable
building blocks: `BoardSvg`, the WASM engine (live solve/step), recharts, motif
swatches, the difficulty data. Prefer animation/interaction over static where it
aids understanding. Center and shape layouts for readability; these are hard
topics and the UX should make them a pleasure.
