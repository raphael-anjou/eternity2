# Idea-slot tournament (night 2026-11-12)

Judge: a fresh session that wrote none of the proposals. Inputs: the ten
inventor files, `lead-proposals.md` (L1, L3, L4; L2 merged into
`cryptanalysis.md` and judged as one entry), `NOVELTY.md`, EXPLORED.md,
AGENDA.md (A1 to A22), PROTOCOL_TARGETS_PROPOSAL.md and research-lead.md.
Nothing was run except the Elo arithmetic. Every stall quote was checked
against its file (all 13 are real; several are line-wrapped in the source).

Codes: GT game-tree-search, CO compilers, TW tiling-words, CR cryptanalysis
(+L2), CT coding-theory, SI statistical-inference, OC pair-or-compbio, GC
pair-glasses-compilers, ST pair-synthesis-tiling, CM
pair-cryptanalysis-matching, L1, L3, L4.

Reference numbers used throughout (all `measured`): hard-ladder baseline
`scb-gacolor_ac3_random` solves R6 (6x6 c=5) 30/30 at a KM median of 951
nodes and R7 (7x7 c=6) 16/30 at 921,773 nodes within 1e6
(`research/experiments/hard-ladder/README.md`, baselines table); A22 `bb-mrv`
fails G1 at R6 and R7 (0.80, 0.70) and is slower there (1.64x, 1.85x); 8x8 at
critical colours is unsolved by every solver.

## 1. Cards

**GT (proof-number search over revocable branching cells).**
Stall: A22's critical-rung G1 failure, quote real; the mechanism (cell choice
decided by explored subtrees, not by domain size) bears on it directly, since
A16 measured domain size as the only usable cell signal.
Cheapest falsifier: stage 0 headroom oracle r = refutation size under MRV's
cell over best of 3 cells, 300 nodes at R6/R7, under 2 core-hours.
Likely death: K0 (r near 1) or, if headroom exists, abandoned splits plus
transposition-table memory eat it (K2/K3).

**CO (cut-keyed trace partitioning).**
Stall: same A22 quote plus "pruning fades about two cells out"; the
relational state over cut colours and the piece pool bears on it.
Cheapest falsifier: stage 0 replay on fixed bb-mrv trees with random-key and
POAC controls, under 1 core-hour.
Likely death: wall clock. Up to c^2 = 36 propagations per node against a
baseline already 1.85x slower than scb at R7; the replay counts nodes, not
work.

**TW (band consistency by composed tile transducers).**
Stall: same quotes; the cycle argument (AC is exact only on acyclic graphs,
the grid is all 4-cycles) is correct and bears on the stall.
Cheapest falsifier: dead-node census of k = 1, 2, 3 band filters, about 30
minutes.
Likely death: stage 1 under 5% because distinctness is relaxed inside the
band; the k=1 control is clean.

**CR (dissection into seam-guessed bands, with L2).**
Stall: MITM memory wall (open problem) and the critical rung; quotes real.
The dissection mechanism bears on the memory wall, not on R6 first-solution
counts.
Cheapest falsifier: stage 0 profile of band counts, 2 to 4 core-hours; L2's
b-break enumeration at 6x6 and 8x8 is the version that tests the memory claim.
Likely death: K1 as written fires by construction (DFS needs about 951 nodes
at R6; any half-board count exceeds that), so the kill decides nothing.

**CT (information-set decoding of a local-search incumbent).**
Stall: A22 plus "local search saturated"; the bridge quote (A3's T50, 0.61 vs
0.59 exponent) is real and `measured`.
Cheapest falsifier: stage 1 diagnostic, p_isd(t) against a spatial-window
control and a null incumbent, about 3 core-hours.
Likely death: K1, positional agreement f of PT incumbents far below 0.5
(f^t with t about 11 at R6 is under 1e-3 for f below 0.53); end to end it
cannot beat about 951 nodes at R6 at all.

**SI (adaptive multilevel splitting on local search).**
Stall: A4's PT 0/8 at 8x8 and 10x10 calibration (`measured`) and A10's cheap
saddle (`measured`, framed 5x5/6x6 at 13 to 14 colours); the proposal admits
the two are different regimes.
Cheapest falsifier: 6x6 unbiasedness gate plus 7x7 four-arm run, about 3
core-hours.
Likely death: tie extinction on the score plateau (score is the wrong
importance function); prior art Botev and Kroese generalized splitting is
close, and the proposal's "none gives an unbiased probability" is wrong.

**OC (piece-priced frontier DP with Carrillo-Lipman banding).**
Stall: A22, A17's R7 NO-GAIN (`measured`), and the tensor-network patch stop
(lossy truncation, a single notebook attempt: `argued`). The decision "L > 0"
is preserved exactly by the banding, so the soundness argument holds.
Cheapest falsifier: stage 0 soundness plus 1a signal on sampled dead nodes,
about 1.5 core-hours (10 in total).
Likely death: K1, the Lagrangian bound over integral colour tilings is
feasible at most dead nodes (fractional mixtures of tilings meet the counts);
then K5 at 9x9.

**GC (cavity summaries at the point-to-set scale).**
Stall: A22 quote real; boundary-keyed refutation of a bounded window plus
reuse across prefixes bears on both the scale and the repetition of
refutations.
Cheapest falsifier: stage 0 census of refutation length r and key recurrence
with a plus2 null, under 1 core-hour.
Likely death: K0b (keys carry the pool, so they rarely repeat) or wall clock
of the cavity solves. Pair rule broken (section 3b). Mocsi's Failure Lookup
Table is close prior work but `reported` and abandoned for a bug, not a stop.

**ST (boundary-word lemmas learned from refutations).**
Stall: same quotes; mechanism bears like GC's, with translation and rotation
invariance added.
Cheapest falsifier: stage 1 lemma extraction with planted-path soundness
check, about 4 core-hours.
Likely death: local-core rate under 10% or recurrence too low; the lemma
machinery (deletion minimisation, partial-word matching) is the heaviest build
of the census family. Pair rule broken (section 3b).

**CM (auction prices plus exact score-band counting).**
Stall: A22 JUDGE line 5 and A17, quotes real; the mechanism bears on the
locality of inference.
Cheapest falsifier: stage 0 exact band counts under four price arms, claimed 2
core-hours but the 7x7 cost-window DP needs several GiB per layer pair.
Likely death: the band stays far above DFS node counts. K1 accepts a 2-order
cut from about 1e13 relaxed boards, which still leaves about 1e11 against
about 1e6 DFS nodes; the kill is mis-set.

**L1 (relaxed decision diagrams as a score bound).**
Stall: break-tolerant DFS saturated at 467/470 (`reported`, community
records); absence of any bound in `engine/src/break_solver.rs` lines 15 to 24
is confirmed in code. Bears directly on the 470 regime.
Cheapest falsifier: stage 0 bound gap by rows remaining and width W, 4 to 8
core-hours.
Likely death: the frontier itself must be merged at W of 4096 or less (8^8
frontier states at 8x8 c=8), so the bound goes loose; stage 1 as written is
vacuous (no board scores optimum + 1 on a planted board).

**L3 (price 471 by rare-event splitting on break-tolerant DFS).**
Stall: A2 never run; whether 471 is compute or method is unknown. Not a
recorded stall in the measured sense but the central open question.
Cheapest falsifier: splitting versus direct Monte Carlo on observable 10x10
events, about 5 core-hours.
Likely death: variance blow-up from DFS-depth levels (crossing rates near 0
or 1); it measures 471 and cannot move it.

**L4 (partial-match ladder). Instrument, not a method.**
Stall: every judged experiment of rounds 2 and 3 measured nodes to a first
perfect solution (true, from AGENDA); the scaling ladder covers part (22
colours, 12 s, not critical colours).
Cheapest falsifier: order existing engines on calibrated boards and compare
with their 16x16 order, about 10 core-hours.
Likely death: no size and colour count puts engines in the 2% to 4% band, or
a rank test on 5 engines that a single swap fails.

## 2. Round robin

Criterion (from the brief): which entry better attacks a recorded stall with a
cheaper falsifying test. A stall supported only by a `reported` or `argued`
dead-end claim is a risk to state, never the reason a pairing is lost
(section 5). Being on the 470 regime counts toward "attacks a recorded stall"
because 470 is the stall the 16x16 actually has.

Elo: start 1000, K = 32, expected score 1/(1+10^((Rb-Ra)/400)), three passes
over the 78 pairs in the order below (entries in the order GT, CO, TW, CR,
CT, SI, OC, GC, ST, CM, L1, L3, L4; pair (i, j) for i < j, i outer). The
outcomes are identical in every pass. Two outcomes break the overall order
(pairs 31 and 43); everything else is transitive.

| # | pair | winner | reason |
|---|---|---|---|
| 1 | GT-CO | GT | 2 h headroom oracle decides before any solver is built; k=1 control isolates revocable splits; CO: up to c^2 propagations per node against a baseline already slower in wall clock |
| 2 | GT-TW | GT | 2 h headroom oracle decides before any solver is built; k=1 control isolates revocable splits; TW: relaxes distinctness, so it is provably no stronger than an exact check of the same band |
| 3 | GT-CR | GT | 2 h headroom oracle decides before any solver is built; k=1 control isolates revocable splits; CR: its R6 kill is decided in advance (DFS needs about 951 nodes) |
| 4 | GT-CT | GT | 2 h headroom oracle decides before any solver is built; k=1 control isolates revocable splits; CT: needs positional agreement f > 0.5 that nothing measured supports |
| 5 | GT-SI | GT | 2 h headroom oracle decides before any solver is built; k=1 control isolates revocable splits; SI: its no-barrier premise comes from another regime; generalized splitting is close prior art |
| 6 | GT-OC | GT | 2 h headroom oracle decides before any solver is built; k=1 control isolates revocable splits; OC: 10 core-hours and a bound whose strength at dead nodes is argued only |
| 7 | GT-GC | GT | 2 h headroom oracle decides before any solver is built; k=1 control isolates revocable splits; GC: needs only its compilers half, so the pair rule is broken |
| 8 | GT-ST | GT | 2 h headroom oracle decides before any solver is built; k=1 control isolates revocable splits; ST: 4 h stage 1, one field suffices, heavier build than a census of the same quantity |
| 9 | GT-CM | GT | 2 h headroom oracle decides before any solver is built; k=1 control isolates revocable splits; CM: K1 at 2 orders is lenient against about 1e13 relaxed boards; multi-GiB DP, no path past c^n |
| 10 | GT-L1 | GT | 2 h headroom oracle decides before any solver is built; k=1 control isolates revocable splits; L1: stage 1 target (optimum + 1) is vacuous on planted boards; stage 0 costs 4 to 8 h |
| 11 | GT-L3 | GT | 2 h headroom oracle decides before any solver is built; k=1 control isolates revocable splits; L3: prices 471 but cannot move it; depth levels risk variance blow-up |
| 12 | GT-L4 | GT | 2 h headroom oracle decides before any solver is built; k=1 control isolates revocable splits; L4: 10 core-hours and a 5-engine rank test that one swap breaks; not a method |
| 13 | CO-TW | CO | under 1 h replay on fixed bb-mrv trees, random-key and POAC controls; TW: relaxes distinctness, so it is provably no stronger than an exact check of the same band |
| 14 | CO-CR | CO | under 1 h replay on fixed bb-mrv trees, random-key and POAC controls; CR: its R6 kill is decided in advance (DFS needs about 951 nodes) |
| 15 | CO-CT | CO | under 1 h replay on fixed bb-mrv trees, random-key and POAC controls; CT: needs positional agreement f > 0.5 that nothing measured supports |
| 16 | CO-SI | CO | under 1 h replay on fixed bb-mrv trees, random-key and POAC controls; SI: its no-barrier premise comes from another regime; generalized splitting is close prior art |
| 17 | CO-OC | CO | under 1 h replay on fixed bb-mrv trees, random-key and POAC controls; OC: 10 core-hours and a bound whose strength at dead nodes is argued only |
| 18 | CO-GC | GC | under 1 h census of refutation length and key reuse, with a plus2 null; CO: up to c^2 propagations per node against a baseline already slower in wall clock |
| 19 | CO-ST | CO | under 1 h replay on fixed bb-mrv trees, random-key and POAC controls; ST: 4 h stage 1, one field suffices, heavier build than a census of the same quantity |
| 20 | CO-CM | CO | under 1 h replay on fixed bb-mrv trees, random-key and POAC controls; CM: K1 at 2 orders is lenient against about 1e13 relaxed boards; multi-GiB DP, no path past c^n |
| 21 | CO-L1 | CO | under 1 h replay on fixed bb-mrv trees, random-key and POAC controls; L1: stage 1 target (optimum + 1) is vacuous on planted boards; stage 0 costs 4 to 8 h |
| 22 | CO-L3 | CO | under 1 h replay on fixed bb-mrv trees, random-key and POAC controls; L3: prices 471 but cannot move it; depth levels risk variance blow-up |
| 23 | CO-L4 | CO | under 1 h replay on fixed bb-mrv trees, random-key and POAC controls; L4: 10 core-hours and a 5-engine rank test that one swap breaks; not a method |
| 24 | TW-CR | TW | 30 min band census with a clean k=1 cycle control; CR: its R6 kill is decided in advance (DFS needs about 951 nodes) |
| 25 | TW-CT | TW | 30 min band census with a clean k=1 cycle control; CT: needs positional agreement f > 0.5 that nothing measured supports |
| 26 | TW-SI | TW | 30 min band census with a clean k=1 cycle control; SI: its no-barrier premise comes from another regime; generalized splitting is close prior art |
| 27 | TW-OC | TW | 30 min band census with a clean k=1 cycle control; OC: 10 core-hours and a bound whose strength at dead nodes is argued only |
| 28 | TW-GC | GC | under 1 h census of refutation length and key reuse, with a plus2 null; TW: relaxes distinctness, so it is provably no stronger than an exact check of the same band |
| 29 | TW-ST | TW | 30 min band census with a clean k=1 cycle control; ST: 4 h stage 1, one field suffices, heavier build than a census of the same quantity |
| 30 | TW-CM | TW | 30 min band census with a clean k=1 cycle control; CM: K1 at 2 orders is lenient against about 1e13 relaxed boards; multi-GiB DP, no path past c^n |
| 31 | TW-L1 | L1 | attacks the 470 cell where TW only trims a constant on R7 nodes, both stage 0s cheap |
| 32 | TW-L3 | TW | 30 min band census with a clean k=1 cycle control; L3: prices 471 but cannot move it; depth levels risk variance blow-up |
| 33 | TW-L4 | TW | 30 min band census with a clean k=1 cycle control; L4: 10 core-hours and a 5-engine rank test that one swap breaks; not a method |
| 34 | CR-CT | CT | 3 h, success predicted from the measured T50, spatial-window control; CR: its R6 kill is decided in advance (DFS needs about 951 nodes) |
| 35 | CR-SI | SI | 3 h with a validity gate and a random-kill control; runs unchanged on 16x16 local search; CR: its R6 kill is decided in advance (DFS needs about 951 nodes) |
| 36 | CR-OC | OC | names the stop it removes (lossy truncation) and carries four one-component controls; CR: its R6 kill is decided in advance (DFS needs about 951 nodes) |
| 37 | CR-GC | GC | under 1 h census of refutation length and key reuse, with a plus2 null; CR: its R6 kill is decided in advance (DFS needs about 951 nodes) |
| 38 | CR-ST | CR | settles the uncommitted bulge model and targets the open MITM memory wall; ST: 4 h stage 1, one field suffices, heavier build than a census of the same quantity |
| 39 | CR-CM | CR | settles the uncommitted bulge model and targets the open MITM memory wall; CM: K1 at 2 orders is lenient against about 1e13 relaxed boards; multi-GiB DP, no path past c^n |
| 40 | CR-L1 | L1 | bound aimed at the saturated 470 cell, its absence confirmed in break_solver.rs; CR: its R6 kill is decided in advance (DFS needs about 951 nodes) |
| 41 | CR-L3 | L3 | clean unbiasedness check against direct sampling on observable 10x10 events; CR: its R6 kill is decided in advance (DFS needs about 951 nodes) |
| 42 | CR-L4 | L4 | instrument that makes 470-type methods judgeable at small size; CR: its R6 kill is decided in advance (DFS needs about 951 nodes) |
| 43 | CT-SI | CT | its premise (T50) is measured, SI's no-barrier premise is cross-regime |
| 44 | CT-OC | OC | names the stop it removes (lossy truncation) and carries four one-component controls; CT: needs positional agreement f > 0.5 that nothing measured supports |
| 45 | CT-GC | GC | under 1 h census of refutation length and key reuse, with a plus2 null; CT: needs positional agreement f > 0.5 that nothing measured supports |
| 46 | CT-ST | CT | 3 h, success predicted from the measured T50, spatial-window control; ST: 4 h stage 1, one field suffices, heavier build than a census of the same quantity |
| 47 | CT-CM | CT | 3 h, success predicted from the measured T50, spatial-window control; CM: K1 at 2 orders is lenient against about 1e13 relaxed boards; multi-GiB DP, no path past c^n |
| 48 | CT-L1 | L1 | bound aimed at the saturated 470 cell, its absence confirmed in break_solver.rs; CT: needs positional agreement f > 0.5 that nothing measured supports |
| 49 | CT-L3 | CT | 3 h, success predicted from the measured T50, spatial-window control; L3: prices 471 but cannot move it; depth levels risk variance blow-up |
| 50 | CT-L4 | L4 | instrument that makes 470-type methods judgeable at small size; CT: needs positional agreement f > 0.5 that nothing measured supports |
| 51 | SI-OC | SI | 3 h with a validity gate and a random-kill control; runs unchanged on 16x16 local search; OC: 10 core-hours and a bound whose strength at dead nodes is argued only |
| 52 | SI-GC | GC | under 1 h census of refutation length and key reuse, with a plus2 null; SI: its no-barrier premise comes from another regime; generalized splitting is close prior art |
| 53 | SI-ST | SI | 3 h with a validity gate and a random-kill control; runs unchanged on 16x16 local search; ST: 4 h stage 1, one field suffices, heavier build than a census of the same quantity |
| 54 | SI-CM | SI | 3 h with a validity gate and a random-kill control; runs unchanged on 16x16 local search; CM: K1 at 2 orders is lenient against about 1e13 relaxed boards; multi-GiB DP, no path past c^n |
| 55 | SI-L1 | L1 | bound aimed at the saturated 470 cell, its absence confirmed in break_solver.rs; SI: its no-barrier premise comes from another regime; generalized splitting is close prior art |
| 56 | SI-L3 | SI | 3 h with a validity gate and a random-kill control; runs unchanged on 16x16 local search; L3: prices 471 but cannot move it; depth levels risk variance blow-up |
| 57 | SI-L4 | SI | 3 h with a validity gate and a random-kill control; runs unchanged on 16x16 local search; L4: 10 core-hours and a 5-engine rank test that one swap breaks; not a method |
| 58 | OC-GC | GC | under 1 h census of refutation length and key reuse, with a plus2 null; OC: 10 core-hours and a bound whose strength at dead nodes is argued only |
| 59 | OC-ST | OC | names the stop it removes (lossy truncation) and carries four one-component controls; ST: 4 h stage 1, one field suffices, heavier build than a census of the same quantity |
| 60 | OC-CM | OC | names the stop it removes (lossy truncation) and carries four one-component controls; CM: K1 at 2 orders is lenient against about 1e13 relaxed boards; multi-GiB DP, no path past c^n |
| 61 | OC-L1 | L1 | bound aimed at the saturated 470 cell, its absence confirmed in break_solver.rs; OC: 10 core-hours and a bound whose strength at dead nodes is argued only |
| 62 | OC-L3 | OC | names the stop it removes (lossy truncation) and carries four one-component controls; L3: prices 471 but cannot move it; depth levels risk variance blow-up |
| 63 | OC-L4 | OC | names the stop it removes (lossy truncation) and carries four one-component controls; L4: 10 core-hours and a 5-engine rank test that one swap breaks; not a method |
| 64 | GC-ST | GC | under 1 h census of refutation length and key reuse, with a plus2 null; ST: 4 h stage 1, one field suffices, heavier build than a census of the same quantity |
| 65 | GC-CM | GC | under 1 h census of refutation length and key reuse, with a plus2 null; CM: K1 at 2 orders is lenient against about 1e13 relaxed boards; multi-GiB DP, no path past c^n |
| 66 | GC-L1 | GC | under 1 h census of refutation length and key reuse, with a plus2 null; L1: stage 1 target (optimum + 1) is vacuous on planted boards; stage 0 costs 4 to 8 h |
| 67 | GC-L3 | GC | under 1 h census of refutation length and key reuse, with a plus2 null; L3: prices 471 but cannot move it; depth levels risk variance blow-up |
| 68 | GC-L4 | GC | under 1 h census of refutation length and key reuse, with a plus2 null; L4: 10 core-hours and a 5-engine rank test that one swap breaks; not a method |
| 69 | ST-CM | ST | exact-checked lemmas with a planted-path soundness stop; CM: K1 at 2 orders is lenient against about 1e13 relaxed boards; multi-GiB DP, no path past c^n |
| 70 | ST-L1 | L1 | bound aimed at the saturated 470 cell, its absence confirmed in break_solver.rs; ST: 4 h stage 1, one field suffices, heavier build than a census of the same quantity |
| 71 | ST-L3 | L3 | clean unbiasedness check against direct sampling on observable 10x10 events; ST: 4 h stage 1, one field suffices, heavier build than a census of the same quantity |
| 72 | ST-L4 | L4 | instrument that makes 470-type methods judgeable at small size; ST: 4 h stage 1, one field suffices, heavier build than a census of the same quantity |
| 73 | CM-L1 | L1 | bound aimed at the saturated 470 cell, its absence confirmed in break_solver.rs; CM: K1 at 2 orders is lenient against about 1e13 relaxed boards; multi-GiB DP, no path past c^n |
| 74 | CM-L3 | L3 | clean unbiasedness check against direct sampling on observable 10x10 events; CM: K1 at 2 orders is lenient against about 1e13 relaxed boards; multi-GiB DP, no path past c^n |
| 75 | CM-L4 | L4 | instrument that makes 470-type methods judgeable at small size; CM: K1 at 2 orders is lenient against about 1e13 relaxed boards; multi-GiB DP, no path past c^n |
| 76 | L1-L3 | L1 | bound aimed at the saturated 470 cell, its absence confirmed in break_solver.rs; L3: prices 471 but cannot move it; depth levels risk variance blow-up |
| 77 | L1-L4 | L1 | bound aimed at the saturated 470 cell, its absence confirmed in break_solver.rs; L4: 10 core-hours and a 5-engine rank test that one swap breaks; not a method |
| 78 | L3-L4 | L4 | instrument that makes 470-type methods judgeable at small size; L3: prices 471 but cannot move it; depth levels risk variance blow-up |

## 3. Things weighed

### (a) One shared stage 0 for the region-reasoning family

TW, CO, GC and ST all ask the same first question: at the dead nodes that
`bb-mrv` keeps after propagation at the critical rungs, how many can a check
over a bounded region of empty cells prove dead, and how often does the same
refutation recur? OC asks a near neighbour of it (its lambda = 0 control is a
colour-only region check). One measurement decides most of them.

**Dead-node refutation census.** Ladder R6 and R7 test seeds 1..30, plus the
plus2 null at 7x7 c=8 seeds 1..30; `bb-mrv` deterministic, cap 1e6, full tree
logged with subtree sizes. Sample up to 200 dead nodes per instance
(subtree exhausted without a solution), stratified by depth, every statistic
weighted by subtree size (a proof at a 10-node subtree saves nothing). At
each sampled node, with its propagated domains and pool, run five oracles,
each also logging its work in propagation operations:

- A. band GAC over k = 1, 2, 3 rows below the frontier, distinctness relaxed
  (TW, with its k=1 control);
- B. k x k cavities touching the placed region, k = 2, 3, 4, exact with
  distinctness and the current pool, giving the refutation length r (GC);
- C. connected interior pockets of up to 2n cells grown from the most-wiped
  cells, exact (ST's local core);
- D. partitioned propagation on 2 cut edges, on 2 random edges, and POAC on
  the MRV cell at matched propagation budget (CO's K1 to K3);
- E. colour-only feasibility of the whole empty region under current domains
  (OC's lambda = 0 floor).

For B and C also record the recurrence of each refuting boundary key across
distinct prefixes, raw and canonical under translation and rotation.
Outputs per oracle and rung: weighted share proven dead, median work, and the
recurrence rates. Budget about 3 core-hours, one logger and one sampler. It
decides TW (A), GC (B plus raw reuse), ST (C plus canonical reuse) and CO (D)
outright, and gives OC's K2 floor (E), not OC's main measurement.

Every share is an upper bound on node savings with the same branching; the
promotion gate must subtract the oracle's work (PROTOCOL target 2).

### (b) Pair proposals that need one field

The scout found GC needs only the compilers half (the glass field picks the
window size, which a sweep over k picks equally well) and ST needs only the
synthesis half (the tiling field justifies canonicalisation, a standard
symmetry reduction). Both break the brief's pair rule. Treatment: they are
not disqualified, because the criterion is about stalls and tests, but the
rule break is the deciding clause where a pairing is otherwise close (GC loses
to GT on it), and GC enters the top five relabelled as a single-field
compilers entry, with ST's canonical recurrence folded into it as a census
readout instead of a separate entry. OC and CM do need both fields: OC's
lambda = 0 and unbanded controls, and CM's K2, each isolate one field.

### (c) Which entries bear on 470 against 480

- **On the partial-match regime directly:** L1 (score bound inside
  break-tolerant DFS), L3 (prices 471 for a record engine), L4 (the
  instrument), SI (local search runs unchanged on 16x16 with score levels),
  CR with L2 (BANDSAW's memory wall is a record-hunting endgame tool).
- **Partly:** CT. Its acceptance rule needs a perfect completion; on 16x16 it
  would need a break-tolerant re-encoder, which no stage tests.
- **First-solution node counts only:** GT, CO, TW, OC, GC, ST, CM. By
  PROTOCOL_TARGETS_PROPOSAL.md section 2 item 4, only growth over n = 6..8 on
  a fixed colour track bears on 16x16. None of these preregisters a growth
  stage (CM states a growth prediction, OC reports a 9x9 band fraction as
  descriptive). All of them test R6 and R7 only, and R6 is a 951-node rung
  where a factor-2 gain is noise in wall clock. Every top-five correction
  below adds a growth stage on the +2 track (6x6 c=7, 7x7 c=8, 8x8 c=10).

The ranking favours the node family because its stage 0s are the cheapest;
four of the top five are node-count entries. That is a property of the
criterion, and it is the strongest reason to schedule L4 next regardless of
its rank: without it, L1, L3, SI and CT cannot be judged on the target the
16x16 has.

### (d) L4 is an instrument

L4 proposes no mechanism; it ranks 8th because its falsifying test is the most
expensive (10 core-hours) and its kill (rank correlation of at least 0.8 over 5
engines) is broken by one swap. As an instrument it is the precondition for
every 470-regime entry. Recommendation: build it as a measurement track
(like the hard ladder), starting from the scaling-ladder page and harness, not
as an idea-slot method.

## 4. Ranking

| rank | entry | rating | wins |
|---|---|---|---|
| 1 | game-tree-search (GT) | 1314 | 12 |
| 2 | pair-glasses-compilers (GC) | 1260 | 11 |
| 3 | compilers (CO) | 1204 | 10 |
| 4 | L1 relaxed decision diagrams | 1160 | 9 |
| 5 | tiling-words (TW) | 1098 | 8 |
| 6 | statistical-inference (SI) | 1013 | 6 |
| 7 | pair-or-compbio (OC) | 1008 | 6 |
| 8 | L4 partial-match ladder (instrument) | 956 | 5 |
| 9 | coding-theory (CT) | 946 | 5 |
| 10 | L3 price 471 by splitting | 844 | 3 |
| 11 | cryptanalysis with L2 (CR) | 789 | 2 |
| 12 | pair-synthesis-tiling (ST) | 732 | 1 |
| 13 | pair-cryptanalysis-matching (CM) | 676 | 0 |

Ranks 2, 3 and 5 share one stage 0 (section 3a), so the top five cost three
stage 0s, not five. The freed slot should go to SI (rank 6), the cheapest
entry on the 470 regime.

### T1. Revocable branching cells by proof-number search · proposed

- **Hypothesis.** At R7, MRV's cell often has a much larger refutation tree
  than another of the 3 smallest-domain cells; df-pn over revocable splits,
  same propagation as `bb-mrv`, finds the cheaper refutations at net gain.
- **Borrowed from.** Allis et al., AIJ 66, 1994; df-pn (Nagai 2002).
- **Nearest cell, missing mechanism.** Dynamic fill orders (saturated: one
  committed cell per node). Missing: effort moved between refutations.
- **Cheap test.** Stage 0 headroom oracle (r = MRV cell size over best of 3,
  300+ refuted nodes, R6/R7). Stage 1 df-pn k=3, R6/R7. Correction: add a
  growth stage on the +2 track n = 6..8, and count work (nodes plus
  propagations) beside nodes.
- **Baseline.** Better of `bb-mrv` and scb per rung (R7: scb 16/30 within
  1e6). Controls: df-pn k=1; added: dom/wdeg on `bb-mrv` (untested here),
  so a gain is not plain failure weighting.
- **Kill.** K0: median r at most 1.25 and r at least 2 at under 20% of nodes,
  both rungs. K1: R7 paired node ratio not at most 0.7 or CI includes 1. K2:
  k=3 not below k=1 (Wilcoxon p at least 0.01). K3: wall ratio above 1. Added:
  growth slope CI not below 0.
- **Budget.** Stage 0 under 2, stage 1 4, growth 3 core-hours.

### T2. Boundary-keyed cavity summaries (single field: compilers) · proposed

- **Hypothesis.** At R7 most of `bb-mrv`'s dead nodes are refuted by a k x k
  cavity, k at most 3, given their boundary and pool, and those boundary keys
  recur, so tabulated refutations (sound by pool monotonicity) cut work.
- **Borrowed from.** Reps, Horwitz, Sagiv, POPL 1995 (summary edges). The glass
  framing (Montanari and Semerjian 2006) is kept as the stage-0 readout only.
- **Nearest cell, missing mechanism.** No-good learning (mapped). Missing: keys
  bounded to a window boundary. Prior: Mocsi's lookup table (`reported`,
  never worked); Chu et al. 2010.
- **Cheap test.** Stage 0 is the shared census (3a), oracle B plus raw and
  canonical key recurrence (ST's readout folded in). Stage 1: four arms as
  proposed on R6/R7 fresh seeds, then the +2 growth track.
- **Baseline.** Better of `bb-mrv` and scb; arm 2 (lookahead, no table) and
  arm 4 (random-shape keys).
- **Kill.** K0a: under 30% of weighted dead nodes with r at most 3 at R7.
  K0b: key recurrence under 10%. K0c: r at R7 not above r at 7x7 c=8. Stage 1:
  G1 NO-GAIN on work at R7, wall ratio above 1, arm 3 not below arm 2.
- **Budget.** Census share about 1, stage 1 1 to 3, growth about 3 core-hours.

### T3. Cut-keyed trace partitioning · proposed

- **Hypothesis.** `bb-mrv` loses at the critical rungs by joining cut-colour
  cases at each cell; keeping partitions keyed on 2 cut-edge colours,
  persistent through the descent, closes failing subtrees at their roots.
- **Borrowed from.** Rival and Mauborgne, TOPLAS 29(5), 2007.
- **Nearest cell, missing mechanism.** Arc consistency (mapped). Missing: a
  relational, persistent state keyed on an auxiliary cut quantity; POAC
  (Bennaceur and Affane 2001) is the control.
- **Cheap test.** Stage 0 is the shared census, oracle D (cut key, random key,
  POAC at matched budget), plus the full replay on bb-mrv trees with
  persistent partitions. Correction: report replay work (surviving nodes times
  live partitions times propagation cost), not nodes alone.
- **Baseline.** `bb-mrv`, scb; C1 random key, C2 POAC.
- **Kill.** K1 replaced: replay work ratio above 1 at R7, or replay node ratio
  above 0.5. K2: C1 within 0.1 of the cut key at both rungs. K3: POAC at or
  below the cut key. Stage 1: G1 NO-GAIN at R7, wall ratio above 1, at most
  baseline + 2 solves. Added: growth slope on the +2 track.
- **Budget.** Census share about 0.5, stage 1 2 to 3, growth about 3
  core-hours.

### T4. Relaxed decision diagrams as a score bound in break-tolerant DFS · proposed

- **Hypothesis.** A width-W relaxed DD that keeps frontier colours exact and
  relaxes piece distinctness bounds the best score reachable from a prefix
  tightly enough to prune break-tolerant DFS before the endgame.
- **Borrowed from.** Bergman, Cire, van Hoeve, Hooker, IJOC 28(1), 2016; Cire
  and van Hoeve, Oper. Res. 61(6), 2013.
- **Nearest cell, missing mechanism.** Break-tolerant DFS (saturated); no
  bound in `break_solver.rs` lines 15 to 24. Missing: a per-node score bound.
  Record engines' code was not inspected; their absence of a bound is
  `reported`.
- **Cheap test.** Stage 0 bound gap by rows remaining and W in {64, 512,
  4096}; report the share of merges that relax frontier colours. Correction:
  stage 1 "prove no board at optimum + 1" is vacuous on planted boards;
  replace with work to reach optimum minus k (k = 1..4) on 8x8 c=8 boards
  where `BreakSolver` stalls, with and without the bound (L4's metric).
- **Baseline.** `BreakSolver` with its schedule; control W = 1.
- **Kill.** Stage 0: median gap with 3 rows left above 4 edges at every W.
  Stage 1: work ratio above 0.5 or wall ratio above 1 at any k.
- **Budget.** 4 to 8 core-hours.

### T5. Band consistency by composed tile transducers · proposed

- **Hypothesis.** GAC over a k-row band (k = 2, 3), built by composing the row
  transducer of the cells' current domains, removes values arc consistency
  keeps because it is exact on the band's cycles.
- **Borrowed from.** Jeandel and Rao, Adv. Comb. 2021 (transducer section to
  be checked); Pesant, CP 2004 (forward-backward filtering).
- **Nearest cell, missing mechanism.** Arc consistency (mapped). Missing:
  cycle-exact consistency computed from current domains during search.
- **Cheap test.** Stage 0 is the shared census, oracle A, with oracle B on the
  same bands as an exact-distinctness ceiling, so the cost of the relaxation
  is measured, not argued. Stage 2 as proposed, then the +2 growth track.
- **Baseline.** Better of scb and `bb-mrv` (correction: the proposal names scb
  only as baseline); k=1 control; full-piece-set relaxation control.
- **Kill.** Census: k=2 proves dead under 5% of weighted dead nodes at both
  rungs. Stage 2: G1 no gain at R6 and R7, or CPU ratio above 2 at both.
  Mechanism: k=1 within the k=2 CI. Added: growth slope CI not below 0.
- **Budget.** Census share about 0.5, stage 2 about 3, growth about 3
  core-hours.

## 5. Dead-end claims this ranking leaned on

Grades: `measured` (committed results or a reproducible run here),
`reported` (forum or someone else's number, not rerun here), `argued`
(reasoning, or one small or single attempt). Only `measured` claims decided a
pairing.

| claim | file and line | grade | affects |
|---|---|---|---|
| bb-mrv fails G1 at R6/R7, slower in wall clock | research/program/AGENDA.md line 336; research/experiments/mrv-baseline/JUDGE.md line 5 | measured | GT, CO, TW, GC, ST, OC, CM, CT (stall) |
| scb R6 951 nodes, R7 16/30 within 1e6 | research/experiments/hard-ladder/README.md lines 90 to 91 | measured | CR (K1 decided in advance), CT, every node entry |
| Hall matching NO-GAIN at R7 | AGENDA.md line 288 | measured | OC, CM |
| T50 exponent 0.61 vs 0.59 | research/experiments/sat-backdoors/README.md line 158 | measured | CT |
| PT 0/8 at 8x8 and 10x10 calibration | research/experiments/quality-diversity-basins/README.md line 33 | measured | SI |
| mid band a cheap saddle, not a barrier (framed 5x5/6x6) | research/LEDGER.md line 2580; AGENDA.md line 212 | measured, other regime | SI |
| no bound in BreakSolver | engine/src/break_solver.rs lines 15 to 24 | measured (code read) | L1 |
| "pruning fades about two cells out" | EXPLORED.md line 22; web/content/research/build/dead-ends.mdx lines 123 to 124 (msg 6208) | reported | TW, CO, GC, CM, OC (stall framing only) |
| no-goods outrun memory at 16x16 | EXPLORED.md line 23; build/reduce/nogood-learning.mdx lines 250 to 263 (capiman accounting) | reported | GC, ST, GT (transposition table) |
| Mocsi's Failure Lookup Table, boundary-keyed | build/reduce/nogood-learning.mdx lines 181 to 186 | reported (never worked, a bug) | GC, ST: prior art, not a stop |
| deletion-minimised 28-piece core | build/reduce/nogood-learning.mdx lines 241 to 248 | reported | ST |
| pattern databases carry no information | build/dead-ends.mdx lines 255 to 262 | argued (root only, true by construction) | TW, GC, ST |
| local cluster enumeration blows up | build/dead-ends.mdx lines 356 to 360 | reported (labelled Measured on the page, no committed result) | GC, ST |
| Lagrangian-patched tensor contraction stops at memory | build/dead-ends.mdx lines 187 to 197 | argued (single notebook attempt) | OC, CM |
| multi-way MITM collapses back into single-sided search | build/exact/meet-in-the-middle.mdx lines 234 to 253 | argued (uncommitted model) | CR |
| BANDSAW about 20x memory per extra mismatch | EXPLORED.md line 25; lab/.../meet-in-the-middle/bandsaw.mdx lines 108 to 111 | reported (figures not committed) | CR |
| fake 480 collapses, distinctness binds globally | build/dead-ends.mdx lines 670 to 676 | reported (Millilaw) | L1 risk |
| repair seeded by exact prefix ends at 424 | build/dead-ends.mdx lines 681 to 703 | argued (one seed, one configuration) | CT |
| near-top obstacle is entropic | build/local-search/parallel-tempering.mdx line 140 | argued | SI |
| local optima are displaced planted blocks | AGENDA.md A13 (proposed, unrun) | argued | CT (its own expected death) |
| break-tolerant DFS saturated at 467/470 | EXPLORED.md line 20 | reported (community records) | L1, L3, L4 |
