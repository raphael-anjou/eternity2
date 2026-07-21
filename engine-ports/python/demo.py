"""Command-line demo of the pure-Python engine. Generates a small solvable
puzzle and solves it, then runs a short bounded probe of the official 16x16 set.

    python3 demo.py [size] [seed] [colors]

``colors`` defaults to 5: fewer colours than edges means several pieces share an
edge colour, so the search has to *backtrack* to find the fit (with the maximum
colour count every placement is forced and the solve is trivial — boring).

The real correctness check is ``python3 spec.py`` (parity vs the Rust engine).
"""

import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

import eternity2 as E  # noqa: E402


def main() -> None:
    size = int(sys.argv[1]) if len(sys.argv) > 1 else 6
    seed = int(sys.argv[2]) if len(sys.argv) > 2 else 42
    colors = int(sys.argv[3]) if len(sys.argv) > 3 else 5

    print(f"Eternity II — pure Python engine (Python {sys.version_info.major}.{sys.version_info.minor})")
    print("-" * 52)

    # 1. Generate and fully solve a small puzzle. Fewer colours -> real backtracking.
    colors = min(colors, E.max_colors(size))
    p = E.generate(size, colors, seed)
    path = E.build_path("row-major", size, size, 0)
    assert path is not None
    sv = E.Solver(p, path)
    while True:
        r = sv.step(2_000_000)
        if r.status != E.Status.RUNNING:
            break
    print(f"generated {size}x{size} (c={colors}, seed={seed}):")
    print(
        f"  status={r.status}  placed={r.placed}/{size * size}  "
        f"score={E.score_board(p, sv.board_cells())}/{E.max_score(size, size)}"
    )
    print(f"  nodes={r.nodes}  attempts={r.attempts}  backtracks={r.backtracks}")

    # 2. Official set: short bounded probe (it does not finish — nothing does).
    with open(os.path.join(HERE, "official_eternity2.csv")) as f:
        off = E.official_puzzle(f.read())
    opath = E.build_path("row-major", 16, 16, 0)
    assert opath is not None
    osv = E.Solver(off, opath)
    orep = osv.step(50_000)
    print("official Eternity II (16x16, 50,000-step probe):")
    print(
        f"  status={orep.status}  placed={orep.placed}  "
        f"best_placed={orep.best_placed}  attempts={orep.attempts}"
    )
    print("-" * 52)
    print("OK")


if __name__ == "__main__":
    main()
