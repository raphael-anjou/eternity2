# Wiki page candidates from the groups.io archive

The exhaustive, prioritized catalogue of wiki pages the community archive
supports, synthesized from digests 0001–0015 (msg_num 8–11823, 2000-10 →
2026-03) and checked against the current content inventory
(`web/content/research/`, 2026-07-02).

Conventions:
- Every entry is grounded in specific digest-sourced messages ("key msgs").
  Nothing without solid sourcing made the list.
- `section` uses the wiki's layout: `build/concepts` (technique shelf),
  `build/solvers` (engine catalogue), `build (flat)` (reference pages next to
  benchmarks/known-facts/history — including root-level records/reference
  where noted), `why` (findings), `lab`.
- Entries marked **(upgrade)** target an existing page rather than a new one.
- Priorities: **P1** community-defining technique/story with rich sources;
  **P2** solid but narrower; **P3** nice-to-have.
- Citation form for pages: `https://groups.io/g/eternity2/message/<n>`.

Tally: **9 × P1, 20 × P2, 10 × P3 = 39 entries.**

---

## edge-slipping — Edge slipping: the mismatch that wins prizes
- section: build/concepts
- kind: concept
- people: Louis Verhaard (invented/deployed for the 467); Brendan Owen (theory extension); Max (mm_4761, counting formula); Joshua Blackwood ("breaks", the modern descendant)
- key msgs: 6328 (definition: "putting a piece on the board that does not match… its neighbours"), 7321 (the 467 method: edge slipping gated at chosen depths; found 50+ times; most 467s had clean score 247), 6891 (first-person 467 account, 247 flawless pieces), 6390 (Max's partial-count formula S(480−N) ≈ 2^(N−1)·C(420,N)·S(480); ~60x cost per extra edge), 6408/6412 (Owen extends complex theory to slips: 13 slips ⇒ 2.05e35× more solutions), 6423 (Verhaard's Markov chain over (depth, slips) to optimize the slip array), 5767/5770/5772/5780/5787 (the Verhaard–Max method exchange, peak-at-170 diagnostics), 6687 (independent 82-day reproduction of a 467 with the public solver), 10051 (Blackwood's no-adjacent-breaks discipline — the lineage's modern form)
- scope: Teaches the single most consequential partial-score technique in E2 history: deliberately placing mismatching pieces at scheduled depths so the search reaches deep, high-scoring boards it could never reach cleanly. Covers the counting theory (why each extra matched edge costs ~30–100x), the slip-array/depth-gating design, and the direct lineage from Verhaard's 2008–09 machinery to Blackwood's break indexes. Boundary: the break-index mechanics of the modern solver stay on the Blackwood page; this page is the concept and its mathematics.
- already-covered: history/history-2 narrate the 467 and the record wave; blackwood.mdx covers breaks as one lever of one engine; parity-arguments tells the 479 story. No page teaches slipping as a technique with the S(480−N) mathematics and the Verhaard slip-array design — the biggest single gap in the concepts shelf.
- educational-angle: interactive "slip budget" explorer — a board fill where the user sets N allowed slips and gate depths, and watches both the reachable score and the solution-count blow-up (2^(N−1)·C(420,N)) move; overlay the real 467/468/469/470 boards' break positions.
- priority: P1

## fill-order-theory — Scan orders: why the path decides everything
- section: build/concepts
- kind: concept
- people: Brendan Owen (scanline optimality, decreasing squares), Louis Verhaard ("magic 10x16 square", comb search), Max (order measurements), doc_s_smith & Txibilis (automated vs hand-tuned strategy race), leenstrab/Bruce (scan-order distance metric), Juraj Pivovarov (visualizer), istarinz ("SPEED" answer)
- key msgs: 347/350 (the 2007 fixed-vs-dynamic debate; least-options lineage from E1), 2714 (Owen: "Best Search Order" — scanline near-optimal after the hint), 2392/2425 (fixed beats dynamic empirically), 2896/2928 (the 89,794-node strategy race, human vs machine), 5860 (why fixed paths win: one word, speed), 5868/5879 (the magic 10x16 square: frontier shape explains order quality), 6015/6023 (all 8 scanline orientations measured, ~2.7e40), 6112/6126 (comb search for high scores), 5980/5983/6062 (border-piece equivalence pruning, ~10% — "finally something that beats simple scan-row"), 3124 (decreasing squares), 6142 (hybrid increasing-squares→scanline), 8179/8187/8195 (scan-order distance metric + node-count experiments), 9013 (scan-order visualizer), 9146/9150 (21valy: basic scanline beats border-tree; +42% border endgame), 11546/11562/11573 (Owen's 2025 join-probability and search-order graphs)
- scope: The community's twenty-year experimental science of fill orders: why row scan wins on a balanced design, what the search frontier's shape has to do with it, how orders were measured and raced, and which deviations (comb, hybrid, spiral, border-first) pay off for which goal (full search vs high score). Boundary: complex-theory keeps the funnel math; this page is the practice, the tournaments and the folklore-turned-measurement.
- already-covered: complex-theory.mdx has "the order decides the funnel" (one section, theory-side); solvers/index mentions fill order; beam-search/blackwood touch order in passing. No page collects the empirical order science — the digests' single most recurrent technical topic.
- educational-angle: animated frontier visualizer — pick an order (scanline, spiral, comb, increasing squares), watch the frontier shape and a live node-count estimate; reproduce the "magic 10x16" rule visually.
- priority: P1

## verhaard-eii — Verhaard's eii, decoded: the 467 engine
- section: build/solvers
- kind: concept
- people: Louis Verhaard (author); Max (sparring partner), JSA (independent benchmark), Anna Karlsson (the entry's name)
- key msgs: 5940 (release: "I am stuck… my only hope is brute force", 50-50 prize terms), 6275 (documentation + decoder release; 467 found 40+ times by users), 7321 (depth-gated slipping design; clean-score 247 base), 6423 (Markov-chain optimization of search order and slip array), 5767–5787 (heuristics measured by the node-depth peak ~170 — "equivalent to eliminating one interior colour"), 6112 (comb-search orders), 6687/6571/6653 (JSA's 82-day public benchmark: 4M 463s, 625 466s, two 467s — the 466→467 rarity gap), 7439/7451 (relocation to shortestpath.se; "my wife submitted my best solution… and won 10 000 dollars"), 7306 (the 249 clean-score run), 9890 (2019 confirmation the 249 partial is real), 10582 (Bucas: the 467's constraint was self-imposed — Verhaard misread the rules)
- scope: A solver-catalogue page for the engine that held the record for twelve years, structured like the Blackwood page: tileability-maximizing heuristics, comb-search orders, the depth-gated slip array, the distribution model (community users as the compute farm), and the measured 466→467 gap. Boundary: the prize story lives in history part I; this page is the machine.
- already-covered: history.mdx narrates the road to 467; records.mdx row cites shortestpath.se; blackwood.mdx is the only solver page. The solvers section has a one-engine catalogue — this is the missing second engine, and the archive documents it unusually well.
- educational-angle: recreate JSA's benchmark curve — an interactive log-scale histogram of scores found per CPU-day (2M 463s → 2 467s), letting the user feel the exponential rarity ladder; side-by-side lever comparison with Blackwood.
- priority: P1

## solver-engineering — The fast backtracker: engineering below the algorithm
- section: build/concepts
- kind: concept
- people: Marc Lebel (first public fast C++ solver), Nathan (minimal perfect hash), cambdacambda, istarinz/Tom (generated C, multithreading, AVX2 plans), Michael Field (70M/s code generator), Arnaud Carré (114M rec/s), Adam Miles (BMI2), Joshua Blackwood & Jef Bucas (cache-fitting, code generation), Yendor & Reinout Annaert (modern single-core numbers)
- key msgs: 1704/1734/1831 (the Lebel-thread deep dive: bipieces, 4D edge-lookup vs Nathan's 1024-key minimal perfect hash, cache misses, recursion vs unrolling), 4683/5480/5438 (non-recursive backtrackers; per-puzzle code generation with Intel compiler), 5804/6212 (multithreaded 100M+ pps; 558M pps on a Core i7), 3843/3936/3946 (node-vs-step counting conventions — why period speed claims don't compare), 9739/9740/9746 (the pieces-per-second convention pinned down, and its bias), 9796/9808/9809 (BMI/BMI2 bit-extract, 64-bit candidate packing), 10056 (Blackwood: L3-cache-driven single shared piece cache), 10065/10078 (libblackwood: Python emitting fast C; 2x from the port alone), 11633/11634 (2025 numbers: 72.7M/s C++, 68.4M/s Rust-from-Blackwood), 9003/9004 (Field's cycle-level analysis: ~26 cycles/tile, half stalled on memory), 11657 ("we will not make a dent" — the honest ceiling)
- scope: The craft layer every record solver shares: table layouts that fit in cache, perfect-hash edge lookups, non-recursive/generated inner loops, bit tricks, thread pinning — and the measurement discipline (what a "node" is, why Mpps claims mislead). Ends with the structural point: 20 years of engineering bought ~10x per core while the problem needs ~10^20. Boundary: per-engine heuristics stay on the solver pages.
- already-covered: blackwood.mdx has "Make it fit in cache" for one engine; prune-vs-speed.mdx makes the strategic argument without the engineering content. The 2007 Lebel thread, MPH lookups, code generation and the node-counting convention wars are nowhere.
- educational-angle: an animated memory-layout diagram (piece table → cache lines) plus an interactive "cycles per placement" budget calculator; a timeline chart of best single-core pps 2007→2025 against the flat record line, making the prune-vs-speed point viscerally.
- priority: P1

## restarts-heavy-tails — Restarts and heavy tails
- section: build/concepts
- kind: concept
- people: antminder (first claim), Txibilis (the 500-puzzle experiment), louis.verhaard (mountain-sampling rationale), Joshua Blackwood (the 50-billion-node cap), Peter McGavin (randomized-restart 9x9 census)
- key msgs: 2420/2822 (the defining experiment: restarts win only 52% of races but the tail is decisive — losses ~90M nodes vs wins up to 50,463M), 5182 (Verhaard's explore-then-exploit rationale: sample mountain ranges, then climb the Himalaya), 3089 (deadline-aware risk-seeking search: prefer 1% chance in the window over better average), 6798/6801 (PRNG hygiene: period limits, Mersenne Twister), 10066 (Blackwood's restart threshold: 50 billion iterations, "arbitrary"), 9342 (McGavin's 9x9 solutions from ~20 scavenged PCs running randomized restarts, manually restarted when stuck), 9688 (the 10x10 solve as a law-of-large-numbers restart campaign)
- scope: Why backtracking runtimes on E2-class puzzles are heavy-tailed, and why every serious engine restarts: the community measured this in 2007 and every record since (467, the 10x10, 468–470) was a restart portfolio. Connects the archive's experiments to the Gomes–Selman–Kautz randomized-restart literature and to Las Vegas algorithm theory. Boundary: the specific engines' restart parameters stay on their pages.
- already-covered: blackwood.mdx mentions the 50B cap in one section; nothing teaches the heavy-tail phenomenon, the 2007 experiment, or restart policy as a concept. A canonical CSP topic with unusually good community data.
- educational-angle: live runtime-distribution simulator — run many small-board searches, plot the survival curve on log-log axes, toggle restarts on/off and watch the tail collapse; slider for restart cutoff.
- priority: P1

## design-recipe — Designing the hardest puzzle: the E2 recipe
- section: why
- kind: finding
- people: Brendan Owen (the recipe and derivation), Alex Selby & Oliver Riordan (the actual designers, per Owen/press), Dave Clark (the Monckton generation-process phone call), Al Hopfer (colour-in generation theory), Michael Field (piece design-space census)
- key msgs: 1947/1962 (the hardest-puzzle recipe: compact shape, no symmetric/duplicate pieces, separate border/interior types, flat distribution, ~1 expected solution; I = (196!·4^196)^(1/392) ≈ 17.14 → 17), 2164 (hardest-parameter table for all board sizes), 5243/5263 (empirical check: for the 5/17 split, scanline is the *only* good order — "no weak areas to start tiling from"), 4968 (the flip side: a designed 16x16 with ~10^40 solutions), 1054 (the measured flat distribution at launch), 8014–8034 (piece design-space census: 256 pieces from ~21,000 possible, symmetric forms avoided), 4177 (Clark's Monckton call: judges' keyboard entropy, regenerate-until-happy, printed once and vaulted), 6842/6844 (Hopfer's colour-in generation theory and the deliberate balance), 6892/6894/6900 (generation bias: generated-from-solution over-samples friendly sets), 9538/9539/9541 (who wrote the generator: Verhaard's recollection, McGavin's Selby guess), 716/901/2697/3373 (the Selby/Riordan design credit trail)
- scope: How you design an unsolvable-but-solvable puzzle, as reverse-engineered by the community in the launch year and confirmed over two decades: the parameter derivation, the anti-heuristic properties (flatness, no duplicates, no symmetric pieces), and what is actually known about the generation process. Boundary: the hardness-peak *measurement* stays on phase-transition; this page is the recipe and the provenance lore.
- already-covered: phase-transition.mdx covers the 17-colour hardness peak (and the README lead says to add Owen's msg 1947 derivation there); rare-color-geography covers the border split as a signature; history has the Monckton-call anecdote in passing. The full recipe — flat distribution, symmetric-piece avoidance, "no weak areas", the design-space census, the generation-process testimony — has no home.
- educational-angle: an interactive puzzle-designer — sliders for colours/split/distribution flatness generating live expected-solution and difficulty readouts, with "E2's choices" highlighted at every optimum; a provenance timeline of who-knew-what about the generator.
- priority: P1

## tooling-lineage — The community toolchain, 2007–2026
- section: build (flat)
- kind: reference
- people: Dave Clark (eternity2.net formats, R&D solver), eii4me & trans.spam/Thomas (manual-placement programs), Yannick Kirschhoffer (Eternity II Editor), Fred/Eternity Blogger (E2Lab), Ole Knudsen (e2walker), Jef Bucas (e2.bucas.name, libblackwood), David Barr (open-source solvers), mtmidgee (E2Play), Michal Dobrogost (eii-pgen)
- key msgs: 3571/3572/10952 (the e2pieces.txt/e2hints.txt formats and their BOINC-era origin), 1063/1073 (the CRC-16 checksum protocol — verify without sharing), 3541 (Manual Placement Program), 5686/6765 (E2_Manual, then SourceForge), 4544/6679/9064 (Eternity II Editor 1.0 → solver-equipped 1.4 → author resurfacing), 7071/7148/7150 (E2Lab origin story, the removed "magic button"), 7056 (e2walker, "the only program of its kind"), 6572 (WhichWayToGo fill-path evaluator), 9955/10590 (e2.bucas.name launch and revamp — boards as URLs, the de-facto record book), 10037/10078 (Blackwood's solver and libblackwood), 11088 (Barr's solveforscore), 10208 (E2Play), 11033/10970 (the piece-file survey — "it's a mess!" — and the canonical list), 11752 (eii-pgen tested generator), 7284 (the E2Lab 471 popup — why tooling reliability matters to records)
- scope: A reference map of nineteen years of community software: what each tool does, where it lives now, which formats it speaks, and the interoperability story (formats, checksums, the piece-copyright constraint that shaped everything). Doubles as the newcomer's "what do I download" page the list keeps re-answering. Boundary: solver *algorithms* stay in build/solvers; this is the tool census.
- already-covered: scattered one-line mentions (bucas.name in records/known-facts, editors in history). Nothing collects the lineage, the formats, or the live links — an explicitly requested and fully sourced gap.
- educational-angle: an interactive timeline/genealogy diagram of tools (manual-placement → editors → viewers; solver formats as edges), each node linking to its source message and, where alive, its current home.
- priority: P1

## complex-theory-provenance — (upgrade) Complex theory: full lineage, proof, and validations
- section: why
- kind: finding (upgrade to why/complex-theory.mdx)
- people: Brendan Owen (author), Peter McGavin (LaTeX transcription, C reference, validations), louis.verhaard (defender), jwortmann (Julia extension)
- key msgs: 5197/5209/5210 (the 2008 "Nailed the theory" posts and formulas), 5193 (independent Verhaard estimate converging), 8125/8126 (the 256·(1−1/e)=161.8 peak-depth proof), 8924 (the 14,702 / 1.15e7 estimates as early as 2011), 9188 (McGavin's complex_theory.pdf LaTeX transcription, 2013), 11197/11193/11198 (complex_theory.c, the headline numbers, spiral-path demo, 2024), 11248 (the heuristics caveat), 8793/8801–8803 (9x9 exhaustive validation), 9686/9688/9725/9693 (the 10x10 solve landing inside predictions — "truly just as hard as predicted"), 7810/7823/7861 (the 2010 defense against Monte-Carlo estimators), 11546 (Owen's 2025 refined join probabilities), 11603 (Julia extension with invalid joins + slip array), 11560 (Owen: never formally published outside the group)
- scope: Upgrade the existing page's "Provenance and validation" section into the full story: Owen 2008 → PDF 2013 → C 2024 → this site's TS port, plus the closed-form peak-depth proof, the exhaustive-search validations (9x9, 10x10, clue puzzles, border rows), and the 2025 refinements. Several of these are standing README leads.
- already-covered: why/complex-theory.mdx exists and is strong on the idea/funnel/regimes; its provenance section is thin relative to what the digests now support (the README lists this as a ready lead).
- educational-angle: a "theory vs reality" scatter chart — every exhaustively-searched benchmark (6x6…10x10, clue puzzles, border rows) plotted predicted-vs-measured, showing the order-of-magnitude agreement that made the community trust it.
- priority: P1

## records-corrections — (upgrade) Records table: sourcing and classification fixes
- section: build (flat) — targets root records.mdx + RecordsView + known-facts record table
- kind: reference (upgrade)
- people: Joshua Blackwood, Peter McGavin, Jef Bucas, Carlos Fernandez, Bruno Gauthier, "Puzzled_For_Eternity", Naohiro Takahashi (claim only)
- key msgs: 10032/10033 (the 468 relay: Reddit via Puzzled_For_Eternity, board verified by Bucas — fix the relay attribution), 10045/10117/10554 (469 and 470 are the *same* 1-mandatory-hint regime: piece 139@I8, no clue pieces placed — the canonical/variant split is unsupported), 11046 (under contest rules only the starter was mandatory — the 5-clue "canonical" is our convention, not the contest's), 11074/11076 (Gauthier's 460: his own program produced the grid; the Editor only rescored it), 10754/10802 (NS-1 credit split: Hopfer stated the condition, Fernandez did the 4-minute quadrant solves), blackwood-page citation of msg 11905 (post-dates the export — validity rests on Raphaël's firsthand copy; keep the note), 9694/9697/9698 (Takahashi 468: unverified, different-rules variant — never promote), 7284/7383/8246/8636 (the unverified 471/472/476 claims that must stay out)
- scope: Not a new page — the batch of corrections digests 0014/0015 verified programmatically against the primary messages, listed in the README as "RECORDS CORRECTIONS PENDING". Reclassify 469/470 consistently, fix relay/method attributions, split the NS-1 credit, and keep the unverified-claims quarantine explicit.
- already-covered: records.mdx/RecordsView carry the rows; the digests show three specific misattributions and one unsupported classification.
- educational-angle: n/a (data hygiene), though a "how we verify a record" sidebar would pair well with the claim-filtering page below.
- priority: P1

---

## comb-search — Comb search: fill orders for high scores
- section: build/concepts
- kind: concept
- people: Louis Verhaard (named and used it), Max (independent convergence), Brendan Owen (posed the question)
- key msgs: 6111/6112 (Owen's question; Verhaard: rows horizontal, remaining rows vertical — "the lower the score you aim for, the longer the teeth"), 6126 (Max within 1 edge with a near-identical order), 5985/6123 (boustrophedon and snake/ripple generalizations), 5787 (why high-score search lives deeper than the full-search peak)
- scope: The high-score-specific fill geometry behind the 467: why chasing partial scores wants a different frontier than full search, and how the comb's tooth length trades depth for breadth. Boundary: could be a section of fill-order-theory or edge-slipping rather than a standalone page — recommend folding unless the interactive justifies a page.
- already-covered: nowhere on the wiki (grep: zero hits). Thin as a standalone; strong as a named section.
- educational-angle: side-by-side frontier animation of scanline vs combs of increasing tooth length, with a target-score slider morphing the optimal comb.
- priority: P2

## transposition-tables — Transposition tables and failure caching
- section: build/concepts
- kind: concept
- people: Michael Lindström (main experimenter), Max & louis.verhaard (chess-style advice), Mocsi/Laszlo (Failure Lookup Table), Joseph DeVincentis (the correctness fix)
- key msgs: 6138/6144/6145/6152 (the 2008 scheme: store unsolvable board states; fixed memory, hash, overwrite), 6218 (improved: no per-piece state, 75–80% of branches discarded), 8854/8866/8871/8886 (the restore-list saga: memoized pruning that silently loses solutions, and the multiple-restore-lists fix), 8875 (measured: 133M → 30M iterations on a 6x6 ring), 8887 (Mocsi's FLT, abandoned over a bug), 9609/9610/9618 (duplicate-subtree detection in row scan, 2016)
- scope: Teaches state caching for edge-matching search: what a "position" is when the frontier is a colour signature, why memory budgets force chess-style overwriting, and — the archive's distinctive contribution — the correctness traps, dissected publicly over 30+ messages. An honest page: real savings on small boards, unproven at 16x16.
- already-covered: nowhere (grep: zero hits). A standard technique with an unusually instructive community failure story.
- educational-angle: animated hash-table view during a small-board search — watch signatures being stored, hit, overwritten; a "break it" toggle that reproduces the lost-solutions bug and its fix.
- priority: P2

## metatiles-bipieces — Metatiles: bipieces, 2x2 blocks and the domain-reduction mirage
- section: build/concepts
- kind: concept
- people: ped7g (sparked the debate), Max (the "in sync every 4 pieces" argument), louis.verhaard (cache argument), Michael Field (7x1 frame metatiles), Tom Aspinall (the 2020 paper), Patrick Hegland (2x2 assembly partials)
- key msgs: 1734/1831 (1x2 bipieces: 20–30% speedup; 2x2 rejected as ~4.2x slower), 3044/3054 (the exact 2x2 census: 5,248 / 292,012 / 4,059,952), 5842/5899/5911 (the definitive debate: metatiles are a speedup, not a domain reduction — the constraint frontier is identical), 6114 (Verhaard's symmetrical multi-piece pruning: ~18% of 2x2 blocks share an outside), 7663/7664 (frames from 7x1 metatiles: one frame, a billion internal arrangements), 7675/7677 (duplicate-elimination sizing: memory runs out first), 10055/10059/10061/10077 (the Aspinall et al. 2x2 domain-space paper and member experiments), 11229/11231 (Hegland's 245-piece partials via 2x2 segment assembly — the technique's best modern result)
- scope: The recurring dream of searching in blocks instead of pieces, and the theory of why it keeps disappointing: same constraint frontier, exploding tables — plus the two places it genuinely pays (implementation speed; Verhaard's outside-equivalence pruning; Hegland's assembly partials). Boundary: exact 2x2/4x4 counts stay on the reference-numbers page.
- already-covered: complex-theory.mdx has a "Bigger tiles" section (theory-side); dead-ends has "Enumerate local clusters first" (one project-side paragraph); reference.mdx holds the counts. The community debate, the sync argument, and the 15-year lineage from bipieces to the Aspinall paper are uncovered.
- educational-angle: interactive frontier comparison — step a 1x1 solver and a 2x2 solver over the same board and highlight that their constraint frontiers coincide every 4 pieces; table-size counter running out of memory as block size grows.
- priority: P2

## distributed-solving — Distributed solving: BOINC, syndicates, and core farms
- section: build (flat)
- kind: reference
- people: Dave Clark (eternity2.net), Amos/silversarace (the Syndicate), royale_zerezo (French project), e2dude (E2@home), louis.verhaard (eii as distributed campaign), capiman/Martin & Peter McGavin (top-row farming)
- key msgs: 222/226 (first SETI-style proposal and Owen's impossibility reply), 756/768 (eternity2.net launch on BOINC; skeptics: 100,000 machines change nothing), 3511/3509 (shutdown: 1.6 TFlops, 10^19 operations, mid-460s bests), 1253 (the French distributed site), 3021/3105 (the Eternity 2 Syndicate: prize-sharing by placements contributed), 5474 (E2@home micro-syndicate), 5940/6275 (eii's 50-50 prize terms — distribution as record strategy), 9164/9167/9177 (the 10x10 top-row reservation protocol), 9688/10045 (the modern form: 400+ cores on ranked rows; 469 from ~200 cores), 8106 (EC2 suggested, 2010), 9642 (AWS Lambda searches)
- scope: Both the story and the technique of throwing crowds and clusters at E2: how work gets partitioned (search-space slices, first-row lists), what the prize-sharing economics were, and why every distributed effort confirmed rather than beat the counting argument. Boundary: the social history highlights stay in history part I; this page is the systems view with the full project census.
- already-covered: history.mdx has "Big iron and syndicates" (narrative); prune-vs-speed makes the futility argument abstractly. No page covers work-partitioning protocols or the project-by-project census.
- educational-angle: a "how many computers would it take" calculator fed by real archive numbers (eternity2.net's 1.6 TFlops, McGavin's 400 cores, the 3.17e29 core-years estimate), with a map-style timeline of every distributed project and its fate.
- priority: P2

## gpu-solving — GPUs vs the backtracker
- section: build/concepts
- kind: concept
- people: David Barr (OpenCL, open source), Adam Miles (DirectX 12 / Xbox One X), Michael Field (the definitive negative analysis), trans.spam/Thomas (first OpenCL talk), simon.chapple & James (2007 proposals)
- key msgs: 351/355 (first GPU proposals, 2007), 5407/5409 (CUDA question, GLSL CSP solver, 2008), 7653 (first OpenCL discussion, 2010), 8982/9003/9004 (Field's bandwidth argument: cross-communication or >8KB state — no order-of-magnitude win available), 9360/9364/9367 (Barr's working OpenCL solver + GitHub), 9811/9814/9818/9819 (Miles's compute-shader solver; kernel-design notes on why teraflops disappoint), 9822 (the payoff: 9x9 census exhaustively re-verified in 25.4 h), 9984 (keeping processors busy — the parallelism problem stated), 11598 (2025: rented RTX 4090 at 3.17B placements/s), 10056 (Blackwood: GPU measured, not kept)
- scope: Why the most parallel hardware on earth gives E2 backtracking only a modest constant: divergence, memory access, per-thread state — traced through fifteen years of real implementations, ending with what GPUs *are* good for here (exhaustive verification, cheap cloud sweeps). Boundary: raw speed tables belong to solver-engineering; this page is the architecture argument.
- already-covered: dead-ends has one "just throw more compute" paragraph; benchmarks covers the Miles 9x9 re-verification as an event. The why — Field's analysis, Miles's kernel notes — is uncovered and genuinely teachable.
- educational-angle: animated SIMT divergence demo — 32 lanes stepping a backtrack tree, watch lanes idle as paths diverge; compare effective throughput vs theoretical FLOPs.
- priority: P2

## nogood-learning — No-good learning: mining what can never happen
- section: build/concepts
- kind: concept
- people: Michael Lindström (invalid-placement mining), capiman/Martin (industrialised 2/3-piece clause mining, SAT invalidation service), nc9201 (spare-possibilities eliminations), 21valy & capiman (chain/pile exclusion ports)
- key msgs: 6743 (the collaboration call: ~4,500 invalid single placements, 20–50M invalid pairs estimated), 6951/6955–6958 (possibility-matrix comparison catching bugs), 7050 (nc9201's Hall-set-style "spare possibilities" eliminations, donated), 7198/8702/8707 (chain exclusion / pile exclusion from the Sudoku literature: detects more than AC, active only near the empty board), 10832/10973/11131 (the modern programme: ~68 million invalid 2-piece combinations; ~32,000 invalidations would pin a solution), 10289/10292 (SAT as an invalidation service for others' partials), 6813 (the domino-collapse observation: propagation can tip suddenly)
- scope: Learning clauses about E2 itself — placements and small combinations provably absent from any solution — and what they buy: a shared, solver-agnostic pruning asset, and the honest accounting of how far 68M clauses sit from the ~8.1e9 undecided. Bridges CSP propagation and CDCL thinking. Boundary: SAT encodings themselves stay on sat-csp-encodings.
- already-covered: sat-csp-encodings mentions clause learning inside SAT solvers; arc-consistency covers propagation. The community's explicit no-good-mining programme (2009 → 2026) is uncovered.
- educational-angle: a live possibility-matrix heatmap — click to place a piece and watch eliminations propagate; overlay the mined invalid-pair counts per cell to show how sparse 68M clauses are against the space.
- priority: P2

## lp-ilp-relaxations — LP, ILP and the fractional-piece plateau
- section: build/concepts
- kind: concept
- people: vlastikw363/Vlasta (LP/SAT hybrid models), Jimmy Timmermans (algebraic campaign), okifinoki/Benjamin & bozmo2004/Andrew (implicit solvers), David Munjak (assignment-problem retrospective), Marcus Garvie (modern ILP), Wauters group (MILP + Max-Clique paper)
- key msgs: 5602 (ILP dies at 8x8; 160,254 binaries), 6616/6728/6745 (nonlinear systems, toric ideals, CPLEX BIP; relaxations converge, integrality doesn't), 6905/6911 (the cleanest plateau documentation: LP reaches zero error with fractional pieces — 30% piece 1, 20% piece 2 — and stalls at the equivalent of 420–440/480 when forced integral), 8077 (153k-variable BIP), 7858 (MILP→SAT conversion claims parity), 8791 (Munjak's no-backtracking assignment method: 200–224 pieces then infeasibility), 9683 (MILP + Max-Clique heuristics, arXiv 2017), 11502 (Garvie's ILP solves at 10x10/6-colour scale), 166/627 (the 2007 max-clique framing — the encoding lineage's start)
- scope: The whole optimization-formulation family — LP relaxation, ILP, BIP, assignment subproblems, max-clique — and the one lesson they all teach: the relaxation is easy and uninformative, because fractional superpositions fake matches no board can have. Includes the honest positive: Hungarian-refill inside local search (see alns) is where assignment solving actually pays.
- already-covered: dead-ends "Relaxation bounds" (one project-side paragraph); papers.mdx ranks the academic papers; sat-csp-encodings covers the SAT side. The archive's fifteen years of first-person LP/ILP campaigns and the plateau mechanism are uncovered.
- educational-angle: interactive fractional-board visualizer — show a cell as a pie of piece fractions at LP optimum, then a rounding slider that watches the score collapse; plot the 420–440 plateau.
- priority: P2

## iterated-maps — Iterated maps and divide-and-concur
- section: build/concepts
- kind: concept
- people: Veit Elser (the method, external), JSA (introduced it to the list), Dima (empirical test), dvholten (edge-colouring dual proposals), Wyatt Carpenter (Douglas–Rachford naming)
- key msgs: 9347/9350 (the dual problem formalized; Elser's PNAS "Searching with iterated maps", whose cover was an edge-matching puzzle), 9351 (Dima's test: a simple hill-climber beats the difference map on the paper's own benchmark), 9434/9442/9445/9447 (the 2015 revival: edge-colouring dual, "committing to a colour is almost like committing to a tile", the SRD edge-swap precedent), 11592 (2025: Douglas–Rachford named — the projection-method family finally connected), 6184/6185 (the 2008 triangle/dual-domain ancestor)
- scope: The one famous academic method built *for* edge-matching-like problems: projections between two easy constraint sets (pieces valid / edges matched), why the physics community celebrated it, and what community tests actually found on E2-class instances. A projection-methods primer with an honest scoreboard. Boundary: continuous LP relaxations live on the lp-ilp page.
- already-covered: nowhere (grep: zero hits for Elser/iterated maps/Douglas–Rachford). Explicitly flagged in the task; modest but clean sources.
- educational-angle: animated two-set projection dance in 2D — a point ping-ponging between "pieces legal" and "edges match" constraint sets, converging on toy instances and cycling on hard ones.
- priority: P2

## rotation-sets — Rotation sets and the two-phase dream
- section: build/concepts
- kind: concept
- people: Dave Clark (2007 anomaly hunt), dmitri_ulitski (ILP in 5 seconds), Pierre Schaus (polynomial permutation phase), Juraj Pivovarov (the 108-second demolition), Al Hopfer & dvholten (2022 revival), mrjohngilbert & Varga Laszlo (negative experiments)
- key msgs: 3245/3254/3320/3443/3447 (the 2007 "rotation solutions" saga: anomaly suspected, solved by ILP, written off), 5656/5659/5661 (fast generation, zero filtering power), 8560/8591 (two-step idea; balanced sets "crash and burn super fast"), 9069/9070/9071/9075 (SA ~1000 sets/s vs MILP in milliseconds; the open counting questions), 9400/9404/9406/9411 (the decisive result: fixed-orientation E2 is polynomial/easy — Schaus's JFPC algorithm, Juraj's 100-instances-in-108s demo — so ALL the hardness is in orientations), 10785/10796 (2022: "balanced rotation set" named; balance ≠ placement), 8898–8978 (the checkerboard-balance mega-thread; balanced sets stall backtrackers *faster*)
- scope: The most-reinvented idea in the archive: fix every piece's rotation first, then permute. Teaches the beautiful half (the permutation phase is genuinely easy) and the fatal half (rotation sets are astronomically many, carry no position information, and restricting to them prunes less than it costs). A definitive fold-the-recurrence page.
- already-covered: nowhere as a technique (parity-arguments touches directional counts; dead-ends doesn't list it). Fifteen years of consistent negative results deserve one authoritative write-up.
- educational-angle: two-phase interactive — phase 1 lets the user balance rotations (easy, satisfying), phase 2 tries to place the balanced set and visibly stalls sooner than the unrestricted search; counter showing ~4.5e485 balanced sets exist.
- priority: P2

## border-endgame — The border sub-puzzle: frames, rings and endgame tables
- section: build/concepts
- kind: concept
- people: okifinoki/Benjamin (equivalence-class border counting), Michael Field (frame generation, 7x1 metatiles), 21valy (border endgame tables), capiman/Martin (ring-0 enumeration), Al Hopfer (222-with-border), mrjohngilbert (swappable sections)
- key msgs: 1752 (first count: ~4e38 border solutions), 6543/6548/6554/6557 (exact border enumeration to depth 16 via 25 colour-pair classes; ~2e34 frames), 6544 (why border-first is not the shortcut: ~4e37 nodes to exhaust vs ~3e40 for a full solve), 6707/6767/6770 (swappable border sections; the "free beer" interior-equivalence challenge, 384 identical interiors), 7663 (flexible frames: one frame, 1.02e9 rearrangements), 9146/9150 (21valy's precomputed final-16-border endgame: +42%), 9267/9268/10854 (exact top-row count 215,828,878,068,560, computed twice independently, vs theory 2.25e14), 10792/10793/11820 (ring-0 enumeration + invalidation programme; pieces 17 and 38 as border "keys"), 10862/10880 (Hopfer's 222 partial with completed border: 5,712 border completions counted)
- scope: Everything the community learned about the 60-piece frame as a sub-problem: exact counts and how they were computed, why solving it first doesn't help, where border structure *does* pay (endgame tables, equivalence pruning), and the modern ring-0 programme. Boundary: the balance invariant stays on border-balance; frame-vs-interior colour split stays on rare-color-geography.
- already-covered: border-balance.mdx covers the NS-1 invariant only; known-facts has a couple of counts. The enumeration methods, swappable-section analysis and endgame tabulation are uncovered.
- educational-angle: border-builder interactive — assemble a frame from colour-class blocks with a live count of how many real frames share the signature; a "swap detector" highlighting interchangeable sections on record boards.
- priority: P2

## solution-counting — How many solutions does Eternity II have?
- section: why
- kind: finding
- people: Brendan Owen & Alan O'Donnell (the launch-era formulas), mjqxxxx (rigorous framework), kubzpa (the 15M paper), jagbrain (the Markov 14,702), Peter McGavin (the modern canonical numbers), okifinoki (parity-corrected averages)
- key msgs: 38/94 (the first formulas within days of the announcement), 1221 (mjqxxxx's rigorous counting framework), 987/992 (Owen's launch-day estimate: ~5,930 with hint / ~4.6M without; Monckton's ~5M), 3385 (the closed-form expected-solutions formula), 3497/3583 (kubzpa's ~15M paper and its peer review), 6390 (partial-solution counting), 5758 (jagbrain's Markov closed form: 14,702, matching complex theory), 8924 (McGavin 2011: 1.1527e7 without starter, 14,702 with — and nodes-per-solution unchanged), 11193 (the 2024 canonical statement: 14,702 with starter, 4e-8 with all five clues — effectively unique), 8515/2922/2930 (the "20,000 solutions" claim traced to the French official site), 6892/6894 (generation bias: why the count depends on how the puzzle was made), 7364/7365 (solutions are complete restructures, not swaps)
- scope: The twenty-year arc of one question — from launch-week formulas through the 15M paper to the modern 14,702/4e-8 consensus — including why the marketing numbers ("thousands of ways", 20,000) differ, why hints collapse the count, and why more solutions ≠ easier (nodes-per-solution is the real currency). Boundary: the estimation *machinery* stays on complex-theory; this page is the question, its history, and its answer.
- already-covered: complex-theory.mdx states the headline numbers in one section; known-facts has the search-size numbers. The estimate lineage, the marketing-number forensics, and the counting subtleties (parity corrections, generation bias, hint arithmetic) are uncovered.
- educational-angle: interactive estimate timeline — every published community estimate plotted by date with error bars converging on 14,702; a hint-slider showing the expected count collapsing from 14,702 to 4e-8 as clues are placed.
- priority: P2

## clue-puzzles — The clue puzzles: hints 1–4, what they were and what they're worth
- section: build (flat)
- kind: reference
- people: Tomy (publisher), danmayoh & Michael Bastion (hint-derivation episode), louis.verhaard (clue-value counting), apal1969 (exhaustive clue-puzzle counts), Jef Bucas (the viewer's Clues page), Peter McGavin (hint-value theory)
- key msgs: 182 (the rules: four clue puzzles, two per year, hint pieces), 3858/3965 (Clue 3 & 4 surface; 36 and 72 pieces), 3543 (one clue reduces the corner space by factor ~767), 1107/5652 (hints theory: ~12–13 spread-out hints needed for tractability), 3971 (hints sold on eBay), 5650/5651 (the possibility-matrix derivation and the moderator deletion), 6768/6769 (CRC-16 checksums for clue-puzzle piece sets), 7185/7747/7752 (sold out by 2009; never released in some regions), 8052 (Clue #1 has ~3.6e10 solutions), 8168 (Clue #3: exactly 2,195,647,488 — a complex-theory validation), 8808/8809/9067/9210 (the placements circulating as community lore; the owner's copyright refusal), 10082/10085/10542 (the viewer's Clues page; 208@C3, 249@N14 with rotations; "most progress has been done *without* the clues"), 11736 (hint *placement* analysis: scattered beats concentrated), 11046 (rules: only the starter was mandatory)
- scope: The reference page for the four clue puzzles: what each contained, the CRC checksums, the availability saga (sold out, eBay, regional gaps), the exact placements as community lore, and the quantitative answer to "do hints help" (factor-767 per clue; 14,702 → 4e-8; but ~12–13 spread hints needed for tractability). Boundary: the hint-value mathematics summary links to complex-theory; the Joe-prune-method work is a records-page topic.
- already-covered: known-facts "The five clues" (placements + provenance, compact); complex-theory carries the 4e-8. The puzzles themselves, their counts, checksums and availability story have no page — an explicitly requested reference.
- educational-angle: an interactive board showing the five placements and a "hint value" meter — toggle each clue and watch expected-solutions and corner-search-space shrink by the archive's measured factors.
- priority: P2

## almost-solutions — The almost-solutions: unframed 480s, two-set boards and other false positives
- section: build (flat)
- kind: reference
- people: Peter McGavin (unframed E2, mixed-set builds), louis.verhaard (the results-page Easter egg), Vlastislav W. (distinct-tiles metric), Carlos Fernandez, Henk van der Griendt (physical verification)
- key msgs: 9736/9750/9755 (unframed E2: 479 then thousands of 480/480-internal-joins boards from the real pieces), 9754 (physically laid out and confirmed), 9757/9766/9767 (the precise disqualification: gray edges not on the rim — "I framed my own challenge and solved it"), 9781–9786 (which border-band designs are even possible), 9305/9309/9310 (two-E2-set and E2+clue-set "solutions", 2014), 11167/11169/11170 (the 2023 mixed-set 480 and the max-distinct-tiles metric, record 238), 9352/9355 (Verhaard confirms his site's "Solution!" picture was always a two-set build), 8094 (the 8-colour-projection 16x16 solved — another almost), 9574/9575 (a web "solution" debunked by mixed pieces)
- scope: A field guide to boards that look like solutions and aren't: the unframed 480 (the closest anyone has come with real pieces), duplicate-piece multi-set builds, colour-projection solves, and the exact rule each one breaks. Teaches readers to interrogate any claimed "480" before excitement. Boundary: fraud/hoax *claims* go to claim-filtering; these are honest constructions.
- already-covered: records.mdx promises "why some headline '480' boards are not actually the real puzzle" (RecordsView sidebar); history-2 narrates the unframed solve in a paragraph. The full catalogue with the design-space analysis is uncovered.
- educational-angle: a gallery where each almost-solution renders with its rule-violation highlighted (gray edges glowing in the interior; duplicate pieces pulsing); a "spot the flaw" quiz mode.
- priority: P2

## claim-filtering — "Post the grid": how the community filters solution claims
- section: build (flat)
- kind: reference
- people: Sergio Demian Lerner (phantom-sets warning), Brendan Owen & trans.spam (zero-knowledge test), Michael Field, Peter McGavin & Conny Öström & Arnaud Carré (the Vautrin debunk), Juraj Pivovarov, Mocsi (ELTE epilogue)
- key msgs: 1210 (the 2007 phantom-sets warning: a "random benchmark" could be the real puzzle re-encoded), 4048/4051 (first fake-photo debunk within a day), 5228/5247/5257 (the YouTube fake; super-resolution forensics), 6538/6565/6764 (the ELTE claim: translation, algorithm-list skepticism, deleted account), 7284 (the unsaved 471 popup), 7383/7427 (the ≥472 hand-solve claim), 8246/8636/8670/8676/8681 (the Alain Bidon 476/480s and the "zero knowledge test"), 8262/8264/8267 ("prove it on Brendan's 10x10" becomes the standard challenge), 9280/9300/9301/9303/9308 (the Vautrin affair: 115 duplicates found, bug identified, concession), 8951/8952 (the pattern named: claimants become "indignant or silent"), 10993 (ChatGPT declares E2 solved), 11755/11818/11819 (the Base44 saga: LLM-app claims, patient verification), 8470/8483/8501 (a no-solution claim dismantled the same way)
- scope: The community's second product after solvers: a claim-verification protocol (post the grid; solve a known benchmark; checksum your pieces; zero-knowledge tests) and twenty years of case studies from photo fakes to LLM hallucinations. Both a history page and a practical how-to-verify guide that pairs with the records table. Boundary: honest near-misses live on almost-solutions.
- already-covered: history pages narrate two or three episodes in passing; records.mdx quarantines unverified rows. The protocol itself — the community's real institutional knowledge — has no page.
- educational-angle: an interactive decision-tree ("someone claims a solve — what do you ask first?") built from the real cases, each branch linking to the archive episode where it was learned.
- priority: P2

## subpuzzle-ladder — The sub-puzzle ladder: from 4x49 to the unsolved 14x14 interior
- section: build (flat)
- kind: reference
- people: Brendan Owen (challenge series), Max (4x49 solve), Juraj Pivovarov (framed 14x14, 8-colour), Richard/hilbert099120 (13x13), Vlastislav W. & Carlos Fernandez (quadrants), Peter McGavin (hole puzzles, 358/364 frontier)
- key msgs: 2444/2458 (the 14x14 challenge, solved and physically built), 4878/4931 (interior-rectangle difficulty table; the 4x49 challenge as a training ground), 5074/5112/5167 (4x49 solved; 14x14-interior partials stall at 187–189), 8114/8131 (framed 14x14 solved; unframed inner 14x14 explicitly open), 8094 (8-colour 16x16 solved), 8998 (Owen's own bordered 14x14 from 196 of the 256 pieces), 10744/10747 (a complete 13x13; the outer-edge wall), 10800/10802/10803 (14x14 quadrants in minutes), 11329/11330/11473 (the hole puzzles: 16x16-minus-6x6 and 12x16+4clues solved), 10550/10552 (the standing frontier: 196-piece 14x14 interior unsolved, best 358/364 joins), 9719 (unframed-E2 difficulty equated to the 14x14 interior)
- scope: The ladder of real-piece sub-puzzles the community uses to measure progress — which rungs have fallen, when, to whom, and which rung (the 196-piece interior 14x14) still stands as the honest frontier between "220-piece regions are tractable" and "the last ring is not". Boundary: synthetic Brendan-set benchmarks stay on benchmarks.mdx.
- already-covered: benchmarks.mdx is entirely about the synthetic sets; history-2 mentions a few milestones; known-facts doesn't track sub-puzzles. The real-piece ladder is uncovered and is the community's actual progress metric.
- educational-angle: a ladder/mountain visualization — each sub-puzzle a rung with its solve date and solver, difficulty scaled by complex-theory node estimates, the unsolved 14x14 interior glowing at the top.
- priority: P2

## people-index — Who's who of the Eternity II hunt
- section: build (flat)
- kind: reference
- people: everyone — Owen, Verhaard, McGavin, Bucas, Blackwood, Wolz (doc_s_smith), Stertenbrink, Clark, Eddy, Txibilis, Max, Field, Miles, Barr, Hopfer, Knudsen (Kronjuvel), Schaus, Wauters, Mateu, Carré, Fernandez, capiman, JSA, Juraj Pivovarov, Selby & Riordan, Monckton…
- key msgs: 384/404 (Owen founded the group in 2000), 2972/1321 (Wolz: principal E1 solution finder), 6337/6349/6891 (the Verhaard/Karlsson 467), 3511 (Clark's eternity2.net), 9188/9686/11197 (McGavin's arc), 9955/10078 (Bucas's infrastructure), 10032/10117/10185 (Blackwood's arc), 8810 (Owen's E1 chicken-wire account), 9005/9017 (Wauters), 5589/9400 (Schaus), 9982 (Mateu), 11500/11560 (Owen's return; theory never formally published), 11771 (the Kronjuvel tribute — founder missing since 2023), 1342/8477 (Monckton's two defining interventions)
- scope: A carefully-scoped index of the recurring figures: their signed names/handles, their era, and their documented contributions with message links — strictly public-list facts and self-descriptions, no outside biography. Serves as the cross-reference hub the history pages and solver pages link into. Boundary: no real-world identity research; entries only for people with substantial public archive traces.
- already-covered: people appear scattered across history/records/solver pages with no index. The digests' "Active people" rollups make this nearly write-itself — but the privacy rule (public-list facts only) needs enforcing per entry.
- educational-angle: an interactive timeline of participation — each person a horizontal bar across 2000–2026 with their milestone messages as dots; filter by role (theorist, engine author, tool builder, record holder).
- priority: P2

## mcgavin-row-farming — (candidate solver page) McGavin's method: theory-ranked row farming
- section: build/solvers
- kind: concept
- people: Peter McGavin (method), capiman/Martin (row enumeration), David Barr (parallel campaign), Brendan Owen (the correlation question)
- key msgs: 9164/9167 (the 10x10 first-row list, 4,318,956 rows, distributed reservations), 9549/9561 (46,681 row-searches with per-row histograms, shared publicly), 9626/9627 (Owen's correlation question and the correlation-aware row selection), 9688 (the winning recipe: rank ~20M rows by complex-theory chance-per-node, farm to 400+ cores; solution at row-search ~92,907 of a predicted ~1-per-70,000), 9725 (independent validation), 10045 (the same farm posture landing the 469 with Blackwood's engine)
- scope: The method that solved the community's hardest open benchmark: enumerate a search dimension (first rows), rank slices by complex theory, farm exhaustively, and trust the law of large numbers. Teaches the general pattern (theory as a scheduler) beyond the one solve. Boundary: overlaps benchmarks.mdx's 10x10 narrative and complex-theory's validation section — only worth a standalone page if the solvers catalogue wants a third engine; otherwise fold into those two.
- already-covered: benchmarks.mdx "Brendan's 10x10s" and history-2 both tell the story; complex-theory provenance cites it. The reusable method framing is the only uncovered part.
- educational-angle: a scheduler simulation — thousands of row tickets ranked by predicted yield, cores drawing from the top, a win arriving statistically on schedule.
- priority: P3

## benchmarks-upgrade — (upgrade) Benchmarks page: the missing suites and generators
- section: build (flat)
- kind: reference (upgrade to build/benchmarks.mdx)
- people: Sergio "Zerjillo" Alonso, Thierry Benoist, leenstrab/Bruce, Michal Dobrogost, Max (census CSV)
- key msgs: 201/203 (Zerjillo's 10,080-board corpus — the first shared benchmark set, pre-launch), 5432/5438 (Benoist's 48-puzzle academic no-hint set — the lineage used in published papers), 8958 (Bruce's edge-ratio correction: why small boards over-reward border strategies, with the piece-count table), 11445 (Max's brendans_puzzles.csv of exhaustive counts with theory comparisons), 11752 (eii-pgen: the modern tested generator with 5-hint instances), 6464/6501/6525 (the 2009 theory-vs-empirical benchmark grid), 5919 (the results wiki), 8735 (the 10x10 leaderboard)
- scope: Round out the strong existing page with the suites it skips: the pre-launch Zerjillo corpus, the academic Benoist set, the scaling caveat (edge-ratio correction) every small-board benchmarker needs, the census CSV, and the modern generator. Small, high-value additions to an already-good page.
- already-covered: benchmarks.mdx covers the census protocol, Txibilis suite, beginner suite, hints.20.3, 9x9/10x10 thoroughly — this is genuinely an upgrade, not a gap.
- educational-angle: the edge-ratio table as a small interactive (board-size slider showing border share drift vs the 16x16's ratio).
- priority: P2

## alns-hungarian-upgrade — (upgrade) Local search page: the Hungarian-refill provenance
- section: build/concepts
- kind: concept (upgrade to build/concepts/local-search-alns.mdx)
- people: Pierre Schaus (the JFPC paper and the explanation), antminder (first practitioner report), doc_s_smith (VNS hybrid), Tony Wauters (the academic lineage continuing)
- key msgs: 5589 (antminder's 462/day hybrid: backtracker + GA with the assignment-problem repair operator), 5601 (Schaus explains the Munkres neighborhood: remove non-adjacent pieces, refill *optimally* in O(n^3); half the board removable chessboard-style; >16 pieces not useful), 5591 (the honest caveat: it targets partial score, "makes it worse" for full solutions), 7809/7825 (doc_s_smith's DFS+VNS hybrid, 522/544), 8750 (Wauters arrives asking for a 240 partial to improve — the academic local-search line meeting the list), 9017/9023 (the hyper-heuristic paper: 461/480; alternative objective functions as the lesson)
- scope: Add the community-provenance section the README lead calls for: the Hungarian/Munkres refill neighborhood was first detailed on-list in June 2008, three record-generations before ALNS terminology arrived — plus the Wauters academic lineage (2012 hyper-heuristic, 2017 MILP/Max-Clique) as the technique's published track.
- already-covered: local-search-alns.mdx teaches the destroy/rebuild loop and operator portfolio well; it lacks the 2008 origin story and the academic-paper trail (standing README lead).
- educational-angle: the existing page's demos suffice; add a small assignment-refill step-through (chessboard removal → optimal Munkres refill) if budget allows.
- priority: P2

## community-dead-ends — (upgrade or companion) The community's negative-results ledger
- section: build (flat)
- kind: reference (upgrade to build/dead-ends.mdx, or a companion "what the community tried" page)
- people: Markus Zajc (negative-results lists), mobaladje & sam_maes (GA plateaus), okifinoki (SA disappointment), Michael Field (tie-colour experiment), juraj.pivovarov & leenstrab (bipartition chance-level result), quarter pander (possibility-deletion portfolios), Joshua Blackwood (the modern catalogue)
- key msgs: 302/565/2360/2516 (GA: crossover unhelpful, ~406/480 plateau), 5997 (naive SA disappoints even on small puzzles), 6028/6032/6033 (colour-merging relaxation refuted), 7138/7139 (tying swappable pairs with artificial colours: no gain, solver separates them), 7734 (Markus's explicit list: propagation sees ~20 tiles, small metatiles useless, multicore is linear), 7936/7942 (colour-balanced bipartitions overlap solutions at exactly chance level), 8962/8971/8976 (randomized possibility-deletion portfolios, implemented and sunk), 8046/8048 (oriented chromatic sets: never found one valid), 8591 (balanced rotation sets stall faster), 10056 (Blackwood: SAT solvers, GPU, 2x2 caching, colour-saving — all measured, none kept), 8728/8732 (a 20-person SAT department never got past 10x10), 5705/5754/6928 (physics/CA/energy models: interfering equilibria)
- scope: The dead-ends page currently documents *this project's* negatives; the archive contributes a second, older ledger of community-tested failures with names, dates and measurements. Either merge as a sourced "the community tried it too" line per entry plus new entries (GA, naive SA, colour merging, tie-colours, bipartitions, deletion portfolios), or a companion reference. Prevents the wiki's readers from re-running twenty years of failed experiments.
- already-covered: dead-ends.mdx covers 8 project-side entries; several archive negatives (GA, colour merging, bipartitions, tie-colours, deletion portfolios, oriented sets) have no entry anywhere.
- educational-angle: keep the existing verdict-tag format; add "first tried / by whom / message" provenance chips per entry.
- priority: P2

## border-balance-upgrade — (upgrade) Border balance: the full 2007→2022 lineage
- section: why
- kind: finding (upgrade to why/border-balance.mdx)
- people: Brendan Owen & David Eddy & mjqxxxx (2007 observations), Al Hopfer (2009 doctrine, 2022 formalization), Carlos Fernandez (the quadrant solves)
- key msgs: 414/422/485 (Owen's paired-edge-count anomaly — the earliest statement), 2073/2098 (the 12-per-side border balance; mjqxxxx's stronger left/right-split constraint), 6842/6844/6960 (Hopfer's 2009 balance doctrine — the bridge), 10754 (the 2022 condition, the NS-1 source), 10757 (the crisp multiset restatement), 10802 (Fernandez's 4-minute quadrant solves — currently misattributed in RecordsView)
- scope: Execute the standing README leads: replace the generic groups.io link with msgs 10754/10757, add the 2007 origins and the 2009 bridge, and split the Hopfer-condition / Fernandez-solves credit. A citation-and-lineage upgrade, not new content.
- already-covered: why/border-balance.mdx exists with the law, demo and caveat; only the provenance is thin/imprecise (README: "APPLY NOW").
- educational-angle: existing demo suffices; add a three-era provenance strip (2007 observation → 2009 doctrine → 2022 formalization).
- priority: P2

---

## fpga-solving — FPGA backtracking: the hardware that never quite shipped
- section: build/concepts
- kind: concept
- people: Michael Field (the design), capiman/Martin (cost/benefit analysis), mulisak (FPGA-friendliness note)
- key msgs: 8063/8064 (2010 feasibility exchange: one hardware backtracker merely matches a CPU core), 9226/9228 (the 2014 design: uniform top-left-15x15 restriction shrinks tables to ~4KB; ~50 instances at 200 MHz, ~5B placements/s per chip projected), 9231/9232/9236/9237 (simulator progress, memory-bandwidth redesign, the symmetry bug), 9003/9004 (Field's own analysis of why the state size fights the fabric), 9260 (the e2coin footnote: E2 as CPU-friendly, GPU-unfriendly, FPGA-friendly)
- scope: The one serious hardware-solver design in the archive: how restricting the search made the datapath uniform enough for an FPGA, what the projected numbers were, and why no completed run was ever reported. A short, honest page; could alternatively merge with gpu-solving into one "custom hardware" page.
- already-covered: dead-ends' compute paragraph gestures at FPGAs; nothing covers the actual design. Narrow but well-sourced.
- educational-angle: block-diagram animation of 50 parallel backtracker instances sharing a 4KB table, versus the memory wall a general solver hits.
- priority: P3

## topcoder-takahashi — The TopCoder variant and the Takahashi 468 claim
- section: build (flat)
- kind: reference
- people: Naohiro Takahashi / chokudai (claimant), Dima (surfaced it), Peter McGavin & David Barr (the scrutiny)
- key msgs: 9694/9696 (the claim surfaces: a 2009 tweet, "467 has been broken… 468"), 9695/9697 (the rules check: TopCoder used randomly generated pieces, possibly different colour counts — comparison may be invalid), 9698 (the counterpoint: the tweet says "Eternity 2" and the profile distinguishes the contest), 9700 (the contact plan; no resolution ever appears)
- scope: A reference note pinning down what the TopCoder marathon variant was, what Takahashi claimed, and exactly why the claim is unresolved and rule-incomparable — so the records table's quarantine has a page to point at. Boundary: strictly the documented trail; no promotion without primary evidence (standing README rule).
- already-covered: history-2/known-facts mention it in passing; the README carries the never-promote rule. A short page closes the loop.
- educational-angle: a side-by-side rules table (real E2 vs TopCoder variant) making "different puzzle, different record" visual.
- priority: P3

## hand-solvers — Solving by hand: the other community
- section: build (flat)
- kind: reference
- people: Dawn (dawn_hp), Christine Raisin, nc9201, anthonycross, louis.verhaard (patron), Ib Lindeneg Berntsen (claim)
- key msgs: 5925/5929/5933 (the hand-solvers arrive; Louis's mini-competition with his 7-year-old daughter), 5958 (the pattern nicknames: "pink swords", "castles", "kipper ties"), 5965 (keep placing past the first mismatch — hand-scoring advice), 4005 (438/480 by hand), 6515 (455 by hand in three weeks), 7687/7694 (the 2010 census: 9.5 rows; a claimed 462 by hand), 5687 (452 by hand-filling a legal partial)
- scope: The physical-puzzle culture: who solved by hand, how far they got, the sorting vocabularies they invented, and how their scores compare to solver output. A human-interest page that also documents scoring pitfalls unique to hand play. Boundary: unverifiable hand claims stay labelled as such.
- already-covered: nowhere; history mentions the milieu in passing. Charming but narrow.
- educational-angle: a "sort your pieces" mini-game using the community's pattern nicknames; a chart of hand scores vs machine scores by year.
- priority: P3

## eternity1-lessons — What Eternity I taught them
- section: build (flat)
- kind: reference
- people: Brendan Owen (chicken-wire method), Dietmar Wolz/doc_s_smith (7 of 12 E1 solutions), guenter stertenbrink, Alex Selby & Oliver Riordan (winners → E2 designers), Dave Clark (ESolve)
- key msgs: 8810 (Owen's first-person E1 machinery: hexagon grids, and the surprise that three offset redundant grids beat one), 7759 (the E1 solution census: 12 known, and who found them), 2972/1321 (Wolz identified as the dominant E1 solution finder; on-the-fly strategy optimization), 1667 (why the E1 recipe died: piece-difficulty ordering needs uneven tileability, and E2's is flat), 21/24 (the veterans regroup, with retrospective odds), 716/901 (the winners hired to design E2)
- scope: The bridge story: which E1 techniques transferred (strategy optimization, distributed solving), which were deliberately designed out of E2 (piece-difficulty ordering, parity leverage), and the chicken-wire void-detection account that exists nowhere else. Boundary: E2's design response lives on design-recipe; this is the E1 side.
- already-covered: history part I covers the community lineage; the *technical* E1 lessons (8810 especially) are uncovered.
- educational-angle: a two-column "worked on E1 / killed in E2" comparison with the design counter-measure for each.
- priority: P3

## copyright-chill — The copyright chill: sharing knowledge without sharing pieces
- section: build (flat)
- kind: reference
- people: Christopher Monckton (enforcement), Brendan Owen (disqualified; moderator-enforcer), Steve Moyer & Dave Clark (checksum scheme), Alan O'Donnell ("rite of passage")
- key msgs: 1342/1352/1374 (the disqualification ultimatum), 1230 (moderation: "do not ask for the pieces"), 1063/1073 (the CRC-16 answer: verify without disclosure), 2229/2370 (derived counts as the copyright-safe verification protocol), 5650/5651 (the possibility-matrix deletion), 7555/7578/7591 (the 2010 flamewar; "we ALL did it"), 8233/8238 (the 462-photo removal), 9210/9211 (the owner's clue-copyright line, post-contest), 10034 (Blackwood shipping his solver without piece data — the norm persisting into 2020)
- scope: How one legal threat shaped fifteen years of community architecture: checksums instead of piece files, derived counts instead of data, boards-as-piece-numbers, and the norms that persisted long after the contest died. Boundary: the Monckton disqualification *event* stays in history part I; this page is the institutional consequence.
- already-covered: history.mdx covers the disqualification drama; benchmarks.mdx covers the census protocol's mechanics. The through-line (threat → verification culture → modern repo norms) is uncovered but is arguably a history-page sidebar.
- educational-angle: a "share this result without sharing the pieces" walkthrough — live CRC-16 of a typed piece list, then a derived-count comparison.
- priority: P3

## parity-upgrade — (upgrade) Parity arguments: checkerboard bipartitions and unsolvable boards
- section: build/concepts
- kind: concept (upgrade to build/concepts/parity-arguments.mdx)
- people: Robert Gerbicz (unsolvable constructions), juraj.pivovarov & mrjohngilbert & nc9201 (the checkerboard mega-thread), leenstrab & juraj (chance-level bipartition result)
- key msgs: 9952/9957/9961 (provably-unsolvable E2-like boards, brute-force confirmed; unsolvability "not a local property"), 10224/10232 (the parity mechanism spelled out: odd colour totals, top-bottom/left-right sums), 8898/8906/8907/8913 (the checkerboard-balance problem as a hard PARTITION instance; balanced sets stall backtrackers faster), 8977/8978 (Nick's pen-and-paper near-balanced set; balance ≠ tiling), 7936/7942 (bipartitions overlap the true solution at exactly chance level), 6772/6774 (full-parity rotations quickly proved impossible)
- scope: Extend the existing page beyond the 479 story with the archive's two best parity results: Gerbicz's constructive unsolvable boards (what parity *can* prove) and the checkerboard-balance negative programme (what it can't buy a search). Rounds the page from one anecdote into the topic's full scoreboard.
- already-covered: parity-arguments.mdx covers double-counting and the 479 story well; these two strands are absent.
- educational-angle: an "unsolvable board" generator following Gerbicz's odd-count trick, with the parity violation highlighted; a bipartition-overlap histogram centered exactly on chance.
- priority: P3

## colour-reduction — Colour reduction: solving easier puzzles that aren't this one
- section: build/concepts
- kind: concept
- people: juraj.pivovarov (8-colour ladder and solve), Max & JSA (merging refutation), Razvan (re-colouring theory), Walter Schreur (coarse-to-fine idea)
- key msgs: 6028/6032/6033 (colour merging refuted: merged solutions almost never lift back), 7995/8001/8002/8019 (the 8-colour background projection: 478/480 in 1.5h by beam search; hardness turns on around ~6 colours), 8094 (the 8-colour 16x16 fully solved), 9976/9977 (the coarse-to-fine relaxation ladder proposal), 11707/11727 (Razvan's reduced-colour instances and re-colour analysis of 2-colour structure), 9457/9458 (the generator lesson: E2-like colour statistics alone flip a solver from instant to never-ending)
- scope: The family of "solve a coarser colouring first" ideas: the projection ladder, where hardness switches on (~6 colours), why lifted solutions don't survive, and what the reduced instances still teach about structure. A compact relaxation-family page with mostly negative but well-quantified results.
- already-covered: nowhere; phase-transition covers colour *count* hardness for full instances but not projections/merging. Narrow.
- educational-angle: a colour-merge slider on a real board — watch the merged puzzle solve easily, then watch the lift back to 22 colours fail edge by edge.
- priority: P3

## arc-consistency-provenance — (upgrade) Arc consistency: the community lineage
- section: build/concepts
- kind: concept (upgrade to build/concepts/arc-consistency.mdx)
- people: Geoff (Harris) (the primer and hybrid solver), istarinz & capiman & doc_s_smith (the 2010 convergence), Markus Zajc (realtime AC)
- key msgs: 2902 (Geoff's 198-node solve — propagation's arrival), 4920 (the beginners paper on node/arc/path consistency), 5455 (the 40-hint 12x12 in ~7s/1.29e8 nodes), 7884/7890/7902/7906/7929 (the 2010 group project: domain dumps exchanged, 135,126→80,323 domains, Hopcroft–Karp feasibility, node estimates falling 1.6e13→2.6e12), 7118 (incremental AC "without overhead"), 7107 (the three-step reduction recipe)
- scope: Add the community-provenance thread to the existing concept page: E2's propagation lineage runs Geoff 2007-08 → the 2010 domain-reduction collaboration, with measured numbers at each step. Citation upgrade only.
- already-covered: arc-consistency.mdx teaches AC-3-and-up with project measurements; the community history is absent.
- educational-angle: existing demos suffice.
- priority: P3

## forbidden-combos-upgrade — (upgrade) Forbidden patterns: the E|N void census and piece dependencies
- section: why
- kind: finding (upgrade to why/forbidden-patterns.mdx)
- people: Al Hopfer (the 17x17 census), michaellindstrom (dependency mining), louis.verhaard (prior-art pointer)
- key msgs: 7672/7673 (the E|N census: of 289 edge combinations, 20 are voids, 47 single-piece; placing #121 creates up to 3 new voids; #174 extreme), 6743 (placing one piece at (2,2) removes thousands of candidates board-wide), 7506 (the 5x5-extension prune: ~5% more 4x4s removable), 6079/6089/6100 (orphan holes: why fast backtrackers avoid 2-sided dead cells by construction)
- scope: Enrich the existing forbidden-patterns finding with the community's earlier censuses at the *pair* level (edge-combination voids, void-creating pieces, dependency fans) — the archive's data predates and complements the page's 2x2 statistics.
- already-covered: forbidden-patterns.mdx covers 2x2 impossibility rates; the pair-level void census and dynamic void creation are absent.
- educational-angle: a 17x17 E|N heatmap with the 20 voids and single-piece cells marked; click a piece to watch new voids appear.
- priority: P3

---

# Summary

| priority | count | entries |
|---|---|---|
| P1 | 9 | edge-slipping, fill-order-theory, verhaard-eii, solver-engineering, restarts-heavy-tails, design-recipe, tooling-lineage, complex-theory-provenance (upgrade), records-corrections (upgrade) |
| P2 | 20 | comb-search, transposition-tables, metatiles-bipieces, distributed-solving, gpu-solving, nogood-learning, lp-ilp-relaxations, iterated-maps, rotation-sets, border-endgame, solution-counting, clue-puzzles, almost-solutions, claim-filtering, subpuzzle-ladder, people-index, benchmarks-upgrade, alns-hungarian-upgrade, community-dead-ends, border-balance-upgrade |
| P3 | 10 | fpga-solving, topcoder-takahashi, hand-solvers, eternity1-lessons, copyright-chill, parity-upgrade, colour-reduction, arc-consistency-provenance, forbidden-combos-upgrade, mcgavin-row-farming |

Evaluated-and-rejected (no page): e2coin (April-1 joke, 9260), quantum solving
(covered by dead-ends + one Grover aside, 10258), Prolog/BurrTools language
experiments (tooling trivia, 8774/9273), the Samsung phone (7930), the
"Language Solution"/steganography threads (history colour, already in
history's orbit), onesmallstep identity (zero archive presence — Discord-only
by design), Sudoku-backdoor and PRNG-reverse-engineering speculation
(4177/6131 — folded into design-recipe's generation-lore scope), 479-parity
story (already the centrepiece of parity-arguments), meet-in-the-middle
half-boards (existing concept page already carries the 9716/9717 refutation
angle via BANDSAW).
