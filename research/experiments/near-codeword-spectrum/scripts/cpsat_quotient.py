"""Round-2 fix solves (PREREG_AMENDMENTS.md A6 to A8).

    python scripts/cpsat_quotient.py --instances results/instances.jsonl \
        --stage {star1,star7,k1c} --out results/raw/<stage>.jsonl

star1/star7: minimise breaks subject to the rotation-quotiented distance
d* = min_k d_k in a band, d_k = colour-tuple distance to the planted board
turned k quarter turns (pin kept, so each rotation class appears once).
k1c: the original K1c test (distance d_0, mid band) with breaks capped at the
free far-band certificate (turned planted copy, corner swapped back).
"""
import argparse
import json
import math
import os
import sys
import time
from multiprocessing import Pool

from ortools.sat.python import cp_model

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cpsat_spectrum import GRAY, bands, check_board, joints, legal, placements, rot  # noqa: E402


def rot_cell(n, c):
    x, y = c % n, c // n
    return x * n + (n - 1 - y)


def turned(n, pieces, k):
    """Planted board turned k quarter turns clockwise: list of (piece, rot) per cell."""
    board = [None] * (n * n)
    for c in range(n * n):
        d = c
        for _ in range(k):
            d = rot_cell(n, d)
        board[d] = (c, k % 4)
    return board


def targets(n, pieces):
    """T[k][c] = colour tuple of the planted board turned k times, at cell c."""
    return [[rot(pieces[p], r) for p, r in turned(n, pieces, k)] for k in range(4)]


def dstar(n, pieces, board):
    tup = [rot(pieces[p], r) for p, r in board]
    T = targets(n, pieces)
    ds = [sum(1 for c in range(n * n) if tup[c] != T[k][c]) for k in range(4)]
    return min(ds), ds


def legal_rot(n, c, e):
    for r in range(4):
        if legal(n, c, rot(e, r)):
            return r
    raise AssertionError("no legal rotation")


def free_certificates(n, pieces):
    """For k = 1..3: planted board turned k times with the planted corner (piece 0)
    swapped back into cell 0. Returns [(k, breaks, d0, d*), ...]."""
    out = []
    for k in (1, 2, 3):
        b = turned(n, pieces, k)
        j = next(c for c in range(n * n) if b[c][0] == 0)
        q = b[0][0]
        b[0] = (0, legal_rot(n, 0, pieces[0]))
        b[j] = (q, legal_rot(n, j, pieces[q]))
        br, a = check_board(n, [tuple(e) for e in pieces], b)
        out.append((k, br, a, dstar(n, [tuple(e) for e in pieces], b)[0]))
    return out


def solve(job):
    n, pieces = job["n"], [tuple(e) for e in job["pieces"]]
    N = n * n
    lo, hi = job["lo"], job["hi"]
    m = cp_model.CpModel()
    opts = placements(n, pieces)
    x = {}
    for c in range(N):
        for p, r, t in opts[c]:
            x[c, p, r] = m.NewBoolVar("")
    for c in range(N):
        m.AddExactlyOne(x[c, p, r] for p, r, _ in opts[c])
    by_piece = {}
    for (c, p, r), v in x.items():
        by_piece.setdefault(p, []).append(v)
    for p in range(N):
        m.AddExactlyOne(by_piece[p])
    m.Add(x[0, 0, 0] == 1)
    side = {}
    for c in range(N):
        for p, r, t in opts[c]:
            for s in range(4):
                if t[s] != GRAY:
                    side.setdefault((c, s, t[s]), []).append(x[c, p, r])
    breaks_terms = []
    for a_, sa, b_, sb in joints(n):
        ka = {k for (c, s, k) in side if c == a_ and s == sa}
        kb = {k for (c, s, k) in side if c == b_ and s == sb}
        ms = []
        for k in sorted(ka & kb):
            v = m.NewBoolVar("")
            m.Add(v <= sum(side[a_, sa, k]))
            m.Add(v <= sum(side[b_, sb, k]))
            ms.append(v)
        br = m.NewBoolVar("")
        m.Add(sum(ms) + br == 1)
        breaks_terms.append(br)
    T = targets(n, pieces)
    ks = range(4) if job["quotient"] else (0,)
    dist = {k: N - sum(x[c, p, r] for c in range(N) for p, r, t in opts[c] if t == T[k][c]) for k in ks}
    for k in ks:
        m.Add(dist[k] >= lo)
    if hi < N:
        z = []
        for k in ks:
            zk = m.NewBoolVar("")
            m.Add(dist[k] <= hi).OnlyEnforceIf(zk)
            z.append(zk)
        m.AddBoolOr(z)
    obj = sum(breaks_terms)
    if job.get("cap") is not None:
        m.Add(obj <= job["cap"])
    m.Minimize(obj)
    s = cp_model.CpSolver()
    s.parameters.max_time_in_seconds = job["time"]
    s.parameters.num_workers = job["workers"]
    s.parameters.random_seed = 1000 * n + job["seed"]
    t0 = time.time()
    st = s.Solve(m)
    wall = time.time() - t0
    rec = {k: job[k] for k in ("key", "stage", "n", "colors", "seed", "kind", "lo", "hi", "time",
                                "workers", "quotient", "cap")}
    rec["status"] = s.StatusName(st)
    rec["wall_s"] = round(wall, 3)
    if st in (cp_model.OPTIMAL, cp_model.FEASIBLE):
        board = []
        for c in range(N):
            for p, r, t in opts[c]:
                if s.Value(x[c, p, r]):
                    board.append((p, r))
        br, a0 = check_board(n, pieces, board)
        ds, dks = dstar(n, pieces, board)
        a = ds if job["quotient"] else a0
        ub = int(round(s.ObjectiveValue()))
        if br != ub or not (lo <= a <= hi) or (job["quotient"] and min(dks) < lo):
            raise RuntimeError(f"witness mismatch {job['key']}: breaks {br} vs {ub}, a {a}, d_k {dks}")
        rec.update(ub=ub, lb=int(math.ceil(s.BestObjectiveBound() - 1e-6)), witness_a=a,
                   witness_d0=a0, witness_dk=dks, board=[p * 4 + r for p, r in board])
        if st == cp_model.OPTIMAL:
            rec["lb"] = ub
    else:
        rec.update(ub=None, lb=None, witness_a=None, witness_d0=None, witness_dk=None, board=None)
    return rec


def qbands(n):
    b = bands(n)
    N = n * n
    return {"near": b["near"], "mid": b["mid"], "far": (b["far"][0], N), "k2": b["k2"]}


def jobs_for(stage, insts, seeds_override=None):
    jobs = []

    def add(n, c, seed, kind, lo, hi, t, w, quotient, cap=None):
        inst = insts[n, c, seed]
        jobs.append(dict(key=f"{n}x{n}/C{c}/s{seed}/{kind}", stage=stage, n=n, colors=c, seed=seed,
                         kind=kind, lo=lo, hi=hi, time=t, workers=w, quotient=quotient, cap=cap,
                         pieces=inst["pieces"]))

    if stage == "star1":
        for n, c, seeds in ((5, 13, range(1, 31)), (5, 9, range(1, 11)),
                            (6, 14, range(1, 31)), (6, 10, range(1, 11))):
            for seed in seeds:
                for kind in ("near", "mid", "far", "k2"):
                    add(n, c, seed, kind, *qbands(n)[kind], 120, 4, True)
    elif stage == "star7":
        for seed in range(1, 11):
            for kind in ("mid", "far"):
                add(7, 15, seed, kind, *qbands(7)[kind], 150, 4, True)
    elif stage == "k1c":
        for n, c in ((7, 15), (8, 16)):
            for seed in range(1, 11):
                cert = min(br for k, br, a, d in free_certificates(n, insts[n, c, seed]["pieces"])
                           if a >= bands(n)["far"][0])
                add(n, c, seed, "mid", *bands(n)["mid"], 300, 4, False, cap=cert)
    elif stage == "dev":
        for (n, c, seed) in sorted(insts):
            for kind in ("near", "mid", "far", "k2"):
                add(n, c, seed, kind, *qbands(n)[kind], 120, 4, True)
    else:
        raise SystemExit(f"unknown stage {stage}")
    return jobs


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--instances", required=True)
    ap.add_argument("--stage", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--cores", type=int, default=8)
    args = ap.parse_args()
    insts = {}
    with open(args.instances) as f:
        for line in f:
            d = json.loads(line)
            insts[d["n"], d["colors"], d["seed"]] = d
    jobs = jobs_for(args.stage, insts)
    done = set()
    if os.path.exists(args.out):
        with open(args.out) as f:
            done = {json.loads(l)["key"] for l in f if l.strip()}
    todo = [j for j in jobs if j["key"] not in done]
    w = todo[0]["workers"] if todo else 1
    procs = max(1, args.cores // w)
    print(f"{args.stage}: {len(todo)} of {len(jobs)} jobs, {procs} at a time", file=sys.stderr)
    t0 = time.time()
    with Pool(procs) as pool, open(args.out, "a") as out:
        for i, rec in enumerate(pool.imap_unordered(solve, todo), 1):
            out.write(json.dumps(rec) + "\n")
            out.flush()
            print(f"  {i}/{len(todo)} {rec['key']} {rec['status']} [{rec['lb']},{rec['ub']}] "
                  f"{rec['wall_s']:.0f}s, total {time.time() - t0:.0f} s", file=sys.stderr)


if __name__ == "__main__":
    main()
