# hardness-peak compute crate

One binary, `hardness_peak`, that measures how hard a single generated board is
for the engine's plain backtracking DFS. The shell (`../run.sh`) sweeps it over
interior-color counts and seeds to trace the hardness curve.

## What one invocation does

```
hardness_peak <interior_colors> <seed> <node_budget>
```

1. Builds a solvable framed board with `interior_colors` interior colors plus a
   fixed **5-color border band** (real Eternity II's border-color count), so the
   total palette is `interior_colors + 5`. Uses the engine's
   `generate_solved_framed_bc` (explicit border count, so the interior palette is
   exactly what we ask across the whole sweep), then applies the same scramble
   `generate_framed` uses so the solver gets a genuinely unsolved board.
2. Runs the engine's step-able row-major backtracking DFS until it reaches a
   first full solution or spends `node_budget` DFS steps (placements +
   backtracks).
3. Prints one JSON object: `solved`, `nodes` (steps to first solution, else the
   budget/exhaustion point), `placements`, `backtracks`, `bestPlaced` (deepest
   cell count reached), and `seconds` (informational only).

Effort is counted in **nodes**, not seconds, so the verdict is deterministic and
hardware-independent: the same `(interior_colors, seed, node_budget)` yields the
same row on any machine.

## Board size and why the peak needs a small board

`E2_SIZE` (default 16) sets the board edge length. This is the one calibration
knob that matters:

- **16x16 (real Eternity II).** A plain, unguided DFS never solves a full 16x16
  at any color count within a feasible budget (it stalls around 197-247 of 256
  cells). So on 16x16 `nodes`-to-solution is pegged at the budget for every `c`,
  and the only measurable signal is `bestPlaced` under a fixed budget, which
  falls **monotonically** as `c` rises (more colors = more constrained = the DFS
  dies shallower). That shows the board sits *past* the point where effort has
  already gone vertical, but it does not trace a *peak*. Note also that a 16x16
  board holds at most 17 interior colors (the 22-color ceiling minus the 5-color
  border), which is exactly Eternity II's value, so the sweep cannot even go past
  17 on a full board: requests above 17 are clamped and flagged
  `clampedToCeiling`.
- **A smaller board (e.g. `E2_SIZE=8`).** Here instances actually get solved, so
  the solve-rate and median-nodes curve is **traversable** and shows a real
  peak: a hard band where the DFS fails within budget, flanked by easier regions
  on both sides. This is the regime that produces a peaked curve to put beside
  the page's external citations.

## Owen's one-expected-solution criterion

```
hardness_peak --criterion <size>
```

Prints the interior-color count at which a framed `size`x`size` board has about
one expected solution, by Owen's first-moment estimate `I = (P! * 4^P)^(1/2P)`
with `P = (size-2)^2` interior pieces. At size 16 this is
`(196! * 4^196)^(1/392) = 17.14`, exactly Owen's published figure (groups.io
msg 1947, 2007); at size 8 it is `(36! * 4^36)^(1/72) = 7.56`. The size-8 value
uses the identical formula, so comparing it to the measured 8x8 hard band is a
fair scale-transfer test of the criterion.

## Reproduce

Deterministic per `(interior_colors, seed, node_budget, E2_SIZE)`:

```sh
cargo build --release
./target/release/hardness_peak --criterion 8     # predicted peak color count
E2_SIZE=8 ./target/release/hardness_peak 8 1 30000000   # one solve
```

The sweeps produce two result files, one per regime:
`results/hardness-peak-8x8.json` and `results/hardness-peak-16x16.json`. The
sweep, parallelism, and resume live in `../run.sh`; `../run-both.sh` runs both
regimes in order.
