#!/usr/bin/env python3
"""Directed count of Eternity II frame rings (stdlib only).

Border pieces cannot flip and keep their gray side facing out, so walking the
ring clockwise traverses every border piece in one fixed direction: from the
colour on the slot before its gray run to the colour on the slot after it.
The ring is therefore a directed multigraph on the 5 frame colours, and the
BEST theorem counts its Euler circuits exactly. A valid frame additionally
needs the four corners 15 positions apart; that fraction is estimated by
sampling uniform Euler circuits (Wilson's algorithm + the BEST bijection).

Usage: python3 ring_count.py [samples] > ../results/ring_count.json
"""
import json, math, os, random, sys
from multiprocessing import Pool

HERE = os.path.dirname(os.path.abspath(__file__))
CSV = os.path.join(HERE, "../../../../engine/data/official_eternity2.csv")
GRAY = 65535


def load():
    rows = open(CSV).read().split("\n")
    assert int(rows[0]) == 16
    pieces = []
    for line in rows[1:257]:
        f = line.split(",")
        pieces.append([int(x, 2) for x in f[:4]])
    return pieces


def ring_edges(pieces):
    edges = []  # (from_colour, to_colour, is_corner)
    for p in pieces:
        g = [i for i in range(4) if p[i] == GRAY]
        if len(g) == 1:
            z = g[0]
            edges.append((p[(z + 3) % 4], p[(z + 1) % 4], False))
        elif len(g) == 2:
            a = g[0] if (g[0] + 1) % 4 == g[1] else g[1]
            assert p[(a + 1) % 4] == GRAY
            edges.append((p[(a + 3) % 4], p[(a + 2) % 4], True))
    assert len(edges) == 60
    return edges


def det(m):
    m = [row[:] for row in m]
    n, d = len(m), 1.0
    for c in range(n):
        piv = max(range(c, n), key=lambda r: abs(m[r][c]))
        if abs(m[piv][c]) < 1e-12:
            return 0.0
        if piv != c:
            m[c], m[piv] = m[piv], m[c]
            d = -d
        d *= m[c][c]
        for r in range(c + 1, n):
            k = m[r][c] / m[c][c]
            for j in range(c, n):
                m[r][j] -= k * m[c][j]
    return d


def setup():
    edges = ring_edges(load())
    cols = sorted({e[0] for e in edges} | {e[1] for e in edges})
    vid = {c: i for i, c in enumerate(cols)}
    E = [(vid[a], vid[b], corner) for a, b, corner in edges]
    return cols, E


def sample_batch(args):
    seed, n = args
    cols, E = setup()
    V = len(cols)
    rng = random.Random(seed)
    out = [[i for i, e in enumerate(E) if e[0] == v] for v in range(V)]
    root = 0
    e0 = out[root][0]  # fixed first edge: circuits are counted up to rotation
    hits = 0
    for _ in range(n):
        # Wilson: uniform in-arborescence toward root (each v != root picks its
        # last exit edge). Uniform over labelled edges since out-degrees are equal.
        in_tree = [False] * V
        in_tree[root] = True
        nxt = [None] * V
        for s in range(V):
            u = s
            while not in_tree[u]:
                nxt[u] = rng.choice(out[u])
                u = E[nxt[u]][1]
            u = s
            while not in_tree[u]:
                in_tree[u] = True
                u = E[nxt[u]][1]
        order = []
        for v in range(V):
            if v == root:
                rest = [i for i in out[v] if i != e0]
                rng.shuffle(rest)
                order.append([e0] + rest)
            else:
                rest = [i for i in out[v] if i != nxt[v]]
                rng.shuffle(rest)
                order.append(rest + [nxt[v]])
        ptr = [0] * V
        u, seq = root, []
        for _ in range(60):
            i = order[u][ptr[u]]
            ptr[u] += 1
            seq.append(i)
            u = E[i][1]
        assert len(seq) == 60 and u == root
        pos = sorted(k for k, i in enumerate(seq) if E[i][2])
        if all(pos[j] - pos[0] == 15 * j for j in range(4)):
            hits += 1
    return hits


def main():
    samples = int(sys.argv[1]) if len(sys.argv) > 1 else 4_000_000
    cols, E = setup()
    V = len(cols)
    outdeg = [sum(1 for e in E if e[0] == v) for v in range(V)]
    indeg = [sum(1 for e in E if e[1] == v) for v in range(V)]
    loops = sum(1 for e in E if e[0] == e[1])
    L = [[0.0] * V for _ in range(V)]
    for a, b, _ in E:
        if a != b:
            L[a][a] += 1
            L[a][b] -= 1
    trees = round(det([row[1:] for row in L[1:]]))
    log10_ec = math.log10(trees) + sum(math.lgamma(d) for d in outdeg) / math.log(10)
    workers = os.cpu_count() or 1
    per = max(1, -(-samples // workers))
    with Pool(workers) as pool:
        hits = sum(pool.map(sample_batch, [(1000 + w, per) for w in range(workers)]))
    n = per * workers
    frac = hits / n
    se = math.sqrt(frac * (1 - frac) / n)
    uniform_frac = 1 / math.comb(59, 3)
    log10_naive = (math.lgamma(5) + math.lgamma(57)) / math.log(10) + 60 * math.log10(0.2)
    res = {
        "model": "directed ring multigraph, clockwise traversal, frame colours as vertices",
        "frame_colours": cols,
        "out_degree": outdeg,
        "in_degree": indeg,
        "self_loops": loops,
        "arborescences": trees,
        "log10_euler_circuits_BEST": log10_ec,
        "corner_spacing": {
            "samples": n,
            "hits": hits,
            "fraction": frac,
            "std_error": se,
            "uniform_position_fraction": uniform_frac,
        },
        "log10_valid_frame_placements": math.log10(4 * frac) + log10_ec if hits else None,
        "log10_naive_first_moment_4f_56f_0.2pow60": log10_naive,
    }
    print(json.dumps(res, indent=2))


if __name__ == "__main__":
    main()
