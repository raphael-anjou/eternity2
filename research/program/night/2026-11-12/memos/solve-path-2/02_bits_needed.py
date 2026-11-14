#!/usr/bin/env python3 -I
"""How many bits does a solver need? First-moment model of the row-scan search tree.

Ensemble: the official colour-side counts, kept per class (corner rim sides, edge
inward sides, inner-piece sides, and the two kinds of ring side), dealt uniformly over the sides of
that class. P(all joints of a fixed geometry match) is exact for that ensemble (see
the LNQ_* formulas). For partial boards the same quantity is spread evenly over the
joints of each class. That is the one approximation; it is exact at depth 256.

E_k = expected number of valid partial tilings of the first k cells in row-major
order = expected node count at depth k of a backtracker that checks the left and
top joints, plus the joints toward any hinted cell as soon as their other end is placed.
Sum_k E_k is the expected tree size; E_256 is the expected number of solutions.

Run: python3 -I 02_bits_needed.py
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
N = 16
LN2 = math.log(2)


def ln_dfact(n):  # ln (n!!) for odd n
    return sum(math.log(k) for k in range(n, 0, -2))


def ln_falling(n, k):
    return sum(math.log(n - i) for i in range(k))


def load_counts():
    """Per-colour side counts by class, read from the official piece set."""
    import collections
    csv = os.path.join(HERE, "..", "..", "..", "..", "..", "..", "engine", "data", "official_eternity2.csv")
    rows = [l.strip() for l in open(csv) if l.strip()][1:]
    seam = collections.Counter(); inner = collections.Counter()
    cw = collections.Counter(); acw = collections.Counter()
    for l in rows:
        p = [int(x, 2) for x in l.split(",")[:4]]
        p = [0 if v == 65535 else v for v in p]
        g = p.count(0)
        if g == 0:
            inner.update(p)
            continue
        # sides are listed clockwise (top, right, bottom, left). A frame-colour side whose clockwise
        # successor is grey is the side met when walking the ring one way; the other kind is met the other way.
        for i in range(4):
            if p[i] == 0:
                continue
            if p[(i + 1) % 4] == 0:
                cw[p[i]] += 1
            elif p[(i - 1) % 4] == 0:
                acw[p[i]] += 1
            else:
                seam[p[i]] += 1
    return seam, inner, cw, acw


SEAM, INNER, CW, ACW = load_counts()
assert sum(SEAM.values()) == 56 and sum(INNER.values()) == 784 and CW == ACW and sum(CW.values()) == 60
# Class-aware exact first moment.
# Seam: each of the 56 inward border sides meets an inner-piece side.
# Inner: the remaining 728 inner-piece sides pair among themselves.
# Ring: every ring joint pairs one piece's "cw" side with the next piece's "acw" side (bipartite),
#       so P = prod_c a_c! / 60! with a_c the per-colour count on each side kind.
LNQ_SEAM = (sum(ln_falling(INNER[c], SEAM[c]) for c in INNER) - ln_falling(784, 56)) / 56
LNQ_INT = (sum(ln_dfact(INNER[c] - SEAM[c] - 1) for c in INNER) - ln_dfact(727)) / 364
LNQ_FR = (sum(math.lgamma(CW[c] + 1) for c in CW) - math.lgamma(61)) / 60

OFFICIAL_HINTS = [(8, 7), (13, 2), (2, 2), (13, 13), (2, 13)]  # (row, col); only the cell matters here
CENTRE = [(8, 7)]


def kind(r, c):
    e = (r in (0, N - 1)) + (c in (0, N - 1))
    return ("inner", "edge", "corner")[e]


def joint_lnq(a, b):
    """ln match probability of the joint between adjacent cells a and b."""
    (r1, c1), (r2, c2) = a, b
    ka, kb = kind(*a), kind(*b)
    if ka == "inner" and kb == "inner":
        return LNQ_INT
    if "inner" in (ka, kb):
        return LNQ_SEAM
    return LNQ_FR


def tree(hints):
    """Return (list of ln E_k for k = 1..256, ln of sum, ln E_256)."""
    hs = set(hints)
    left = {"corner": 4, "edge": 56, "inner": 196 - len(hs)}
    placed = set(hs)  # hinted cells are on the board from the start
    ln_e = 0.0
    # joints between two hinted cells are satisfied by construction (none are adjacent here)
    out = []
    for r in range(N):
        for c in range(N):
            cell = (r, c)
            if cell not in hs:
                k = kind(r, c)
                ln_e += math.log(left[k] * (4 if k == "inner" else 1))
                left[k] -= 1
                for nb in ((r - 1, c), (r, c - 1), (r + 1, c), (r, c + 1)):
                    if 0 <= nb[0] < N and 0 <= nb[1] < N and nb in placed:
                        ln_e += joint_lnq(cell, nb)
                placed.add(cell)
            out.append(ln_e)
    m = max(out)
    ln_sum = m + math.log(sum(math.exp(x - m) for x in out))
    return out, ln_sum, out[-1]


def main():
    res = {"q_inner": math.exp(LNQ_INT), "q_seam": math.exp(LNQ_SEAM), "q_ring": math.exp(LNQ_FR)}
    print("effective match probability per joint: " + ", ".join(f"{k} 1/{1 / v:.3f}" for k, v in res.items()))
    for name, h in (("centre clue only", CENTRE), ("five clues", OFFICIAL_HINTS)):
        prof, ln_sum, ln_sol = tree(h)
        res[name] = {"log2_tree": ln_sum / LN2, "log10_tree": ln_sum / math.log(10),
                     "expected_solutions": math.exp(ln_sol), "log2_peak": max(prof) / LN2,
                     "peak_depth": prof.index(max(prof)) + 1}
        print(f"{name:18s} E[solutions]={math.exp(ln_sol):.3e}  tree=2^{ln_sum / LN2:.1f}=10^{ln_sum / math.log(10):.1f}  peak depth {prof.index(max(prof)) + 1}")
    # greedy best extra correct placements (interior cells), model-optimal positions
    hints = list(OFFICIAL_HINTS)
    curve = []
    _, ln_sum, _ = tree(hints)
    curve.append((0, ln_sum / LN2, None))
    for extra in range(1, 41):
        best = None
        for r in range(1, N - 1):
            for c in range(1, N - 1):
                if (r, c) in hints:
                    continue
                if any(abs(r - a) + abs(c - b) == 1 for a, b in hints):
                    continue  # keep hints non-adjacent so the joint model stays valid
                _, s, _ = tree(hints + [(r, c)])
                if best is None or s < best[0]:
                    best = (s, (r, c))
        hints.append(best[1])
        curve.append((extra, best[0] / LN2, best[1]))
    res["greedy_curve"] = curve
    print("\nextra correct placements (greedy, model-optimal cells) -> log2 expected tree size")
    for extra, b, cell in curve:
        print(f"  +{extra:2d}  2^{b:6.1f}   bits bought so far {curve[0][1] - b:6.1f}   cell {cell}")
    # budgets
    rate = 1e8  # placements per core-second, assumption
    for name, cores_years in (("3 desktops x 16 cores x 1 year", 48), ("1000 core-years", 1000)):
        nodes = rate * cores_years * 365.25 * 86400
        b = math.log2(nodes)
        need = curve[0][1] - b
        k = next((e for e, x, _ in curve if x <= b), None)
        res[name] = {"log2_budget": b, "bits_short": need, "extra_placements_needed": k}
        print(f"{name}: budget 2^{b:.1f} placements; short by {need:.1f} bits; needs {k} extra correct placements at model-optimal cells")
    # regular lattices of 18 hints, no official clues, in the spirit of McGavin's layout
    # ("every third column on a few odd rows"; his exact cells are not known to this script)
    lattices = {
        "top rows 1,3,5 x cols 2,5,8,11,14 + (7,2),(7,8),(7,14)": [(r, c) for r in (1, 3, 5) for c in (2, 5, 8, 11, 14)] + [(7, 2), (7, 8), (7, 14)],
        "top rows 1,3,5 x cols 1,4,7,10,13 + (7,4),(7,7),(7,10)": [(r, c) for r in (1, 3, 5) for c in (1, 4, 7, 10, 13)] + [(7, 4), (7, 7), (7, 10)],
        "spread rows 1,5,9,13 x cols 2,5,8,11,14 (first 18)": [(r, c) for r in (1, 5, 9, 13) for c in (2, 5, 8, 11, 14)][:18],
    }
    res["lattice18"] = {}
    for name, lat in lattices.items():
        _, s, _ = tree(lat)
        res["lattice18"][name] = s / LN2
        print(f"18-hint lattice, {name}: tree 2^{s / LN2:.1f}")
    json.dump(res, open(os.path.join(HERE, "02_bits_needed.json"), "w"), indent=1)


if __name__ == "__main__":
    main()
