# Exact count of frame placements with the corner constraint (agenda A5)

Pre-registration: [`PREREG.md`](PREREG.md) (first commit on this branch,
pushed before any run). No amendments.

## What is counted

The 60 labelled border pieces of the official set, placed on the 60 fixed
border cells of the 16x16 board (corners on corner cells, orientation forced by
the gray side), with all 60 ring-facing adjacencies matching. Board rotations
are not quotiented; interior-facing colours are ignored. This is the quantity
whose log10 the ring-purity Monte Carlo reports as
`log10_valid_frame_placements` (log10 of 4 f EC). N = 4 N_cyc, and
f = N_cyc / EC is the fraction that Monte Carlo estimated.

## Methods

- **M1 (Rust, `src/main.rs`, method under test).** For each assignment of the
  4 corner pieces to the 4 corner cells, a transfer matrix over the multiset of
  edge-arc types used so far (u64 packed key, sorted arrays per current colour,
  k-way merge per step). Meet in the middle: sides 0 and 1 forward, sides 3 and
  2 backward, joined on complementary multisets. N = prod m_ab! x sum W(sigma).
  Exact u128 on reduced rings; on the official ring, residues mod three 61-bit
  primes, recombined by CRT.
- **M2 (Python stdlib, `scripts/m2_best.py`, independent).** Side tables by
  BEST + matrix-tree (exact Fraction determinants) over Parikh matrices, then an
  explicit convolution over all decompositions of the edge-type multiset.
  Reduced rings only.
- **M3 (Rust, `Brute` in `src/main.rs`).** Depth-first enumeration of labelled
  placements cell by cell, no type grouping. Node cap 2e10 per instance.

## Commands

From `research/experiments/` (or `just experiments frame-count-corners`):

```sh
cargo build --release --manifest-path frame-count-corners/Cargo.toml
frame-count-corners/target/release/frame_count reduced \
    --cells 5x3,5x4,6x3,6x4,6x6,7x4,7x6,8x5,8x6 --seeds 1,2,3,4,5 \
    --cap 2e10 --threads 8 --out frame-count-corners/results/reduced_m1_m3.jsonl
python3 frame-count-corners/scripts/m2_best.py \
    frame-count-corners/results/reduced_m1_m3.jsonl frame-count-corners/results/reduced_m2.jsonl 8
for i in 0 1 2; do /usr/bin/time -v frame-count-corners/target/release/frame_count \
    official --prime-index $i > frame-count-corners/results/official_p$i.json \
    2> frame-count-corners/results/official_p$i.time; done
python3 frame-count-corners/scripts/summarize.py
```

Reduced boards: border pieces of `generate_framed(n, colors, seed, true)`,
seeds 1 to 5. Everything is deterministic; only the timing fields change
between runs. `cargo test --release` runs 4 unit tests (M1 = M3 on a hand-made
ring and on 3 generated rings, residues = exact, official ring shape);
`cargo clippy --release --all-targets` is clean.

## Hardware and wall clock

Devin VM, 8 vCPU Intel Xeon, 31 GiB RAM, no swap, no GPU. 2026-11-11.

| step | wall clock | peak RSS |
|---|---|---|
| reduced M1 + M3, 45 instances, 8 threads | 8 min 57 s | 5 MiB |
| reduced M2, 8 processes | 10 s | |
| official M1, prime 0 | 8 min 24 s | 17.49 GiB |
| official M1, prime 1 | 9 min 30 s | 17.49 GiB |
| official M1, prime 2 | 8 min 49 s | 17.49 GiB |

Total about 36 minutes.

## Results

### Reduced rings (validation)

| n | colours | frame colours | edge types | M3 finished (of 5) | M1 = M3 | M1 = M2 | N range (M1) | max M3 nodes |
|---|---|---|---|---|---|---|---|---|
| 5 | 3 | 2 | 4-4 | 5 | 5/5 | 5/5 | 829440 to 1824768 | 1.27e+07 |
| 5 | 4 | 3 | 6-9 | 5 | 5/5 | 5/5 | 6624 to 10752 | 1.18e+05 |
| 6 | 3 | 2 | 4-4 | 4 | 4/4 | 5/5 | 2043740160 to 4389396480 | 2.00e+10 |
| 6 | 4 | 3 | 8-9 | 5 | 5/5 | 5/5 | 2486784 to 5101056 | 4.85e+07 |
| 6 | 6 | 5 | 11-16 | 5 | 5/5 | 5/5 | 1648 to 14592 | 3.69e+05 |
| 7 | 4 | 3 | 8-9 | 0 | 0/0 | 5/5 | 5111562240 to 10713047040 | 2.00e+10 |
| 7 | 6 | 5 | 14-16 | 5 | 5/5 | 5/5 | 390144 to 945792 | 1.95e+07 |
| 8 | 5 | 4 | 12-15 | 0 | 0/0 | 5/5 | 25879265280 to 47604547584 | 2.00e+10 |
| 8 | 6 | 5 | 16-21 | 5 | 5/5 | 5/5 | 142765056 to 399461120 | 7.17e+09 |

M3 finished under the cap on 34 of 45 instances (threshold 30); M1 = M3 on all
34. M1 = M2 on all 45, including the per-assignment W(sigma) values. The
24-way sum equals 4 x the 6-way cyclic sum on all 45, and the residue path
matches the exact u128 values mod each prime on all 45. Per-instance data:
`results/reduced_m1_m3.jsonl`, `results/reduced_m2.jsonl`.

### Official frame

| quantity | value |
|---|---|
| frame colours, edge-arc types | 5, 24 |
| prod m_ab! | 660451885056 |
| EC (BEST, recomputed; arborescences 3432, matches committed) | 347797030249103657835033274613760000000000 |
| W6 = sum of W(sigma) over the 6 assignments with corner piece 0 on cell 0 | 15337626164089830494138310 |
| N_cyc = prod m_ab! x W6 | 10129764112357354924373158797886095360 |
| **N = 4 N_cyc (exact)** | **40519056449429419697492635191544381440** |
| log10 N | 37.60766 |
| f_exact = N_cyc / EC | 2.91255e-5 |
| committed Monte Carlo f (s.e.) | 2.775e-5 (1.317e-6) |
| z = (f_MC - f_exact) / s.e. | -1.04 |
| inside MC 95% interval [2.517e-5, 3.033e-5] | yes |
| committed MC log10 placements | 37.58665 |
| CRT: reconstructions from prime pairs {0,1}, {0,2}, {1,2} | identical |
| CRT bound 4 EC / prod m_ab! | 2.1e30, below every pair product (about 5.3e36) |
| max layer (states), max join side (states) | 527625153, 108618782 |

f_exact is the exact rational
39327246574589308959329 / 1350268319657182464000000000. The six cyclic
W(sigma) values are in `results/summary.json`; raw per-prime residues and
timings are in `results/official_p{0,1,2}.json` and `.time`.

## Outcome against the pre-registered kill criterion

Kill: "Method disagrees with brute force on the reduced ring, or the state
space does not fit in memory."

- K1: 0 mismatches between M1 and M3 over the 34 instances where M3 finished
  (exact integer equality). Not triggered.
- K2: all three official runs exited 0 with peak RSS 17.49 GiB, below the
  30 GiB threshold on a 31 GiB machine. Not triggered.
- Validation was not thin (34 >= 30).

Self-assessment: **survived**. The exact count of official frame placements
with the corner constraint is N = 40519056449429419697492635191544381440
(10^37.608), consistent with the committed Monte Carlo estimate at z = -1.04.

Scope of the evidence: the brute-force agreement covers generated reduced rings
with n from 5 to 8 and 2 to 5 frame colours (no 7x7/4 or 8x8/5 instance
finished under the cap; on those M1 is checked against M2 only). The official
value rests on M1 alone (one implementation, three primes); M2 was not run on
the official ring.
