#!/usr/bin/env python3
"""Tests for the solver's arithmetic/comparison helpers, in isolation, before
wiring them into the full DFS. Same run-on-bf.py-and-check approach."""

from bfc import BF
from bf import run
import solver_src as S

passed = failed = 0


def check(label, got, want):
    global passed, failed
    if got == want:
        passed += 1
    else:
        failed += 1
        print(f"  FAIL {label}: got {got} want {want}")


def out_of(bf, stdin=b""):
    return list(run(bf.code(), stdin))


def fresh():
    b = BF()
    # the helpers lean on a common set of scratch regs; declare them.
    b.reg("i", "tmp", "tmp2", "tmp3", "byte")
    return b


# _divmod4
for v in [0, 1, 4, 7, 9, 35, 63]:
    b = fresh(); b.reg("src", "q", "r")
    b.set("src", v)
    S._divmod4(b, "src", "q", "r")
    b.write("q"); b.write("r"); b.write("src")
    check(f"divmod4({v})", out_of(b), [v // 4, v % 4, v])

# _row_from: pid*4 + rmod
for pid, rm in [(0, 0), (1, 2), (8, 3), (5, 1)]:
    b = fresh(); b.reg("pid", "rm", "row")
    b.set("pid", pid); b.set("rm", rm)
    S._row_from(b, "pid", "rm", "row")
    b.write("row")
    check(f"row_from({pid},{rm})", out_of(b), [pid * 4 + rm])

# _xy with SIZE
for v in range(S.N):
    b = fresh(); b.reg("src", "x", "y")
    b.set("src", v)
    S._xy(b, "src", "x", "y")
    b.write("x"); b.write("y")
    check(f"xy({v})", out_of(b), [v % S.SIZE, v // S.SIZE])

# _lt
for a, c in [(0, 1), (1, 1), (2, 5), (5, 2), (0, 0), (35, 36)]:
    b = fresh(); b.reg("a", "c", "d")
    b.set("a", a); b.set("c", c)
    S._lt(b, "a", "c", "d")
    b.write("d"); b.write("a"); b.write("c")
    check(f"lt({a},{c})", out_of(b), [1 if a < c else 0, a, c])

# _align4_up: ((v+3)//4)*4
for v in [0, 1, 4, 5, 8, 33]:
    b = fresh(); b.reg("x")
    b.set("x", v)
    S._align4_up(b, "x")
    b.write("x")
    check(f"align4_up({v})", out_of(b), [((v + 3) // 4) * 4])

print(f"\n{passed} passed, {failed} failed")
raise SystemExit(0 if failed == 0 else 1)
