#!/usr/bin/env python3
"""Baseline tournament analysis (PREREG.md). Stdlib only.

Reads results/raw/<entrant>/s<seed>-<rung>.jsonl, writes
results/summary.json, results/tables.md and results/identity.csv.
Run from the experiment directory: python3 analysis/analyze.py
"""
import csv
import glob
import json
import math
import os
import random

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(HERE, ".."))
EXP = os.path.normpath(os.path.join(ROOT, ".."))
CAP = 1_000_000
RUNGS = ["R6", "R7", "R8"]
INF = math.inf
ADOPTED = {"R6": "scb-gacolor_ac3_random", "R7": "scb-gacolor_ac3_random", "R8": "bb-mrv"}
KTAM_ARMS = {"B3": "ktam-b3", "V1": "ktam-v1", "V2": "ktam-v2"}
KTAM_CONFIGS = {"b3-rinf-k2", "v1-beta2-rinf-k1", "v2-t0-n0.1"}


def load_raw():
    rows = {}
    files = glob.glob(os.path.join(ROOT, "results/raw/*/s*-R*.jsonl")) + glob.glob(os.path.join(ROOT, "results/raw-m1/*/s*-R*.jsonl")) + glob.glob(os.path.join(ROOT, "results/raw-x862/*/s*-R*.jsonl"))
    for f in sorted(files):
        rs = [json.loads(l) for l in open(f) if l.strip()]
        if not rs:  # placeholder for work moved to the other machine
            continue
        ent = os.path.basename(os.path.dirname(f))
        seed = int(os.path.basename(f).split("-")[0][1:])
        rung = os.path.basename(f).split("-")[1].split(".")[0]
        assert (ent, seed, rung) not in rows, f"duplicate {ent} s{seed} {rung}"
        rows[(ent, seed, rung)] = rs
    return rows


def wilson(k, n, z=1.959964):
    p = k / n
    d = 1 + z * z / n
    c = (p + z * z / (2 * n)) / d
    h = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / d
    return max(0.0, c - h), min(1.0, c + h)


def km(rs):
    v = sorted(r["nodes"] if r["status"] == "solved" else INF for r in rs)
    return v[7], v[14], v[22]


def stats(rs):
    solved = sum(r["status"] == "solved" for r in rs)
    q1, med, q3 = km(rs)
    walls = sorted(r["wall_ms"] for r in rs)
    return {
        "n_rows": len(rs),
        "solved": solved,
        "wilson": wilson(solved, len(rs)),
        "q1": q1,
        "median": med,
        "q3": q3,
        "sum_nodes": sum(r["nodes"] if r["status"] == "solved" else CAP for r in rs),
        "median_wall_ms": walls[len(walls) // 2] if len(walls) % 2 else (walls[len(walls) // 2 - 1] + walls[len(walls) // 2]) / 2,
        "total_wall_s": sum(walls) / 1000,
        "time_censored": sum(bool(r.get("time_censored")) for r in rs),
        "errors": sum(r["status"] == "error" for r in rs),
        "inconsistent": sum(r["inconsistent"] for r in rs),
        "equals_planted": sum(bool(r["equals_planted"]) for r in rs),
    }


def seed_median(vals):
    s = sorted(vals)
    return s[(len(s) - 1) // 2]


def logratio(c, b):
    cs, bs = c["status"] == "solved", b["status"] == "solved"
    if cs and bs:
        return math.log(c["nodes"] / b["nodes"])
    if cs:
        return math.log(c["nodes"] / CAP)
    if bs:
        return INF
    return 0.0


def g1(cand, base):
    bi = {r["instance"]: r for r in base}
    pairs = [(r, bi[r["instance"]]) for r in cand if r["instance"] in bi]
    lr = [logratio(c, b) for c, b in pairs]
    n = len(lr)
    um = sorted(lr)[n // 2]  # upper median (16th of 30)
    rng = random.Random(20261111)
    boots = []
    for _ in range(10_000):
        s = sorted(lr[rng.randrange(n)] for _ in range(n))
        boots.append(s[n // 2])
    boots.sort()
    lo, hi = boots[int(0.025 * 10_000)], boots[int(0.975 * 10_000) - 1]
    if any(c.get("machine") != b.get("machine") for c, b in pairs):
        wr = None  # different machines: wall clock not comparable
    else:
        wr = sorted(c["wall_ms"] / max(b["wall_ms"], 1e-3) for c, b in pairs)
    return {
        "pairs": n,
        "ratio": math.exp(um) if um != INF else INF,
        "ci": [math.exp(lo) if lo != INF else INF, math.exp(hi) if hi != INF else INF],
        "beats": um != INF and math.exp(um) <= 0.5 and hi < 0,
        "wall_ratio": None if wr is None else ((wr[n // 2 - 1] + wr[n // 2]) / 2 if n % 2 == 0 else wr[n // 2]),
    }


def committed_rows():
    """Every committed run row of another experiment, keyed by (entrant, run_seed, ladder, n, c, seed)."""
    out = {}
    for f in glob.glob(os.path.join(EXP, "*/results/**/*.jsonl"), recursive=True):
        rel = os.path.relpath(f, EXP)
        if rel.startswith("baseline-tournament/") or "passes" in rel:
            continue
        try:
            rs = [json.loads(l) for l in open(f) if l.strip()]
        except (json.JSONDecodeError, UnicodeDecodeError):
            continue
        for r in rs:
            if not isinstance(r, dict) or "status" not in r or "nodes" not in r or "seed" not in r or "n" not in r:
                continue
            if r.get("role", "test") != "test":
                continue
            if "arm" in r and "config" in r:  # A15 rows
                if r["arm"] not in KTAM_ARMS or r["config"] not in KTAM_CONFIGS or r.get("node_cap") != CAP:
                    continue
                ent, rs_, ladder = KTAM_ARMS[r["arm"]], 1, "primary"
            else:
                if r.get("cap") != CAP or "solver" not in r:
                    continue
                ent, rs_, ladder = r["solver"], r.get("run_seed", 0), r.get("ladder")
            tc = r["status"] == "censored" and r.get("nodes_counted", CAP) < CAP
            key = (ent, rs_, ladder, r["n"], r.get("c"), r["seed"])
            out.setdefault(key, []).append((rel, r["status"], r["nodes"], tc))
    return out


def fmt(x):
    if x == INF:
        return "> cap"
    if isinstance(x, float):
        return f"{x:,.3f}"
    return f"{x:,}"


def main():
    raw = load_raw()
    entrants = sorted({k[0] for k in raw})
    meta = {}
    for (e, s, r), rs in raw.items():
        meta[e] = {"source": rs[0]["source"], "complete": rs[0]["complete"], "stochastic": rs[0]["stochastic"], "guard_s": rs[0]["guard_s"]}

    # Identity against committed rows.
    com = committed_rows()
    ident = []
    for (e, s, rung), rs in raw.items():
        for r in rs:
            key = (e, r["run_seed"], r["ladder"], r["n"], r["c"], r["seed"])
            for rel, st, nodes, tc in com.get(key, []):
                same = (st, nodes) == (r["status"], r["nodes"])
                kind = "match" if same else ("guard-difference" if (tc or r["time_censored"]) else "MISMATCH")
                ident.append({"entrant": e, "run_seed": r["run_seed"], "rung": rung, "instance": r["instance"],
                              "committed_file": rel, "committed_status": st, "committed_nodes": nodes,
                              "rerun_status": r["status"], "rerun_nodes": r["nodes"], "result": kind})
    with open(os.path.join(ROOT, "results/identity.csv"), "w", newline="") as fh:
        w = csv.DictWriter(fh, fieldnames=list(ident[0].keys()) if ident else ["entrant"])
        w.writeheader()
        w.writerows(sorted(ident, key=lambda d: (d["entrant"], d["committed_file"], d["instance"])))
    id_summary = {}
    for d in ident:
        k = f'{d["entrant"]} | {d["committed_file"]}'
        a = id_summary.setdefault(k, {"match": 0, "MISMATCH": 0, "guard-difference": 0})
        a[d["result"]] += 1

    # m1 verification rerun (PREREG_AMENDMENTS.md 1).
    macchk = {"compared": 0, "mismatches": 0}
    for f in glob.glob(os.path.join(ROOT, "results/check-m1/*/s*-R*.jsonl")):
        ent = os.path.basename(os.path.dirname(f))
        seed = int(os.path.basename(f).split("-")[0][1:])
        rung = os.path.basename(f).split("-")[1].split(".")[0]
        x86 = {r["instance"]: r for r in raw.get((ent, seed, rung), [])}
        for l in open(f):
            if not l.strip():
                continue
            r = json.loads(l)
            d = x86.get(r["instance"])
            if d is None:
                continue
            macchk["compared"] += 1
            macchk["mismatches"] += (r["status"], r["nodes"]) != (d["status"], d["nodes"])

    # Kill criterion: anchors.
    def anchor_bad(ent, files):
        return sum(v["MISMATCH"] for k, v in id_summary.items() if k.startswith(ent + " |") and any(f in k for f in files))
    anchors = {
        "bb-mrv": {"mismatches": anchor_bad("bb-mrv", ["mrv-baseline/", "backdoor-branching/"]),
                   "compared": sum(sum(v.values()) for k, v in id_summary.items() if k.startswith("bb-mrv |") and ("mrv-baseline/" in k or "backdoor-branching/" in k))},
        "scb-gacolor_ac3_random": {"mismatches": anchor_bad("scb-gacolor_ac3_random", ["hard-ladder/results/test/primary", "mrv-baseline/"]),
                                   "compared": sum(sum(v.values()) for k, v in id_summary.items() if k.startswith("scb-gacolor_ac3_random |") and ("hard-ladder/results/test/primary" in k or "mrv-baseline/" in k))},
    }
    killed = any(a["mismatches"] > 0 for a in anchors.values()) or any(a["compared"] == 0 for a in anchors.values())

    # Per-rung tables.
    summary = {"mac_m1_check": macchk, "anchors": anchors, "killed": killed, "identity": id_summary, "rungs": {}}
    md = ["# Baseline tournament tables (generated by analysis/analyze.py)", ""]
    md.append(f"Kill criterion (anchor identity): {'KILLED' if killed else 'not fired'}; " +
              "; ".join(f"{k}: {v['mismatches']} mismatches over {v['compared']} compared rows" for k, v in anchors.items()))
    md.append(f"m1 verification (scb-gacolor_ac3_random s1 rerun on m1 vs x86 laptop): {macchk['mismatches']} mismatches over {macchk['compared']} rows")
    md.append("")
    for rung in RUNGS:
        base = raw.get((ADOPTED[rung], 1 if ADOPTED[rung].startswith("scb") else 0, rung))
        rows = []
        for e in entrants:
            seeds = sorted(s for (ee, s, r) in raw if ee == e and r == rung)
            if not seeds:
                continue
            per = {s: stats(raw[(e, s, rung)]) for s in seeds}
            valid = all(p["errors"] == 0 and p["inconsistent"] == 0 for p in per.values())
            agg = {k: seed_median([p[k] for p in per.values()]) for k in ["solved", "median", "q1", "q3", "sum_nodes", "median_wall_ms", "time_censored"]}
            # Wall clock is per machine: take the seeds run on the first seed's machine.
            m0 = raw[(e, seeds[0], rung)][0].get("machine", "x86")
            same = [s for s in seeds if raw[(e, s, rung)][0].get("machine", "x86") == m0]
            agg["median_wall_ms"] = seed_median([per[s]["median_wall_ms"] for s in same])
            agg["wall_machine"] = m0
            agg["solved_range"] = [min(p["solved"] for p in per.values()), max(p["solved"] for p in per.values())]
            agg["median_range"] = [min(p["median"] for p in per.values()), max(p["median"] for p in per.values())]
            agg["wilson"] = wilson(agg["solved"], 30)
            g = {s: g1(raw[(e, s, rung)], base) for s in seeds} if base and e != ADOPTED[rung] else {}
            machines = sorted({x.get("machine", "x86") for s in seeds for x in raw[(e, s, rung)]})
            rows.append({"entrant": e, "machines": machines, "seeds": seeds, "valid": valid, "per_seed": per, "agg": agg, "g1": g, **meta[e]})
        complete = [r for r in rows if r["complete"] and r["valid"]]
        complete.sort(key=lambda r: (-r["agg"]["solved"], r["agg"]["median"], r["agg"]["q1"], r["agg"]["q3"], r["agg"]["sum_nodes"], r["entrant"]))
        for i, r in enumerate(complete):
            r["rank"] = i + 1
        summary["rungs"][rung] = rows
        md += [f"## {rung} (ranked complete entrants; adopted baseline {ADOPTED[rung]})", "",
               "| rank | entrant | seeds | solved/30 | Wilson 95% | KM Q1 | KM median | KM Q3 | median wall ms | time-cens | G1 vs adopted (ratio, 95% CI, beats) | wall ratio | machine | source |",
               "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"]
        for r in complete:
            a = r["agg"]
            sd = ",".join(map(str, r["seeds"]))
            solved = f'{a["solved"]}' + (f' ({a["solved_range"][0]}-{a["solved_range"][1]})' if len(r["seeds"]) > 1 else "")
            med = fmt(a["median"]) + (f' ({fmt(a["median_range"][0])} to {fmt(a["median_range"][1])})' if len(r["seeds"]) > 1 else "")
            gs = r["g1"]
            if gs:
                g1s = "; ".join(f's{s}: {fmt(v["ratio"])} [{fmt(v["ci"][0])}, {fmt(v["ci"][1])}] {"BEATS" if v["beats"] else "no"}' for s, v in gs.items())
                wrs = "; ".join("n/c (other machine)" if v["wall_ratio"] is None else f'{v["wall_ratio"]:.2f}' for v in gs.values())
            else:
                g1s, wrs = "(baseline)", ""
            md.append(f'| {r["rank"]} | {r["entrant"]} | {sd} | {solved} | {a["wilson"][0]:.2f}-{a["wilson"][1]:.2f} | {fmt(a["q1"])} | {med} | {fmt(a["q3"])} | {a["median_wall_ms"]:,.0f} | {a["time_censored"]} | {g1s} | {wrs} | {",".join(r["machines"])} | {r["source"]} |')
        inval = [r for r in rows if not r["valid"]]
        if inval:
            md += ["", f"Invalid at {rung} (not ranked):", ""]
            for r in inval:
                p = list(r["per_seed"].values())
                md.append(f'- {r["entrant"]}: errors {sum(x["errors"] for x in p)}, inconsistent tries {sum(x["inconsistent"] for x in p)}')
        inc = [r for r in rows if not r["complete"] and r["valid"]]
        if inc:
            md += ["", f"### {rung} incomplete entrants (not ranked against complete solvers)", "",
                   "| entrant | seeds | solved/30 (range) | KM median nodes (range) | median wall ms |", "|---|---|---|---|---|"]
            for r in inc:
                a = r["agg"]
                md.append(f'| {r["entrant"]} | {",".join(map(str, r["seeds"]))} | {a["solved"]} ({a["solved_range"][0]}-{a["solved_range"][1]}) | {fmt(a["median"])} ({fmt(a["median_range"][0])} to {fmt(a["median_range"][1])}) | {a["median_wall_ms"]:,.0f} |')
        md.append("")
    md += ["## Rerun identity against committed rows", "", "| entrant | committed file | match | mismatch | guard difference |", "|---|---|---|---|---|"]
    for k in sorted(id_summary):
        v = id_summary[k]
        e, f = k.split(" | ")
        md.append(f'| {e} | {f} | {v["match"]} | {v["MISMATCH"]} | {v["guard-difference"]} |')
    md.append("")
    # Compact ranked tables for the README.
    cm = []
    for rung in RUNGS:
        rows = summary["rungs"][rung]
        cm += [f"### {rung}", "", "| rank | entrant | solved/30 | KM median nodes | median wall ms (machine) | time-cens |", "|---|---|---|---|---|---|"]
        for r in sorted([x for x in rows if "rank" in x], key=lambda x: x["rank"]):
            a = r["agg"]
            st = f'{a["solved"]}' + (f' ({a["solved_range"][0]}-{a["solved_range"][1]})' if len(r["seeds"]) > 1 else "")
            cm.append(f'| {r["rank"]} | {r["entrant"]}{" (seeds " + ",".join(map(str, r["seeds"])) + ")" if r["stochastic"] else ""} | {st} | {fmt(a["median"])} | {a["median_wall_ms"]:,.0f} ({a["wall_machine"]}) | {a["time_censored"]} |')
        inc = [x for x in rows if not x["complete"]]
        if inc:
            cm += ["", f"Incomplete at {rung} (not ranked): " + "; ".join(f'{x["entrant"]} {x["agg"]["solved"]} ({x["agg"]["solved_range"][0]}-{x["agg"]["solved_range"][1]})/30, KM median {fmt(x["agg"]["median"])}' for x in inc)]
        cm.append("")
    with open(os.path.join(ROOT, "results/ranked-compact.md"), "w") as fh:
        fh.write("\n".join(cm))
    with open(os.path.join(ROOT, "results/tables.md"), "w") as fh:
        fh.write("\n".join(md))

    def clean(o):
        if isinstance(o, float) and o == INF:
            return "inf"
        if isinstance(o, dict):
            return {str(k): clean(v) for k, v in o.items()}
        if isinstance(o, (list, tuple)):
            return [clean(v) for v in o]
        return o
    with open(os.path.join(ROOT, "results/summary.json"), "w") as fh:
        json.dump(clean(summary), fh, indent=1)
    print("\n".join(md))


if __name__ == "__main__":
    main()
