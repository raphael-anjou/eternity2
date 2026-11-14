# Independent re-check: real-prefix part

2026-11-14. Official pieces, wrong prefixes; not Eternity II progress. Scripts: `scripts/recheck/recompute.py` (own parser, reads the raw outputs; its output is `results/analysis/recheck.recompute.out.txt`) and `scripts/recheck/rerun.py` (reruns programs and compares counts). The reruns (6 core-minutes) were done on a separate machine and are reported here, not repeated in this folder.

## Ruling

The stop line fired. Depth clause: fired, firmly, on the measure that carries cost (summed work). Size clause (toy part): leaning fired, not established. Together: the kill stands for clause learning on this encoding with these solvers. The 7x7 toy run that would settle the size clause formally is not worth about 30 core-hours.

## (a) Soundness

Sound. Checked:

- 800 raw runs, 160 prefixes, no duplicate, no prefix split over machines, none killed. The independent parser matches `results/analysis/per_prefix.out.tsv` on every count and cut flag. 13 cut runs, all at depth 88: CaDiCaL on 3 prefixes, clasp default on 10.
- Reruns: tree 160 of 160 identical, arc consistency 157 of 157 (all but the three heaviest), clasp without learning 5 of 5, CaDiCaL 5 of 5, of which 4 first ran on another machine. Clasp default 4 of 5: `d88_31` gives 50,222 conflicts on the re-check machine against 52,914 on machine-b. Clasp default counts depend on the build; the other arms do not.
- Completions: all 800 runs report 0, so "all arms agree" tests nothing on the real pieces. What does support the encoding: summed conflicts of clasp without learning sit within 0.04 bits of arc consistency's failed branches at depths 104, 96 and 88.
- Sampler. Uniform over the level is the population whose mean subtree is the cost of the search; a single descent would favour nodes with few siblings. An improvement.
- Depth definition. A population estimate of this tree (rows 0 to 15, 20,000 a level, 2 runs) gives mean subtrees 2^8.5, 2^13.3, 2^18.6, 2^24.6 at depths 112, 104, 96, 88. The sample gives 2^5.7, 2^11.1, 2^18.3, 2^24.5. The definition is confirmed at 96 and 88 (within 0.3 bits). The figures 2^12.4 and 2^17.1 quoted in the plan belong to the profile's reversed scan, where the centre clue falls elsewhere. At 112 and 104 the sample is 2.7 and 2.2 bits short of the mean: 40 prefixes miss the rare ones that carry the cost.
- Moved prefixes. Counts of finished runs cannot change. Where a cut lands can: `d88_16` (on machine-b from launch, not moved) was cut at 1.68 million conflicts because machine-b runs CaDiCaL at 1,374 conflicts a second against 2,628 and 3,148 on the two Apple machines. Its bound of +1.08 is weak for that reason only.
- Seconds. The five arms of a prefix share a machine. Per machine at depth 88, CaDiCaL against arc consistency is -3.32, -3.68, -3.78 bits; arc consistency against the plain tree -5.27, -5.91, -5.12. Pooling does not distort the ratios.
- Order of events. Only the pre-stated stop line provably predates the data; the ruling uses only that. The plan file is dated five seconds before the first run of the full set and after the pilot.

## (b) The stop line

As written in the plan (median over 40, branches tried): 0.00 at 104, 0.00 at 88, rise 0.00. Fired, and meaningless: both counts are zero on 37 of 40 prefixes at 104.

On summed work, log2(sum arc consistency tried / sum CaDiCaL):

| Depth | Lead | Note |
|---|---|---|
| 104 | -0.07 | exact; 3 prefixes with work, 909 branches in all |
| 96 | +0.14 | exact; bootstrap -0.04 to +0.34 |
| 88 | at most -0.25 | CaDiCaL cut on the 3 largest; no lower bound |

At 88, on the 37 prefixes CaDiCaL finished: -0.65. On the 30 every arm finished (19% of the tree): +0.05. Rise from 104 to 88: at most -0.18; bootstrap of the bound -1.55 to +0.92, none of 10,000 resamples reaches 3. The informative step is 96 to 88: at most -0.39. A cut run can only lower the lead, so every bound favours firing.

Like for like (conflicts against conflicts, same CNF), CaDiCaL against clasp without learning: -1.93, -1.68, at most -2.07 at 104, 96, 88. CaDiCaL needs more conflicts on 45 of the 45 prefixes with work (3, 18, 24).

Verdict on the trend: flat within half a bit from 104 to 96 to 88, with a lean to shrinking that the cut runs cannot confirm. It does not grow.

## (c) Worth for the 16x16

Counts: learning adds 0 bits over arc consistency (range -0.7 to +0.1) and costs 1.7 to 2.1 bits against propagation alone on the same encoding. Time: -3.3 to -3.8 bits against arc consistency and about -9 against the plain row scan, which is the fastest program at every depth (15.5 s summed at 88 against 689 for arc consistency). Against a gap of 56 to 84 bits: nothing.

Properties of the method, as far as this shows: the learner never beats propagation alone in conflicts; clasp's learnt clauses average 511, 934 and 1,366 literals at 104, 96, 88 (median over prefixes with work), growing as the prefix gets shallower. Clauses that long prune almost nothing elsewhere.

Properties of these programs: every figure in seconds; arc consistency's 2^11.5 plain nodes per branch (a tuned incremental version might recover 3 to 5 bits and reach about break-even with the plain tree, no more); CaDiCaL's conflict counts under its default configuration and 5,000-conflict calls; clasp default's 2.6 to 3.9 bits behind its own no-learning mode; the clause lengths, which come from a pairwise direct encoding.

Further runs: none. A propagator-based learner with short explanations is the only untested variant. The re-check's estimates (judgement, not computed): about 0.5% for 10 bits from clause learning, about 2% for 3 bits from that variant.

## (d) Statements stronger than the data, and omissions

These are applied in `measurement.md` ("Corrections from the independent re-check").

1. "The plan predates the data": by file time only, and after the pilot.
2. "All arms agree": true and vacuous (0 against 0).
3. The profile figures 2^12.4 and 2^17.1 are quoted as the check of the depth definition. They are another tree's; this one has 2^8.5 and 2^13.3, and the sample falls short of both.
4. The largest omission: the arm with the fewest steps overall is clasp with learning off and a static order (4.19 million at depth 88, against 14.8 million for arc consistency, at least 17.6 for CaDiCaL, at least 25.8 for clasp default). Any lead of a solver arm over "branches tried" is therefore a change of unit (1.8 bits), and learning subtracts from it on every prefix with work.
5. "Prefixes with work: about +0.1" compares conflicts with branches tried. Against failed branches it is about -1.7.
6. `d88_16` "at most +1.08" stands beside bounds from machines twice as fast (CaDiCaL at 1,374 to 3,148 conflicts a second, not "about 1,300").
7. Median log2 of the tree at 104 and 96 (6.2, 13.6) is the upper middle value; the midpoint gives 5.5 and 13.2.
8. Clasp default counts are presented as fixed; one of five differed on rerun.
9. Arc consistency against the plain tree in seconds (-5.1 to -5.9 bits per machine at depth 88) is absent from the seconds table.

## Unplanned observations

- Learnt clause length grows as depth falls, the opposite of what a rising lead needs.
- At depths 112 and 104 the cost sits in prefixes rarer than 1 in 40.
