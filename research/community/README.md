# Community archive mining — pipeline

The full eternity2 groups.io archive lives locally at
`../../v2/community-exports/messages.jsonl` (11,511 messages, 2000-10-22 →
2026-03-31; fields: `created`, `msg_num`, `name`, `subject`, `snippet`
(plain text), `body` (HTML), `topic_id`). `msg_num` maps to the public URL
`https://groups.io/g/eternity2/message/<msg_num>` — the canonical citation.

**Process (user directive 2026-07-02): start from the oldest messages and
sweep chronologically toward today.** Each window produces one digest in
`digests/` (format below). Digests are the working layer; wiki pages
(history, sourced records, concept attributions) are synthesized FROM them.

## Windows

- `0001` 2000-01 → 2007-07 (pre-launch + launch) ✅ 2026-07-02
- `0002` 2007-08 → 2007-10 ✅ 2026-07-02
- `0003` 2007-11 → 2008-02 ✅ 2026-07-02
- `0004` 2008-03 → 2008-06 ✅ 2026-07-02
- `0005` 2008-07 → 2008-10 ✅ 2026-07-02 (the 467 run-up; NO public 467 announcement exists — beware the Oct-2008 '468 TEST puzzle' trap)
- `0006` 2008-11 → 2009-02 ✅ 2026-07-02 (scrutiny date: \$10k to 'Anna Karlsson' = Verhaard's entry, msg 6337/6349)
- `0007` 2009-03 → 2009-08 ✅ 2026-07-02 (Verhaard's first-person 467 account msg 6891: 247 flawless pieces; ELTE claim collapses msg 6764; eii benchmarked to 467 in ~82 days msg 6687)
- `0008` 2009-09 → 2010-02 ✅ 2026-07-02 (2nd scrutiny: no winner, NO new $10k — Tomy Q&A msg 7488; 467 stands, msg 7494)
- `0009` 2010-03 → 2010-08 ✅ 2026-07-02 (doc's return, toolbox era, complex-theory defense)
- `0010` 2010-09 → 2011-02 ✅ 2026-07-02 (contest ends: Monckton msg 8477; 467 stands, Owen farewell 8429; peak-depth 256·(1−1/e) proof msg 8125)
- `0011` 2011-03 → 2011-12 ✅ 2026-07-02 (post-contest year: solution-in-a-safe msg 8823; 9x9 exhaustive validations)
- `0012` 2012-01 → 2014-12 ✅ 2026-07-02 (quiet years: Wauters academia, Czech re-prize, McGavin's complex_theory.pdf msg 9188, FPGA/Carré arrivals)
- `0013` 2015-01 → 2018-12 ✅ 2026-07-02 (McGavin solves Brendan's 10x10 — complex theory's strongest validation, msgs 9686/9688; TopCoder 468 claim surfaces unresolved, msg 9694; unframed-480, msg 9750)
- `0014` 2019-01 → 2021-12 ✅ 2026-07-02 (record wave confirmed: 468 msg 10033, 469 msg 10045, wave msg 10067, 470 msg 10117; Yahoo→groups.io migration story)
- `0015` 2022-01 → 2026-03 ✅ 2026-07-02 (SWEEP COMPLETE. Hopfer NS-1 = msgs 10754/10757;
  Gauthier 460 = 11074; mixed-set 480 = 11169; complex_theory.c = 11197; Bucas 470 tie = 11401.
  NOTE: msg 11905 (Jef's wrapper_blackwood permission) post-dates the export (ends 11823,
  2026-03-31) — it was provided firsthand by Raphaël from his inbox; citation valid.)
- …continue ~quarterly through 2011, then yearly 2012→2026 (append to the
  list as windows complete; keep numbering sequential).

## Digest-driven wiki improvement leads (grows as digests land)

- APPLY NOW — border-balance: Hopfer's exact messages are 10754 (2022-05-24,
  the condition) and 10757 (the crisp restatement); replace the generic
  groups.io link. The 4-minute quadrant solves were Carlos Fernandez (10802).
- border-balance page: the finding traces to 2007 — Owen's paired-edge-count
  observation (msgs 414/422/485) and the 12-per-side border balance
  (msgs 2073/2098). Add these as earlier sources alongside Hopfer 2022.
- ~~known-facts: parity proof that 479 is impossible (msg 1640)~~ APPLIED then
  CORRECTED: Verhaard's msg 6317 defeats the parity argument through the
  unscored outward border edges; verified against the piece set (14
  qualifying border pieces). known-facts now tells the full story.
- records: the 467 award's primary sources are msgs 6337 (result surfaced,
  'Anna Karlsson from Lund') + 6349/6356 (Verhaard confirms it was his
  household's entry) — richer than shortestpath.se alone.
- complex-theory page: Owen's original 'nailed it' posts are msgs 5197/5209/
  5210 (May 2008); 5209 is already cited. McGavin's 14,702-with-hint estimate
  appears as early as 2011 (msg 8924) — an earlier source than his 2024 post.
- history part II material: Owen on the designers' solution "locked in a
  safe" (msg 8823); the official unclaimed-prize confirmation (msg 8846);
  the contest's actual ending sequence (site suspended 8270, Tomy France
  8339, official page 8373, Monckton "extension not allowed" 8477).
- complex-theory page: Owen's peak-depth proof 256·(1−1/e)=161.8 (msg 8125)
  — the closed form behind the funnel chart's empirical peak (~157-161).
- complex-theory lineage complete: Owen 2008 (5197/5209) → McGavin's LaTeX
  complex_theory.pdf in group Files (msg 9188, 2013) → McGavin's C reference
  (msg 11197, 2024) → this site's TS port. Worth a provenance paragraph.
  PLUS its strongest validation: McGavin's 2017 10x10 set_1 solve via
  complex-theory-ranked first rows, ~180 core-years, inside predictions
  (msgs 9686/9688, verified 9725).
- records: the TopCoder/Takahashi 2009 "468" claim (surfaced msg 9694,
  challenged 9697, defended 9698) is UNRESOLVED — never promote without
  primary evidence; it's also a different-rules variant (randomized pieces).
- 468/Toulis question further narrowed: nothing above 467 verified through
  contest close (Owen farewell 8429).
- concepts/local-search-alns: Pierre Schaus's JFPC paper + Hungarian/Munkres
  refill neighborhood first detailed at msgs 5589/5601 (June 2008).
- records 467 row: Verhaard's definitive first-person account is msg 6891
  (2009-08: "Anna is my wife… it was my program that did the job"; 247
  flawless pieces). History page carries it; row keeps the open-access
  shortestpath.se link.
- OPEN QUESTION (narrowed by 0008): nobody beat 467 through 2010-02
  (Verhaard, msg 7494; Tomy Q&A, msg 7488 — partials weren't even scored in
  2009). doc's June-2010 "beat 468" (msg 7803) therefore most likely refers
  to the Toulis simulated-annealing PAPER's claimed 468 — read the paper
  before any records-table change. Also: two unverified claims (471 E2Lab
  popup msg 7284; ≥472 hand-solve msg 7383) must never enter the table.
- records provenance: shortestpath.se/eii is Verhaard's own site (solver
  relocated there, msg 7439) — our 467 source link is first-party.
- RECORDS CORRECTIONS PENDING (from 0014's programmatic cross-check):
  (a) the canonical(469)/variant(470) split is unsupported — both boards
  have start piece 139@I8 and none of the other clues (470 confirmed at
  msg 10554): same regime, classify them consistently;
  (b) the 468 was relayed from Reddit by "Puzzled_For_Eternity" (msg 10032),
  not announced by Bucas;
  (c) "onesmallstep" never appears in the list archive — Discord-only,
  no-source status confirmed correct.
  Apply to RecordsView after digest 0015 corroborates.
- blackwood page leads: primary sources for the four levers + break indexes
  are msgs 10076/10056/10051; libblackwood re-publication "exact code used
  to find a 470" is msg 10161.
- border-balance history: Al Hopfer's balance doctrine appears 2009-08
  (msgs 6842/6844/6960) — the bridge between the 2007 observations and his
  2022 formalization.
- phase-transition: Owen's launch-era derivation of ~17.14 interior colours
  from the "one expected solution" criterion (msg 1947) — a community
  precursor to the published papers; cite it.
- history page candidates: Monckton disqualification drama (1342-1412),
  eternity2.net rise+shutdown (756, 3511), the Syndicate (3021), Clark's
  Monckton phone call on puzzle generation (4177), benchmark culture
  (Txibilis suite, 2896/2928).

## Digest format (one file per window: `digests/0001-2000-01--2007-07.md`)

    # Digest NNNN — <period>
    Coverage: <count> messages, msg_num <a>–<b>.

    ## Notable events (chronological)
    ### YYYY-MM-DD — <short title>
    - who: <author name as posted>
    - what: 2–4 factual sentences.
    - source: msg_num <n> — https://groups.io/g/eternity2/message/<n> — "<subject>"
    - wiki-relevance: records | technique:<topic-slug> | history | tooling | people | none

    ## Rollups
    - Records/boards claimed this period (score, who, when, msg link)
    - Techniques discussed (name → first/best message link)
    - Active people (name → rough role)

## Rules

- The list requires membership to read: publish **facts with attribution and
  a message link**, not long quotes. Quotes ≤ 2 sentences, only when the
  wording itself matters. (Exception: material with explicit permission,
  e.g. Jef Bucas's notes.)
- Every claim that reaches a wiki page must carry its msg_num link.
- Record claims: cross-check against `research/topics/record-boards` and the
  site's records timeline before promoting to a page.
- Digests are append-only once committed; corrections go in later digests.
