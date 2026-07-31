# Site quality backlog

*Scientific-content audit · 31 July 2026 · 179 verified findings and 14 programmes, across content, writing, animation, illustration and organization*

---

## What this is

A prioritised backlog for eternity2.dev, produced by auditing the site as a scientific publication rather than as a codebase. It covers the five axes asked for: **content**, **writing**, **animation**, **illustration** and **organization**, plus the localization, accessibility and performance work that decides whether any of it reaches a reader.

It is deliberately front-loaded with **large items**. Part 1 is fourteen multi-week programmes; the item-level backlog follows. The single most requested item, mapping solvers across many instances with throughput and nodes-to-plateau, is programme **P1**.

### Relationship to the previous audit

`content-quality-audit-2026-07-17.md` was a fact-checking and consistency pass. This is not a re-run of it. Its findings were treated as prior state and spot-checked: the truncated-`description` cluster it flagged is **fixed** (zero truncated descriptions remain across 140 pages). This audit asks a different question: is the site *good*, is it *legible*, is it *seen*, and what should it become.

### The headline judgement

The corpus is strong and unusually disciplined. Numbers trace to groups.io message IDs, claims carry `repro{}` pointers, a four-tier evidence convention is stated and mostly honoured, and community researchers are credited structurally as equals. `why/walls-and-methods` and `build/approaches-map` are better than most published survey work.

The problems are not errors of substance. They cluster into three failures of **delivery**:

1. **The site's evidence is invisible.** 52% of English pages carry no figure at all, and of the 76 `<Figure>` blocks that exist, 55 are collapsed and **zero** are open. A reader scrolling any research page top to bottom sees a static document. The live WASM solver — the site's one irreplaceable asset — is behind a disclosure triangle.
2. **The corpus outgrew its own maps.** Twelve `why/` pages shipped after the synthesis page that is supposed to rank them; eighteen pages have no inbound prose link, including the science door's own entry essay; nine `build/` sub-hubs are a shared boilerplate paragraph.
3. **The measurement layer stops one step short.** The benchmark facility exists and is well built, but it records one seed, one budget and one final number per run, so the questions a reader actually has — how fast, how far, how soon, how reliably — are not derivable from the committed data.

Everything in Part 1 follows from those three.

### How to read the backlog

- **Size** — `S` under a day · `M` a few days · `L` one to three weeks · `XL` a multi-week programme.
- **Impact** — reader value, not effort.
- **V** (verdict) — every finding was re-checked by an adversarial verifier against the files: **C** = confirmed as stated, **P** = the concern holds but a specific was wrong (the corrected claim is in the source JSON). No finding is listed that was refuted.
- **Trilingual cost** — anything touching MDX prose is ×3 (EN/FR/ES). Noted where it dominates.

Findings marked ★ below were additionally re-derived by hand during synthesis; their numbers are exact.

---
## Part 1 — Programmes

Fourteen multi-week initiatives, merged from 23 independently generated proposals. Ordered by what unblocks what: P1–P4 are the measurement spine, P5–P8 the science, P9–P12 the experience, P13–P14 the trust layer.

Each carries a **first step** chosen to be independently publishable, so no programme has to be finished to be worth starting.

---

### P1 · The Solver Atlas — map the solvers across many maps ★

*Size XL · the headline item*

**Today.** `research/experiments/single-core-benchmark/` is a genuinely good facility: 17 solvers declared in `solvers.toml`, one canonical rescorer, a reproducible grid runner. What it committed is `results/results.jsonl` — **150 rows, 15 algorithms, 10 instances, `seed=1` on every row, one 60 s budget, one terminal score.** From that data you cannot derive nodes-to-plateau, time-to-score-k, variance, or a score-versus-time curve, because no run recorded a trajectory. The two strongest engines (`producer`, `alns`) never ran at all, which is why `web/src/data/single-core-benchmark.json` ships `"paradox": {}` and the leaderboard's throughput-versus-score tiles have no data. The README still claims "Seventeen solvers … 170 runs".

**Three axes, crossed deliberately.**

- **Engine** (~25 rows from an extended `solvers.toml`): the 15 that ran, the four strong binaries already present as crates under `engine/crates/bench-audit/src/bin/`, the 19 dfs-study variants, the repair-study family.
- **Instance class**: the 10 corner-pinned official variants; planted ladder rungs at N ∈ {8,10,12,14,16} where the ceiling 2N(N−1) is *proved by construction* so distance-from-optimum is exact; an interior-colour sweep around the measured hardness peak; hint geometries (0/1/5 clues, scattered lattice versus contiguous).
- **Budget decades**: 1 s, 10 s, 100 s, 1000 s single-core, seeds 1–3+.

**The metric layer is the actual deliverable.** Add `--trace out.jsonl --trace-every-ms 250` to `run_algo` and the four strong-engine binaries, emitting `(t_ms, nodes, best_score, max_depth, restarts)`. Everything else derives post-hoc: nodes/s (per family, never cross-compared — keep the existing `nps_unit` discipline); score@decade; **t_plateau** and **nodes-to-plateau**; plateau-score *distribution*, not a mean; time-to-score-k as an empirical CDF for k ∈ {400,420,440,450,460,464,470}; normalised anytime AUC; marginal value of a 10× budget. Candidate headline law: fit `score(B) = s∞ − k·B^−γ` per engine and test whether γ is invariant across families.

**Deliverables.** `research/experiments/solver-atlas/` (registry, runner, `make_site_json.py`, downsampled traces) · a `just experiments solver-atlas` recipe · `web/src/data/solver-atlas{,-traces}.json` · `/research/lab/experiments/solver-atlas/{index,method,findings}` ×3 languages · `AtlasTrajectoryChart`, `PlateauScatter` (the paradox tiles finally get real data), `AtlasHeatmap`, `DecadeLadder`.

**First step (≈3 core-hours, publishable alone).** Add trace checkpoints to `run_algo` and re-run the *existing* 15×10 grid at 60 s with tracing on. That produces the site's first time-to-plateau chart and first score-versus-time curves, on a page that already exists, with no new instances.

**Fix these in the same commit — verified by hand:**
- README says 17 solvers / 170 runs; the committed results are 15 / 150. `scripts/run_grid.py:17` still claims the four strong engines "live in the private v2 vault and are NOT in this repo", although their crates are present.
- **Only 11 distinct score vectors exist among the 15 algorithms.** `border_first_lcv` ≡ `rare_color_first`; `gacolor_ac3` ≡ `gacolor_ac3_ns1` ≡ `verhaard_preferred`; `joe_depth150` ≡ `joe_depth150_bp` — bit-identical across all ten variants. Either three heuristics are not wired in, or three ablations are genuine nulls that were never reported. Resolve this *before* the atlas is presented as an ablation study; the CSP-presets page currently reads as if these are distinct arms.

**Risks.** The full manifold is ~400 core-hours and needs a resumable runner (the ledger records a hardness-peak sweep already lost to a mid-run edit). Commit downsampled curves, keep raw traces out of git. Cross-family nodes/s stays incomparable by construction — the metric spec page must be written *before* the charts, not after.

---

### P2 · E2BENCH-1 — a versioned, published instance suite

*Size XL · precondition for P1, P5, P6*

Every study invents its own instances and none are versioned together. The site publishes 14 instances; the 10 centre-clue instances built for the fair head-to-head live only inside `research/experiments/dfs-study/variants-unpinned/`; the scaling ladder's rungs exist only as `rungs.txt`; the hint study and hardness-peak sweeps generate at run time and commit nothing. A reader cannot answer "what did you run this on".

Seven families under `research/datasets/suite/v1/` in the existing normalised schema, extended with `seed`, `generatorVersion`, `frameRestricted`, `ceiling {value, kind: proved|planted|unknown}`, `sha256`: OFFICIAL-PIN (carried forward), OFFICIAL-CENTRE (promoted out of dfs-study), LADDER-PLANTED (N ∈ {6…16} × 10 seeds, ceiling proved by construction), COLOUR-SWEEP, HINT-GEOM, FRAMED-AB (the `generate_framed` mode is parity-tested across four ports and benchmarked nowhere), and HISTORIC-CAL (the community's calibration instances and their agreed answers, already cited with message numbers on `build/benchmarks.mdx`).

Governance is the point: a semver manifest, per-instance checksums, and a `just suite-verify` that re-derives every checksum, re-scores every planted board against its declared ceiling with the canonical scorer, and re-checks every historic expected answer against its citation. Wire it beside `check:research`. v1 freezes on publication; every result page names the suite version it ran against.

**First step.** Copy what already exists — the 10 centre-clue instances and the ladder rungs — into `suite/v1/`, write `verify.py`, publish the manifest. That more than doubles the published instance count and puts the verifier in place before any new generation.

**Risk.** Copyright shaped this community; use its own solution — publish derived counts and checksums always, instance files only where redistribution is clear. The real cost is discipline: freezing v1 means every study page declares its suite version, a trilingual edit across ~12 pages.

---

### P3 · The anytime metric layer and honest statistics

*Size XL · can merge into P1*

One shared run-record schema (`research/experiments/common/run.schema.json`) emitted by every runner: identity (suite version, instance, solver commit, seed, replicate), a ~200-point log-spaced anytime trace, terminals including `peakRssBytes` (nothing measures memory today), and environment (CPU, physical cores, co-scheduled jobs, binary SHA). One shared derivation module computes every metric from the traces, so metrics are never re-implemented per study.

The statistics gap is the sharper half. The corpus **states means with no spread anywhere**, though the committed results carry it. Add: plateau-score histograms rather than means, bootstrap 95% CIs on every published mean, a **variance decomposition separating between-instance from between-seed variance** (the current `std` column conflates them), and reuse of the paired-t/Wilcoxon already implemented in `research/starter-kit/scripts/compare.py`.

**First step.** Lift the repair study's proven `curve_stride` mechanism into `common/`, add the environment block, re-run only the DFS grid at R=3 with traces — ~2.5 core-hours, and the site's first anytime chart.

**Risk, stated plainly.** ~10× the compute of today's grids, and replication may move published headlines: a mean of 440.8 measured at n=1 per cell is not guaranteed to survive R=10. Budget a correction pass across EN/FR/ES and a ledger entry naming which numbers moved.

---

### P4 · The foreign-engine harness and a joinable benchmark

*Size XL*

Generalise the community-engines plan into a reusable harness: per-engine containers, board-output parsers, and a **three-tier comparability model rendered as a badge** (our re-implementation / the author's code we built / the author's reported number). This matters because the site already re-implements Blackwood and Verhaard and says so carefully in prose; the tier belongs in the data, not only in a callout.

Then open it. A submission is a directory: `results.jsonl` in the atlas schema, one canonical `BoardDoc` per run, and a hardware block. `e2kit verify-submission` recomputes every score with the canonical scorer, checks instance checksums, rejects self-reported numbers. Verified third-party rows render in a separate, clearly labelled band — never merged into the project's own rows. **Seed it** by pushing the three community-buildable engines through the protocol first; an empty leaderboard is worse than none.

Add a **hardware normalisation** run — a fixed reference workload on each machine — so a submission from unknown hardware carries a measured scale factor rather than a claim.

---

### P5 · Core scaling and the multi-core grid

*Size L*

The single-core README explicitly parks this: "A multi-core grid — same 60 s, more cores — is a separate experiment, and is future work." All four strong engines now accept `--threads N`. Build the threads axis, ship a calibration kernel so cross-machine numbers mean something, and check the contention flaw in the current measurement (co-scheduled jobs are not recorded today, which P3's environment block fixes).

---

### P6 · Search-tree cartography

*Size L*

`why/complex-theory` predicts a search-tree width curve; nothing measures one. Instrument the solvers to emit per-depth tree profiles and sampled tree-size estimates, then put the measured width curve on the same axes as the predicted one. This is the cheapest way to turn a theory page into a result page, and it directly serves the fill-order and prune-versus-speed arguments.

---

### P7 · The collapse surface

*Size XL*

The site has three one-dimensional slices — the hardness peak (colours), the scaling ladder (size), the hint study (clue density) — and treats them separately. Measure solve-rate over the **joint** (size, interior colours, hint density) surface with both a heuristic and an exact arm, and test whether the collapse location is predicted by the one-expected-solution criterion. This is the natural next tier after the area law, and it is the kind of result that would make the site the reference rather than a summary of one.

---

### P8 · The replication ledger and the refutation register

*Size XL + L*

**Replication ledger.** Extract the headline quantitative claim from each of the 23 papers on `/research/papers` and give each a reproduction status backed by a topic in the research pipeline. This converts a bibliography — which is what `papers.mdx` is today — into the thing no one else has: a checked map of what the literature actually established.

**Refutation register.** The methodology page promises first-class negatives and links a shelf; **the shelf is empty**. All fourteen outcome-bearing lab experiments landed on `plateaued` or `new-basin`, and CAS's own prose says its schedule is refuted while its frontmatter still says `plateaued`. Retag first, then write two or three genuine negative pages from material already in the prose (the hard-ban trap list that made boards worse, the whole-band destroy that was inert on nine of ten instances). Give every negative a pre-registered hypothesis, a falsification criterion, a committed budget, and a statement of the effect size it could have detected.

---

### P9 · Archive analytics

*Size L*

The 11,511-message groups.io archive is used as a quotation source. Turn it into a dataset: a census of every score ever claimed with its verification status, technique adoption over time, who worked on what and when. The archive is already local (`research/community-exports/messages.jsonl`) and already gated by a citation checker, so the hard part — provenance — is done.

---

### P10 · The Flight Recorder — traces, replay theatre, race replays

*Size XL*

Make every solver run a recorded, scrubbable, annotatable artifact: commit traces the way the repo already commits results, then build a replay theatre where a reader scrubs a real run, annotates a moment, and links to it. Two engines already prove the pieces: `ComplexFunnelAnimated` has the only scrub control on the site, and `StepThroughSolver` precomputes up to 4,000 snapshots into an array and then exposes only Back/Pause/Forward/Restart — the scrubbable data is already in memory and no slider is offered.

This is the programme that would make "watch a solver fail" the thing the site is known for, and it shares its data format with P1's traces.

---

### P11 · Instrumented essays

*Size XL*

Build the data bridge and knob primitives that turn a finding's own sweep into a control the reader operates, then convert the flagship `why/` and `lab/` pages from prose-about-a-sweep into the sweep. The site already has the sweeps committed under `research/topics/*/results`; what is missing is the primitive that binds a JSON result to a slider.

Pair this with the single highest-leverage small change on the whole site: **ship the primary figure open** (see T1).

---

### P12 · One grammar for every figure

*Size L*

~90 figure styles were independently invented. Replace them with a documented visual language: semantic colour roles, one board-field primitive, one figure frame carrying caption, source and permalink, and a chart palette that passes contrast in both themes. Codify the three genuinely good figures into a house spec so new figures start correct.

This is also where the **dark theme** gets resolved: `src/index.css:14` declares `@custom-variant dark (&:is(.dark *))`, `:94` defines a full `.dark` palette, and 106 `dark:` utilities exist in source — but **nothing in `src/` ever sets the `dark` class or a `data-theme` attribute**. Either ship it or delete it; today every chart's dark styling is untested dead code. ★

---

### P13 · Teaching surfaces

*Size L*

The homepage says the site was built for Terra Numerica, which makes education its stated reason for existing, and then hands a teacher a card that is not even clickable. The raw material is unusually good — a printable cut-out generator that deliberately does not spoil the solution, a 3×3→16×16 level ladder, a live visualiser, a measured difficulty chart — and there is no lesson plan, no age band, no worksheet, no answer key, no "what to ask the class".

Build `/classroom` (trilingual, in `PAGE_PATHS`): three ready-made sessions with durations, the exact Print settings for each, the questions to ask at each beat *and their answers*, a one-page handout. Then the eight-lesson guided course with saved progress, a projector mode, and a challenge ladder with a real ceiling. Link it from the Home card and the Start teacher trail, and have `Print.tsx` and `Solve.tsx` link back so the loop closes.

---

### P14 · Provenance, recomputation and citability

*Size XL, merged from four proposals*

The site's central promise is that every number is traceable. Make it mechanically true:

- **Evidence graph.** Fact ids that resolve a number in prose to a JSON pointer inside a committed result file, rendered as a provenance popover, with coverage gated in CI.
- **Recompute CI.** Run the reproduction pipeline on a tiered schedule, hash every committed result, and gate the hand-copied site datasets against their sources — `web/src/data/reference-table.json` is a manual copy of a research result today, with no automatic sync, which AGENTS.md acknowledges.
- **Durable citations.** Commit the archive the citation checker already depends on, snapshot and hash every external source, add accessed-dates and archived fallbacks to `sources[]`.
- **Versioned research.** Derive per-page revision history from git, model superseded results as first-class supersession chains, publish a corpus changelog with a feed. Related: **`updated` is currently unreliable — 98 of 140 pages carry a date older than their last content commit.**
- **Citable by default.** `CITATION.cff`, a Zenodo DOI on tagged releases, a per-page cite widget, and a machine-readable results API.

---
## Part 2 — Themes

Eight patterns that recur across many items. Fixing the theme is usually cheaper than fixing its instances one at a time.

---

### T1 · The evidence is shipped folded shut ★

*The single highest ratio of reader value to effort on the site.*

`<Figure>` (`web/src/components/docs/mdx-components.tsx:103`) supports `collapsible` and `open`. Across the 140 EN pages: **76 uses, 55 `collapsible`, `open` used exactly 0 times.** A Playwright crawl of all 184 prerendered EN pages found `details[open]` = 0 on every one.

The consequence is the site's default reading experience. On `build/local-search/local-search-alns`, under the heading "Watch it run" and the sentence "Reading the loop is one thing; watching it learn is another", the reader sees a single 40 px grey bar. Forced open, that same figure is one of the best things on the site — score climbing 90/112 → 112/112, operator weights moving, a live "accepted (sideways move)" badge.

**Fix.** Adopt "one open, rest collapsed": the figure carrying the page's argument renders open, secondary figures stay collapsed. Mechanically that is adding `open` to ~30 tags plus their `.fr`/`.es` sidecars. The CPU argument is already handled elsewhere — `useRunWhileVisible` stops loops off-screen, so an open figure below the fold costs nothing. Add a `check-research-style.mjs` rule that fails a page whose only interactive figure is collapsible, so it cannot regress.

Compounding it: only 13 of 76 figures carry a caption, and ~22 interactive components sit in prose with no `<Figure>` wrapper at all.

---

### T2 · Illustration stopped tracking content growth ★

**73 of 140 EN pages (52%) import no component at all.** 50 of those exceed 900 words. 39 have no component, no table and no code block.

It is a time series, not a constant:

| `updated` | pages | no figure | |
|---|---|---|---|
| 2026-07-01 | 5 | 0 | 0% |
| 2026-07-02 | 27 | 13 | 48% |
| 2026-07-13 | 9 | 7 | 78% |
| 2026-07-21 | 34 | 14 | 41% |
| 2026-07-22 | 36 | 25 | 69% |
| 2026-07-23 | 2 | 2 | 100% |

The early wave shipped figures with every page; the two large recent waves mostly did not. **15 of 29 `why/` pages** — the science door — have no figure of any kind, and that set is almost exactly the July 22–23 batch.

Three specific absurdities: the site's longest page, `build/dead-ends` (8,062 words, 36 sections), has zero figures, zero tables and zero code. The two orientation pages that exist to be maps — `build/approaches-map` (1,846 w) and `why/walls-and-methods` (3,029 w) — contain no visual map. And no component is unused, so this is an embed-density problem, not dead code.

**Fix.** Set a figure budget per `kind: finding` page and treat it as a publication gate, the way `repro{}` already is. Then decide deliberately which of the ~101 non-interactive pages should gain a lab rather than adding them ad hoc.

---

### T3 · Live labs open on an empty void

Roughly a dozen engine-backed surfaces — the most expensive and most distinctive assets on the site — present as nothing on first paint. `LadderLiveLab` renders three paragraphs and a Run button with a **zero-height** chart region, so pressing Run reflows the whole card. `PhaseTransitionLiveLab` reserves 254 px of blank above a bare axis. `LodestoneRarityLab`, `BreakIndexLab` and the DFS scan labs open on a dark empty board with all-zero readouts. `/playground/watch` — "Watch the machine think" — opens with an 816 px empty grid that pushes its live-stats card below the fold. `/viewer` opens on an 816 × 816 empty dashed rectangle.

The prose consistently describes something the default view does not show: "Each bar is a real probe's depth", "Watch the rare pieces drain away".

**Fix.** One shared *seeded idle* contract in the lab shell: every live lab ships a small committed snapshot of a finished run, rendered at full fidelity on mount, with the control relabelled "Run it live" and a caption saying this is a stored result. The frames are cheap (6×6 and 8×8) and can come from the same `research/topics/*/results` pipeline. Hard requirement: the idle frame must occupy the same box as the running frame, so Run never reflows.

---

### T4 · The corpus outgrew its own maps ★

- **`why/walls-and-methods` is stale by twelve pages.** Twelve of the door's 29 pages were added after its last update; none appears in its confidence table, its method map or its four-walls summary — and it is the page the hub makes the first "start here".
- **The door advertises three incompatible wall counts**: three in the index description, four in walls-and-methods, thirteen families in theorem-sweep. No page answers "how many walls are there, which is strongest, and which two are the same wall".
- **18 pages have zero inbound prose links**, including `why/why-e2-is-hard` — the science door's own entry essay — plus `clue-corridors`, `constraint-immediacy`, `frame-is-not-the-basin`, `no-height-function`, `build/exact/exact-tail-endgame` and `pipelines/cas`. Thirteen more have exactly one.
- **Nine `build/` sub-hubs share a verbatim boilerplate paragraph** and range from 85 to 583 words. `build/exact/index.mdx` repeats its own `description` as its first body paragraph, so the reader reads the same 38 words twice, then a sentence shared with seven siblings.
- **`build/techniques` (92 words) promises** "each with what it is, what it costs, and what it actually bought when measured on this puzzle" and contains no techniques — while three separate curated journeys route readers to it as the place where cost-per-method lives.
- **39 of 140 pages carry no topic tag**, and topic sizes are lopsided (structure 36 … hardware 3).
- Documentation drift: content and components use "the nine roads"; `README.md` and `research/SITE_PLAN.md` still describe "three doors".

---

### T5 · Navigation loses the reader at three specific points ★

1. **Breadcrumbs are fixed at three levels and drop every intermediate hub — 74 of 140 pages lose their real parent.** The tree goes six segments deep (21 pages).
2. **No table of contents below 1280 px.** 66 of 140 pages have six or more headings; the longest is 8,069 words, which is roughly 32 phone screens with no in-page navigation.
3. **The sidebar never scrolls the active page into view**; on long sections the "you are here" marker can sit 1,300 px out of view. Compounding it, **23 of 28 sidebar titles are truncated at every viewport width**, with no tooltip — 26 of 140 titles exceed 40 characters (max 83) against a ~22-character budget. In the `why/` rail every row also carries the same `FINDING` badge, so the badge conveys nothing.
4. **Prev/next disagrees with the sidebar**: 24 of 102 Build links jump to a non-neighbouring page, by as much as 43 positions. The `why/` hub names a four-page reading order and the Next arrow contradicts it on the first click.

---

### T6 · Two evidence tiers wear the same badge

The lab has silently grown two standards. Six pages (CAS, beam-width, fluid-frame, FROSTLINE, LEDGER, scaling-ladder — all backed by a `research/topics/` artifact) run to a high standard: pre-registered gates, paired statistics, expected-versus-measured tables, censoring declared, a limits section. Eleven pages are single unreplicated runs with no baseline arm, no spread and no limits section. **Both wear `rigor: measured` and sit in the same sortable table**, so a reader cannot distinguish a Wilcoxon-backed +2.06 from an unquantified "far more often".

Worse, the ranking is confounded: CLOISTER's 453 cost 0.067 core-hours, PALIMPSEST's 463 roughly 24 — a 60× spread — and `hardware.runs` is unset on every pipeline page, so the automatic cost column silently under-reports stochastic runs by 6–16×. **A reader ranking these methods is ranking compute, not ideas, and the site's own metadata already knows it.**

Two related gaps: scores are reported without their clue regime (CLOISTER's 453 is unhinted, LADDER's 451 is not, and the chart ranks the first above the second), and most headline numbers are composites where a shared refinement tail adds the last few edges — so the leaderboard largely ranks a downstream stage that every entry shares and that has no page.

---

### T7 · Voice has drifted, in a corpus that documents its own rule ★

AGENTS.md requires attribution-neutral prose. **243 first-person occurrences across 45 of 140 pages** — concentrated exactly on the flagship pages (`why/sigma-cycles` 44, `build/dead-ends` 27, `build/exact/lp-relaxations` 22, `why/rigidity-wall` 21) while the other 95 pages comply. The July-22 wave additionally writes in first-person *singular*, next to neighbours that are declarative.

Density is the other half: 145 paragraphs exceed 140 words (rigidity-wall 12, solver-engineering 11); `build/dead-ends` carries 77 sentences over 40 words. The em-dash ban is plausibly a contributing cause — clauses that would be joined by a dash become long semicolon chains — which is worth acknowledging in the style guide rather than fighting page by page.

Twelve experiment pages duplicate "How it works" against "Method", and the newest batch ends on three identical boilerplate closes.

---

### T8 · Trilingual is complete in pages and leaky everywhere else ★

Page parity is real: 140 EN + 140 FR + 140 ES. The leak is everything around the pages.

- **`topics.json` and `authors.json` are not localized**, so ~20 generated Spanish pages ship entirely in English.
- **The Build sidebar's group spine** — the section's whole organising principle — is hardcoded English on the French and Spanish sites.
- **The corpus contains zero non-breaking spaces in 5,262 places French typography requires one.**
- **"matched edge" has four French and four Spanish renderings.** There is no bilingual term authority.
- 32 hardcoded English `aria-label`s on SVG figures; four chart components have English-only string tables, so FR/ES pages get English axes; the experiment leaderboard ships English headers and month names.
- Engine slugs (`row-major`, `spiral-in`, `double-snake`) appear verbatim on `/fr` and `/es`.
- Every FR/ES page links the **English** `.md` sibling.
- Copied English number tables in FR/ES are **wrong, not merely inconsistent**.
- The house rule says French and Spanish are "written, never translated"; the corpus is uniformly translated. Reconcile the rule with the practice, in one direction or the other.

**Strategic call worth making explicitly:** stop adding trilingual pages and spend the next quarter's i18n budget closing the chrome gap. A third language of body text sitting inside English navigation, English axes and English aria-labels is worth less than two languages that are complete.

---
## Part 3 — Quick wins

Small, verified, independently shippable. Each was re-derived by hand during synthesis.

| # | Fix | Where |
|---|---|---|
| 1 | **Subnav highlights the wrong section.** `/research/records` and `/research/papers` highlight **Build a solver** while the breadcrumb and sidebar say **History & community**. A hardcoded regex contradicts the section map. | `web/src/components/docs/ResearchSubnav.tsx:74` vs `web/src/lib/research/nav.ts:123` |
| 2 | **The TOC omits component-rendered headings.** `DocsToc` consumes the MDX manifest only, so `RecordsView`'s seven `<h2>`s never appear and `/research/records` shows a one-item "On this page". | `web/src/components/docs/DocsToc.tsx`, `web/src/components/research/views/RecordsView.tsx:396,420,506,619,627,646,693` |
| 3 | **Heading contradicts its own table and both translations.** EN says "The four committed measurements" over a five-row table; the lede says "Five of the six"; FR and ES both correctly say five. | `web/content/research/why/why-e2-is-hard.mdx:59` |
| 4 | **Nested anchors from the glossary auto-linker break hydration** on all four door hub pages — this is the source of the React #418 errors. | glossary auto-linker; reproduces on `/research` |
| 5 | **The floating Feedback button overlaps body text** on every page, desktop and mobile. | `web/src/components/FeedbackButton.tsx` |
| 6 | **Delete the duplicated first paragraph** from the nine `build/*` sub-hubs, where the body repeats the frontmatter `description` verbatim. Zero risk, and it makes T4's real fix visible. | `web/content/research/build/*/index.mdx` ×3 languages |
| 7 | **Fix the benchmark README drift**: it claims 17 solvers / 170 runs against 15 / 150 committed; `scripts/run_grid.py:17` says the strong engines are not in the repo, but their crates are. | `research/experiments/single-core-benchmark/` |
| 8 | **Three of four lines in the repair-study convergence trace are the same blue.** | `RepairStudyLeaderboard` / convergence chart |
| 9 | **`RecordTimeline` linearly interpolates a step function**, drawing records that never existed. | `web/src/components/research/RecordTimeline.tsx` |
| 10 | **`ExperimentScoreChart` starts its bar baseline at 420**, exaggerating differences. | `web/src/components/research/ExperimentScoreChart.tsx` |
| 11 | **`CorpusScoreDistribution`'s linear y-axis hides the 460+ tail** its own caption is about. | `web/src/components/research/CorpusScoreDistribution.tsx` |
| 12 | **Labs read `SEARCHING` before anything runs** — replace the raw solver status with an explicit idle state. | shared lab shell |
| 13 | **Mobile subnav clips People, Glossary and Search off-screen** with no scroll affordance. | `ResearchSubnav.tsx` |
| 14 | **Wide tables scroll correctly but nothing signals it** — add a scroll affordance. | `ProseTable` in `mdx-components.tsx` |
| 15 | **Correct the digit count on `/algorithms`**: the number quoted is not the count just derived. | `web/src/pages/Algorithms.tsx:124-129` |
| 16 | **`seo.ts` carries two dead page entries**, one using the banned word "Inventions". | `web/src/seo.ts:196-212` |
| 17 | **Resolve GAUNTLET's contradictory lift budgets** (30 min in the summary, 5 min in the method). | `pipelines/gauntlet.mdx` |
| 18 | **Reconcile the two "near-twin pairs" counts** of the official set: 114 on three pages, 50 on a fourth. | `why/why-e2-is-hard.mdx:64` + 3 |

---

## Part 4 — Full item backlog

All 179 findings, by category, sorted by size then impact. `V` = verdict: **C** confirmed, **P** partial (concern holds, one specific corrected). Nothing refuted is listed.


### Content & science (39)

| Size | Impact | V | Item | Anchor |
|---|---|---|---|---|
| XL | high | C | Build a real /classroom surface: the Terra Numerica promise is one dead card | `web/src/pages/Home.tsx:63-73` |
| XL | high | C | Build the missing curriculum: a numbered "learn to solve it" track from /algorithms to the toolkit | `web/src/pages/Algorithms.tsx:2` |
| XL | high | P | Document the engines the pipelines actually run on: "The shared engines" documents none of them | `web/content/research/lab/experiments/raphael-anjou/engines/index.mdx:40-76` |
| XL | high | C | Promote and pursue the CP-SAT paradigm cliff: the door's most consequential experiment is buried in one page and contradicted nowhere | `web/content/research/why/how-hard-is-this-instance.mdx:215-288` |
| XL | high | C | Reconcile "no forced moves" with the door's own branching arithmetic, which says a typical cell has about two candidates, not dozens | `web/content/research/why/index.mdx:31-33` |
| XL | high | C | Ship pseudocode or reference implementations: forty technique pages contain no code at all | `web/content/research/build/reduce/edge-slipping.mdx:224-226` |
| XL | high | C | Write the missing page: the reference architecture of a competitive solver (construct → repair → exact tail) | `web/content/research/build/solvers/index.mdx:34-40` |
| XL | medium | C | Ship a teacher's pack: the classroom assets exist, the teaching material does not | `web/src/pages/Start.tsx:57-73` |
| L | high | C | Establish one bilingual term authority: "matched edge" has four French and four Spanish renderings across three surfaces | `web/content/research/glossary.json` |
| L | high | C | Fill the nine Build sub-hubs: 85 to 180 words each, and one of them is the promised technique shelf | `web/content/research/build/techniques.mdx:20-31` |
| L | high | C | Make the notebook's negative space visible: zero refuted outcomes and zero negative contributions in the whole lab | `web/content/research/lab/` |
| L | high | C | Move the "one caveat" column off walls-and-methods and onto the 26 finding pages that need it | `web/content/research/why/walls-and-methods.mdx:72-88` |
| L | high | P | Paths: the race has no payoff, and its leaderboard compares lanes run under different rules | `web/src/pages/playground/Paths.tsx:859-918` |
| L | high | C | Publish the parameter values the technique pages withhold (weights, widths, schedules, operator sets) | `web/content/research/build/local-search/local-search-alns.mdx:149` |
| L | high | C | Run the experiment no-forced-moves stops at: does the candidate count ever collapse inside a live partial board? | `web/content/research/why/no-forced-moves.mdx` |
| L | medium | C | Write the missing page: why building a solution is easy and finding one is hard | `web/content/research/why/design-recipe.mdx:212-228` |
| M | high | P | Join the two halves of the double-break finding: REPLAY and the DFS study contradict each other and never meet | `web/content/research/lab/experiments/raphael-anjou/dfs-study/findings.mdx:124-132` |
| M | medium | P | /is-it-a-scam never answers the two questions a scam-searcher actually types | `web/src/pages/Scam.tsx:30-41` |
| M | medium | P | Give readers something to do: the site has no exercise, checkpoint or self-test anywhere | `src/pages/` |
| M | medium | C | Make number formatting derive from the page language; there are five conventions in play and none does | `web/src/lib/format.ts:2` |
| M | medium | C | Measure a restart policy for score-hunting, the regime every engine on this site actually runs in | `web/content/research/build/backtracking/restarts.mdx:140-162` |
| M | medium | P | Nine Build sub-hubs are pass-through pages: the description repeated verbatim, then one shared boilerplate paragraph | `research/build/reduce` |
| M | medium | C | Price the flux-invariant endgame certificate against prune-vs-speed, the door's own standard for whether a pruner pays | `web/content/research/why/prune-vs-speed.mdx:104-116` |
| M | medium | C | Print: the classroom tool with no teaching in it, and no link to the difficulty result that would set its dials | `web/src/pages/playground/Print.tsx:43-45` |
| M | medium | P | Rebalance depth: the families that produced zero boards get more words than the families that produced the records | `web/content/research/records.mdx:49` |
| M | medium | C | Reconcile the "written, never translated" house rule with a corpus that is uniformly translated | `web/content/research/why/the-470-wall.fr.mdx` |
| M | medium | P | Reconcile the two explanations of the same plateau: the-470-wall calls the high 460s entropically cheap, walls-and-methods shows methods stopping thirty points lower | `web/content/research/why/the-470-wall.mdx:114-121` |
| M | medium | C | Refresh open-problems: it predates the twelve new pages and none of the open questions they explicitly raise reached the board | `web/content/research/open-problems.mdx:10` |
| M | medium | C | Rewrite single-core-benchmark's "Open work": it proposes measurements three other lab pages already made | `web/content/research/lab/experiments/single-core-benchmark.mdx:176-190` |
| M | medium | C | Solve: throw away less — the per-level times are the exponential lesson and the page discards them | `web/src/pages/playground/Solve.tsx:285-294` |
| M | medium | C | Surface the one newcomer-sized open problem: 8 of 9 entries are marked hard, well-mapped | `web/content/research/open-problems.mdx:29-34` |
| M | medium | C | Turn papers.mdx from a bibliography into a ledger of what the site did with each paper | `web/content/research/papers.mdx:16-21` |
| M | medium | P | Unify the throughput unit: nodes, placements and tiles are compared as if interchangeable, including inside one table | `web/content/research/lab/experiments/peter-mcgavin/backtracker.mdx:171-182` |
| M | medium | C | Viewer reports verdicts it never explains, and carries six dead generator strings in three languages | `web/src/pages/Viewer.tsx:110-124` |
| M | medium | P | Watch: the lede promises a 16×16 lesson the default configuration cannot deliver | `web/src/pages/playground/Watch.tsx:44` |
| S | medium | C | Extend contribute.mdx's house rules from provenance to comparability | `web/content/research/contribute.mdx` |
| S | medium | C | Extend the glossary to the corpus's actual high-frequency jargon, and reuse ScoringPrimer beyond one page | `research/glossary.json` |
| S | medium | C | Make reference.mdx usable as a test harness, not just a table | `web/content/research/reference.mdx:32-44` |
| S | low | C | seo.ts carries two dead page entries, one of them using the banned word "Inventions" | `web/src/seo.ts:196-212` |

### Evidence & method (31)

| Size | Impact | V | Item | Anchor |
|---|---|---|---|---|
| XL | high | C | Add a cross-family bake-off page: every method family on the same ten instances at the same budget | `web/content/research/build/approaches-map.mdx:33` |
| XL | high | C | Publish an equal-budget bake-off: the lab's score rankings are confounded by a 60x compute spread | `web/src/lib/research/hardware-cost.ts` |
| XL | high | C | Split the two evidence tiers the notebook has silently grown, or raise the older score pages to the newer standard | `web/content/research/lab/experiments/raphael-anjou/pipelines/beam-width.mdx:100-101` |
| L | high | C | Algorithms §3: the hardness-peak conclusion is not supported by the data on the page | `web/src/pages/Algorithms.tsx:158-167` |
| L | high | P | Fix core-hours: the "mandatory" cost number reports one unit of a sweep, or nothing at all | `web/content/research/lab/experiments/index.mdx:52-59` |
| L | high | C | Give dead-ends the same reproducibility apparatus every positive experiment page is build-gated on | `web/content/research/build/dead-ends.mdx:1-28` |
| L | high | C | Give the door a stated benchmark protocol and point it at the site's own instance suite | `web/content/research/build/benchmarks.mdx:262` |
| L | high | C | Publish a stage decomposition: report each pipeline's pre-lift score, not just its post-refinement number | `web/content/research/lab/experiments/raphael-anjou/learning/prior.mdx:130-132` |
| L | high | C | Report the clue regime with every score: CLOISTER's 453 is unhinted and LADDER's 451 is not | `web/content/research/lab/experiments/raphael-anjou/pipelines/cloister.mdx:20-22` |
| L | high | C | Split theorem-sweep into what is published, what is proved-but-unwritten, and what is conjectured; nine of thirteen families are promissory | `web/content/research/why/theorem-sweep.mdx:12` |
| M | high | C | Enforce the leaderboard rule the methodology page states: BANDSAW and REPLAY are not solvers | `web/content/research/lab/experiments/methodology.mdx:57-58` |
| M | high | C | Fix clue-corridors' branching model: it uses 22 interior colours where the set has 17, and reads the resulting gap as a finding | `web/content/research/why/clue-corridors.mdx:57-63` |
| M | high | C | Give PALIMPSEST a control arm and publish the provenance of the 461 board it starts from | `web/content/research/lab/experiments/raphael-anjou/learning/palimpsest.mdx:136-143` |
| M | high | C | Report dispersion: the corpus states means with no spread anywhere, though the committed results carry it | `web/content/research` |
| M | high | C | The Scam FAQ contradicts the Scam page it appears on, and asserts an unsourced escrow | `web/src/seo.ts:368` |
| M | medium | C | Attach effect sizes to the door's strongest qualitative claims | `web/content/research/build/local-search/local-search-alns.mdx:165-174` |
| M | medium | C | BANDSAW is the notebook's only proven page and its central claims carry no numbers or repro command | `web/content/research/lab/experiments/raphael-anjou/meet-in-the-middle/bandsaw.mdx:31-32` |
| M | medium | P | Give the seven literature-only sources[] lists a pointer to the data they actually rest on |  |
| M | medium | C | Make `updated` mean something: 98 of 140 pages carry a date older than their last content commit | `research/build/exact/index.html` |
| M | medium | C | Name or drop "the source", "the paper" and "the notebook": load-bearing numbers cite an authority the reader cannot reach | `web/content/research/why/flux-invariants.mdx:172` |
| M | medium | C | Puzzle.tsx restates the record ladder that /research/records owns, and the table links nowhere | `web/src/pages/Puzzle.tsx:188-227` |
| M | medium | C | Replace vague quantifiers in the learning study with the numbers its own sibling page reports | `web/content/research/lab/experiments/raphael-anjou/learning/keyring.mdx:97-99` |
| M | medium | P | Resolve irreducible-hard-region's title against its own control, and give the page sources | `web/content/research/why/irreducible-hard-region.mdx:2` |
| M | medium | P | State plainly that the area-law exponent is fit on two block sizes and extrapolated fifty-fold in area | `web/content/research/why/entropy-area-law.mdx:176-177` |
| S | high | C | Correct the 560-digit lesson on /algorithms: the number quoted is not the count just derived | `web/src/pages/Algorithms.tsx:124-129` |
| S | high | C | Repair the copied English number tables in FR/ES, which are wrong, not merely inconsistent | `web/content/research/why/forbidden-patterns.fr.mdx:103-106` |
| S | medium | C | Fix the units and the like-for-like claim in the engine catalogue table | `web/content/research/build/solvers/index.mdx:30-32` |
| S | medium | P | Make the ALNS demo hard enough to demonstrate the adaptation it narrates | `web/src/components/research/concepts/AlnsLoopLab.tsx:621-643` |
| S | medium | C | Mark or remove constraint-immediacy's superseded engine table, which the page's own control overturns | `web/content/research/why/constraint-immediacy.mdx:116-119` |
| S | medium | C | Reconcile the two "near-twin pairs" counts of the official set, 114 on three pages and 50 on a fourth | `web/content/research/why/why-e2-is-hard.mdx:64` |
| S | low | C | Resolve GAUNTLET's contradictory lift budgets: 30 minutes in the summary, 5 minutes in the method | `web/content/research/lab/experiments/raphael-anjou/pipelines/gauntlet.mdx:82-83` |

### Writing & voice (15)

| Size | Impact | V | Item | Anchor |
|---|---|---|---|---|
| L | high | C | Build a French typography pass: the corpus has zero non-breaking spaces in 5,262 places that require one | `web/content/research/why/the-470-wall.fr.mdx:2` |
| L | high | P | Rewrite the nine build/* section hubs: they are a duplicated lede plus one shared boilerplate paragraph | `research/build/` |
| M | medium | C | Algorithms: the best paragraph on the site is a 14-sentence wall, and the page ends without a close | `web/src/pages/Algorithms.tsx:149-169` |
| M | medium | C | Collapse the "How it works" / "Method" duplication that runs through twelve experiment pages | `web/content/research/lab/experiments/raphael-anjou/pipelines/gauntlet.mdx:78-80` |
| M | medium | C | Kill the new "internal joints" synonym and settle the board-dimension typography | `research/glossary.json` |
| M | medium | C | Replace the seven byte-identical subsection hubs with a decision table per family |  |
| M | medium | C | Restore attribution-neutral voice: the July-22 wave writes in first person singular while its neighbours are declarative | `web/content/research/why/theorem-sweep.mdx:43` |
| M | medium | C | Take solver-engineering.mdx out of the first person and give its measurements a repro path | `web/content/research/build/faster/solver-engineering.mdx:92-93` |
| S | medium | C | Break the 232-word paragraph carrying five separate lessons in /algorithms section 3 | `web/src/pages/Algorithms.tsx:149-168` |
| S | medium | C | Replace the raw solver status with an explicit idle state — labs read "SEARCHING" before anything runs | `web/src/components/research/GauntletLiveRace.tsx:73` |
| S | medium | C | Retire the first-person memoir register and the three identical boilerplate closes from the newest batch |  |
| S | medium | P | Stop escalating at the door: /research/why ends on its hardest prose | `web/content/research/why/index.mdx:15-19` |
| S | medium | C | Turn the formulaic three-question closes into a real research agenda | `web/content/research/lab/experiments/raphael-anjou/pipelines/mosaic.mdx:133-136` |
| S | low | P | Clean up the small Spanish register and regional inconsistencies while the glossary work is open |  |
| S | low | C | Fix the garbled sentence in ring-purity's completion-rate result | `web/content/research/why/ring-purity.mdx:136-140` |

### Illustration & figures (24)

| Size | Impact | V | Item | Anchor |
|---|---|---|---|---|
| XL | high | P | Add the five missing figure types the arguments keep asking for | `web/content/research/why/walls-and-methods.mdx:131-144` |
| XL | high | P | Give the research wiki a figure budget: 114 of 140 EN pages show no figure on load | `web/content/research` |
| L | high | P | Build one shared chart palette module; the current ad-hoc palette fails every colour check | `web/src/components/research/` |
| L | high | P | Redraw the five Blackwood raster figures natively; they are illegible and stylistically foreign | `web/content/research/lab/experiments/joshua-blackwood/solver.mdx` |
| L | high | P | Ship collapsible figures open: 55 of 76 are closed by default and 0 are open | `web/src/components/docs/mdx-components.tsx:107` |
| L | high | P | Stop collapsing every interactive figure by default: 55 of 76 are shut, 0 open, so the evidence never renders | `web/content/research` |
| L | medium | C | Codify the three good figures into a house figure spec so new figures start correct |  |
| L | medium | C | Either ship dark mode or delete it — the .dark palette and 105 dark: utilities never render | `web/src/index.css:14` |
| L | medium | C | Work one technique on the official board and land on a number from /research/reference | `web/src/components/research/concepts/AcThreeLab.tsx:1` |
| M | high | C | Only 13 of 76 figures carry a caption, and ~22 interactive components sit in prose with no figure wrapper at all | `web/content/research` |
| M | medium | P | Cap cell size rather than board size — a 3×3 renders at 710px against a 60px tray tile |  |
| M | medium | C | Illustrate the fifteen why/ pages that carry no figure of any kind | `web/src/components/research` |
| M | medium | C | Reflow, don't shrink, the tree and graph diagrams below ~500px |  |
| M | medium | C | Reuse the live ALNS lab on the repair-study page instead of a static four-panel still | `research/lab/experiments/raphael-anjou/repair-study/findings` |
| S | high | C | Three of the four lines in the repair-study convergence trace are the same blue | `web/src/components/research/RepairStudyLeaderboard.tsx:388` |
| S | medium | C | Add a scroll affordance to wide tables: they scroll correctly but nothing tells the reader columns exist | `web/src/components/docs/mdx-components.tsx:160-166` |
| S | medium | C | CorpusScoreDistribution's linear y-axis hides the 460+ tail its own caption is about | `web/src/data/dataset-corpus.json` |
| S | medium | C | Dark mode is compiled but unreachable, so every chart's dark: styling is untested dead code | `web/src/index.css:14` |
| S | medium | P | FrontierPhaseChart: labels collide in the cluster and the bubble size encoding has no key |  |
| S | medium | P | Interactive figures open to an empty frame, so the reader needs two gestures to see anything | `web/src/components/research/LodestoneRarityLab.tsx:112` |
| S | medium | P | No legend anywhere explains the board's own visual language | `web/src/components/board/BoardSvg.tsx` |
| S | medium | C | RecordTimeline linearly interpolates a step function, drawing records that never existed | `web/src/components/research/RecordTimeline.tsx:119` |
| S | medium | P | Stop truncating bar baselines: ExperimentScoreChart starts its x-axis at 420 | `web/src/components/research/ExperimentScoreChart.tsx:39-40` |
| S | medium | P | The hint-study "free floor" stacked bar asks for a comparison its geometry forbids | `web/src/components/research/HintStudyCharts.tsx:216-253` |

### Animation & interaction (5)

| Size | Impact | V | Item | Anchor |
|---|---|---|---|---|
| XL | high | P | Extract ComplexFunnelAnimated's transport bar into a shared primitive and mount it from Lab | `web/src/components/research/ComplexFunnelAnimated.tsx` |
| XL | high | C | Give every live engine lab a pre-seeded default frame instead of an empty black board | `research/lab/experiments/raphael-anjou/dfs-study/findings` |
| L | high | C | Show the backtrack — the DFS labs promise dead ends and render a frozen counter | `web/src/pages/playground/Watch.tsx:44` |
| M | high | C | Default /playground/watch to a puzzle on the hard side of the phase transition | `research/why/phase-transition` |
| M | medium | C | Make GauntletStepThrough scrubbable — it is a 63-minute loop navigated one step at a time | `web/src/components/research/GauntletStepThrough.tsx:121` |

### Organization & navigation (37)

| Size | Impact | V | Item | Anchor |
|---|---|---|---|---|
| XL | high | C | Build the object indexes the frontmatter schema already encodes (engines, boards, scores, repro topics, hardware) | `web/content.config.ts:148-170` |
| XL | high | C | Open the primary interactive figure on every page — 55 of 76 are collapsed and 0 are open | `web/src/components/docs/mdx-components.tsx:113` |
| XL | high | C | Rebuild walls-and-methods as a current synthesis: it is stale by twelve pages and the door now has three competing wall lists | `web/content/research/why/walls-and-methods.mdx:14` |
| L | high | P | Add a chronology, a recent-changes feed and a programme status dashboard — the data exists, the surfaces do not | `web/content.config.ts:660-666` |
| L | high | C | Add a reader-facing level axis: the frontmatter has five evidence axes and zero difficulty axes | `web/content.config.ts:192` |
| L | high | C | Breadcrumbs are fixed at three levels and drop every intermediate hub — 74 of 140 pages lose their real path | `web/src/components/docs/DocsShell.tsx:473-494` |
| L | high | C | Fix the glossary auto-linker: it spends its 10-term budget on "piece", "cell" and "score" | `web/plugins/research-glossary-autolink.ts:63` |
| L | high | C | Link the twelve new pages into the rest of the site: none has an inbound reference from outside why/, and five are total orphans | `web/content` |
| L | high | C | Make prev/next follow the hub's own "Where to start", not the URL tree | `web/src/lib/research/nav.ts:238` |
| L | high | C | Make the research wiki send readers back to the toys: the funnel runs one way | `web/content/research/` |
| L | high | C | Reconcile prev/next with the sidebar in Build: 24 of 102 links jump to a non-neighbouring page, up to 43 rows away | `web/src/lib/research/nav.ts:88-118` |
| L | high | P | Search misses ~40 generated pages, has no importance ranking, is research-only, and keeps stale queries | `web/src/components/docs/SearchDialog.tsx:107-121` |
| L | high | P | Stop adding trilingual pages; spend the next quarter's i18n budget closing the chrome gap instead | `research/topics/` |
| L | high | C | Write "The three record engines, side by side" and demote the per-researcher stubs | `web/content/research/lab/experiments/joshua-blackwood/index.mdx:14-18` |
| L | medium | C | Decide deliberately which of the 101 non-interactive research pages should gain a lab | `web/content/research` |
| L | medium | C | Topic hubs are title-only link lists that re-cut the tree; 39 of 140 pages carry no topic at all | `web/src/components/docs/TopicPages.tsx:83-123` |
| M | high | C | Fix the docs sidebar: 23 of 28 titles are truncated at every viewport width and none has a tooltip | `research/why/prune-vs-speed/` |
| M | high | C | No table of contents below 1280px: 66 of 140 pages have six or more headings and the longest runs 8069 words | `web/src/components/docs/DocsToc.tsx:44` |
| M | high | C | Restore the table of contents below 1280 px: a 5,100-word article is 32 phone screens with no way to navigate it | `web/src/components/docs/DocsToc.tsx:44` |
| M | high | C | The sidebar never scrolls the active page into view; on long sections the 'you are here' marker is 1300px off-screen | `web/src/components/docs/DocsSidebar.tsx:403-427` |
| M | high | C | Wire in the seven orphan pages from the 22-23 July batch, starting with why/why-e2-is-hard | `research/why/why-e2-is-hard` |
| M | medium | P | Close the loops in Start.tsx: five trails, no endings, and one mis-routed step | `web/src/pages/Start.tsx:387-426` |
| M | medium | C | Extend search and the glossary to the product half of the site | `web/src/components/docs/SearchDialog.tsx` |
| M | medium | C | Fill or delete the technique shelf: techniques.mdx is 92 words of links that duplicate approaches-map | `web/content/research/build/techniques.mdx:20-30` |
| M | medium | C | Give the 39 untagged pages topics, or the topic taxonomy never reaches them | `research/why` |
| M | medium | C | Reduce /research to one navigation system: six compete on the same screen | `web/content/research/index.mdx:41-55` |
| M | medium | C | Reproduce index and the ten by-contribution shelves have no nav home: exactly one inbound link each | `research/build/reproduce` |
| M | medium | P | Route readers out of the why/ door, and rescue the community cluster from near-isolation | `research/records` |
| M | medium | C | Start: two profile trails promise destinations that do not deliver, and the researcher path omits the frontier | `web/src/pages/Start.tsx:63` |
| M | medium | C | The Why section is a flat, ungrouped list of 29 near-identical rows with redundant kind labels and truncated titles | `research/why/` |
| M | medium | P | The homepage card grid ignores the two pages that carry the site's search intent | `web/src/pages/Home.tsx:36-62` |
| S | medium | C | Fix the door's on-ramp: build/index omits the dataset, the benchmark protocol and the formats page | `web/content/research/build/index.mdx:17-38` |
| S | medium | C | Fix the two order collisions and the stale "Where to start" list, which leave the sidebar arbitrary at the argument's turn | `web/content/research/why/flux-invariants.mdx:14` |
| S | medium | C | Lab ownership is ambiguous: folder path and `author:` disagree, and a researcher plus a scored experiment vanish from the gallery | `web/src/components/research/ExperimentAuthors.tsx:57-63` |
| S | medium | C | Mobile subnav clips People, Glossary and Search off-screen with no scroll affordance | `web/src/components/docs/ResearchSubnav.tsx:50` |
| S | medium | C | The subnav highlights 'Build a solver' on /research/records and /research/papers, which sit in History & community | `web/src/components/docs/ResearchSubnav.tsx:62-63` |
| S | low | C | Fix the drifting counts: "one of three studies" where the notebook has four | `web/content/research/lab/experiments/raphael-anjou/learning/index.mdx:99` |

### Localization (12)

| Size | Impact | V | Item | Anchor |
|---|---|---|---|---|
| XL | high | C | Extend the localization contract to the data registries: topics.json and authors.json have no Spanish at all | `web/content/research/topics.json` |
| M | high | C | Localize topics.json and authors.json: 20 generated Spanish pages ship entirely in English | `web/content/research/` |
| M | high | C | The Build sidebar's group spine — the section's whole organising principle — is hardcoded English in French and Spanish | `web/content.config.ts:300` |
| M | medium | P | Fix the glossary auto-linker's singular-only matching, which is why FR/ES have 5-7× more dead terms than EN | `web/plugins/research-glossary-autolink.ts:215` |
| M | medium | P | Gate the frontmatter prose fields the parity check ignores: stages, complexity and repro.scope stay English | `web/content/research/lab/experiments/raphael-anjou/learning/keyring.fr.mdx:36-43` |
| M | medium | C | Ship FR/ES markdown siblings, or stop advertising them: every FR/ES page links the English .md | `research/.` |
| M | medium | C | The trilingual string tables have drifted in meaning, not just wording | `web/src/pages/Algorithms.tsx:166-167` |
| M | medium | C | Translate the engine's scan-order slugs — row-major/spiral-in/double-snake appear verbatim on /fr and /es | `web/src/components/research/DfsScanOrderLab.tsx:193-195` |
| M | medium | P | Translate the figure captions and note props still in English inside 40 FR and 55 ES MDX bodies | `research/lab/experiments/raphael-anjou/hint-study/findings/index.html` |
| S | medium | C | Four chart components have English-only string tables, so FR and ES pages get English axes | `web/src/i18n/index.tsx:129` |
| S | medium | C | Localize the experiment leaderboard chrome: English headers, family labels and month names on FR/ES hubs | `web/src/components/research/ExperimentResultsTable.tsx:304-310` |
| S | medium | C | Preserve the hash on language switch, and decide whether translated heading anchors are supportable | `web/src/i18n/index.tsx:111-116` |

### Accessibility (11)

| Size | Impact | V | Item | Anchor |
|---|---|---|---|---|
| L | high | C | Pass respectReducedMotion in the 13 concept labs — 6 of 8 tested still animate under reduce | `research/build/backtracking/fill-order` |
| L | high | C | Ship a dark theme that is reachable: the full stylesheet exists and works but nothing ever sets the .dark class | `web/src/index.css` |
| L | medium | P | Add a pause control to the 7 labs and 13 diagrams that loop forever with no way to stop them | `web/src/components/research/concepts/FillOrderLab.tsx:208` |
| L | medium | C | Give research pages a print and classroom-PDF path: today printing an article emits the whole site chrome | `web/src/index.css` |
| M | high | C | Give BoardSvg a text alternative: the site's most-used visual is a silent SVG on 31 components | `web/src/components/board/BoardSvg.tsx` |
| M | high | C | Nested anchors from the glossary auto-linker break hydration on all four 'door' hub pages | `research/community` |
| M | medium | P | Add a hover and touch affordance to clickable board cells — cursor:pointer is the only signal | `web/src/components/board/BoardSvg.tsx:194-222` |
| M | medium | C | Replace colour-only encodings in the labs: green/red/amber legends carry the finding with no second channel | `web/src/components/research/PieceTheftLab.tsx:52` |
| M | medium | C | Wire the 32 hardcoded English aria-labels on SVG figures into the dictionaries they sit next to | `src/components` |
| S | medium | P | Label the 20 Slider call sites: the sliders that carry the argument announce nothing | `web/src/components/ui/slider.tsx` |
| S | medium | C | Move the fixed Feedback button off the prose: it overlaps body text on every page at mobile widths | `web/src/components/feedback-shared.ts` |

### Performance (5)

| Size | Impact | V | Item | Anchor |
|---|---|---|---|---|
| XL | high | C | Split the 996 KB trilingual research manifest that every wiki page downloads before it renders | `web/build/client` |
| M | medium | P | Cut the WASM engine from routes that never run it, and check whether the search index needs to be 400 KB | `research/why/phase-transition/` |
| S | medium | C | Fix the four hydration mismatches (React #418) on the research hub pages | `research/community/` |
| S | low | P | Fix the React #418 hydration error on the experiments hub, which re-renders the score chart client-side | `research/lab/experiments/raphael-anjou/` |
| S | low | P | Serve responsive images: two 1200 px JPEGs ship 510 KB to fill 672 CSS px, and never load on mobile | `research/lab/experiments/joshua-blackwood/solver/` |

---

## Appendix A — Corpus measurements

Every number below was computed directly against the working tree at commit `d5ee166`, and is reproducible from the scripts noted.

**Shape.** 140 EN research pages, 188,552 body words, full trilingual parity (420 MDX files). 27 `index.mdx` hubs. URL depth in segments: 1→1, 2→11, 3→54, 4→35, 5→18, 6→21. ~24 words per sentence corpus-wide (7,794 sentence terminators).

**Figures.** 73/140 pages import no component; 50 of those are >900 words; 39 have no component, table or code block. `<Figure>`: 76 uses, 55 collapsible, 0 open. 13/76 carry a caption. No component under `src/components/research|learn` is unused.

**Links.** 18 pages with zero inbound prose links; 13 with exactly one. Most-linked: `/research/records` 50, `why/rigidity-wall` 30, `lab/.../joshua-blackwood/solver` 21, `why/complex-theory` 21, `why/prune-vs-speed` 21.

**Metadata.** 39 pages untagged. 28 with empty `sources[]` (mostly hubs; `why/irreducible-hard-region` is a `finding`). 14 with empty `related[]`. 98/140 carry an `updated` older than their last content commit. Zero truncated descriptions remain.

**Voice.** 243 first-person occurrences across 45 pages. 145 paragraphs over 140 words. `build/dead-ends`: 77 sentences over 40 words.

**Build.** `manifest-*.js` is 1,018,759 bytes (995 KB) and `web/src/lib/research/manifest.ts` statically imports **all three languages**, so every wiki page pays EN+FR+ES before it renders. The search indexes are larger still — 1.14 MB EN / 1.39 MB FR / 1.34 MB ES — but are **dynamically** imported inside `SearchDialog.tsx:91-95`, so they are lazy and correctly off the critical path. The WASM engine is 176 KB.

**Benchmark facility.** `results/results.jsonl`: 150 rows, 15 algorithms, 10 variants, `seed=1` throughout, single 60 s budget, terminal score only. `solvers.toml` declares 17. **Only 11 distinct score vectors exist across the 15 algorithms** (three groups are bit-identical on all ten variants). `web/src/data/single-core-benchmark.json` ships `"paradox": {}`.

**Dark mode.** `src/index.css:14` declares the variant, `:94` defines the palette, 106 `dark:` utilities exist — and nothing in `src/` ever sets `.dark` or `data-theme`.

---

## Appendix B — Method, and what was checked but not found

**Method.** Two adversarial workflows, 26 agents, ~4.0 M subagent tokens, ~2,000 tool calls, ~3.6 agent-hours wall clock.

- *Content workflow* (12 agents): six readers over disjoint slices — the `why` door, the `build` door, the lab notebook, the product TSX pages, corpus-wide consistency, and pedagogy/reader journeys — each paired with an adversarial verifier that re-opened the cited files. 99 findings, 81 confirmed, 18 partial.
- *Craft workflow* (14 agents): five slices — animation/interaction, illustration/dataviz, information architecture, localization, accessibility/performance — each verified, plus four independent programme generators. 80 findings, 52 confirmed, 28 partial; 23 programmes merged to 14.

Both craft and IA agents drove the **rendered** site: a production build served locally and crawled with Playwright/Chromium, including all 184 prerendered EN pages for the figure census, mobile viewports, and screenshots read back visually.

The orchestrator audited independently rather than only aggregating: all ★ findings were re-derived by hand, and two agent claims were corrected in the process (the search-index size was reported as on-the-critical-path when it is lazily imported; a rendered box was reported at 500 px when it is 816 px).

**Checked, and the site is fine.** No horizontal overflow at 390 px on the pages tested. No unused interactive components. No truncated frontmatter descriptions remain — the prior audit's finding was acted on. `Start.tsx` genuinely does implement designed persona trails with a surprise link each; the site is not missing an on-ramp, whatever else is wrong with the trails. The single-core-benchmark *page* is scrupulously honest about its own limits ("Fifteen solvers", "What this grid cannot tell you") — the drift is in the repo README, not the publication.

**Caveat on verdicts.** Verifiers refuted nothing outright, which is a weaker signal than it looks; treat `P` items as "real concern, check the specific". The ★ items carry my own arithmetic and can be relied on as stated.

**Not covered.** Engine/solver source quality, deployment and infrastructure, and the FR/ES *scientific* accuracy beyond the sampled pages.
