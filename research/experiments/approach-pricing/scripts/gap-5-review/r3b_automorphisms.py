"""Second analysis script 3b. Colour automorphisms of the official set by exhaustive backtracking (not by
colour refinement): permutations pi of the 23 colours (grey fixed) such that the image of every
tile, up to rotation, is a tile of the set; and the same after mirroring every tile.
Control: a set built to have a colour swap must report it.
Run: python3 -I r3b_automorphisms.py
"""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rcommon import load, rots


def canon(t):
    return min(rots(t))


def automorphisms(tiles, mirror=False, limit=10):
    S = {canon(t) for t in tiles}
    src = [(t[0], t[3], t[2], t[1]) if mirror else t for t in tiles]
    cols = sorted({c for t in tiles for c in t})
    cnt = {c: sum(t.count(c) for t in tiles) for c in cols}
    # order colours so that tiles become fully assigned early
    order = [0] + sorted((c for c in cols if c), key=lambda c: -cnt[c])
    by_last = {c: [] for c in order}
    pos = {c: i for i, c in enumerate(order)}
    for t in src:
        by_last[max(t, key=lambda c: pos[c])].append(t)
    pi = {}; used = set(); out = []; nodes = [0]

    def rec(j):
        if len(out) >= limit:
            return
        if j == len(order):
            out.append(dict(pi)); return
        c = order[j]
        for d in ([0] if c == 0 else [x for x in cols if x and x not in used and cnt[x] == cnt[c]]):
            nodes[0] += 1
            pi[c] = d; used.add(d)
            if all(canon(tuple(pi[x] for x in t)) in S for t in by_last[c]):
                rec(j + 1)
            used.discard(d); del pi[c]
    rec(0)
    return out, nodes[0]


if __name__ == "__main__":
    P, _ = load()
    for mirror in (False, True):
        auts, nodes = automorphisms(P, mirror)
        nontriv = [a for a in auts if any(k != v for k, v in a.items())]
        print(f"official set, mirror={mirror}: colour maps preserving the set: {len(auts)} "
              f"(non-trivial: {len(nontriv)}), search nodes {nodes}")
    # control: take the official set and add, for each tile, its image under the swap 6 <-> 7
    sw = {c: c for c in range(23)}; sw[6], sw[7] = 7, 6
    ctl = list({canon(t) for t in P} | {canon(tuple(sw[c] for c in t)) for t in P})
    auts, nodes = automorphisms(ctl)
    print(f"control (official set united with its image under the swap of colours 6 and 7, "
          f"{len(ctl)} tiles): maps found {len(auts)}, the swap among them: {sw in auts}")
    ctl2 = list({canon(t) for t in P} | {canon((t[0], t[3], t[2], t[1])) for t in P})
    auts, nodes = automorphisms(ctl2, mirror=True)
    print(f"control (official set united with its mirror image, {len(ctl2)} tiles): mirror maps found "
          f"{len(auts)}, identity among them: {any(all(k == v for k, v in a.items()) for a in auts)}")
