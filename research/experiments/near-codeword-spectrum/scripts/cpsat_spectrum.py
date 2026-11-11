"""CP-SAT solves for the near-codeword spectrum (see PREREG.md).

    python scripts/cpsat_spectrum.py --instances results/instances.jsonl \
        --stage stage1 --out results/raw/stage1.jsonl

Each job minimises breaks subject to lo <= a <= hi. Results are appended to
the output jsonl; jobs already present are skipped (resumable).
"""
import argparse
import json
import math
import os
import sys
import time
from multiprocessing import Pool

from ortools.sat.python import cp_model

GRAY = 0


def rot(e, r):
    r &= 3
    return (e[(4 - r) & 3], e[(5 - r) & 3], e[(6 - r) & 3], e[(7 - r) & 3])


def outward(n, c):
    x, y = c % n, c // n
    return (y == 0, x == n - 1, y == n - 1, x == 0)


def legal(n, c, e):
    o = outward(n, c)
    return all(o[s] == (e[s] == GRAY) for s in range(4))


def joints(n):
    js = []
    for y in range(n):
        for x in range(n - 1):
            c = y * n + x
            js.append((c, 1, c + 1, 3))
    for y in range(n - 1):
        for x in range(n):
            c = y * n + x
            js.append((c, 2, c + n, 0))
    return js


def bands(n):
    N = n * n
    q = math.ceil(N / 4)
    t = (3 * N) // 4
    return {
        "near": (1, q - 1),
        "mid": (q, t),
        "far": (t + 1, N - 1),
        "k2": (math.ceil(N / 3), t),
    }


def placements(n, pieces):
    """cell -> list of (piece, rot, tuple), gray-legal, duplicate rotations merged."""
    out = []
    for c in range(n * n):
        opts = []
        for p, e in enumerate(pieces):
            seen = set()
            for r in range(4):
                t = rot(e, r)
                if t in seen or not legal(n, c, t):
                    continue
                seen.add(t)
                opts.append((p, r, t))
        out.append(opts)
    return out


def check_board(n, pieces, board):
    """board: list of (piece, rot). Returns (breaks, a) or raises."""
    N = n * n
    assert sorted(p for p, _ in board) == list(range(N)), "not a permutation"
    tup = [rot(pieces[p], r) for p, r in board]
    for c in range(N):
        assert legal(n, c, tup[c]), f"illegal at {c}"
    assert board[0][0] == 0, "pin violated"
    br = sum(1 for a, sa, b, sb in joints(n) if tup[a][sa] != tup[b][sb])
    a = sum(1 for c in range(N) if tup[c] != tuple(pieces[c]))
    return br, a


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
    same = [x[c, p, r] for c in range(N) for p, r, t in opts[c] if t == pieces[c]]
    dist = N - sum(same)
    m.Add(dist >= lo)
    m.Add(dist <= hi)
    m.Minimize(sum(breaks_terms))
    s = cp_model.CpSolver()
    s.parameters.max_time_in_seconds = job["time"]
    s.parameters.num_workers = job["workers"]
    s.parameters.random_seed = 1000 * n + job["seed"]
    t0 = time.time()
    st = s.Solve(m)
    wall = time.time() - t0
    rec = {k: job[k] for k in ("key", "stage", "n", "colors", "seed", "kind", "lo", "hi", "time", "workers")}
    rec["status"] = s.StatusName(st)
    rec["wall_s"] = round(wall, 3)
    if st in (cp_model.OPTIMAL, cp_model.FEASIBLE):
        board = []
        for c in range(N):
            for p, r, t in opts[c]:
                if s.Value(x[c, p, r]):
                    board.append((p, r))
        br, a = check_board(n, pieces, board)
        ub = int(round(s.ObjectiveValue()))
        if br != ub or not (lo <= a <= hi):
            raise RuntimeError(f"witness mismatch {job['key']}: breaks {br} vs {ub}, a {a}")
        rec.update(ub=ub, lb=int(math.ceil(s.BestObjectiveBound() - 1e-6)), witness_a=a,
                   board=[p * 4 + r for p, r in board])
        if st == cp_model.OPTIMAL:
            rec["lb"] = ub
    else:
        rec.update(ub=None, lb=None, witness_a=None, board=None)
    return rec


PRIMARY = {5: 13, 6: 14, 7: 15, 8: 16, 10: 18}
CONTRAST = {5: 9, 6: 10, 7: 11, 8: 12}


def jobs_for(stage, insts):
    jobs = []

    def add(n, c, seed, kind, lo, hi, t, w):
        inst = insts[n, c, seed]
        jobs.append(dict(key=f"{n}x{n}/C{c}/s{seed}/{kind}", stage=stage, n=n, colors=c, seed=seed,
                         kind=kind, lo=lo, hi=hi, time=t, workers=w, pieces=inst["pieces"]))

    if stage == "stage1":
        for n in (5, 6):
            b = bands(n)
            for seed in range(1, 31):
                for kind in ("near", "mid", "far", "k2"):
                    add(n, PRIMARY[n], seed, kind, *b[kind], 120, 4)
            for seed in range(1, 11):
                for kind in ("near", "mid", "far", "k2"):
                    add(n, CONTRAST[n], seed, kind, *b[kind], 120, 4)
    elif stage == "curve":
        # PREREG_AMENDMENTS.md A2: curve on seeds 1..10 only.
        for n in (5, 6):
            for seed in range(1, 11):
                for a in range(1, n * n):
                    add(n, PRIMARY[n], seed, f"a{a}", a, a, 30, 2)
    elif stage == "stage2":
        for n in (7, 8):
            b = bands(n)
            for seed in range(1, 11):
                add(n, PRIMARY[n], seed, "near", *b["near"], 60, 4)
                add(n, PRIMARY[n], seed, "mid", *b["mid"], 300, 4)
                add(n, PRIMARY[n], seed, "far", *b["far"], 300, 4)
            for seed in range(1, 6):
                add(n, CONTRAST[n], seed, "mid", *b["mid"], 60, 4)
                add(n, CONTRAST[n], seed, "far", *b["far"], 60, 4)
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
            if i % 20 == 0 or i == len(todo):
                print(f"  {i}/{len(todo)} done, {time.time() - t0:.0f} s", file=sys.stderr)


if __name__ == "__main__":
    main()
