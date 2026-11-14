"""Second analysis script 2b. Per prefix at depths 112 to 144: proven dead, arc consistency, and (read
from r6_cell*.out) the cell-level linear relaxation, side by side.
Run: python3 -I r2b_depth112_flags.py"""
import sys, os, json, re
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import r2a_prefix_death as r2a
from rcommon import load
P, _ = load()
cell = {}
for f in ("r6_cell.out", "r6_cell_depth112.out"):
    for line in open(os.path.join(HERE, f)):
        m = re.match(r"(\d+) (\d+) (True|False) ", line)
        if m:
            cell[(int(m.group(1)), int(m.group(2)))] = m.group(3) == "False"
from collections import Counter
tab = {}
for s, d, pf in json.load(open(os.path.join(HERE, "prefixes_noclue.json"))):
    if d not in (112, 128, 144):
        continue
    pref = [(i, tuple(rt)) for i, rt in pf]
    st, n, _ = r2a.exact_below(P, pref, 400000)
    ac = r2a.ac_refutes(P, pref)
    key = ("dead" if st == "dead" else "not exhausted", "AC refutes" if ac else "AC passes",
           "cell LP refutes" if cell.get((s, d)) else "cell LP feasible")
    tab.setdefault(d, Counter())[key] += 1
for d in sorted(tab):
    for k, v in sorted(tab[d].items()):
        print(d, k, v)
