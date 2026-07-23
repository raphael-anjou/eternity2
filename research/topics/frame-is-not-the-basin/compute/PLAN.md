# frame-is-not-the-basin — reproduction plan

> **Repro tier: seeded-statistical.** exact = re-run reproduces committed
> results byte-for-byte. seeded-statistical = committed results are the record
> of our run (hardware and seeds noted in the article); reproduction = a fresh
> run agrees on the qualitative facts (tops distinct, tops LOW, no frame near
> the record band) and lands the score band within a few points. qualitative =
> only the sign/ordering is lineage-bound.

## (a) The exact claim, with expected numbers

Source: the research vault, `vault/papers/vol-238/D-FRAME-FIRST.md`
(vol-238 invention D, 2026-07-14), with structural background in
`vault/concepts/bb60-frame-manifold.md` (vol-244).

The border-first hypothesis is that the **border frame is the basin selector**:
because the 60-cell border ring is the most constrained part of the puzzle, a
*different* strong (BB = 60) frame should pin a *different* interior skeleton and
so open a *different high* basin. If true, generating several distinct strong
frames and completing each would give several distinct high tops, and border-
first diversification would be a record lever.

The source tested this and found the **opposite**, with a clean mechanism:

1. **The frame is a loose object.** The border ring is a self-contained cyclic
   edge-matching problem over only 5 ring colours with balanced supply, so
   color-valid frames are astronomically abundant and structurally distinct
   ones are trivial to generate (inter-frame rows-0..12 tile-Hamming near
   maximal, about 200/208).
2. **Distinct frames give distinct LOW tops, not distinct high tops.** Eight
   structurally distinct frames each completed to only **252-258** under the
   source's greedy interior beam (beam 128, 2 seeds), with maximally different
   tops. The frame diversifies the basin but selects for LOW basins.
3. **No distinct frame completes anywhere near >= 455.** The source's decisive
   framing: *"the frame is a strong diversifier and a useless quality
   selector."* Freezing even the record-461 board's own border and completing
   its interior reached only ~204-209, the same band as random frames. Where
   the basin actually lives is the rows-0..12 interior skeleton, not the frame.

**The claim reproduced here is the load-bearing negative:** distinct strong
frames, filled by ONE fixed interior producer at a fixed budget, give tops that
are **distinct** (interior Hamming >> 0) but **LOW** (no frame reaches the
>= 455 record band). Frame identity does not predict the interior ceiling.

### What is reproduced vs out of scope

- **Reproduced (this bin):** generate N distinct BB = 60 frames from scratch;
  fill each with the same break-tolerant interior beam at a fixed width over K
  seeds; report the top band (min/median/max), the pairwise interior tile-
  Hamming between best completions, whether any frame reaches 455, and the gap
  to 455. The two load-bearing rows are "tops distinct" and "tops LOW".
- **Out of scope:** the record-461 freeze-only control and the freeze-rows curve
  (§5, §5.1 of the source). Those need the record-461 board, which lives in the
  research repo's untracked `output/` under a foreign colour labelling and is
  not part of the public starter kit. The negative reproduced here does not
  depend on it: it is established purely from generated frames, which is exactly
  the robustness form the source itself asked for.

## (b) Interior producer, and why it differs from the source's

The source used a **hard-fit greedy** interior beam that stalls when no perfect-
fit piece remains, so its reported band (252-258) is partly an artifact of a
weak solver. This bin deliberately uses a **stronger** interior producer — a
break-tolerant beam that always fills all 196 interior cells, scoring each
candidate by matched-minus-mismatched edges against placed neighbours (frame
included) and pruning to a fixed width with a seeded tie-break. Consequences:

- The absolute band this bin measures is **higher** than the source's 252-258
  (a better interior solver reaches the 420s-430s). This does not weaken the
  claim; it strengthens it. The source's caveat 7 explicitly warns that its 214
  ceiling is partly its weak greedy, and argues the *comparison* is solver-
  independent: no interior solver can make the frame informative when the frame
  is a downstream, low-entropy shell.
- The reproduced negative is therefore stated against the record band, not
  against the source's exact digits: even with a much stronger interior solver,
  **no distinct frame reaches >= 455**, and the frames remain near-maximally
  distinct. The gap-to-455 is the headline number.

## (c) Scoring-convention mapping

- Every reported score is the kit's canonical `score_cells` / `score_board`:
  matched, non-grey right/down adjacencies over the whole 16x16 grid, max 480.
  A frame-only board scores exactly BB (empty interior cells score nothing), as
  in the frame-manifold topic.
- **Strict-5/5 is preserved by construction.** The 5 official clues sit at
  interior cells {34, 45, 135, 210, 221}, none on the ring; the bin asserts this
  and pins all five before filling, so every completion obeys all five clues.
  The best board's `hints_intact_strict5` and `border_bb_still_60` are reported.
- The record band for context: our from-scratch strict-5/5 high is 461, the
  community strict-5/5 record 460, the vol-233 "wall" that D probes is that
  every >= 455 board shares its top rows. "Near the record band" here means
  >= 455.

## (d) Scale-faithfulness

The claim only makes sense on the **canonical 16x16 piece set**: the ring is a
5-colour cyclic matching problem coupling to the interior only through 60 inward
colours over the real 196-piece / 22-colour interior. A generated small-N
instance has a different border pool and a different interior pool, so the
frame->interior coupling it exhibits is not the E2 one. All measurements run on
`official_instance(true)` (clues on).

The one deliberate deviation from the source: frames are generated by this
crate's own randomised DFS rather than from the original run's frame files. The
source itself flags own-generated frames as the desired robustness check.
Expected consequence: the qualitative negative (distinct-but-low, none >= 455)
replicates; the exact band shifts with the interior solver strength.

## (e) Exact reproduction steps

```sh
cd research/topics/frame-is-not-the-basin/compute
cargo run --release -- --frames 16 --beam 128 --seeds 3 --seed0 1 > ../results/frame_first.json
# robustness pass on an independent seed base:
cargo run --release -- --frames 12 --beam 128 --seeds 2 --seed0 7001 > ../results/frame_first_seed7001.json
# quick pass (seconds):
cargo run --release -- --frames 3 --beam 32 --seeds 1
```

Acceptance: `frames_generated == frames_requested`; the pairwise interior
Hamming mean is large (near the 196-cell maximum, so tops are distinct);
`any_frame_reaches_455 == false` and `gap_best_to_455 > 0` (tops are LOW);
`best_frame_first_board.hints_intact_strict5 == true` and
`border_bb_still_60 == true` (the board is a legal strict-5/5 completion of a
perfect frame). The scale is reduced from a hypothetical exhaustive run to keep
total runtime under ~20 minutes on one core; the negative is a band result and
does not need many frames to land, but N = 16 x 3 seeds gives an honest spread.

Expected runtime: on one core of an Apple Silicon machine, one frame-seed at
beam 128 takes about 9 seconds, so the main run (16 frames x 3 seeds = 48 frame-
seeds) is roughly 7-8 minutes; the robustness pass adds a few more. A preflight
of 2 frames x 1 seed at beam 128 completed in about 18 seconds.
