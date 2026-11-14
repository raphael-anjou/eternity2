"""TOY BOARDS ONLY. T1 analysis: annealing (iso_t rows) against the relaxation (growth-exponent rows).

Inputs (all in this folder):
  raw_m1/*.jsonl         annealing rows from m1 (Apple M1, 8 jobs at once)
  raw_calib_m4/*.jsonl   calibration rows from this Mac (Apple M4, 2 jobs at once, nice 10)
  relaxation_rows/s6mac.jsonl, relaxation_rows/s6_x86.jsonl   copies, read only
Every solved annealing run is re-verified here from its final board: each tile once, every
inner edge matched, every rim side grey (colour 0).
Statistic per board: expected CPU seconds per verified solution = total thread CPU seconds of
all runs (unsolved runs at their full budget, relaxation starts capped at their cap) divided by
solutions found; exact Poisson 95 percent interval on the count.
Usage: uv run --with scipy python analyze.py
"""
import glob, json, os, statistics, collections
from scipy.stats import chi2

H = os.path.dirname(os.path.abspath(__file__))


def tiles_of(board):
    with open(os.path.join(H, "boards", f"b_{board}.txt")) as f:
        n, T = map(int, f.readline().split())
        return n, [tuple(map(int, f.readline().split())) for _ in range(T)]


def verify(board, placed):
    n, tiles = tiles_of(board)
    if sorted(t for t, _ in placed) != list(range(len(tiles))):
        return False
    col = lambda x, d: tiles[placed[x][0]][(d + placed[x][1]) & 3]
    for x in range(n * n):
        r, c = divmod(x, n)
        for d, (dr, dc) in enumerate(((-1, 0), (0, 1), (1, 0), (0, -1))):
            rr, cc = r + dr, c + dc
            if 0 <= rr < n and 0 <= cc < n:
                if col(x, d) != col(rr * n + cc, (d + 2) & 3):
                    return False
            elif col(x, d) != 0:
                return False
    return True


def load(pattern):
    rows = []
    for fn in sorted(glob.glob(os.path.join(H, pattern))):
        tag, rest = os.path.basename(fn)[:-6].split("_", 1)
        board = rest.rsplit("_s", 1)[0]
        for line in open(fn):
            if '"traj"' not in line:
                continue
            r = json.loads(line)
            solved = r["solved_at"] >= 0
            ok = solved and verify(board, r["board"]) and r["check"] == 0 and r["final_dist"] == 0
            if solved and not ok:
                print("VERIFY FAILED", fn, r["run"])
            rows.append(dict(tag=tag, board=board, cpu=r["cpu_s"], steps=r["steps_done"], solved=ok, file=os.path.basename(fn)))
    return rows


def interval(total, k):
    lo = chi2.ppf(0.025, 2 * k) / 2 if k > 0 else 0.0
    hi = chi2.ppf(0.975, 2 * k + 2) / 2
    return (total / hi, total / lo if lo > 0 else float("inf"))


def fmt(x):
    return "inf" if x == float("inf") else (f"{x:.1f}" if x < 100 else f"{x:.0f}")


def summarise(rows, label):
    out = collections.OrderedDict()
    for key in sorted({(r["tag"], r["board"]) for r in rows}):
        rs = [r for r in rows if (r["tag"], r["board"]) == key]
        k = sum(r["solved"] for r in rs)
        total = sum(r["cpu"] for r in rs)
        tsol = [r["cpu"] for r in rs if r["solved"]]
        lo, hi = interval(total, k)
        e = total / k if k else float("inf")
        out[key] = dict(n=len(rs), k=k, total=total, exp=e, lo=lo, hi=hi,
                        med_sol=statistics.median(tsol) if tsol else None,
                        med_run=statistics.median(r["cpu"] for r in rs),
                        sps=sum(r["steps"] for r in rs) / total)
        o = out[key]
        ms = "-" if o["med_sol"] is None else f"{o['med_sol']:.2f}"
        print(f"{label} {key[0]:9s} {key[1]:8s} solved {k:3d}/{len(rs):3d}  total {total:9.1f} s  "
              f"exp/sol {fmt(e):>7s} [{fmt(lo)}, {fmt(hi)}]  median to solution {ms}  "
              f"median per run {o['med_run']:.2f}  steps/s {o['sps']:.3e}")
    return out


def relaxation(fn, label):
    rows = [json.loads(l) for l in open(os.path.join(H, "relaxation_rows", fn))]
    out = collections.OrderedDict()
    for b in sorted({r["board"] for r in rows}):
        rs = [r for r in rows if r["board"] == b]
        k = sum(1 for r in rs if r["solved"] and r["end_is_solution"] and r.get("solution_in_enumeration"))
        total = sum(r["seconds"] for r in rs)
        lo, hi = interval(total, k)
        e = total / k if k else float("inf")
        out[b] = dict(n=len(rs), k=k, total=total, exp=e, lo=lo, hi=hi)
        print(f"relax {label:5s} {b}: rows {len(rs):2d} solved {k}  total {total:8.0f} s  exp/sol {fmt(e)} [{fmt(lo)}, {fmt(hi)}]")
    print(f"relax {label}: {len(rows)} rows in all")
    return out


if __name__ == "__main__":
    m1 = summarise(load("raw_m1/*.jsonl"), "M1")
    print()
    m4 = summarise(load("raw_calib_m4/*.jsonl"), "M4")
    print()
    a = m1[("a2e8", "6_4_5_0")]; c = m4[("calib2e8", "6_4_5_0")]
    ratio = (1 / c["sps"]) / (1 / a["sps"])
    print(f"calibration, board 6_4_5_0, 2e8 steps: M4 {c['sps']:.4e} steps per CPU s (n={c['n']}), "
          f"M1 {a['sps']:.4e} (n={a['n']}); M4 seconds per step / M1 seconds per step = {ratio:.3f}")
    print()
    rx = {"mac": relaxation("s6mac.jsonl", "mac"), "x86": relaxation("s6_x86.jsonl", "x86")}
    print()
    print("kill line (a) counts: boards where annealing exp/sol <= relaxation exp/sol")
    for tag in ("a2e6", "a2e8"):
        for mach, R in rx.items():
            for conv, f in (("M1 seconds", 1.0), ("M1 converted to M4 seconds", ratio)):
                if mach == "x86" and f != 1.0:
                    continue
                hits = []
                for s in range(5):
                    b6 = f"b6_s{s}"; ab = (tag, f"6_4_5_{s}")
                    if b6 not in R or ab not in m1:
                        continue
                    ea, er = m1[ab]["exp"] * f, R[b6]["exp"]
                    # both arms with 0 solutions: no measurement, counted as undetermined
                    hits.append((s, "both 0 solved" if ea == er == float("inf") else ea <= er))
                yes = sum(1 for _, h in hits if h is True)
                und = sum(1 for _, h in hits if h == "both 0 solved")
                print(f"  {tag} vs relaxation on {mach} ({conv}): at or below on {yes} of {len(hits)} boards "
                      f"({und} undetermined); per board {hits}")
