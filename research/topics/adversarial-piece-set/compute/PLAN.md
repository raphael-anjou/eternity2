# Reproduction plan: adversarial-piece-set

> **Repro tier: exact.**
> exact = re-run reproduces committed results byte-for-byte. seeded-statistical =
> committed results are the record of our run (hardware noted in the article);
> reproduction = fresh run agreeing within the stated tolerance. qualitative =
> the effect (sign/ordering/band) reappears; exact digits are lineage-bound.

Class: **measurement** (deterministic invariants of the official piece set; no
search, no seeds, no variance).

## (a) The exact claim, with expected numbers

Source documents (research vault, vol-65, 2026-05-15):

- `vault/concepts/e2-maximally-adversarial-thesis.md`
- `vault/concepts/piece-orbit-structure.md`

The thesis has seven axes. Only the piece-set-level axes are reproducible from
the official instance alone; those are the scope of this topic:

| # | Claim | Expected number (vault) |
|---|---|---|
| A1 | Every piece has full rotation orbit (size 4); no rotation-symmetric piece | 256 of 256 pieces, 1024 distinct piece-rotations |
| A1b | Distinct unordered edge-colour multisets | 251 of 256; exactly 5 twin pairs |
| A1c | Near-twin pairs (3 of 4 same-position edges shared, stored orientation) | 114 pairs, 79 groups |
| A2 | Fixed-orientation matching cap: sum over colours c of min(E_c, W_c) + min(N_c, S_c) | 307 (vs geometric max 480) |
| A3 | Every non-border colour has an even total side count; sum of floor(N_c / 2) | all even; sum = 480 exactly |

The vault lists the 5 twin pairs by research piece id: (2,3), (5,14), (7,51),
(109,110), (171,181). The reproduction checks the *count* (5 pairs) as the
primary claim and prints the pairs it finds; exact ids are secondary (see
caveats).

Out of scope here (they depend on data beyond the piece set):

- Axis 4 (bimodal compatibility spectrum, algebraic connectivity 0.507,
  spectral gap 2.95): needs an eigensolver; deferred to a follow-up step
  (numpy script) once results/ conventions are pinned.
- Axes 5-7 (sigma-cycle indecomposability, isolated 469 basin, joint-MIP
  local optimality of 458+ records): need specific record boards and a MIP
  solver; not reproducible from the official instance and not claimed by this
  topic's article.

## (b) Scoring-convention mapping

The kit's canonical scorer (`score_cells` / `score_board`) counts **matched
interior edges, rim excluded**, maximum 480 on 16x16 (`MAX_SCORE_16`). The
claim's two score-like numbers are in exactly this convention:

- The A2 cap of 307 is a bound on *matched interior edges* when all pieces
  are frozen in stored orientation. Same unit as `score_cells`; directly
  comparable to the 480 maximum.
- The A3 sum of 480 equals the geometric interior-edge count 2 * 16 * 15,
  i.e. the kit's `MAX_SCORE_16`.

No strict-5/5 versus matched-edges distinction arises: nothing here scores a
candidate board against hints. Border colour is 0 in the kit and is excluded
from all per-colour counts, matching the vault scripts (which map the CSV
border sentinel 65535 to 0 and skip colour 0).

## (c) Scale-faithfulness

**These claims only make sense on the canonical 16x16 piece set.** They are
statements about Christopher Monckton's specific published set (as allocated
by Selby-Riordan), not about the puzzle class. A generated small-N instance
from the kit's generator would measure the *generator's* symmetry properties,
not Eternity II's, and could falsely refute (e.g. the kit generator only
best-effort guarantees distinctness up to rotation on small shapes, per the
`instance_from_generated` docs; a random small set can easily contain
rotation-symmetric pieces or many multiset collisions). The vault's own
comparison point is explicit: the canonical set is *harder than a random
16x16 piece-matching* because these invariants are at their adversarial
extremes. So: run on `official_instance(false)` only. Small-N runs are
permitted as a *contrast* exhibit (showing random sets do not have these
properties) but never as a test of the claim.

## (d) Exact reproduction steps

1. `cd research/topics/adversarial-piece-set/compute`
2. `cargo run --release` (about one second; no arguments)
3. The binary loads `e2_kit::official_instance(false)` (256 pieces, no
   hints), computes A1, A1b, A1c, A2, A3, prints one JSON document to stdout,
   and exits non-zero if any expected number fails.
4. Once conventions below are confirmed, redirect stdout to
   `../results/invariants.json` and commit (not yet done; results/ is
   intentionally absent from this draft).

### Verification status (2026-07-22)

The binary was built and run once (well under one second). **All nine checks
pass**, including the orientation-dependent ones: cap = 307, near-twins =
114 pairs in 79 groups, and the five twin pairs come out as (2,3), (5,14),
(7,51), (109,110), (171,181), identical to the vault's research-numbering
list. So the kit's `official.json` stores the same canonical rotations and
the same id order as the research CSV, and the caveats below are resolved in
practice; they remain documented for anyone running against a different
piece-set export. Remaining work before publish: commit
`results/invariants.json`, and optionally add the axis-4 spectral step.

### Known caveats (resolved for the bundled official.json; see above)

- **Orientation dependence (A1c, A2).** Near-twin counting and the 307 cap
  depend on each piece's stored orientation. The vault computed them from
  `size_16_official_eternity.csv` row orientations; the kit's bundled
  `official.json` must store the same canonical rotations for the numbers to
  match exactly. A1, A1b, A3 are rotation-invariant and immune. If A2 differs
  from 307, first diff the two datasets' per-piece canonical rotations before
  concluding anything.
- **Id numbering (A1b twin list).** Vault piece ids come from the research
  CSV order; kit ids come from `official.json` order. The pair *count* is
  numbering-independent; the printed pairs may need an id crosswalk.
- **Colour count.** The vault says "22 non-border colours". The binary prints
  the observed distinct non-border colour count rather than hard-coding it.
