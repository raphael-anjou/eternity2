"""Gap 1: what does it cost to make the reason of a dead node small in tile-set terms?

A node at free depth d is (frontier f, used set U). Its plain refutation is the subtree
under it, T_0 nodes in the mean, and its reason is the whole state. Generalise the
reason by declaring k of the used interior tiles "not needed": the refuter must then
kill the frontier with those k tiles available again, so its tree is searched with a
pool of P + k interior tiles instead of P. First-moment size of that tree, from the
measured level widths N (Track 10):

    T_k = sum_j (N_(d+j) / N_d) * prod_(i < int(j)) (P + k - i) / (P - i)

(int(j) = interior cells among the next j cells). N is the independently re-checked first-moment
profile (rv_lib, class-aware, five clues, read only), because the measured one stops
at free depth 192 and the generalised tree lives beyond it. A generalised reason is
counted as a reason only while the expected number of complete fillings under it is
below one (otherwise it is alive in the mean and proves nothing). The generalised reason (f, R) with
|R| = ui - k is contained in N_d * 2^-b real nodes of the level, b = bits(f) +
log2(C(pool, ui-k) / C(ui, ui-k)), and never fewer than one.

    net gain(k) = log2(reuse_k) - log2(T_k / T_0)     (bits; positive = learning pays)

usage: python3 -I 04_generalise.py
"""
import sys
from math import comb, log2, log10

sys.path.insert(0, __import__("os").path.dirname(__file__))
import e2real as E

LIB = "research/program/night/2026-11-12/notes/price-and-ledger-review"
sys.path.insert(0, LIB)
import rv_lib as RV

_p, _h = RV.load_official()
_m = RV.Model(16, RV.census(_p))
w = [0.0] + list(_m.profile(RV.rows_from(16, top=False), _h, mode="class"))
L10 = log2(10)
print("model profile: total 2^%.2f, widest level free depth %d (measured: 2^134.39, 158)" % (RV.logsum(w) * L10, w.index(max(w))))
POOL = E.INTERIOR_POOL


def is_int(p):
    r, c = E.CELLS[p]
    return 0 < r < 15 and 0 < c < 15


print("depth | log2 N_d | interior used ui, left P | bits of the frontier | log2 T_0 | largest k still a reason | k at which a second real node is covered | net gain there | best net gain over 1 <= k <= largest")
for d in (52, 80, 96, 113, 128, 144, 158, 166):
    ui = sum(1 for p in range(d) if is_int(p))
    P = POOL - ui
    bf = log2(17)  # left colour
    for c in range(16):
        bf += log2(5) if c in (0, 15) else log2(17)
    logN = w[d] * L10
    jmax = len(w) - 1 - d
    ints = [0]
    for j in range(1, jmax + 1):
        ints.append(ints[-1] + (1 if is_int(d + j - 1) else 0))

    def T(k):
        """(log2 tree, log2 expected complete fillings)"""
        tot, f, done = 0.0, 0.0, 0
        for j in range(jmax + 1):
            ji = ints[j]
            while done < ji:
                f += log2((P + k - done) / (P - done))
                done += 1
            last = (w[d + j] - w[d]) * L10 + f
            tot += 2 ** last
        return log2(tot), last

    T0 = T(0)[0]
    best, first, kmax = None, None, 0
    for k in range(1, ui + 1):
        tk, full = T(k)
        if full >= 0:
            break
        kmax = k
        b = bf + log2(comb(POOL, ui - k) / comb(ui, ui - k))
        bmin = b
        reuse = max(0.0, logN - b)
        g = reuse - (tk - T0)
        if best is None or g > best[0]:
            best = (g, k, b, tk)
        if first is None and reuse > 0:
            first = (k, g)
    print("%3d | %.1f | %d, %d | %.1f | %.1f | %d (reason then %.0f bits) | %s | %s | %.1f (k = %d, reason %.0f bits, proof 2^%.1f)" % (
        d, logN, ui, P, bf, T0, kmax, bmin, first[0] if first else "none", "%.1f" % first[1] if first else "-", best[0], best[1], best[2], best[3]))

# Whole-search price of "walk the row scan to depth d, cut every node there with a
# reason generalised over k tiles": log2( sum_(d' <= d) N_d' + (N_d / reuse_k) * T_k ).
print()
print("whole search: best (d, k), no limit on the number of reasons, then with reasons of at most 61 and 40 bits")
tot_all = RV.logsum(w) * L10
for limit in (10 ** 9, 61, 40):
    best = None
    for d in range(60, 200):
        ui = sum(1 for p in range(d) if is_int(p))
        P = POOL - ui
        bf = log2(17) + 14 * log2(17) + 2 * log2(5)
        logN = w[d] * L10
        jmax = len(w) - 1 - d
        ints = [0]
        for j in range(1, jmax + 1):
            ints.append(ints[-1] + (1 if is_int(d + j - 1) else 0))
        walk = RV.logsum(w[: d + 1]) * L10
        for k in range(0, ui + 1):
            tot, f, done, last = 0.0, 0.0, 0, 0.0
            for j in range(jmax + 1):
                while done < ints[j]:
                    f += log2((P + k - done) / (P - done))
                    done += 1
                last = (w[d + j] - w[d]) * L10 + f
                tot += 2 ** last
            if k and last >= 0:
                break
            b = bf + log2(comb(POOL, ui - k) / comb(ui, ui - k))
            if k and b > limit:
                continue
            reuse = max(0.0, logN - b)
            proofs = logN - reuse + log2(tot)
            price = max(walk, proofs) + log2(1 + 2 ** (-abs(walk - proofs)))
            if best is None or price < best[0]:
                best = (price, d, k, b, logN - reuse, log2(tot))
    print("  limit %s bits: price 2^%.2f (plain 2^%.2f, gain %.2f bits) at d = %d, k = %d, reason %.0f bits, %s reasons of 2^%.1f nodes each" % (
        "none" if limit > 10 ** 6 else limit, best[0], tot_all, tot_all - best[0], best[1], best[2], best[3], "2^%.1f" % best[4], best[5]))
