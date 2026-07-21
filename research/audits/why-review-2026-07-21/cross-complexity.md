# Cross-cutting read: adversarial COMPLEXITY-FRAMING review

Reviewer stance: complexity theorist reading for a professional audience.
Target pages:
- `web/content/research/why/how-hard-is-this-instance.mdx` (EN, + .fr/.es siblings)
- `web/content/research/why/complex-theory.mdx` (EN, + siblings)
- their surfacing on `web/content/research/why/index.mdx`

Governing concern: complexity classes are properties of problem FAMILIES. A single
instance is not NP-complete; worst-case hardness of edge-matching does not by itself
explain THIS instance's difficulty. Counting arguments must carry the right caveats
(constrained vs unconstrained, arrangements vs search states). Titles must not
overpromise.

Bottom line up front: **both pages get the hard part right, and one of them
(`how-hard-is-this-instance`) is a model of exactly the discipline this review exists
to enforce.** There is no NP-completeness-implies-this-instance-is-hard fallacy on
either page. The findings below are minor and mostly about a single counting caveat
and a couple of framing softenings; nothing rises to a correctness defect.

---

## A. `how-hard-is-this-instance.mdx` — the family/instance distinction

This is the page most at risk of the category error, and it is instead **built around
refuting it.** Verified claim by claim:

1. **Title** — `"Is this instance NP-complete, and how do I encode it?"` The title poses
   the exact question a naive reader asks and the body answers "no, an instance has no
   complexity class." A title that poses a question the page then dismantles is not
   overpromising; it is the correct pedagogical framing. NOT a fault. **SAY SO: the
   title is careful.**

2. **Description / metaDescription** — "Edge matching is NP-complete as a family, but
   that says nothing about one fixed 16×16 board: a single instance is a constant, not a
   problem." This is precisely correct and, unusually, gets the caveat into the SEO
   surface itself. **SAY SO: careful.**

3. **"First, the category error" section** (lines 39–57) — Defines NP-completeness as a
   property of "an infinite family of instances indexed by a size parameter n," gives the
   edge-matching family as "given n, a set of square tiles, and a colour alphabet, decide
   whether the tiles tile an n×n frame..." This is the correct decision-problem framing,
   and it matches what Demaine & Demaine 2007 actually prove (verified below). The "single
   bit is a constant / `return true` / is the number 17 polynomial-time" analogy is the
   textbook correct statement of why "is this instance NP-complete?" is ill-formed.
   **SAY SO: this is the single best treatment of the family/instance distinction on the
   site, and correct.**

4. **"What is actually true" section** (lines 63–88) — Explicitly keeps two statements
   apart: (a) worst-case hardness of the family ("says nothing about how a specific input
   behaves"), (b) empirical hardness of this instance ("a claim about this board, not
   about the family"). It then states the both-directions caveat a complexity theorist
   would demand: "A family can be NP-complete while a given instance is trivial (many
   are), and an instance can be brutally hard to solve in practice even inside a family
   that is polynomial." That sentence is exactly the guard against the inference the
   review is hunting for. **SAY SO: careful, and complete — it names both failure modes
   of the naive inference.**

5. **The `Callout` "one-line version"** (lines 90–95) — three Q&As that separate "Is
   Eternity II NP-complete? No: an instance has no complexity class" from "Is the
   edge-matching problem NP-complete? Yes." Correct and clean.

6. **"Where this leaves you"** (lines 209–221) — closes with "complexity classes describe
   families, and this board is a fixed input with a fixed answer... the difficulty you
   actually feel is empirical." No backsliding into an instance-hardness-from-family
   claim. Careful.

### The one counting-caveat gap (the substantive finding)

The `1.115×10^557` figure appears three times (lines 72, 93, 216) as "distinct
piece-and-rotation arrangements" / "the search space is ~10^557 wide" / "a 10^557-wide
space over a nearly empty solution set."

Cross-checked against the site's own derivation in
`web/content/research/build/known-facts.mdx:170`:

> Corners in corners, edges on the border (orientation forced), clue pinned:
> `4! · 56! · 195! · 4^195` ≈ `1.115 × 10^557` — "computed; matches the widely quoted
> Wikipedia figure"

So `10^557` is the **constrained** count: it already assumes corners in the four
corners, edges on the border with grey-out orientation forced, and the clue pinned — it
is NOT the unconstrained arrangement count. The unconstrained count is the OTHER row in
that same table: `256! · 4^256 ≈ 1.15 × 10^661`. The two differ by ~10^104.

On `how-hard-is-this-instance.mdx` the `10^557` is introduced with no indication that it
is the border-aware, clue-pinned figure rather than the raw one. A professional reader
who knows `256! · 4^256 ≈ 10^661` will notice the number is smaller and not be told why.
More importantly for this review's mandate: the phrase "the search space is ~10^557
wide" quietly slides from **arrangements** (a static count of complete labelled boards
satisfying the placement constraints, ignoring edge colours) to **search-space width**
(a dynamic property of a backtracking tree). Those are different objects. The
complex-theory page is explicit that the tree's widest point is ~10^45 (not 10^557), and
that the 10^557-style total-arrangement count is not the number of search states a good
solver visits. `how-hard` uses `10^557` loosely as "the search space," which — while
rhetorically fine for a lay reader — is the one place a complexity theorist would want
the arrangements-vs-search-states distinction drawn, especially since the page otherwise
draws distinctions so scrupulously.

This is a **minor** finding: the number is correct and sourced, the claim "wide space,
nearly empty solution set" is directionally true, and the page links to complex theory
for the real tree-width estimate. But "distinct piece-and-rotation arrangements" should
say it is the constrained (border-and-clue-pinned) count, and "the search space is
~10^557 wide" conflates an arrangement count with a search-tree width the site itself
elsewhere puts at 10^45. One clause would fix both.

### Encoding sections (SAT / XCC / ILP / max-clique)

Out of the strict complexity-framing mandate, but checked for stray hardness claims:
these are careful. "complete SAT solvers stall on the full board... the instance is [the
bottleneck]" is an empirical claim, correctly flagged as empirical. The Algorithm
X-vs-Algorithm C (XCC) distinction is technically correct (plain exact cover cannot
express the colour-agreement constraint; XCC secondary items can). No complexity-class
overreach.

### Demaine & Demaine 2007 citation — verified

Source cited: Demaine & Demaine, "Jigsaw Puzzles, Edge Matching, and Polyomino Packing:
Connections and Complexity," Graphs and Combinatorics 23, 2007, DOI
10.1007/s00373-007-0713-4. Confirmed via the authors' own listing (erikdemaine.org) and
the Springer record: the paper proves signed AND unsigned square-tile edge-matching
puzzles are NP-complete (and equivalent to jigsaw and polyomino packing). The page's use
— "edge matching is NP-complete" for the family — is a faithful rendering, not an
overreach. The page does NOT claim Demaine-Demaine says anything about the E2 instance;
it explicitly says the opposite. **Citation accurate; usage accurate.**

### Ansótegui et al. citation — verified in role

Cited as the EMPIRICAL case ("building solver benchmarks from it and measuring the
difficulty directly rather than appealing to the general theorem"). This is the correct
role for that paper ("How Hard is a Commercial Puzzle: the Eternity II Challenge," CCIA
2008) and the page does not inflate it into a theorem. Careful.

---

## B. `complex-theory.mdx` — counting the search before you run it

This page is `rigor: conjectured` and `contribution: exposition`, and it repeatedly
labels its own status. It is a first-moment (expected-count) model, and the page says so.
Checked for counting-caveat discipline:

1. **The independence caveat is stated up front** (lines 61–64): "It is an average, not a
   true count: it assumes the 22 edge colours are drawn independently, which they aren't
   (four edges are bolted to one rigid piece)." This is exactly the caveat the model
   needs, and it is the same caveat Brendan Owen himself states in the primary source
   (msg 8125: the closed form "applies to an 'ideal' puzzle... E2 is only an
   approximation"). **SAY SO: careful — the caveat matches the primary source verbatim in
   spirit.**

2. **"What it can't see" section** (lines 162–168): "Complex theory is a first-moment
   estimate, so it is blind to one thing: whether the many counted partial boards are
   genuinely distinct... use complex theory to choose orders and read the tree's shape,
   never as a true count or a bound." This is the correct, explicit disclaimer that the
   count is neither exact nor a bound. **SAY SO: careful — it pre-empts exactly the misuse
   a reviewer would flag.**

3. **Arrangements vs search states** — this page is the one that gets it RIGHT: the
   plateau width is given as ~10^45 "ways to extend" (line 130-133), which is a
   branching/tree-width quantity, clearly distinguished from a total-arrangement count.
   The funnel (growth 1→10^27, plateau ~10^45, collapse →10^4) is presented as
   expected-branch width at each depth, not as a count of final boards. This is the
   distinction `how-hard` blurs and this page keeps.

4. **Peak-depth closed form `256(1−1/e) ≈ 161.8`** (lines 176–178) — verified against
   primary source msg 8125 (Brendan Owen, "Magic of 161", 2010-11-13): "256 x (1 - 1/e) =
   161.8," proven for an ideal puzzle. Accurate, correctly attributed, and the "ideal
   puzzle" caveat is carried.

5. **The 14,702 figure** (lines 71–73, 181–182) — verified against msg 8924 (Peter
   McGavin, 2011-06-12): "Estimated number of E2 solutions ... with piece 139 constraint
   = ... 14702." The page's "published ... as early as 2011" is exactly right. The
   "≈1 solution with all five clues / ≈4×10^-8" figure is the model's expected-count
   output and is labelled as such ("This is the formal reason the 5-clue puzzle has a
   single designed solution") — note it says "formal reason" for the DESIGN, consistent
   with the site's binding directive that a 480 EXISTS as a given, not that the model
   proves existence. No overreach.

6. **10×10 validation / "roughly 180 core-years"** (lines 182–187) — verified against msg
   9688 (Peter McGavin, 2017): "nearly 2*10^17 nodes (about 180 core years) and less than
   0.5% of the entire search tree," searching "rows with highest chance of a solution per
   number of nodes first." The page's "landing inside the theory's predictions" is
   supported by the source. Accurate.

7. **"NxM ... eleven orders of magnitude / R^2 ≈ 0.84"** (lines 90–100) — sourced to
   Brendan Owen's groups.io Files studies (NxM_actual_theory.pdf, heuristics.pdf), cited
   in frontmatter. These are external community artifacts; tier is proven-external
   (Owen's own validation), presented as such ("the real basis for trusting the
   estimate"). Not independently re-verified here (files not in the local archive), but
   the claim is attributed, not asserted as this project's own computation. No tier
   inflation.

8. **Reference implementation** — the page says its live estimator "ports [McGavin's 2024
   C], line for line," and the repo carries `research/topics/complex-theory/reference/
   complex_theory.c`. `repro: kind: exact, topic: complex-theory` is therefore backed.
   Good.

### Minor framing notes on complex-theory

- **"Play the sweep... it climbs into the billions, then barely moves for a hundred cells
  ... that flat crawl through an astronomically wide band is the wall"** (lines 104–108).
  The prose describes the counter as an integer that "climbs into the billions," but the
  Three-regimes section puts the plateau at ~10^45. The animated counter presumably shows
  a running figure; "billions" undersells the 10^45 plateau by ~36 orders of magnitude.
  This is a description of a UI animation, not a stated numeric claim, so it is **minor /
  prose** — but a professional reader who reads "billions" and then "10^45" three
  paragraphs later may be briefly confused about which number the animation shows.

- **`rigor: conjectured` is the honest tier.** The model is validated empirically on small
  boards (proven-external via Owen), but as applied to the 256-cell board it is an
  unvalidatable estimate, and "conjectured" is the correct, non-inflated label. **SAY SO:
  the rigor tag is correctly calibrated — the page does not present a measured-single or
  proven tier for an estimate.**

---

## C. index.mdx surfacing

`why/index.mdx` does not itself restate NP-completeness; it routes to the walls pages and
carries the design-story ("engineered to resist cleverness," Selby/Riordan vetting) and a
correction (border-motif separation is "automatic, not engineered"). No complexity-class
claim is made on the index, so nothing to overpromise there. The two target pages are
reached via their frontmatter titles/descriptions, both of which (checked above) carry
the family/instance caveat into the surfaced text. Careful.

One observation: `how-hard-is-this-instance.mdx` has `order: 25` and `complex-theory.mdx`
`order: 30`, so on the section index the NP-completeness page surfaces just before complex
theory — a sensible order, since `how-hard` explicitly hands the "expected-count estimate"
question off to complex theory (line 76-77). No issue.

---

## Verdict against the mandate

- **Family-level NP-completeness conflated with instance-level claims?** NO on both pages.
  `how-hard-is-this-instance` is actively structured to prevent exactly this and states
  the both-directions caveat a specialist would require. `complex-theory` makes no
  complexity-class claim at all; it is an expected-count model, labelled conjectured.
- **"This instance is hard because the family is NP-complete" inference made?** NO. The
  page that raises the question explicitly refutes the inference ("says nothing about how
  a specific input behaves"; "a family can be NP-complete while a given instance is
  trivial").
- **Counting arguments with right caveats?** MOSTLY. complex-theory is exemplary
  (arrangements vs tree-width kept distinct; first-moment/independence/not-a-bound caveats
  all stated). The one gap is on `how-hard`: the `10^557` figure is presented as raw
  "search space width / arrangements" without noting it is the CONSTRAINED (border- and
  clue-pinned) count, and it is loosely equated with search-tree width that the site
  itself elsewhere puts at 10^45. Minor, one-clause fix.
- **Titles overpromise?** NO. The `how-hard` title poses a question the body answers
  correctly; it does not assert instance-level NP-completeness.
- **Citations (Demaine-Demaine, Ansótegui, Owen/McGavin msgs)?** All verified. Demaine
  confirmed via authors' listing + Springer; all six cited groups.io msg_nums (8125, 8924,
  9686, 9688, 5197/5209, 11197) located in the local archive and their content matches the
  page's use, including the 256(1-1/e), 14702, and 180-core-year figures verbatim.
