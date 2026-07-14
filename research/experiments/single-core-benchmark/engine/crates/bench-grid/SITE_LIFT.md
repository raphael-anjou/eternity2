# Lifting bench-grid solvers to the public site (eternity2/engine)

The whole grid speaks the SITE schema on purpose, so a solver benchmarked here
can move to `eternity2/engine` with minimal glue.

## The shared contract (already identical on both sides — verified)
| concept        | site `eternity2-engine`         | v2 `bench-grid` / `eternity2-core` |
|----------------|----------------------------------|------------------------------------|
| Puzzle         | `Puzzle{name,width,height,numColors,pieces:[[u8;4]] URDL,hints:[{pos,piece,rot}]}` | `SiteInstance` (serde `camelCase`, byte-compatible) |
| board cell     | `piece*4 + rot`, -1 empty, row-major | same |
| rotation       | clockwise quarter-turns, URDL    | `Edges::rotated` (identical) |
| scorer         | `score_board(&puzzle,&board)`    | `score_cells` (identical formula; reproduces 469) |
| bucas encode   | (decode only, in interop test)   | `cells_to_bucas_url` (= `eternity2_export::bucas_url`) |

Because `SiteInstance` round-trips through the site's `Puzzle` unchanged, the
`variant_NN.json` files ARE valid site puzzles. A site demo can load them and
call `WasmSolver`/`scoreBoard` directly.

## To publish a solver to the site
1. Port the algorithm to operate on the site `Puzzle` + a `Vec<i32>` board
   (most of ours already do via the `Solver` trait; the geometry is identical).
2. Add a `score_board` gate + bucas export (both already exist site-side; add
   the URL emitter — the encoder is 4 lines, see `cells_to_bucas_url`).
3. Expose via `wasm-bindgen` like the existing `WasmSolver`.

## What stays in v2 (not liftable as-is)
- Multi-core `_par` presets (site is WASM/single-thread).
- The heavy standalone binaries (producer/blackwood/verhaard) are native-only
  compute; the site would host a *step-able* educational variant, not the
  60s-batch version.
