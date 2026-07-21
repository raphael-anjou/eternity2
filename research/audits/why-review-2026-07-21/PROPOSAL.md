# /research/why — deep review verdict, reconstruction proposal, and experiment slate

*2026-07-21. Method: 49-agent review (17 per-page claim inventories on a five-tier
evidence scale, 5 cross-cutting analysts, adversarial verification of every major
finding). 22 findings survived verification, 2 refuted, 43 minors. Evidence
reports live beside this file (cross-*.md, RESULT.json). The private vault was
out of scope throughout.*

## 1. Verdict

The section's architecture is sound and should NOT be rebuilt. The structure
review's bottom line: no page clears the fold bar (every tail page has inbound
citations or a distinct evidence artifact), and walls-and-methods already is the
closing synthesis the section needs. Several pages came back clean at a high
standard (walls-and-methods: zero number drift across 12 method scores and
8 sampled FR/ES passages; how-hard-is-this-instance: "unusually rigor-clean";
phase-transition's numbers all verify to the message level, including a
byte-level match of Owen's 1947 derivation).

The section's real debt is narrower and deeper: **five of its strongest claims
are not backed by their own reproduce commands.** The entropy area-law headline
(alpha ~ 0.085, the ~80-cell collapse) is carried from off-site computation
while the page badges repro: exact; sigma-cycles' three headline cycle numbers
(154/80/85) exist nowhere but the page's own prose in three languages;
rigidity's MIP proof table has no committed result (only the weaker SAT halo
reproduces); phase-transition's repro covers only the 17/5 color split, not the
titular peak; prune-vs-speed's committed results do not reproduce from its
command. On a site whose core promise is "no result is an unbacked claim,"
this is the thing to fix, and prose softening is the wrong primary fix:
**the right fix is to build the missing experiments** (section 4).

## 2. What ships now (corrections, no user decision needed)

The 22 verified findings, grouped; full text in RESULT.json:

- **Inverted/unsupported numbers:** complex-theory's depth claim inverts its own
  source (Joe, msg 11725: time is spent past depth 132/150, not below);
  design-recipe's record history contradicts the records page; hint-geometry's
  "88+ contiguous hints" stat has no source; border-balance's ">=85%
  interior-to-interior" is uncommitted (softened now, exact recount queued as
  experiment E7); design-recipe's "roughly 21,000 piece designs" unbacked.
- **Rigor calibration:** phase-transition's title/meta assert "exactly where
  hardest" while the body correctly says "around 17, a band"; entropy-area-law's
  frontmatter presents vault-carried results as repro: exact (recalibrated until
  E3 lands); rigidity-wall's rigor: proven scoped to what is actually committed;
  sigma-cycles' honest three-pair hedge surfaced to the top and into the live
  Lab component note.
- **Structure quick wins:** the index's "structural walls" H2 leads into
  nothing (now leads into the wall list); how-hard-is-this-instance is a
  zero-inbound-link orphan (linked from index + complex-theory); tier badges
  normalized (the two heaviest hub pages were unbadged).
- **i18n:** FR terminology split on the core noun board (3 pages use a
  different word than the glossary's plateau); ES register inconsistency
  (usted vs tu across pages); ES thousands separators left as English commas.
- **Refuted by verification (no action):** the benj39100 annealing paragraph
  (page already frames it as relayed) and the Zamofing quote attribution.

## 3. Reconstruction proposal (approval requested)

Three additions, no page moves, no folds, no redirects:

1. **Render the confidence map.** cross-confidence-map.md contains a
   publication-ready wall-by-wall table: strongest supported form of each
   claim, tier, replication status, one-line caveat. Proposal: new section
   "What each wall rests on" at the top third of walls-and-methods (the page
   the review certifies as the synthesis), plus a one-line pointer from the
   index. This is the single highest-value addition the review found: the
   section mixes exact internal counts, one externally-replicated result, and
   single-instance measurements under one visual weight, and only rigidity has
   multi-method replication.
2. **State the two supportable wall connections; pose the three open ones.**
   From cross-interdependence.md: stateable as fact (b-tier): piece theft is
   the cell-level face of the use-once constraint whose area-cost the entropy
   page measures (both pages already say this separately; neither links);
   forbidden patterns and the area law are both exact scarcity-of-small-patches
   counts (with the precision caveat: color-match axis vs use-once axis).
   Proposal: one connective paragraph on the index under the walls list +
   reciprocal related[] links. Open (c-tier), proposed as three new
   /research/open-problems entries, not as facts: is rigidity caused by the
   distinctness collapse (the shared ~80-cell scale); does the hardness peak
   produce the no-forced-moves branching profile; is sigma-cycle geometry the
   common cause of constructive-search saturation.
3. **Reading-order alignment.** Make order metadata match the index's actual
   argument (framing -> walls -> mechanisms -> synthesis), so sidebar order and
   the curated start list stop disagreeing (details in cross-structure.md §1).

## 4. The experiment slate (approval requested; all site-repo pipeline, no vault)

Each experiment closes a named gap; "changes" states which sentence upgrades.

| id | Experiment | Compute shape | Est. cost | What it changes |
|---|---|---|---|---|
| E1 | **Sigma-cycles topic**: permutation-cycle decomposition between ALL pairs of the 16 bundled boards (record-boards + known-boards), committed results + the exact board pairs | Rust, exact, embarrassingly parallel over pairs | minutes | The three page-only numbers (154/80/85) become proven-internal with committed evidence; the three-pair sample becomes a population statement over every public pair; the Lab note gets real data |
| E2 | **Rigidity: extend the committed SAT halo** to radius 3-4 on the five public boards, with positive controls | SAT (existing rigidity-sat-halo crate), hours | hours | The page's proven badge covers a stronger committed statement; the external MIP table stays but stops carrying the page alone |
| E3 | **Entropy area-law in-repo**: extend the topic crate to compute the block counts A(n)/B(n) and the rho(n) fit that the page and Lab currently carry from off-site | Rust exact counts (n<=4/5), then the fit | hours-day | repro: exact becomes true; the alpha ~ 0.085 and ~80-cell figures become proven-internal (as counts) + measured (fit); the site JSON regenerates from the command |
| E4 | **Hardness-peak sweep**: solve-time vs interior-color-count on generated 16x16 instances (the site's own generator), N seeds per count | existing engine, embarrassingly parallel | ~day on 8 cores | The peak stops being externally-cited-only; "tuned to the peak" gains an in-repo measured curve; title's "exactly" either earns itself or the band stays |
| E5 | **prune-vs-speed repro fix**: make the committed results regenerate from repro.cmd (bug in the crate or stale results; determine which) | debug + rerun | hours | The repro: exact declaration becomes true |
| E6 | **Contiguous-vs-scattered hints**: run the existing hint-study experiment harness on contiguous-row layouts to measure the threshold the page asserts as "88+" | existing hint-study engine | hours | The unsourced 88+ becomes measured (or corrected); stat card gets a real number |
| E7 | **Border-balance recount**: exact interior-to-interior share of unmatched edges across the bundled 469-class boards | trivial script over bundled boards | minutes | ">=85%" becomes proven-internal (or corrected) |
| E8 | **Archive refresh**: extend community-exports past msg 11823 via the groups.io API | data pull | minutes | The four post-snapshot citations (11848/56/11901/02) become locally verifiable; two walls lose their only-unverifiable-source caveat |

Suggested order: E7, E5, E1 (cheap, close CONFIRMED findings), then E2/E6/E8,
then E3, then E4 (the only genuinely expensive one).

## 5. What was deliberately not proposed

Folding any page (bar not met); a per-kind layout branch (re-judged: vocabulary
was the defect); a new synthesis page (walls-and-methods is it); charting the
in-house ladder (blocked on publication of the strict ladder); anything
requiring vault material.
