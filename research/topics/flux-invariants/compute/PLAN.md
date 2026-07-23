# Reproduction plan: flux-invariants

> **Repro tier: exact.**
> exact = re-run reproduces committed results byte-for-byte. seeded-statistical =
> committed results are the record of our run (hardware noted in the article);
> reproduction = fresh run agreeing within the stated tolerance. qualitative =
> the effect (sign/ordering/band) reappears; exact digits are lineage-bound.

Source of the claim: research repo, `research/vault/papers/vol-226/ALG.md`
(vol-226 round 1, track ALG, "Flux and Character Invariants for Eternity-II
Edge Matching"), Sections 1 (instance facts), 6.2-6.3 (flux-law rank), and 6.6
(the incremental mod-2 endgame certificate). This plan cites the exact numbers
the checker must reproduce.

## Edge / rotation convention

The paper reads a tile clockwise from the top as `(N, E, S, W)` and defines the
CW quarter turn `rho: (N,E,S,W) -> (W,N,E,S)`. The starter kit stores a piece's
edges as URDL = `(up, right, down, left)`, which is exactly `(N, E, S, W)`, and
`e2_kit::rotated(e, 1)` sends `[t,ri,b,l] -> [l,t,ri,b]`, i.e. `(N,E,S,W) ->
(W,N,E,S)`. So the kit's rotation index `k` is the paper's number of CW turns
with no reindexing, and the flux vector `d^(0)_{t,c} = (E-W) + i(S-N)` rotates
by `i^k` under `rot(k)` directly. The checker documents this in
`edge_convention` and relies on it.

## (a) The exact claims and expected numbers

### Instance facts (paper Section 1, "Verified instance facts")

| # | Fact | Expected value |
|---|------|----------------|
| 1 | Piece-type histogram by gray-slot count | 196 interior / 56 edge / 4 corner |
| 2 | Gray (colour 0) half-edge count | 64 (the grid perimeter) |
| 3 | Frame colours := non-gray colours absent from all interior pieces | exactly 5 (colours 1..5), 24 half-edges each, supply 120, border-only |
| 4 | Non-frame interior colour multiplicities | 5 colours at 48, 12 colours at 50 |
| 5 | Every colour count even | necessary for a perfect half-edge matching |
| 6 | The five clues are all interior pieces | true (piece ids 138,180,207,248,254 in the kit's 0-based labels = the paper's 139,181,208,249,255) |

### Flux-law rank (paper Sections 6.2-6.4)

Form the 22x256 base matrix `M[c][t] = d^(0)_{t,c} = r + i s` with
`r = [E=c]-[W=c]`, `s = [S=c]-[N=c]`. The whole-board law is
`sum_t i^{k_t} d^(0)_{t,c} = 0` for every colour c.

| # | Quantity | Expected value |
|---|----------|----------------|
| 7 | Complex row rank of M | 22 of 22 (full) |
| 8 | Independent real constraints (Re, Im stacked, 44x256) | 40 |
| 9 | Independent F2 parity constraints | 21 (augmented rank also 21: consistent) |
| 10 | Census orthogonality: the i->1 (rotation-blind) parity shadow | identically zero for all 22 colours (so flux constraints are invisible to colour counting) |
| 11 | Nontriviality: colours with nonzero base (k=0) flux, j=1 | 20 of 22 (paper 6.1) |

### Endgame certificate (paper Section 6.6b, the sound mod-2 test)

The remaining tiles' rotation parities must satisfy an F2 system: two rows per
colour, coefficient `(r+s) mod 2` on each parity bit, constants `r` (real row)
and `s` (imag row), with the placed prefix moved to the RHS. Solvability by
Gaussian elimination decides feasibility; an infeasible partial is a certified
dead end. The paper measures, on planted 8x8 boards, 30 trials per fill, one
illegal rotation injected into the placed prefix:

| fill fraction | valid partials passing (soundness) | single-rotation error caught |
|---|---|---|
| 0.50 | 30/30 | 4/30 (13%) |
| 0.75 | 30/30 | 15/30 (50%) |
| 0.90 | 30/30 | 30/30 (100%) |

The reproducible *shape* is: soundness (no valid partial ever rejected) at
every fill, and a catch rate that **rises monotonically toward ~100% as the
board fills**. The absolute catch percentages are planted-setup-bound (they
depend on board size, colour count, and the error-injection rule), so the
checker reproduces the mechanism and direction exactly and reports its own
measured catch curve; digit-level agreement with 13/50/100 is not expected and
is flagged as a caveat.

Facts 1-11 are exact instance/linear-algebra facts of the canonical 256-piece
set: any deviation refutes the claim outright. The certificate soundness
(clean partials always pass) is exact; the catch rate is a measured curve whose
monotone rise toward full fill is the reproduced effect.

## (b) Scoring-convention mapping

This topic makes no score claim, so no strict-5/5 vs matched-edges mapping is
needed. The certificate is an obstruction (a necessary condition): it certifies
a partial *dead*, never *completable* (paper Section 7 item 5). It prunes the
rotation/orientation sub-problem, orthogonal to and additive on top of
edge-matching and colour-count pruning (paper Section 6.4).

Colour labels: the checker derives the frame-colour set from the data (colours
with zero occurrences on interior pieces) rather than assuming labels 1..5, so
it is robust to any renumbering between the kit's `official.json` and the
paper's piece list.

## (c) Scale-faithfulness

The instance facts (1-6) and the flux-law ranks (7-11) are exact properties of
the official 16x16 / 256-piece design, so the checker computes them on
`official_instance(true)` — the canonical instance, no generated board.

The certificate (paper 6.6) is a general law that holds for any valid tiling,
which the paper tested on planted 8x8 boards. The checker mirrors that: it
builds a pool of framed solved 8x8 boards through the kit's
`generate_solved_framed(size, colours, seed, framed=true)` (piece i @ cell i,
rot 0 is a genuine valid tiling), scrambles each tile by a random rotation and
records the restoring rotation as the planted solution `k_t` (this hands the
certificate a real nontrivial planted rotation vector, exactly the paper's
"planted board with random base orientations and recorded rotations"), then
runs the mod-2 feasibility test over the fill sweep. Board size 8, colours 13
(same 5-frame / 8-interior split *shape* as the official set, a richer palette
than a bare 7 so the per-colour parity equations have realistic density). The
certificate is structural, not colour-count-critical, so the mechanism and its
monotone catch curve reproduce; the exact catch percentages are setup-bound.

## (d) Exact reproduction steps

1. `cd research/topics/flux-invariants/compute`
2. `cargo run --release > ../results/flux_invariants.json`
3. Compare the emitted JSON against the expected tables above. Every field in
   `instance_facts` and `flux_law_rank` must match exactly (the binary exits
   nonzero and prints the first failing check on any deterministic mismatch);
   `endgame_certificate.soundness_no_valid_partial_rejected` must be true and
   the `by_fill` catch rates must rise monotonically toward full fill.

RNG: the certificate uses the kit generator's `XorShift` with fixed base seeds
derived from `seed_base` (default 1, overridable with `--seed`), so reruns are
bit-reproducible (byte-stable JSON). The board pool is generated once and reused
across all fills and both probes to keep the framed-generator cost (the one slow
step) amortized; total runtime is well under a second.

## Checker structure

`src/main.rs`, single file, dependencies `e2-kit` (path dep on the starter kit)
and `serde_json`:

- load `official_instance(true)`; classify pieces by `border_edge_count()`;
- half-edge colour histogram split by piece class; derive frame colours;
  clue placements from `instance.hints`;
- build the 22x256 base flux matrix; complex row rank (complex Gaussian
  elimination), real rank (44x256 float GE), F2 parity rank + augmented rank
  (bitset GE), census shadow, base-orientation nontriviality count;
- certificate: framed solved 8x8 board pool, planted rotations, mod-2 residual
  feasibility over the fill sweep (soundness + catch rate);
- print one JSON object to stdout; nonzero exit on any deterministic failure.
