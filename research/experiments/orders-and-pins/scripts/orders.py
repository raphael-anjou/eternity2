"""Part A order set: the named static orders on 16x16 and their 6x6 analogues,
plus the orders found by simulated annealing in the first-moment model.

    python3 -I scripts/orders.py            # writes orders/16/*.txt, orders/6/*.txt, orders/manifest.json

Cells are (r, c) with r the CSV row (field 6, y) and c the column (field 5,
x); pos = 16 r + c. Track 10's row scan is rows 15, 14, ..., 0, columns 0 to
15 inside a row (`rows-b-lr` here). The five official clues sit at (8, 7),
(2, 2), (2, 13), (13, 2), (13, 13). Their 6x6 analogues are (3, 2), (1, 1),
(1, 4), (4, 1), (4, 4) (each coordinate scaled by 5/15 and rounded).

Deterministic: the annealing uses random.Random(seed) with fixed seeds.
"""
import hashlib
import json
import math
import os
import random
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
EXP = os.path.dirname(HERE)
sys.path.insert(0, HERE)
from model import Model  # noqa: E402

CSV = os.path.join(EXP, "..", "single-core-benchmark", "engine", "data", "puzzles",
                   "size_16_official_eternity.csv")
CLUES16 = [(8, 7), (2, 2), (2, 13), (13, 2), (13, 13)]
CLUES6 = [(3, 2), (1, 1), (1, 4), (4, 1), (4, 4)]


def load_official():
    lines = [l.strip() for l in open(CSV) if l.strip()]
    pieces = []
    for l in lines[1:]:
        c = l.split(",")
        pieces.append(tuple(0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4]))
    return pieces


def rows(n, bottom=True, lr=True, snake=False):
    rr = range(n - 1, -1, -1) if bottom else range(n)
    out = []
    for i, r in enumerate(rr):
        cs = list(range(n)) if lr else list(range(n - 1, -1, -1))
        if snake and i % 2 == 1:
            cs.reverse()
        out += [(r, c) for c in cs]
    return out


def cols(n, left=True, bt=True, snake=False):
    cc = range(n) if left else range(n - 1, -1, -1)
    out = []
    for i, c in enumerate(cc):
        rs = list(range(n - 1, -1, -1)) if bt else list(range(n))
        if snake and i % 2 == 1:
            rs.reverse()
        out += [(r, c) for r in rs]
    return out


def spiral_in(n):
    """Track 10's spiral: clockwise inward from (0, 0)."""
    out = []
    top, left, bot, right = 0, 0, n - 1, n - 1
    while top <= bot and left <= right:
        out += [(top, c) for c in range(left, right + 1)]
        out += [(r, right) for r in range(top + 1, bot + 1)]
        if top < bot:
            out += [(bot, c) for c in range(right - 1, left - 1, -1)]
        if left < right:
            out += [(r, left) for r in range(bot - 1, top, -1)]
        top, left, bot, right = top + 1, left + 1, bot - 1, right - 1
    return out


def spiral_out(n, centre):
    """Square rings of growing Chebyshev distance around `centre`, each ring
    walked clockwise from the cell straight above the centre; cells off the
    board are skipped. The centre itself comes first (it is a clue when the
    order is used with clues, and is then skipped)."""
    cr, cc = centre
    out = [centre]
    for d in range(1, 2 * n):
        ring = []
        # clockwise from (cr - d, cc): top edge rightwards, right edge down,
        # bottom edge leftwards, left edge up, top edge rightwards to the start
        ring += [(cr - d, c) for c in range(cc, cc + d + 1)]
        ring += [(r, cc + d) for r in range(cr - d + 1, cr + d + 1)]
        ring += [(cr + d, c) for c in range(cc + d - 1, cc - d - 1, -1)]
        ring += [(r, cc - d) for r in range(cr + d - 1, cr - d - 1, -1)]
        ring += [(cr - d, c) for c in range(cc - d + 1, cc)]
        out += [(r, c) for (r, c) in ring if 0 <= r < n and 0 <= c < n]
    assert len(out) == n * n and len(set(out)) == n * n
    return out


def frame_then_rows(n):
    """The frame in Track 10's ring order (gate (b)), then the interior by
    rows from the bottom, left to right."""
    ring = spiral_in(n)[: 4 * (n - 1)]
    inner = [(r, c) for (r, c) in rows(n) if 0 < r < n - 1 and 0 < c < n - 1]
    return ring + inner


def antidiag(n):
    """Anti-diagonals from the bottom-left corner (15, 0), each from its
    lowest column upward (as in solve-path-5/e2_orders.py)."""
    allc = [(r, c) for r in range(n) for c in range(n)]
    return sorted(allc, key=lambda p: ((n - 1 - p[0]) + p[1], p[1]))


def bands2(n):
    """Comb: two-row bands from the bottom, column by column inside a band,
    lower cell first (solve-path-5/e2_orders.py's 2-row bands)."""
    return [(n - 1 - 2 * b - k, c) for b in range(n // 2) for c in range(n) for k in range(2)]


def named(n):
    centre = (8, 7) if n == 16 else (3, 2)
    return {
        "rows-b-lr": rows(n, True, True),
        "rows-b-rl": rows(n, True, False),
        "rows-t-lr": rows(n, False, True),
        "rows-t-rl": rows(n, False, False),
        "cols-l-bt": cols(n, True, True),
        "cols-l-tb": cols(n, True, False),
        "cols-r-bt": cols(n, False, True),
        "cols-r-tb": cols(n, False, False),
        "snake-rows-b": rows(n, True, True, snake=True),
        "snake-cols-l": cols(n, True, True, snake=True),
        "spiral-in": spiral_in(n),
        "spiral-out": spiral_out(n, centre),
        "frame-then-rows": frame_then_rows(n),
        "antidiag": antidiag(n),
        "bands2": bands2(n),
    }


def anneal(model, start, fixed, seed, proposals, t0=0.5, t1=0.001):
    """Simulated annealing on the model's log10 tree size. Moves: move one
    cell by up to 40 positions, or reverse a segment of 2 to 16 cells.
    Geometric temperature from t0 to t1 dex. Returns the best order seen."""
    rng = random.Random(seed)
    cur = list(start)
    ct = model.total(cur, fixed)
    best, bt = list(cur), ct
    L = len(cur)
    for it in range(proposals):
        T = t0 * (t1 / t0) ** (it / proposals)
        o = list(cur)
        if rng.random() < 0.5:
            i = rng.randrange(L)
            j = max(0, min(L - 1, i + rng.randint(-40, 40)))
            if i == j:
                continue
            x = o.pop(i)
            o.insert(j, x)
        else:
            i = rng.randrange(L - 1)
            j = min(L, i + rng.randint(2, 16))
            o[i:j] = o[i:j][::-1]
        t = model.total(o, fixed)
        if t <= ct or rng.random() < math.exp(-(t - ct) / T):
            cur, ct = o, t
            if ct < bt - 1e-12:
                best, bt = list(cur), ct
    return best, bt


def beam(model, fixed, width):
    """Beam search over order prefixes: keep the `width` prefixes (distinct
    cell sets) with the smallest sum of model widths so far, extend each by
    every free cell, until the board is full. Ties break by the prefix's cell
    list (deterministic)."""
    n = model.n
    free = [(r, c) for r in range(n) for c in range(n) if (r, c) not in fixed]
    beam_ = [(0.0, [], model.start(fixed))]
    for _ in range(len(free)):
        cand = {}
        for score, pre, st in beam_:
            for cell in free:
                if cell in st[0]:
                    continue
                nst, lw = model.extend(st, cell)
                sc = score + 10 ** (lw - 30)  # widths summed in units of 1e30
                key = nst[0]
                if key not in cand or (sc, pre + [cell]) < cand[key][:2]:
                    cand[key] = (sc, pre + [cell], nst)
        beam_ = sorted(cand.values(), key=lambda x: (x[0], x[1]))[:width]
    return beam_[0][1] + list(fixed)  # clue cells appended (skipped by every consumer)


SEARCHES = [  # (kind, start or beam width, seed)
    ("sa", "rows-b-lr", 11), ("sa", "cols-l-bt", 12), ("sa", "snake-rows-b", 13),
    ("sa", "spiral-out", 14), ("sa", "antidiag", 15), ("beam", 256, None), ("sa-beam", 256, 16),
]
PROPOSALS = 200_000
KEEP = 5


def write(path, order, n, note):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    body = " ".join(str(r * n + c) for (r, c) in order)
    with open(path, "w") as fh:
        fh.write(f"# {note}\n{body}\n")
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def main():
    pieces = load_official()
    M = Model(16, pieces)
    man = {"model": "scripts/model.py (bulge port), official colour census, five official clues",
           "orders": {}}
    orders16 = named(16)
    orders6 = named(6)
    for name, o in orders16.items():
        sha = write(os.path.join(EXP, "orders", "16", f"{name}.txt"), o, 16, f"{name}, 16x16, pos = 16 r + c")
        write(os.path.join(EXP, "orders", "6", f"{name}.txt"), orders6[name], 6, f"{name}, 6x6 analogue")
        prof = M.profile(o, CLUES16)
        man["orders"][name] = {"kind": "named", "sha256": sha, "model_log10_five": round(M.total(o, CLUES16), 4),
                               "model_log10_peak": round(max(prof), 4), "model_peak_depth": prof.index(max(prof)) + 1}
    # Search: every entry of SEARCHES; an output equal to a named order on the
    # free cells (clue cells dropped) or to an earlier output is dropped; the
    # KEEP lowest by model total are kept and measured.
    def free_seq(o):
        return tuple(x for x in o if x not in CLUES16)
    named_seqs = {free_seq(o): k for k, o in orders16.items()}
    found = []
    beam_order = None
    for i, (kind, arg, seed) in enumerate(SEARCHES):
        if kind == "sa":
            best, bt = anneal(M, orders16[arg], CLUES16, seed, PROPOSALS)
            desc = f"annealing from {arg}, seed {seed}, {PROPOSALS} proposals"
        elif kind == "beam":
            best = beam(M, CLUES16, arg)
            beam_order = best
            bt = M.total(best, CLUES16)
            desc = f"beam search, width {arg}"
        else:
            best, bt = anneal(M, beam_order, CLUES16, seed, PROPOSALS)
            desc = f"annealing from the width-{arg} beam order, seed {seed}, {PROPOSALS} proposals"
        fs = free_seq(best)
        dup = named_seqs.get(fs) or next((f["name"] for f in found if f["seq"] == fs), None)
        name = f"s{i + 1}-{kind}"
        print(name, desc, round(bt, 4), "duplicate of " + dup if dup else "new", flush=True)
        found.append({"name": name, "seq": fs, "order": best, "total": bt, "desc": desc, "dup": dup})
    man["search_log"] = [{"name": f["name"], "desc": f["desc"], "model_log10_five": round(f["total"], 4),
                          "duplicate_of": f["dup"]} for f in found]
    kept = sorted((f for f in found if not f["dup"]), key=lambda f: (f["total"], f["name"]))[:KEEP]
    for f in kept:
        name = f["name"]
        sha = write(os.path.join(EXP, "orders", "16", f"{name}.txt"), f["order"], 16, f"{name}: {f['desc']}")
        prof = M.profile(f["order"], CLUES16)
        diff = sum(1 for a, b in zip(f["order"], orders16["rows-b-lr"]) if a != b)
        man["orders"][name] = {"kind": "searched", "how": f["desc"], "sha256": sha,
                               "model_log10_five": round(f["total"], 4),
                               "model_log10_peak": round(max(prof), 4), "model_peak_depth": prof.index(max(prof)) + 1,
                               "positions_differing_from_rows-b-lr": diff}
    with open(os.path.join(EXP, "orders", "manifest.json"), "w") as fh:
        json.dump(man, fh, indent=1)
    for k, v in man["orders"].items():
        print(f"{k:28s} {v['model_log10_five']:8.3f} peak {v['model_log10_peak']:7.3f} at {v['model_peak_depth']}")


if __name__ == "__main__":
    main()
