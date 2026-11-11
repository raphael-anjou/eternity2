"""Independent recheck of every witness in results/raw/{star1,k1c,star7}.jsonl.

Rebuilds tuples, legality, breaks and the four turned-copy distances from the
instance pieces with numpy (no solver code), and checks the recorded UB, the
band (on d* for quotient jobs, on d_0 otherwise), the pin and the breaks cap.

    python scripts/check_fix.py
"""
import json
import os

import numpy as np

R = os.path.join(os.path.dirname(__file__), "..", "results")


def grid(n, pieces, board):
    g = np.zeros((n, n, 4), dtype=int)
    for c, (p, r) in enumerate(board):
        e = pieces[p]
        g[c // n, c % n] = [e[(s - r) % 4] for s in range(4)]
    return g


def turn(g):
    """Whole board a quarter turn clockwise: cells move, each tuple turns by one side."""
    return np.roll(np.rot90(g, k=-1), 1, axis=2)


def main():
    insts = {}
    for line in open(os.path.join(R, "instances.jsonl")):
        d = json.loads(line)
        insts[d["n"], d["colors"], d["seed"]] = d["pieces"]
    fails = checked = 0
    for name in ("star1", "k1c", "star7"):
        path = os.path.join(R, "raw", f"{name}.jsonl")
        if not os.path.exists(path):
            continue
        for line in open(path):
            rec = json.loads(line)
            if rec["board"] is None:
                assert rec["status"] in ("INFEASIBLE", "UNKNOWN"), rec["key"]
                continue
            checked += 1
            n, N = rec["n"], rec["n"] ** 2
            pieces = insts[n, rec["colors"], rec["seed"]]
            board = [(v // 4, v % 4) for v in rec["board"]]
            g = grid(n, pieces, board)
            plant = grid(n, pieces, [(c, 0) for c in range(N)])
            gray = g == 0
            edge = np.zeros_like(gray)
            edge[0, :, 0] = edge[:, -1, 1] = edge[-1, :, 2] = edge[:, 0, 3] = True
            br = int((g[:, :-1, 1] != g[:, 1:, 3]).sum() + (g[:-1, :, 2] != g[1:, :, 0]).sum())
            ds, t = [], plant
            for _ in range(4):
                ds.append(int((g != t).any(axis=2).sum()))
                t = turn(t)
            d = min(ds) if rec["quotient"] else ds[0]
            ok = (sorted(p for p, _ in board) == list(range(N)) and (gray == edge).all()
                  and board[0][0] == 0 and br == rec["ub"] and rec["lo"] <= d <= rec["hi"]
                  and (rec["cap"] is None or br <= rec["cap"]) and ds == rec["witness_dk"])
            if not ok:
                fails += 1
                print("FAIL", rec["key"], br, rec["ub"], ds, rec["witness_dk"])
    print(f"{checked} witnesses rechecked, {fails} failures")


if __name__ == "__main__":
    main()
