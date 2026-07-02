# Digest 0016 — Discord #general (2021-11 → 2026-05)

Coverage: 1,198 messages, 2021-11-23 → 2026-05-11, from
`v2/community-exports/eternity2-discord-general.csv` (columns AuthorID/Author/
Date/Content/Attachments/Reactions). Single channel (#general) of the Eternity
II Discord server founded by "onesmallstep".

**Citation adaptation (this digest only):** Discord has no public per-message
URLs, so citations are `Discord #general, <author>, <ISO date>` instead of
groups.io message links. Where several messages on one day form one exchange,
one citation covers the exchange. AuthorIDs are recorded in the source CSV;
they are cited here only where identity continuity matters (the "Deleted
User" problem, below).

## Notable events (chronological)

### 2021-11-23 — Server founded by onesmallstep
- who: onesmallstep
- what: First substantive message: "Recently made this, so feel free to invite
  anyone who may be interested." Pitches the puzzle via Wikipedia ("14 years
  later, and still unsolved"), uploads a Java solver + piece data file
  (e202.txt, still referenced by newcomers in 2025), and sets a house record
  protocol: beat 458/480, upload a screenshot, save the file as the new record.
- source: Discord #general, onesmallstep, 2021-11-23
- wiki-relevance: history | people

### 2021-11-28 — Joshua Blackwood present in founding week; 469→470 took "a few months"
- who: onesmallstep; "Deleted User" (AuthorID 456226577798135808)
- what: onesmallstep posts the e2.bucas.name `Joshua_Blackwood_470` board link;
  a user (account since deleted) replies "Yes, that was mine" and confirms to
  arthur089454 "Your blackwood?!!" — "Yes. Nice to meet you." Asked how long
  470 took from the previous 469: "A few months." This is a first-person
  Blackwood data point 8 days after the server opened, and within months of
  the 470 landing on the mailing list (groups.io msg 10117, digest 0014).
- source: Discord #general, Deleted User (id 456226577798135808), 2021-11-28/29
- wiki-relevance: records | people

### 2021-12-02 — Blackwood's difficulty arithmetic
- who: Deleted User (Blackwood, same AuthorID)
- what: "getting 1 fewer break is about 30-50x harder"; "A 471 will be 10^19
  times harder than a 458." His recipe: reduce search space, optimize code,
  run on powerful computers, and get lucky. (10^19 ≈ 30-50^13 — consistent
  per-break scaling from 458 to 471.)
- source: Discord #general, Deleted User, 2021-12-02
- wiki-relevance: technique:search-difficulty | records

### 2021-11→12 — onesmallstep's own scores at founding: 456–458
- who: onesmallstep
- what: Live-streams solver runs; reports 456, then 457/458 ("458 with the
  corners in different places"), phone solver "stuck on 458/480". December:
  "Just trying to get 462... after 460, nothing much stays in the same place."
- source: Discord #general, onesmallstep, 2021-11-28 → 2021-12-06
- wiki-relevance: people

### 2021-12-20 — Hand heuristics from piece scarcity
- who: onesmallstep
- what: Proposes elimination from pattern scarcity: find the border colour
  absent from corner pieces, the colour occurring once on a corner piece
  ("Only 11 pieces can connect to the corner-piece with the yellow star"),
  and work outward from pieces with three same-colour edges.
- source: Discord #general, onesmallstep, 2021-12-20
- wiki-relevance: technique:piece-rarity

### 2022-05-11 — "Do the hints help?" debate opens; bucas letter-encoding
- who: onesmallstep, arthur089454
- what: onesmallstep suspects the clue pieces make the puzzle harder to solve,
  and works through converting his numeric tileset to e2.bucas.name's a–w
  letter encoding (patterns listed a = grey … w = pink-with-blue-cross).
- source: Discord #general, onesmallstep, 2022-05-11
- wiki-relevance: technique:hints | tooling

### 2022-08-10 — puzzlingaddiction.com browser solver enters the toolkit
- who: onesmallstep
- what: Recommends `puzzlingaddiction.com/et2js/worker16.html` ("Runs in a
  browser, which is good"). This JS solver family (worker3/worker16) remains
  onesmallstep's daily driver through 2026, alongside a "Java-solver from
  2009" and a SpecBAS (modern Sinclair BASIC) solver.
- source: Discord #general, onesmallstep, 2022-08-10 (toolkit restated 2025-04-27)
- wiki-relevance: tooling

### 2023-01-15 — onesmallstep's stated best: 466
- who: onesmallstep
- what: Asked his best score: "466, or a 14x14 with a few extra on the side."
  Confirms the 470 he cites is "on e2.bucas.name" — i.e. Blackwood's, not his.
  Uses "an old Java solver", a backtracker, not his own code.
- source: Discord #general, onesmallstep, 2023-01-15
- wiki-relevance: people | records

### 2024-10/11 — The server becomes a hub: jblackwood3 and reinout_ join
- who: jblackwood3 (joined 2024-10-22, AuthorID 841699006329782307), reinout_
  (joined 2024-11-15)
- what: After two quiet years (2023 has ~30 messages), the two most important
  arrivals land within a month. Note the jblackwood3 account is a *different
  AuthorID* from the 2021 "Deleted User" Blackwood account — same consistent
  story, but account continuity is not verifiable from the CSV alone.
- source: Discord #general join events, 2024-10-22 and 2024-11-15
- wiki-relevance: history | people

### 2024-11-15 — Blackwood's first-person 470 Q&A
- who: jblackwood3, reinout_, onesmallstep
- what: The richest primary account of the 470 anywhere: "Stopped my efforts
  at 470"; "470 took 1 month on my home pc" (AMD Threadripper 3970X); no
  hints used; total effort "about 3 months during covid... Maybe 150 hours";
  "My 470 code is online for everybody to see"; never tried for 471 — would
  need to "recalibrate it to target a 471" plus lots of compute. Also:
  "Good logic makes you 1000000x better at solving partial solutions."
- source: Discord #general, jblackwood3, 2024-11-15
- wiki-relevance: records | technique:blackwood

### 2024-11-15 — onesmallstep: "working on this for about 8 years"
- who: onesmallstep
- what: In the same thread, states he has worked on E2 ~8 years (i.e. since
  ~2016) and thinks "the record is still 470". Runs "linear solving with all
  the hints in place" and is "not getting close to 471".
- source: Discord #general, onesmallstep, 2024-11-15
- wiki-relevance: people

### 2024-12-27/28 — Record restated; onesmallstep's ceiling: 467
- who: reinout_, onesmallstep
- what: reinout_ to a newcomer: "470/480 is the current record, by Joshua
  Blackwood. There are 4 clues and a middle piece." Next day onesmallstep
  gives his own tally: "463 for me so far, or 467 sometime ago" — the highest
  score onesmallstep ever claims for himself in this archive.
- source: Discord #general, reinout_ 2024-12-27; onesmallstep 2024-12-28
- wiki-relevance: records | people

### 2025-01-18 — Unique-piece facts: #62, #17, #38
- who: onesmallstep, truttle1, simonfez
- what: onesmallstep lays out board-elimination facts he uses: inner piece 62
  is "the only piece that doesn't have a colour matching any of those on the
  other five hints" (so it can't touch a hint); edge pieces 17 and 38 "have
  patterns that only occur once on the border"; a correct second ring forces
  the outer ring. Same thread: simonfez describes his SpecBas (Sinclair
  BASIC) solver with 7-letter rotation-string piece encoding (~55-60k
  placements/min); truttle1 solves rotation-free 16x16 "TetraVex" in seconds.
- source: Discord #general, onesmallstep / simonfez / truttle1, 2025-01-18/19
- wiki-relevance: technique:piece-rarity | tooling

### 2025-03-08 — reinout_'s best: 469 (2 months × 20 threads)
- who: reinout_, onesmallstep
- what: reinout_ has run his (Rust) solver 2 months on 20 threads without a
  470; best 469. onesmallstep asks for the board to compare with "the 469 on
  the website". By 2025-04-27 the same run is at 4 months, still no 470.
- source: Discord #general, reinout_, 2025-03-08 and 2025-04-27
- wiki-relevance: records(sub-record context) | people

### 2025-04-27 — Hint provenance settled: "They directly come from Tomy's Hint Puzzles"
- who: onesmallstep, neoteristis, reinout_
- what: onesmallstep airs doubt the published hints are real ("the top scores
  so far haven't used them"); newcomer neoteristis asks for a trustable
  source; reinout_ closes it: "They directly come from Tomy's Hint Puzzles.
  They are real." Same day: neoteristis shares his C++/Docker solver
  (github.com/StartUpNationLabs/eternity2, ~250k placements/s on M1);
  onesmallstep shares a broken Python project (github.com/GrafSpiel/CUBETS).
- source: Discord #general, reinout_, 2025-04-27
- wiki-relevance: technique:hints | tooling

### 2025-07-16 — Record relayed to a newcomer: "470, a few years ago" — with both boards
- who: onesmallstep, dimkadimon9892, jblackwood3
- what: dimkadimon9892 joins ("I've worked on this puzzle in 2016") and asks
  if the best is still 469. onesmallstep: "470, a few years ago", then posts
  BOTH e2.bucas.name 470 board links — `Joshua_Blackwood_470` and
  `JBlackwood+Jef_470`. Asked "Wait are you Joshua?" — "No, but he's in
  here." jblackwood3 confirms presence ("Thanks!... If anybody needs any
  help, let me know"), attributes method to "Brute-forcing and heuristics."
  onesmallstep: "471 seems almost impossible though."
- source: Discord #general, onesmallstep/dimkadimon9892/jblackwood3, 2025-07-16
- wiki-relevance: records | people

### 2025-07-16 — Beam-search + ML proposal; Jonker-Volgenant tip
- who: dimkadimon9892, reinout_
- what: dimkadimon9892 announces a group applying Beam Search with learned
  state-evaluation functions to E2, citing arXiv 2502.13266 and 2502.18663.
  reinout_ counters that "Predicting the quality of the current solution is
  probably the hardest part of E2" and notes Jonker-Volgenant is faster than
  the Hungarian method with identical solutions (for the tile-refill
  assignment step); cites T. Wauters's papers and a max-clique formulation
  that "doesn't scale".
- source: Discord #general, dimkadimon9892 / reinout_, 2025-07-16
- wiki-relevance: technique:beam-search | technique:local-search-alns

### 2025-07-16/18 — onesmallstep's actual July-2025 state: a 197-consecutive linear partial
- who: onesmallstep
- what: What onesmallstep is personally reporting in July 2025: a snake-path
  solver with all hints "still only at 197" consecutive pieces, backtracked
  halfway across the sixth row; posts `still_working.png` (first five rows +
  pieces right of 190); "The challenge with this one is to find a 198."
  Notes "There are two 470s discovered" and that because "the two 470s are
  almost the same, there isn't a way to eliminate groups of pieces"; suggests
  "You could probably get a 471 from backtracking quite far up that 470."
  Also links a second Discord server (discord.gg/sVUA2D5d2w) holding solver
  outputs "with much better solutions".
- source: Discord #general, onesmallstep, 2025-07-16 → 2025-07-18
- wiki-relevance: records(negative evidence) | technique:backtracking

### 2025-07-26/31 — Linear-score subculture: Reinout's 229s; 'tested positions' channel
- who: onesmallstep, reinout_
- what: onesmallstep credits "Reinout's solver" with finding "a lot of linear
  229s"; reinout_ on 2025-07-30: "4 229s and still no 230." On 2025-07-31
  onesmallstep adds a 'tested positions' channel "so people can show what
  they've tried that works or doesn't work" — the server's distributed-
  elimination ethos made concrete.
- source: Discord #general, onesmallstep / reinout_, 2025-07-26 → 2025-07-31
- wiki-relevance: records(linear sub-records) | history

### 2025-08 — Spam-bot wave; moderation hardening
- who: onesmallstep, neoteristis, reinout_, duathion
- what: A recurring scam-link bot (homoglyph-renamed accounts, private DMs
  before channel posts) forces repeated bans, removal of the @everyone
  permission, and verification-bot suggestions. Community-history colour for
  how a small server run by one person handles 2025-era Discord.
- source: Discord #general, onesmallstep, 2025-08-10 → 2025-08-24
- wiki-relevance: history | none

### 2025-11→12 — Bucas-format exchange; distributed-solving call
- who: onesmallstep, reinout_
- what: onesmallstep posts a 211-linear board as a bucas URL "placeholder";
  reinout_ asks for "promising unexplored partial solutions... Bucas format
  yes" (2025-12-24); onesmallstep: "If anyone wants to help participate in
  distributed solving, let me know" (2025-12-26). e2.bucas.name URLs are the
  server's de-facto board interchange format throughout.
- source: Discord #general, onesmallstep / reinout_, 2025-11-26 → 2025-12-26
- wiki-relevance: tooling | history

### 2026-01-24/27 — Exact enumerations: piece-62 adjacency table; 1216 top-left 2x2s
- who: onesmallstep
- what: Publishes small exhaustive results: piece 62 connects to neither a
  hint nor piece 38, reducing its placement "to 170 tiles in one of four
  rotations"; posts a border-tile/rotation connection table and a
  corner-adjacency list ("pieces 1 and 2 only connect to one piece (5)").
  Then: "there are only 1216 possible 2x2s in the top-left with all hints
  on", with ZIP attachments of the enumerated blocks
  (`Top-left_2x2s_with_all_hints.zip`, `2x2s_1_with_hints_on-1.zip`).
- source: Discord #general, onesmallstep, 2026-01-24 and 2026-01-27/28
- wiki-relevance: technique:exhaustive-blocks | technique:piece-rarity

### 2026-01-29 — "The record for linear is 230"; checkerboard search
- who: onesmallstep, xp2_882030kgz010602, reinout_
- what: onesmallstep states the linear (consecutive-from-start) record is
  230/256; reinout_'s stated goals (2026-01-20): "Getting a 231/256 linear
  score, getting a 471/480 partial edge matching score." Long checkerboard
  thread: placing one parity class uniquely determines the rest; onesmallstep
  reports 64 checkerboard placements with hints and neighbour-checking; also
  "I've found hundreds of thousands" of valid borders, guesses billions, and
  has "never managed to find five complete outer rings". Internal 14x14:
  "Nobody has found a complete one" (restated from 2025-07-25).
- source: Discord #general, onesmallstep / reinout_ / xp2_882030kgz010602,
  2026-01-20 → 2026-01-29
- wiki-relevance: records(linear) | technique:checkerboard | technique:border

### 2026-01-20 — Server scale, in passing
- who: onesmallstep
- what: "there were almost 100 people in here at one time" — the only
  membership figure in the archive. Message volume tells the arc: ~140
  messages in the 2021 founding burst, near-dormant 2023, reignited by the
  Blackwood/Reinout arrivals late 2024, busiest mid-2025 → early 2026.
- source: Discord #general, onesmallstep, 2026-01-20
- wiki-relevance: history

## Records evidence — the "2025-07 · 470 · onesmallstep" row

**Verdict: the CSV does NOT support the row; it actively contradicts it.**
`RecordsView.tsx` line 59 currently reads: date "2025-07", score 470, author
"onesmallstep", method "Continued 470 ties — reported on the community
Discord only; no archived board".

What the archive actually shows:

1. **No 470 claim by onesmallstep exists anywhere in the 1,198 messages
   (2021-11 → 2026-05).** His self-reported scores are: 456–458 (2021-11/12),
   466 (2023-01-15), "Only a 466 before the laptop crashed" (2024-05-22), and
   his maximum ever: "463 for me so far, or 467 sometime ago" (2024-12-28).
2. **In July 2025 specifically**, onesmallstep attributes the record to
   others. Asked for the best solution on 2025-07-16 he answers "470, a few
   years ago." and posts the two e2.bucas.name boards named
   `Joshua_Blackwood_470` and `JBlackwood+Jef_470`. Asked "Wait are you
   Joshua?" he answers "No, but he's in here."
3. The only board data onesmallstep posts in July 2025 is `still_working.png`
   — a 197-consecutive-pieces linear partial (2025-07-16), explicitly a
   work-in-progress, not a 470.
4. The likely origin of the row is a misreading of the 2025-07-16
   conversation: onesmallstep *relaying* the two existing 470s (Blackwood
   2021; Blackwood+Jef tie, cf. groups.io msg 11401 / digest 0015) to a
   newcomer, which a summary could compress into "onesmallstep reported a 470
   on Discord in 2025-07".
5. Positive by-product: the archive contains Blackwood's own 470 provenance
   (2024-11-15: 1 month on a Threadripper 3970X home PC, no hints, ~150 hours
   total, code public) and his 2021 statement that 469→470 took "a few
   months" — stronger sourcing for the *existing* 2021 470 row than we had.

**Action for RecordsView: delete or re-attribute the 2025-07 row.** Digest
0015 already noted "onesmallstep" appears nowhere in the groups.io archive;
this digest closes the loop: the Discord source doesn't exist either. If a
"reported on Discord" row is kept at all, the defensible entry is the
second 470 (`JBlackwood+Jef_470`), which is Bucas-archived and groups.io-
sourced (msg 11401), not Discord-only and not onesmallstep's.

## Rollups

### Records/boards claimed this period
- 470 — none claimed on Discord; both known 470s relayed via e2.bucas.name
  links (`Joshua_Blackwood_470`, `JBlackwood+Jef_470`) — Discord #general,
  onesmallstep, 2021-11-28 and 2025-07-16.
- 470 provenance (first-person) — jblackwood3, 2024-11-15: 1 month, home PC
  (3970X), no hints, ~150 h total, code public; 469→470 "a few months"
  (Deleted User/Blackwood, 2021-11-29).
- 469 — reinout_, 2025-03-08 (2 months × 20 threads; Rust).
- 467 — onesmallstep's personal max, "sometime ago" as of 2024-12-28. Also
  466 (2023-01-15, 2024-05-22).
- Linear (consecutive) sub-records: 229 ×4 (reinout_, 2025-07-30), record 230
  (stated by onesmallstep, 2026-01-29), goal 231 (reinout_, 2026-01-20);
  onesmallstep 211 linear (2025-03-19, 2025-11-26), 197-consecutive with all
  hints (2025-07-16).
- Enumeration results: 1216 top-left 2x2s with all hints (onesmallstep,
  2026-01-27, ZIPs attached); internal 14x14 never completed (2025-07-25).

### Techniques discussed
- Piece-uniqueness elimination (#62 no-hint-contact, #17/#38 unique border
  patterns) → onesmallstep, 2025-01-18 and 2026-01-24
- Break-difficulty scaling (30-50× per break; 471 = 10^19 × a 458) →
  Deleted User (Blackwood), 2021-12-02
- Beam search + learned evaluation (arXiv 2502.13266 / 2502.18663) →
  dimkadimon9892, 2025-07-16
- Hungarian vs Jonker-Volgenant refill → reinout_, 2025-07-16
- Checkerboard/parity placement → xp2_882030kgz010602 + onesmallstep,
  2026-01-29
- Hint provenance ("Tomy's Hint Puzzles. They are real.") → reinout_,
  2025-04-27
- Backtrack-the-470-for-a-471 idea → onesmallstep, 2025-07-18
- Two-470s-similarity blocks group elimination → onesmallstep, 2025-07-16

### Active people (Discord handle → what they state about themselves)
- **onesmallstep** — founder/moderator (2021-11-23), the server's constant
  presence (523 of 1,198 messages, greets nearly every joiner). States: ~8
  years on E2 as of 2024-11 (≈2016 start); NOT Joshua Blackwood (explicit,
  2025-07-16); personal best 467; owns a physical copy; uses others' solvers
  (2009 Java solver, puzzlingaddiction.com JS workers, SpecBAS, a C++
  backtracker he redistributes, briefly Rust) rather than his own; codes in
  Amstrad BASIC/QB64; organizes distributed solving and 'tested positions'.
  No real name, location, or mailing-list identity ever disclosed.
- **jblackwood3** — Joshua Blackwood, 470 record holder; joined 2024-10-22;
  self-description above. Earlier presence as a since-deleted account
  (AuthorID 456226577798135808) in founding week 2021 — same claims,
  different account, continuity plausible but unverifiable from CSV.
- **reinout_** — matches groups.io "Reinout Annaert" (digest 0015: Rust
  solver based on Blackwood's; here: "I wrote mine in Rust", 469 best,
  linear-229 hunter, the server's technical referee). The site's TODO.md
  Discord suggestions trace to him.
- **dimkadimon9892** — worked on E2 in 2016, returned 2025-07-16; fronts a
  beam-search+ML group with two arXiv papers.
- **arthur089454** — founding-era regular (2021-2022), hardware enthusiast;
  claimed "i think i beat your score" (2022-06-02) with no number or board.
- **simonfez** — SpecBas/Sinclair BASIC solver author, Cracking the Cryptic
  sudoku setter; joined 2025-01-18.
- **truttle1** — parallel-computing student; rotation-free 16x16 in seconds
  (MPI/C++); posted a labelled-fake solution image (2025-02-05).
- **neoteristis** — C++/Docker solver (StartUpNationLabs/eternity2), random
  14x14s solved with official colour ratios; joined 2025-04-27 via Reddit.
- **squ45765, xp2_882030kgz010602** — 2026 arrivals driving the block-
  enumeration and checkerboard threads.
- NOT present: no author matching Jef Bucas, Hopfer, Verhaard, McGavin, or
  any other mailing-list principal besides Blackwood and Reinout;
  e2.bucas.name is cited constantly but its owner never posts here.

### Community structure
- Founded 2021-11-23 by onesmallstep; ~100 members at peak (2026-01-20).
- Activity arc: founding burst (2021-11/12) → near-dormant 2023 → reignited
  by jblackwood3 (2024-10) and reinout_ (2024-11) → busiest 2025-07 → 2026-01.
- Relationship to the mailing list: one-directional awareness — reinout_
  points newcomers to "a forum... with alot of information" (2025-07-16);
  nobody posts groups.io links; the bridge people are Blackwood and Reinout
  themselves. Satellite servers exist ("K4 server", 2025-04-27; a
  solver-outputs server, discord.gg/sVUA2D5d2w, 2025-07-18).

## Leads for the wiki
- **RecordsView (records timeline) — CORRECTION REQUIRED:** remove or
  re-attribute the "2025-07 · 470 · onesmallstep" row (RecordsView.tsx:59);
  this digest is the primary negative source (see Records evidence). Cite
  this digest in the row's removal note alongside digest 0015's
  "onesmallstep appears nowhere in the list archive".
- **Records 470 row (2021, Blackwood):** enrich with the first-person Discord
  provenance — 1 month on a home Threadripper 3970X, no hints, ~150 hours
  total, 469→470 "a few months" (2024-11-15; 2021-11-29). Complements
  groups.io msgs 10117/10161.
- **People page — onesmallstep entry:** founder of the Discord (2021-11-23),
  explicitly not Blackwood, personal best 467, ~2016 start, orchestrator of
  distributed solving; identity otherwise undisclosed. Source: this digest.
- **People page — Reinout Annaert:** Discord handle reinout_; 469 best,
  linear-229/230 subculture, "Tomy's Hint Puzzles" hint-provenance statement,
  technical mentor role; connects to TODO.md's 2026 Discord suggestions.
- **Hints/known-facts page:** the hint-authenticity Q&A (doubt 2025-04-27,
  settled same day by reinout_) and the "do hints help or hurt" debate
  (2022-05-11) are citable community history.
- **Concepts (piece rarity / known facts):** piece 62's no-hint-contact
  property, #17/#38 unique border patterns, 1216 top-left 2x2s with hints,
  "internal 14x14 never completed" — all first documented here for the wiki.
- **Linear/partial-score page (if created):** the 229/230/231 linear
  sub-record thread is exclusively Discord-sourced; this digest is its only
  citation base.
- **History page (post-contest era):** the Discord as the community's
  newcomer funnel since 2021 (Reddit→Discord arrivals; mailing list as the
  archive of record) — one paragraph, cite this digest.
