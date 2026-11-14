#!/usr/bin/env python3
"""Referee script 3. The memo's own model, asked the questions the memo did not ask.

Run: python3 -I r3_sensitivity.py   (about one minute, one core, stdlib only)

Imports the memo's model.py read-only (no bytecode written). Parts:
 1  Score histogram of ONE engine with a FIXED list (what JSA's log and the volunteer
    dashboard report), as the model predicts it, against the "price per point".
 2  Price of a point with the 12-entry list that is in the repo (201 ... 239, 241, 256)
    instead of the memo's assumed eleventh unlock at 196.
 3  The tail conditional on a perfect depth-160 prefix whose pool has fewer colours
    (the memo's own description of the record engine): where the nodes are, the
    price per point, the value of the ideal reuse bound and of a perfect oracle.
 4  Seventeen-colour tree with the pre-unlock part discounted by a factor s.
"""
import importlib.util
import math
import os
import sys
from math import comb, log, exp

sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
MEMO = os.path.join(HERE, '..', 'partial-match-regime', 'model.py')
spec = importlib.util.spec_from_file_location('memo_model', MEMO)
M = importlib.util.module_from_spec(spec)
spec.loader.exec_module(M)
LN10 = math.log(10.0)
lse = M.logsumexp

E2 = M.Board(16, [12] * 5, [24] * 5 + [25] * 12)
BW = [201, 206, 211, 216, 221, 225, 229, 233, 237, 239]


def cumulative(unlocks):
    return lambda d: sum(1 for u in unlocks if d >= u)


def tot(levels):
    return [lse(list(l.values())) for l in levels]


def part1():
    print('== 1. Score histogram of one engine with a fixed list (model) ==')
    for name, ul in (('Blackwood 10 entries', BW), ('repo 12 entries', BW + [241, 256]),
                     ('13 entries (181..)', [186, 191, 196] + BW)):
        K = len(ul)
        lv = E2.tree(K, cumulative(ul))
        fin = lv[-1]
        js = sorted(fin)
        print('%-22s log10 boards by breaks:' % name, ' '.join('%d:%.2f' % (j, fin[j] / LN10) for j in js if j >= K - 5))
        print('%-22s factor per point lost :' % '', ' '.join(
            '%d->%d: x%.0f' % (480 - j + 1, 480 - j, exp(fin[j] - fin[j - 1])) for j in js if j >= K - 4 and (j - 1) in fin and fin[j - 1] > -math.inf))
    print('Observed, one engine, one run (site, edge-slipping.mdx and eii.mdx, msg 6687): 17.7, 16.7, 21.8 per point from 463 to 466;')
    print('five-clue fleet (five-clue-fleet.mdx): 7645 at 463, 327 at 464, 11 at 465: 23.4 and 29.7.')


def nodes_per_board(ul, K=None):
    K = len(ul) if K is None else K
    lv = E2.tree(K, cumulative(ul))
    t = tot(lv)
    return (lse(t[1:]) - t[-1]) / LN10, t[-1] / LN10, lse(t[1:]) / LN10


def part2():
    print('\n== 2. Price of a point with the lists that exist ==')
    rows = [('10 entries (470 target)', BW), ('9 entries, first dropped', BW[1:]), ('9 entries, last dropped', BW[:-1]),
            ('11 entries: + 241 (repo)', BW + [241]), ('12 entries: + 241, 256 (repo)', BW + [241, 256]),
            ('11 entries: 196 + list (memo assumption)', [196] + BW)]
    base = nodes_per_board(BW)[0]
    for name, ul in rows:
        q, b, n = nodes_per_board(ul)
        print('%-42s log10 nodes/board %.2f (boards %.2f, nodes %.2f)  against the 10-entry list: 10^%+.2f' % (name, q, b, n, q - base))


def tail_tree(d0, ul, ci, cr):
    """Per depth d = d0..256: {j: ln E[# partial boards]} below ONE perfect depth-d0 prefix
    whose remaining half-edges are spread evenly over ci interior and cr ring colours."""
    K = len(ul)
    stats = E2.prefix_stats()
    la0, er0, ei0 = stats[d0]
    er, ei = stats[-1][1] - er0, stats[-1][2] - ei0
    inner = M.JointLaw(M.interior_dist(M.even_split(2 * ei, ci)))
    rc = M.balanced(er, cr)
    ring = M.JointLaw(M.ring_dist(rc, rc))
    cap_at = cumulative(ul)
    pat = {(0, 0): 1}
    levels = {d0: {0: 0.0}}
    for d, (cls, joints) in enumerate(E2.cells, start=1):
        if d <= d0:
            continue
        cap = min(K, cap_at(d))
        new = {}
        ni, nr = joints.count('i'), joints.count('r')
        for (ji, jr), cnt in pat.items():
            for bi in range(ni + 1):
                for br in range(nr + 1):
                    if bi + br > 1 or ji + jr + bi + br > cap:
                        continue
                    key = (ji + bi, jr + br)
                    new[key] = new.get(key, 0) + cnt * comb(ni, bi) * comb(nr, br)
        pat = new
        la, e_r, e_i = stats[d]
        lev = {}
        for (ji, jr), cnt in pat.items():
            v = (la - la0) + log(cnt) + inner.lspec(e_i - ei0, ji) + ring.lspec(e_r - er0, jr)
            lev.setdefault(ji + jr, []).append(v)
        levels[d] = {j: lse(v) for j, v in lev.items()}
    return levels


def tail_summary(d0, ul, ci, cr, c0=0.0, bound=None):
    """log10 of (c0 + tail nodes) / boards, plus pieces. c0 = ln cost of producing the prefix."""
    lv = tail_tree(d0, ul, ci, cr)
    K = len(ul)
    boards = lse(list(lv[256].values()))
    per_depth = {}
    for d in range(d0 + 1, 257):
        if bound is None or d == 256:
            per_depth[d] = lse(list(lv[d].values()))
        else:
            per_depth[d] = lse([v + min(0.0, bound[d][K - j]) for j, v in lv[d].items()])
    nodes = lse(list(per_depth.values()))
    late = lse([per_depth[d] for d in per_depth if d >= ul[0]]) if ul else -math.inf
    total = lse([c0, nodes])
    return {'q': (total - boards) / LN10, 'boards': boards / LN10, 'nodes': nodes / LN10,
            'late_share': exp(late - total), 'peak': max(per_depth, key=lambda d: per_depth[d])}


def descend(f, start, lo, hi, steps=(-8, -3, -1, 1, 3, 8), sweeps=4):
    cur = list(start)
    best = f(cur)
    for _ in range(sweeps):
        improved = False
        for i in range(len(cur)):
            for st in steps:
                cand = sorted(cur[:i] + [min(hi, max(lo, cur[i] + st))] + cur[i + 1:])
                v = f(cand)
                if v < best - 1e-6:
                    best, cur, improved = v, cand, True
        if not improved:
            break
    return cur, best


def part3():
    print('\n== 3. Tail below one perfect depth-160 prefix, pool spread over fewer colours ==')
    print('   (the memo, section 2.5: the record engine exhausts two interior colours and one ring colour by depth 160)')
    d0 = 160
    for ci, cr in ((17, 5), (15, 4)):
        print('-- %d interior + %d ring colours left --' % (ci, cr))
        rl = {d: M.reuse_log_counts(E2, d, ci, cr, 10) for d in range(1, 256)}
        s10 = tail_summary(d0, BW, ci, cr)
        s9 = tail_summary(d0, BW[1:], ci, cr)
        print('Blackwood list: tail nodes per prefix 10^%.2f, boards per prefix 10^%.2f, share of tail nodes at depth >= 201: %.3g, peak depth %d' % (
            s10['nodes'], s10['boards'], s10['late_share'], s10['peak']))
        print('   prefix cost c0 at which the first 160 cells and the tail cost the same: 10^%.2f nodes per quota-satisfying prefix' % s10['nodes'])
        for c0l in (0.0, 3.0, 6.0, 9.0, 12.0):
            c0 = c0l * LN10
            a = tail_summary(d0, BW, ci, cr, c0)
            b = tail_summary(d0, BW[1:], ci, cr, c0)
            ab = tail_summary(d0, BW, ci, cr, c0, bound=rl)
            f = lambda ul: tail_summary(d0, ul, ci, cr, c0)['q']
            o10, q10 = descend(f, BW, d0 + 1, 255)
            o9, q9 = descend(lambda ul: tail_summary(d0, ul, ci, cr, c0)['q'], BW[1:], d0 + 1, 255)
            fb = lambda ul: tail_summary(d0, ul, ci, cr, c0, bound=rl)['q']
            ob, qb = descend(fb, o10, d0 + 1, 255)
            sh = tail_summary(d0, o10, ci, cr, c0)['late_share']
            # perfect oracle from depth 201, ten breaks free after it
            free = tail_tree(d0, [201] * 10, ci, cr)
            boards = lse(list(free[256].values()))
            pre = lse([c0] + [free[d][0] for d in range(d0 + 1, 201)])
            orc = (lse([pre, boards + log(56)]) - boards) / LN10
            print('   c0 = 10^%-4.0f| Blackwood list 470: 10^%.2f per board (late share %.3g); 471 by truncation: x10^%.2f; ideal reuse bound on it: /10^%.2f' % (
                c0l, a['q'], a['late_share'], b['q'] - a['q'], a['q'] - ab['q']))
            print('               | best list 470: 10^%.2f %s (late share %.3g); best list 471: x10^%.2f; best list with ideal reuse bound: 10^%.2f (gain 10^%.2f); perfect oracle from 201: 10^%.2f' % (
                q10, o10, sh, q9 - q10, qb, q10 - qb, orc))


def part4():
    print('\n== 4. Seventeen colours, nodes before depth 201 discounted by s (crude stand-in for everything that makes the strict part cheap) ==')
    K = 10
    rl = {d: M.reuse_log_counts(E2, d, 17, 5, K) for d in range(1, 256)}
    strict = tot(E2.tree(0, lambda d: 0))

    def cost(ul, sl, bound=False):
        lv = E2.tree(len(ul), cumulative(ul))
        boards = lse(list(lv[-1].values()))
        terms = []
        for d in range(1, 257):
            w = sl * LN10 if d < 201 else 0.0
            for j, v in lv[d].items():
                x = v + w
                if bound and d < 256:
                    x += min(0.0, rl[d][len(ul) - j])
                terms.append(x)
        late = lse([v for d in range(201, 257) for v in lv[d].values()])
        return (lse(terms) - boards) / LN10, exp(late - lse(terms))

    for sl in (0.0, -2.0, -4.0, -6.0, -8.0):
        a, sha = cost(BW, sl)
        b, _ = cost(BW[1:], sl)
        ab, _ = cost(BW, sl, True)
        o10, q10 = descend(lambda ul: cost(ul, sl)[0], BW, 2, 255)
        o9, q9 = descend(lambda ul: cost(ul, sl)[0], BW[1:], 2, 255)
        ob, qb = descend(lambda ul: cost(ul, sl, True)[0], o10, 2, 255)
        boards = lse(list(E2.tree(10, lambda d: 10 if d >= 201 else 0)[-1].values()))
        orc = (lse([x + sl * LN10 for x in strict[1:201]] + [boards + log(56)]) - boards) / LN10
        print('s = 10^%-3.0f| Blackwood list: 10^%.2f (late share %.3g); 471 by truncation x10^%.2f; ideal reuse bound /10^%.2f' % (sl, a, sha, b - a, a - ab))
        print('           | best list %s: 10^%.2f (late share %.3g); best 471 list: x10^%.2f; best list with bound 10^%.2f (gain 10^%.2f); perfect oracle from 201: 10^%.2f' % (
            o10, q10, cost(o10, sl)[1], q9 - q10, qb, q10 - qb, orc))


if __name__ == '__main__':
    part1()
    part2()
    part3()
    part4()
