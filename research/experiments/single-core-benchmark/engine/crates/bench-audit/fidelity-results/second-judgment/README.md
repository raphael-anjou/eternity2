# Second judgment, raw outputs (2026-11-13)

Run output of this project's own engine (`blackwood_bt`, and the frozen Track
15 engine `fcq`), pulled from m1 `~/work/`. Plain text and JSON only;
nothing is edited. The numbers they back are in the "Second judgment" section
of `../../src/bin/JUDGE.md`.

- `d2-judge/runs/`: the first judgment's pre-branch (`old`, 637a42be) outputs,
  no-clue seeds 101 to 104 and 108, five clues seeds 501 to 506. Each folder
  holds `stdout`, `stderr` (with `/usr/bin/time -p`) and the emitted boards.
  The input files `noclue.csv`, `five.csv`, `centre.csv` are not in this pull.
- `d2-rejudge/runs/<name>/`: one folder per run, produced by `one.sh`
  (`/usr/bin/time -p <binary> <csv> <args> --emit-dir <dir>`). Command lines
  are in `d2-rejudge/jobs.txt` (a, b, c, leg-*, hnew-*, hj1-*), `abl.txt` and
  `abl2.txt` (x-*), and `r_batch.sh`, `r_abl.sh`, `r_abl2.sh`.
  - `a-*`: old binary, defaults, five clues, 501 to 508, 1e8 nodes.
  - `b-*`: new binary (1a4e5882) with the six legacy switches, same seeds.
  - `c-*`: new binary with its defaults, same seeds, `--hist`.
  - `leg-*`: legacy switches against the first judgment's `old` outputs.
  - `hnew-*` / `hj1-*`: 1e7 nodes, `--hist`, seeds 1 to 4, new defaults
    against the first judgment's binary (07c96165).
  - `x-*`: five-clue ablations (`ro` row-major, `kl` legacy keying, `publeg`
    published order with the other legacy switches, `lh` legacy hints, `lq`
    legacy quota test, `rolh` row-major with legacy hints).
  - `fcq/`: `r_fcq.sh`, `fcq bw ... --priv 22,21,20` with and without
    `--orientfix`, the legal accounting for a and b.
- `d2-rejudge/timing/`, `timing2/`: the F1 timing sets (`r_time.sh`,
  `r_time2.sh`); the host was loaded (`timing/uptime.txt`), so compare ratios.
- `d2-rejudge/test.log`, `build.out`: the 13 of 13 test run and build.
  `mut.out`, `mut-test.log`, `r_mut.sh`: the F2 mutation (legal sort reverted).
