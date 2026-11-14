"""Two checks on the real tiles. Stdlib only. Run: python3 -I r_twins.py <official.json>
(1) Non-local merges: two tile-disjoint fillings of the same small shape with the same outline can be
    exchanged between two distant places of a partial board. Same used set, same frontier, so it is a
    merge that no window of the board sees. Census of such twin pairs per shape, and the first-moment
    rate m-1 per pair of slots (uniform over fillings of each slot).
(2) Lemma B: a 0/1 allowed-set term whose perfect matchings are all valid fillings but which does not
    fix the colour of an internal edge."""
import json, sys
from itertools import permutations
from collections import defaultdict
import os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))   # -I drops the script folder
import r_merge as M

R = M.R
R["mixed 2x1"] = M.region([(0, 0), (1, 0)], [((1, 0), 2)])          # interior tile above an edge piece
R["corner+edge L"] = M.region([(0, 0), (0, 1)], [((0, 0), 2), ((0, 0), 3), ((0, 1), 2)])
R["rim 1x3"] = R["rim 1x3"]

def census(name, isometries):
    cells, total, classes = M.enumerate_region(name, keep_all=True)
    by_out = defaultdict(list)
    for (tiles, outline), fills in classes.items():
        for f in fills: by_out[outline].append(frozenset(tiles))
    twins = shared = 0
    for outline, sets in by_out.items():
        for a in range(len(sets)):
            for b in range(a + 1, len(sets)):
                if sets[a].isdisjoint(sets[b]): twins += 1
                elif sets[a] != sets[b]: shared += 1
    rate = isometries * twins / total ** 2
    print("%-14s fillings %6d  outlines seen %6d  tile-disjoint twin pairs %5d  (sharing a tile: %d)  m-1 per pair of slots %.2e"
          % (name, total, len(by_out), twins, shared, rate))
    return rate

print("(1) twin census")
r_rim = census("rim 1x2", 1)
r_int = census("interior 1x2", 2)
r_mix = census("mixed 2x1", 1)
r_cor = census("corner+edge L", 1)
r_tri = census("rim 1x3", 1)

# slots present in a row-scan partial board at depth 158 (rim row 0 full, 8 interior rows full, 14 cells of the next)
rows_full = 9; part = 14
rim_slots = 13 + 2 * (rows_full - 1)            # edge-edge dominoes: bottom row, left and right columns
int_h = 13 * (rows_full - 1) + (part - 2); int_v = 14 * (rows_full - 2) + (part - 1)
mix_slots = 14 + 2 * (rows_full - 1)
pairs = lambda s: s * (s - 1) // 2 - 2 * s      # crude: drop overlapping and touching pairs
est = [("rim dominoes", rim_slots, r_rim), ("interior dominoes", int_h + int_v, r_int), ("mixed dominoes", mix_slots, r_mix)]
tot = 0
for nm, s, r in est:
    c = pairs(s) * r; tot += c
    print("  %-18s slots %4d  slot pairs %6d  contribution to m-1: %.2e" % (nm, s, pairs(s), c))
from math import log2
print("  non-local exchange total m-1 about %.2e ; with the memo's local 4e-3: m-1 %.2e, log2 m = %.3f bit"
      % (tot, tot + 4e-3, log2(1 + tot + 4e-3)))

print("\n(2) Lemma B counterexample on the bottom-rim 1x3 region, tiles 6, 7, 51")
T = [t[2:] + t[:2] for t in M.T]                # half turn: grey side down, as placed on the bottom rim
for i in (6, 7, 51): print("   tile %d placed URDL %s" % (i, T[i]))
S = [{6, 7}, {7, 51}, {51, 6}]                  # allowed tiles per cell
found = []
for perm in permutations((6, 7, 51)):
    if all(perm[c] in S[c] for c in range(3)):
        valid = all(T[perm[c]][1] == T[perm[c + 1]][3] for c in range(2))
        outline = (T[perm[0]][3], T[perm[2]][1]) + tuple(T[perm[c]][0] for c in range(3))
        found.append((perm, valid, outline, [T[perm[c]][1] for c in range(2)]))
for p in found: print("   perfect matching %s valid %s outline (L,R,ups) %s internal edge colours %s" % p)
assert len(found) == 2 and all(p[1] for p in found) and found[0][2] == found[1][2] and found[0][3] != found[1][3]
print("   the term has permanent 2, counts only valid fillings of one outline, and its first internal edge is not one colour")
print("   allowed pair (6 at cell 0, 51 at cell 1) compatible: %s  (allowed, incompatible, in no perfect matching)" % (T[6][1] == T[51][3]))
