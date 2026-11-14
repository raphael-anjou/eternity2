"""TOY BOARDS ONLY. Writes the generated tile lists used by iso.c, from the shared painter in
../far-ideas-geometry/common.py (same (n, cf, ci, seed) -> same tiles as gap 6a, 6b and
the far-ideas probes). File format: "n T" then T lines "N E S W" in painter order.
Usage: python3 -I export_boards.py
"""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "far-ideas-geometry"))
from common import gen  # noqa: E402

BOARDS = [(5, 3, 4, s) for s in range(5)] + [(6, 4, 5, s) for s in range(5)] + [(7, 4, 7, s) for s in range(2)]
os.makedirs(os.path.join(HERE, "boards"), exist_ok=True)
for n, cf, ci, s in BOARDS:
    t = gen(n, cf, ci, s)
    with open(os.path.join(HERE, "boards", f"b_{n}_{cf}_{ci}_{s}.txt"), "w") as f:
        f.write(f"{n} {len(t)}\n")
        for tile in t:
            f.write(" ".join(map(str, tile)) + "\n")
    print(n, cf, ci, s, len(t))
