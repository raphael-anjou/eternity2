# Cross-cutting review: wall interdependence in /research/why

Scope: EN pages under `web/content/research/why/*.mdx`, judged only against what
is published on-site plus the repo's `research/topics|experiments` results and
the local community archive (`research/community-exports/messages.jsonl`). The
private vault is out of scope.

The section presents its walls as parallel entries (walls-and-methods lists
"four walls, in one line each"; the index lists them as a flat bulleted set).
The question here is which logical *edges between walls* are (a) already stated,
(b) supportable from on-site evidence and could be stated, or (c) plausible but
open and therefore belong on /research/open-problems as a question, not as an
asserted fact.

Verdict key:
- **(a) STATED** — the connection is already asserted somewhere on-site.
- **(b) SUPPORTABLE** — on-site published evidence backs it; it could be stated
  as fact but currently isn't (or only obliquely).
- **(c) OPEN** — plausible, not backed to fact tier by anything published;
  belongs on open-problems as a question.

---

## Edge 1 — Piece theft is the local mechanism of global distinctness (area law)

**Candidate claim.** The piece-theft failure (a scarce piece spent in the wrong
place starves a later cell) is the microscopic, cell-level face of the entropy
area law (the use-each-piece-once rule collapsing the count of distinct partial
boards past ~80 cells). Both are the same constraint — use-once scarcity — seen
at two scales.

**On-site evidence.**
- `entropy-area-law.mdx`: "the wall isn't in the part that looks hard, matching
  colors; it's in the quiet rule that each piece is used once, whose cost grows
  with area." The whole page's thesis is that the use-once constraint, not
  edge-matching, is the hardness.
- `piece-theft.mdx`: the entire mechanism is a use-once accounting failure —
  "the single piece that could ever serve an upcoming cell was used for
  something else." It explicitly frames the trap as global-supply-fine /
  local-allocation-fatal: "Globally the supply is fine; the failure is one
  scarce piece misallocated, not a shortage."

**Verdict: (b) SUPPORTABLE, currently UNSTATED.** Both pages independently name
the use-each-piece-once rule as the seat of hardness, but neither links to the
other, and neither says piece theft is the area law "in the small." The logical
identity — a scarce-piece misallocation is precisely a step of the distinctness
collapse — is directly supported by both pages' own published framing. It can be
stated as fact because it is a definitional relationship (both are the use-once
constraint), not an empirical conjecture. Note: piece-theft is rigor `measured`,
entropy-area-law is rigor `measured`; asserting their *shared mechanism* does
not upgrade either tier, because the shared mechanism (use-once scarcity) is the
`proven` structural fact both rest on. The pages do not currently cross-link
(piece-theft relates only to no-forced-moves and PRIOR; entropy-area-law relates
to phase-transition/prune-vs-speed/forbidden-patterns). This edge could be added
to the connective prose and as a `related` cross-link, at (b) tier.

---

## Edge 2 — Rigidity is a consequence of entropy / distinctness collapse

**Candidate claim.** Record boards are locally frozen (rigidity wall) *because*
distinct partial boards collapse past ~80 cells (area law): the near-empty set
of genuinely-distinct completions is why no small rearrangement finds a better
one.

**On-site evidence.**
- `entropy-area-law.mdx` explicitly reaches toward rigidity: "Eighty cells is
  not arbitrary. It's the size of the smallest moves that separate the best
  known boards," and closes with "See why basin-hopping is impossible"
  (sigma-cycles). So it ties the ~80-cell scale to the size of inter-record
  moves.
- `sigma-cycles.mdx` corroborates the number from the other side: the giant
  cycles between records are 80–154 cells — i.e. exactly in / above the
  distinctness-collapse regime.
- `rigidity-wall.mdx`, however, derives rigidity from an *independent* source:
  MIP halo-optimality proofs (exact, region-by-region), plus annealing (benj39100)
  and SAT (Millilaw) corroboration. It does **not** claim entropy/distinctness as
  the *cause*; it treats rigidity as an exactly-measured fact in its own right.

**Verdict: (c) OPEN.** The ~80-cell coincidence (distinctness collapse scale =
smallest inter-record move size) is genuinely suggestive and *is* pointed at on
the entropy page, but nowhere on-site is the causal claim "boards are rigid
*because* of the area law" actually made or proven. Rigidity is established
independently (by MIP), and the area law is a first-moment / distinctness
estimate; a causal reduction of one to the other is a hypothesis, not a result.
The honest form is a question: "Is local rigidity a consequence of the area-law
distinctness collapse, or an independent fact that merely shares a length
scale?" That belongs on open-problems. Crucially, **no page currently asserts
this as fact** — the entropy page only observes the shared scale and links
onward, which is legitimate. So there is no (c)-as-fact finding to report here;
the recommendation is to add the *question* to open-problems, not to fault a
page.

---

## Edge 3 — Forbidden patterns are the microscopic face of the area law

**Candidate claim.** The 99.72%-forbidden 2×2 density (forbidden-patterns) is
the small-scale, color-plus-piece version of the area-law distinctness collapse
(entropy-area-law): both say realizable patches are exponentially rare as the
patch grows.

**On-site evidence.**
- `forbidden-patterns.mdx` already shows the growth-with-size table: pairs
  38.96% forbidden → L-of-three 83.26% → 2×2 99.72%. That *is* an
  area-law-shaped decay (feasibility falling fast as the shape grows), computed
  exactly.
- `entropy-area-law.mdx` is the formal statement: `ρ(n) ≈ exp(−α n²)`,
  distinctness collapsing in the area. The two pages are already cross-linked
  (`forbidden-patterns` relates to `entropy-area-law` and vice versa).
- BUT the two measure subtly different things. Forbidden-patterns counts
  color-match feasibility of small placements (its 2×2 count is over
  distinct-piece placements, matching-only). Entropy-area-law separates the two
  rules explicitly: it first counts color-valid patches (rich, exponential
  growth), *then* applies use-once to get the ρ collapse. So on the entropy
  page, pure color-matching is the *generous* rule and use-once is the killer;
  on the forbidden-patterns page, color-matching alone already kills 99.72% of
  2×2s. These are consistent (different questions: "fraction of all placements
  that color-match" vs "fraction of color-valid patches that are distinct-piece
  realizable") but they are not the same axis.

**Verdict: (b) SUPPORTABLE with a precision caveat.** The edge "forbidden
patterns and the area law are the same phenomenon at different scales" is
partly supportable — both are exact scarcity-of-small-patches results and are
already cross-linked — but the *clean* version ("forbidden patterns is the
micro face of the area law") over-simplifies, because forbidden-patterns is a
color-matching count while the area law's decay is specifically the *use-once*
overlay on top of color-valid patches. The defensible statement is narrower:
both are exact, exhaustive scarcity counts showing realizable small patches
vanish super-linearly with size; the area law isolates *which rule* (use-once)
does the collapsing. Stateable at (b), but the prose must not conflate the
color-match axis with the distinctness axis. No page currently over-asserts
this, so no finding.

---

## Edge 4 — No-forced-moves sits at / is caused by the hardness peak

**Candidate claim.** The absence of forced moves (branching factor never
collapses; 73–137 legal neighbours) is a manifestation of sitting at the
phase-transition hardness peak (~17 interior colors → about one solution).

**On-site evidence.**
- `no-forced-moves.mdx` relates to `phase-transition.mdx` (in `related`), and
  its thesis (lots of local freedom, almost no global consistency) is thematically
  adjacent to the peak.
- `prune-vs-speed.mdx` is the page that *does* place both under one roof: it
  lists no-forced-moves and the hardness peak as two of the "four walls [that]
  are, underneath, all the same statement: there is nothing local to prune on."
  That is a stated grouping, but it is a grouping-under-a-common-theme, not a
  causal claim that the peak *produces* the high branching factor.
- Mechanistically they are distinct: high branching factor (no-forced-moves) is
  a per-cell candidate count on the piece set; the peak (phase transition) is a
  statement about the *number of solutions* at 17 colors. A puzzle could in
  principle have a high branching factor without being at the one-solution peak,
  and vice versa. No page derives one from the other.

**Verdict: (c) OPEN for the causal form; (a) STATED for the thematic grouping.**
The *thematic* union ("both are versions of 'nothing local to prune'") is
already stated on prune-vs-speed and walls-and-methods — that is (a) and fine.
The stronger *causal* claim ("no forced moves *because* the puzzle is tuned to
the one-solution peak") is not made on-site and is not backed to fact; it is a
plausible hypothesis that belongs on open-problems as a question. No page
asserts the causal form as fact, so no finding — the thematic version is
correctly scoped as "same statement," not "one causes the other."

---

## Edge 5 — Sigma-cycles are the reason local search saturates where DFS also saturates

**Candidate claim.** The sigma-cycle structure (the only move between records is
one indivisible 80–154-cell cycle, every partial application scoring worse) is
the common reason *both* local search (annealing/hill-climbing) and
constructive DFS/beam saturate at the same ceiling — a single mechanism behind
two independently observed walls.

**On-site evidence.**
- `sigma-cycles.mdx` explicitly unifies with rigidity: "Together with the
  rigidity wall, this closes off the two obvious escape routes at once. You
  can't climb out of a good board locally, and ... you can't hop to a neighbour
  either." That is a stated link between local-improvement failure (rigidity)
  and basin-hopping failure (sigma-cycles).
- `rigidity-wall.mdx` reciprocates: its closing insight box routes to
  sigma-cycles ("If you can't improve a board locally, maybe you can hop ...
  That fails too") and it already gathers *three methods, one answer* (MIP,
  annealing, SAT) hitting the same frozen core, plus Millilaw's budget argument
  (48-cell repair reach vs 225-cell inter-basin gap).
- The DFS/beam side: `walls-and-methods.mdx` observes that from-scratch/beam
  producers (PRIOR, KEYRING, GAUNTLET) "saturate like the others" and that the
  Millilaw diversity survey found the standard arsenal keeps rediscovering the
  same basins. So DFS/constructive saturation is documented.

**Verdict: mixed — (a) STATED for local-vs-hop; (c) OPEN for the full
local+DFS-under-one-mechanism claim.** What is *already stated* (a): rigidity +
sigma-cycles jointly close local-improvement and basin-hopping — this is on both
pages and is fine. What is *not* established to fact (c): that sigma-cycle
geometry is the *cause* of *constructive DFS/beam* saturation. The sigma-cycle
result is computed only between finished record boards (permutation composition),
not about the constructive search tree; and sigma-cycles is rigor `conjectured`
on a three-pair sample ("three pairs is a small sample and it is not proven to
hold in general"). Extending it to "this is why DFS also caps out" is a
reasonable hypothesis but is not proven and is not asserted on-site. It belongs
on open-problems as a question. Importantly, sigma-cycles.mdx is *careful* here:
it says "This is a **plausible** part of why the 470 record has stood," which is
correctly hedged, not asserted. So no finding.

---

## Summary table of edges

| # | Edge | Verdict | Where evidence lives |
|---|---|---|---|
| 1 | Piece theft = local mechanism of area-law distinctness | (b) supportable, unstated | piece-theft + entropy-area-law both name use-once as the seat of hardness |
| 2 | Rigidity is a consequence of the area law | (c) open (shared ~80-cell scale only) | entropy-area-law observes the scale; rigidity derived independently by MIP |
| 3 | Forbidden patterns = micro face of the area law | (b) supportable, with color-vs-distinctness caveat | already cross-linked; both are exact small-patch scarcity counts |
| 4 | No forced moves ⇐ hardness peak | (a) thematic union stated; (c) causal form open | prune-vs-speed groups both under "nothing local to prune" |
| 5 | Sigma-cycles = why both local search and DFS saturate | (a) local+hop stated; (c) DFS-cause open | sigma-cycles + rigidity mutually linked; DFS extension unproven |

**Findings for the review (pages asserting a (c)-grade edge as fact): NONE.**
Every page that touches a (c)-grade connection hedges it correctly:
- sigma-cycles.mdx says "a **plausible** part of why the 470 record has stood"
  and flags the three-pair sample as not proven in general.
- entropy-area-law.mdx only *observes* the shared ~80-cell scale and links
  onward; it does not claim rigidity is caused by the area law.
- prune-vs-speed.mdx groups the four walls under one *theme* ("nothing local to
  prune"), which it states as "underneath, all the same statement" — a
  thematic, not causal, unification, and defensible as such.

So the section's rigor discipline on interdependence is sound: the stated edges
are (a)/(b)-grade, and the (c)-grade causal reductions are either hedged or
absent. The gap is not over-claiming; it is *under-linking* — edges 1 and 3 are
supportable-to-fact but the connective tissue does not yet spell them out, and
edges 2/4/5's open causal forms are not yet routed to open-problems as
questions.

---

## Verifiability note (citations)

The message numbers I could check against the local archive
(`research/community-exports/messages.jsonl`, which covers msg_num 1..11823) all
verify and match their on-site descriptions: 1947 (Owen "hardest puzzle"
criteria), 4909 (Owen rectangle phase-transition counts), 5027 (Al, 20 void
corner-pairs of 289), 10045 (McGavin 469 announcement). Four load-bearing
citations sit **beyond the archive's range** and could not be verified locally:
11848 and 11856 (prune-vs-speed: McGavin/@95A31 on pruning cost),
11901 and 11902 (rigidity-wall/mismatch-geometry: Zamofing top-band residual and
benj39100 GPU-annealing frozen-core). These back the *corroboration* passages
(external method agreement), not the primary MIP/area-law claims, so they do not
change any interdependence verdict; they are flagged only so a reviewer knows
the local archive cannot confirm them and a groups.io check is needed.

---

## Connective-tissue prose (publication-ready, states only (a)+(b), routes (c) to open-problems)

Drop-in paragraph for the section index or walls-and-methods, after the
four-walls list. No em dashes.

> The four walls are not four separate difficulties; several are the same
> constraint seen at different scales. Piece theft, where one scarce piece spent
> early starves a cell rows later, is the cell-level face of the area law: both
> are the use-each-piece-once rule, the quiet second rule that carries all the
> hardness, one felt as a single dead cell and the other measured as the
> collapse of genuinely distinct boards past about eighty cells. Forbidden
> patterns sit one level lower still, as the exact, exhaustive count of how
> quickly small patches run out of legal arrangements as they grow, the same
> scarcity the area law then sharpens by adding the use-once overlay on top of
> mere colour matching. And the two escape routes close together rather than
> apart: rigidity says you cannot improve a record board locally, and the
> sigma-cycle structure says you cannot hop to a neighbouring one either,
> because the nearest better board is one indivisible move away. Whether the
> deeper reductions hold, whether local rigidity is itself a consequence of the
> distinctness collapse that shares its eighty-cell scale, whether the
> high branching factor is forced by sitting at the one-solution peak, and
> whether the sigma-cycle geometry is what caps constructive search the same way
> it caps local repair, are real questions rather than settled facts, and they
> live on the [open problems](/research/open-problems) board with the rest of
> the frontier.

### Optional shorter variant (index-sized)

> These walls interlock. Piece theft is the area law felt in a single cell:
> both are the use-each-piece-once rule that carries the hardness, one as a
> lone dead cell and one as the collapse of distinct boards past about eighty
> cells. Forbidden patterns are the exact small-patch scarcity underneath both.
> And the exits close in pairs: you cannot polish a record board upward
> (rigidity) and you cannot hop to a better neighbour (sigma-cycles), because
> the nearest better board is one indivisible move away. Whether the deeper
> causal links hold, rigidity as a consequence of the area law, or the
> sigma-cycle geometry as the reason constructive search also caps, are open
> questions on the [open problems](/research/open-problems) board, not claims
> made here.

### Suggested open-problems entries for the (c) edges

Three questions to add to /research/open-problems (as questions, not assertions):

1. **Is local rigidity a consequence of the area-law distinctness collapse?**
   The smallest inter-record moves and the distinctness-collapse scale are both
   ~80 cells. Is that a causal link (rigidity follows from the area law) or a
   coincidence of length scale? Wall: rigidity + area law.
2. **Is the high branching factor forced by the one-solution peak?** No-forced-moves
   and the phase-transition peak are grouped as "nothing to prune," but no
   derivation ties the 73–137 branching count to sitting at ~17 colours. Wall:
   no-forced-moves + hardness peak.
3. **Does the sigma-cycle wall explain constructive DFS/beam saturation, not
   just local-search saturation?** Sigma-cycles are computed between finished
   record boards; extending the mechanism to why beam/DFS producers also cap at
   458–463 (rather than just local repair) is unproven. Wall: sigma-cycles +
   rigidity, diversity axis.
