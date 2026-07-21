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
    # The EXACT official Eternity II clue cells (idx is row-major, idx(row, col)):
    # I8 (row 8, col 7) centre, C3 (2,2), C14 (2,13), N3 (13,2), N14 (13,13).
    # Must match the Rust generator's e2_clue_shape at 16x16.
    return [idx(8, 7), idx(2, 2), idx(2, 13), idx(13, 2), idx(13, 13)]


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


def _bfs_from(anchors):
    seen = [False] * (N * N)
    order, q = [], deque()
    for h in anchors:
        if not seen[h]:
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
    for p in range(N * N):
        if not seen[p]:
            order.append(p)
    return order


def connect_hints():
    return _bfs_from(HINTS)


def trace_hints():
    # Skeleton (square between the 4 outer hints + diagonals to centre), then BFS
    # fill outward. Mirrors the Rust `trace_hints` / `trace_hints_seq`.
    if len(HINTS) < 5:
        return _bfs_from(HINTS)
    def xy(cell):
        return cell % N, cell // N
    centre = min(HINTS, key=lambda cell: (xy(cell)[0] - N // 2) ** 2 + (xy(cell)[1] - N // 2) ** 2)
    outer = [c for c in HINTS if c != centre]
    def corner_key(cell):
        x, y = xy(cell)
        top, left = y < N // 2, x < N // 2
        return {(True, True): 0, (True, False): 1, (False, False): 2, (False, True): 3}[(top, left)]
    outer.sort(key=corner_key)
    seen = [False] * (N * N)
    order = []
    def push(cell):
        if not seen[cell]:
            seen[cell] = True
            order.append(cell)
    def line(a, b):
        x0, y0 = xy(a)
        x1, y1 = xy(b)
        dx, dy = abs(x1 - x0), abs(y1 - y0)
        sx, sy = (1 if x0 < x1 else -1), (1 if y0 < y1 else -1)
        err = dx - dy
        while True:
            push(idx(y0, x0))
            if x0 == x1 and y0 == y1:
                break
            e2 = 2 * err
            if e2 > -dy:
                err -= dy
                x0 += sx
            if e2 < dx:
                err += dx
                y0 += sy
    for i in range(len(outer)):
        line(outer[i], outer[(i + 1) % len(outer)])
    for o in outer:
        line(o, centre)
    q = deque(order)
    while q:
        cell = q.popleft()
        r, c = cell // N, cell % N
        for nb in (idx(r - 1, c) if r > 0 else -1, idx(r, c - 1) if c > 0 else -1,
                   idx(r, c + 1) if c < N - 1 else -1, idx(r + 1, c) if r < N - 1 else -1):
            if nb >= 0 and not seen[nb]:
                seen[nb] = True
                order.append(nb)
                q.append(nb)
    for p in range(N * N):
        if not seen[p]:
            order.append(p)
    return order


SEQ = {
    "rowmajor": row_major(), "rowmajor-bottomup": row_major_bottomup(),
    "spiral-in": spiral_in(), "spiral-out": spiral_out(),
    "verhaard-comb": verhaard_comb(), "clue-rows-first": clue_rows_first(),
    "connect-hints-first": connect_hints(), "trace-hints": trace_hints(),
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

    def lattice_per_line(k):
        # k evenly-spaced points per line (k×k), matching the Rust generator.
        if k == 0:
            return []
        lo, hi = 1, N - 2
        span = hi - lo
        def coord(i):
            return N // 2 if k == 1 else lo + (span * i) // (k - 1)
        s = set()
        for iy in range(k):
            for ix in range(k):
                s.add(idx(coord(iy), coord(ix)))
        return sorted(s)

    table = {
        "clue_shape_5": HINTS,
        "ladder_clustered_k2_20": clustered(2), "ladder_clustered_k3_45": clustered(3),
        "ladder_clustered_k4_80": clustered(4),
        "ladder_spread_04": lattice_per_line(2), "ladder_spread_09": lattice_per_line(3),
        "ladder_spread_16": lattice_per_line(4), "ladder_spread_25": lattice_per_line(5),
        "ladder_spread_36": lattice_per_line(6),
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
               "verhaard-comb", "clue-rows-first", "connect-hints-first", "trace-hints"]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run", required=True, help="results dir (containing results.jsonl)")
    ap.add_argument("--out", default=str(SITE_JSON))
    args = ap.parse_args()

    rp = Path(args.run)
    rp = rp / "results.jsonl" if rp.is_dir() else rp
    rows = [json.loads(l) for l in open(rp) if l.strip()]

    # Use only the COMMON COMPLETE seed set, so every layout/path is compared over
    # the SAME instances (a paired design). A "complete" seed has the full row count
    # (layouts × solvers); we take it as the max observed per-seed count so the
    # threshold tracks the grid size automatically. This removes the partial-fleet
    # artifact where different rows were medianed over different seed subsets.
    per_seed = defaultdict(int)
    for r in rows:
        per_seed[r["seed"]] += 1
    full = max(per_seed.values(), default=0)
    complete = sorted(s for s, c in per_seed.items() if c >= full)
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
    for layout in ["ladder_spread_04", "ladder_spread_09", "ladder_spread_16",
                   "ladder_spread_25", "ladder_spread_36",
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

    # HINT-GEOMETRY comparison: the /research/why/hint-geometry page's 18-scattered
    # vs 18-contiguous layouts, on OUR boards. Reported per path so we can say
    # whether scattered beats contiguous here as that page claims. Uses row-major
    # (the compact sweep) as the headline plus the full per-path spread.
    hintgeo = {}
    for layout in ["hintgeo_scattered_18", "hintgeo_contiguous_18"]:
        per_path = []
        for path in PATHS_ORDER:
            rs = by.get((layout, f"dfs-{path}"))
            if not rs:
                continue
            sc = [r["score"] for r in rs]
            per_path.append({"path": path, "score": agg(sc),
                             "solved": sum(1 for v in sc if v >= MAXSCORE), "of": len(sc)})
        hintgeo[layout] = per_path

    # BEAM contrast on the clue shape (the non-backtracker solver, reported not hidden).
    beam = scores_by_seed("clue_shape_5", "beam-20k")
    beam_block = None
    if beam:
        bs = [beam[s] for s in sorted(beam)]
        beam_block = {"score": agg(bs), "solved": sum(1 for v in bs if v >= MAXSCORE),
                      "of": len(bs), "per_seed": [{"seed": s, "score": beam[s]} for s in sorted(beam)]}

    out = {"meta": {"seeds": complete, "n_seeds": len(complete), "n_rows": len(rows),
                    "max_score": MAXSCORE, "budget_s": rows[0]["budget"] if rows else None},
           "path_axis": path_axis, "count_axis": count_axis, "beam": beam_block,
           "hintgeo": hintgeo}
    json.dump(out, open(args.out, "w"), indent=2)
    print(f"wrote {args.out}  ({len(complete)} complete seeds, {len(rows)} rows)")


if __name__ == "__main__":
    main()
