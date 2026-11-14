# Real-prefix part: plan (2026-11-14)

Second half of the pre-stated test of the toy part (see `../README.md`): learning against propagation below wrong prefixes of the real Eternity II pieces. Nothing here is Eternity II progress.

Order of events: this plan was written after a first pilot and before the full run. Only the stop line quoted below predates the data with an external record; the rest of the plan is documented here, not independently timestamped.

## Design fixed in advance

- Prefixes: `scripts/mkprefixes.c` (sampler and its bias are described in its header), 40 at each depth 112, 104, 96, 88 (seed 1000*d + i). Row scan from cell 0; the five clues are on the board from the start. Depth d means d free (non-clue) cells placed.
- Arms, all on the same prefixes, each exhausting the subtree: T `rowscan_real.c` (nodes), AC `dyn.c AC ROW` (branches tried, and failed = tried - alive), K CaDiCaL 1.5.3 via `cdcl_pysat.py` (conflicts), C clasp 3.3.10 default (conflicts), U0 clasp `--no-lookback --lookahead=no --heuristic=None` (conflicts). CNF from `enc_real.py`.
- One worker per core; the five arms of a prefix run one after the other on that worker, order shuffled by prefix name.
- Limit 1200 s wall inside every arm; backstop SIGTERM at 1230 s. Counts printed on every exit.
- Pilot: prefixes 00 to 02 of each depth on machine-c. Full run only if the estimate is under 40 core-hours. The full run reruns the pilot prefixes; the pilot outputs stay in `results/raw/pilot-corrected/` and are not pooled.

## Analysis rules fixed in advance

- Work of a run = count + 1 (a prefix refuted at the root with no search has count 0).
- Lead of X over Y on a prefix = log2(work Y / work X). Stop line: the median over 40 prefixes of the lead of K over AC (AC branches tried), depth 88 minus depth 104, against 3 bits. Quoted from the pre-stated test: the lead "does not rise by ... 3 bits from depth 104 to 88".
- Also: K over AC failed branches; C over AC; U0 over AC; every arm against T; the same in seconds.
- Seconds: wall seconds of the search phase as each program reports it (T, AC, K internal monotonic clock; clasp "Solving"), floored at 0.01 s (clasp's resolution). Also total wall and processor seconds of the whole process measured from outside, which include reading the instance.
- A cut run gives a bound, not a value: its count and seconds are lower bounds. Medians are reported as an interval [low, high] obtained by pushing every bounded lead to minus or plus infinity on its open side. Bootstrap (10,000 resamples of prefixes, percentile 95%) on both ends.
- Secondary, because the cost of a search is its mean: log2(sum of work Y / sum of work X) per depth.
- The per-prefix table is committed in full; nothing is dropped. A run with no counts is rerun.

## Correction of 2026-11-14, before any run that counts

The first pilot used a reversed depth definition (251 - d free cells placed, 139 to 163 in all) and was discarded before the full run. Every subtree then had 0 to 51 nodes and every solver did 0 or 1 unit of work, so those data carry nothing about the comparison. They are kept in `../wrong-depth-pilot/` and used nowhere. `mkprefixes.c` was changed to place d free cells; the seed rule (1000*d + i) and everything above are unchanged. The definition was checked against the mean subtree sizes of the real-puzzle profile (2^12.4, 2^17.1, 2^18.3, 2^24.2 at depths 112, 104, 96, 88); the independent re-check later showed those profile figures belong to a scan in the opposite row order and found the definition confirmed at depths 96 and 88 (see `recheck.md`).

## Pilot under the corrected definition and the launch decision

12 prefixes on machine-c, 4 workers: all 60 runs finished, none cut, all arms agree (no completion anywhere), 0.21 core-hours in all, of which 0.15 on one prefix (d88_02: tree 8.7 million nodes). Peak memory of K at depth 88: 6.5 GB (24.6 million clauses), so the number of workers is set by memory: machine-c 3, machine-a 2 (16 GB), machine-b 4 (cpus 0, 2, 4, 5, pinned). Prefix index i goes to machine-c when i mod 9 < 3, to machine-a when it is 3 or 4, to machine-b otherwise; every machine gets all four depths. Estimate for 160 prefixes: 2.8 core-hours at the pilot's mean; with an allowance for a tail the pilot did not reach (say 8 prefixes with four arms cut, 80 minutes each) about 14 core-hours. Under 40, so the full run was launched. Hard ceiling if every run were cut: 267 core-hours.
