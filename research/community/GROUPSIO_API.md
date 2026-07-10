# Reading groups.io posts via the API

The eternity2 discussion list lives at <https://groups.io/g/eternity2>. Reading
it programmatically is the source for the [community mining pipeline](./README.md)
and for verifying record claims.

**The bulk archive is already exported** to `../../v2/community-exports/messages.jsonl`
(11,511 messages, 2000-10-22 → 2026-03-31). For historical sweeps, read that
file — it needs no network and no key. Use the **live API below only** to fetch
posts newer than the export, or to pull a single thread on demand.

## Why the API, not a scraper

The groups.io **web pages** sit behind a Spur/Monocle JS anti-bot probe: plain
`curl` and `WebFetch` both bounce to `/probe` and never reach `/g/eternity2/*`.
Do not try to scrape the HTML. Use the **native groups.io REST API** instead —
it is a first-party JSON API and is not probe-gated.

## Credentials

- **Key:** `GROUPS_IO_SCRAPING_KEY` in
  `/Users/raphaelanjou/Documents/dev-projects/polytech/eternity2/.env`
  (the parent of this git repo — the file is outside the repo and is never
  committed). It is a genuine groups.io API key minted at
  <https://groups.io/settings> → API, **not** a third-party scraper key
  (the name is historical).
- **Auth header:** `Authorization: Bearer <key>`.
- Never paste the key into committed files, commit messages, or logs.

## Endpoints

- **Base URL:** `https://groups.io/api/v1` (note: `groups.io/api/v1`, *not*
  `api.groups.io`).
- The eternity2 **group_id is `41375`**.

| Call | Purpose |
| --- | --- |
| `GET /getuser` | Verify the key works; returns your account. |
| `GET /gettopic?topic_id=<id>&limit=10` | A whole thread with full message bodies. Paginate with `has_more` / `next_page_token`. |

Each message in a topic carries:

- `body` — the post as HTML,
- `name` — the sender handle (e.g. `@benj39100`),
- `created` — ISO timestamp,
- `subject` — thread subject.

The canonical public citation for a message is
`https://groups.io/g/eternity2/message/<msg_num>` — always cite the message,
not the API URL.

## Quick start

```bash
# from the repo root; key lives in the PARENT .env
set -a; . ../.env; set +a

# 1. confirm auth
curl -s https://groups.io/api/v1/getuser \
  -H "Authorization: Bearer $GROUPS_IO_SCRAPING_KEY" | jq '.email'

# 2. pull a thread (first page)
curl -s "https://groups.io/api/v1/gettopic?topic_id=12345&limit=10" \
  -H "Authorization: Bearer $GROUPS_IO_SCRAPING_KEY" \
  | jq '.data[] | {name, created, subject}'
```

Paginate a long thread by following `next_page_token` while `has_more` is true:

```bash
curl -s "https://groups.io/api/v1/gettopic?topic_id=12345&limit=10&page_token=$TOKEN" \
  -H "Authorization: Bearer $GROUPS_IO_SCRAPING_KEY"
```

## Board links in posts

Posts share solved/partial boards as `e2.bucas.name` URLs whose hash encodes the
tiling. Decode and re-score them with `web/src/lib/bucas.ts`:

- `decodeBucas(hash)` → the cell/edge layout,
- `scoreCells(cells)` → matched interior edges out of **480** (matches bucas's
  own score directly).

This is how a claimed record (e.g. the five-clue record work) gets verified
against its board rather than taken on the poster's word.

## Etiquette

The list requires membership to read. When publishing anything sourced here,
follow the [pipeline rules](./README.md#rules): publish **facts with attribution
and a message link**, not long quotes (≤ 2 sentences, and only when the wording
itself matters).
