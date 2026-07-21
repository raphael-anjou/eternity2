#!/usr/bin/env python3
"""Turn the hint-study results.jsonl into the site JSON the study pages read.

Emits web/src/data/hint-study.json with three blocks:
  - path_axis: on the E2 clue-shape, per fill path — score (median + spread),
    reached depth, solved count, AND the peak open frontier (computed from the
    path geometry, no solver), so the phase chart puts computed cause against
    measured effect.
  - count_axis: spread vs clustered ladders with the pinned-seam floor, so the
    honest (floor-subtracted) comparison is available.
  - meta: seeds, budget, the max score.

Every number carries n and spread; nothing is a single-seed point.
"""
import argparse
import json
import statistics as st
from collections import defaultdict, deque
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
REPO = HERE.parent.parent.parent
SITE_JSON = REPO / "web/src/data/hint-study.json"

N = 16
MAXSCORE = 2 * N * (N - 1)


def idx(r, c):
    return r * N + c


def clue_shape():
    q = N // 4
    return [idx(N // 2, N // 2), idx(q, q), idx(q, N - 1 - q), idx(N - 1 - q, q), idx(N - 1 - q, N - 1 - q)]


HINTS = clue_shape()


# --- fill-path sequences (mirror the DFS engine) ---
def row_major():
    return list(range(N * N))


def row_major_bottomup():
    return [idx(r, c) for r in range(N - 1, -1, -1) for c in range(N)]


def spiral_in():
    top, bot, left, right = 0, N - 1, 0, N - 1
    v = []
    while top <= bot and left <= right:
        v += [idx(top, c) for c in range(left, right + 1)]
        v += [idx(r, right) for r in range(top + 1, bot + 1)]
        if top < bot:
            v += [idx(bot, c) for c in range(right - 1, left - 1, -1)]
        if left < right:
            v += [idx(r, left) for r in range(bot - 1, top, -1)]
        top, bot, left, right = top + 1, bot - 1, left + 1, right - 1
    return v


def spiral_out():
    return list(reversed(spiral_in()))


def border_first():
    b, i = [], []
    for r in range(N):
        for c in range(N):
            (b if r in (0, N - 1) or c in (0, N - 1) else i).append(idx(r, c))
    return b + i


def verhaard_comb(h=10):
    v = [idx(r, c) for r in range(h) for c in range(N)]
    v += [idx(r, c) for c in range(N) for r in range(h, N)]
    return v


def clue_rows_first():
    cr = [2, 8, 13]
    v = [idx(r, c) for r in cr for c in range(N)]
    v += [idx(r, c) for r in range(N) if r not in cr for c in range(N)]
    return v


def connect_hints():
    seen = [False] * (N * N)
    order, q = [], deque()
    for h in HINTS:
        seen[h] = True
        order.append(h)
        q.append(h)
    while q:
        cell = q.popleft()
        r, c = cell // N, cell % N
        for nb in (idx(r - 1, c) if r > 0 else -1, idx(r, c - 1) if c > 0 else -1,
                   idx(r, c + 1) if c < N - 1 else -1, idx(r + 1, c) if r < N - 1 else -1):
            if nb >= 0 and not seen[nb]:
                seen[nb] = True
                order.append(nb)
                q.append(nb)
    return order


SEQ = {
    "rowmajor": row_major(), "rowmajor-bottomup": row_major_bottomup(),
    "spiral-in": spiral_in(), "spiral-out": spiral_out(), "border-first": border_first(),
    "verhaard-comb": verhaard_comb(), "clue-rows-first": clue_rows_first(),
    "connect-hints-first": connect_hints(),
}


def frontier_stats(seq):
    """Return (peak, mean) of the open frontier while filling `seq`. The mean
    (the sustained frontier cost, i.e. the area under the frontier curve) tracks
    the measured score slightly better than the peak, so it is what the phase
    chart plots."""
    hint = set(HINTS)
    filled = set(HINTS)
    peak = 0
    total = 0
    steps = 0
    for cell in seq:
        if cell in hint:
            continue
        filled.add(cell)
        f = 0
        for fc in filled:
            r, c = fc // N, fc % N
            nbrs = (idx(r - 1, c) if r > 0 else -1, idx(r, c - 1) if c > 0 else -1,
                    idx(r, c + 1) if c < N - 1 else -1, idx(r + 1, c) if r < N - 1 else -1)
            if any(nb >= 0 and nb not in filled for nb in nbrs):
                f += 1
        peak = max(peak, f)
        total += f
        steps += 1
    return peak, round(total / steps, 1) if steps else 0.0


def cells_for(layout):
    def lattice(stride, off=1):
        return [idx(r, c) for r in range(off, N, stride) for c in range(off, N, stride)]

    def clustered(k):
        off = 1
        anchors = [(off, off), (N - off - k, off), (off, N - off - k),
                   (N - off - k, N - off - k), (N // 2 - k // 2, N // 2 - k // 2)]
        s = set()
        for x0, y0 in anchors:
            for dy in range(k):
                for dx in range(k):
                    x, y = x0 + dx, y0 + dy
                    if x < N and y < N:
                        s.add(idx(y, x))
        return sorted(s)

    def spread_nearest(target):
        best, gap = lattice(2), abs(len(lattice(2)) - target)
        for stride in range(2, N + 1):
            c = lattice(stride)
            if c and abs(len(c) - target) < gap:
                best, gap = c, abs(len(c) - target)
        return best

    table = {
        "clue_shape_5": HINTS,
        "ladder_clustered_k2_20": clustered(2), "ladder_clustered_k3_45": clustered(3),
        "ladder_clustered_k4_80": clustered(4),
        "ladder_spread_04": spread_nearest(4), "ladder_spread_09": spread_nearest(9),
        "ladder_spread_16": spread_nearest(16), "ladder_spread_25": spread_nearest(25),
        "geom_scattered": lattice(4),
    }
    return table.get(layout)


def floor(layout):
    c = cells_for(layout)
    if c is None:
        return None
    pin = set(c)
    f = 0
    for r in range(N):
        for cc in range(N):
            i = r * N + cc
            if cc < N - 1 and i in pin and i + 1 in pin:
                f += 1
            if r < N - 1 and i in pin and i + N in pin:
                f += 1
    return f


def agg(vals):
    return dict(n=len(vals), median=st.median(vals), mean=round(st.mean(vals), 1),
                min=min(vals), max=max(vals),
                sd=round(st.pstdev(vals), 1) if len(vals) > 1 else 0.0)


PATHS_ORDER = ["rowmajor", "rowmajor-bottomup", "spiral-in", "spiral-out",
               "border-first", "verhaard-comb", "clue-rows-first", "connect-hints-first"]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run", required=True, help="results dir (containing results.jsonl)")
    ap.add_argument("--out", default=str(SITE_JSON))
    args = ap.parse_args()

    rp = Path(args.run)
    rp = rp / "results.jsonl" if rp.is_dir() else rp
    rows = [json.loads(l) for l in open(rp) if l.strip()]

    # Use only the COMMON COMPLETE seed set, so every layout/path is compared over
    # the SAME instances (a paired design). A "complete" seed has all 17 layouts ×
    # 9 solvers = 153 rows. This removes the partial-fleet artifact where different
    # rows were medianed over different seed subsets.
    per_seed = defaultdict(int)
    for r in rows:
        per_seed[r["seed"]] += 1
    complete = sorted(s for s, c in per_seed.items() if c >= 153)
    if not complete:  # fall back to whatever the fullest seeds are (dev preview)
        mx = max(per_seed.values(), default=0)
        complete = sorted(s for s, c in per_seed.items() if c >= max(1, int(mx * 0.9)))
    seedset = set(complete)
    rows = [r for r in rows if r["seed"] in seedset]

    by = defaultdict(list)
    for r in rows:
        by[(r["layout"], r["solver"])].append(r)

    # Per (layout, solver): the per-seed score, keyed by seed (so we can pair with
    # the no-hint baseline of the SAME seed).
    def scores_by_seed(layout, solver):
        return {r["seed"]: r["score"] for r in by.get((layout, solver), [])}

    # PATH axis on clue_shape_5 — richer, bimodality-aware, baseline-relative.
    baseline = {p: scores_by_seed("baseline_00", f"dfs-{p}") for p in PATHS_ORDER}
    clue = {p: scores_by_seed("clue_shape_5", f"dfs-{p}") for p in PATHS_ORDER}

    path_axis = []
    for path in PATHS_ORDER:
        cs = clue.get(path, {})
        if not cs:
            continue
        seeds_here = sorted(cs)
        scores = [cs[s] for s in seeds_here]
        a = agg(scores)
        solved = sum(1 for v in scores if v >= MAXSCORE)
        pf, mf = frontier_stats(SEQ[path]) if path in SEQ else (None, None)
        # baseline-relative: score WITH the 5 clue hints minus score with NO hints,
        # paired per seed. Negative = the hints HURT this path.
        bl = baseline.get(path, {})
        deltas = [cs[s] - bl[s] for s in seeds_here if s in bl]
        base_scores = [bl[s] for s in seeds_here if s in bl]
        path_axis.append({
            "path": path,
            "score": a,
            "per_seed": [{"seed": s, "score": cs[s]} for s in seeds_here],
            "solved": solved, "of": len(scores),
            "peak_frontier": pf, "mean_frontier": mf,
            "baseline_median": st.median(base_scores) if base_scores else None,
            "delta_median": st.median(deltas) if deltas else None,
            "delta_min": min(deltas) if deltas else None,
            "delta_max": max(deltas) if deltas else None,
        })
    path_axis.sort(key=lambda x: -x["score"]["median"])

    # COUNT axis (rowmajor) — floor-corrected + solved-rate. Common seed set.
    count_axis = []
    for layout in ["ladder_spread_04", "ladder_spread_09", "ladder_spread_16", "ladder_spread_25",
                   "ladder_clustered_k2_20", "ladder_clustered_k3_45", "ladder_clustered_k4_80"]:
        rs = by.get((layout, "dfs-rowmajor"))
        if not rs:
            continue
        scores = [r["score"] for r in rs]
        a = agg(scores)
        fl = floor(layout) or 0
        solved = sum(1 for v in scores if v >= MAXSCORE)
        count_axis.append({"layout": layout, "floor": fl, "score": a,
                           "earned": a["median"] - fl, "solved": solved, "of": len(scores)})

    # BEAM contrast on the clue shape (the non-backtracker solver, reported not hidden).
    beam = scores_by_seed("clue_shape_5", "beam-20k")
    beam_block = None
    if beam:
        bs = [beam[s] for s in sorted(beam)]
        beam_block = {"score": agg(bs), "solved": sum(1 for v in bs if v >= MAXSCORE),
                      "of": len(bs), "per_seed": [{"seed": s, "score": beam[s]} for s in sorted(beam)]}

    out = {"meta": {"seeds": complete, "n_seeds": len(complete), "n_rows": len(rows),
                    "max_score": MAXSCORE, "budget_s": rows[0]["budget"] if rows else None},
           "path_axis": path_axis, "count_axis": count_axis, "beam": beam_block}
    json.dump(out, open(args.out, "w"), indent=2)
    print(f"wrote {args.out}  ({len(complete)} complete seeds, {len(rows)} rows)")


if __name__ == "__main__":
    main()
