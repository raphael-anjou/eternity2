# Benchmark boards for the JIT backtracker

Two committed 16×16 boards so the throughput numbers on the
[JIT backtracker page](https://eternity2.dev/research/lab/experiments/raphael-anjou/jit-backtracker)
can be reproduced exactly, and compared like-for-like against Peter McGavin's C
engine on the same board.

| File | Hints | Solves? | Character |
| --- | --- | --- | --- |
| `bench-easy.json` | 18 | yes (3,577,121,570 nodes) | low-branching; the search lingers in cheap regions. This is Joe's `71` test puzzle (groups.io [msg 11749](https://groups.io/g/eternity2/message/11749) / [11746](https://groups.io/g/eternity2/message/11746)), converted to site-JSON. |
| `bench-hard.json` | 40 | no (non-terminating) | deep, densely constrained — the regime the real Eternity II lives in. Generated to E2's colour recipe. |

Both are ordinary site-JSON puzzles (`{name,width,height,numColors,pieces,hints}`),
pieces in URDL edge order, `0` = the grey border.

## Reproduce our numbers

From `research/experiments/dfs-study/engine`. There are three anchor points.

```bash
# 1. the HONEST baseline: naive-clean, a separate plain recursive backtracker.
#    This is the ~44 M the page's "~2.5x/~2.8x lift" is measured against.
cargo run --release -p dfs-run --bin run_dfs -- \
  --puzzle crates/dfs-codegen/bench/bench-hard.json --algo naive-clean --seed 1 --budget-s 10
#   -> ~44 M search-nodes/s

# 2. the codegen path's own floor: the data-driven JIT (no fusion, no chain).
cargo run --release -p dfs-codegen --bin run_dfs_codegen_jit -- \
  --puzzle crates/dfs-codegen/bench/bench-hard.json --budget-s 15 --opt native
#   -> ~61 M

# 3. the fused champion (this is what the page's "our engine" numbers are)
cargo run --release -p dfs-codegen --bin run_dfs_codegen_jit -- \
  --puzzle crates/dfs-codegen/bench/bench-easy.json --budget-s 40 --chain2 --opt native
#   -> score=480 in ~29 s, 3,577,121,570 nodes, ~122 M search-nodes/s
cargo run --release -p dfs-codegen --bin run_dfs_codegen_jit -- \
  --puzzle crates/dfs-codegen/bench/bench-hard.json --budget-s 15 --chain2 --opt native
#   -> plateaus (never solves), ~108-110 M search-nodes/s

# sweep the fusion width (--group N); on the hard board 2/4/6 land within noise (~107-111 M)
cargo run --release -p dfs-codegen --bin run_dfs_codegen_jit -- \
  --puzzle crates/dfs-codegen/bench/bench-hard.json --budget-s 15 --group 4 --opt native
```

The intermediate rungs on the page (u32 cells, function-chain, cached gain, byte
used-set) are not separate flags — they are the commit sequence in the engine's history,
reproducible with `git checkout` of each labelled commit.

On `bench-easy` every rung prints `score=480` at the **same node count** — that
identity is the proof the ladder only changes speed, never the search. Use a
budget long enough to solve (≥30 s) or to reach steady state; a very short budget
reports warm-up, not throughput.

## Reproduce the McGavin comparison

McGavin's `genbody.c` (`genbody71.zip`, groups.io
[msg 11749](https://groups.io/g/eternity2/message/11749)) is not vendored here — it
stays on the list where its author put it. Fetch it, then:

```bash
# 1. convert our board to his .puz / .hnt format (handles the x/y transpose)
python3 crates/dfs-codegen/bench/to_mcgavin.py \
  crates/dfs-codegen/bench/bench-hard.json /tmp/bench-hard

# 2. build his engine HEADLESS. His default has `#define INTERACTIVE`, a live
#    terminal display that costs ~2.7x throughput — comment it out first, or you
#    will measure the display, not the engine.
#    (edit genbody.c: `// #define INTERACTIVE`)
gcc -o genbody genbody.c -lm -Ofast -mcpu=native -DG
./genbody /tmp/bench-hard.puz /tmp/bench-hard.hnt        # emits body.c for this board
gcc -o solve   genbody.c -lm -Ofast -mcpu=native         # links body.c
./solve /tmp/bench-hard.puz /tmp/bench-hard.hnt
#   read the "Rate:" line (placements / elapsed) on a hard board,
#   or the final "N tiles placed, R tiles/second" summary on a solvable one.
```

Measured on an Apple M1, headless, `-mcpu=native`, back to back:

| Board | McGavin (his C) | Ours (chain2) |
| --- | --- | --- |
| `bench-easy` | ~289 M/s | ~122 M/s |
| `bench-hard` | ~102 M/s | ~108–110 M/s |

**The counters match.** Both count one committed placement per event: our engine
solves `bench-easy` in 3,577,121,570 nodes, his in 3,577,121,588 tiles placed —
the 18-tile difference is exactly the 18 hints he counts and we bake out. So
"search-nodes/s" and "tiles placed/s" are the same measurement, and the two
numbers above are comparable.

If `bench-hard` regenerates a `body.c` that does not match, or a solvable board
fails to solve in his engine, your `genbody.c` disagrees with the transpose
assumption — re-run the converter with `--no-transpose`.
