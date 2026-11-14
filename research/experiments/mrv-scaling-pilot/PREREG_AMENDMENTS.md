# Amendments to PREREG.md

## A1 (2026-11-12, about 19:00 CET, before any test run): placement on three machines

What:

- A run may execute on any of three machines: this Mac (`mac-m4`, Apple M4,
  4 performance + 6 efficiency cores), `dell` (Linux x86_64, Intel Core Ultra
  7 165U, 14 threads) and `mac-m1` (macOS arm64, Apple M1, 8 cores). Remote
  machines are provisioned by the lead (code copied by rsync over ssh, Rust
  toolchain through `nix shell`); none is used before the lead confirms its
  directory, build command and determinism check.
- Both solvers of a seed still start together, on the same machine.
- Every row records `machine`, `cpu`, `os`, `binary` (path) and
  `binary_sha256` (the build), plus `started_at`, `ended_at`, `guard_s`.
  Rows go to `results/raw/wave<k>-<machine>.jsonl` (wave k = seeds 99 + 2k and
  100 + 2k as in PREREG.md). Remote runs execute under tmux or nohup so they
  survive an ssh disconnect, write rows to a file on the remote machine, and
  are fetched back with rsync and committed here unchanged (the machine name is
  already in each row).
- Node counts are machine-independent: both searches are deterministic (no RNG
  in bb-mrv, run seed 1 in scb), which the lead's determinism check verifies on
  each remote machine (A22 8x8 c=10 seeds against committed node counts) and
  this directory's build check verifies here. If time allows, one full pair's
  cheaper arm is re-run on a second machine and must give the same `nodes` and
  `status`.
- Wall clock and nodes/s are reported per machine and never compared across
  machines. The paired wall-clock ratio is reported only for pairs whose two
  runs ran on the same machine (always the case under the rule above).
- The 05:15 stop is the fixed epoch 1794543300 (2026-11-13 05:15:00 CET) on
  every machine, so remote clocks in another time zone stop at the same instant.

Why: the lead may add two machines so the 16 runs fit in the night; placement
must not change any number the kill criterion uses.

Effect on the kill criterion: none. The statistic, the threshold, the cap and
the censoring rules are unchanged.

## A2 (2026-11-12, 19:00 CET): mac-m1 launch repeated once

What: the first launch on `mac-m1` started the run script twice (two ssh
invocations, 18:58:39 and 18:58:43), so seeds 107 and 108 ran as 8 processes
writing to the same per-run files. All of them were killed about 30 s after
launch, before any row was written, and the 4 runs were launched once at
18:59:09. The aborted launch's heartbeat is kept as
`results/remote/mac-m1/heartbeat-mac-m1-aborted-launch.log`. Placement is the
lead's: seeds 101, 102 on `mac-m4`, 103 to 106 on `dell` (SLOTS=8), 107, 108 on
`mac-m1` (SLOTS=4). Both remote builds passed `pilot_instances check` and the
build check (plus2 7x7 seeds 1 to 10, both arms, node-identical to A22) before
launch; their check files are under `results/remote/<machine>/check/`.

Why: a launch error, not a result. No search output was seen.

Effect on the kill criterion: none.
