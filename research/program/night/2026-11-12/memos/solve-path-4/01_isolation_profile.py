"""First-moment (annealed, independent-edge) counting for the last-ten-edges memo.

Model: a class-respecting placement (corners in corners, border pieces on the rim with grey out,
interior pieces inside in any of 4 rotations) is drawn uniformly; each along-frame joint matches
with probability p_f, every other internal joint with probability p_i, independently.
This is the model of research/topics/the-470-wall (landscape.json); section A re-derives its table.

A. level sets: log10 N(k) = boards with exactly k mismatches, whole board
B. isolation profile: expected number of boards with at most k mismatches that differ from a
   given SOLUTION exactly on a region S (pieces of S permuted inside S), for rectangles S,
   complements of rectangles, and scattered cells; cheapest board per distance band
C. improvement radius: the same count around a given 470 BOARD (at most 9 mismatches wanted)
D. row-major perfect-prefix profile (tree size), starter only and five clues
E. the same band minima for the small palettes measured in research/experiments/near-codeword-spectrum
Stdlib only: python3 -I 01_isolation_profile.py
"""
import math, os, sys, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib4 import *

NEG = float("-inf")


def pmf_binom(E, q, kmax):
    """log10 pmf of number of mismatches (prob q each) among E edges, for 0..kmax."""
    out = []
    for j in range(kmax + 1):
        if j > E:
            out.append(NEG)
        else:
            out.append(lg_binom(E, j) + (j * math.log10(q) if j else 0.0) + ((E - j) * math.log10(1 - q) if E - j else 0.0))
    return out


def lg_cdf2(Ef, Ei, pf, pi, k):
    """log10 P(mismatches among Ef frame joints + Ei other joints <= k)."""
    a = pmf_binom(Ef, 1 - pf, k)
    b = pmf_binom(Ei, 1 - pi, k)
    tot = NEG
    for x in range(k + 1):
        if a[x] == NEG:
            continue
        for y in range(k + 1 - x):
            if b[y] == NEG:
                continue
            tot = lg_add(tot, a[x] + b[y])
    return min(tot, 0.0)


def lg_pmf2(Ef, Ei, pf, pi, k):
    a = pmf_binom(Ef, 1 - pf, k)
    b = pmf_binom(Ei, 1 - pi, k)
    tot = NEG
    for x in range(k + 1):
        if a[x] != NEG and b[k - x] != NEG:
            tot = lg_add(tot, a[x] + b[k - x])
    return tot


def region_stats(n, cells):
    """(corners, border cells, interior cells, frame joints, other joints) touched by a cell set."""
    S = cells
    hc = he = hi = 0
    for r, c in S:
        rim = (r in (0, n - 1)) + (c in (0, n - 1))
        if rim == 2:
            hc += 1
        elif rim == 1:
            he += 1
        else:
            hi += 1
    Ef = Ei = bl = 0
    for a, sa, b, sb in internal_edges(n):
        if a in S or b in S:
            along = (a[0] == b[0] and a[0] in (0, n - 1)) or (a[1] == b[1] and a[1] in (0, n - 1))
            if along:
                Ef += 1
            else:
                Ei += 1
            if (a in S) != (b in S):
                bl += 1
    return hc, he, hi, Ef, Ei, bl


def _der(m):
    d = [1, 0]
    for i in range(2, m + 1):
        d.append((i - 1) * (d[i - 1] + d[i - 2]))
    return d[m]


def lg_arr(hc, he, hi):
    """log10 number of rearrangements of the region's own pieces in which EVERY cell changes
    (piece or rotation). Border and corner pieces have one legal rotation per cell."""
    out = 0.0
    for m in (hc, he):
        if m == 0:
            continue
        if m <= 30:
            d = _der(m)
            if d == 0:
                return NEG
            out += math.log10(d)
        else:
            out += lg_fact(m) - 1 / L10
    if hi:
        if hi <= 25:
            v = sum((-1) ** j * math.comb(hi, j) * math.factorial(hi - j) * 4 ** (hi - j) for j in range(hi + 1))
            out += math.log10(v)
        else:
            out += lg_fact(hi) + hi * math.log10(4) - 0.25 / L10
    return out


def droplet(ci):
    """Decades of shape entropy per unit of boundary length: exact surface entropy of a 2D Ising
    droplet at boundary fugacity ci**-0.5 (Onsager), an upper bound for a finite region."""
    return -math.log10(math.tanh(0.25 * math.log(ci)))


def families(n):
    """Regions: every rectangle and every complement of a rectangle. Yields (size, stats)."""
    allc = {(r, c) for r in range(n) for c in range(n)}
    seen = {}
    for r0 in range(n):
        for r1 in range(r0, n):
            for c0 in range(n):
                for c1 in range(c0, n):
                    rect = {(r, c) for r in range(r0, r1 + 1) for c in range(c0, c1 + 1)}
                    for S in (rect, allc - rect):
                        if not S:
                            continue
                        st = region_stats(n, S)
                        key = (len(S), st)
                        seen[key] = seen.get(key, 0) + 1
    return seen


def band_minimum(fam, pf, pi, lo, hi_, kmax=40, mult=True, ent=0.0):
    """Smallest k such that the expected number of boards with <= k mismatches, differing from the
    reference exactly on some region of the family with lo <= |S| <= hi_, is at least 1."""
    for k in range(kmax + 1):
        tot = NEG
        for (h, (hc, he, hi, Ef, Ei, bl)), cnt in fam.items():
            if lo <= h <= hi_:
                v = lg_arr(hc, he, hi) + lg_cdf2(Ef, Ei, pf, pi, k) + (math.log10(cnt) if mult else 0) + ent * bl
                tot = lg_add(tot, v)
        if tot >= 0:
            return k, tot
    return None, tot


def profile(n, pf, pi, fam, ks, sizes, ent=0.0):
    rows = []
    for lo, hi_ in sizes:
        row = []
        for k in ks:
            tot = NEG
            for (h, (hc, he, hi, Ef, Ei, bl)), cnt in fam.items():
                if lo <= h <= hi_:
                    tot = lg_add(tot, lg_arr(hc, he, hi) + lg_cdf2(Ef, Ei, pf, pi, k) + math.log10(cnt) + ent * bl)
            row.append(tot)
        rows.append(((lo, hi_), row))
    return rows


def main():
    n = 16
    pf, pi = 0.2, 0.058843537  # research/topics/the-470-wall/results/landscape.json values (recomputed in 03)
    lgW = lg_fact(4) + lg_fact(56) + lg_fact(196) + 196 * math.log10(4)
    print("== A. level sets, whole board ==")
    print("log10 W_geom = %.2f" % lgW)
    lgN = {}
    for k in (0, 1, 2, 5, 9, 10, 11, 15, 20, 30, 40):
        lgN[k] = lgW + lg_pmf2(60, 420, pf, pi, k)
        print("  k=%2d (score %d): log10 N = %.2f" % (k, 480 - k, lgN[k]))
    print("  slope near k=10: %.2f decades per edge" % (lgN[11] - lgN[10]))

    fam = families(n)
    print("\n== B. boards near a given solution: log10 expected count, by size of the changed region ==")
    print("regions: all rectangles and complements of rectangles (%d distinct types)" % len(fam))
    ks = (0, 4, 10, 20, 30)
    bands = [(1, 1), (2, 2), (3, 4), (5, 8), (9, 16), (17, 32), (33, 64), (65, 96), (97, 128), (129, 160), (161, 192), (193, 224), (225, 240), (241, 250), (251, 256)]
    print("  size band    " + "".join("  k<=%-3d" % k for k in ks))
    for (lo, hi_), row in profile(n, pf, pi, fam, ks, bands):
        print("  %3d..%-3d    " % (lo, hi_) + "".join(" %7.1f" % v for v in row))
    ent = droplet(17)
    print("  same, with the droplet shape-entropy bound (+%.3f decades per unit of boundary length):" % ent)
    for (lo, hi_), row in profile(n, pf, pi, fam, ks, bands, ent):
        print("  %3d..%-3d    " % (lo, hi_) + "".join(" %7.1f" % v for v in row))
    print("  cheapest board per band (smallest k with expected count >= 1), rectangles only / with shape-entropy bound:")
    for lo, hi_ in bands:
        k, tot = band_minimum(fam, pf, pi, lo, hi_, kmax=60)
        k2, tot2 = band_minimum(fam, pf, pi, lo, hi_, kmax=60, ent=ent)
        print("    size %3d..%-3d  k_min = %s / %s" % (lo, hi_, k, k2))

    print("\n  scattered regions (no two changed cells adjacent), k<=10:")
    tot_near = NEG
    for hi in range(0, 7):
        for he in range(0, 9):
            if hi + he == 0 or (hi == 1 and he == 0 and False):
                continue
            # derangements for border cells (rotation forced), any non-identity arrangement for interior
            der = [1, 0, 1, 2, 9, 44, 265, 1854, 14833]
            if he == 1:
                continue
            arr_i = (math.factorial(hi) * 4 ** hi - 1) if hi else 1
            if hi == 1:
                arr_i = 3
            lg = lg_binom(196, hi) + lg_binom(56, he) + math.log10(max(arr_i, 1)) + math.log10(der[he]) + lg_cdf2(2 * he, he + 4 * hi, pf, pi, 10)
            tot_near = lg_add(tot_near, lg)
            if (hi, he) in ((1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (0, 2), (0, 4), (0, 6), (0, 8), (2, 4)):
                print("    interior %d + border %d changed: log10 count = %.1f" % (hi, he, lg))
    print("    near band total (up to 6 interior + 8 border cells changed): log10 = %.1f" % tot_near)
    for nsol, lab in ((1, "five clues, 1 solution"), (14702, "starter only, 14,702 expected")):
        print("    fraction of 470 boards in the near band, %s: 10^%.1f (naive N) to 10^%.1f (N raised by the 5.9-decade correction)" % (
            lab, tot_near + math.log10(nsol) - lgN[10], tot_near + math.log10(nsol) - lgN[10] - 5.9))

    print("\n== C. improvement radius around a 470 board (10 mismatches inside the region, at most 9 wanted) ==")
    for lo, hi_ in bands:
        # no multiplicity: the region must contain the board's own mismatches, so take the best single region
        best = max((lg_arr(hc, he, hi) + lg_cdf2(Ef, Ei, pf, pi, 9)) for (h, (hc, he, hi, Ef, Ei, bl)) in fam if lo <= h <= hi_)
        best2 = max((lg_arr(hc, he, hi) + lg_cdf2(Ef, Ei, pf, pi, 9) + ent * bl) for (h, (hc, he, hi, Ef, Ei, bl)) in fam if lo <= h <= hi_)
        print("  region size %3d..%-3d: best single region log10 E[boards with <=9 mismatches] = %7.1f (with shape-entropy bound %7.1f)" % (lo, hi_, best, best2))
    # rows 1..5 full width, the band that holds every mismatch of the 470 board
    S = {(r, c) for r in range(1, 6) for c in range(16)}
    hc, he, hi, Ef, Ei, bl = region_stats(n, S)
    for k in (9, 5, 0):
        print("  rows 1..5 full width (80 cells: %d border, %d interior; %d+%d joints), <=%d mismatches: log10 E = %.1f" % (
            he, hi, Ef, Ei, k, lg_arr(hc, he, hi) + lg_cdf2(Ef, Ei, pf, pi, k)))
    S = {(r, c) for r in range(0, 6) for c in range(16)}
    hc, he, hi, Ef, Ei, bl = region_stats(n, S)
    for k in (9, 0):
        print("  rows 0..5 full width (96 cells), <=%d mismatches: log10 E = %.1f" % (k, lg_arr(hc, he, hi) + lg_cdf2(Ef, Ei, pf, pi, k)))

    print("\n== D. row-major perfect-prefix counts (naive first moment) ==")
    clue_cells_all = []
    lines = [l.strip() for l in open(CSV) if l.strip()]
    for i, l in enumerate(lines[1:]):
        c = l.split(",")
        if len(c) >= 7 and (int(c[4]) or int(c[5]) or int(c[6])):
            clue_cells_all.append((int(c[5]), int(c[4])))
    print("  clue cells (row, col) from the official csv:", clue_cells_all)
    center = [rc for rc in clue_cells_all if 6 <= rc[0] <= 9 and 6 <= rc[1] <= 9]
    rm = [(r, c) for r in range(n) for c in range(n)]
    two_front = [(r, c) for r in (0, 1, 2) for c in range(n)] + [(r, c) for r in (15, 14, 13) for c in range(n)] + [(r, c) for r in range(3, 13) for c in range(n)]
    col_first = [(r, c) for c in (0, 1, 2) for r in range(n)] + [(r, c) for c in (15, 14, 13) for r in range(n)] + [(r, c) for c in range(3, 13) for r in range(n)]
    ring = []
    seen = set()
    for layer in range(8):
        for r in range(n):
            for c in range(n):
                if min(r, c, n - 1 - r, n - 1 - c) == layer and (r, c) not in seen:
                    ring.append((r, c)); seen.add((r, c))
    results = {}
    for oname, order in (("row-major", rm), ("rows 0-2, rows 15-13, then rows 3-12", two_front), ("frame and rings inward", ring)):
        for lab, clues in (("no clue", []), ("starter only", center), ("five clues", clue_cells_all)):
            nc, ne, ni = 4, 56, 196 - len(clues)
            lg = 0.0
            peak = NEG
            total = NEG
            placed = set()
            marks = {}
            for idx, (r, c) in enumerate(order):
                rim = (r in (0, n - 1)) + (c in (0, n - 1))
                if (r, c) in clues:
                    pass
                elif rim == 2:
                    lg += math.log10(nc); nc -= 1
                elif rim == 1:
                    lg += math.log10(ne); ne -= 1
                else:
                    lg += math.log10(ni * 4); ni -= 1
                for (rr, cc) in ((r - 1, c), (r, c - 1), (r + 1, c), (r, c + 1)):
                    if (rr, cc) not in placed:
                        continue
                    along = (rr == r and r in (0, n - 1)) or (cc == c and c in (0, n - 1))
                    lg += math.log10(pf if along else pi)
                placed.add((r, c))
                peak = max(peak, lg)
                total = lg_add(total, lg)
                if (idx + 1) % 16 == 0:
                    marks[(idx + 1) // 16] = lg
            results[(oname, lab)] = total
            print("  %-38s %-12s after 16,32,48,64,128,192,256 cells: %s | peak %.1f | sum over depths (tree size) %.1f" % (
                oname, lab, " ".join("%.1f" % marks[m] for m in (1, 2, 3, 4, 8, 12, 16)), peak, total))
    base = results[("row-major", "starter only")]
    print("  reported anchor: 9.28e42 nodes per solution and 14,702 solutions with the starter (McGavin, cited on build/analysis/solution-counting.mdx), i.e. a tree of 10^%.1f" % math.log10(9.28e42 * 14702))
    for key, v in results.items():
        if key[1] == "five clues":
            print("    five-clue tree, %s: naive 10^%.1f, anchored (naive ratio to starter row-major times the reported tree) 10^%.1f" % (key[0], v, v - base + math.log10(9.28e42 * 14702)))

    print("\n== E. small palettes of research/experiments/near-codeword-spectrum (committed medians in its README) ==")
    for n_, C, ref in ((5, 13, "near* 2, mid* 4, far* 7"), (6, 14, "near* 2, mid* 4, far* [5,12]"), (7, 15, "mid* [3,15], far* [3,16] (open)"), (8, 16, "not measured"), (10, 18, "not measured"), (12, 19, "not measured"), (16, 22, "the real puzzle")):
        ci = C - 5
        pf_, pi_ = 1 / 5, 1 / ci
        N_ = n_ * n_
        fam_ = families(n_)
        near = band_minimum(fam_, pf_, pi_, 1, math.ceil(N_ / 4) - 1)[0]
        mid_hi = band_minimum(fam_, pf_, pi_, math.ceil(N_ / 4), (3 * N_) // 4)[0]
        mid_lo = band_minimum(fam_, pf_, pi_, math.ceil(N_ / 4), (3 * N_) // 4, ent=droplet(ci))[0]
        mid = "%s..%s" % (mid_lo, mid_hi)
        # far: boards unrelated to the planted one: whole-board first moment
        lgW_ = lg_fact(4) + lg_fact(4 * (n_ - 2)) + lg_fact((n_ - 2) ** 2) + (n_ - 2) ** 2 * math.log10(4)
        Ef, Ei = 4 * (n_ - 1), 2 * n_ * (n_ - 1) - 4 * (n_ - 1)
        far = None
        for k in range(0, 80):
            if lgW_ + lg_cdf2(Ef, Ei, pf_, pi_, k) >= 0:
                far = k
                break
        print("  %2dx%-2d C=%2d (5 frame + %2d interior): model near %s, mid %s, far %s | measured: %s" % (n_, n_, C, ci, near, mid, far, ref))


main()
