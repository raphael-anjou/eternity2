"""Part B in the first-moment model: what one more known tile is worth, and where.

    python3 -I scripts/pins_model.py ORDER     # writes pins/model.json

The model sees only positions (which class a pinned cell belongs to and which
joints the order closes against it), never the pinned piece's colours, so its
predictions are the same for the official set and for every census-painted
board (they share the official colour census exactly). These are model
predictions, graded argued wherever they are quoted for the real pieces.

Output: single-pin gain (log10 and bits) for every non-clue cell under ORDER;
the greedy pin sequence (add the cell with the largest model gain, ties to
the lowest pos) until the model tree falls under 2^61.5 nodes, continued to
k = 64; the model tree of uniform random pin sets of size k (median and
quartiles over 200 sets per k, seed 20261113).
"""
import json
import math
import os
import random
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
EXP = os.path.dirname(HERE)
sys.path.insert(0, HERE)
from model import Model  # noqa: E402
from orders import CLUES16, load_official  # noqa: E402

B615 = 61.5 * math.log10(2)  # 1,000 core-years at 1e8 nodes per second
B78 = 78 * math.log10(2)
L2 = math.log2(10)


def read_order(name):
    raw = open(os.path.join(EXP, "orders", "16", f"{name}.txt")).read()
    v = [int(t) for l in raw.splitlines() if not l.startswith("#") for t in l.split()]
    return [(p // 16, p % 16) for p in v]


def main():
    name = sys.argv[1]
    M = Model(16, load_official())
    order = read_order(name)
    base = M.total(order, CLUES16)
    cells = [(r, c) for r in range(16) for c in range(16) if (r, c) not in CLUES16]
    single = {}
    for cell in cells:
        # rounded so that model ties (equal to 1e-9) break by position, not float noise
        single[cell] = round(base - M.total(order, CLUES16 + [cell]), 9)
    ranked = sorted(cells, key=lambda x: (-single[x], x[0] * 16 + x[1]))
    greedy, gt = [], [base]
    pins = []
    while len(greedy) < 64:
        cur = M.total(order, CLUES16 + pins)
        best, bv = None, None
        for cell in cells:
            if cell in pins:
                continue
            v = round(M.total(order, CLUES16 + pins + [cell]), 9)
            if bv is None or v < bv:
                best, bv = cell, v
        pins.append(best)
        greedy.append(best)
        gt.append(bv)
    k615 = next(k for k, t in enumerate(gt) if t < B615)
    k78 = next(k for k, t in enumerate(gt) if t < B78)
    rng = random.Random(20261113)
    rand = {}
    for k in [1, 2, 4, 8, 12, 16, 24, 32, 48, 64, 96, 128]:
        ts = sorted(M.total(order, CLUES16 + rng.sample(cells, k)) for _ in range(200))
        rand[k] = {"median": ts[100], "q25": ts[50], "q75": ts[150], "share_under_2^61.5": sum(t < B615 for t in ts) / 200,
                   "share_under_2^78": sum(t < B78 for t in ts) / 200}
    out = {
        "order": name, "model_log10_base": base, "budget_log10_2^61.5": B615, "budget_log10_2^78": B78,
        "single_gain_log10": {f"{r},{c}": round(single[(r, c)], 4) for (r, c) in cells},
        "ranked_best10": [[r, c, round(single[(r, c)], 4), round(single[(r, c)] * L2, 3)] for (r, c) in ranked[:10]],
        "ranked_worst10": [[r, c, round(single[(r, c)], 4), round(single[(r, c)] * L2, 3)] for (r, c) in ranked[-10:]],
        "median_single_gain_log10": sorted(single.values())[len(single) // 2],
        "greedy": [[r, c] for (r, c) in greedy],
        "greedy_log10_tree": [round(t, 4) for t in gt],
        "greedy_k_under_2^61.5": k615, "greedy_k_under_2^78": k78,
        "random_log10_tree": rand,
    }
    os.makedirs(os.path.join(EXP, "pins"), exist_ok=True)
    with open(os.path.join(EXP, "pins", "model.json"), "w") as fh:
        json.dump(out, fh, indent=1)
    print("base", round(base, 3), "k under 2^61.5:", k615, "k under 2^78:", k78)
    print("best", out["ranked_best10"][:5])
    print("worst", out["ranked_worst10"][-5:])
    print("median single gain (log10)", round(out["median_single_gain_log10"], 3))
    print("greedy", greedy[:20])
    print("greedy tree", [round(t, 2) for t in gt[:25]])
    for k, v in rand.items():
        print("random k", k, {a: round(b, 3) for a, b in v.items()})


if __name__ == "__main__":
    main()
