"""Exact count of valid frames that agree with each given subset of border
placements, by plain depth-first enumeration of the ring (small boards only).
Independent of the Rust SMC counter; stage 0 compares the two.

  python3 -I framecount.py PUZ SUBSETS  -> one JSON line per subset
"""
import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common  # noqa: E402


def ring(n):
    v = list(range(n))
    v += [r * n + n - 1 for r in range(1, n)]
    v += [(n - 1) * n + c for c in range(n - 2, -1, -1)]
    v += [r * n for r in range(n - 2, 0, -1)]
    return v


def main():
    puz = common.load_puz(sys.argv[1])
    n, pieces = puz["n"], puz["pieces"]
    cells = ring(n)
    opts = {}
    for pos in cells:
        lst = []
        for pid, e in enumerate(pieces):
            for r in range(4):
                f = common.rotate(e, r)
                if all((f[s] == 0) == (common.nbr(n, pos, s) is None) for s in range(4)):
                    lst.append((pid, f))
                    break
        opts[pos] = lst

    def side(a, b):
        return next(s for s in range(4) if common.nbr(n, a, s) == b)

    for sid, sub in common.load_frames(sys.argv[2]):
        fixed = {pos: pid for pos, pid, _ in sub}
        fixed_pieces = set(fixed.values())
        used = set()
        placed = [None] * len(cells)
        cnt = [0]

        def rec(i):
            if i == len(cells):
                f0 = placed[0]
                if f0[side(cells[0], cells[-1])] == placed[-1][side(cells[-1], cells[0])]:
                    cnt[0] += 1
                return
            pos = cells[i]
            for pid, f in opts[pos]:
                if pos in fixed:
                    if pid != fixed[pos]:
                        continue
                elif pid in used or pid in fixed_pieces:
                    continue
                if i > 0 and f[side(pos, cells[i - 1])] != placed[i - 1][side(cells[i - 1], pos)]:
                    continue
                used.add(pid)
                placed[i] = f
                rec(i + 1)
                used.discard(pid)

        rec(0)
        print(json.dumps({"id": sid, "size": len(sub), "count": cnt[0],
                          "log10": math.log10(cnt[0]) if cnt[0] else None}), flush=True)


if __name__ == "__main__":
    main()
