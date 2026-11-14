#!/usr/bin/env python3 -I
"""The loophole: painters that are NOT exchangeable (spatially stratified colour balance).

For each hypothetical painter this script reports
  (a) how many bits about the arrangement its prior would carry
      (log2 of all colour arrangements over the 420 interior-colour joints, minus log2 of the
      arrangements the painter can produce; lower and upper estimate), and
  (b) whether the official piece set could have come from it: the distribution, under the painter,
      of the number of inner pieces' equal-colour side pairs (opposite, adjacent), the only
      piece-level quantity a stratified painter shifts. Official values: 23 opposite, 47 adjacent.

Painters: the 420 joints are split into strata; every colour is spread as evenly as possible over
the strata; within a stratum the order is uniformly random.
Run: python3 -I 03_stratified_painters.py [boards per painter]
"""
import collections
import importlib.util
import json
import math
import os
import random
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("traces", os.path.join(HERE, "01_piece_set_traces.py"))
tr = importlib.util.module_from_spec(spec)
spec.loader.exec_module(tr)
N = 16
INTER = tr.INTER


def strata_of(kind, joints):
    lab = []
    for (a, b) in joints:
        r, c, side = a
        horiz = side == 1
        if kind == "exchangeable":
            lab.append(0)
        elif kind == "hv":
            lab.append(int(horiz))
        elif kind == "quadrants":
            lab.append((r >= 8) * 2 + (c >= 8))
        elif kind == "blocks16":
            lab.append((r // 4) * 4 + c // 4)
        elif kind == "lines":
            lab.append(("h", r) if horiz else ("v", c))
    return lab


def deal(rng, labels, colours_sorted):
    """Spread each colour as evenly as possible over strata; return colour per joint and the count matrix."""
    by = collections.defaultdict(list)
    for i, l in enumerate(labels):
        by[l].append(i)
    names = list(by)
    room = {l: len(by[l]) for l in names}
    seq = []
    while len(seq) < len(labels):
        order = [l for l in names if room[l] > 0]
        rng.shuffle(order)
        for l in order:
            seq.append(l)
            room[l] -= 1
    got = collections.defaultdict(list)
    for col, l in zip(colours_sorted, seq):
        got[l].append(col)
    out = [0] * len(labels)
    mat = {}
    for l in names:
        cols = got[l]
        rng.shuffle(cols)
        for i, col in zip(by[l], cols):
            out[i] = col
        mat[l] = collections.Counter(cols)
    return out, mat, {l: len(by[l]) for l in names}


def lg(n):
    return math.lgamma(n + 1)


def main():
    boards = int(sys.argv[1]) if len(sys.argv) > 1 else 2000
    rng = random.Random(20261112)
    off = tr.load()
    side_count = collections.Counter(v for p in off for v in p)
    ring, seam, inner = tr.joints()
    slots = seam + inner
    frame_cols = [c for c in tr.FRAME for _ in range(side_count[c] // 2)]
    inter_sorted = [c for c in INTER for _ in range(side_count[c] // 2)]
    n_c = {c: side_count[c] // 2 for c in INTER}
    ln_all = lg(420) - sum(lg(n) for n in n_c.values())
    obs = tr.stats(off)
    o6, o7 = obs["T6_inner_opposite_equal"], obs["T7_inner_adjacent_equal"]
    res = {}
    print(f"official: opposite-equal {o6}, adjacent-equal {o7}, sum {o6 + o7}")
    print(f"{'painter':13s} {'strata':>6s} {'bits lo':>8s} {'bits hi':>8s} {'mean opp':>8s} {'mean adj':>8s} {'sd sum':>7s} {'z official':>10s} {'P(sum>=obs)':>11s} {'sd opp':>7s} {'P(opp>=obs)':>11s}")
    for kind in ("exchangeable", "hv", "quadrants", "blocks16", "lines"):
        labels = strata_of(kind, slots)
        sums, opps, adjs, bits = [], [], [], []
        for _ in range(boards):
            cols, mat, sizes = deal(rng, labels, inter_sorted)
            ln_fixed = sum(lg(sizes[l]) - sum(lg(k) for k in mat[l].values()) for l in sizes)
            bits.append((ln_all - ln_fixed) / math.log(2))
            rng.shuffle(frame_cols)
            b = [[[0, 0, 0, 0] for _ in range(N)] for _ in range(N)]
            for (x, y), col in zip(ring, frame_cols):
                b[x[0]][x[1]][x[2]] = col
                b[y[0]][y[1]][y[2]] = col
            for (x, y), col in zip(slots, cols):
                b[x[0]][x[1]][x[2]] = col
                b[y[0]][y[1]][y[2]] = col
            st = tr.stats([tuple(b[r][c]) for r in range(N) for c in range(N)])
            opps.append(st["T6_inner_opposite_equal"])
            adjs.append(st["T7_inner_adjacent_equal"])
            sums.append(opps[-1] + adjs[-1])
        S = len(set(labels))
        # upper bound on the number of admissible count matrices: each colour chooses which strata get its remainder
        ln_k = 0.0
        for c, n in n_c.items():
            rem = n % S
            ln_k += lg(S) - lg(rem) - lg(S - rem)
        bits_hi = sum(bits) / len(bits)
        bits_lo = max(0.0, bits_hi - ln_k / math.log(2))
        m = sum(sums) / len(sums)
        sd = math.sqrt(sum((x - m) ** 2 for x in sums) / (len(sums) - 1))
        p = sum(x >= o6 + o7 for x in sums) / len(sums)
        z = (o6 + o7 - m) / sd
        mo = sum(opps) / boards
        sdo = math.sqrt(sum((x - mo) ** 2 for x in opps) / (boards - 1))
        po = sum(x >= o6 for x in opps) / boards
        res[kind] = {"strata": S, "bits_lo": bits_lo, "bits_hi": bits_hi, "mean_opp": sum(opps) / boards,
                     "mean_adj": sum(adjs) / boards, "sd_sum": sd, "z_official": z, "p_sum_ge_official": p,
                     "sd_opp": sdo, "p_opp_ge_official": po}
        print(f"{kind:13s} {S:6d} {bits_lo:8.1f} {bits_hi:8.1f} {sum(opps) / boards:8.2f} {sum(adjs) / boards:8.2f} {sd:7.2f} {z:10.2f} {p:11.4f} {sdo:7.2f} {po:11.4f}")
    json.dump(res, open(os.path.join(HERE, "03_stratified_painters.json"), "w"), indent=1)


if __name__ == "__main__":
    main()
