# Amendments to PREREG.md (Track 9, resumable runs)

`PREREG.md` stays frozen at `7fda502`. Every later change of behaviour,
format or gate is recorded here: what, why, when, effect on the gates.

## A1. Fix round after the judge (2026-11-12, written 21:35 CET, before any run of the fixed code)

**Why.** `JUDGE.md` (verdict keep) found three defects. D1: a checkpoint
whose content was altered (last level `index`, `totals`, `hash`, `hashed`)
resumed silently, in the `index` case as a different search. D2: replay
errors and a cap at or below the checkpoint's node count exited with code 0,
and the too-low-cap row reported 0 nodes. D3: an instance refuted at the root
reported closed share 0.0. The first runner also documented that a progress
file whose directory does not exist was skipped silently.

**What changed (code, `compute/src/`).**

1. Checkpoint format **version 2**. Two new fields: `cap` (total cap of the
   segment that wrote it) and `digest`, FNV-1a 64 (16 hex digits) over the
   compact JSON of the checkpoint with `digest` set to "". Every struct of
   the format now rejects unknown fields. `serde_json` is built with
   `float_roundtrip` so `closed_share` and `elapsed_s` parse back to the
   same bits (needed both for the digest and for the closed-share check).
2. Load order: JSON parse; `format`; `version` (a version 1 file is refused
   with its own message: written before this fix, no digest, an altered index
   or total cannot be detected, rerun from the start); digest, before any
   replay; then checks that need no puzzle: solver name, segments >= 1,
   nodes <= cap, propagation = nodes + 1 (one propagate call per node plus
   the root's; held on all 368 committed G-a/G-b rows), backtracks <= nodes,
   finite elapsed time, hash and hashed both set or both null, hashed =
   nodes, hex fields well formed, stack non-empty and no deeper than the
   board, deepest >= stack depth, closed share in [0, 1), every level's cell
   and index in range. After replay (which already checks cell, option list
   and `state_fnv`): the closed share recomputed from the replayed stack must
   equal the stored one bit for bit. Any failure: error row, non-zero exit.
3. Exit codes: `run`, `resume` and `gate-a` exit 1 when any row has status
   `error` (`gate-a` also on a mismatch). `resume` writes an error row for a
   checkpoint that cannot be loaded and goes on with the other files of a
   directory (before: exit 1 with no row, or exit 0 with an error row).
4. A cap at or below the checkpoint's node count: the error row reports the
   checkpoint's totals, segment, closed share, depth, deepest and hash.
5. Root refutation: closed share 1.0, depth 0.
6. The progress file is opened (its directory created) before the search;
   failure is an error before any node. A failed progress write or periodic
   checkpoint write (before: ignored) makes the row an error.
7. New subcommand `tamper` (gate G-e below).

None of this touches `dfs`, `branch`, `dfs_resume`, `propagate`, `choose`
or the per-node event check; `event` (cold, not inlined) changes only in how
it reports a failed write.

**Gates rerun.** G-a, G-b and G-c are rerun exactly as preregistered with the
fixed binary (a copy at `compute/target/gate-bin/resumable_runs`, so a later
build cannot change it mid-run), writing to `results/fix/raw/` and
`results/fix/ckpt/` (the committed version 1 results stay as they are).
Pass: the same checks as before, 0 mismatches, and additionally each rerun
row equal to the committed version 1 row of the same instance and file on
status, nodes, backtracks, propagation, board, hash, hashed, closed share
and deepest. G-d is not rerun (see the README).

**New gate G-e (tamper suite).** Checkpoints written by the fixed binary
at a cap: 7x7 primary test seed 1 at 400,000 nodes (the judge's case), 8x8
8-c10 test seed 5 at 333,333, 9x9 plus2 test seed 1 at 500,000. For each,
`resumable_runs tamper` enumerates alterations from the file itself: every
JSON leaf at any depth (integers +1 and, when above 0, -1; floats to the next
f64 and +1; 16-hex strings with the last digit changed; other strings with a
character appended), every object key removed, an unknown key added to every
object, every array (the stack, each option list, each option) with its last
element dropped, its first duplicated and its first two swapped when they
differ, `version` set to 1 and 3, and truncation at bytes 0, 1, len/4,
len/2, 3len/4, len-2, len-1. This covers each level's index and option list,
each total, the hash, the hashed count, the instance identity, the cap and
the version. Each altered file is resumed in a new process to total cap
(checkpoint nodes + 10,000).
**Pass:** every alteration refused (non-zero exit code and an error row),
and the unaltered copy resumes with exit 0 to a row equal to an
uninterrupted run to the same cap on status, nodes, backtracks, propagation,
board, hash, hashed, closed share and deepest. One accepted alteration fails
G-e. Reported, not gated: the same alterations with the digest recomputed
(forged), to show what the digest-independent checks catch alone.

**Unit tests.** `compute/tests/` (calibration seeds only), run with
`cargo test --release`: digest round trip and refusal of altered, version 1
and truncated text; D2 exit code and row through the binary; D3 on a puzzle
refuted at the root; a missing progress directory.

**Timing.** Not rerun: the hot path is unchanged (diff in the README).
