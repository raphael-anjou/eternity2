"""Claim B: expected number of solutions of a generated board, by exact enumeration.

python3 -I b_solutions.py
Part 1: bookkeeping of the arrangement count and seam counts, on random (not planted)
        piece sets with independent uniform colours, where the first moment is exact.
Part 2: number of valid frames of generated boards against the naive and balanced counts.
Part 3: number of solutions of generated boards against the naive and balanced first moments.
Part 4: the critical colour count under both formulas, and the 16x16 numbers.
"""
import math, os, random, statistics as st, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rvlib as rv

rng = random.Random(20261112)
t0 = time.time()

print("PART 1. iid ensemble, all four rotations counted, 4x4. E[solutions] = 24 * 8! * 4! * 4^4 / (cf^12 c^12)")
for cf, c, reps in ((2, 3, 4000), (3, 2, 4000), (2, 2, 1500)):
    boards = [("b%d" % i, 4, rv.iid_board(4, c, cf, rng)) for i in range(reps)]
    res = rv.run_enum("full", 10**9, boards, allrot=True)
    s = [r["solutions"] for r in res]
    m = st.mean(s)
    se = st.pstdev(s) / math.sqrt(len(s))
    print("  cf=%d c=%d boards=%d: measured mean %.3f (se %.3f), formula %.3f" % (cf, c, reps, m, se, math.exp(rv.lnS_naive(4, c, cf))))

print()
print("PART 2. valid frames (ring seams only) of generated boards, 5 frame colours")
print("  n  boards  measured mean (se)  naive 24(4w)!/5^(4(n-1))  balanced 24(4w)! prod f_k!/(4(n-1))!")
for n, reps in ((5, 200), (6, 200), (7, 12)):
    boards = [("b%d" % i, n, rv.planted_board(n, n + 1, 5, rng)[0]) for i in range(reps)]
    res = rv.run_enum("ring", 0, boards)
    s = [r["frames"] for r in res]
    print("  %d  %d  %.4g (%.2g)  %.4g  %.4g" % (n, reps, st.mean(s), st.pstdev(s) / math.sqrt(len(s)), math.exp(rv.lnF_naive(n)), math.exp(rv.lnF_balanced(n))))
for ladder, n in (("primary", 6), ("primary", 7)):
    inst = rv.ladder_instances(ladder, n)
    k = 30 if n == 6 else 10
    res = rv.run_enum("ring", 0, [(a, b, c) for a, b, c, _ in inst[:k]])
    s = [r["frames"] for r in res]
    print("  committed %s n=%d (%d instances): measured mean %.4g, naive %.4g, balanced %.4g" % (ladder, n, k, st.mean(s), math.exp(rv.lnF_naive(n)), math.exp(rv.lnF_balanced(n))))
print("  16x16: naive 24*56!/5^60 = %.3g, balanced 24*56!*(12!)^5/60! = %.3g, Owen's census 4.05e37" % (math.exp(rv.lnF_naive(16)), math.exp(rv.lnF_balanced(16))))

print()
print("PART 3. solutions of generated boards (pieces distinct up to rotation; counts include the 4 turns)")
print("  n c boards capped | mean solutions, median, share with more than 4 | mean minus 4 | naive S | balanced S | mean full tree")
plan = [(4, 2, 400, 0), (4, 3, 2000, 0), (4, 4, 2000, 0), (4, 5, 2000, 0),
        (5, 3, 300, 0), (5, 4, 1000, 0), (5, 5, 1000, 0), (5, 6, 1000, 0),
        (6, 4, 16, 0), (6, 5, 30, "primary"), (6, 6, 30, "secondary"), (6, 7, 30, "plus2")]
for n, c, reps, src in plan:
    if src:
        boards = [(a, b, p) for a, b, p, _ in rv.ladder_instances(src, n)][:reps]
        label = "committed " + src
    else:
        boards = []
        while len(boards) < reps:
            p, _ = rv.planted_board(n, c, 5, rng)
            if p is None:
                break
            boards.append(("b%d" % len(boards), n, p))
        label = "review generator"
    res = rv.run_enum("full", 3 * 10**9, boards)
    ok = [r for r in res if not r["capped"]]
    s = [r["solutions"] for r in ok]
    print("  %d %d %d %d | %.2f, %g, %.2f | %.3g | %.3g | %.3g | %.3g  (%s)" % (
        n, c, len(ok), len(res) - len(ok), st.mean(s), st.median(s), sum(1 for v in s if v > 4) / len(s),
        st.mean(s) - 4, math.exp(rv.lnS_naive(n, c)), math.exp(rv.lnS_balanced(n, c)), st.mean(r["nodes"] for r in ok), label))
    sys.stdout.flush()

print()
print("PART 4. colour count where the first moment equals 1 (all four turns counted) and where it equals 4")
print("  n  Owen  naive(memo)  balanced S=1  balanced S=4")
for n in (5, 6, 7, 8, 9, 10, 12, 16):
    print("  %d  %.2f  %.2f  %.2f  %.2f" % (n, rv.owen(n), rv.solve_c(lambda c: rv.lnS_naive(n, c)),
          rv.solve_c(lambda c: rv.lnS_balanced(n, c)), rv.solve_c(lambda c: rv.lnS_balanced(n, c), target=math.log(4))))
print("  rungs: n c | lnS naive | lnS balanced | c minus balanced critical")
for n, c in ((6, 5), (6, 6), (7, 6), (7, 7), (8, 7), (8, 8), (9, 9), (9, 8), (10, 10), (10, 9), (6, 7), (7, 8), (8, 10), (9, 11), (10, 12), (16, 17)):
    print("  %d %d | %.1f | %.1f | %+.2f" % (n, c, rv.lnS_naive(n, c), rv.lnS_balanced(n, c), c - rv.solve_c(lambda q: rv.lnS_balanced(n, q))))
sb = rv.lnS_balanced(16, 17)
print("  16x16 c=17: naive S = %.3g; balanced S = %.4g with no clue, %.1f with the starter pinned (divide by 784), %.2g with five clues (784^5)" % (
    math.exp(rv.lnS_naive(16, 17)), math.exp(sb), math.exp(sb) / 784, math.exp(sb) / 784**5))
print("elapsed %.0f s" % (time.time() - t0))
