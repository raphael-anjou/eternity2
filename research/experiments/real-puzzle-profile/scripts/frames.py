"""Decode the frame (60 border cells) of committed record boards into fixed-cell
files for `rpp enum --fixed`. A board is used only if all 256 cells map to
distinct pieces of the official CSV up to rotation (bucas letter 'a' + c is
colour c). Prints, per board, whether it maps, whether it honours the five
clues, and its matched-edge count, and writes results/m2/frame-<id>.txt.

Run: python3 -I scripts/frames.py
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
EXP = os.path.dirname(HERE)
REPO = os.path.dirname(os.path.dirname(os.path.dirname(EXP)))
CSV = os.path.join(EXP, "..", "single-core-benchmark", "engine", "data", "puzzles", "size_16_official_eternity.csv")
BOARDS = os.path.join(REPO, "research", "topics", "record-boards", "boards.json")
N = 16


def rotate(e, r):
    t, ri, b, l = e
    return [(t, ri, b, l), (l, t, ri, b), (b, l, t, ri), (ri, b, l, t)][r % 4]


def load_csv():
    lines = [l.strip() for l in open(CSV) if l.strip()]
    pieces, hints = [], []
    for i, l in enumerate(lines[1:]):
        c = l.split(",")
        pieces.append(tuple(0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4]))
        x, y, r = int(c[4]), int(c[5]), int(c[6])
        if x or y or r:
            hints.append((y * N + x, i, r))
    return pieces, hints


def main():
    pieces, hints = load_csv()
    lookup = {}
    for pid, e in enumerate(pieces):
        for r in range(4):
            lookup.setdefault(rotate(e, r), []).append((pid, r))
    boards = json.load(open(BOARDS))["boards"]
    os.makedirs(os.path.join(EXP, "results", "m2"), exist_ok=True)
    for b in boards[:2]:  # the two preregistered boards
        p = dict(kv.split("=", 1) for kv in b["params"].split("&"))
        s = p["board_edges"]
        cells = [tuple(ord(ch) - ord("a") for ch in s[4 * i: 4 * i + 4]) for i in range(N * N)]
        place, ok = [], True
        for pos, e in enumerate(cells):
            m = lookup.get(e)
            if not m:
                ok = False
                break
            place.append(m[0])
        distinct = ok and len({pid for pid, _ in place}) == N * N
        matched = 0
        for pos, e in enumerate(cells):
            r_, c_ = divmod(pos, N)
            if c_ + 1 < N and e[1] == cells[pos + 1][3]:
                matched += 1
            if r_ + 1 < N and e[2] == cells[pos + N][0]:
                matched += 1
        clues = distinct and all(place[pos] == (pid, r) for pos, pid, r in hints)
        print("%s: maps=%s distinct=%s clues_honoured=%s matched_edges=%d" % (b["id"], ok, distinct, clues, matched))
        if not distinct:
            continue
        out = os.path.join(EXP, "results", "m2", "frame-%s.txt" % b["id"])
        with open(out, "w") as fh:
            fh.write("# frame of %s (stand-in, not the true frame): pos piece rot\n" % b["id"])
            for pos in range(N * N):
                r_, c_ = divmod(pos, N)
                if r_ in (0, N - 1) or c_ in (0, N - 1):
                    fh.write("%d %d %d\n" % (pos, place[pos][0], place[pos][1]))
        inner_joints = sum(
            1 for pos in range(N * N)
            for (q, s, t) in ((pos + 1, 1, 3), (pos + N, 2, 0))
            if (pos % N + (s == 1)) < N and q < N * N
            and ((pos // N) in (0, N - 1) or pos % N in (0, N - 1)) != ((q // N) in (0, N - 1) or q % N in (0, N - 1))
            and cells[pos][s] != cells[q][t]
        )
        print("   wrote %s; frame-to-interior joints broken on the record board: %d" % (os.path.relpath(out, EXP), inner_joints))


main()
