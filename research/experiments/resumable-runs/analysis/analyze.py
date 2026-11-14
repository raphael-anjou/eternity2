#!/usr/bin/env python3
"""Track 9 analysis (stdlib only). Reads results/raw/*.jsonl, writes results/summary.json.

Gate checks (PREREG.md): G-a, G-b, G-c, G-d; timing ratio with bootstrap CI;
checkpoint write / replay cost by depth.
"""
import json
import random
import statistics
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
import sys
# Optional: analyze.py [RAW_DIR] [SUMMARY_OUT] (the fix round reads results/fix/raw).
RAW = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else HERE / "results" / "raw"
OUT = Path(sys.argv[2]).resolve() if len(sys.argv) > 2 else HERE / "results" / "summary.json"
KEYS = ["status", "nodes", "backtracks", "propagation", "board", "hash", "hashed"]
SETS = ["r6-cal", "r6-test", "r7-cal", "r7-test", "r8-cal", "r8-test", "r8-fresh"]
GATE_PE = 100_000


def rows(p):
    p = RAW / p
    if not p.exists():
        return None
    return [json.loads(l) for l in p.read_text().splitlines() if l.strip()]


def upper_median(xs):
    s = sorted(xs)
    return s[len(s) // 2]


def check_progress(lines, final_nodes, censored, pe):
    """Progress lines of one instance: node counts are exactly the multiples
    k * pe with k * pe < final (or <= final when censored), and closed share
    and node counts never decrease."""
    want = [k * pe for k in range(1, final_nodes // pe + 1)
            if k * pe < final_nodes or (censored and k * pe == final_nodes)]
    got = [l["nodes"] for l in lines]
    shares = [l["closed_share"] for l in lines]
    mono = all(a <= b for a, b in zip(shares, shares[1:]))
    in_range = all(0.0 <= s <= 1.0 for s in shares)
    return got == want and mono and in_range


def strip(l):
    return {k: v for k, v in l.items() if k not in ("elapsed_s", "segment")}


def gate_set(name, pe, cap):
    ga = rows(f"ga-{name}.jsonl")
    if ga is None:
        return None
    pa = rows(f"progress-ga-{name}.jsonl") or []
    by_inst_pa = {}
    for l in pa:
        by_inst_pa.setdefault(l["instance"], []).append(l)
    out = {"instances": len(ga), "ga_mismatch": [], "ga_progress_bad": [],
           "share_at_cap": [], "progress_lines": len(pa)}
    for r in ga:
        f, res = r["fast"], r["resumable"]
        if not (r["equal"] and all(f[k] == res[k] for k in KEYS)):
            out["ga_mismatch"].append(r["instance"])
        cens = f["status"] == "censored"
        if not check_progress(by_inst_pa.get(r["instance"], []), f["nodes"], cens, pe):
            out["ga_progress_bad"].append(r["instance"])
        if cens:
            out["share_at_cap"].append(res["closed_share"])
    gb = [rows(f"gb-{name}-s{i}.jsonl") for i in (1, 2, 3)]
    if any(x is None for x in gb):
        return out
    final, segs = {}, {}
    for i, rs in enumerate(gb, 1):
        for r in rs:
            final[r["instance"]] = r
            segs.setdefault(r["instance"], []).append(r)
    pb = rows(f"progress-gb-{name}.jsonl") or []
    by_inst_pb = {}
    for l in pb:
        by_inst_pb.setdefault(l["instance"], []).append(l)
    classes = {"solved_seg1": 0, "solved_seg2": 0, "solved_seg3": 0, "censored_at_C": 0, "other": 0}
    gb_bad, gb_prog_bad, chain_bad = [], [], []
    for r in ga:
        i = r["instance"]
        f, res = r["fast"], r["resumable"]
        fin = final.get(i)
        ok = fin is not None and all(fin[k] == f[k] for k in KEYS) \
            and fin["closed_share"] == res["closed_share"] and fin["deepest"] == res["deepest"]
        if not ok:
            gb_bad.append(i)
        if [strip(l) for l in by_inst_pb.get(i, [])] != [strip(l) for l in by_inst_pa.get(i, [])]:
            gb_prog_bad.append(i)
        # Every non-final segment is censored and resumable, segments are 1, 2, 3 in order,
        # and the per-segment node counts sum to the total.
        ss = segs.get(i, [])
        chain = [s["segment"] for s in ss] == list(range(1, len(ss) + 1)) \
            and all(s["status"] == "censored" and s["resumable"] for s in ss[:-1]) \
            and sum(s["nodes_segment"] for s in ss) == (fin["nodes"] if fin else -1) \
            and sum(s["backtracks_segment"] for s in ss) == (fin["backtracks"] if fin else -1) \
            and sum(s["propagation_segment"] for s in ss) == (fin["propagation"] if fin else -1)
        if not chain:
            chain_bad.append(i)
        if fin is None:
            classes["other"] += 1
        elif fin["status"] == "solved":
            classes[f"solved_seg{fin['segment']}"] += 1
        elif fin["status"] == "censored" and fin["nodes"] == cap:
            classes["censored_at_C"] += 1
        else:
            classes["other"] += 1
    out.update({"gb_mismatch": gb_bad, "gb_progress_mismatch": gb_prog_bad,
                "gb_segment_chain_bad": chain_bad, "classes": classes})
    return out


def ckpt_cost():
    """Checkpoint write cost (from the row of the segment that wrote it at its cap) and
    replay cost (from the row of the segment that resumed from it), by depth."""
    writes, replays = [], []
    for p in sorted(RAW.glob("gb-*-s?.jsonl")):
        prev = {}
        s = int(p.name[-7])
        if s > 1:
            for r in rows(p.name.replace(f"-s{s}.", f"-s{s-1}.")):
                prev[r["instance"]] = r
        for r in rows(p.name):
            if r["checkpoint_write_ms"] is not None:
                writes.append({"n": r["n"], "depth": r["depth"], "ms": r["checkpoint_write_ms"],
                               "bytes": r["checkpoint_bytes"]})
            if r["resumed_from"] and r["instance"] in prev:
                replays.append({"n": r["n"], "depth": prev[r["instance"]]["depth"],
                                "ms": r["replay_ms"]})
    def table(xs, key):
        bins = {}
        for x in xs:
            b = (x["n"], x["depth"] // 10 * 10)
            bins.setdefault(b, []).append(x)
        return [{"n": n, "depth_bin": f"{d}-{d+9}", "count": len(v),
                 "median_ms": statistics.median(x["ms"] for x in v),
                 "max_ms": max(x["ms"] for x in v),
                 **({"median_bytes": statistics.median(x["bytes"] for x in v),
                     "max_bytes": max(x["bytes"] for x in v)} if key == "w" else {})}
                for (n, d), v in sorted(bins.items())]
    return {"writes": len(writes), "replays": len(replays),
            "write_by_depth": table(writes, "w"), "replay_by_depth": table(replays, "r")}


def timing():
    rs = (rows("time-r8-test.jsonl") or []) + (rows("time-r8-fresh.jsonl") or [])
    if not rs:
        return None
    per = {}
    for r in rs:
        per.setdefault(r["instance"], {}).setdefault(r["solver"], []).append(r)
    ratios, ratios_med, same = [], [], True
    tot_a = tot_b = 0.0
    for i, d in sorted(per.items()):
        a, b = d["bb-mrv-fast"], d["bb-mrv-fast-resumable"]
        if len({(x["nodes"], x["status"]) for x in a + b}) != 1:
            same = False
        ma, mb = min(x["wall_ms"] for x in a), min(x["wall_ms"] for x in b)
        tot_a += ma
        tot_b += mb
        ratios.append(mb / ma)
        ratios_med.append(statistics.median(x["wall_ms"] for x in b)
                          / statistics.median(x["wall_ms"] for x in a))

    def boot(xs):
        rng = random.Random(20261111)
        meds = sorted(upper_median([rng.choice(xs) for _ in xs]) for _ in range(10_000))
        return [meds[249], meds[9749]]
    return {"instances": len(ratios), "nodes_status_identical": same,
            "ratio_min_median": upper_median(ratios), "ratio_min_ci95": boot(ratios),
            "ratio_med_median": upper_median(ratios_med), "ratio_med_ci95": boot(ratios_med),
            "sum_min_ms_fast": tot_a, "sum_min_ms_resumable": tot_b,
            "accept_upper_ci_le_1_03": boot(ratios)[1] <= 1.03}


def gate_d():
    d = rows("gd-x86.jsonl")
    ga = rows("ga-n9.jsonl")
    if d is None or ga is None:
        return None
    ref = {r["instance"]: r["fast"] for r in ga}
    unint = {r["instance"]: r["resumable"] for r in ga}
    bad = [r["instance"] for r in d
           if not (all(r[k] == ref[r["instance"]][k] for k in KEYS)
                   and r["closed_share"] == unint[r["instance"]]["closed_share"]
                   and r["deepest"] == unint[r["instance"]]["deepest"])]
    host = RAW / "gd-x86-host.txt"
    return {"instances": [r["instance"] for r in d], "mismatch": bad,
            "resumed_from_nodes": sorted({r["nodes"] - r["nodes_segment"] for r in d}),
            "host": host.read_text().strip() if host.exists() else None}


def main():
    gates = {s: gate_set(s, GATE_PE, 1_000_000) for s in SETS}
    gates["n9"] = gate_set("n9", GATE_PE, 3_000_000)
    summ = {"gate": gates, "gate_d": gate_d(), "checkpoint_cost": ckpt_cost(), "timing": timing()}
    def fails(g):
        return g is None or any(g.get(k) for k in ("ga_mismatch", "ga_progress_bad", "gb_mismatch",
                                                     "gb_progress_mismatch", "gb_segment_chain_bad")) \
            or "gb_mismatch" not in g
    summ["verdict"] = {
        "G-a+G-b (140 instances)": "pass" if not any(fails(gates[s]) for s in SETS) else "FAIL or incomplete",
        "G-c (9x9)": "pass" if not fails(gates["n9"]) else "FAIL or incomplete",
        "G-d": None if summ["gate_d"] is None else ("pass" if not summ["gate_d"]["mismatch"] else "FAIL"),
        "timing_accept": None if summ["timing"] is None else summ["timing"]["accept_upper_ci_le_1_03"],
    }
    OUT.write_text(json.dumps(summ, indent=1) + "\n")
    print(json.dumps(summ["verdict"], indent=1))
    for s, g in gates.items():
        if g:
            print(s, g["instances"], "ga", len(g["ga_mismatch"]), len(g["ga_progress_bad"]),
                  "gb", len(g.get("gb_mismatch", ["?"])), len(g.get("gb_progress_mismatch", ["?"])),
                  len(g.get("gb_segment_chain_bad", ["?"])), g.get("classes"))
    if summ["timing"]:
        t = summ["timing"]
        print("timing", t["instances"], t["ratio_min_median"], t["ratio_min_ci95"], t["nodes_status_identical"])


if __name__ == "__main__":
    main()
