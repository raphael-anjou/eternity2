"""Exact row-major enumeration on small boards against the bulge model.
Part A: boards generated here (4x4, 5x5, 6x6). Part B: the committed
hard-ladder 6x6 instances and their committed hp-dfs node counts.
Usage: python3 -I small_check.py [A|B|pred]"""
import sys, os, json, random, math, time, statistics
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from bulge import *
sys.setrecursionlimit(10000)
LAD = os.path.join(ROOT, "research", "experiments", "hard-ladder")


def gen(n, fc, ic, rng):
    """Framed planted puzzle: fc colours on border joins, ic on middle joins,
    uniform at random; pieces shuffled and rotated."""
    H = [[0] * (n - 1) for _ in range(n)]  # H[r][c] join between (r,c),(r,c+1)
    V = [[0] * n for _ in range(n - 1)]
    for r in range(n):
        for c in range(n - 1):
            H[r][c] = rng.randint(1, fc) if r in (0, n - 1) else fc + rng.randint(1, ic)
    for r in range(n - 1):
        for c in range(n):
            V[r][c] = rng.randint(1, fc) if c in (0, n - 1) else fc + rng.randint(1, ic)
    ps = []
    for r in range(n):
        for c in range(n):
            p = (V[r - 1][c] if r else 0, H[r][c] if c < n - 1 else 0, V[r][c] if r < n - 1 else 0, H[r][c - 1] if c else 0)
            k = rng.randrange(4)
            ps.append(p[k:] + p[:k])
    rng.shuffle(ps)
    return ps


def rot(p, k):  # k clockwise quarter turns of (top,right,bottom,left)
    k %= 4
    return p[-k:] + p[:-k] if k else p


def enumerate_tree(n, pieces, cap=None, stop_first=False):
    """Row-major DFS. One node per (cell, piece, rotation) that agrees with the
    rim and the placed neighbours. Returns (nodes per depth, solutions,
    nodes at first solution, complete?)."""
    idx = {}
    for i, p in enumerate(pieces):
        for k in range(4):
            q = rot(p, k)
            idx.setdefault((q[3], q[0]), []).append((i, q[1], q[2]))
    N = n * n
    per = [0] * (N + 1)
    used = [False] * len(pieces)
    right = [0] * N
    bottom = [0] * N
    st = {"sol": 0, "first": None, "nodes": 0, "ok": True}

    def go(pos):
        r, c = divmod(pos, n)
        left = right[pos - 1] if c else 0
        top = bottom[pos - n] if r else 0
        lastc, lastr = c == n - 1, r == n - 1
        for (i, rt, bt_) in idx.get((left, top), ()):
            if used[i] or (rt == 0) != lastc or (bt_ == 0) != lastr:
                continue
            per[pos + 1] += 1
            st["nodes"] += 1
            if cap and st["nodes"] > cap:
                st["ok"] = False
                return True
            if pos + 1 == N:
                st["sol"] += 1
                if st["first"] is None:
                    st["first"] = st["nodes"]
                    if stop_first:
                        return True
                continue
            used[i] = True
            right[pos], bottom[pos] = rt, bt_
            if go(pos + 1):
                return True
            used[i] = False
        return False

    go(0)
    return per, st["sol"], st["first"], st["ok"]


def model_for(n, pieces):
    bt, mt = histograms(pieces)
    m = Model(n, n, bt, mt, 0)
    return m.profile(rowscan(n, n, from_bottom=False))


def knuth(n, pieces, probes, rng):
    """Knuth 1975 random-probe estimates of the row-major tree size."""
    idx = {}
    for i, p in enumerate(pieces):
        for k in range(4):
            q = rot(p, k)
            idx.setdefault((q[3], q[0]), []).append((i, q[1], q[2]))
    N = n * n
    est = []
    for _ in range(probes):
        used = set(); right = [0] * N; bottom = [0] * N; w = 1.0; tot = 0.0
        for pos in range(N):
            r, c = divmod(pos, n)
            left = right[pos - 1] if c else 0
            top = bottom[pos - n] if r else 0
            cand = [x for x in idx.get((left, top), ()) if x[0] not in used and (x[1] == 0) == (c == n - 1) and (x[2] == 0) == (r == n - 1)]
            if not cand:
                break
            w *= len(cand); tot += w
            i, rt, b = rng.choice(cand)
            used.add(i); right[pos], bottom[pos] = rt, b
        est.append(tot)
    return est


def partA():
    rng = random.Random(20261112)
    t0 = time.time()
    print("Part A: generated boards. log10 ratio = log10(exact / model)")
    print("n fc ic inst | exact tree (geo mean) | model tree (geo mean) | mean log10 ratio tree | sd | sum exact sols / sum model sols | mean first*sols/tree")
    for (n, fc, ic, k, cap) in ((4, 2, 3, 40, None), (4, 3, 4, 40, None), (5, 3, 4, 20, None), (5, 3, 5, 20, None), (6, 4, 6, 8, 4_000_000), (6, 5, 7, 8, 4_000_000)):
        lr, ex, mo, sx, sm, ev, done, depth_lr = [], [], [], 0, 0.0, [], 0, []
        for _ in range(k):
            ps = gen(n, fc, ic, rng)
            w = model_for(n, ps)
            per, sol, first, ok = enumerate_tree(n, ps, cap)
            if not ok:
                continue
            done += 1
            T = sum(per); M = float(sum(w))
            lr.append(math.log10(T / M)); ex.append(math.log10(T)); mo.append(math.log10(M))
            sx += sol; sm += float(w[-1])
            ev.append(first * sol / T)
            # per-depth ratio at the widest model depth
            d = max(range(len(w)), key=lambda i: w[i])
            depth_lr.append(math.log10(per[d + 1] / float(w[d])))
        print(f"{n} {fc} {ic} {done}/{k} | 10^{statistics.mean(ex):.2f} | 10^{statistics.mean(mo):.2f} | {statistics.mean(lr):+.3f} | {statistics.pstdev(lr):.3f} | {sx}/{sm:.1f} = {sx / sm:.2f} | mean {statistics.mean(ev):.2f} median {statistics.median(ev):.2f} | peak-depth ratio {statistics.mean(depth_lr):+.3f}  [{time.time() - t0:.0f}s]")
    # Knuth estimator variance on one 6x6
    ps = gen(6, 5, 7, rng)
    per, sol, first, ok = enumerate_tree(6, ps, 6_000_000)
    T = sum(per)
    est = knuth(6, ps, 20000, rng)
    mu = statistics.mean(est); cv = statistics.pstdev(est) / mu
    print(f"Knuth on one generated 6x6 (fc 5, ic 7): exact tree {T} (complete {ok}), 20000 probes mean {mu:.4g} (ratio {mu / T:.3f}), single-probe coefficient of variation {cv:.1f}, median probe {statistics.median(est):.4g}")
    for m in (100, 1000):
        ms = [statistics.mean(est[i:i + m]) / T for i in range(0, len(est), m)]
        print(f"   batches of {m} probes: min {min(ms):.2f} median {statistics.median(ms):.2f} max {max(ms):.2f} of the exact value")


def partB(which="primary", size=6, cap=20_000_000, limit=30):
    print(f"Part B: committed hard-ladder instances, ladder {which}, n={size}")
    inst = [json.loads(l) for l in open(os.path.join(LAD, "instances", f"{which}-test.jsonl"))]
    inst = [x for x in inst if x["n"] == size][:limit]
    com = {}
    for l in open(os.path.join(LAD, "results", "test", which, "hp-dfs.jsonl")):
        j = json.loads(l); com[j["instance"]] = j
    rows = []
    t0 = time.time()
    for x in inst:
        ps = [tuple(p) for p in x["puzzle"]["pieces"]]
        w = model_for(size, ps)
        M = float(sum(w)); S = float(w[-1])
        per, sol, first, ok = enumerate_tree(size, ps, cap)
        c = com[x["id"]]
        rows.append((x["id"], M, S, sum(per), sol, first, ok, c["nodes"], c["status"]))
        print(f"{x['id']:12s} model tree {M:10.4g} sols {S:7.2f} | exact tree {sum(per):9d} sols {sol:4d} complete {ok} | my first-solution nodes {first} | committed hp-dfs {c['nodes']} ({c['status']})  [{time.time() - t0:.0f}s]")
    full = [r for r in rows if r[6]]
    lr = [math.log10(r[3] / r[1]) for r in full]
    print(f"complete {len(full)}/{len(rows)}; mean log10(exact/model tree) {statistics.mean(lr):+.3f}, sd {statistics.pstdev(lr):.3f}; sum sols exact {sum(r[4] for r in full)} vs model {sum(r[2] for r in full):.1f}")
    same = sum(1 for r in rows if r[5] == r[7])
    print(f"my first-solution node count equals the committed hp-dfs count on {same}/{len(rows)} instances")
    pred = [r[3] / (r[4] + 1) for r in full]
    obs = [r[7] for r in full]
    print(f"median committed first-solution nodes {statistics.median(obs):.0f}; median exact tree/(sols+1) {statistics.median(pred):.0f}; median model tree/(model sols+4+1) {statistics.median([r[1] / (r[2] + 5) for r in full]):.0f}")
    print(f"mean of committed_first * sols / tree = {statistics.mean([r[7] * r[4] / r[3] for r in full]):.2f} (1.0 means first-solution cost = tree / sols)")


if __name__ == "__main__":
    a = sys.argv[1] if len(sys.argv) > 1 else "A"
    if a == "A":
        partA()
    elif a == "pred":
        for which, size in (("primary", 6), ("secondary", 6), ("primary", 7), ("secondary", 7)):
            inst = [json.loads(l) for l in open(os.path.join(LAD, "instances", f"{which}-test.jsonl"))]
            ms = [model_for(size, [tuple(p) for p in x["puzzle"]["pieces"]]) for x in inst if x["n"] == size]
            print(which, size, "model tree geo-mean 10^%.2f, model sols median %.2f" % (statistics.mean([L(sum(w)) for w in ms]), statistics.median([float(w[-1]) for w in ms])))
    else:
        partB(sys.argv[2], int(sys.argv[3]))
