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
- `0004` 2008-03 → 2008-06 (launched)
- `0005` 2008-07 → 2008-10 (launched; expect Verhaard 467 era)
- `0006` 2008-11 → 2009-02 (launched; first scrutiny date)
- …continue ~quarterly through 2011, then yearly 2012→2026 (append to the
  list as windows complete; keep numbering sequential).

## Digest-driven wiki improvement leads (grows as digests land)

- border-balance page: the finding traces to 2007 — Owen's paired-edge-count
  observation (msgs 414/422/485) and the 12-per-side border balance
  (msgs 2073/2098). Add these as earlier sources alongside Hopfer 2022.
- known-facts: parity proof that 479/480 is impossible (msg 1640).
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
