"""Probe other order families under the bulge model (official 16x16, five clues)."""
import sys, os, random
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from bulge import *
n, pieces, hints = load_official()
bt, mt = histograms(pieces)
allh = [(h[0], h[1]) for h in hints]
m5 = Model(16, 16, bt, mt, 5)
def tot(o):
    w = m5.profile(o, allh); return L(sum(w)), L(max(w))
allc = [(r, c) for r in range(16) for c in range(16)]
fam = {}
fam["row scan from bottom"] = rowscan(16, 16)
fam["anti-diagonal from bottom-left"] = sorted(allc, key=lambda p: ((15 - p[0]) + p[1], p[1]))
fam["corner squares (L shells) from bottom-left"] = sorted(allc, key=lambda p: (max(15 - p[0], p[1]), p[1], -p[0]))
box = [(r, c) for r in range(13, 1, -1) for c in range(2, 14)]
fam["12x12 clue box first, then ring"] = box + [p for p in rowscan(16, 16) if p not in box]
box2 = [(r, c) for r in range(15, 1, -1) for c in range(0, 16)]
fam["rows 15..2 (same as row scan)"] = box2 + [p for p in rowscan(16, 16) if p not in box2]
fam["bottom 3 rows, top 3 rows, then middle upward"] = [p for p in rowscan(16,16) if p[0] >= 13] + [p for p in rowscan(16,16,False) if p[0] <= 2] + [p for p in rowscan(16,16) if 2 < p[0] < 13]
fam["frame first, then row scan"] = [p for p in rowscan(16,16) if m5.kind(*p)] + [p for p in rowscan(16,16) if not m5.kind(*p)]
fam["2-row bands from bottom, column-major inside band"] = [(15 - 2*b - k, c) for b in range(8) for c in range(16) for k in range(2)]
fam["4-row bands, column-major inside band"] = [(15 - 4*b - k, c) for b in range(4) for c in range(16) for k in range(4)]
fam["8-row bands, column-major inside band"] = [(15 - 8*b - k, c) for b in range(2) for c in range(16) for k in range(8)]
for k, o in fam.items():
    assert len(set(o)) == 256
    t, p = tot(o)
    print(f"{k:52s} total 10^{t:6.2f} peak 10^{p:6.2f}")
# hill climb: adjacent transpositions and block moves on the row scan, 60 s cap
import time
random.seed(1)
cur = rowscan(16, 16); ct = tot(cur)[0]; t0 = time.time(); it = 0
while time.time() - t0 < 60:
    it += 1
    i = random.randrange(256); j = max(0, min(255, i + random.randint(-40, 40)))
    if i == j: continue
    o = cur[:]; x = o.pop(i); o.insert(j, x)
    t = tot(o)[0]
    if t < ct - 1e-9:
        cur, ct = o, t
print(f"hill climb (single-cell moves, {it} proposals, 60 s): total 10^{ct:.2f}")
print("order:", cur)
