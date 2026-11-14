"""Review of gap 6b, script 4b: a harder search for a region with fewer strict
partial boards than the row-scan prefix of the same size (five clues, real-count
ledger and the memo's uniform ledger). Incremental annealing, several starts,
plus hand-built families (rectangles, interior blocks, frame ring plus rows).
Usage: python3 -I r4b_region_min.py [seed]
"""
import sys, os, random
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *
PIr, PFr, PIu, PFu = tables()
cells = [(r, c) for r in range(16) for c in range(16)]
idx = {x: i for i, x in enumerate(cells)}
KIND = [kind(*x) for x in cells]
ISCLUE = [x in CLUES for x in cells]
NB = [[(idx[(x[0] + dr, x[1] + dc)], KIND[idx[x]] > 0 and kind(x[0] + dr, x[1] + dc) > 0) for dr, dc in ((1, 0), (-1, 0), (0, 1), (0, -1)) if 0 <= x[0] + dr < 16 and 0 <= x[1] + dc < 16] for x in cells]
LF4 = [lfall(4, i) for i in range(5)]; LF56 = [lfall(56, i) for i in range(57)]; LF191 = [lfall(191, i) for i in range(192)]

class State:
    def __init__(self, S, PI, PF):
        self.PI, self.PF = PI, PF
        self.inS = [False] * 256; self.n = [0, 0, 0]; self.ni = 0; self.eff = 0; self.ein = 0
        for i in S: self.add(i)
    def add(self, i):
        for j, ff in NB[i]:
            if self.inS[j]:
                if ff: self.eff += 1
                else: self.ein += 1
        self.inS[i] = True; self.n[KIND[i]] += 1
        if KIND[i] == 0 and not ISCLUE[i]: self.ni += 1
    def rem(self, i):
        self.inS[i] = False; self.n[KIND[i]] -= 1
        if KIND[i] == 0 and not ISCLUE[i]: self.ni -= 1
        for j, ff in NB[i]:
            if self.inS[j]:
                if ff: self.eff -= 1
                else: self.ein -= 1
    def val(self):
        return LF4[self.n[2]] + LF56[self.n[1]] + LF191[self.ni] + 2 * self.ni + self.PI[self.ein] + self.PF[self.eff]
    def open(self):
        return sum(1 for i in range(256) if self.inS[i] for j, _ in NB[i] if not self.inS[j])

def anneal(start, PI, PF, rng, steps, T0):
    st = State(start, PI, PF); inn = list(start); out = [i for i in range(256) if i not in set(start)]
    cur = st.val(); best = cur; bestS = list(inn)
    for t in range(steps):
        T = T0 * (1 - t / steps) + 0.05
        a = rng.randrange(len(inn)); b = rng.randrange(len(out))
        i, j = inn[a], out[b]
        st.rem(i); st.add(j); v = st.val()
        if v <= cur or rng.random() < 2 ** ((cur - v) / T):
            cur = v; inn[a], out[b] = j, i
            if v < best - 1e-9: best = v; bestS = list(inn)
        else:
            st.rem(j); st.add(i)
    return best, bestS

def show(S):
    s = set(S)
    return "\n".join("    " + "".join("#" if idx[(r, c)] in s else ("c" if (r, c) in CLUES else ".") for c in range(16)) for r in range(16))

if __name__ == "__main__":
    rng = random.Random(int(sys.argv[1]) if len(sys.argv) > 1 else 7)
    for lname, PI, PF in (("real counts", PIr, PFr), ("uniform (memo)", PIu, PFu)):
        prof = [State(range(d), PI, PF).val() for d in range(1, 257)]
        pk = max(prof)
        print(f"== {lname}: row-scan peak level 2^{pk:.2f} at depth {prof.index(pk)+1}")
        floor = []
        for k in range(8, 249, 8):
            cands = []
            pre = list(range(k)); cands.append(("row-scan prefix", pre))
            # hand families
            inter = [i for i in range(256) if KIND[i] == 0]
            if k <= 196: cands.append(("interior cells only, row order", inter[:k]))
            ring = [i for i in range(256) if KIND[i] > 0]
            if k >= 60: cands.append(("frame ring plus interior rows", ring + inter[:k - 60]))
            cands.append(("column-major prefix", [idx[(r, c)] for c in range(16) for r in range(16)][:k]))
            cands.append(("bottom-up prefix", list(range(255, 255 - k, -1))))
            sq = sorted(range(256), key=lambda i: (max(cells[i]), cells[i]))[:k]; cands.append(("corner square", sq))
            best = None
            for nm, S in cands:
                v = State(S, PI, PF).val()
                if best is None or v < best[0]: best = (v, nm, S)
            for trial in range(4):
                start = best[2] if trial < 2 else rng.sample(range(256), k)
                v, S = anneal(start, PI, PF, rng, 150000, 4.0 if trial % 2 else 1.0)
                if v < best[0] - 1e-9: best = (v, "annealed", S)
            st = State(best[2], PI, PF)
            floor.append((k, prof[k - 1], best[0], best[1], st.open(), tuple(st.n), best[2]))
            print(f"  k={k:3d} row-scan prefix {prof[k-1]:7.2f}  best found {best[0]:7.2f} ({best[1]}, open edges {st.open()}, interior/edge/corner cells {st.n[0]}/{st.n[1]}/{st.n[2]})  gain {prof[k-1]-best[0]:+6.2f}", flush=True)
        m = max(floor, key=lambda x: x[2])
        print(f"  bottleneck estimate: max over k of the best value found = 2^{m[2]:.2f} at k={m[0]} ({m[3]}); row-scan peak on the same grid of k: 2^{max(f[1] for f in floor):.2f}")
        for k, _, v, nm, _, _, S in floor:
            if k in (160, 192, 216) : print(f"  best set at k={k} ({nm}, 2^{v:.2f}):\n" + show(S))
