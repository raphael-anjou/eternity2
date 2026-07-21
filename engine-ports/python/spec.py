"""Parity test: validates the pure-Python engine against golden outputs captured
from the Rust engine (``golden.txt``, produced by ``cargo run --bin golden``).

    python3 spec.py

Every assertion compares Python output to a value the Rust engine actually
produced — generated puzzles (RNG parity), the official set, all path
permutations, and full solver runs DOWN TO the exact node / attempt / backtrack
counts. Matching the counts proves the two engines explore the search tree
identically, not merely that they both happen to solve.

This is the same golden.txt the Lua port checks against (see ../engine-lua),
copied alongside for a self-contained run.
"""

import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

import eternity2 as E  # noqa: E402

_passed = 0
_failed = 0


def check(cond: bool, msg: str) -> None:
    global _passed, _failed
    if cond:
        _passed += 1
    else:
        _failed += 1
        print(f"  FAIL: {msg}")


def field(tok: str) -> int:
    """'key=123' -> 123."""
    return int(tok.split("=", 1)[1])


def field_str(tok: str) -> str:
    return tok.split("=", 1)[1]


# Load the official CSV + the golden lines.
with open(os.path.join(HERE, "official_eternity2.csv")) as f:
    OFFICIAL_CSV = f.read()
with open(os.path.join(HERE, "golden.txt")) as f:
    GOLDEN = [ln for ln in f.read().splitlines() if ln.strip()]


# ---------------------------------------------------------------------------
print("== generator parity ==")
for line in GOLDEN:
    t = line.split()
    if t[0] != "GEN":
        continue
    size, colors, seed = int(t[1]), int(t[2]), int(t[3])
    p = E.generate(size, colors, seed)
    got = [f"{e[0]},{e[1]},{e[2]},{e[3]}" for e in p.pieces]
    expect = t[4:]
    check(got == expect, f"generate({size},{colors},{seed}) pieces match")


# ---------------------------------------------------------------------------
print("== official set parity ==")
off = E.official_puzzle(OFFICIAL_CSV)
for line in GOLDEN:
    t = line.split()
    if t[0] == "OFF":
        check(len(off.pieces) == field(t[1]), "official piece count")
        check(off.num_colors == field(t[2]), "official color count")
        check(len(off.hints) == field(t[3]), "official hint count")
hi = 0
for line in GOLDEN:
    t = line.split()
    if t[0] == "OFFHINT":
        h = off.hints[hi] if hi < len(off.hints) else None
        check(
            h is not None
            and h.pos == int(t[1])
            and h.piece == int(t[2])
            and h.rot == int(t[3]),
            f"official hint {hi + 1}",
        )
        hi += 1


# ---------------------------------------------------------------------------
print("== path parity ==")
for line in GOLDEN:
    t = line.split()
    if t[0] != "PATH":
        continue
    kind, w, h = t[1], int(t[2]), int(t[3])
    path = E.build_path(kind, w, h, 1)
    expect = [int(x) for x in t[4:]]
    check(path == expect, f"path {kind} {w}x{h}")


# ---------------------------------------------------------------------------
print("== solver parity (generated 4x4 seed 11, all paths) ==")
for line in GOLDEN:
    t = line.split()
    if t[0] != "SOLVE":
        continue
    kind = t[1]
    e_status = field_str(t[2])
    e_placed, e_score = field(t[3]), field(t[4])
    e_nodes, e_attempts, e_backtracks = field(t[5]), field(t[6]), field(t[7])

    p = E.generate(4, 4, 11)
    path = E.build_path(kind, 4, 4, 0)
    assert path is not None
    sv = E.Solver(p, path, use_hints=True, shuffle_pieces=False, seed=0)
    while True:
        r = sv.step(5_000_000)
        if r.status != E.Status.RUNNING:
            break
    sc = E.score_board(p, sv.board_cells())
    check(r.status == e_status, f"solve {kind} status ({r.status} vs {e_status})")
    check(r.placed == e_placed, f"solve {kind} placed")
    check(sc == e_score, f"solve {kind} score")
    check(r.nodes == e_nodes, f"solve {kind} nodes ({r.nodes} vs {e_nodes})")
    check(r.attempts == e_attempts, f"solve {kind} attempts ({r.attempts} vs {e_attempts})")
    check(r.backtracks == e_backtracks, f"solve {kind} backtracks")


# ---------------------------------------------------------------------------
print("== official partial-run parity (fixed step budget) ==")
for line in GOLDEN:
    t = line.split()
    if t[0] != "OFFICIALRUN":
        continue
    budget = field(t[1])
    e_status = field_str(t[2])
    e_placed, e_best = field(t[3]), field(t[4])
    e_nodes, e_attempts, e_backtracks = field(t[5]), field(t[6]), field(t[7])

    p = E.official_puzzle(OFFICIAL_CSV)
    path = E.build_path("row-major", 16, 16, 0)
    assert path is not None
    sv = E.Solver(p, path, use_hints=True, shuffle_pieces=False, seed=0)
    r = sv.step(budget)
    check(r.status == e_status, "official status")
    check(r.placed == e_placed, f"official placed ({r.placed} vs {e_placed})")
    check(r.best_placed == e_best, f"official best ({r.best_placed} vs {e_best})")
    check(r.nodes == e_nodes, f"official nodes ({r.nodes} vs {e_nodes})")
    check(r.attempts == e_attempts, f"official attempts ({r.attempts} vs {e_attempts})")
    check(r.backtracks == e_backtracks, "official backtracks")


# ---------------------------------------------------------------------------
print(f"\n{_passed} passed, {_failed} failed")
sys.exit(0 if _failed == 0 else 1)
