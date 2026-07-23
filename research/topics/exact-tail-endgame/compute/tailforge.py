# /// script
# requires-python = ">=3.11"
# dependencies = ["ortools>=9.10"]
# ///
"""exact-tail-endgame: the bottom band as its own exact optimisation.

    uv run tailforge.py --seeds 24 --rows 2,3 --caps 1,2 --out ../results/tailforge.json

Reproduces the TAILFORGE / R3-COMB-SQUEEZE core finding (research vault
vol-228 TAILFORGE.md, vol-234 R3-COMB-SQUEEZE.md): a strong partial's
bottom-band endgame can be solved EXACTLY as a small constraint problem given
a frozen top half, and doing so is (or is not) the binding constraint on the
score.

Pipeline, all on the official 16x16 piece set bundled with the starter kit
(same producer and scoring convention as the published tail-finishability-
frostline topic, for lineage):

  1. Produce a full 16x16 board with a seeded randomized greedy row-major
     filler (the frostline producer, verbatim). Record its own score.
  2. Freeze rows 0..15-R; free the bottom R rows (the "tail band"). Frozen
     canonical hint cells are NEVER freed (strict-5 preservation).
  3. Exactly optimise the tail band with OR-Tools CP-SAT: one border-legal
     (piece, rotation) per free cell over the board's own leftover pool,
     AllDifferent, per-side colour channelled by Element, hard rim, MATCH
     booleans on every tail-touching adjacency, maximise matches; optional
     per-cell INCOMING-break cap (top+left mismatches <= cap), the producer's
     <=1-break/cell regime at cap=1.
  4. Compare: exact-tail full-board score vs the producer's own full-board
     score on the identical frozen top. Positive delta => the producer left
     points in the tail (tail-limited at the margin); zero delta => the
     producer already tail-optimal (tail-tight).

Every returned board is re-verified independently of the solver's status:
piece ids distinct, hints obeyed, matched-edge count recomputed from raw
edges. One JSON object with per-(seed, rows, cap) records on stdout.

Rotation semantics mirror the kit exactly (e2-core `rotated`, URDL quads):
rot(e, r)[side] == e[(side - r) % 4], sides URDL = 0,1,2,3, colour 0 = grey
border. Scoring counts interior matched edges only (rim-excluding), identical
to the kit's score_cells; max on a full 16x16 board is 480.
"""

from __future__ import annotations

import argparse
import json
import random
import sys
import time
from pathlib import Path

from ortools.sat.python import cp_model

N = 16
CELLS = N * N
U, R, D, L = 0, 1, 2, 3
BORDER = 0

KIT_DATA = Path(__file__).resolve().parents[3] / "starter-kit" / "data" / "official.json"


def rot(e, r):
    """Rotate a URDL edge quad clockwise r quarter-turns (e2-core `rotated`)."""
    return tuple(e[(i - r) % 4] for i in range(4))


def load_official():
    doc = json.loads(KIT_DATA.read_text())
    assert doc["width"] == N and doc["height"] == N, "expected the 16x16 official set"
    pieces = [tuple(p) for p in doc["pieces"]]
    hints = [(h["pos"], h["piece"], h["rot"]) for h in doc["hints"]]
    return pieces, hints


# ------------------------------------------------------------- producer -----


def border_legal(e, row, col):
    """Hard border legality: grey (0) exactly on rim-facing edges."""
    up, right, down, left = e
    if (up == 0) != (row == 0):
        return False
    if (down == 0) != (row == N - 1):
        return False
    if (left == 0) != (col == 0):
        return False
    if (right == 0) != (col == N - 1):
        return False
    return True


def greedy_board(pieces, seed, hints):
    """Seeded randomized greedy row-major fill that HONORS the canonical hints.

    The frostline producer, extended to pin the 5 official clue cells first so
    every produced board is strict-5 by construction (the exact-tail model then
    keeps those cells frozen and never freed). At a hint cell the producer is
    forced to the clue (piece, rot); elsewhere it picks the border-legal,
    unused piece maximising incoming (top+left) matches, ties broken by the
    seeded RNG. Returns [(piece_id, rot)] per cell or None on deadlock.
    """
    rng = random.Random(seed)
    used = [False] * len(pieces)
    hint_at = {pos: (pid, r) for pos, pid, r in hints}
    for _, pid, _ in hints:
        used[pid] = True
    board = []                 # (pid, rot, right_colour, down_colour)
    down_color = [0] * N
    for cell in range(CELLS):
        row, col = divmod(cell, N)
        left_color = board[-1][2] if col > 0 else None
        if cell in hint_at:
            pid, r = hint_at[cell]
            e = rot(pieces[pid], r)
            board.append((pid, r, e[R], e[D]))
            down_color[col] = e[D]
            continue
        best = None
        for pid, base in enumerate(pieces):
            if used[pid]:
                continue
            for r in range(4):
                e = rot(base, r)
                if not border_legal(e, row, col):
                    continue
                m = 0
                if row > 0 and e[U] == down_color[col]:
                    m += 1
                if col > 0 and e[L] == left_color:
                    m += 1
                key = (m, rng.random(), pid, r, e)
                if best is None or key > best:
                    best = key
        if best is None:
            return None
        _, _, pid, r, e = best
        used[pid] = True
        board.append((pid, r, e[R], e[D]))
        down_color[col] = e[D]
    return [(pid, r) for pid, r, _rc, _dc in board]


def score_board(pieces, board):
    """Canonical rim-excluding interior matched-edge count (== kit score_cells)."""
    grid = [rot(pieces[pid], r) for pid, r in board]
    s = 0
    for c in range(CELLS):
        x, y = c % N, c // N
        if x + 1 < N and grid[c][R] == grid[c + 1][L]:
            s += 1
        if y + 1 < N and grid[c][D] == grid[c + N][U]:
            s += 1
    return s


def viewer_url(name, board, pieces):
    """eternity2.dev viewer URL, byte-identical to e2_kit::viewer_url."""
    blob = "".join(chr(ord("a") + c) for pid, r in board for c in rot(pieces[pid], r))
    return f"https://eternity2.dev/viewer?puzzle={name}&puzzle_size={N}&board_edges={blob}"


# ------------------------------------------------------- exact tail model ---


def exact_tail(pieces, board, hints, rows, cap):
    """Freeze rows 0..15-rows; exactly optimise the bottom `rows` rows.

    Free cells = the bottom `rows` rows MINUS any canonical hint cell (never
    freed). Free pool = the pieces the producer placed in those free cells.
    Maximise tail-touching matches; optional per-cell incoming-break cap.

    Returns a dict with the exact full-board score, the producer's own score
    on the same top, and CP-SAT status / timing / verification.
    """
    hint_pos = {p for p, _, _ in hints}
    first_free_row = N - rows
    free_cells = [c for c in range(CELLS)
                  if c // N >= first_free_row and c not in hint_pos]
    frozen = {c: board[c] for c in range(CELLS) if c not in free_cells}

    pool = [board[c][0] for c in free_cells]  # the producer's own tail pieces
    m = len(free_cells)
    idx_of_cell = {c: i for i, c in enumerate(free_cells)}

    model = cp_model.CpModel()

    # one pool slot per free cell, AllDifferent, plus rotation.
    slot = [model.NewIntVar(0, m - 1, f"slot{i}") for i in range(m)]
    rotv = [model.NewIntVar(0, 3, f"rot{i}") for i in range(m)]
    model.AddAllDifferent(slot)
    combo = [model.NewIntVar(0, 4 * m - 1, f"combo{i}") for i in range(m)]
    for i in range(m):
        model.Add(combo[i] == 4 * slot[i] + rotv[i])

    # side_table[s][4*k + r] = colour of pool piece k at rotation r, side s.
    side_table = [[rot(pieces[pool[k]], r)[s] for k in range(m) for r in range(4)]
                  for s in range(4)]
    # (index space is 4*k+r; combo = 4*slot+rot uses the same layout.)
    max_color = max((max(pieces[p]) for p in pool), default=0)
    side = [[model.NewIntVar(0, max_color, f"side{i}_{s}") for s in range(4)]
            for i in range(m)]
    for i in range(m):
        for s in range(4):
            model.AddElement(combo[i], side_table[s], side[i][s])

    def side_var(cell, s):
        """URDL colour var at `cell` on side s: a CP var if free, else a const."""
        if cell in idx_of_cell:
            return side[idx_of_cell[cell]][s]
        pid, r = frozen[cell]
        return rot(pieces[pid], r)[s]

    # hard rim on each free cell.
    for cell in free_cells:
        x, y = cell % N, cell // N
        i = idx_of_cell[cell]
        for s, is_rim in ((U, y == 0), (R, x == N - 1), (D, y == N - 1), (L, x == 0)):
            if is_rim:
                model.Add(side[i][s] == BORDER)
            else:
                model.Add(side[i][s] != BORDER)

    # Every interior adjacency counted once, from the lower-indexed endpoint.
    # A match bool is created iff at least one endpoint is free (a frozen-frozen
    # adjacency is a constant already reflected in producer_score and skipped).
    # `incoming[c]` collects the match bools on cell c's own TOP and LEFT edges
    # (its two incoming edges in row-major order) whenever c is free.
    matches = []
    incoming = {c: [] for c in free_cells}

    def match_bool(cell, s_here, nb, s_there):
        a = side_var(cell, s_here)
        b = side_var(nb, s_there)
        if isinstance(a, int) and isinstance(b, int):
            return None  # frozen-frozen: constant, not modelled
        mv = model.NewBoolVar(f"m_{cell}_{nb}")
        model.Add(a == b).OnlyEnforceIf(mv)
        model.Add(a != b).OnlyEnforceIf(mv.Not())
        matches.append(mv)
        return mv

    for cell in range(CELLS):
        x, y = cell % N, cell // N
        if x + 1 < N:  # horizontal edge cell -- (cell+1); it is (cell+1)'s LEFT
            mv = match_bool(cell, R, cell + 1, L)
            if mv is not None and (cell + 1) in incoming:
                incoming[cell + 1].append(mv)
        if y + 1 < N:  # vertical edge cell -- (cell+N); it is (cell+N)'s TOP
            mv = match_bool(cell, D, cell + N, U)
            if mv is not None and (cell + N) in incoming:
                incoming[cell + N].append(mv)

    # Per-cell incoming-break cap (the producer's <=k-break/cell regime).
    # A free cell's incoming edges are its TOP (from cell-N) and LEFT (from
    # cell-1); both always exist (a tail free cell is never on the top or left
    # rim), so n_in == 2 for every free cell and each produced exactly one
    # match bool. breaks = 2 - sum(matched) <= cap.
    if cap is not None:
        for cell in free_cells:
            x, y = cell % N, cell // N
            n_in = (1 if y > 0 else 0) + (1 if x > 0 else 0)
            mvs = incoming[cell]
            if len(mvs) == n_in and n_in > 0:
                model.Add(sum(mvs) >= n_in - cap)

    model.Maximize(sum(matches))

    solver = cp_model.CpSolver()
    solver.parameters.max_time_in_seconds = float(TIME_LIMIT)
    solver.parameters.num_workers = WORKERS

    t0 = time.monotonic()
    status = solver.Solve(model)
    secs = time.monotonic() - t0
    status_name = solver.StatusName(status)

    producer_score = score_board(pieces, board)
    if status in (cp_model.OPTIMAL, cp_model.FEASIBLE):
        new_board = list(board)
        for i, cell in enumerate(free_cells):
            k = solver.Value(slot[i])
            r = solver.Value(rotv[i])
            new_board[cell] = (pool[k], r)
        exact_score = score_board(pieces, new_board)
        verified = verify(pieces, new_board, hints)
        url = viewer_url(f"tailforge_s{SEED_TAG}_r{rows}_cap{cap}", new_board, pieces)
    else:
        exact_score = None
        verified = False
        url = None

    return {
        "rows": rows,
        "cap": cap,
        "free_cells": m,
        "status": status_name,
        "secs": round(secs, 3),
        "producer_score": producer_score,
        "exact_score": exact_score,
        "delta": (exact_score - producer_score) if exact_score is not None else None,
        "verified": verified,
        "url": url,
    }


def verify(pieces, board, hints):
    """Independent check: distinct ids, hints obeyed, valid rim."""
    ids = [pid for pid, _ in board]
    if len(set(ids)) != len(ids):
        return False
    if len(ids) != CELLS:
        return False
    for pos, pid, r in hints:
        if board[pos] != (pid, r):
            return False
    for c in range(CELLS):
        x, y = c % N, c // N
        e = rot(pieces[board[c][0]], board[c][1])
        if not border_legal(e, y, x):
            return False
    return True


# --------------------------------------------------------------- driver -----

TIME_LIMIT = 60.0
WORKERS = 8
SEED_TAG = 0


def main():
    global TIME_LIMIT, WORKERS, SEED_TAG
    ap = argparse.ArgumentParser()
    ap.add_argument("--seeds", type=int, default=24, help="producer seeds 1..S")
    ap.add_argument("--rows", default="2,3", help="tail band heights to free")
    ap.add_argument("--caps", default="1,2", help="incoming-break caps; 'none' = uncapped")
    ap.add_argument("--band-seeds", default="",
                    help="per-band seed cap, e.g. '1:24,2:6' (keeps the wide, "
                         "slow bands to fewer seeds so the run stays in budget). "
                         "Bands not listed use --seeds.")
    ap.add_argument("--time-limit", type=float, default=60.0)
    ap.add_argument("--workers", type=int, default=8)
    ap.add_argument("--out", default="../results/tailforge.json")
    args = ap.parse_args()

    TIME_LIMIT = args.time_limit
    WORKERS = args.workers
    rows_list = [int(x) for x in args.rows.split(",")]
    caps_list = [None if x.strip() == "none" else int(x) for x in args.caps.split(",")]
    band_seed_cap = {}
    for tok in args.band_seeds.split(","):
        if ":" in tok:
            b, s = tok.split(":")
            band_seed_cap[int(b)] = int(s)

    pieces, hints = load_official()

    records = []
    print(f"{'seed':>4} {'prod':>5} {'rows':>4} {'cap':>4} {'free':>4} "
          f"{'exact':>5} {'delta':>5} {'status':>10} {'secs':>7} {'ok':>3}",
          file=sys.stderr)
    for seed in range(1, args.seeds + 1):
        board = greedy_board(pieces, seed, hints)
        if board is None:
            print(f"{seed:>4}  deadlock, skipped", file=sys.stderr)
            continue
        SEED_TAG = seed
        prod = score_board(pieces, board)
        for rows in rows_list:
            if seed > band_seed_cap.get(rows, args.seeds):
                continue
            for cap in caps_list:
                row = exact_tail(pieces, board, hints, rows, cap)
                row["seed"] = seed
                records.append(row)
                print(f"{seed:>4} {prod:>5} {rows:>4} "
                      f"{str(cap):>4} {row['free_cells']:>4} "
                      f"{str(row['exact_score']):>5} {str(row['delta']):>5} "
                      f"{row['status']:>10} {row['secs']:>7.2f} "
                      f"{'yes' if row['verified'] else 'NO':>3}",
                      file=sys.stderr)

    # aggregate: per (rows, cap), delta distribution and how often the tail is
    # the binding constraint (delta > 0).
    agg = {}
    for r in records:
        key = f"rows{r['rows']}_cap{r['cap']}"
        agg.setdefault(key, []).append(r)
    summary = {}
    for key, rs in agg.items():
        deltas = [x["delta"] for x in rs if x["delta"] is not None]
        opt = [x for x in rs if x["status"] == "OPTIMAL"]
        summary[key] = {
            "n": len(rs),
            "n_optimal": len(opt),
            "n_verified": sum(1 for x in rs if x["verified"]),
            "delta_min": min(deltas) if deltas else None,
            "delta_median": sorted(deltas)[len(deltas) // 2] if deltas else None,
            "delta_max": max(deltas) if deltas else None,
            "delta_mean": round(sum(deltas) / len(deltas), 3) if deltas else None,
            "n_tail_limited": sum(1 for d in deltas if d > 0),
            "n_tail_tight": sum(1 for d in deltas if d == 0),
            "secs_median": round(sorted(x["secs"] for x in rs)[len(rs) // 2], 3),
        }

    out = {
        "topic": "exact-tail-endgame",
        "instance": "official-16x16",
        "producer": "greedy-row-major (frostline producer)",
        "hints": "5 canonical, pinned (never freed)",
        "scoring": "rim-excluding interior matched edges, max 480",
        "time_limit_s": TIME_LIMIT,
        "workers": WORKERS,
        "summary": summary,
        "records": records,
    }
    outp = Path(args.out)
    outp.parent.mkdir(parents=True, exist_ok=True)
    outp.write_text(json.dumps(out, indent=2))
    print(f"\nwrote {outp}", file=sys.stderr)


if __name__ == "__main__":
    main()
