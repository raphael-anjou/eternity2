"""R2: CDCL refuter (Glucose 4.2.1 through PySAT; CaDiCaL in PySAT has no interruptible solve) on a direct encoding, with
the 60 border placements of a frame passed as assumptions, so that the
solver's failed-assumption set is a core over border placements.

Encoding (one CNF per puzzle, the frame is never in the CNF):
  x[c,p,r]  piece p at cell c with rotation r. Border cells: border pieces in
            their forced orientation. Clue cells: the clue placement only, as
            a unit clause. Other interior cells: every free interior piece in
            every distinct rotation.
  col[j,k]  joint j carries colour k (at most one colour per joint).
  x -> col  each placement fixes the colour of its (up to) four joints;
            a placement whose colour cannot appear on the other side is false.
  cell ALO  every cell holds at least one placement.
  piece     z[c,p] = "p at c in some rotation"; each piece in exactly one
            cell (ALO clause over z, AMO by a sequential counter over z).
Per-class piece and cell counts are equal, so cell ALO plus piece AMO forces
exactly one placement per cell in every model.

Usage:
  python3 sat.py refute --puz P --frames F --tcap S --mincap S --percall S --ccap S [--nocore]
  writes one JSON line per frame to stdout.
"""
import json
import sys
import threading
import time

from pysat.solvers import Solver

import os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common  # noqa: E402

SOLVER = "glucose42"


def encode(puz):
    n, pieces = puz["n"], puz["pieces"]
    fixed = {pos: (pid, r) for pos, pid, r in puz["fixed"]}
    clue_pieces = {pid for pid, _ in fixed.values()}
    nv = [0]

    def new():
        nv[0] += 1
        return nv[0]

    zeros = [sum(1 for c in e if c == 0) for e in pieces]
    places = {}  # pos -> list of (pid, r, var, edges)
    for pos in range(n * n):
        lst = []
        if pos in fixed:
            pid, r = fixed[pos]
            lst.append((pid, r, new(), common.rotate(pieces[pid], r)))
        elif common.is_border(n, pos):
            for pid, e in enumerate(pieces):
                if zeros[pid] == 0:
                    continue
                for r in range(4):
                    f = common.rotate(e, r)
                    if all((f[s] == 0) == (common.nbr(n, pos, s) is None) for s in range(4)):
                        lst.append((pid, r, new(), f))
                        break
        else:
            for pid, e in enumerate(pieces):
                if zeros[pid] > 0 or pid in clue_pieces:
                    continue
                seen = set()
                for r in range(4):
                    f = common.rotate(e, r)
                    if f in seen:
                        continue
                    seen.add(f)
                    lst.append((pid, r, new(), f))
        places[pos] = lst
    clauses = []
    # joints
    for pos in range(n * n):
        for s in (1, 2):
            q = common.nbr(n, pos, s)
            if q is None:
                continue
            a = {f[s] for _, _, _, f in places[pos]}
            b = {f[(s + 2) % 4] for _, _, _, f in places[q]}
            dom = sorted(a & b)
            cv = {k: new() for k in dom}
            ks = list(cv.values())
            for i in range(len(ks)):
                for j in range(i + 1, len(ks)):
                    clauses.append([-ks[i], -ks[j]])
            for cell, side in ((pos, s), (q, (s + 2) % 4)):
                for _, _, v, f in places[cell]:
                    if f[side] in cv:
                        clauses.append([-v, cv[f[side]]])
                    else:
                        clauses.append([-v])
    # cells
    for pos in range(n * n):
        clauses.append([v for _, _, v, _ in places[pos]])
        if pos in fixed:
            clauses.append([places[pos][0][2]])
    # pieces
    by_piece = {}
    for pos in range(n * n):
        group = {}
        for pid, _, v, _ in places[pos]:
            group.setdefault(pid, []).append(v)
        for pid, vs in group.items():
            if len(vs) == 1:
                z = vs[0]
            else:
                z = new()
                for v in vs:
                    clauses.append([-v, z])
                clauses.append([-z] + vs)
            by_piece.setdefault(pid, []).append(z)
    for pid, zs in by_piece.items():
        clauses.append(list(zs))
        # sequential counter AMO
        if len(zs) > 1:
            s = [new() for _ in zs[:-1]]
            clauses.append([-zs[0], s[0]])
            for i in range(1, len(zs) - 1):
                clauses.append([-zs[i], s[i]])
                clauses.append([-s[i - 1], s[i]])
                clauses.append([-zs[i], -s[i - 1]])
            clauses.append([-zs[-1], -s[-1]])
    lit = {}
    for pos in range(n * n):
        for pid, r, v, _ in places[pos]:
            lit[(pos, pid, r)] = v
    return clauses, lit, places, nv[0]


def decode(model, places, n):
    true = {v for v in model if v > 0}
    board = []
    for pos in range(n * n):
        on = [(pid, r) for pid, r, v, _ in places[pos] if v in true]
        board.append((pos,) + on[0])
    return board


def timed_solve(s, assumptions, cap):
    """Returns (status True/False/None, seconds, conflicts)."""
    before = (s.accum_stats() or {}).get("conflicts", 0)
    timer = threading.Timer(cap, s.interrupt)
    t0 = time.perf_counter()
    timer.start()
    st = s.solve_limited(assumptions=assumptions, expect_interrupt=True)
    timer.cancel()
    secs = time.perf_counter() - t0
    s.clear_interrupt()
    after = (s.accum_stats() or {}).get("conflicts", 0)
    return st, secs, after - before


def fresh(clauses):
    s = Solver(name=SOLVER)
    s.append_formula(clauses)
    return s


def main():
    args = sys.argv[2:]
    o = {}
    i = 0
    while i < len(args):
        k = args[i].lstrip("-")
        if i + 1 < len(args) and not args[i + 1].startswith("--"):
            o[k] = args[i + 1]
            i += 2
        else:
            o[k] = "1"
            i += 1
    puz = common.load_puz(o["puz"])
    n = puz["n"]
    t0 = time.perf_counter()
    clauses, lit, places, nvars = encode(puz)
    enc_secs = time.perf_counter() - t0
    tcap, mincap = float(o["tcap"]), float(o.get("mincap", 0))
    percall, ccap = float(o.get("percall", 30)), float(o.get("ccap", 0))
    inc = [None]
    for fid, frame in common.load_frames(o["frames"]):
        assum = [lit[(pos, pid, r)] for pos, pid, r in frame]
        back = {lit[(pos, pid, r)]: (pos, pid, r) for pos, pid, r in frame}
        tl = time.perf_counter()
        if "incremental" in o:
            # one solver kept alive across the frames of the file (learnt
            # clauses carry over); cores off
            if inc[0] is None:
                inc[0] = fresh(clauses)
            s = inc[0]
        else:
            s = fresh(clauses)
        load_secs = time.perf_counter() - tl
        st, secs, conf = timed_solve(s, assum, tcap)
        row = {"id": fid, "refuter": "R2-glucose", "vars": nvars, "clauses": len(clauses),
               "encode_secs": round(enc_secs, 3), "load_secs": round(load_secs, 3),
               "verdict": {True: "sat", False: "unsat", None: "capped"}[st],
               "secs": round(secs, 6), "conflicts": conf}
        if st is True:
            board = decode(s.get_model(), places, n)
            row["solution"] = [list(x) for x in board]
            row["check"] = common.check_board(puz, board, frame)
        if st is False and "nocore" not in o and "incremental" not in o:
            core = [l for l in assum if l in set(s.get_core() or [])]
            row["core0"] = sorted(back[l][0] for l in core)
            # deletion-based minimisation under a fixed per-frame budget
            tm = time.perf_counter()
            k = 0
            calls = {"unsat": 0, "sat": 0, "capped": 0}
            undecided = []
            unsat_secs = []
            while k < len(core):
                left = mincap - (time.perf_counter() - tm)
                if left <= 0:
                    undecided.extend(core[k:])
                    break
                trial = core[:k] + core[k + 1:]
                st2, secs2, _ = timed_solve(s, trial, min(percall, left))
                if st2 is False:
                    calls["unsat"] += 1
                    unsat_secs.append(round(secs2, 4))
                    c2 = set(s.get_core() or trial)
                    core = [l for l in trial if l in c2]
                elif st2 is True:
                    calls["sat"] += 1
                    k += 1
                else:
                    calls["capped"] += 1
                    undecided.append(core[k])
                    k += 1
            row["core"] = sorted(back[l][0] for l in core)
            row["core_undecided"] = sorted(back[l][0] for l in undecided if l in core)
            row["min_calls"] = calls
            row["min_unsat_secs"] = unsat_secs
            row["min_secs"] = round(time.perf_counter() - tm, 3)
            # cost of proving the core alone unsatisfiable, fresh solver
            if ccap > 0:
                s2 = fresh(clauses)
                st3, secs3, conf3 = timed_solve(s2, core, ccap)
                row["core_verdict"] = {True: "sat", False: "unsat", None: "capped"}[st3]
                row["core_secs"] = round(secs3, 6)
                row["core_conflicts"] = conf3
                s2.delete()
            row["core_placements"] = [list(back[l]) for l in core]
        row["incremental"] = "incremental" in o
        if "incremental" not in o:
            s.delete()
        print(json.dumps(row), flush=True)


if __name__ == "__main__":
    main()
