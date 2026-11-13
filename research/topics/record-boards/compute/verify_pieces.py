"""Verify Eternity II boards given as official piece numbers and rotations.

Usage: python3 -I verify_pieces.py BOARD.json [BOARD.json ...] > out.json

Each BOARD.json holds "pieces": 256 pairs [piece, rotation], row-major from the
top-left cell, where piece is the 1-based line number in
engine/data/official_eternity2.csv (after the size line) and rotation is the
number of clockwise quarter-turns, the engine's convention. Optional
"board_edges" (e2.bucas.name letters, URDL per cell, 'a' = grey) is checked
against the edges recomputed from the pieces, so a viewer link built from it
shows the same board.

Checks, all from the official CSV alone (standard library only):
  - 256 placements, each official piece used exactly once;
  - the five official clue pieces at their official cells and rotations;
  - grey sides only on the outer frame;
  - matched internal edges out of 480.
Deterministic: the same input gives byte-identical output.
"""

import json
import os
import sys

N = 16
CSV = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                   "..", "..", "..", "..", "engine", "data", "official_eternity2.csv")


def load_official(csv_path):
    lines = [l.strip() for l in open(csv_path) if l.strip()]
    assert int(lines[0]) == N
    pieces, clues = [], []
    for i, line in enumerate(lines[1:]):
        f = line.split(",")
        sides = tuple(0 if int(s, 2) == 65535 else int(s, 2) for s in f[:4])  # URDL
        pieces.append(sides)
        x, y, r = int(f[4]), int(f[5]), int(f[6])
        if x or y or r:
            clues.append({"piece": i + 1, "cell": y * N + x, "rotation": r})
    assert len(pieces) == N * N and len(clues) == 5
    return pieces, clues


def rotated(sides, r):
    """Clockwise by r quarter-turns: the side that faced up now faces right."""
    return tuple(sides[(i - r) % 4] for i in range(4))


def verify(board, official, clues):
    placed = board["pieces"]
    out = {"id": board.get("id"), "claimedScore": board.get("claimedScore")}
    ok_shape = len(placed) == N * N and all(1 <= p <= N * N and 0 <= r <= 3 for p, r in placed)
    out["cells"] = len(placed)
    if not ok_shape:
        out["valid"] = False
        return out
    ids = [p for p, _ in placed]
    out["distinctPieces"] = len(set(ids))
    out["everyPieceOnce"] = sorted(ids) == list(range(1, N * N + 1))
    cells = [rotated(official[p - 1], r) for p, r in placed]

    honoured = []
    for c in clues:
        p, r = placed[c["cell"]]
        honoured.append({"piece": c["piece"], "cell": c["cell"], "rotation": c["rotation"],
                         "onBoard": [p, r], "ok": p == c["piece"] and r == c["rotation"]})
    out["cluesHonoured"] = sum(h["ok"] for h in honoured)
    out["clues"] = honoured

    off_rim = 0
    for y in range(N):
        for x in range(N):
            u, rt, d, l = cells[y * N + x]
            want = ((u == 0) == (y == 0) and (d == 0) == (y == N - 1)
                    and (l == 0) == (x == 0) and (rt == 0) == (x == N - 1))
            off_rim += not want
    out["cellsWithGreyOffFrame"] = off_rim

    matched = 0
    for y in range(N):
        for x in range(N):
            i = y * N + x
            if x + 1 < N and cells[i][1] == cells[i + 1][3]:
                matched += 1
            if y + 1 < N and cells[i][2] == cells[i + N][0]:
                matched += 1
    out["internalEdges"] = 2 * N * (N - 1)
    out["matchedEdges"] = matched

    if "board_edges" in board:
        letters = "".join(chr(97 + s) for c in cells for s in c)
        out["boardEdgesAgree"] = letters == board["board_edges"]

    out["starterOnlyValid"] = out["everyPieceOnce"] and honoured_starter(honoured)
    out["strictFiveClueValid"] = out["everyPieceOnce"] and out["cluesHonoured"] == 5
    out["valid"] = (out["everyPieceOnce"] and out["matchedEdges"] == board.get("claimedScore")
                    and out.get("boardEdgesAgree", True))
    return out


def honoured_starter(honoured):
    # The starter clue is piece 139 at cell 135 (I8); it is the only mandatory one.
    return any(h["piece"] == 139 and h["ok"] for h in honoured)


def main(paths):
    official, clues = load_official(CSV)
    results = [verify(json.load(open(p)), official, clues) for p in paths]
    json.dump({"maxScore": 2 * N * (N - 1),
               "note": "Boards verified from official piece numbers and rotations against engine/data/official_eternity2.csv.",
               "boards": results}, sys.stdout, indent=2)
    sys.stdout.write("\n")
    return 0 if all(r["valid"] for r in results) else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
