# Community reference engines — measured on the M1 (single core, 60 s)

McGavin's C (`genbody.c`) and Blackwood's C# were both **built and run on this
machine** for the DFS study. Neither can be dropped onto the study's ten
corner-pinned variants: each is built around a specific clue configuration, and
each collapses when the instance changes. So they are reported here on their
native instances, as reference points, NOT on the from-scratch leaderboard. This
is exactly why the study's own break variants (which handle pins natively) are
the runnable, comparable stand-ins.

## McGavin (C, `-Ofast`, codegen per puzzle)

| instance | max depth (of 256) | throughput | note |
|:--|--:|--:|:--|
| official 5-clue (its native build) | **205** | **109 M tiles/s** | strict backtracker; comparable to our strict wall (~208) on depth, ~3× our node throughput |
| our corner-pinned variants (v00, v03, …) | **21** | 222 M tiles/s | **collapses** — see below |

**The corner-pin collapse (a real finding).** McGavin's generator emits a scan
path optimised for the puzzle it is compiled against, and that path does not
visit the corners early. Pinning three corners places pieces the scan reaches
only late, but a pinned piece constrains its neighbourhood immediately, so the
fixed path dead-ends almost at once: depth **203 → 21** just from adding three
corner hints (confirmed on multiple variants). The shallow search also inflates
the node rate (222 M tiles/s at depth 21 vs 109 M at depth 205), the same
shallow-search artifact that makes node rate incomparable across depths. This is
the concrete reason a fixed-scan-path record engine cannot be benchmarked on an
arbitrary pinned instance — and why our engines, which treat a pin as a
pre-placed cell the scan skips, are the right stand-ins.

McGavin optimises for **depth/completion, not matched-edge score**, and reports
the deepest board rather than the best-scoring one, so a matched-edge score is
not the right axis for it; its comparable contributions are depth and throughput,
both read directly from its own output.

## Blackwood (C#, .NET 8, single search thread)

| instance | best cells placed (60 s) | throughput | note |
|:--|--:|--:|:--|
| official 5-clue | ~**34** | **18.9 M nodes/s** | break-DFS; heuristic phase thrashes on the pinned 5-clue |

Blackwood's record (470) was set on the **1-clue** variant; his break heuristic
and scan order are tuned for that near-unconstrained instance. Pinning the five
official clues makes his early heuristic phase thrash — he reaches only ~34 of
256 cells in 60 s here, consistent with the project's earlier finding that
Blackwood is "fast only unconstrained." Built via `dotnet` (retargeted
netcoreapp3.1 → net8.0 for arm64; no algorithm change), 256 pieces and the 5
clues embedded in his coordinate system, thread count pinned to one search
thread.

## Honest framing for the site

- Reported as **cited reference engines on their native instances**, in a panel
  separate from the from-scratch leaderboard.
- Blackwood required a build-target change (net8.0) and single-thread pinning —
  adapted to run here, not "purely as published"; the search itself is unchanged.
- The headline the community engines contribute: McGavin's **~110 M tiles/s** is
  ~3× our fastest strict engine (the "what elite cache/codegen buys" datapoint),
  and the **corner-pin collapse** is the finding that a fixed scan path cannot
  take arbitrary pins — motivating the whole from-scratch study.

_Measured 2026-07-16 on Apple M1, one core. Sources kept in the session
scratchpad (`community/mcgavin`, `community/blackwood-cs`), not vendored into the
repo._
