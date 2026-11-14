"""Review of gap 6b, script 1: the official pieces.
Checks (a) that a four-side colour pattern identifies one tile and one rotation,
over ALL 256 tiles (the memo's script only looks at the 196 interior tiles),
(b) three-side and two-side ownership, (c) the real colour counts.
Usage: python3 -I r1_tiles.py <size_16_official_eternity.csv>
"""
import sys
from collections import Counter, defaultdict

def load(p):
    rows = [l.strip().split(",") for l in open(p) if "," in l]
    tiles = [tuple(int(x, 2) for x in r[:4]) for r in rows]
    extra = [r[4:] for r in rows]
    return tiles, extra

if __name__ == "__main__":
    tiles, extra = load(sys.argv[1])
    RIM = max(c for t in tiles for c in t)
    print("tiles", len(tiles), "rim code", RIM)
    kinds = Counter(sum(1 for c in t if c == RIM) for t in tiles)
    print("tiles by number of rim sides", dict(kinds))
    print("rows with a non-zero trailing field (clues):", [(i, e) for i, e in enumerate(extra) if any(x not in ("0", "") for x in e)])
    # (a) uniqueness over all tiles and all four rotations
    owner = defaultdict(set)
    sym = 0
    for i, t in enumerate(tiles):
        rots = {t[k:] + t[:k] for k in range(4)}
        if len(rots) < 4: sym += 1
        for r in rots: owner[r].add(i)
    shared = [p for p, s in owner.items() if len(s) > 1]
    print(f"all tiles: oriented patterns {len(owner)} (1024 if none is rotation-symmetric); tiles with a rotational symmetry {sym}; patterns owned by 2+ tiles {len(shared)}")
    # as unordered necklaces (the puzzle allows rotation, not reflection); also mirror twins
    mirror = 0
    canon = lambda t: min(t[k:] + t[:k] for k in range(4))
    cset = Counter(canon(t) for t in tiles)
    for t in tiles:
        m = canon(tuple(reversed(t)))
        if m in cset and m != canon(t): mirror += 1
    print(f"distinct tiles up to rotation {len(cset)}; tiles whose mirror image is another tile {mirror}")
    # (b) partial patterns on interior tiles
    inner = [t for t in tiles if RIM not in t]
    place = [(i, t[k:] + t[:k]) for i, t in enumerate(inner) for k in range(4)]
    for name, idx in (("3 sides", (0, 1, 2)), ("2 adjacent", (0, 1)), ("2 opposite", (0, 2))):
        d = defaultdict(set)
        for i, p in place: d[tuple(p[j] for j in idx)].add(i)
        sizes = Counter(len(s) for s in d.values())
        print(f"interior, {name}: distinct patterns {len(d)}, by number of owning tiles {dict(sorted(sizes.items()))}")
    # tiles with a repeated colour
    rep = Counter(4 - len(set(t)) for t in inner)
    print("interior tiles by (4 - distinct colours):", dict(sorted(rep.items())))
    # (c) real counts
    fcol = Counter(); icol_int = Counter(); icol_frame = Counter()
    for t in tiles:
        nr = sum(1 for c in t if c == RIM)
        if nr == 0:
            for c in t: icol_int[c] += 1
        else:
            k = [j for j in range(4) if t[j] == RIM]
            for j in range(4):
                if t[j] == RIM: continue
                # a side adjacent (cyclically) to a rim side on an edge tile is a frame-frame side; the side opposite the rim faces inward
                if nr == 2: fcol[t[j]] += 1
                elif (j - k[0]) % 4 == 2: icol_frame[t[j]] += 1
                else: fcol[t[j]] += 1
    print("frame-colour half-edges", sorted(fcol.items()), "total", sum(fcol.values()))
    print("interior-colour half-edges on interior tiles", sorted(icol_int.items()), "total", sum(icol_int.values()))
    print("interior-colour half-edges on frame inward sides", sorted(icol_frame.items()), "total", sum(icol_frame.values()))
    tot = icol_int + icol_frame
    print("interior colours, total half-edges", sorted(tot.items()))
    print("all totals even:", all(v % 2 == 0 for v in tot.values()) and all(v % 2 == 0 for v in fcol.values()))
    print("frame colours and interior colours disjoint:", not (set(fcol) & set(tot)))
