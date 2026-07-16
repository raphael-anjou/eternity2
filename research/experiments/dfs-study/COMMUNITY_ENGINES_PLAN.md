# Plan: community engines in the leaderboards (two grids, canonical rescore)

Status: PLAN + first slice. Author decisions locked 2026-07-16.

## The problem being solved

The community record engines (McGavin C, Blackwood C#) are documented in the
DFS study but kept OFF the leaderboard chart, reported only in a native-instance
panel. Reason: the study's grid is corner-pinned, and a fixed-scan-path engine
collapses on an arbitrary pin (McGavin 203 -> 21 depth; Blackwood ~34/256 cells).

The author wants them IN the leaderboards, and wants the collapse shown as data,
not hidden. Resolution: two grids, community engines on both, every score
canonically rescored from the engine's own board output.

## Decisions (locked with the author)

1. **Two grids, not an exile panel.**
   - **Pinned grid (existing 10 corner-pinned variants):** ADD McGavin + Blackwood
     as rows with their real (collapsed) canonical score, badged "stalls on pins."
     The collapse is the finding — put it on the chart.
   - **Unpinned grid (NEW):** 10 variants, **center clue only** (piece 139 at I8,
     the one mandatory contest clue), diversity from each engine's seed. Every
     engine — ours + the 3 community-buildable — runs properly here. The fair
     head-to-head.

2. **Canonical rescore for every row (the key honesty step).** Each engine emits a
   board (or best partial). A post-run step recomputes matched edges from the
   board's URDL edge string with the SAME canonical scorer the blog uses (the
   dataset build / record-boards verifier: count interior adjacencies, 'a'=rim).
   Never trust the engine's self-reported number. Report canonical score AND
   throughput (nodes|tiles/s, labelled per engine, never cross-compared) AND max
   depth.

3. **Engines: the 3 that build on the M1.**
   - McGavin C (`body.c` + genbody codegen) — clang/gcc present, runs live
     (verified: loads the 256-piece 5-clue, solves).
   - Blackwood C# — prebuilt arm64 net8.0 binary present AND `~/.dotnet/dotnet`
     exists; runs. Hardcodes pieces/threads, so it takes the plain instance only.
   - Verhaard reimpl (ours) — already in single-core-benchmark (438 on 5-clue).
   Verhaard's ORIGINAL binary is Win32-only, cannot run — reimpl stands in.

## Sources (this session's scratchpad — TRANSIENT, copy before relying)

`/private/tmp/claude-501/<...>/5dd94049-.../scratchpad/community/`
- `mcgavin/` — body.c, genbody_* prebuilt arm64 binaries, *.puz, *.hnt, run logs
- `blackwood-cs/` — EternityII_Solver (arm64 Mach-O + .dll), Program.cs, README
- `verhaard_5clue/`
McGavin prints its board as a grid dump via `print_state` (piece id + 4 edge
colours per cell) — parseable but custom; needs a grid-dump -> URDL parser.

## Build order (each a checkpoint)

1. **[THIS SLICE] Unpinned instance set + canonical rescorer.**
   - `variants-unpinned/`: 10 instances, official pieces, center clue only. Reuse
     the pinned variant generator idea but pin only pos 135 (I8) = piece 139.
     Actually the official center is piece 139 at cell I8; confirm index.
   - A standalone canonical rescorer (Python, mirrors research/datasets scorer):
     board edge-string (1024 URDL letters) -> matched interior edges. Reused by
     every engine's post-run step. This is the linchpin; build + test it first.
2. Engine board-output parsers -> URDL edge string, one per engine:
   - McGavin grid-dump parser (from print_state output).
   - Blackwood best-board output (best_board.txt / best_grid.txt format).
   - Verhaard reimpl already emits canonical .json.
   Each parser has a round-trip test: parse a known committed board, rescore,
   assert it matches the known score.
3. Run harness: (engine × grid) -> results.jsonl with canonical score + nps +
   depth + budget. Single core, 60 s, seed policy documented. Copy scratchpad
   sources into the backing dir first (scratchpad is transient).
4. Site JSON: extend dfs-study.json (or a sibling) with community rows on BOTH
   grids + the new unpinned grid block. Badge "stalls on pins" on collapsed rows.
5. Leaderboard component: render community rows (flagged) on the pinned chart;
   new unpinned-grid chart/panel. i18n + light/dark, dataviz-validated palette.
6. Page: dfs-study findings — rewrite the "why they can't take our grid" section
   to "here they are on both grids"; the collapse is now a charted row, and the
   unpinned grid is the fair comparison. Keep the native-panel numbers.
7. Verify: rescore parity tests, `just` recipe reruns both grids, research
   style/links/citations/hardware, typecheck, lint, build. Ledger + memory.

## Honesty / rigor notes

- Every score canonically rescored; engine self-reports never trusted.
- Throughput labelled by unit (McGavin tiles/s, others nodes/s), NEVER
  cross-compared as one axis — same rule as the existing study.
- Community rows are "their code, their number (canonically rescored)"; clearly
  not our engines.
- The collapse rows must be labelled so a reader knows 21/256 is a harness
  mismatch (fixed scan path vs arbitrary pin), not the engine being weak.
- No em dashes / no "honest" in published prose (CI-enforced).
- Single-core enforced; never write vol-NNN in the public repo.

## Open confirmations for next session

- Exact center-clue cell + piece (memory: mandatory center piece 139 at I8;
  the pinned variants also carry hints at pos 135/210/... — confirm the center
  cell index for a 16x16 row-major board; I8 = row 8 col 8 -> 8*16+8 = 136? or
  135? verify against the official clue table in build/known-facts).
- Whether the unpinned grid is its own page section or a second chart on the
  existing dfs-study findings page (leaning: second chart, same page).
