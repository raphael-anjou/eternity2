#!/usr/bin/env python3
"""Apply the pre-registered checks (PREREG.md) and write results/summary.json.

Inputs (all under results/): reduced_m1_m3.jsonl (Rust M1 + M3),
reduced_m2.jsonl (Python M2), official_p{0,1,2}.json (Rust M1 residues) and
official_p{0,1,2}.time (/usr/bin/time -v output).
"""
import json
import math
import os
import re
import sys
from fractions import Fraction
from math import factorial, prod

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.join(HERE, "..", "results")
MC = os.path.join(HERE, "..", "..", "..", "topics", "ring-purity", "results", "ring_count.json")
PRIMES = [2305843009213693951, 2305843009213693921, 2305843009213693907]
GIB = 1024 ** 3
RSS_LIMIT_GIB = 30


def det(m):
    m = [[Fraction(x) for x in row] for row in m]
    n, d = len(m), Fraction(1)
    for c in range(n):
        p = next((r for r in range(c, n) if m[r][c] != 0), None)
        if p is None:
            return Fraction(0)
        if p != c:
            m[c], m[p] = m[p], m[c]
            d = -d
        d *= m[c][c]
        for r in range(c + 1, n):
            f = m[r][c] / m[c][c]
            for k in range(c, n):
                m[r][k] -= f * m[c][k]
    return d


def euler_circuits(arcs, nv):
    """BEST: cyclic Euler circuits of a balanced labelled multigraph."""
    out = [0] * nv
    for a, _ in arcs:
        out[a] += 1
    root = 0
    rest = [v for v in range(nv) if v != root]
    idx = {v: i for i, v in enumerate(rest)}
    lap = [[0] * len(rest) for _ in rest]
    for a, b in arcs:
        if a == root:
            continue
        lap[idx[a]][idx[a]] += 1
        if b != root:
            lap[idx[a]][idx[b]] -= 1
    arb = int(det(lap))
    return arb, arb * prod(factorial(o - 1) for o in out)


def crt(r1, p1, r2, p2):
    inv = pow(p1, -1, p2)
    x = r1 + p1 * (((r2 - r1) * inv) % p2)
    return x % (p1 * p2)


def time_v(path):
    txt = open(path).read()
    rss_kb = int(re.search(r"Maximum resident set size \(kbytes\): (\d+)", txt).group(1))
    wall = re.search(r"Elapsed \(wall clock\) time \(h:mm:ss or m:ss\): (\S+)", txt).group(1)
    parts = [float(x) for x in wall.split(":")]
    secs = sum(x * 60 ** i for i, x in enumerate(reversed(parts)))
    status = int(re.search(r"Exit status: (\d+)", txt).group(1))
    return rss_kb, secs, status


def main():
    red = [json.loads(l) for l in open(os.path.join(RES, "reduced_m1_m3.jsonl")) if l.strip()]
    m2 = {(d["n"], d["colors"], d["seed"]): d
          for d in (json.loads(l) for l in open(os.path.join(RES, "reduced_m2.jsonl")) if l.strip())}
    rows, k1_mismatch, m2_mismatch, rot_mismatch, res_mismatch = [], [], [], [], []
    for d in red:
        key = (d["n"], d["colors"], d["seed"])
        n1 = int(d["N_M1"])
        n3 = None if d["N_M3"] is None else int(d["N_M3"])
        n2 = int(m2[key]["N_M2"])
        if n3 is not None and n1 != n3:
            k1_mismatch.append(key)
        if n1 != n2 or m2[key]["W_sigma"] != d["W_sigma"]:
            m2_mismatch.append(key)
        if n1 != int(d["N_M1_cyc6x4"]):
            rot_mismatch.append(key)
        w6 = sum(int(w) for s, w in zip(d["sigmas"], d["W_sigma"]) if s[0] == 0)
        if [w6 % p for p in PRIMES] != d["W6_residues"]:
            res_mismatch.append(key)
        rows.append({
            "n": d["n"], "colors": d["colors"], "seed": d["seed"], "frame_colours": d["nv"],
            "edge_types": len(d["types"]), "N_M1": str(n1), "N_M2": str(n2),
            "N_M3": None if n3 is None else str(n3), "M3_nodes": d["M3_nodes"],
            "M3_capped": d["M3_capped"], "max_layer_states": d["max_layer_states"],
            "M1_seconds": d["M1_seconds"], "M2_seconds": m2[key]["M2_seconds"],
            "M3_seconds": d["M3_seconds"],
        })
    finished = sum(1 for r in rows if not r["M3_capped"])

    out = {
        "reduced": {
            "instances": len(rows), "M3_finished_under_cap": finished,
            "K1_mismatches_M1_vs_M3": [list(k) for k in k1_mismatch],
            "M1_vs_M2_mismatches": [list(k) for k in m2_mismatch],
            "rotation_24_vs_4x6_mismatches": [list(k) for k in rot_mismatch],
            "residue_mismatches": [list(k) for k in res_mismatch],
            "nonzero_instances": sum(1 for r in rows if r["N_M1"] != "0"),
            "rows": rows,
        }
    }

    offs = []
    for i in range(3):
        p = os.path.join(RES, f"official_p{i}.json")
        if os.path.exists(p):
            o = json.load(open(p))
            rss_kb, wall, status = time_v(os.path.join(RES, f"official_p{i}.time"))
            o.update({"peak_rss_kb": rss_kb, "wall_seconds": wall, "exit_status": status})
            offs.append(o)
    crt_agree = None
    if len(offs) == 3:
        o0 = offs[0]
        arcs = [tuple(a) for a in o0["corners"] + o0["edges"]]
        arb, ec = euler_circuits(arcs, o0["nv"])
        mc_all = json.load(open(MC))
        mc = dict(mc_all["corner_spacing"], **{k: v for k, v in mc_all.items() if k != "corner_spacing"})
        pf = int(o0["prod_mult_fact"])
        assert pf == prod(factorial(m) for m in o0["mult"])
        bound = 4 * ec // pf
        res = [sum(o["W_sigma_mod_p"]) % o["prime"] for o in offs]
        recon = {}
        for a, b in [(0, 1), (0, 2), (1, 2)]:
            assert bound < PRIMES[a] * PRIMES[b], "CRT bound violated"
            recon[f"{a}{b}"] = crt(res[a], PRIMES[a], res[b], PRIMES[b])
        crt_agree = len(set(recon.values())) == 1
        w6 = recon["01"]
        per_sigma = []
        for j in range(6):
            r = [o["W_sigma_mod_p"][j] for o in offs]
            per_sigma.append(str(crt(r[0], PRIMES[0], r[1], PRIMES[1])))
        n_cyc = pf * w6
        n = 4 * n_cyc
        f_exact = Fraction(n_cyc, ec)
        se = mc["std_error"]
        z = (mc["fraction"] - float(f_exact)) / se
        lo, hi = mc["fraction"] - 1.96 * se, mc["fraction"] + 1.96 * se
        peak = max(o["peak_rss_kb"] for o in offs)
        out["official"] = {
            "frame_colours": o0["nv"], "edge_types": o0["types"], "mult": o0["mult"],
            "corners": o0["corners"], "prod_mult_fact": str(pf),
            "arborescences": arb, "arborescences_matches_committed": arb == mc["arborescences"],
            "euler_circuits_EC": str(ec), "log10_EC": math.log10(ec),
            "log10_EC_matches_committed": abs(math.log10(ec) - mc["log10_euler_circuits_BEST"]) < 1e-9,
            "W6_residues": res, "crt_reconstructions": {k: str(v) for k, v in recon.items()},
            "crt_bound_4EC_over_prod_fact": str(bound), "crt_agree": crt_agree,
            "W_sigma_cyclic": per_sigma, "sigmas_cyclic": o0["sigmas"],
            "W6": str(w6), "N_cyc": str(n_cyc), "N": str(n), "log10_N": math.log10(n),
            "f_exact": float(f_exact), "f_exact_fraction": f"{f_exact.numerator}/{f_exact.denominator}",
            "mc_fraction": mc["fraction"], "mc_se": se,
            "mc_log10_placements": mc["log10_valid_frame_placements"],
            "z_mc_minus_exact_over_se": z, "mc_95_interval": [lo, hi],
            "inside_mc_95": lo <= float(f_exact) <= hi,
            "max_layer_states": max(o["max_layer_states"] for o in offs),
            "max_join_states": max(o["max_join_states"] for o in offs),
            "per_prime_wall_seconds": [o["wall_seconds"] for o in offs],
            "per_prime_peak_rss_kb": [o["peak_rss_kb"] for o in offs],
            "peak_rss_gib": peak / 1024 ** 2,
            "exit_statuses": [o["exit_status"] for o in offs],
        }
        k2_fail = peak * 1024 > RSS_LIMIT_GIB * GIB or any(o["exit_status"] != 0 for o in offs)
    else:
        k2_fail = None
    k1_fail = bool(k1_mismatch)
    if k1_fail or k2_fail or crt_agree is False:
        outcome = "killed"
    elif finished < 30 or k2_fail is None:
        outcome = "inconclusive"
    else:
        outcome = "survived"
    out["kill"] = {"K1_disagreement": k1_fail, "K2_memory": k2_fail, "CRT_disagreement": crt_agree is False,
                   "validation_thin": finished < 30, "self_assessed_outcome": outcome}
    json.dump(out, open(os.path.join(RES, "summary.json"), "w"), indent=1)
    print(json.dumps(out["kill"]))
    if "official" in out:
        o = out["official"]
        print("N =", o["N"], "log10 =", o["log10_N"], "f =", o["f_exact"], "z =", o["z_mc_minus_exact_over_se"])
    return 0


if __name__ == "__main__":
    sys.exit(main())
