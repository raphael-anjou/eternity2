---

# Content Quality Audit — Eternity II Research Wiki

*Editor-in-chief report · 17 July 2026 · multi-pass audit (page-level + corpus-level), critical/major findings adversarially re-verified against the files*

---

## 1. Executive summary

This is a genuinely rigorous research site, and the audit confirms it. Its defining strengths are real: numbers are traced to groups.io message IDs, empirical claims carry `repro{}` pointers and a declared three-tier honesty convention (Proven / Measured / Reported), the voice is deliberately attribution-neutral, and community researchers are credited by name and structurally as equals to the project's own work. The adversarial re-verification pass is itself evidence of health — **7 of the initially-flagged findings were rejected as false positives**, and a further large batch of "critical/major" flags were *downgraded* on re-read because the pages turned out to be internally consistent with conventions the first pass had misread. Nothing in this corpus is fabricated, no central thesis is broken, and no page is structurally unpublishable.

The weaknesses that survive verification cluster into **five themes**, in rough order of importance:

1. **A record-attribution and record-count contradiction across flagship pages** — the site's single most-visible fact (the 470 tie count, and who holds the 464 five-clue record) is stated two different ways on pages a reader can compare side by side. This is the one finding rated *critical*.
2. **Numeric overstatements in pedagogical/summary prose** — a handful of derived figures and headline glosses (a "4x per core", a "24 points short", a "low 400s", two "fourteen-point swings", a 262,144≈100,000 equation) contradict the exact numbers stated on the very same page. These undercut the site's core promise of numeric honesty.
3. **Machine-truncated frontmatter descriptions** — at least five hub pages ship a `description` cut off mid-sentence with a literal ellipsis, which renders as a broken lead paragraph / social card.
4. **Convention drift** — the forbidden word "invention" for the project's own algorithms (an explicit AGENTS.md breach) appears on two pages; first-person "we" process-narration recurs; and metadata occasionally overclaims (rigor:proven on a self-declared conjecture).
5. **Corpus-wide terminology and citation-style inconsistency** — the headline metric drifts across five names ("matched edges" / "internal joints" / "edge-pairs"…), `16×16` mixes ASCII and Unicode, `border-first`/`frame-first` compete, GitHub source URLs use `/tree/` where `/blob/` is required, and many inline message citations are missing from `sources[]`.

**Overall grade: A− (strong).** The site is well above the bar for a serious technical wiki. The defects are almost entirely *polish and consistency* — one confidence-eroding cross-page contradiction, a cluster of self-contradicting numbers, and a long tail of drift — not errors of substance. Fix the top table and the corpus-wide standardizations and this is a clean A.

---

## 2. Top priorities (confirmed critical + major)

Ranked by reader impact. All rows survived adversarial re-verification at the severity shown.

| # | Sev | File | Problem | Fix |
|---|-----|------|---------|-----|
| 1 | **Critical** | `web/src/pages/Status.tsx` + `web/src/pages/Puzzle.tsx` | Two flagship pages disagree on how many times the 470 record has been tied: Status says "tied once, by Jef Bucas, Dec 2024"; Puzzle says "tied twice… 2024 and 2025" (FR mirrors both). Stated as hard fact on both. | Pick one canonical account; make both pages (EN+FR) match. If two ties, Status must say "tied twice" and name/date both; if one, Puzzle must drop the 2025 claim. |
| 2 | Major | `lab/experiments/single-core-benchmark.mdx` (L133) | Reference table credits the strict 5-clue record of 464 to "Pejic and benj39100" — contradicting every other page + RecordsView, which credit **Benjamin Riotte** (Pejic only reached the range independently). | Change to "Benjamin Riotte, 2026". Use the real name, not the `benj39100` username. |
| 3 | Major | `build/faster/solver-engineering.mdx` (L230–242) | "≈4x per core" headline is carried entirely by McGavin's 225–295M/s figure; the other two modern figures (72.7, 68.4M/s) are ~1x vs the 60–80M/s baseline, and 295M/s is "newest hardware" the same page says barely helps. | Scope the claim (generated straight-line C on comparable hardware), or present a range "~1x tuned portable → ~4x generated C", separating craft gain from hardware gain. |
| 4 | Major | `build/exact/sat-csp-encodings.mdx` (L80–83) | Derivation equates 262,144 raw variables with the community's "~100,000", a 2.6x gap; "before auxiliaries" doesn't close it (auxiliaries *add* variables). In a section whose stated purpose is exactness. | State 262,144 is *before pruning*; ~100K is what survives after border/impossible placements are removed. |
| 5 | Major | `lab/.../repair-study/index.mdx` (L60) | "Most variants finish in the low 400s" — but 13 of 15 variants land 325–377; only random-destroy (402) and DFS-seeded (446) exceed 400. | "Most variants finish in the 360s–370s" — which also sharpens the next clause about the strong-backtracked start being the standout. |
| 6 | Major | `lab/experiments/single-core-benchmark.mdx` (L81–82) | "Every engine here is still 24 points short of… 464" conflates mean with best and is false for 3 of 4 engines: best board is 451 (13 short); naive is ~98 short. Only the Verhaard *mean* (440.8) is ~24 short. | "The best board (451) is 13 short of 464; the top engine's *mean* sits ~24 short." Don't say "every engine … 24". |
| 7 | Major | `build/exact/exact-cover-dlx.mdx` (L147–149) | Garbled sentence: a project-attribution clause is spliced into "follow Knuth's algorithm" advice and doesn't parse ("…to the letter, measured on this project's engine, in the sense that the bug reports were ours"). | Split into two sentences, or drop the grafted clause: "…follow Knuth's published algorithm to the letter; the days lost above were ours to lose." |
| 8 | Major | `build/learning/anti-pattern-mining.mdx` (L36–54) | "Two frequencies, not one" promises a p_high-vs-p_all comparison it never delivers — both categories are distinguished by the *ceiling* alone; p_high does no work. | State the real discriminant (high p_all + high ceiling = structure; high p_all + capped ceiling = trap) and demote the "compare two frequencies" claim, **or** make p_high enter the rule explicitly. Same latent gap on `palimpsest.mdx` L107–120 — verify and fix both. |
| 9 | Major | `lab/.../repair-study/findings.mdx` (L151–152) | "strict to annealing is a fourteen-point swing" but the page's own means are 361→377 = **16**. | "sixteen-point swing". Same error duplicated on `repair-study/index.mdx` L106 (rated minor) — fix together. |
| 10 | Major | `lab/.../pipelines/gauntlet.mdx` (L80–81) | Circular dependency: GAUNTLET says PRIOR is a "later" experiment that "builds on" it, but PRIOR is order:10 vs GAUNTLET order:50, and GAUNTLET says four times it *forks* PRIOR's beam. | Drop PRIOR from the sentence: "…what the later KEYRING builds on." |
| 11 | Major | `why/sigma-cycles.mdx` (L8) | `rigor: proven` but body says "three pairs is a small sample and it is not proven to hold in general." Title "Why basin-hopping is impossible" compounds the overclaim. | Change rigor to `conjectured` (a valid schema enum; "empirical" is **not** in the enum). |
| 12 | Major | `why/sigma-cycles.mdx` (L77–79) | Payoff sentence "The 470 record has stood since 2021" — but the page's only source is McGavin's *469* board (msg 10045). 470 and "since 2021" are unsourced here. | Add a Blackwood-470 source (as `rigidity-wall.mdx` cites), or restate against the 469 the page actually analyzes. |
| 13 | Major | `why/complex-theory.mdx` (L100–101) | `## The funnel` heading has no preceding blank line — under CommonMark/MDX it will not render as a heading (the sole exception among 10 headings). | Insert a blank line before `## The funnel`. |
| 14 | Major | `why/prune-vs-speed.mdx` (L95) | Uses "**invention**" for the project's own algorithms — an explicit AGENTS.md breach ("experiments, never 'inventions'"). | Replace with "experiment". (One occurrence, singular.) |
| 15 | Major | `why/walls-and-methods.mdx` (L27 + closing Callout) | Same breach: "the **invention** results are this project's own work" / "one volume per **invention**". | Replace both with "experiment(s)". |
| 16 | Major | `why/mismatch-geometry.mdx` (L16) | Garbled source label: "the board whose eleven breaks the page analyses" — "eleven" dangles, "breaks the page analyses" is nonsense. User-facing citation. | "…the board whose eleven unmatched edges this page analyses (groups.io msg 10045, September 2020)". |
| 17 | Major | `build/reduce/index.mdx` (L4) | Frontmatter `description` truncated mid-clause with a literal ellipsis ("…learned no-goods, and the…"). | Restore the body's full clause: "…and the edge-slipping invariant." (See §4 for the pattern.) |
| 18 | Major | `build/local-search/index.mdx` (L4) | Same truncation ("…the most reliable polishers here, and the…"). | Complete: "…and the cleanest demonstrations of the rigidity wall." |
| 19 | Major | `build/construct/index.mdx` (L4) | Same truncation ("…grows them cell by cell. The workhorse…"). Note: `DocsShell.tsx:466` renders `description` as the *visible* lead paragraph, so the harm is on-page, not just SEO. | Complete: "…The workhorse behind this project's from-scratch builders, and a clean illustration of why breadth alone stalls in the deep interior." |
| 20 | Major | `lab/.../raphael-anjou/analyses/index.mdx` (L3–7) | Hub advertises *two* analyses; the section holds only one. The promised strict-460 reconstruction (REPLAY) now lives at `learning/replay.mdx` and is never linked — a dangling promise in `description` *and* `metaDescription`. | Link REPLAY, or rewrite the description/body to describe only the band solve. |
| 21 | Major | `lab/.../raphael-anjou/engines/index.mdx` (L36–50) | Hub says "the CSP presets are the one engine documented and measured in full", but `verhaard-reimpl.mdx` (measured, 438/480) sits in the *same folder*, unlinked. Reader can't reach one of its own two engines. | Add a `<Door>` to `verhaard-reimpl`; soften the exclusivity claim (L50 + frontmatter L6). |

**Downgraded on re-verification (do not treat as top-priority):** the `dead-ends.mdx` "uncited claims" flag (the page's Reported/Measured/Proven tiers are self-consistent), the `known-facts.mdx` 10^557 "contradiction" (regimes already demarcated), the `parallel-tempering.mdx` 467 "two origin stories" (both name one engine, eii), the `boards.mdx` five-clue-ladder and Riotte-citation flags, `solution-counting`'s "twenty-year convergence", and several `rigidity-wall` tone flags. These are all now minor/nit or rejected — see the JSON `verify_reason` fields for the reasoning; they are folded into §3/§4 where a real residue remains.

---

## 3. Findings by theme

Compact: *file — problem — fix*. Duplicates merged; corpus-wide items pulled up to §4.

### Correctness & Evidence
- **`build/dead-ends.mdx`** — "Relaxation bounds" says best real boards "sit near 458"; the batch elsewhere puts the record at 470 and project boards at 460–463 — reconcile or clarify it means the LP comparison board.
- **`build/benchmarks.mdx`** — McGavin "~295M/s" here vs the project's own M1 note "~279M tiles/s" — confirm placements vs tiles vs nodes; disambiguate "his claim" vs "measured here", or correct a transcription slip.
- **`build/variants.mdx`** — mantissa `1.1527×10⁷` vs the derived `14,702×784 = 1.1526×10⁷`; present the multiplier as derived (≈784×), not two independent measurements. Also "proven local dead-end" asserts a proof with no repro/message — cite the exhaustive check or soften.
- **`build/tooling.mdx`** — one census row collapses two David Barr tools born 8 years apart (OpenCL 2015; solveforscore ~2023) under "2015-04" — split the row.
- **`build/exact/meet-in-the-middle.mdx`** — 16 GiB crossover "n≈56" doesn't follow from the page's own 16-bytes/entry model (that gives n>60) — recompute or adjust the RAM figure. Also stray word "sky" before "MITM time bar" (edit artifact).
- **`build/exact/exact-cover-dlx.mdx`** — interior seams said to carry "22 colours" (the full palette); interior edges use only the 17 interior colours — change to 17 (aligns with SAT c=17, LP 420×17).
- **`build/reduce/arc-consistency.mdx`** — AC-2001 memory "~367,000" uses e=480 while the whole page uses e=960 (would give ~733,000) — reconcile or state the table is keyed per undirected arc.
- **`build/reduce/alldiff-regin.mdx`** — description "the strongest propagator *anyone has measured*" overstates the sourced basis (body: Ansotegui et al. "most powerful they found" + "not independently replicated") — soften to the sourced claim.
- **`build/local-search/parallel-tempering.mdx`** — Swendsen & Wang 1986 credited with "introducing" parallel tempering; standard attribution is Geyer 1991 / Hukushima–Nemoto 1996 (S&W is the *precursor* replica Monte Carlo) — correct or reframe as "precursor". *Also (downgraded to minor):* reword the "annealing built the 467" section so composition-annealing reads as eii's scaffolding step within a backtracker, not the standalone origin.
- **`build/hardware/quantum.mdx`** — QUBO mismatch term sums over ⟨c,c'⟩ but introduces unbound p,r,p',c',r' indices; "mismatching pairs" never defined as a set — bind the placement indices explicitly.
- **`why/complex-theory.mdx`** — "99%… below depth 150" attributed to Joe with no `sources[]` entry (sibling `hint-geometry.mdx` cites msg 11725) — add it.
- **`why/prune-vs-speed.mdx`** — bare "232× faster engine" has no `sources[]` backing and no repro (only a self-referential article.md that also lacks backing; the results JSON uses an illustrative 1000×, not 232) — add a benchmark link or soften to qualitative.
- **`why/phase-transition.mdx`** *(nit)* — "then it dies completely" overstates: L2/L3 are nonzero (1.5%, 0.25%) before the true-zero band at L4 — "then collapses: by length 4…".
- **`community/boards.mdx`** — "proof… even at 469 there is flexibility" is a single-instance observation, not proof — use "evidence"/"a demonstration". Also drop marketing "brand-new 458" for a project board below the community line.
- **`community/hunt-part-2.mdx`** — Blackwood-470 "retuned schedule published in advance" clause is unsourced on both pages — cite or drop, keeping only the sourced "exact code" (msg 10161). Also scope "already near-optimal" to "within the swept parameters".
- **`community/hunt.mdx`** — the 17.14 closed-form (msg 1947) is the page's most specific claim; confirm msg 1947 contains that exact arithmetic verbatim or mark it derived.
- **`lab/.../analyses/bandsaw.mdx`** — `score:437` + `rigor:proven` risks reading as "437 is a proven optimum"; the proof is only the band endgame and 437 is a frame-free byproduct — state plainly that 437 carries no optimality claim.
- **`lab/.../joshua-blackwood/solver.mdx`** — parenthetical ALNS "462" is an unsourced one-off with no repro — add a "not a benchmarked result" disclaimer or a repro pointer.
- **`lab/.../learning/lodestone.mdx`, `keyring.mdx`, `palimpsest.mdx`, `prior.mdx`** — headline scores (451/422/380; 460; 463; 460) are backed in `sources[]` only by internal *concept* pages, and `repro.cmd` points at a board *verifier*, not the sweep/experiment that produced the effect — point repro at the generating command or add the committed results to `sources[]`. PALIMPSEST also: 461/462 ceiling thresholds unexplained vs the 460 p_high cut; the seed "461 board" has no id/link. PRIOR description oversells "reaches a high score with no starting board" when the body admits a refinement tail — tighten.
- **`lab/.../single-core-benchmark/csp-presets.mdx`** — "a producer at 455" is undefined and unsourced (parent's best is 451) — cite the producer experiment, use 451, or drop the figure.
- **`why/how-hard-is-this-instance.mdx`** — see §4 (mantissa 1.15 vs known-facts 1.115).
- **Product TSX** *(all minor)* — `Home.tsx` "not by supercomputers" overstates vs the site's careful "human or machine" (hobbyists, single machines) — "not by machines". `Algorithms.tsx` "tuned to land on it" is a designer-intent inference from an 8×8-max extrapolation — add "suggests". `Puzzle.tsx` mandatory badge hard-codes `pos===135` alongside prose "I8"/"piece 139" (maps correctly today, but fragile) — derive from clue metadata.

### Clarity & Structure
- **`build/backtracking/fill-order.mdx`** — full-tree `1.07×10⁵⁰` and per-depth peak `~10⁴³` sit close for the same order without flagging they measure different quantities — add "peak at that depth, not the full-tree total above".
- **`build/reduce/arc-consistency.mdx`** — one sentence defines e "loosely as 480 taken in both directions" (reads as e=480) — match the crisp later phrasing "e = 960 (480 adjacencies, both directions)".
- **`build/learning/learned-value-ordering.mdx`** — two parallel example rules set off only by commas read as a comma-series — delimit with dashes.
- **`build/hardware/fpga-solving.mdx`** — the Mar-1 bullet states a superseded ~1/3 ratio before reaching the corrected 1/8 — mark the 1/3 explicitly provisional.
- **`lab/.../learning/index.mdx`** — "458 to 460" conflates REPLAY with GAUNTLET's strict ladder with no antecedent — name/link GAUNTLET. Also the "Reached" column shows bare numbers without the `/480` denominator — add it to the header.
- **`lab/.../pipelines/index.mdx`** — "the two studies that take a single paradigm apart" names/links neither — link the DFS study and repair study.
- **`lab/.../analyses/bandsaw.mdx`, `lodestone.mdx`** — "The result" and "Method" restate identical figures near-verbatim — let Method add mechanism without re-quoting.
- **`research/index.mdx`** — lede opens on a verbless fragment "For the people who want to actually solve it." — lead with what the page is; also duplicated nav (line-85 sentence restates the door directly above).
- **`community/index.mdx`** — two doors both promise "the whole story" over overlapping spans — differentiate timeline vs long-form.
- **Product TSX** — `Watch.tsx`/`Paths.tsx` expose raw engine slugs ("row-major", "spiral-in") in a Select instead of friendly labels — add a label map. `Convert.tsx` abbreviates "(URDL)" unexpanded while the sibling e2pieces help spells out "(top, right, bottom, left)" — expand on first use.

### Metadata
- **Truncated `description` fields** (see §4 for the full list) — `build/backtracking/index.mdx`, `build/faster/index.mdx`, `build/hardware/index.mdx` all trail off mid-clause with `…` in addition to the three in the top table. Complete each from the body/`metaDescription`.
- **`sources[]` completeness** — `build/local-search/evolutionary.mdx` (msgs 1311/3443/8787/8900 back numbers but aren't listed), `build/exact/iterated-maps.mdx` (msg 9348 inline but not listed), `build/hardware/gpu-solving.mdx` (table cites 9996, absent from body+sources), `build/hardware/quantum.mdx` (msg 263 is the lede source, not listed), `build/analysis/solution-counting.mdx`, `build/learning/{corpus-priors,anti-pattern-mining,decoding-records}.mdx` (empirical figures backed only by internal concept pages) — add the load-bearing message/experiment entries. This is the most common metadata gap.
- **`why/mismatch-geometry.mdx`** — `repro: kind: exact` omits `cmd`/`topic` every sibling carries — add them or downgrade the kind.
- **`build/known-facts.mdx`** — 2×2 census quoted to 9 sig-figs anchored only by a page link — confirm `just research-forbidden-patterns` emits those exact integers.
- **`build/run-it-yourself.mdx`** — several `just` recipe names cited as repro commands — verify each maps to a real justfile recipe.
- **`build/techniques.mdx`** — description promises per-technique cost/benefit content the body (a pure signpost) doesn't contain — add the shelf or soften the description.
- **`papers.mdx`** — evaluative reading-list rankings but no `sources[]` and no `updated` field (siblings have them) — add `updated` and note the tiers are editorial judgment.
- **`lab/.../verhaard-reimpl.mdx`, `keyring.mdx`, `staged.mdx`** — `sources[]` backing-directory URLs point at wrong paths / concept pages, not the committed board — verify the GitHub tree URL resolves and points at the artifact of record.
- **`lab/.../palimpsest.mdx`** — corpus-mining stage lacks `published:` key (defaults visible) while siblings mark every stage `published:false` — confirm intentional.
- **`lab/experiments/methodology.mdx`** — the "tier" axis it calls central isn't stamped on sibling pages (they carry `rigor:`/`contribution:`, no `tier`) — reconcile the vocabulary (`status: report` == "Technical report"?) or add the stamp.

### Consistency
- **`build/variants.mdx` vs `known-facts.mdx`** — "TOMY" (all-caps) vs "Tomy" — see §4.
- **`build/known-facts.mdx`** — 460 row's Track column "2023–2026" overlaps the 464 row that supersedes it Jul 2026 — "2023 until beaten Jul 2026".
- **`build/backtracking/restarts.mdx` vs `distributed-solving.mdx`** — 92,907 is a *count* on one page, the *ordinal index* of the successful search on the other — phrase both as the index.
- **`build/faster/distributed-solving.mdx`** — adjacent refs to the same result cite 9686 vs 9688; the census table lists only 9688 — cite consistently.
- **`build/analysis/solution-counting.mdx`** — without-hint figure written "~4.6M" (source label) vs "4.65 million" (body) — pick one.
- **`build/reduce/arc-consistency.mdx`** — domain size "~764" vs "roughly 760" — use 764. **`alldiff-regin.mdx`** — Hopcroft-Karp given as `O(E√V)` then `O(m√n)` — pick one notation.
- **`why/sigma-cycles.mdx`** — cycle size drifts 154 vs "150-cell" — use 154 consistently. **`why/hint-geometry.mdx`** — contiguous-hint figure "80+" (meta/intro) vs "88+" (stat card) — pick the measured number.
- **`why/complex-theory.mdx`** — 14,702 presented as "fifteen thousand" and "14,702" with no cross-note they're one quantity — tie them.
- **`lab/.../keyring.mdx`** — corpus split "23 high / 1255 low" (=1278) unexplained vs the 7,658-board published corpus — add "corpus size at the time of that run". Also `2x2` vs `2×2` glyph drift.
- **`lab/.../gauntlet.mdx`** — `cp=(3,0,1,2)` vs `cp = (3, 0, 1, 2)`; "sixteen seeds" vs "9 scans × 4 seeds" — normalize.
- **`lab/.../midden.mdx`** — "around 150 cells to the 170s" vs "~153 to 167–174 (+21)" — anchor both to the same figure.
- **`lab/.../single-core-benchmark/csp-presets.mdx`** — parent calls the presets "solvers" ("Fifteen solvers"), child insists they're "not a dozen solvers, one AC core wearing different heads" — align the parent to "algorithms/configurations".
- **`records.mdx` / `contribute.mdx` / `people.mdx`** — see §4 (fifteen vs nineteen years vs two decades).
- **`community/hunt.mdx`** — flawless-piece counts 247/249/249 for three *different* boards read as an inconsistency — one distinguishing half-sentence.
- **`lab/.../louis-verhaard/eii.mdx`** — `kind: page` + `contribution: exposition` while sibling engine pages use `kind: experiment` — confirm intentional. `history.mdx`/`people.mdx` use `kind: reference` on narrative hubs — consider `kind: page`.
- **`lab/.../joshua-blackwood/solver.mdx`** — description asserts "tuned near-optimally" as fact where the body hedges it (low samples, "not 100% sure") — soften to "appears near-optimal in Bucas's sampling".
- **Product TSX** — `Home.tsx` prize "$2 million" vs "$2,000,000" same page; `Tomy`/`TOMY` across Puzzle/Solve/Scam/Status — pick one each (see §4).

### Language & Mechanics
- **Comma splices** (recurring, one canonical fix each — join independent clauses with period/semicolon/dash): `build/learning/corpus-priors.mdx` L85; `build/local-search/local-search-alns.mdx` L163 (also missing article on "beam builds"); `why/complex-theory.mdx` L104 (three clauses); `lab/.../bandsaw.mdx` L102; `lab/.../prior.mdx` L116, L131; `lab/.../replay.mdx` L109; `lab/.../joshua-blackwood/solver.mdx` L176 (double-colon stutter).
- **`build/known-facts.mdx`** — two table rows open lowercase "of which…" reading as fragments — prefix "— frame-only colours".
- **`build/tooling.mdx`** — colon after a quote already ending in "!" — recast with dashes.
- **`why/index.mdx`** — description is a verbless fragment ("The design that was tuned…") — make it a full sentence.
- **`why/rigidity-wall.mdx`** — "twenty-eight million four- and five-move combinations" — use the numeral "28 million" (matches page style).
- **`build/benchmarks.mdx`** *(nit)* — verbatim archive quote "amazing that it is being truly just as hard" reads as a typo — add `[sic]` or "as one member put it".
- **`lab/.../louis-verhaard/eii.mdx`** *(nit)* — "$10,000" vs quoted "10 000 dollars" — leave the quote verbatim, keep prose at "$10,000".
- **`community/hunt-part-2.mdx`** *(nit)* — "(msg 2)" reads like a dropped digit — "(the new group's msg 2)".
- **`Scam.tsx`** *(nit)* — verbatim quote "Are there a notary" — leave as-is; flag only so a copy pass doesn't "correct" it.

### Tone & Positioning
- **"invention" breaches** — `why/prune-vs-speed.mdx` and `why/walls-and-methods.mdx` (top table #14, #15). These are the only two hard AGENTS.md violations of this rule.
- **First-person "we" process-narration** (recurring; recast declaratively) — `build/clue-puzzles.mdx` ("we canonicalised", "We solved"), `build/techniques.mdx` ("where we have them"), `build/learning/index.mdx` ("keeps the section from reading as boosterism"), `why/rigidity-wall.mdx` ("We didn't just try… We asked" — the project's *own* process), `why/walls-and-methods.mdx` (narrating Millilaw's campaign), `people.mdx` ("We reproduced"), `contribute.mdx` ("we will handle the rest / find it"), `methodology.mdx` ("the notebook stopped pretending"). *Note:* naming community researchers and their methods is the site's convention and is **not** a violation — only the project's-own-process narration and vague "we" are.
- **Unearned superlatives in descriptions** (soften to declarative) — `why/prune-vs-speed.mdx` ("The single most important idea"), `why/sigma-cycles.mdx` ("the reason is beautiful"), `why/complex-theory.mdx` ("Many in the community consider it the single most important thing").
- **`papers.mdx`** — "curated from a 200-plus-volume research notebook" leans on a private vault's size as an SEO credential — trim to the substantive claim.
- **`community/hunt-part-2.mdx`** *(nit)* — "rewrites the record book" / "detonated a records bombshell" — deliberate literary register for this narrative page; low priority, tone down only if house voice tightens.

### Accessibility
- **`why/complex-theory.mdx`** L100 — heading won't render (top table #13; strictly a structure/a11y defect).
- **`community/boards.mdx`** — `<BoardsFoundGallery />` dropped in with no lead-in sentence — add a one-line caption.
- **Product TSX** — `Watch.tsx` live `BoardSvg` and `Home.tsx` hero `BoardSvg` have no `aria-label`/`<figure>` association (the stats `dl` and caption cover the essentials, so minor/nit) — add aria-labels; wrap board+caption in `<figure>/<figcaption>`.

---

## 4. Corpus-wide patterns (canonical choices)

These recur across many pages; fixing them once, everywhere, removes the largest share of the tail.

| Pattern | Variants seen | Canonical choice | Scope of fix |
|---|---|---|---|
| **Headline metric name** | "matched edges", "internal edges", "interior edge-pairs", "internal joints", "internal edge-pairs" (+ FR "côtés appariés" / "arêtes intérieures") | **"matched edges"** (EN) / **"côtés appariés"** (FR) — the term `ScoringPrimer.tsx` declares canonical and the research components use dozens of times | Collapse "joints"/"edge-pairs"/"internal edges" in `Puzzle.tsx`, `Status.tsx`, `Algorithms.tsx`, `Home.tsx`; FR labels too |
| **Grid dimensions** | ASCII `16x16` (121×) vs Unicode `16×16` (214×) | **Unicode `16×16`** in prose | Retypeset prose site-wide; leave ASCII `x` only in code spans, filenames, slugs (`tiles_16x16`, `official-16x16-pin3corner`) |
| **DFS fill-order name** | "border-first" (16) vs "frame-first" (7); "border ring" (23) vs "frame ring" (4) | **"border-first" / "border"** | Rename in `mcgavin/backtracker.mdx`, `staged.mdx`, `walls-and-methods.mdx`, `cloister.mdx`, `solvers/index.mdx`. Keep "frame" only where it's the fixed compound "border frame" or a genuine construction-anchor sense |
| **5-clue record attribution** | "Benjamin Riotte" (everywhere + RecordsView) vs "Pejic and benj39100" (single-core-benchmark L133) | **"Benjamin Riotte, 2026"**; Pejic reached the range independently, not a co-holder | Fix the one outlier row (top table #2); never use the `benj39100` username as a record attribution |
| **470-tie count** | "tied once" (Status) vs "tied twice / 2024 and 2025" (Puzzle, EN+FR) | **One verified account**, applied to both flagship pages | Top table #1 — resolve the fact first, then propagate |
| **Puzzle age framing** | "fifteen years" (records), "nineteen years" (formats/contribute/quantum), "two decades" (index/people) | Anchor explicitly: **"since the 2007 launch" (≈19 years)** is the dominant framing | Make `records.mdx` say "community effort since the 2007 launch"; disambiguate list-founding (2000) vs launch (2007) wherever the raw number appears |
| **Constrained arrangement count** | `1.15×10⁵⁵⁷` (how-hard) vs `1.115×10⁵⁵⁷` (known-facts) | **`1.115×10⁵⁵⁷`** (the known-facts computed table) | Fix `how-hard-is-this-instance.mdx` L70; ensure it means the constrained (not the 10⁶⁶¹ unconstrained) figure |
| **GitHub source URLs** | `/tree/main/.../file.mdx` (broken for files) | **`/blob/main/.../file.mdx`** | 6+ occurrences: `corpus-priors:16`, `learned-value-ordering:15`, `decoding-records:16`, `when-learning-collapses:16`, `anti-pattern-mining` — global find/replace on file-pointing source URLs |
| **Truncated `description` frontmatter** | `…` mid-clause on ≥6 hub pages | **Complete sentence** (reuse the body/`metaDescription` clause) | `reduce/index`, `local-search/index`, `construct/index`, `backtracking/index`, `faster/index`, `hardware/index` |
| **Inline citations absent from `sources[]`** | Many empirical numbers cite a msg/experiment inline but not in frontmatter | **Every load-bearing number gets a `sources[]` entry** (the AGENTS.md standard) | `evolutionary`, `iterated-maps`, `gpu-solving`, `quantum`, `solution-counting`, the four `learning/*` pages |
| **Manufacturer casing** | "TOMY" vs "Tomy" | Pick one (prose mostly uses **"Tomy"**) | `variants.mdx`, `known-facts.mdx`, `Puzzle/Solve/Scam/Status.tsx` |
| **First-person "we" process voice** | recurs on ~9 pages | **Declarative, attribution-neutral** | Batch recast (see Tone theme); community-researcher naming stays |
| **"≈" vs "about"; digit vs word counts** | mixed for 14,702, seed counts, cycle sizes | Pick one per figure class | Low-priority polish pass |

---

## 5. Recommended workflow

Do it in this order — highest confidence-per-minute first.

1. **Resolve the two facts, then propagate (do first, blocks other edits).**
   - Establish the truth of the **470-tie count** and the **464 five-clue holder** (both are `MEMORY.md`-adjacent: the five-clue record is documented as Riotte in `e2-five-clue-record-464.md`). Fix `Status.tsx`/`Puzzle.tsx` (top #1, EN+FR) and `single-core-benchmark.mdx` L133 (top #2). These are the only reader-facing *contradictions* of fact.

2. **One mechanical batch: truncated descriptions (5 min, zero-risk).**
   Complete the `…` descriptions on the six hub pages from their own body/`metaDescription`. Pure copy-paste; no judgment needed.

3. **One mechanical batch: broken source URLs.**
   Global `/tree/` → `/blob/` on file-pointing `sources[]` URLs (6+ files).

4. **Self-contradicting numbers (the numeric-honesty batch).**
   Fix in one sitting, each a same-page contradiction with an unambiguous correct value: `repair-study` "14→16" (×2 files), `repair-study` "low 400s→360s–370s", `single-core-benchmark` "24 points", `solver-engineering` "4x", `sat-csp-encodings` "262,144≈100K", `known-facts` "2023–2026" overlap, `how-hard` mantissa. Each is a one-line edit backed by numbers already on the page.

5. **Garbled/unparseable text + the one non-rendering heading.**
   `exact-cover-dlx` spliced sentence, `mismatch-geometry` source label, `meet-in-the-middle` stray "sky", and the `complex-theory` missing blank line before `## The funnel`. Small, high-visibility.

6. **Convention breaches (find/replace with review).**
   "invention"→"experiment" on the two `why/*` pages; then the "we" process-voice recast across the ~9 flagged pages and the superlative-softening in the three descriptions. Batch by convention, not by page.

7. **Metadata completeness: rigor + `sources[]`.**
   `sigma-cycles` rigor `proven`→`conjectured` and its unsourced 470; then the `sources[]`/`repro` gaps (the `learning/*` sweep repros, `evolutionary`, `iterated-maps`, `quantum`, etc.). These need a glance at the archive/experiment to pick the right anchor — batch them because they share the same "does this msg/experiment back this number" question.

8. **Terminology standardization (one dedicated pass each).**
   Run "matched edges", `16×16` Unicode, and "border-first" as three separate corpus-wide find/replace passes, each with the exclusion rules in §4 (skip code spans/slugs/quotes). Do these last so they don't churn files you're still editing.

9. **Long-tail polish (opportunistic).**
   Comma splices, `≈`/"about" style, casing (Tomy), heading case in `clue-puzzles.mdx`, the missing gallery lead-in, TSX aria-labels and friendly path labels. Sweep whenever a file is already open for a higher-priority fix.

**What to leave alone:** the 7 rejected false positives and the downgraded flags whose pages are already internally consistent (dead-ends' tier convention, the 10^557 regime split, the eii "two origin stories", the boards five-clue ladder, hunt-part-2's literary register). Re-editing those would *reduce* quality. Their reasoning is preserved in the JSON `verify_reason` fields if a reviewer wants to double-check.

---

*Files cited above are relative to the repo root; the two product pages are `web/src/pages/Status.tsx` and `web/src/pages/Puzzle.tsx`, all research pages under `web/content/research/`. No files were modified in producing this report.*