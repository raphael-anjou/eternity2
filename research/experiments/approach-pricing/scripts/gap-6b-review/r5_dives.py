"""Review of gap 6b, script 5: a small version of the memo's experiment X2 on the
official pieces. Plain Knuth dives die at the clue corners (300 of 300 before
depth 32), so the sample is taken by randomised depth-first search with restarts,
five clues in place: every strict partial board the search visits at the listed
depths is tested (capped per restart and depth). This is a BIASED sample of each
level (boards a depth-first search reaches first, many of them siblings), good
enough to see whether a test fires at all, not to price it to a fraction of a bit.
Two tests on the cells that touch the filled region:
  FC   some frontier cell has no unused tile that fits its known sides
  HALL the frontier cells cannot be matched to distinct unused tiles
Question: how often does HALL refute a node that FC does not?
The scan runs from the last cell to the first so that the centre clue is cell 120,
as in the measured tree (widest level near depth 158).
Usage: python3 -I r5_dives.py <csv> <restarts> [seed] [node budget per restart] [cap per depth per restart]
"""
import sys, os, random
from collections import defaultdict
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import load
tiles, extra = load(sys.argv[1]); ND = int(sys.argv[2]); rng = random.Random(int(sys.argv[3]) if len(sys.argv) > 3 else 5)
RIM = 65535
def rot(t, r): return tuple(t[(j - r) % 4] for j in range(4))      # sides are up, right, down, left
PL = [(i, rot(t, r)) for i, t in enumerate(tiles) for r in range(4)]
byside = defaultdict(list)
for i, s in PL:
    for d in range(4): byside[(d, s[d])].append((i, s))
DIRS = ((-1, 0), (0, 1), (1, 0), (0, -1))
clue = {}
for i, (x, y, r) in enumerate(extra):
    if (x, y, r) != (0, 0, 0): clue[(y, x)] = (i, rot(tiles[i], r))
assert len(clue) == 5 and all(RIM not in s for _, s in clue.values())
order = [(y, x) for y in range(15, -1, -1) for x in range(15, -1, -1)]
assert order.index((8, 7)) == 120 and (8, 7) in clue
DEPTHS = (64, 96, 112, 128, 144, 152, 160, 168, 176)

def constraints(board, y, x):
    cons = []
    for d, (dy, dx) in enumerate(DIRS):
        yy, xx = y + dy, x + dx
        if not (0 <= yy < 16 and 0 <= xx < 16): cons.append((d, RIM, True))
        elif (yy, xx) in board: cons.append((d, board[(yy, xx)][1][(d + 2) % 4], True))
        else: cons.append((d, RIM, False))        # must NOT be rim
    return cons
def fits(board, used, y, x):
    cons = constraints(board, y, x)
    eq = [(d, c) for d, c, must in cons if must]
    if eq:
        d0, c0 = min(eq, key=lambda e: len(byside[e]))
        cand = byside[(d0, c0)]
    else: cand = PL
    out = []
    for i, s in cand:
        if i in used: continue
        ok = True
        for d, c, must in cons:
            if (s[d] == c) != must: ok = False; break
        if ok: out.append((i, s))
    return out
def hall(doms):
    match = {}
    def aug(u, seen):
        for v in doms[u]:
            if v in seen: continue
            seen.add(v)
            if v not in match or aug(match[v], seen):
                match[v] = u; return True
        return False
    return sum(1 for u in sorted(range(len(doms)), key=lambda u: len(doms[u])) if aug(u, set()))

BUDGET = int(sys.argv[4]) if len(sys.argv) > 4 else 150000
CAP = int(sys.argv[5]) if len(sys.argv) > 5 else 100
stat = {d: dict(n=0, fc=0, hall=0, mind=0, front=0, restarts=0) for d in DEPTHS}
import time
T0 = time.time(); WALL = 540      # stop starting new restarts after 9 minutes
deepest = []
for restart in range(ND):
    if time.time() - T0 > WALL: break
    board = dict(clue); used = {i for i, _ in clue.values()}
    seen = {d: 0 for d in DEPTHS}; nodes = 0; maxd = 0
    stack = []          # (depth, remaining candidates, placed cell or None)
    depth = 0
    def test(depth):
        st = stat[depth]
        if seen[depth] >= CAP: return
        if seen[depth] == 0: st["restarts"] += 1
        seen[depth] += 1; st["n"] += 1
        front = [c for c in order[depth:] if c not in board and any((c[0] + dy, c[1] + dx) in board for dy, dx in DIRS)]
        doms = [sorted({i for i, _ in fits(board, used, *c)}) for c in front]
        st["front"] += len(front); st["mind"] += min(len(d) for d in doms)
        if any(not d for d in doms): st["fc"] += 1
        elif hall(doms) < len(doms): st["hall"] += 1
    # iterative depth-first search
    def cands(depth):
        y, x = order[depth]
        if (y, x) in clue: return None
        f = fits(board, used, y, x); rng.shuffle(f); return f
    frames = [cands(0)]; placed = [None]
    while frames and nodes < BUDGET:
        d = len(frames) - 1
        f = frames[-1]
        if placed[-1] is not None:
            c = placed[-1]; used.discard(board[c][0]); del board[c]; placed[-1] = None
        if f is None:
            if d + 1 in stat and not getattr(f, "x", False): pass
            # clue cell: pass through once
            frames[-1] = []
            nd = d + 1
        elif f:
            i, s = f.pop(); c = order[d]; board[c] = (i, s); used.add(i); placed[-1] = c
            nd = d + 1
        else:
            frames.pop(); placed.pop(); continue
        nodes += 1; maxd = max(maxd, nd)
        if nd in stat: test(nd)
        if nd >= 200 or nd >= 256:
            continue
        frames.append(cands(nd)); placed.append(None)
    deepest.append(maxd)
deepest.sort()
print(f"restarts {len(deepest)} of {ND} asked, node budget {BUDGET} each, cap {CAP} boards per depth per restart; deepest level reached: median {deepest[len(deepest)//2]}, min {deepest[0]}, max {deepest[-1]}")
print("depth  boards tested (restarts reaching)  mean frontier cells  mean smallest domain  refuted by FC  refuted by HALL only")
for d in DEPTHS:
    st = stat[d]
    if st["n"] == 0: print(f"{d:4d}   0"); continue
    print(f"{d:4d}   {st['n']:6d} ({st['restarts']:4d})                {st['front']/st['n']:6.1f}             {st['mind']/st['n']:6.2f}        {st['fc']:6d} ({st['fc']/st['n']:.3f})   {st['hall']:6d} ({st['hall']/st['n']:.4f})")
print("FC is what any forward-checking search already has. HALL only is what a matching test adds on top of it.")
