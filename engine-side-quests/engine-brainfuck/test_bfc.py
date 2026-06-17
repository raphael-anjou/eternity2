#!/usr/bin/env python3
"""Unit tests for the bfc primitives. Each compiles a tiny program, runs it on
the bf.py interpreter, and checks the result (output bytes or final tape). If
these pass, the solver built on top of them rests on solid ground."""

from bfc import BF
from bf import run, run_tape

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


def cell(bf, name, stdin=b""):
    _, tape, _ = run_tape(bf.code(), stdin)
    return tape[bf.off[name]]


# ---- scalars ----
b = BF(); b.reg("x"); b.set("x", 42); b.write("x")
check("set", out_of(b), [42])

b = BF(); b.reg("x"); b.set("x", 10); b.inc("x", 5); b.dec("x", 3); b.write("x")
check("inc/dec", out_of(b), [12])

b = BF(); b.reg("x", "y"); b.set("x", 7); b.move("x", "y"); b.write("y"); b.write("x")
check("move", out_of(b), [7, 0])

b = BF(); b.reg("x", "y", "t"); b.set("x", 9); b.copy("x", "y", "t"); b.write("y"); b.write("x")
check("copy", out_of(b), [9, 9])

b = BF(); b.reg("a", "s", "t"); b.set("a", 20); b.set("s", 22); b.addto("a", "s", "t"); b.write("a"); b.write("s")
check("addto", out_of(b), [42, 22])

b = BF(); b.reg("a", "s", "t"); b.set("a", 50); b.set("s", 8); b.subto("a", "s", "t"); b.write("a"); b.write("s")
check("subto", out_of(b), [42, 8])

b = BF(); b.reg("x"); b.read("x"); b.write("x")
check("read", out_of(b, bytes([99])), [99])

# ---- logic ----
for av, bv, want in [(5, 5, 1), (5, 6, 0), (0, 0, 1), (3, 0, 0)]:
    b = BF(); b.reg("a", "bb", "d", "t"); b.set("a", av); b.set("bb", bv)
    b.eq("a", "bb", "d", "t"); b.write("d")
    check(f"eq({av},{bv})", out_of(b), [want])

for v, want in [(0, 0), (1, 1), (7, 1)]:
    b = BF(); b.reg("x", "d"); b.set("x", v); b.boolify("x", "d"); b.write("d")
    check(f"boolify({v})", out_of(b), [want])

# ifnz_copy preserves the flag and gates the body
b = BF(); b.reg("flag", "out")
b.set("flag", 3); b.set("out", 0)
with b.ifnz_copy("flag"):
    b.set("out", 9)
b.write("out"); b.write("flag")
check("ifnz_copy true", out_of(b), [9, 3])

b = BF(); b.reg("flag", "out")
b.set("flag", 0); b.set("out", 5)
with b.ifnz_copy("flag"):
    b.set("out", 9)
b.write("out")
check("ifnz_copy false", out_of(b), [5])

# while_nz countdown
b = BF(); b.reg("n", "acc")
b.set("n", 4); b.set("acc", 0)
with b.while_nz("n"):
    b.inc("acc", 2)
    b.dec("n")
b.write("acc")
check("while_nz", out_of(b), [8])

# ---- arrays (runtime index) ----
# Build arr = [10,20,30,40] via const init, then aload every index.
def make_arr():
    b = BF(); b.array("A", 4); b.reg("v", "i", "out", "t")
    for k, val in enumerate([10, 20, 30, 40]):
        b.set("v", val); b.ainit_const("A", k, "v", "t")
    return b

for idx, want in [(0, 10), (1, 20), (2, 30), (3, 40)]:
    b = make_arr()
    b.set("i", idx); b.aload("A", "i", "out")
    b.write("out"); b.write("i")  # i must be preserved
    check(f"aload[{idx}]", out_of(b), [want, idx])

# astore then aload back
for idx in range(4):
    b = make_arr()
    b.set("i", idx); b.set("v", 77); b.astore("A", "i", "v")
    b.set("out", 0); b.aload("A", "i", "out")
    b.write("out"); b.write("v")  # value preserved
    check(f"astore[{idx}]", out_of(b), [77, 77])

# astore must not disturb other slots
b = make_arr()
b.set("i", 2); b.set("v", 99); b.astore("A", "i", "v")
b.set("i", 0); b.aload("A", "i", "out"); b.write("out")
b.set("i", 3); b.aload("A", "i", "out"); b.write("out")
check("astore isolation", out_of(b), [10, 40])

print(f"\n{passed} passed, {failed} failed")
raise SystemExit(0 if failed == 0 else 1)
