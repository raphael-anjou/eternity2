"""TOY BOARDS ONLY. Summary of results/<step>.jsonl rows: per board and pooled, success (a
verified valid rounding within the run's cap), median and 90th percentile CPU seconds to the
verified solution (successes only, and the "censored" median counting failures as the cap),
evaluations to the solution, and the broken control.
Usage: python3 -I py/summarise.py results/s1.jsonl [tag]"""
import json, sys
from statistics import median


def pct(v, q):
    v = sorted(v)
    if not v:
        return None
    k = (len(v) - 1) * q
    lo = int(k)
    hi = min(lo + 1, len(v) - 1)
    return v[lo] + (v[hi] - v[lo]) * (k - lo)


def f(x, nd=1):
    return "n/a" if x is None else f"{x:.{nd}f}"


def summarise(rows, label=""):
    by = {}
    for r in rows:
        by.setdefault(r["board"], []).append(r)
    out = []
    allt, alle, alln, alls, cens = [], [], 0, 0, []
    for b in sorted(by):
        rs = by[b]
        cap = max(x["seconds"] for x in rs)
        ok = [x for x in rs if x["solved"]]
        ts = [x["t_solution"] for x in ok]
        es = [x["evals_solution"] for x in ok]
        cs = [x["t_solution"] if x["solved"] else x["seconds"] for x in rs]
        bad = [x for x in ok if x.get("solution_in_enumeration") is False]
        if b != "b5_broken" and not b.endswith("broken"):
            allt += ts; alle += es; alln += len(rs); alls += len(ok); cens += cs
        out.append(f"| {label} | {b} | {len(ok)}/{len(rs)} | {f(median(ts) if ts else None)} | {f(pct(ts, 0.9))} | "
                   f"{f(median(cs))} | {f(median(es) if es else None, 0)} | {f(max(x['seconds'] for x in rs))} |"
                   + (" NOT IN ENUMERATION" if bad else ""))
    out.append(f"| {label} | pooled 5 boards | {alls}/{alln} ({100 * alls / max(1, alln):.0f}%) | {f(median(allt) if allt else None)} | "
               f"{f(pct(allt, 0.9))} | {f(median(cens) if cens else None)} | {f(median(alle) if alle else None, 0)} | |")
    return out


if __name__ == "__main__":
    rows = [json.loads(l) for l in open(sys.argv[1])]
    if len(sys.argv) > 2:
        rows = [r for r in rows if r["tag"] == sys.argv[2]]
    print("| step | board | solved | median s to solution | p90 s | median s, failures at cap | median evals to solution | max run s |")
    print("|---|---|---|---|---|---|---|---|")
    print("\n".join(summarise(rows, sys.argv[2] if len(sys.argv) > 2 else rows[0]["tag"])))
