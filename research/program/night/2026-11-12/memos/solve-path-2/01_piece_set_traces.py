#!/usr/bin/env python3 -I
"""Design-trace statistics on the official Eternity II piece set against a matched null.

Null ensemble: a solved 16x16 board painted with the official colour multiset
(12 ring joints for each of the 5 frame colours; 24 or 25 joints for each of the 17
interior colours, spread uniformly at random over the 56 seam + 364 inner joints),
then read off as 256 pieces. This is the "exchangeable painter": every arrangement of
the fixed colour multiset over the joints of its class is equally likely.
A second column conditions the null on the known filter (no duplicate piece, no
rotation-symmetric piece).

Every statistic is a function of the piece multiset only (rotation and order free).
Run: python3 -I 01_piece_set_traces.py [trials] [seed]
"""
import collections
import json
import math
import os
import random
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CSV = os.path.join(HERE, "..", "..", "..", "..", "..", "..", "engine", "data", "official_eternity2.csv")
N = 16
FRAME = (1, 2, 3, 4, 5)
INTER = tuple(range(6, 23))


def load():
    rows = [l.strip() for l in open(CSV) if l.strip()]
    out = []
    for l in rows[1:]:
        c = l.split(",")
        e = [int(x, 2) for x in c[:4]]
        out.append(tuple(0 if v == 65535 else v for v in e))
    return out


def canon(p):
    return min(p[i:] + p[:i] for i in range(4))


def top_abs_eig(M):
    """Largest |eigenvalue| of a symmetric matrix by power iteration on M^2."""
    n = len(M)
    v = [math.sin(1.0 + 0.7 * i) for i in range(n)]
    lam = 0.0
    for _ in range(60):
        w = [sum(M[i][j] * v[j] for j in range(n)) for i in range(n)]
        w = [sum(M[i][j] * w[j] for j in range(n)) for i in range(n)]
        nrm = math.sqrt(sum(x * x for x in w))
        if nrm == 0:
            return 0.0
        v = [x / nrm for x in w]
        lam = nrm
    return math.sqrt(lam)


def stats(pieces):
    """pieces: list of 4-tuples (N,E,S,W), 0 = grey rim."""
    s = {}
    corners = [p for p in pieces if p.count(0) == 2]
    edges = [p for p in pieces if p.count(0) == 1]
    inner = [p for p in pieces if p.count(0) == 0]
    # T1 seam flatness: inward colour of each edge piece
    seam = collections.Counter()
    lr = collections.Counter()
    same_lr = 0
    for p in edges:
        g = p.index(0)
        inward = p[(g + 2) % 4]
        a, b = p[(g + 1) % 4], p[(g + 3) % 4]  # clockwise side, anticlockwise side
        seam[inward] += 1
        lr[(a, b)] += 1
        same_lr += a == b
    mu = 56 / 17
    s["T1_seam_dispersion"] = sum((seam[c] - mu) ** 2 for c in INTER)
    # T2 corner sides flatness
    cs = collections.Counter(v for p in corners for v in p if v)
    s["T2_corner_dispersion"] = sum((cs[c] - 8 / 5) ** 2 for c in FRAME)
    # T3 edge pieces with the same frame colour on both rim-parallel sides
    s["T3_edge_same_lr"] = same_lr
    # T4 chi-square of the 5x5 (clockwise, anticlockwise) frame colour table of edge pieces
    ra = collections.Counter()
    rb = collections.Counter()
    for (a, b), k in lr.items():
        ra[a] += k
        rb[b] += k
    chi = 0.0
    for a in FRAME:
        for b in FRAME:
            e = ra[a] * rb[b] / 56
            if e > 0:
                chi += (lr[(a, b)] - e) ** 2 / e
    s["T4_frame_pair_chi2"] = chi
    # T5 seam sides carried by the five 24-joint interior colours (6..10)
    s["T5_seam_in_small_colours"] = sum(seam[c] for c in range(6, 11))
    # interior pieces
    opp_eq = adj_eq = 0
    distinct = 0
    adj = collections.Counter()   # ordered clockwise (a, b)
    opp = collections.Counter()   # unordered
    for p in inner:
        distinct += len(set(p))
        for i in range(4):
            a, b = p[i], p[(i + 1) % 4]
            adj[(a, b)] += 1
            adj_eq += a == b
        for i in range(2):
            a, b = p[i], p[i + 2]
            opp[(min(a, b), max(a, b))] += 1
            opp_eq += a == b
    s["T6_inner_opposite_equal"] = opp_eq
    s["T7_inner_adjacent_equal"] = adj_eq
    s["T8_inner_mean_distinct"] = distinct / len(inner)
    # T9 chirality: clockwise (a,b) against (b,a)
    ch = 0.0
    for a in INTER:
        for b in INTER:
            if a < b:
                x, y = adj[(a, b)], adj[(b, a)]
                if x + y:
                    ch += (x - y) ** 2 / (x + y)
    s["T9_chirality"] = ch
    # side totals on inner pieces
    tot = collections.Counter(v for p in inner for v in p)
    nside = 4 * len(inner)
    # T10 adjacent unordered pair chi2, T11 opposite pair chi2 (against product of side shares)
    chi_a = chi_o = 0.0
    idx = {c: i for i, c in enumerate(INTER)}
    D = [[0.0] * 17 for _ in range(17)]
    n_adj = 4 * len(inner)
    n_opp = 2 * len(inner)
    for a in INTER:
        for b in INTER:
            if a > b:
                continue
            pa, pb = tot[a] / nside, tot[b] / nside
            share = pa * pb * (1 if a == b else 2)
            oa = adj[(a, b)] + (adj[(b, a)] if a != b else 0)
            oo = opp[(a, b)]
            ea, eo = n_adj * share, n_opp * share
            chi_a += (oa - ea) ** 2 / ea
            chi_o += (oo - eo) ** 2 / eo
            # T12 matrix: opposite pairs minus half the adjacent pairs (zero mean under exchangeability)
            d = oo - oa / 2
            if a == b:
                D[idx[a]][idx[a]] = 2 * d
            else:
                D[idx[a]][idx[b]] = d
                D[idx[b]][idx[a]] = d
    s["T10_adjacent_pair_chi2"] = chi_a
    s["T11_opposite_pair_chi2"] = chi_o
    s["T12_opp_minus_adj_top_eig"] = top_abs_eig(D)
    # filter facts
    cc = collections.Counter(canon(p) for p in pieces)
    s["F_duplicate_pairs"] = sum(k * (k - 1) // 2 for k in cc.values())
    s["F_rot_symmetric"] = sum(1 for p in pieces if p[0] == p[2] and p[1] == p[3])
    return s


def joints():
    """Return three lists of joints ((r,c,side),(r2,c2,side2)) by class."""
    ring, seam, inner = [], [], []
    for r in range(N):
        for c in range(N):
            if c + 1 < N:
                j = ((r, c, 1), (r, c + 1, 3))
                if r in (0, N - 1):
                    ring.append(j)
                elif c == 0 or c + 1 == N - 1:
                    seam.append(j)
                else:
                    inner.append(j)
            if r + 1 < N:
                j = ((r, c, 2), (r + 1, c, 0))
                if c in (0, N - 1):
                    ring.append(j)
                elif r == 0 or r + 1 == N - 1:
                    seam.append(j)
                else:
                    inner.append(j)
    return ring, seam, inner


def main():
    trials = int(sys.argv[1]) if len(sys.argv) > 1 else 6000
    seed = int(sys.argv[2]) if len(sys.argv) > 2 else 20261112
    rng = random.Random(seed)
    off = load()
    side_count = collections.Counter(v for p in off for v in p)
    ring, seam, inner = joints()
    assert (len(ring), len(seam), len(inner)) == (60, 56, 364)
    frame_cols = [c for c in FRAME for _ in range(side_count[c] // 2)]
    inter_cols = [c for c in INTER for _ in range(side_count[c] // 2)]
    assert len(frame_cols) == 60 and len(inter_cols) == 420
    obs = stats(off)
    keys = [k for k in obs if k.startswith("T")]
    samples = {k: [] for k in keys}
    cond = {k: [] for k in keys}
    slots = seam + inner
    for _ in range(trials):
        rng.shuffle(frame_cols)
        rng.shuffle(inter_cols)
        b = [[[0, 0, 0, 0] for _ in range(N)] for _ in range(N)]
        for (x, y), col in zip(ring, frame_cols):
            b[x[0]][x[1]][x[2]] = col
            b[y[0]][y[1]][y[2]] = col
        for (x, y), col in zip(slots, inter_cols):
            b[x[0]][x[1]][x[2]] = col
            b[y[0]][y[1]][y[2]] = col
        ps = [tuple(b[r][c]) for r in range(N) for c in range(N)]
        st = stats(ps)
        ok = st["F_duplicate_pairs"] == 0 and st["F_rot_symmetric"] == 0
        for k in keys:
            samples[k].append(st[k])
            if ok:
                cond[k].append(st[k])
    out = {"trials": trials, "seed": seed, "conditioned_trials": len(cond[keys[0]]),
           "side_counts": dict(sorted(side_count.items())), "stats": {}}
    print(f"trials={trials} conditioned={out['conditioned_trials']}")
    print(f"{'statistic':32s} {'official':>9s} {'null mean':>9s} {'sd':>7s} {'z':>6s} {'P(<=)':>7s} {'P(>=)':>7s} {'cond mean':>9s} {'cond z':>6s}")
    for k in keys:
        xs = samples[k]
        m = sum(xs) / len(xs)
        sd = math.sqrt(sum((x - m) ** 2 for x in xs) / (len(xs) - 1))
        lo = sum(x <= obs[k] + 1e-9 for x in xs) / len(xs)
        hi = sum(x >= obs[k] - 1e-9 for x in xs) / len(xs)
        ys = cond[k]
        cm = sum(ys) / len(ys) if ys else float("nan")
        csd = math.sqrt(sum((y - cm) ** 2 for y in ys) / (len(ys) - 1)) if len(ys) > 1 else float("nan")
        z = (obs[k] - m) / sd if sd else 0.0
        cz = (obs[k] - cm) / csd if csd and csd == csd else float("nan")
        out["stats"][k] = {"official": obs[k], "null_mean": m, "null_sd": sd, "z": z, "p_le": lo, "p_ge": hi,
                           "cond_mean": cm, "cond_sd": csd, "cond_z": cz}
        print(f"{k:32s} {obs[k]:9.3f} {m:9.3f} {sd:7.3f} {z:6.2f} {lo:7.4f} {hi:7.4f} {cm:9.3f} {cz:6.2f}")
    json.dump(out, open(os.path.join(HERE, "01_piece_set_traces.json"), "w"), indent=1)


if __name__ == "__main__":
    main()
