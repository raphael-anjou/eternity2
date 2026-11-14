"""Independent brute force for stage 0 (ii): enumerate EVERY solution of a
small board (row-major depth-first, all pieces, all rotations, no pruning
beyond edge matching) and write the set of frames that occur in some
solution. Shares no code with the Rust engine or the SAT encoding.

  python3 -I brute.py PUZ OUT.json
"""
import json
import sys
import time

import os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common  # noqa: E402


def main():
    puz = common.load_puz(sys.argv[1])
    n, pieces = puz["n"], puz["pieces"]
    fixed = {pos: (pid, r) for pos, pid, r in puz["fixed"]}
    rots = [[common.rotate(e, r) for r in range(4)] for e in pieces]
    board = [None] * (n * n)
    used = [False] * len(pieces)
    border = [p for p in range(n * n) if common.is_border(n, p)]
    frames = set()
    count = [0, 0]

    def fits(pos, f):
        r, c = divmod(pos, n)
        if (r == 0) != (f[0] == 0) or (c == n - 1) != (f[1] == 0):
            return False
        if (r == n - 1) != (f[2] == 0) or (c == 0) != (f[3] == 0):
            return False
        if r > 0 and rots[board[pos - n][0]][board[pos - n][1]][2] != f[0]:
            return False
        if c > 0 and rots[board[pos - 1][0]][board[pos - 1][1]][1] != f[3]:
            return False
        return True

    def rec(pos):
        if pos == n * n:
            count[0] += 1
            frames.add(tuple((p,) + board[p] for p in border))
            return
        opts = [fixed[pos]] if pos in fixed else [(pid, r) for pid in range(len(pieces)) for r in range(4)]
        seen = set()
        for pid, r in opts:
            if used[pid] and pos not in fixed:
                continue
            f = rots[pid][r]
            if (pid, f) in seen or not fits(pos, f):
                continue
            seen.add((pid, f))
            count[1] += 1
            board[pos] = (pid, r)
            used[pid] = True
            rec(pos + 1)
            used[pid] = pos in fixed
            board[pos] = None

    t0 = time.time()
    for pos, (pid, r) in fixed.items():
        used[pid] = True
    rec(0)
    json.dump({"puz": sys.argv[1], "solutions": count[0], "nodes": count[1],
               "secs": round(time.time() - t0, 2),
               "frames": sorted(list(map(list, f)) for f in frames)}, open(sys.argv[2], "w"))
    print(sys.argv[1], "solutions", count[0], "distinct frames", len(frames), "secs", round(time.time() - t0, 1))


if __name__ == "__main__":
    main()
