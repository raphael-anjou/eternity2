# /// script
# requires-python = ">=3.11"
# dependencies = ["ortools>=9.10"]
# ///
"""n13-encoding-cliff, exact arm: the structured CP-SAT encoding.

    uv run cpsat_cliff.py instances/*.json --time-limit 300 --workers 8

Reads instances exported by `cargo run --release -- export` and asks the
decision question "does a full solve exist?" with the encoding the finding is
about:

  * one integer variable per cell over piece ids, under AddAllDifferent
    (the global matching propagator the greedy families lack);
  * a piece-and-rotation index channelled through AddElement side tables into
    per-side colour variables (edge colours become first-class propagation
    variables, where a MIP's sum-to-one rows relax fractionally for free);
  * hard adjacency, hard rim (colour 0), and the five clues fixed.

Every returned board is re-verified independently of the solver's status
line: all piece ids distinct, all clues obeyed, matched-edge count recomputed
from raw edges and compared to the target 2N(N-1). One JSON line per instance
on stdout; a human table on stderr.

Rotation semantics mirror the kit exactly (e2-core `rotated`):
rotated(e, r)[side] == e[(side - r) % 4], sides URDL = 0,1,2,3.
"""

import argparse
import json
import sys
import time

from ortools.sat.python import cp_model

U, R, D, L = 0, 1, 2, 3


def rotated(edges, r):
    return [edges[(s - r) % 4] for s in range(4)]


def solve_instance(doc, time_limit, workers):
    n = doc["size"]
    cells = n * n
    pieces = doc["pieces"]
    npieces = len(pieces)
    assert npieces == cells, f"{npieces} pieces for {cells} cells"
    max_color = max(max(p) for p in pieces)

    model = cp_model.CpModel()

    piece = [model.NewIntVar(0, npieces - 1, f"p{c}") for c in range(cells)]
    rot = [model.NewIntVar(0, 3, f"r{c}") for c in range(cells)]
    idx = [model.NewIntVar(0, 4 * npieces - 1, f"i{c}") for c in range(cells)]
    for c in range(cells):
        model.Add(idx[c] == 4 * piece[c] + rot[c])

    model.AddAllDifferent(piece)

    # side_table[s][4*p + r] = colour of piece p at rotation r on side s.
    side_table = [[rotated(pieces[i // 4], i % 4)[s] for i in range(4 * npieces)]
                  for s in range(4)]

    side = [[model.NewIntVar(0, max_color, f"s{c}_{s}") for s in range(4)]
            for c in range(cells)]
    for c in range(cells):
        for s in range(4):
            model.AddElement(idx[c], side_table[s], side[c][s])

    for c in range(cells):
        x, y = c % n, c // n
        if y == 0:
            model.Add(side[c][U] == 0)
        if x == n - 1:
            model.Add(side[c][R] == 0)
        if y == n - 1:
            model.Add(side[c][D] == 0)
        if x == 0:
            model.Add(side[c][L] == 0)
        if x + 1 < n:
            model.Add(side[c][R] == side[c + 1][L])
            model.Add(side[c][R] != 0)
        if y + 1 < n:
            model.Add(side[c][D] == side[c + n][U])
            model.Add(side[c][D] != 0)

    for h in doc["hints"]:
        model.Add(piece[h["pos"]] == h["piece"])
        model.Add(rot[h["pos"]] == h["rot"])

    solver = cp_model.CpSolver()
    solver.parameters.max_time_in_seconds = time_limit
    solver.parameters.num_workers = workers

    t0 = time.monotonic()
    status = solver.Solve(model)
    secs = time.monotonic() - t0
    status_name = solver.StatusName(status)

    full, verified, score = False, False, None
    if status in (cp_model.OPTIMAL, cp_model.FEASIBLE):
        assignment = [(solver.Value(piece[c]), solver.Value(rot[c])) for c in range(cells)]
        score, verified = verify(doc, assignment)
        full = score == doc["target"]
    return {
        "arm": "cpsat",
        "name": doc["name"],
        "n": n,
        "colors": doc["num_colors"],
        "target": doc["target"],
        "status": status_name,
        "secs": round(secs, 3),
        "score": score,
        "full_solve": full,
        "verified": verified,
    }


def verify(doc, assignment):
    """Never trust a solver's self-report. Recompute everything from raw edges."""
    n = doc["size"]
    pieces = doc["pieces"]
    ids = [p for p, _ in assignment]
    if len(set(ids)) != len(ids):
        return None, False
    for h in doc["hints"]:
        if assignment[h["pos"]] != (h["piece"], h["rot"]):
            return None, False
    grid = [rotated(pieces[p], r) for p, r in assignment]
    score = 0
    for c in range(n * n):
        x, y = c % n, c // n
        if x + 1 < n and grid[c][R] == grid[c + 1][L]:
            score += 1
        if y + 1 < n and grid[c][D] == grid[c + n][U]:
            score += 1
        if (y == 0 and grid[c][U] != 0) or (y == n - 1 and grid[c][D] != 0):
            return score, False
        if (x == 0 and grid[c][L] != 0) or (x == n - 1 and grid[c][R] != 0):
            return score, False
    return score, True


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("instances", nargs="+", help="instance JSONs from the export mode")
    ap.add_argument("--time-limit", type=float, default=300.0)
    ap.add_argument("--workers", type=int, default=8)
    args = ap.parse_args()

    print(f"{'instance':<28} {'status':<10} {'secs':>8} {'score':>6} {'target':>6} {'ok':>4}",
          file=sys.stderr)
    for path in args.instances:
        with open(path) as f:
            doc = json.load(f)
        row = solve_instance(doc, args.time_limit, args.workers)
        print(f"{row['name']:<28} {row['status']:<10} {row['secs']:>8.3f} "
              f"{str(row['score']):>6} {row['target']:>6} "
              f"{'yes' if row['full_solve'] and row['verified'] else 'NO':>4}",
              file=sys.stderr)
        print(json.dumps(row), flush=True)


if __name__ == "__main__":
    main()
