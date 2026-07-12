# compute — rigidity, checked with SAT

A self-contained Rust crate that turns "can this record board be improved by
rearranging the pieces around its mismatches?" into a Boolean SAT instance.

For a given board and halo radius `R`, it frees every cell within Chebyshev
distance `R` of a mismatched internal edge, keeps the rest of the board fixed,
and emits a DIMACS CNF that is satisfiable iff the freed pieces can be placed
back (any rotation) so every edge inside the freed region matches. UNSAT means
the board is a strict local optimum on that halo.

- **Source:** [`src/main.rs`](src/main.rs)
- **Board data:** [`boards.json`](boards.json) — the public record boards
  (Blackwood 470/468, McGavin 469, Verhaard 467) as their bucas edge strings,
  extracted from `web/src/data/known-boards.ts`. Embedded at compile time.
- **Conventions:** shared with the engine (URDL edges, color 0 = border,
  rotation = clockwise quarter-turns).

## Run

Build:

```sh
cargo build --release
```

Self-test (score checks + positive SAT control; do this first):

```sh
cargo run --release -- --selftest
```

Emit one instance (CNF to stdout, stats JSON to stderr):

```sh
cargo run --release -- Joshua_Blackwood_470 2 > halo.cnf
kissat -q halo.cnf   # exit 20 = UNSAT = locally rigid
```

The topic-level [`../run.sh`](../run.sh) sweeps every board and radius and writes
`../results/halo-sat.json`.

## Notes

- The positive control is the point of the self-test: on a fully matched region
  the original placement is a valid refill, so the instance must be SAT. The
  crate proves in code (`--verify-identity`) that the known solution satisfies
  every clause, then the solver confirms SAT. Only then are the UNSAT verdicts
  trustworthy.
- Naive pairwise at-most-one clauses grow quickly; `R = 1` and `R = 2` stay
  laptop-tractable (seconds to tens of seconds). `R ≥ 3` needs a compact
  at-most-one encoding (sequential/commander) and is left out here.
- The pieces are identified by rotation-invariant signature, so no `board_pieces`
  field is needed; the edge string alone fixes each cell's piece.
