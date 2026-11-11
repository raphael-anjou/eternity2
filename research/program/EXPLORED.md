# Explored territory

What has already been searched heavily on Eternity II, compressed from
`web/content/research/build/approaches-map.mdx`, `build/dead-ends.mdx`,
`why/walls-and-methods.mdx`, `open-problems.mdx`, the lab experiments and
`research/LEDGER.md`. A new idea that lands inside one of these cells is a
**variant**, not a new direction. Variants are allowed only when they name the
specific mechanism the earlier attempt lacked and the page that recorded where
it stopped.

Status: **saturated** (many strong attempts, a measured stop), **mapped**
(tried here or in the community, stop understood), **thin** (tried once or
only argued about), **open** (listed on `/research/open-problems`).

## Systematic search

| Family | Status | Where it stopped | Page |
|---|---|---|---|
| Fixed / dynamic fill orders, comb search | saturated | lever on tree size, not on ceiling | build/backtracking/fill-order |
| Break-tolerant DFS with scheduled late mismatches | saturated | 467 (Verhaard), 470 (Blackwood) | build/solvers, records |
| Restart portfolios, heavy tails | saturated | changes which boards, not the ceiling | build/backtracking/restarts |
| Arc consistency, Régin all-different | mapped | pruning fades about two cells out | build/reduce/* |
| No-good learning, shared transposition tables | mapped | no-goods outrun memory at 16x16 | build/reduce/nogood-learning, dead-ends |
| Exact cover / DLX | mapped | small boards and counting only | build/exact/exact-cover-dlx |
| Meet in the middle (BANDSAW) | mapped, open | about 20x memory per extra mismatch | build/exact/meet-in-the-middle |
| Exact tail endgame | mapped | plateaued | build/exact/exact-tail-endgame |
| Solver engineering, JIT codegen | saturated | speed only; more attempts for record hunting | build/faster/*, lab/.../jit-backtracker |

## Encodings

| Family | Status | Where it stopped | Page |
|---|---|---|---|
| SAT / CSP encodings, cube and conquer | mapped | full board stalls; sub-board proofs | build/exact/sat-csp-encodings, dead-ends |
| LP / ILP relaxations, relaxation bounds | mapped | 420 to 440, ILP wall at 8x8 | build/exact/lp-relaxations, dead-ends |
| MIP halo proofs of rigidity | mapped | record basins locally optimal | why/rigidity-wall |

## Local and stochastic search

| Family | Status | Where it stopped | Page |
|---|---|---|---|
| Local search, ALNS, hyper-heuristics | saturated | 461 academic, 460 class here | build/local-search/local-search-alns |
| Simulated annealing, parallel tempering | saturated | below the backtrackers; PT the only new-basin source | build/local-search/parallel-tempering |
| Beam search, beam producers | saturated | breadth stalls in the interior | build/construct/beam-search, lab/.../beam-width |
| Genetic algorithms, crossover, recombining two good boards | mapped | crossover fights the structure | build/local-search/evolutionary, dead-ends |
| Iterated maps / divide-and-concur | thin | tested once | build/exact/iterated-maps |
| Ant colony, MCTS with cheap prior, gradient descent on a soft board | mapped | dead ends | dead-ends |
| Fake 480 and repair, repair seeded by exact prefix | mapped | dead ends | dead-ends, lab/.../repair-study |

## Learning

| Family | Status | Where it stopped | Page |
|---|---|---|---|
| Corpus priors, learned move ordering, anti-pattern mining | saturated | fast to the plateau, not past it | build/learning/* |
| PRIOR, PALIMPSEST, LODESTONE, KEYRING, REPLAY | mapped | 451 to 463 | lab/.../learning/* |
| Transformer board generation, heuristics learned on small boards | mapped | dead ends | dead-ends |

## Propagation and inference

| Family | Status | Where it stopped | Page |
|---|---|---|---|
| Survey propagation, belief-propagation ordering | mapped | signal fades two cells out | dead-ends |
| Tensor-network counting, lattice reduction | mapped | dead ends | dead-ends |
| Pattern databases, macro-tiles, local cluster enumeration | mapped | dead ends | dead-ends |
| Forbidden-pattern / border-supply / Eulerian closure pruning | mapped | no measurable gain in search | dead-ends, why/forbidden-patterns |

## Construction orders and geometry

Perfect rows, centre-out, concentric rings, clue-to-clue corridors, colours
first then pieces, forced diverse borders, fluid frame, frame-is-not-the-basin:
all **mapped** (dead ends or plateaued; see dead-ends and the lab pipelines
CAS, CLOISTER, GAUNTLET, LADDER, MIDDEN, MOSAIC, STAGED).

## Hardware

GPU (measured ports, no tuned full-board solver: **open**), FPGA (designed,
never driven: **thin**), distributed farms (**saturated**), quantum (**thin**,
zero runs).

## Theory already on the site

Phase transition and hardness peak, entropy area law, rigidity, sigma cycles,
no height function, permutation-code wall, flux invariants, ring purity, border
balance, parity defect floor, piece theft, constraint immediacy, rare-colour
geography, the 470 annealed landscape. Several of these still lack a matched
null model (see the deferred findings in `research/LEDGER.md`).

## Explicit open problems

470 to 480; strict five-clue 465 to 466; Owen's hint-free 10x10 set_2; crossing
a rigidity basin; the meet-in-the-middle memory wall; a diversity mechanism
beyond parallel tempering; whether local rigidity follows from area-law
distinctness; whether the hardness peak produces the no-forced-moves profile;
whether the sigma-cycle geometry caps constructive search.
