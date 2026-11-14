# Solve path 5: the price of just doing it

Night 2026-11-12. Lens: what an exhaustive or first-solution search of the real
16x16 costs on 2026 hardware, and what would make it affordable. Scripts and
their outputs: `research/program/night/2026-11-12/memos/solve-path-5/`.
Compute used tonight: about 4 minutes on one core, no solver run on the 16x16.

Grades used below: **measured** (committed data in this repo, or computed
tonight by a script in the folder above), **reported** (published or forum,
not rerun here), **argued** (reasoning only).

## 0. Answer

- **Price, five clues, best known order (row scan), plain edge-match
  backtracking:** 3.1 x 10^40 nodes to exhaustion, 1.5 x 10^40 expected to the
  first (and, in the model, only) solution. Range: 10^39 to 10^42. At 10^8
  nodes per second per core that is about 5 x 10^24 core-years, and at about
  10^14 nodes per euro about 1.5 x 10^26 euros.
- **Price, centre clue only:** 1.4 x 10^47 nodes to exhaustion, 9.3 x 10^42 per
  expected solution. The five-clue route is about 600 times cheaper to a first
  solution, so the 14,702 solutions of the one-clue puzzle do not help.
- **Gap:** a determined individual (10^5 core-years, 3 x 10^20 nodes) is 19.7
  orders of magnitude short. The live volunteer project (reported 1.1 x 10^12
  nodes per second) is 20.6 orders short per year of running. Every credible
  multiplier together is worth about 2.5 orders (range 1.5 to 4). **Remaining
  gap: about 17 orders, plus or minus 2.**
- **No constant factor closes it.** In the model, 17 orders equals 56 bits of
  information about the solution, or six certain piece placements ahead of
  depth 160. The program's task is to find that information somewhere other
  than in a faster or wider search.
- **Probabilities** that brute force with all credible multipliers yields a
  complete solution: three desktops for a year, 10^-20; 1,000 core-years,
  5 x 10^-20; 10^6 core-years, 5 x 10^-17.

## 1. The price

Reference speed: 10^8 nodes per second per core (section 4). One core-year is
then 3.16 x 10^15 nodes. Reference cost: 10^14 nodes per euro, within a factor
of 3 for rented CPU, rented GPU and owned GPU (section 4).

| Search | Nodes (model) | Range | Core-years | Euros |
|---|---|---|---|---|
| Five clues, first solution (expected) | 1.5 x 10^40 | 10^39 to 10^42 | 4.9 x 10^24 | 1.5 x 10^26 |
| Five clues, exhaustion | 3.1 x 10^40 | 10^39 to 10^42 | 9.8 x 10^24 | 3 x 10^26 |
| One clue, first solution (expected, if solutions are spread evenly) | 9.3 x 10^42 | 10^41.5 to 10^44.5 | 2.9 x 10^27 | 9 x 10^28 |
| One clue, exhaustion | 1.4 x 10^47 | 10^45.5 to 10^48.5 | 4.4 x 10^31 | 1.4 x 10^33 |

For scale: world GDP is of order 10^14 euros a year. The cheapest row is twelve
orders above it.

What dominates the range (plus or minus 1.5 orders), largest first:

1. **Whether the designed piece set follows the ensemble at depth 130 to 175.**
   The model is an ensemble average over piece sets with the official colour
   counts. It is checked on random boards (section 3). On the real pieces there
   is one reported sampling estimate from 2008 that agrees with it, with its
   clue set and node definition not stated. I put plus or minus 1 order here
   (argued).
2. **Model bias on random boards.** Measured tonight: real trees are 1.1 to 2.0
   times larger than the model on 4x4 to 6x6, the bias shrinking with size.
   Published checks at 9x9 are within 25%. Worth 0.3 orders.
3. **Node definition and order.** Placement against node, row scan direction:
   a factor of 2 at most. A better order than row scan: none found (section 2).

A site inconsistency this resolves: `web/content/research/lab/experiments/raphael-anjou/going-fast.mdx`
and the McGavin page quote "about 3.1 x 10^40 nodes" and "4.9 x 10^32 years at
100 million nodes per second" for the five-clue path, and the McGavin page
notes that the two do not match. The port below reproduces 3.1 x 10^40 for the
five-clue row scan. At 10^8 nodes per second that is 9.8 x 10^24 years. The
4.9 x 10^32 figure corresponds to about 1.5 x 10^48 nodes and belongs to some
other count. The site should carry 10^25 years, not 4.9 x 10^32.

## 2. The model, and every assumption

The model is Brendan Owen's complex theory as implemented by Peter McGavin
(`research/topics/complex-theory/reference/complex_theory.c`, committed). I
ported it to exact integer arithmetic in `solve-path-5/bulge.py`.

For a set of occupied cells, the expected number of valid partial fills is

    W = P(corners) P(borders) P(middles) 4^middles x pm x pb

with `P` the falling factorials that enforce piece distinctness within each
class, `pm` the probability that the completed interior joins all match given
the exact interior colour histogram (24 or 25 joins per colour, 17 colours),
and `pb` the same for the frame joins (12 per colour, 5 colours). Clues are
pre-placed pieces. The expected node count of a fixed-order backtracker is the
sum of `W` over the prefixes of the order. A node is one placement that agrees
with the rim and with every placed neighbour.

Assumptions, named:

- A1. Edge colours are exchangeable given the histogram (the four colours of a
  piece are treated as independent draws). First moment only, no variance.
- A2. The solver checks only already-placed neighbours, in a fixed order. No
  look-ahead, no learning.
- A3. The official set behaves like a typical member of the ensemble.
- A4. With five clues the solution is unique (the model gives 4.1 x 10^-8
  others), so the expected first-solution cost is half the tree.
- A5. With one clue the 14,702 expected solutions are spread evenly over the
  tree, so the first arrives after tree / solutions nodes. Tested in section 3
  on small boards: it is optimistic.

Output for the official set (`solve-path-5/out_e2_price.txt`,
`out_e2_profile.txt`):

| Setting | Total nodes | Peak width, depth | Solutions (model) |
|---|---|---|---|
| One clue, row scan from the bottom row | 10^47.14 | 10^45.86 at 161 | 14,702 |
| Five clues, row scan from the bottom row | 10^40.49 | 10^39.20 at 163 | 4.1 x 10^-8 plus the planted one |
| No clue, row scan | 10^50.03 | 10^48.76 at 161 | 1.153 x 10^7 |

Width profile, five clues, row scan (log10 of expected partial fills after k
free cells): 16: 14.4, 32: 18.0, 48: 21.6, 64: 26.7, 96: 34.9, 128: 37.7,
160: 39.1, 192: 33.8, 224: 15.7. 98% of all nodes lie between free-cell depths
129 and 175. A budget of 10^20 nodes pays for a complete enumeration of the
first 40 free cells (two and a half rows); each such prefix then carries on
average 10^19 descendants at the peak.

**Fill order (measured tonight in the model, `out_e2_orders.txt`).** Ten order
families under five clues: row scan 10^40.49; two-row bands 10^41.09; four-row
bands 10^42.39; anti-diagonal 10^45.11; corner squares 10^45.98; bottom and
top rows first 10^44.94; frame first 10^50.75; the 12x12 box holding the four
outer clues first 10^59.13. A greedy order that always minimises the next
width gives 10^40.83. A 60 second hill climb of single-cell moves from the row
scan (about 11,000 proposals) found no improvement. The reason is
isoperimetric: the board has 480 joins and 256 cells whatever the order, the
order only decides how early joins are completed, and a row scan against three
rim sides is already near the maximum number of completed joins per placed
cell. The fill-order lever has been harvested (it took the community from
10^60 to 10^40). I credit at most one more order to it (argued).

**The clues.** Four extra clues cut the first-solution cost from 9.3 x 10^42
to 1.5 x 10^40, a factor of about 600, not the factor 784^4 = 3.8 x 10^11 by
which they cut the solution count. Only the clues that the order passes before
the peak pay: the row scan from the bottom passes N3 and N14 at free-cell depth
about 34 to 45 and the centre at about 120, and reaches C3 and C14 after depth
200, where the tree is already collapsing. Each certain placement that the
order passes before the peak is worth a factor 196 x 4 = 784, that is 2.9
orders or 9.6 bits.

## 3. Calibration: predicted against observed

| Check | Model | Observed | Observed / model | Source, grade |
|---|---|---|---|---|
| McGavin's published depth table, official set, one clue (depths 2, 3, 5, 8, 16, 157, 256) | 4; 44.8; 5341; 6.089e6; 3.689e14; 5.045e45; 14,702 | same | 1.000 | `research/topics/complex-theory/README.md`; port check, measured tonight |
| Total, one clue, row scan | 1.38 x 10^47 | 1.365 x 10^47 (table), 1.6 x 10^47 (msg 9710) | 0.99 to 1.16 | same README; McGavin page; reported |
| Total, five clues, best path | 3.1 x 10^40 | 3.1 x 10^40 (msg 11201) | 1.0 | `web/content/research/lab/experiments/peter-mcgavin/backtracker.mdx`; reported, model against model |
| Full tree of the real puzzle by multi-day sampling, 2008 | 3.1 x 10^40 (five clues) | 2.3 to 2.7 x 10^40 | 0.74 to 0.87 | Max, msg 6023, via `web/content/research/build/backtracking/fill-order.mdx` line 282; reported; clue set and node definition not stated, so the agreement may be partly coincidence |
| Brendan's 9x9 set 1 and set 2, full trees | 1.84 x 10^14 | 1.9175 x 10^14; 1.4509 x 10^14 | 1.04; 0.79 | msg 8793 via `web/content/research/build/benchmarks.mdx` lines 184 to 187; reported |
| 9x9 set 1, solutions | 3.2 | 2 | 0.63 | same page; reported |
| 5x5 corner blocks of Brendan's 10x10 | 5.0077 x 10^12 | 4.7398 x 10^12 | 0.95 | msg 8672 via the McGavin page; reported |
| Brendan's 10x10 set_1, first solution | not computable here (the instance is not in the repo) | 2 x 10^17 nodes, 180 core-years, "less than 0.5% of the whole tree", so the tree exceeds 4 x 10^19 | n/a | msg 9686, 9688 via the McGavin page; reported |
| Generated 4x4 (2 frame, 3 interior colours), 40 boards, full tree | 10^3.95 | 10^4.18 | 1.72 (sd 0.17 dex) | `solve-path-5/out_A.txt`; measured tonight |
| Generated 4x4 (3, 4), 40 boards | 10^2.78 | 10^3.08 | 2.00 (sd 0.15 dex) | same |
| Generated 5x5 (3, 4), 20 boards | 10^5.53 | 10^5.70 | 1.49 (sd 0.14 dex) | same |
| Generated 5x5 (3, 5), 20 boards | 10^4.54 | 10^4.65 | 1.30 (sd 0.18 dex) | same |
| Generated 6x6 (4, 6), 8 boards | 10^5.87 | 10^5.93 | 1.13 (sd 0.12 dex) | same |
| Generated 6x6 (5, 7), 8 boards | 10^4.36 | 10^4.48 | 1.32 (sd 0.17 dex) | same |
| Committed hard-ladder 6x6 c=5, 30 test instances, full tree by my enumerator | 4.215 x 10^5 | geometric mean 5.8 x 10^5 | 1.37 (sd 0.16 dex) | instances `research/experiments/hard-ladder/instances/primary-test.jsonl`; `solve-path-5/out_B_primary6.txt`; measured tonight |
| Committed hard-ladder 6x6 c=6, 30 test instances | 2.207 x 10^4 | geometric mean 3.6 x 10^4 | 1.61 (sd 0.17 dex) | `secondary-test.jsonl`; `out_B_secondary6.txt`; measured tonight |
| Same 6x6 c=5 instances, first solution, committed `hp-dfs` | model tree / 5 = 8.4 x 10^4 | median 4.3 x 10^4 (committed), exact tree / (solutions + 1) median 7.1 x 10^4 | 0.5 | `research/experiments/hard-ladder/results/test/primary/hp-dfs.jsonl`; measured (committed) |

Where the model is off, and by how much:

- **Tree size: low by a factor 1.1 to 2.0 on small boards**, standard deviation
  0.12 to 0.18 dex between boards, the bias shrinking as the board grows
  (4x4: 1.7 to 2.0; 5x5: 1.3 to 1.5; 6x6: 1.1 to 1.6). Published 9x9 checks:
  0.79 and 1.04. Nothing here suggests an error beyond 0.3 orders for a random
  16x16.
- **Solution count: wrong for planted small boards.** The 30 committed 6x6 c=5
  instances have 372 solutions in total against 1.9 predicted. Each planted
  board has its 4 rotations and usually 2^k more from interchangeable pieces
  (per-instance counts are 4, 8, 16, 32, 64). The official set has no rotation
  (the centre clue), no duplicate pieces and no rotationally symmetric piece
  (the one dead-end entry backed by committed data,
  `research/topics/adversarial-piece-set/results/invariants.json`), so this
  failure does not transfer, but it shows the first moment says nothing about
  families of related solutions.
- **Cross-check of my counter:** my row-major enumerator reproduces the
  committed `hp-dfs` first-solution node count exactly on 14 of 30 (c=5) and
  19 of 30 (c=6) instances. The rest differ by a candidate-order detail I did
  not chase. The node convention is the same.
- **Even spread of solutions (A5): optimistic.** On the generated boards, first
  solution cost x solutions / tree has medians 1.0, 1.0, 2.1, 2.3, 2.8 and 6.0
  across the six sets (1.0 would be an even spread), with means up to 30.
  Solutions cluster, and the first one arrives later than tree / solutions.
  Committed 6x6: 0.9 (c=5) and 0.7 (c=6). No set shows the first solution
  arriving much earlier than an even spread predicts.

## 4. The price of a node

**CPU, per core.**

| Figure | Setting | Grade, source |
|---|---|---|
| 105 to 110 M nodes/s (peak 123 M) | this repo's JIT backtracker, hard board, Apple M1 | page `web/content/research/lab/experiments/raphael-anjou/jit-backtracker.mdx` (repro kind `heavy`); I found no committed result file, so reported |
| 24 to 41 M nodes/s | break-tolerant Verhaard and Blackwood style engines, 60 s, one thread, M1 with a core-type confound | measured, `research/experiments/single-core-benchmark/results/` |
| 0.7 to 2.4 M nodes/s plain DFS; 13 k to 204 k with propagation | hard-ladder harness, instrumented | measured, `research/experiments/mrv-baseline/results/tables.md` |
| 44.6 M sustained (2013), 105 M (2026, Ryzen 5 5600H), 295 M on small puzzles on a newer CPU, "halves on 16x16" | McGavin's C | reported, msgs 9167, 11782, 11750, 11751 via the McGavin page |
| 35 M average | the 10x10 solve: 2 x 10^17 nodes / 180 core-years | derived from reported |

Reference: 10^8 nodes per second per core, range 0.5 to 3 x 10^8.

**GPU, per device.**

| Figure | Setting | Grade, source |
|---|---|---|
| 2.4 x 10^9 placements/s, about 100 times his own single core (23 M) | Barr, 2026 | reported, msg 11835 via `web/content/research/build/hardware/gpu-solving.mdx` |
| 3.17 x 10^9 placements/s on a 10x10, rented RTX 4090; code "doesn't work well with the full Eternity puzzle" | Barr, 2025 | reported, msg 11598 |
| "north of 10 billion" | Miles, 2026, rule of thumb | reported, msg 11826 |
| 1.3 to 1.9 x 10^9 nodes/s on a GTX 1070 | third-party solver, October 2026 | reported, via `memos/literature-2026.md` section 2 |
| 27-Queens counted in 28.4 days on 8 RTX 5090; about 100 times faster than the 2017 FPGA run of about a year | Yao and Li, arXiv:2511.12009, 15 November 2025, https://arxiv.org/abs/2511.12009 (fetched tonight) | reported, preprint. Derived by me: 2.349 x 10^17 solutions / 2.455 x 10^6 s / 8 = 1.2 x 10^10 solutions per second per GPU, so at least 10^10 bit-parallel backtracking nodes per second per device. N-Queens state is three machine words; an Eternity II node needs table lookups, so this is a ceiling, not a projection |
| 339 x 10^12 nodes in 13 hours on 256 V100 GPUs (flow-shop branch and bound) | Gmys et al., INFORMS J. Computing 2022, https://pubsonline.informs.org/doi/epdf/10.1287/ijoc.2022.1193 (search summary, not re-read) | reported. Derived: 2.8 x 10^7 nodes per second per GPU, with a heavy bounding step per node |
| 1.1 x 10^12 nodes/s on 172 computers, CPU and GPU apps | live volunteer project dashboard, https://boinc.eternityathome.org/eternity/, read by the lead on 2026-11-12 | reported; what a node counts and how results are validated are not stated. Derived: 6.4 x 10^9 per computer, consistent with one GPU each; 3.5 x 10^19 nodes per year |

Measured Eternity II GPU throughput is 2 to 3 x 10^9 per device; 10^10 is a
claim, supported as plausible by the N-Queens figure. Against one tuned core
that is 25 to 100 times. No tuned full-board GPU solver with node counts
checked against a CPU reference exists (agenda item A8).

**FPGA.** Measured: Field's 2014 prototype, about 800 M per small board
(four cores, sixteen threads). Projected by him: 5 x 10^9 per Zynq 7020 chip,
never run at that scale (`web/content/research/build/hardware/fpga-solving.mdx`,
msgs 9226, 9228; reported). His per-solver cost was 334 LUTs and 96 registers;
a large 2026 device with of order 10^6 LUTs would hold about 2,000 solvers and
reach 10^11 per device at 50 M each. That is argued, with no measurement, and
such devices cost thousands of euros, so the per-euro gain over a GPU is at
most a small factor.

**ASIC.** Argued only. The Bitcoin network runs at about 0.93 to 1.0 x 10^21
hashes per second in November 2026 (Kryptex and Hashrate Index readings from
early November 2026, via search summaries:
https://pool.kryptex.com/en/btc/hashrate,
https://hashrateindex.com/blog/hashrate-index-roundup-november-9-2026/), that
is 3 x 10^28 hashes a year. If one Eternity II node cost as much silicon and
energy as one double SHA-256, an industry of that size would do 3 x 10^28
nodes a year.

**Money.** All derived tonight from third-party listings, so reported:

| Hardware | Price | Nodes per unit of money |
|---|---|---|
| Rented dedicated CPU, 48 cores at 199 euros a month (Hetzner AX162, listing via https://www.hetzner.com/dedicated-rootserver/ax162/ and a reseller page, 2026) | 50 euros per core-year | 6 x 10^13 per euro |
| Cloud spot CPU, 0.011 to 0.013 dollars per vCPU-hour (AWS Graviton c7g and c8g, aggregator snapshots January to June 2026, https://calculator.holori.com/aws/ec2/c8g.8xlarge) | 96 to 114 dollars per core-year | 3 x 10^13 per dollar, if a Graviton core reaches 10^8 (not measured) |
| Rented RTX 5090, 0.34 to 0.41 dollars per hour (marketplace aggregators, July and August 2026, https://computeprices.com/providers/vast/gpus/rtx5090) | 3,000 to 3,600 dollars per device-year | 3 x 10^13 (at the measured 3 x 10^9) to 10^14 (at the claimed 10^10) per dollar |
| Owned RTX 5090 class card, about 3,000 euros (not verified tonight) over 3 years plus 0.575 kW at 0.20 euros per kWh | about 2,000 euros per device-year | 5 x 10^13 to 1.6 x 10^14 per euro |

So about 10^14 nodes per euro, within a factor of 3, whichever way it is
bought. The GPU does not change the price per node by an order of magnitude;
it changes how many nodes fit under one desk.

## 5. The gap and its anatomy

Target: 1.5 x 10^40 nodes (five clues, expected first solution).

| Who | Nodes available | Gap, orders |
|---|---|---|
| Three desktops for a year (48 cores, three GPUs at 3 x 10^9 to 10^10) | 10^18 | 22.2 |
| 1,000 core-years | 3.2 x 10^18 | 21.7 |
| The live volunteer project, one year at its reported rate | 3.5 x 10^19 | 20.6 |
| A determined individual, 10^5 core-years | 3.2 x 10^20 | 19.7 |
| 10^6 core-years | 3.2 x 10^21 | 18.7 |
| An exascale machine for a year (El Capitan, 44,544 GPUs per https://hpc.llnl.gov/hardware/compute-platforms/el-capitan, at an assumed 10^10 each) | 1.4 x 10^22 | 18.0 |
| A company spending 10^9 euros | 10^23 | 17.2 |
| Anyone: an ASIC industry the size of Bitcoin mining, one year (argued) | 3 x 10^28 | 11.7 |

### The multiplier table

Baseline: row scan, five clues, plain edge-match check, 10^8 nodes per second
per core. Factor is the gain in solutions found per core-year (or per euro
where stated).

| Multiplier | Factor | Grade | Evidence |
|---|---|---|---|
| Per-core engineering | 1 to 3 | reported | baseline is already a tuned engine; 295 M on small puzzles, halved on the 16x16 (msgs 11750, 11751) |
| SIMD, bit-parallel candidate filtering | 1 to 3, not independent of the row above | argued | Field measured 26 cycles per placement with half of it memory latency (msg 9003); a floor near 8 cycles is plausible |
| GPU, per device against one core | 25 to 100 | reported | Barr 2.4 to 3.2 x 10^9; N-Queens ceiling 10^10 |
| GPU, per euro | 1 to 3 | derived from reported prices | section 4 |
| FPGA, per device | 8 (prototype) to 1,000 (projection) | reported, then argued | Field 2014; per euro at most a small factor |
| ASIC, per euro | 100 to 1,000 over a GPU after a very large fixed cost | argued | by analogy with hashing |
| Parallel efficiency | 0.5 to 1.0 | reported | independent subtrees; Barr's "dwindling workers" on GPU |
| Better fill order | 1 (at most 10) | measured tonight in the model | section 2: ten families and a hill climb, none below the row scan |
| Stronger propagation | 0.2 to 1 in wall clock on small boards; unknown at 16x16 | measured | `mrv-baseline/results/tables.md`: `bb-mrv` cuts nodes 87 times at 6x6 c=5 and 7 times at 8x8 c=10 while running 40 to 90 times slower per node than plain DFS; wall clock 2,658 ms against 559 ms at 8x8. The node reduction does not grow with size in this data |
| Global feasibility pruning (Hall matching, parity, ring cuts) | 1.0 to 1.1 | measured | agenda A17: node ratio 0.88 to 0.90 at 6x6, no gain at 7x7 and 8x8 |
| The five clues | 600 on the first solution | measured tonight in the model | already in the baseline |
| Symmetry | 1 | measured | `adversarial-piece-set/results/invariants.json`; the centre clue removes the board rotation |
| Time-memory trade-off, meet in the middle | 1 | argued | the tree size already is, within a small factor, the number of consistent partial fills at the peak; a join over two halves must enumerate consistent half-boards, and the bottom half alone has 10^37.7 of them at 128 free cells. Memory fails 15 orders earlier |
| Restarts and the heavy tail | 1 | measured tonight on small boards, argued for the 16x16 | with five clues there is one solution at a uniform position in a fixed tree, and the factor 2 for the expected position is in the baseline. With one clue the solutions cluster (section 3), so the many-solution route does not beat the five-clue route |
| Ranking subtrees by predicted solutions per node | 25 to 150 | reported | McGavin: a factor 25 by depth 81 (msg 9751); the 10x10 fell after 92,907 ranked row tests of about 20 million, "less than 0.5% of the whole tree" (msgs 9686, 9688), a gain of 40 to 150 depending on how the four rotated copies are counted. One success, so the rate is loosely constrained |
| Structure of the designed instance | unknown, 0.1 to 10 either way | argued | no measurement of the real set's tree at depth except the 2008 sampling figure |

**The credible product.** Per-core tuning 2, GPU per euro 2, ranking 100:
a factor 400, 2.6 orders. Generous version (3, 3, 150, order 10): 4.1 orders.
Sparse version (ranking 25 alone): 1.4 orders. Nothing else in the table is
above 1 with evidence.

**Remaining gap.** 19.7 minus 2.6 is 17 orders for the determined individual,
with plus or minus 2 from the tree-size range and the spread of the product.
For three desktops it is about 20. For an ASIC industry it is about 9. No
combination closes it, and the conclusion does not depend on which end of any
range is taken.

This also grades the dead-ends entry "throw more compute at it" (flagged stale
in `EVIDENCE_AUDIT.md`, line 88). Its evidence was a 2007 grid, but the claim
itself survives a 2026 recomputation by a margin of 17 to 22 orders. The GPU
question (A8) is worth settling for bounded jobs and record hunting. It cannot
move the solve.

## 6. What kind of idea could supply 17 orders

The anatomy of the bulge says where the orders are. The tree is fat because
between about 60 and 200 placed cells an interior cell placed against two known
edges has, on average, at least one candidate: (pieces left) x 4 / 17^2 is
above 1 while more than 72 interior pieces remain. The width at depth d is the
number of partial fills that are locally consistent; exactly one of them (five
clues) is globally live. The ratio at the peak is 10^39 to 1.

Kinds of idea, by whether they can touch that ratio:

1. **Go faster, go wider, reorder, propagate locally.** Constant factors or
   already harvested. Section 5.
2. **Refute dead prefixes early.** To spend 10^20 nodes, the search must stop
   at about 40 free cells and dispose of each prefix in a few node-equivalents
   instead of 10^19. The remainder problem (about 210 pieces against a fixed
   16-colour frontier, 10^-19 expected solutions) is over-constrained but not
   locally so: every committed test of a necessary condition (parity, Hall
   matching, ring cuts, arc consistency two cells out) is vacuous or near it.
   This matches what is known about refuting random constraint problems at
   fixed density. I see no candidate here.
3. **Algebraic counting with exact distinctness.** Inclusion-exclusion or
   multilinear detection over 256 pieces costs of order 2^256 = 10^77. Worse
   than the tree.
4. **Information about the solution from outside the search.** In the model
   one certain placement ahead of the peak is 2.9 orders. So 17 orders is six
   certain placements, or 56 bits (20 orders: 66 bits). Soft information
   counts pro rata: k hypothesised placements, each right with probability p,
   are worth k x log2(784 p) bits, so twelve placements each known at 3.5%
   (27 times the prior) would do. This is the only kind whose arithmetic
   reaches the gap. Its sources would be a predictor of which prefixes are
   live, or a leak from the way the instance was made.
5. **Different physics.** Quantum backtracking gives a square root: 10^20
   coherent steps times polynomial overhead. Not a 2026 option.

So the program's exact-solving question becomes one sentence: **where could 56
to 66 bits about the planted solution come from?** Every proposal can be
priced in that unit before any code is written.

### Best candidate: measure whether ranking information accumulates with depth

Ranking is the one multiplier in the table with a reported gain that is a
method, not hardware, and the one whose size is not a constant by nature.
Nobody has measured how it scales.

The law. In a planted ensemble the posterior probability that a given valid
prefix is the live one is proportional to its number of completions. A scorer
estimates the log of that number from features of the prefix (complex theory
does it from the colour histogram of the unused pieces and the frontier). If
the scores of valid prefixes at one depth are spread like a normal law with
standard deviation sigma (natural log units) and the scorer is calibrated,
the live prefix sits about sigma standard deviations above the mean
(size-biased sampling), and a best-first search visits a fraction
Phi(-sigma) of the prefixes before it. Gain = 1 / Phi(-sigma):

| sigma | 1 | 2.45 | 4 | 6 | 8.5 | 9.3 |
|---|---|---|---|---|---|---|
| gain | 6 | 140 | 3 x 10^4 | 10^9 | 10^17 | 10^20 |

McGavin's 10x10 result (a gain near 140 at one row) corresponds to sigma near
2.45. The gap needs sigma near 8.5. The question is whether sigma grows with
the depth of the ranked prefix, and how fast. If the variance of the log score
grew linearly with depth, the gain would grow exponentially and ranking at
three or four rows would be worth many orders. If it saturates after the frame
row, ranking is a constant near 100 and this door is closed too.

My prior is that it saturates. A second-order expansion of the pairing count
gives, for the histogram part alone at 48 placed pieces, a spread of about 0.3
natural log units (argued, rough): parity and totals are conserved by every
valid prefix, so the first-order terms cancel. The frame row is different
because five frame colours with few pieces each fluctuate strongly. I give 20%
to "the log gain keeps growing past the second row" and 3% to "it reaches 10
orders by depth 64".

The test (not run; no 16x16 search involved, only sampling on generated
boards):

- Instances: 200 generated planted 16x16 boards with the official colour
  counts (5 frame colours at 12 joins, 17 interior at 24 or 25), five clues
  read off the planted board at the official positions.
- At each depth d in {16, 32, 48, 64} of the row scan: draw 10^4 valid
  prefixes approximately uniformly by sequential Monte Carlo (section 7),
  score each with the complex-theory completion estimate for its unused
  pieces and frontier, and score the planted prefix the same way.
- Statistic: q(d), the fraction of sampled prefixes that score at least as
  high as the planted one; gain(d) = 1 / mean q(d); sigma(d) from the spread
  of scores. Null model: the same with a scorer that sees only shuffled
  features (gain 1 by construction, checks the sampler).
- Cost: 200 x 4 x 10^4 prefix scorings, a few core-hours in plain code.
- Kill: gain(64) below 10 times gain(16), or mean q not falling between 32
  and 64. Then ranking is a constant and the multiplier table stands.
- Promote: log10 gain growing by at least one order per row from row 2 to
  row 4. Then the next step is a better scorer (the same protocol measures any
  scorer, learned ones included) and an estimate of sigma at depth 100.

Whatever the outcome, the protocol is the right meter for the whole kind
(item 4 above): it reports any predictor of liveness directly in orders of
magnitude at 16x16 scale, without a solver run, and it replaces small-board
solve-rate comparisons that cannot separate methods (agenda A17 notes that
8x8 c=10 does not).

## 7. The first experiment: measure the real puzzle's tree by sequential Monte Carlo

The largest term in the price range is assumption A3: nobody has a current,
documented measurement of the official set's tree at depth. A Knuth-style
measurement fixes that for under one core-hour. It is a sampling measurement,
not a search. Not run tonight.

**Why not plain Knuth probes.** Measured tonight on a generated 6x6 with a
31,055-node tree (`out_A.txt`): 20,000 probes give a mean within 7% of the
truth, but a single probe has a coefficient of variation of 12.9 and a median
of 1,144, a factor 27 below the mean. Batches of 100 probes range from 0.18 to
12.8 times the truth; batches of 1,000 from 0.58 to 1.90. The log of the
squared coefficient of variation grows by about 0.14 per level here.
Extrapolated to depth 160 that is a squared coefficient near 10^10, so of
order 10^12 probes for 10% precision, a few thousand core-hours, and the
extrapolation is itself uncertain because probes die: near the peak the mean
branching is between 1 and 2, so a probe meets a cell with no candidate about
once in four steps.

**What to run instead.** The population version of the same estimator
(sequential importance sampling with resampling; Chen's stratified sampling is
the reference in `memos/literature-2026.md` section 6). It is unbiased for the
width at every depth.

- Order: row scan from the bottom row, five clues pinned (the 10^40.49 order).
  A second run with the centre clue only.
- Population N = 10^6 partial boards. At each cell, replace every board by all
  its valid one-cell extensions, record the ratio r_d = extensions / N,
  resample N of them uniformly. The estimate of the width at depth d is the
  product of the r values.
- Run to free-cell depth 175, which covers 99% of the nodes in the model.
  Report log10 width at every depth against `out_e2_profile.txt`, and the
  number of distinct depth-64 ancestors still alive (the diversity check).
- Five independent seeds. Expected precision: per step the relative variance
  is about (squared coefficient of variation of the offspring count) / N, at
  most a few times 10^-6, so about 10^-3 over 160 steps, that is a few percent
  on the peak width if prefixes are not wildly heterogeneous. The spread
  between the five seeds measures that directly. If they disagree by more than
  0.2 orders, raise N.
- Cost: 10^6 boards x 175 cells x a few dozen candidate tests, minutes per run
  in compiled code. Under one core-hour for all runs. Past the peak the
  genealogy collapses and the estimate becomes heavy-tailed; that part adds
  less than a factor 2 to the total and is not needed.

**What result changes the direction.**

- Measured peak within 0.5 orders of 10^39.2: the price stands at 10^40.5,
  the range shrinks from plus or minus 1.5 to plus or minus 0.5 orders, and
  exact solving by search is closed as a matter of record. Constant-factor
  work (A8) is then justified only for bounded problems.
- Measured peak 3 or more orders below the model: the designed set is not
  typical, and the per-depth ratio curve shows at which rows the real pieces
  constrain more than the ensemble. That would be the first exploitable
  structural fact about the instance, and the program should turn to it.
- Measured peak 3 or more orders above: the designers hardened the set, the
  2008 figure used another convention, and the price rises accordingly.

The same code, pointed at Owen's 10x10 set_2 if its pieces are obtained,
delivers number (1) of agenda item A7.

## 8. Probabilities

Brute force with every credible multiplier (ranking 100, tuning 2, GPUs where
the budget allows), tree at the model value of 3.1 x 10^40:

| Resources | Effective nodes | Probability of a complete solution |
|---|---|---|
| Three desktops, one year, no paid compute | 10^18 raw, 2 x 10^20 effective | 10^-20 (10^-22 to 10^-18) |
| 1,000 core-years | 3 x 10^18 raw, 0.6 to 2 x 10^21 effective | 5 x 10^-20 (10^-21 to 10^-17) |
| 10^6 core-years | 3 x 10^21 raw, 0.6 to 2 x 10^24 effective | 5 x 10^-17 (10^-19 to 10^-14) |

Main reason they are not higher: the tree is 10^40 and the only multipliers
with evidence are constants worth under 4 orders together. The numbers are
conditional on the model being right to within its range. The chance that it
is wrong by 15 or more orders in the favourable direction is not something I
can compute; the 2008 sampling figure and a volunteer project reporting
3.5 x 10^19 nodes a year with a best board of 466 both argue against it, and I
would not defend any figure above 10^-9 for any of the three rows.

## 9. The best idea I rejected

**Attack the generator instead of the puzzle.** The official solution was
produced by a program in 2007. If it was a deterministic function of a small
seed (2^32 to 2^48), the search space is the seed space times a list of
plausible generator designs, perhaps 10^13 to 10^16 cheap board generations,
each checked against the official piece multiset by a hash. That would supply
all 40 orders at once.
Rejected because the algorithm is unknown and the hypothesis list has no
natural end: the balanced colour counts (24 or 25 joins per colour) and the
absence of duplicate or symmetric pieces imply rejection or repair steps whose
order of random draws cannot be guessed, a failure tells nothing, and I put
the chance of success under 1%. It costs little, so it is worth one bounded
attempt by someone with a specific generator hypothesis, not a program line.

## 10. Notes for the merge

- Fix on the site: the five-clue time at 10^8 nodes per second is about
  10^25 years, not 4.9 x 10^32 (`going-fast.mdx`, and the McGavin page already
  flags the mismatch).
- The one-clue "14,702 solutions" figure is irrelevant to the price: the
  five-clue route is cheaper by a factor of about 600 and has one solution.
- Brendan's 10x10 and 9x9 piece sets are not in the repository, so the model
  could not be run on the community's largest solved instances tonight.
  Obtaining them would add two strong calibration rows.
- Files: `solve-path-5/bulge.py` (model), `e2_price.py`, `e2_profile.py`,
  `e2_orders.py` (official set), `small_check.py` (enumeration, Knuth probes,
  hard-ladder cross-check), and their `out_*.txt`.
