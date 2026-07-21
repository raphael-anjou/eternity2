# eternity2-engine (Rust)

The **canonical** Eternity II engine. The website loads this crate compiled to
WebAssembly (via `wasm-bindgen`); it is also a normal native Rust library.

Every other implementation in this repository — the TypeScript, C, C++, Python,
Lua, COBOL, and Brainfuck ports — is a literal translation of *this* code, and
each is validated byte-for-byte against the reference outputs this crate emits.

## Where to start (maintainers)

The algorithm is documented once, language-agnostically, in
**[`../engine-ports/ALGORITHM.md`](../engine-ports/ALGORITHM.md)**.
Read that, then read the source alongside it — each module's top comment points
back to the relevant section:

| Module | What it covers | ALGORITHM.md |
| --- | --- | --- |
| [`src/types.rs`](src/types.rs) | the four shared conventions, rotation | §2 |
| [`src/generator.rs`](src/generator.rs) | the XorShift RNG; generating solvable puzzles | §3, §4 |
| [`src/official.rs`](src/official.rs) | the official 16×16 set (parsed from CSV) | §5 |
| [`src/paths.rs`](src/paths.rs) | the nine cell-visit orders | §6 |
| [`src/solver.rs`](src/solver.rs) | the step-able backtracking solver; the scorer | §7, §8 |
| [`src/lib.rs`](src/lib.rs) | the WASM/JS bindings (the public surface) | §9 |

## Build & test

```sh
cargo build              # native library
cargo test               # unit tests (solver, paths, generator)
cargo run --bin golden   # emit the cross-language reference data (golden.txt)
```

The website builds the WASM package separately (see `../web`). `wasm-opt` is
disabled in `Cargo.toml` because the bundled version rejects current rustc
output; run system `wasm-opt` manually if you want it.

## Golden data

`cargo run --bin golden` prints the reference outputs every port checks itself
against: generated-puzzle pieces (RNG parity), the official set summary, all
nine path permutations at several sizes, and full solver runs with exact
node/attempt/backtrack counts. That output is captured as `golden.txt` next to
each port. If you change engine behaviour, regenerate it and re-run every port's
parity test.
