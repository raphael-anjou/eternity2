"""Summarise admm.py logs. Usage: python3 -I summarise_admm.py raw/admm_*.jsonl"""
import json, sys
for f in sys.argv[1:]:
    rows = [json.loads(l) for l in open(f) if l.startswith("{")]
    its = [d for d in rows if "it" in d]
    fin = rows[-1] if "final" in rows[-1] else {}
    first = {th: next((d["it"] for d in its if d["mass_off_per_cell"] < th), None) for th in (1e-2, 1e-3, 1e-5)}
    last = its[-1]
    print(f, "| board", fin.get("board"), "rho", fin.get("rho"), "fr", fin.get("fr"), "aa", fin.get("aa"),
          "| iters", fin.get("iters"), "s", fin.get("seconds"), "ms/it", fin.get("ms_per_iter"), "eig share", fin.get("eig_share"),
          "| first logged it with mass off < 1e-2, 1e-3, 1e-5:", first[1e-2], first[1e-3], first[1e-5],
          "| last: viol %.1e mass off %.1e obj %.5f npos %d linf_vs_scs %s" % (last["viol_max"], last["mass_off_per_cell"], last["obj"], last["npos"], last.get("linf_vs_scs")),
          "| rank", fin.get("rank_1e3"), "sols", fin.get("n_sols"), "obj_scs", fin.get("obj_scs"), "load", fin.get("loadavg"))
