"""TOY. Prints one table per board from raw/res_*.json (5x5: one file per board; 6x6: .sdp.json and
.base.json per board). Usage: python3 -I summarise.py > summary.txt"""
import json, os, glob

HERE = os.path.dirname(os.path.abspath(__file__))
RAW = os.path.join(HERE, "raw")


def load(tag):
    r = {}
    for f in sorted(glob.glob(os.path.join(RAW, f"res_{tag}*.json"))):
        d = json.load(open(f))
        for k, v in d.items():
            if isinstance(v, dict) and k in r and isinstance(r[k], dict):
                r[k].update(v)
            else:
                r[k] = v
    return r


def f(v, nd=2):
    if v is None:
        return "n/a"
    if isinstance(v, float):
        return f"{v:.{nd}f}"
    return str(v)


def row(name, d, kind):
    if not d:
        return f"| {name} | not run |"
    cert = d.get("certificates", {})
    kb = cert.get("kills_beyond_ac", {})
    lk = cert.get("leak_off_solutions", {})
    kills = f"{d.get('killed_ac_keeps@0.0001', 'n/a')}"
    if kb and kb.get("rounds"):
        kills += f" (certified {kb.get('certified_zero')})" if kb.get("finished") else " (certificate unfinished)"
    leak = "n/a"
    if lk.get("rounds"):
        lr = lk["rounds"][-1]
        leak = f"{lr['max_sum']:.3g}" + (" (solve inaccurate)" if "inaccurate" in lr["status"] else "")
    rnd = d.get("rounding_on_x", {})
    return ("| " + " | ".join([
        name,
        f(d.get("status", "")) + (f", {d.get('iters')} it" if "iters" in d else ""),
        f(d.get("bits_sol_mean")), f(d.get("bits_annealed_mean")), f(d.get("z_vs_annealed")),
        f(d.get("bits_sampled_mean")), f(d.get("z_vs_sampled")), f(d.get("z_vs_random")),
        kills, f(d.get("kept_ac_kills@0.0001")),
        f(d.get("mass_off_solution_placements_per_cell"), 4), f(d.get("hull_distance_Linf"), 4), leak,
        "yes" if rnd.get("is_solution") else ("no" if rnd else "n/a"),
    ]) + " |")


HDR = ("| point | solver status | bits on solutions | bits annealed | z vs annealed | bits sampled | z vs sampled | "
       "z vs random | placements x<1e-4 that AC keeps | placements x>=1e-4 that AC kills | x mass per cell off "
       "solution placements | hull distance (Linf) | max sum of x off solutions (LP/SDP certificate) | rounding gives a solution |")
SEP = "|" + "---|" * 14

for tag in ["5_3_4_0", "5_3_4_1", "5_3_4_2", "6_4_5_0", "6_4_5_1", "6_4_5_2"]:
    r = load(tag)
    if not r:
        print(f"\n## board {tag}: no result\n"); continue
    print(f"\n## board ({tag.replace('_', ', ')}): {r.get('n_sols')} solutions, P = {r.get('P')} placements, "
          f"log2 DFS nodes {r.get('log2_dfs_nodes')}, controls annealed {r.get('n_annealed')}, sampled {r.get('n_sampled')}, "
          f"pin {r.get('pin')}\n")
    print(HDR); print(SEP)
    print(row("SDP lift", r.get("sdp"), "sdp"))
    print(row("LP (no PSD)", r.get("lp"), "lp"))
    print(row("arc consistency (Sinkhorn of AC domains)", r.get("ac"), "ac"))
    fl = r.get("flow", {}).get("median")
    if fl:
        print("| probe B flow (median of 4 runs) | n/a | " + " | ".join(f(fl.get(k)) for k in
              ["bits_sol_mean", "bits_annealed_mean", "z_vs_annealed", "bits_sampled_mean", "z_vs_sampled", "z_vs_random"])
              + " | n/a | n/a | n/a | n/a | n/a | n/a |")
    s = r.get("sdp", {})
    if s:
        print(f"\nSDP: top eigenvalues {s.get('top_eigs')}, rank@1e-3 {s.get('rank@1e-3')}, rank of the solutions' span "
              f"{s.get('rank_of_solution_span')}, min eig {f(s.get('min_eig'), 8)}, res_pri {f(s.get('res_pri'), 8)}, "
              f"res_dual {f(s.get('res_dual'), 8)}, solve {s.get('solve_seconds')} s, peak RSS {s.get('max_rss_gb_so_far')} GB, "
              f"free pairs {s.get('free_pairs')}, eigvec rounding {s.get('rounding_on_top_eigvec')}, "
              f"min x on solution placements {f(s.get('min_x_on_solution_placements'), 4)}, "
              f"certificates {json.dumps(s.get('certificates'))}")
    l_ = r.get("lp", {})
    if l_:
        print(f"\nLP: res_pri {f(l_.get('res_pri'), 8)}, solve {l_.get('solve_seconds')} s, certificates {json.dumps(l_.get('certificates'))}")
    a = r.get("ac", {})
    if a:
        print(f"\nAC: alive {a.get('ac_alive')} of {a.get('placements')}, in some solution {a.get('in_some_solution')}, seconds {a.get('seconds')}")
