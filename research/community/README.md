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

- `0001` 2000-01 → 2007-07 (pre-launch + launch)
- `0002` 2007-08 → 2007-10
- `0003` 2007-11 → 2008-02
- …continue ~quarterly through 2011, then yearly 2012→2026 (append to the
  list as windows complete; keep numbering sequential).

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
