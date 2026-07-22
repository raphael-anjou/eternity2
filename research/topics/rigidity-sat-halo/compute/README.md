# compute — rigidity, checked with SAT

A self-contained Rust crate that turns "can this record board be improved by
rearranging the pieces around its mismatches?" into a Boolean SAT instance.

For a given board and halo radius `R`, it frees every cell within Chebyshev
distance `R` of a mismatched internal edge, keeps the rest of the board fixed,
and emits a DIMACS CNF that is satisfiable iff the freed pieces can be placed
back (any rotation) so every edge inside the freed region matches. UNSAT means
the board is a strict local optimum on that halo.

- **Source:** [`src/main.rs`](src/main.rs)
- **Board data:** [`boards.json`](boards.json) — five public boards (Blackwood
  470/468, McGavin 469, Verhaard 467, and Riotte 464, the strict five-clue
  record) as their bucas edge strings, extracted from
  `web/src/data/known-boards.ts`. Embedded at compile time.
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
  every clause. Because the compact at-most-one encoding introduces auxiliary
  ladder variables, the check fixes the primary (candidate) literals to the
  identity assignment and completes the auxiliaries by unit propagation,
  reporting a conflict only if the fixed literals already falsify a clause. Then
  the solver confirms SAT. Only then are the UNSAT verdicts trustworthy.
- At-most-one is encoded pairwise for tiny groups and with Sinz's sequential
  (ladder) encoding above five literals, which is linear in clauses and aux
  variables. That is what makes `R = 3` and `R = 4` tractable: the freed regions
  reach 100 to 150 cells and a naive pairwise encoding would emit hundreds of
  millions of clauses. CNF emission is buffered and hand-formats integers, so
  writing the multi-hundred-megabyte `R = 4` instances is I/O-bound, not
  formatter-bound. `R = 4` solves can still take many minutes; the topic-level
  `run.sh` caps each solve (`KISSAT_TIMEOUT`, default 1800 s) and records any
  instance that hits the cap as `TIMEOUT` with `capSeconds`, so a partial sweep
  reports exactly what it proved.
- The pieces are identified by rotation-invariant signature, so no `board_pieces`
  field is needed; the edge string alone fixes each cell's piece.
