#!/usr/bin/env python3
"""Referee script 4. Small extras. Run: python3 -I r4_extras.py (seconds, stdlib only)

 a  Border pieces of the official set whose two ring sides share a colour (Verhaard's
    flip: such a piece turned end for end gives a 479 board from any 480 board, with
    its grey side facing inward). Bears on "zero at 479".
 b  The memo's looseness estimate, 0.43 r / log10(rho E / b), evaluated on the small
    boards of its own table instead of the rounded 0.19 per cell.
 c  Score histogram of a fixed list in the fewer-colour tail (model).
"""
import os
import sys
from math import factorial, log10

sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from r1_counts import read_pieces

pieces = read_pieces()
same = 0
for e in pieces:
    g = [i for i in range(4) if e[i] == 0]
    if len(g) == 1:
        i = g[0]
        if e[(i + 1) % 4] == e[(i + 3) % 4]:
            same += 1
print('a. border pieces with equal ring sides: %d of 56' % same)


def reuse_ratio(n, rows):
    """log10 of prod over classes r^r / r! for the last `rows` rows of an n x n board."""
    cells = {'c': 0, 'e': 0, 'i': 0}
    for y in range(n - rows, n):
        for x in range(n):
            bx, by = x in (0, n - 1), y in (0, n - 1)
            cells['c' if bx and by else ('e' if bx or by else 'i')] += 1
    return sum(log10(r ** r / factorial(r)) for r in cells.values() if r), cells


print('b. looseness estimate on the small boards of the memo (section 4.1 table)')
for n, ci, rows, b, meas in ((6, 5, 2, 5, 2.75), (6, 5, 3, 4, 2.75), (7, 6, 2, 6, 2.20), (16, 17, 4, 10, None)):
    lr, cells = reuse_ratio(n, rows)
    joints = sum((1 if x > 0 else 0) + 1 for y in range(n - rows, n) for x in range(n))
    rho = ci - 1
    per_break = log10(rho * joints / b)
    print('   %dx%d, %d rows left (%d cells, %d joints): reuse/distinct = 10^%.2f (%.2f per cell); one break = 10^%.2f; estimate %.1f breaks; measured mean %s' % (
        n, n, rows, sum(cells.values()), joints, lr, lr / sum(cells.values()), per_break, lr / per_break, meas))

import importlib.util
spec = importlib.util.spec_from_file_location('r3', os.path.join(HERE, 'r3_sensitivity.py'))
r3 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r3)
from math import exp
print('c. fixed Blackwood list, tail below a perfect depth-160 prefix: factor in boards per point lost (model)')
for ci, cr in ((17, 5), (15, 4), (13, 4), (11, 3)):
    lv = r3.tail_tree(160, r3.BW, ci, cr)[256]
    print('   %2d + %d colours: ' % (ci, cr) + ' '.join('%d->%d: x%.0f' % (480 - j + 1, 480 - j, exp(lv[j] - lv[j - 1])) for j in range(7, 11)))
