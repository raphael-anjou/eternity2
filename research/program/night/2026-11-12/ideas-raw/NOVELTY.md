# Novelty checks (scouts, 2026-11-12)

One block per proposal: nearest explored cell, whether the named missing
mechanism holds, literature, verdict. Condensed by the lead from the scout
reports; "unchecked" means the scout did not verify it.

## game-tree-search.md (proof-number search, revocable branching cells)

- Nearest cells: "Fixed / dynamic fill orders, comb search" (saturated) and
  "No-good learning, shared transposition tables" (mapped). A16 and A22 fix one
  cell per node. No repo hit for "proof-number" or "df-pn"; the MCTS section of
  `web/content/research/build/dead-ends.mdx` (line 473) is a different mechanism.
- Missing mechanism: partly yes. Revocable cell choice is measured nowhere here;
  the headroom claim is untested, and dom/wdeg is not tested here either.
- Literature: Allis et al. 1994 and Kishimoto et al. 2012 as cited (not
  re-checked by the scout); Nagai 2002 unchecked. No use of proof-number search
  for CSP or SAT branching found.
- Verdict: **new (provisional)**.

## compilers.md (cut-keyed trace partitioning)

- Nearest cell: "Arc consistency, Régin all-different" (mapped). No repo hit
  for singleton consistency, POAC or constructive disjunction. Possible overlap
  with `pair-glasses-compilers.md` (cavity lookahead).
- Missing mechanism: partly. Persistent partitions keyed on cut edges are not in
  the repo; per-key pruning is no stronger than cell POAC, as the proposal says.
- Literature: Rival and Mauborgne 2007 and Bennaceur and Affane 2001 confirmed.
  No prior use on edge matching found. The stated difference from POAC is
  accurate.
- Verdict: **variant with a named missing mechanism**. Its POAC control can
  still kill it.

## tiling-words.md (band consistency by tile transducers)

- Nearest cells: "Arc consistency" (mapped) and "Pattern databases, macro-tiles,
  local cluster enumeration" (dead ends). `dead-ends.mdx` line 272: "Patch
  databases indexed by frontier colors and remaining pieces are untested here."
  No band or transducer hit in the repo.
- Missing mechanism: partly yes. Cycle-exact consistency inside a band is not
  measured. The filter does not enforce piece uniqueness inside the band, so it
  may be weak.
- Literature: Jeandel and Rao confirmed, transducer view confirmed. Kari 1996
  and Pesant 2004 not verified by the scout. No 2D automaton constraint on edge
  matching found.
- Verdict: **variant with a named missing mechanism**.

## pair-glasses-compilers.md (cavity summaries at the point-to-set scale)

- Nearest cells: "No-good learning, shared transposition tables" (mapped) and
  "Pattern databases, macro-tiles, local cluster enumeration" (mapped).
  `dead-ends.mdx` line 272 leaves frontier-and-pool patch databases untested.
  Nothing under `research/experiments/` on windows, cavities or patches.
- Missing mechanism: partly. A bounded-window summary keyed on the boundary is
  untested. The reuse mechanism is the same as in `pair-synthesis-tiling.md`.
  The glass field only chooses the window size, so reuse does not need it.
- Literature: Montanari and Semerjian, and Chu et al. 2010, confirmed. Reps et
  al. and Bouchaud and Biroli not checked. No edge-matching application found.
- Verdict: **variant with a named missing mechanism**.
- Lead remark for the tournament: `build/reduce/nogood-learning.mdx` lines 181
  to 186 describe Mocsi's Failure Lookup Table, keyed on outer colours (found by
  the scout for the next proposal). That is close to a boundary-keyed
  infeasibility summary; the judge must read it before ranking this proposal.

## pair-synthesis-tiling.md (boundary-word lemmas learned from refutations)

- Nearest cell: no-good learning, and nearer than the proposal admits.
  `build/reduce/nogood-learning.mdx`: lines 209 to 214, puzzle-level no-goods
  "valid for every solver"; lines 244 to 248, deletion-based minimisation of
  unsatisfiable cores (a 28-piece core); lines 181 to 186, Mocsi's Failure
  Lookup Table keyed on outer colours. The premise "learned keys name cells" is
  therefore partly false.
- Missing mechanism: partly. Shape-and-boundary-word keys with a pool-count
  predicate, invariant under translation and rotation, are not on the site.
  Program synthesis supplies the generalisation; the tiling field only justifies
  canonicalisation, so one field is enough.
- Literature: Feng et al. PLDI 2018 confirmed. Closest in CSP: Katsirelos and
  Bacchus, generalized nogoods, CP 2003 (seen in search, not read).
- Verdict: **variant with a named missing mechanism, weak**.

## pair-or-compbio.md (piece-priced frontier bound with Carrillo-Lipman banding)

- Nearest cells: "Tensor-network counting" (dead end) and "LP / ILP
  relaxations". `dead-ends.mdx` lines 187 to 197: a per-piece Lagrangian solved
  generated 6x6 in five minutes, ran out of memory around 3 GB, and "the
  truncation needed to fit may discard too much".
- Missing mechanism: yes. A solution-preserving truncation is what that page
  names as lacking. Both fields are needed. The claim that the banded bound
  equals the unbanded one is argued, not tested.
- Literature: Carrillo and Lipman confirmed; Fisher not checked. Nearest prior
  work: Konagurthu and Stuckey, incremental Carrillo-Lipman bounds (2006);
  Lagrangian relaxation on decision diagrams, arXiv 1908.07076. No edge-matching
  application found.
- Verdict: **variant with a named missing mechanism**.

## pair-cryptanalysis-matching.md (auction prices plus exact score-band counting)

- Nearest cells: "Tensor-network counting, lattice reduction" (dead ends) and
  "LP / ILP relaxations". Nearest measurement: `dead-ends.mdx` lines 187 to 190,
  a per-piece Lagrangian penalty on exact contraction (generated 4x4 and 6x6).
  No run tests auction prices.
- Missing mechanism: partly. An exact pre-search count of the relaxed boards
  that outrank the true one, plus a band enumerator, is on no page or README.
  Pricing by itself is covered by the Lagrangian entry.
- Literature: Martin et al. ASIACRYPT 2015 and Bertsekas 1988 confirmed. No
  application to edge matching found. "p = 0 is already dual-optimal" is the
  proposal's own derivation, not checked.
- Verdict: **variant with a named missing mechanism** (the band count and
  enumerator).

## cryptanalysis.md, with lead L2 merged (dissection into seam-guessed bands)

- Nearest cell: "Meet in the middle (BANDSAW)" (mapped, open).
  `meet-in-the-middle.mdx` lines 234 to 253 price four quadrants at 10^80
  against 10^62 and say "Multi-way meet in the middle collapses back into the
  single-sided search". The bulge model is uncommitted (line 241).
- Missing mechanism: partly. BANDSAW is a single seam, with no guessed seam and
  no multi-band split (`bandsaw.mdx` lines 54 to 60).
- Literature: Dinur, Dunkelman, Keller, Shamir, CRYPTO 2012 confirmed.
  Schroeppel and Shamir not searched. No dissection on edge matching found.
- Verdict: **variant with a named missing mechanism** (the guessed seam).

## lead L1 (relaxed decision diagrams as a score bound)

- Nearest cells: "Break-tolerant DFS with scheduled late mismatches"
  (saturated) and "LP / ILP relaxations". `engine/src/break_solver.rs` lines 15
  to 24 allow one break per fixed `break_positions` cell and track only
  `breaks_used`: no bound. `edge-slipping.mdx` line 275 calls the method "a
  scheduled, budgeted redefinition". The site describes no bound for Verhaard or
  Blackwood. BANDSAW prunes on exact suffix lower bounds, on endgame bands only
  (`bandsaw.mdx` lines 99 to 101).
- Missing mechanism: yes. The premise holds for the engines the site describes.
  Risk confirmed at the root: `dead-ends.mdx` lines 673 to 676 say a board with
  repeated pieces "matches all of them easily", so a distinctness-relaxed bound
  is loose at the root. Whether it is loose at deep nodes is unmeasured.
- Literature: Bergman, Cire, van Hoeve, Hooker 2016 (DOI
  10.1287/ijoc.2015.0648) and Cire and van Hoeve 2013 (DOI
  10.1287/opre.2013.1221) confirmed. No edge-matching application found.
- Verdict: **variant with a named missing mechanism** (a score bound inside
  break-tolerant DFS).

## statistical-inference.md (adaptive multilevel splitting on local-search runs)

- Nearest cells: "Simulated annealing, parallel tempering" (saturated), A4
  (killed), A11 (proposed). Nearest page: the beam-width lab page (line 49),
  resampling in beam construction, not on local-search trajectories.
- Missing mechanism: partly. Killing by running maximum and resuming at first
  crossings is in no cell; A11 already covers a replica population.
- Literature: Bréhier et al. 2016 confirmed; Cérou and Guyader 2007 and Aldous
  and Vazirani not checked. Closest prior work: Botev and Kroese, generalized
  splitting (Stat. Comput. 22, 2012, SAT counting; MCAP 10, 2008, combinatorial
  optimisation). The proposal's sentence "none gives an unbiased probability" is
  inaccurate: splitting estimators do.
- Verdict: **variant with a named missing mechanism**.

## coding-theory.md (information-set decoding of an incumbent)

- Nearest cell: "Fake 480 and repair, repair seeded by exact prefix" (mapped).
  `dead-ends.mdx` lines 681 to 703 pin a contiguous exact prefix (184 cells) and
  end at 424. The repair-study page (lines 90 to 95) destroys 12 random cells
  and refills greedily. A14 is the Hamming ball.
- Missing mechanism: partly yes. No page keeps a sparse random quarter and
  re-solves the rest exactly with propagation.
- Literature: Prange 1962 and Fossorier and Lin 1995 confirmed. No information-
  set decoding of SAT or CSP incumbents found. Shaw 1998 and Danna et al. 2005
  not checked.
- Verdict: **variant with a named missing mechanism**.

## lead L3 (price 471 by rare-event splitting)

- Nearest: AGENDA A2 (proposed) and "Restart portfolios, heavy tails"
  (saturated). No splitting estimator on any page or README.
- Missing mechanism: partly. A2 extrapolates by a GEV fit with no unbiasedness
  check. Whether DFS depth is a usable level is untested.
- Literature: Glasserman et al. 1999 confirmed; Cérou and Guyader not checked.
  Closest prior work: Botev and Kroese, as above.
- Verdict: **variant with a named missing mechanism**.

## lead L4 (a partial-match ladder)

- Nearest: the scaling-ladder lab page (lines 46 to 60): planted boards at
  N = 8, 10, 12, 14, a proven ceiling, a verified score and a ratio to the
  ceiling, on a fixed 22-colour palette with a 12 s budget, not at critical
  colours. PROTOCOL_TARGETS_PROPOSAL.md section 3 only says "best matched edges
  at a fixed CPU budget". `hard-ladder` measures nodes to first solution.
- Verdict: **does not exist**. The scaling ladder covers part of it (planted
  ceiling, verified score) but not critical colours, work to optimum minus k, or
  the ordering check against the 16x16. A build should start from that page.
