# Hot-path speed and identity after the speed fix (judge finding F1)

Ran on m1, 2026-11-13, one thread per run. Binaries: `base` = pre-branch
blackwood_bt (637a42be), `c8af01491` = this branch before the speed fix
(8af01491), `head` = after it (e1cb4a3e).

- `timing.txt` (`scripts/timing_remote.sh`): user seconds for 1e8 nodes, no-clue
  file, seed 102, one restart, three repeats each run alone; branch binaries
  with the six legacy switches. base 1.05 to 1.06 s, before the fix 1.47 s,
  after 1.24 to 1.28 s. Every run has the same node count.
- `identity.txt` (`scripts/f1_remote.sh`, then `scripts/f1cmp.py`):
  - legacy switches against `base`, 1e8 nodes each: no-clue seeds 101, 102,
    103, 104, 108 and 5-clue seeds 501 to 506. 11 of 11 pairs agree on nodes,
    best scores, deepest depth, completion histogram and emitted board URL
    (`raw/url-*.txt`; runs without a physical board emit none on either side).
    The only text that differs is the new `illegal_placements=` field.
  - new defaults, centre-only file, seeds 1 to 4 at 1e7 nodes: the per-depth
    node histogram is identical before and after the fix (`raw/pre-ce-*`,
    `raw/post-ce-*`).
