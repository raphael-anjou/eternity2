"""Table of raw/cmp/*.jsonl: per method and board, solved of runs, median sweeps to the verified
solution (solved runs only), median final mismatches, median seconds. A "sweep" is one gradient
evaluation for the continuous methods and N proposals for annealing (see each script)."""
import json, os, sys, statistics as st
D = os.path.join(os.path.dirname(os.path.abspath(__file__)), "raw", "cmp")


def rows(name):
    p = os.path.join(D, name)
    return [json.loads(l) for l in open(p)] if os.path.exists(p) else []


def key(r):
    return f"s{r['board'][3]}{' broken' if (r.get('broken') or r.get('mode') == 'broken') else ''}"


def table(label, rs, sweeps, mism, extra=lambda r: ""):
    by = {}
    for r in rs:
        by.setdefault(key(r), []).append(r)
    for k in sorted(by):
        g = by[k]; s = [r for r in g if r["solved"]]
        print(f"{label:22s} {k:10s} solved {len(s):2d}/{len(g):2d}  sweeps to solution (median, min, max) "
              f"{(st.median(sweeps(r) for r in s), min(sweeps(r) for r in s), max(sweeps(r) for r in s)) if s else '-'}  "
              f"final mismatches median {st.median(mism(r) for r in g)} min {min(mism(r) for r in g)}  "
              f"median s {st.median(r['seconds'] for r in g):.2f}  median sweeps all {st.median(sweeps(r) for r in g)} {extra(g)}")


table("cp column (mirror)", rows("soft.jsonl"), lambda r: r["solved_at"] or r["sweeps"], lambda r: r["mismatches"])
table("cp column (annealed)", rows("softanneal.jsonl"), lambda r: r["solved_at"] or r["sweeps"], lambda r: r["mismatches"],
      lambda g: f"distinct end conflicts {len({round(r['conflict'], 6) for r in g})}")
c = rows("coupled.jsonl")
for r_ in sorted({r["r"] for r in c}):
    table(f"cp coupled r={r_}", [r for r in c if r["r"] == r_], lambda r: r["evals"], lambda r: r["final"]["mism"],
          lambda g: f"live columns median {st.median(r['live_columns'] for r in g)}")
table("annealing 2e6 steps", rows("sa.jsonl"), lambda r: r["sweeps"], lambda r: r["mismatches"])
# pass 2 reran, from the same start with a 480 s limit, every real-board run that hit 180 s
p2 = {(r["board"][3], r["start"]): r for r in rows("bm_pass2.jsonl")}
table("baseline r=32, 180 s", rows("bm.jsonl"), lambda r: r["evals"], lambda r: r["mismatches"])
merged = [p2.get((r["board"][3], r["start"]), r) if (not r["broken"] and not r["solved"]) else r for r in rows("bm.jsonl")]
table("baseline r=32, merged", merged, lambda r: r["evals"], lambda r: r["mismatches"],
      lambda g: f"statuses {sorted({r['status'] for r in g})}")
