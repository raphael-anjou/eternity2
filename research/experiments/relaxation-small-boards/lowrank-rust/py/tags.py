"""TOY. Per-tag summary (last row kept per tag, board, start). Broken-board rows are counted apart.
Usage: python3 -I py/tags.py rows.jsonl [prefix]"""
import json, sys
from statistics import median


def p90(v):
    v = sorted(v)
    if not v:
        return None
    k = 0.9 * (len(v) - 1)
    lo = int(k); hi = min(lo + 1, len(v) - 1)
    return v[lo] + (v[hi] - v[lo]) * (k - lo)


rows = {}
for l in open(sys.argv[1]):
    r = json.loads(l)
    rows[(r["tag"], r["board"], r["start"])] = r
pre = sys.argv[2] if len(sys.argv) > 2 else ""
tags = {}
for (t, b, s), r in rows.items():
    if t.startswith(pre):
        tags.setdefault(t, []).append(r)
print("| tag | solved | median evals | p90 evals | max evals | median s | ms/eval | broken solved | median A/F |")
print("|---|---|---|---|---|---|---|---|---|")
for t, rs in tags.items():
    br = [r for r in rs if "broken" in r["board"]]
    rs = [r for r in rs if "broken" not in r["board"]]
    ok = [r for r in rs if r["solved"]]
    # failures count as +infinity for the percentiles (censored)
    ev = sorted([r["evals_solution"] for r in ok] + [float("inf")] * (len(rs) - len(ok)))
    af = [r["active_nonneg"] / r["free_pairs"] for r in rs if "active_nonneg" in r]
    ms = 1e3 * sum(r["seconds"] for r in rs) / max(1, sum(r["evals"] for r in rs))
    print(f"| {t} | {len(ok)}/{len(rs)} | {median(ev) if ev else None} | {p90(ev) if ev else None} | "
          f"{max([r['evals_solution'] for r in ok]) if ok else None} | {median([r['t_solution'] for r in ok]) if ok else 0:.1f} | {ms:.2f} | "
          f"{sum(r['solved'] for r in br)}/{len(br)} | {median(af) if af else 'n/a'} |")
