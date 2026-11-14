#!/usr/bin/env python3
"""Model prices for the candidates. Run: python3 -I a4_price.py  (about 2 minutes)

Everything here is the first-moment model of partial-match-regime/model.py, used through
the review's r3_sensitivity.py (tail below ONE perfect depth-d0 prefix whose remaining
half-edges are spread evenly over ci interior and cr ring colours). No clue. K = 9 is a
471, K = 10 a 470. c0 = nodes spent in the first 160 cells per prefix that reaches depth
160 (measured in a1_hist.py for the corrected quota: about 10^1 to 10^2, opening excluded).

P1  baseline, re-tuned list, and the two floors (perfect oracle from 201, from 161)
P2  what a filter on arrivals can buy: lognormal yields, cost c_a per arrival, c_t per tail
P3  staged exhaustion of one more interior colour between depth 160 and d1
P4  tail first (backward construction): tails per board and head cost
P5  where the last cells should be: perfect completions per prefix for three tail shapes
"""
import importlib.util
import math
import os
import sys
from fractions import Fraction
from math import comb, exp, factorial, lgamma, log

sys.dont_write_bytecode = True
H = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location('r3', os.path.join(H, '..', 'partial-match-regime-review', 'r3_sensitivity.py'))
r3 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r3)
M = r3.M
E2 = r3.E2
LN10 = r3.LN10
lse = r3.lse
BW = r3.BW
L = lambda x: x / LN10


def tree_stats(d0, ul, ci, cr):
    lv = r3.tail_tree(d0, ul, ci, cr)
    per = {d: lse(list(lv[d].values())) for d in lv}
    return lv, per


def p1():
    print('== P1. 15 + 4 colours below a perfect depth-160 prefix: log10 nodes per board ==')
    for K in (10, 9):
        print('-- K = %d (score %d) --' % (K, 480 - K))
        lists = {'Blackwood, last %d entries' % K: BW[10 - K:], 'Blackwood, first %d entries' % K: BW[:K]}
        for c0l in (1.0, 2.0, 3.0):
            c0 = c0l * LN10
            out = []
            for name, ul in lists.items():
                s = r3.tail_summary(160, ul, 15, 4, c0)
                out.append('%s: %.2f (late %.2f)' % (name, s['q'], s['late_share']))
            f = lambda ul: r3.tail_summary(160, ul, 15, 4, c0)['q']
            o, q = r3.descend(f, BW[10 - K:], 161, 255)
            sb = r3.tail_summary(160, o, 15, 4, c0)
            # floors: perfect oracle from depth g = only prefixes that hold a board are entered after g
            fl = []
            for g in (201, 181, 161):
                lv, per = tree_stats(160, [g] * K, 15, 4)
                boards = per[256]
                pre = lse([c0] + [lv[d][0] for d in range(161, g)])
                fl.append('oracle from %d: %.2f (boards per prefix %.2f)' % (g, L(lse([pre, boards + log(257 - g)]) - boards), L(boards)))
            print('  c0 = 10^%.0f | %s | best list %s: %.2f (late %.2f, boards per prefix %.2f) | %s' % (
                c0l, ' ; '.join(out), o, q, sb['late_share'], sb['boards'], ' ; '.join(fl)))


def phi(x):
    return 0.5 * math.erfc(-x / math.sqrt(2.0))


def p2():
    print()
    print('== P2. Filter on arrivals. Yield of an arrival lognormal, sd s decades; the predictor sees a share r2 of the variance ==')
    print('   cost per board without filter = (c_a + c_t) / mean yield ; with threshold z on the predicted part:')
    print('   (c_a + c_t * P(pass)) / E[yield ; pass] ; gain = ratio, at the best z')
    print('   %8s %8s %10s %10s %10s %10s' % ('s (dec)', 'r2', 'ct/ca=1', '10', '100', '1000'))
    for s in (0.25, 0.5, 1.0, 1.4, 2.0):
        for r2 in (1.0, 0.5, 0.25):
            sig = s * LN10 * math.sqrt(r2)  # sd of the predicted part, natural log
            row = []
            for ratio in (1.0, 10.0, 100.0, 1000.0):
                best = 1.0
                z = -4.0
                while z < 8.0:
                    pas = phi(-z)
                    ey = phi(sig - z)  # E[y ; pass] / E[y]
                    if ey > 0:
                        g = (1.0 + ratio) / ((1.0 + ratio * pas) / ey)
                        best = max(best, g)
                    z += 0.05
                row.append('%10.1f' % best)
            print('   %8.2f %8.2f %s' % (s, r2, ' '.join(row)))
    print('   ceiling in every case: 1 + c_t / c_a (the arrivals still have to be made and looked at)')


def coeff_logs(ncs, emax):
    """ln of coefficients 0..emax of prod_c sum_j (n_c)_(2j) x^j / j!  (exact fractions)."""
    poly = [Fraction(1)]
    for n in ncs:
        g = [Fraction(factorial(n), factorial(n - 2 * j) * factorial(j)) for j in range(n // 2 + 1)]
        new = [Fraction(0)] * min(emax + 1, len(poly) + len(g) - 1)
        for i, a in enumerate(poly):
            if a:
                for j, b in enumerate(g):
                    if i + j <= emax:
                        new[i + j] += a * b
        poly = new
    return poly


def flog(fr):
    return log(fr.numerator) - log(fr.denominator) if fr > 0 else -math.inf


def p3():
    print()
    print('== P3. One more interior colour used up between depth 160 and d1 (15 + 4 before d1, 14 + 4 after) ==')
    stats = E2.prefix_stats()
    ei160, ei256 = stats[160][2], stats[-1][2]
    E = ei256 - ei160
    ncs = M.even_split(2 * E, 15)
    print('   tail interior joints %d, half-edges per colour %s' % (E, sorted(set(ncs))))
    for K in (10, 9):
        ul = BW[10 - K:]
        base = {}
        for c0l in (1.0, 2.0):
            base[c0l] = r3.tail_summary(160, ul, 15, 4, c0l * LN10)['q']
        lv1, per1 = tree_stats(160, ul, 15, 4)
        for d1 in (176, 185, 192, 200):
            e1 = stats[d1][2] - ei160
            nC = ncs[-1]
            mC = nC // 2
            allp = coeff_logs(ncs, e1)
            oth = coeff_logs(ncs[:-1], e1)
            if e1 - mC < 0 or oth[e1 - mC] == 0:
                continue
            lnR = flog(Fraction(factorial(nC), factorial(mC)) * oth[e1 - mC]) - flog(allp[e1])
            # self-check of the algebra: P(the e1 stretch joints all match) against the model's own law
            lnP = lgamma(e1 + 1) + flog(allp[e1]) - (lgamma(2 * E + 1) - lgamma(2 * E - 2 * e1 + 1))
            chk = M.JointLaw(M.interior_dist(ncs)).lspec(e1, 0)
            lv2, per2 = tree_stats(d1, ul, 14, 4)
            n2 = lse([per2[d] for d in per2 if d > d1])
            b2 = per2[256]
            lvb, perb = tree_stats(d1, ul, 15, 4)
            a1 = per1[d1]
            pre = lse([per1[d] for d in range(161, d1 + 1)])
            line = []
            for c0l in (1.0, 2.0):
                c0 = c0l * LN10
                filt = lse([c0, pre, a1 + lnR + n2]) - (a1 + lnR + b2)
                ideal = lse([c0, log(d1 - 160), n2]) - b2
                line.append('c0=10^%.0f: baseline %.2f, filter at d1 %.2f, perfect ramp %.2f' % (c0l, base[c0l], L(filt), L(ideal)))
            print('   K=%d d1=%d: stretch joints %d ; share of plain arrivals at d1 that have used the colour up: 10^%.2f ; yield per arrival at d1: plain 10^%.2f, staged 10^%.2f (gain 10^%.2f) ; tail nodes per arrival: plain 10^%.2f staged 10^%.2f | %s | check ln P %.3f = %.3f' % (
                K, d1, e1, L(lnR), L(perb[256]), L(b2), L(b2 - perb[256]), L(lse([perb[d] for d in perb if d > d1])), L(n2), ' ; '.join(line), lnP, chk))


def p4():
    print()
    print('== P4. Tail first: build the last 56 cells from the full pool, then fill the 200-cell head exactly ==')
    stats = E2.prefix_stats()
    cells = E2.cells
    tail = cells[200:]
    ncls = {'c': 0, 'e': 0, 'i': 0}
    ji = jr = 0
    for cls, joints in tail:
        ncls[cls] += 1
        ji += joints.count('i')
        jr += joints.count('r')
    # joints between a tail cell and a head cell: the top joints of the first 16 tail cells and one left joint
    bi, br = 0, 0
    for idx, (cls, joints) in enumerate(tail[:16]):
        pass
    # count by geometry: tail = cells 200..255 = row 12 cols 8..15, rows 13..15
    # top joints into the head: row 12 cols 8..15 (8) and row 13 cols 0..7 (8); left joint of cell (12, 8)
    top = [(12, c) for c in range(8, 16)] + [(13, c) for c in range(0, 8)]
    for (r, c) in top:
        if c in (0, 15):
            br += 1
        else:
            bi += 1
    bi += 1
    ii, ir = ji - bi, jr - br
    lnA = (lgamma(197) - lgamma(197 - ncls['i']) + ncls['i'] * log(4) + lgamma(57) - lgamma(57 - ncls['e'])
           + lgamma(5) - lgamma(5 - ncls['c']))
    print('   tail cells: %s ; joints closed while filling it in row scan: %d interior + %d ring, of which %d + %d go to the head' % (ncls, ji, jr, bi, br))
    for K in (10, 9):
        # perfect tails and tails with at most K breaks inside the tail, independent joints at p = 1/17 and 1/5
        terms = []
        for kr in range(0, K + 1):
            for ki in range(0, K + 1 - kr):
                terms.append(math.log(comb(ii, ki)) + ki * log(16) + math.log(comb(ir, kr)) + kr * log(4))
        lnT = lnA - ii * log(17) - ir * log(5) + lse(terms)
        boards = r3.nodes_per_board(BW[10 - K:])[1]
        print('   K=%d: free tails with at most K breaks inside: 10^%.1f ; boards the list reaches (17 + 5 colours): 10^%.2f ; exact heads per tail: 10^%.1f' % (
            K, L(lnT), boards, boards - L(lnT)))
    print('   so a tail-first search must pay a 200-cell exact search per tail and succeeds once in that many tails.')
    print('   (the review of the price memo puts an exact half-board with its own pool at 10^9 to 10^10 nodes)')


def p5():
    print()
    print('== P5. Perfect completions of a 56 to 60 cell tail by ONE head, first moment, log10 ==')

    def strip(pi, pr):
        return (lgamma(36) + 35 * log(4) + lgamma(20) + log(2) - 87 * log(pi) - 22 * log(pr)) / LN10

    def pocket(pi):
        return (lgamma(57) + 56 * log(4) - (2 * 56 + 7 + 8) * log(pi)) / LN10

    def frame(pi, pr):
        return (lgamma(57) + lgamma(5) - 56 * log(pi) - 60 * log(pr)) / LN10

    for pi, pr in ((17, 5), (15, 4)):
        print('   %d + %d colours: bottom strip (3.5 rows) %.1f ; 7x8 interior pocket %.1f ; the whole frame placed last %.1f' % (
            pi, pr, strip(pi, pr), pocket(pi), frame(pi, pr)))
    print('   the frame-last tail needs a head that is a complete 14x14 interior using all 196 interior pieces:')
    perfect_int = (lgamma(197) + 196 * log(4) - 364 * log(17)) / LN10
    print('   frame-free perfect interiors, first moment: 10^%.1f ; the strict interior-only tree has no border to prune it' % perfect_int)


if __name__ == '__main__':
    p1()
    p2()
    p3()
    p4()
    p5()
