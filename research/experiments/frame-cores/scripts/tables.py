"""Markdown tables for the README from results/summary.json.

  python3 -I tables.py results/summary.json
"""
import json
import math
import sys


def f(x, nd=2):
    if x is None:
        return "n/a"
    if isinstance(x, float) and math.isinf(x):
        return "inf"
    if isinstance(x, float):
        if x != 0 and (abs(x) >= 1e4 or abs(x) < 1e-2):
            return "%.2e" % x
        return ("%." + str(nd) + "f") % x
    return str(x)


def main():
    d = json.load(open(sys.argv[1]))
    print("node rate r (nodes/s, capped R0-rows runs):", d["node_rate"])
    print()
    print("| set | log10 N_F | log10 N_H | beta_H (bits) |")
    print("|---|---|---|---|")
    for s, b in d["base"].items():
        print("| %s | %s | %s | %s |" % (s, f(b["log10_NF"], 3), f(b["log10_NH"], 3), f(b["beta_H_bits"])))
    print()
    print("Stage 1: lower median of t_best_plain / t_X, 95% bootstrap interval, censored pairs")
    print()
    print("| set/stratum | n | R1-rows | R1-spiral | R2 | total-time ratio R1-rows / R1-spiral / R2 | plain censored |")
    print("|---|---|---|---|---|---|---|")
    for k, v in d["stage1"].items():
        cells = []
        n = None
        for x in ("R1-rows", "R1-spiral", "R2-glucose"):
            if x in v:
                n = v[x]["n"]
                cells.append("%s [%s, %s] (%d)" % (f(v[x]["lower_median_ratio"]), f(v[x]["ci95"][0]), f(v[x]["ci95"][1]), v[x]["censored_pairs"]))
            else:
                cells.append("dropped")
        tr = " / ".join(f(v[x]["total_ratio"]) if x in v else "n/a" for x in ("R1-rows", "R1-spiral", "R2-glucose"))
        pc = next(iter(v.values()))["plain_censored"]
        print("| %s | %s | %s | %s | %s |" % (k, n, " | ".join(cells), tr, pc))
    print()
    print("Stage 2: median v per frame (b_H for H, b_F for U), 95% interval, R2 core size")
    print()
    print("| set/stratum | n | censored | median v (bits) | 95% CI | secondary: median v_F - beta_H | 95% CI | median R2 core cells | R2 core quartiles | median best-nogood cells | reading | L (secondary, b_F) |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|")
    for k, v in d["stage2"].items():
        print("| %s | %s | %s | %s | [%s, %s] | %s | [%s, %s] | %s | %s | %s | %s | %s |" % (
            k, v["n"], v["censored"], f(v["median_v"]), f(v["ci95"][0]), f(v["ci95"][1]),
            f(v["secondary_median_vF_minus_beta"]), f(v["secondary_ci95"][0]), f(v["secondary_ci95"][1]),
            f(v["median_r2_core_size"]), v["r2_core_size_quartiles"], f(v["median_best_nogood_size"]),
            v["reading"], f(d["secondary_L"].get(k))))
    print()
    print("A2 floors (log2 mean of 2^v lower bounds)")
    print()
    print("| set/stratum | F(0.4) | F(0.25) | F(0.1) | log2 mean | best | reading |")
    print("|---|---|---|---|---|---|---|")
    for k, v in d["stage2"].items():
        fl = v.get("floors_A2")
        if fl:
            print("| %s | %s | %s | %s | %s | %s | %s |" % (k, f(fl["F(0.4)"]), f(fl["F(0.25)"]), f(fl["F(0.1)"]),
                  f(fl["log2_mean"]), f(fl["best"]), fl["reading_A2"]))
    if "arcs" in d:
        print()
        print("Bottom arcs (A1 item 4), real H frames")
        print()
        print("| arc | cells | refuter | frames | unsat | within 60 s | within 65 s | sat | capped | median s (unsat) | median b_H | median b_F |")
        print("|---|---|---|---|---|---|---|---|---|---|---|---|")
        for k, v in d["arcs"].items():
            if k in ("readings_A1", "A2_nested"):
                continue
            arc, ref = k.split("/")
            print("| %s | %d | %s | %d | %d | %d | %d | %d | %d | %s | %s | %s |" % (
                arc, v["cells"], ref, v["frames"], v["unsat"], v["unsat_within_60s"], v["unsat_within_65s"],
                v["sat"], v["capped"], f(v["median_secs_unsat"]), f(v["median_b_H_bits"]), f(v["median_b_F_bits"])))
        print()
        print("arc readings (A1):", d["arcs"]["readings_A1"])
        print()
        print("| arc | cells | exponent to parent | mean c (s) | log2 mean c (nodes) | budget (log2 nodes) | budget (s) | can kill at cap | below break-even |")
        print("|---|---|---|---|---|---|---|---|---|")
        for k, v in d["arcs"]["A2_nested"].items():
            print("| %s | %d | %s | %s | %s | %s | %s | %s | %s |" % (k, v["cells"], f(v["exponent"], 1), f(v["mean_c_s"]),
                  f(v["log2_mean_c_nodes"], 1), f(v["budget_log2"], 1), f(v["budget_s"]), v["can_kill_at_cap"], v["below_break_even"]))
    print()
    print("uniform real frames:", d.get("uniform_real"))
    print()
    print("Cores (the minimising nogood per frame)")
    print()
    print("| set/stratum | frames | refuted | source R2 / R0-rows / R0-spiral | contiguous | contains a corner cell | median ring distance to a corner | median b (bits) | median log2 C | R2 refuted | R2 median conflicts |")
    print("|---|---|---|---|---|---|---|---|---|---|---|")
    groups = {}
    for fr in d["frames"]:
        groups.setdefault("%s/%s" % (fr["set"], fr["stratum"]), []).append(fr)
    for k, frs in groups.items():
        ok = [x for x in frs if not x["censored"]]

        def med(xs):
            xs = sorted(xs)
            return xs[len(xs) // 2] if xs else None
        src = [sum(1 for x in ok if x["source"] == s) for s in ("R2-glucose", "R0-rows", "R0-spiral")]
        r2 = [x["r2"] for x in frs if x.get("r2")]
        r2ok = [x for x in r2 if x["verdict"] == "unsat"]
        print("| %s | %d | %d | %s | %d | %d | %s | %s | %s | %d | %s |" % (
            k, len(frs), len(ok), " / ".join(map(str, src)), sum(1 for x in ok if x["contiguous"]),
            sum(1 for x in ok if x["has_corner"]), f(med([x["corner_distance"] for x in ok])),
            f(med([x["b_bits"] for x in ok])), f(med([x["log2C"] for x in ok])), len(r2ok),
            f(med([x["conflicts"] for x in r2ok]))))
    print()
    print("verdict:", d["verdict"])


if __name__ == "__main__":
    main()
