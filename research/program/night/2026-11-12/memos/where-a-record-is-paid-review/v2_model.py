#!/usr/bin/env python3
"""Referee script 2. The memo's model asked what the memo did not ask.

Run: python3 -I v2_model.py     (stdlib, about one minute)

Uses, read-only, partial-match-regime/model.py through the earlier review's
r3_sensitivity.py, exactly as the memo's a2/a4/a6 do.

 1  The two ways to cut one unlock (drop the first, drop the last): boards and nodes
    per depth-160 prefix, and the 471/470 cost ratio at the memo's measured split.
 2  The memo's cap "1 / (1 - late share)" for devices that act after depth 201,
    against the model's own oracle and best-list rows in head-heavy regimes.
 3  The published record engine does not row-scan its last 80 cells
    (EternityII_Solver/Util.cs, Get_Board_Order). Same model, his order.
 4  Independent joints (a4 P5, a6 parts 4 and 5) against the exact law.
"""
import importlib.util
import math
import os
import sys
from math import exp, lgamma, log

sys.dont_write_bytecode = True
H = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location('r3', os.path.join(H, '..', 'partial-match-regime-review', 'r3_sensitivity.py'))
r3 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r3)
M, LN10, lse, BW = r3.M, r3.LN10, r3.lse, r3.BW
L = lambda x: x / LN10
ROW = r3.E2


def per_depth(lv):
    return {d: lse(list(lv[d].values())) for d in lv}


def summary(ul, ci, cr, d0=160):
    lv = r3.tail_tree(d0, ul, ci, cr)
    p = per_depth(lv)
    head = lse([p[d] for d in range(d0 + 1, 201)])
    tail = lse([p[d] for d in range(201, 257)])
    return {'boards': p[256], 'head': head, 'tail': tail, 'arr': p[201], 'p': p}


print('== 1. One unlock fewer, two ways (row scan, per perfect depth-160 prefix, log10) ==')
print('   list: 10 = Blackwood ; 9a = first entry dropped (the memo) ; 9b = last entry dropped')
for ci, cr in ((17, 5), (16, 4), (15, 4), (14, 4)):
    s10, s9a, s9b = summary(BW, ci, cr), summary(BW[1:], ci, cr), summary(BW[:-1], ci, cr)
    print('   %2d + %d: boards 10: %.2f, 9a: %.2f (x10^%.2f), 9b: %.2f (x10^%.2f) ; nodes at depth >= 201: 10: %.2f, 9a: %.2f, 9b: %.2f' % (
        ci, cr, L(s10['boards']), L(s9a['boards']), L(s9a['boards'] - s10['boards']), L(s9b['boards']), L(s9b['boards'] - s10['boards']),
        L(s10['tail']), L(s9a['tail']), L(s9b['tail'])))
    # the memo's rule: cost ratio = (boards ratio)^-1 x (nodes ratio), with the measured split s = legal late share
    row = []
    for s in (0.08, 0.20, 0.46):
        out = []
        for nm, t in (('9a', s9a), ('9b', s9b)):
            node_ratio = (1 - s) + s * exp(t['tail'] - s10['tail'])
            out.append('%s x%.0f' % (nm, exp(s10['boards'] - t['boards']) * node_ratio))
        row.append('s=%.2f: %s' % (s, ', '.join(out)))
    print('          471 / 470 cost at a measured late share s (head nodes unchanged): ' + ' ; '.join(row))

print()
print('== 2. Is "1 / (1 - late share)" a cap on what a device after depth 201 can buy? Model rows, K = 9 and 10 ==')
for ci, cr, c0l in ((15, 4, 2.0), (15, 4, 3.0), (16, 4, 2.0), (16, 5, 2.0)):
    for K in (10, 9):
        ul = BW[10 - K:]
        c0 = c0l * LN10
        a = r3.tail_summary(160, ul, ci, cr, c0)
        free = r3.tail_tree(160, [201] * K, ci, cr)
        boards = lse(list(free[256].values()))
        pre = lse([c0] + [free[d][0] for d in range(161, 201)])
        orc = L(lse([pre, boards + log(56)]) - boards)
        cap = 1.0 / (1.0 - a['late_share']) if a['late_share'] < 1 else float('inf')
        print('   %2d + %d, c0 = 10^%.0f, K = %2d: late share %.3f, so the memo\'s cap is %.2f ; perfect knowledge from 201 buys x%.1f ; of which more boards per prefix x%.1f' % (
            ci, cr, c0l, K, a['late_share'], cap, 10 ** (a['q'] - orc), 10 ** (L(boards) - a['boards'])))

print()
print('== 3. The published order of the last 80 cells (rows 0..10 row scan, then five rows filled in columns) ==')
BO = [
    [196, 197, 198, 199, 200, 205, 210, 215, 220, 225, 230, 235, 243, 249, 254, 255],
    [191, 192, 193, 194, 195, 204, 209, 214, 219, 224, 229, 234, 242, 248, 252, 253],
    [186, 187, 188, 189, 190, 203, 208, 213, 218, 223, 228, 233, 241, 247, 250, 251],
    [181, 182, 183, 184, 185, 202, 207, 212, 217, 222, 227, 232, 240, 244, 245, 246],
    [176, 177, 178, 179, 180, 201, 206, 211, 216, 221, 226, 231, 236, 237, 238, 239],
]
seq = {}
for i, rowv in enumerate(BO):
    for col, s in enumerate(rowv):
        seq[s] = (15 - i, col)
for s in range(176):
    seq[s] = divmod(s, 16)
assert sorted(seq) == list(range(256)) and len(set(seq.values())) == 256
placed = set()
cells = []
njoint = []
for s in range(256):
    y, x = seq[s]
    bx, by = x in (0, 15), y in (0, 15)
    cls = 'c' if (bx and by) else ('e' if (bx or by) else 'i')
    joints = []
    for dy, dx in ((0, -1), (0, 1), (-1, 0), (1, 0)):
        if (y + dy, x + dx) in placed:
            joints.append(('r' if by else 'i') if dy == 0 else ('r' if bx else 'i'))
    placed.add((y, x))
    cells.append((cls, joints))
BWO = M.Board(16, [12] * 5, [24] * 5 + [25] * 12)
assert cells[:176] == BWO.cells[:176]
BWO.cells = cells
assert BWO.prefix_stats()[-1][1:] == ROW.prefix_stats()[-1][1:]
kinds = lambda c, a, b: ''.join(x[0] for x in c[a:b])
print('   cell classes 201..255, row scan      : ' + kinds(ROW.cells, 201, 256))
print('   cell classes 201..255, published order: ' + kinds(cells, 201, 256))
print('   border and corner cells after depth 239: row scan %d, published order %d' % (
    sum(1 for c in ROW.cells[240:] if c[0] != 'i'), sum(1 for c in cells[240:] if c[0] != 'i')))
print('   %-8s %-10s %10s %10s %12s %12s %14s %12s %10s' % ('colours', 'order', 'boards K10', 'boards K9', 'arrivals@201', 'tail nodes', 'late, c0=1e2', 'q K10', 'q K9'))
for ci, cr in ((17, 5), (16, 4), (15, 4), (14, 4)):
    for name, B in (('row scan', ROW), ('published', BWO)):
        r3.E2 = B
        s10, s9 = summary(BW, ci, cr), summary(BW[1:], ci, cr)
        a10 = r3.tail_summary(160, BW, ci, cr, 2 * LN10)
        a9 = r3.tail_summary(160, BW[1:], ci, cr, 2 * LN10)
        print('   %2d + %d   %-10s %10.2f %10.2f %12.2f %12.2f %14.3g %12.2f %10.2f' % (
            ci, cr, name, L(s10['boards']), L(s9['boards']), L(s10['arr']), L(s10['tail']), a10['late_share'], a10['q'], a9['q']))
        if (ci, cr) == (15, 4):
            p = s10['p']
            pk = max(range(201, 257), key=lambda d: p[d])
            print('             where a board is lost, per arrival: 201 -> peak (depth %d) %+.2f ; peak -> 240 %+.2f ; 240 -> 256 %+.2f' % (
                pk, L(p[pk] - p[201]), L(p[240] - p[pk]), L(p[256] - p[240])))
r3.E2 = ROW

print()
print('== 4. Independent joints against the exact law (perfect completions of a tail by ONE head, log10) ==')


def dfl(n):  # ln (n-1)!! for even n = number of perfect matchings of n points
    return lgamma(n + 1) - (n // 2) * log(2) - lgamma(n // 2 + 1)


for ci, cr in ((17, 5), (15, 4)):
    lv = r3.tail_tree(200, [], ci, cr)
    strip = L(lv[256][0])
    indep = (lgamma(36) + 35 * log(4) + lgamma(20) + log(2) - 87 * log(ci) - 22 * log(cr)) / LN10
    lv = r3.tail_tree(240, [], ci, cr)
    rim = L(lv[256][0])
    rim_ind = (lgamma(15) + log(2) - 15 * log(cr) - 14 * log(ci)) / LN10
    ncs = M.even_split(254, ci)
    pocket = (lgamma(57) + 56 * log(4) + sum(dfl(n) for n in ncs) - dfl(254)) / LN10
    pocket_ind = (lgamma(57) + 56 * log(4) - 127 * log(ci)) / LN10
    print('   %2d + %d: bottom strip from 200: exact %.1f, independent %.1f ; rim row from 240: exact %.1f, independent %.1f ; 7x8 pocket: exact %.1f, independent %.1f ; pocket minus strip: exact %.1f, independent %.1f' % (
        ci, cr, strip, indep, rim, rim_ind, pocket, pocket_ind, pocket - strip, pocket_ind - indep))

print()
print('== 5. The tail model\'s joint law against the price review\'s rv_lib (independent code), log10 P(first e joints all match) ==')
spec2 = importlib.util.spec_from_file_location('rv_lib', os.path.join(H, '..', 'price-and-ledger-review', 'rv_lib.py'))
rv = importlib.util.module_from_spec(spec2)
spec2.loader.exec_module(rv)
stats = ROW.prefix_stats()
ei = stats[-1][2] - stats[160][2]
er = stats[-1][1] - stats[160][1]
worst = 0.0
for ci, cr in ((17, 5), (15, 4), (14, 4)):
    counts = M.even_split(2 * ei, ci)
    law = M.JointLaw(M.interior_dist(counts))
    tab = rv.p_int_table(counts)
    rc = M.balanced(er, cr)
    rlaw = M.JointLaw(M.ring_dist(rc, rc))
    rtab = rv.p_frame_table(rc)
    row = []
    for e in (10, 40, 72, 120, ei):
        a, b = law.lspec(e, 0) / LN10, tab[e]
        worst = max(worst, abs(a - b))
        row.append('e=%d: %.3f / %.3f' % (e, a, b))
    rrow = []
    for f in (5, 12, er):
        a, b = rlaw.lspec(f, 0) / LN10, rtab[f]
        worst = max(worst, abs(a - b))
        rrow.append('f=%d: %.3f / %.3f' % (f, a, b))
    print('   %2d + %d (tail from 160: %d interior joints, %d ring joints): interior %s ; ring %s' % (ci, cr, ei, er, ' ; '.join(row), ' ; '.join(rrow)))
print('   largest difference: %.4f decades' % worst)
