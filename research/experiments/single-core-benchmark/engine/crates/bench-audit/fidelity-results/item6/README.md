# Item 6 property check (raw rows)

Evidence for FIDELITY.md item 6. Ran on m1, 2026-11-13, one thread per run.

- `bw-<seed>.txt`: stderr of this repo's `blackwood_bt` at commit 8af01491 with
  the six legacy switches (`--keying legacy-port --order row-major
  --legacy-break-path --isolation --legacy-hints --legacy-quota-test`) plus
  `--priv`, `--hist`, node cap as in the folder name.
- `off-<seed>.json`, `onl-<seed>.json`, `on-<seed>.json`: the frozen Track 15
  engine `fcq` (branch `night/2026-11-12-t15-five-clue-quota`,
  `research/experiments/five-clue-quota/engine`): fix off with a legal-node cap,
  fix on with a legal-node cap, fix on with a node cap.
- Folders: `f5` official 5-clue file, (22, 21, 20), seeds 501 to 516, 1e7;
  `f5late` same file, seeds 501, 506, 507, 1e8; `fn13` no-clue file, seeds 101
  to 110, 1e7; `fn13m3` no-clue file, (3, 12, 13), seeds 105 and 108, 1e7.
  The fix-off runs had a node cap of 1.3e8 so the legal cap could be reached.
- `scripts/`: `final_prop.sh` drives `prop1.sh` (ship, run, fetch), which runs
  `prop1_remote.sh` on the host; `prop1.py <folder>` prints one line per seed.
  `summary.txt` is that output: 31 of 31 `bw==fcq-on hist True`; 26 of 26
  non-stranded runs `off.hist_legal==onl.hist_legal True`.

The emitted board files of these runs are not kept (greedy-tail completions,
not results).
