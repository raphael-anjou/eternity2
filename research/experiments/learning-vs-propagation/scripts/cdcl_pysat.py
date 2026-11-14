"""learning-vs-propagation: CaDiCaL 1.5.3 through python-sat,
all solutions by blocking clauses over the placement variables until UNSAT. Records conflicts and decisions at the
first solution and at exhaustion. Stops cleanly after LIMIT wall seconds (censored record, counts are lower bounds).
usage: uv run --with python-sat python3 -I cdcl_pysat.py board.cnf LIMIT_SECONDS [solver]"""
import json, sys, time
from pysat.formula import CNF
from pysat.solvers import Solver
path, limit = sys.argv[1], float(sys.argv[2])
name = sys.argv[3] if len(sys.argv) > 3 else "cadical153"
xs = set()
for line in open(path):
    if line.startswith("c placement vars"):
        xs = set(int(v) for v in line.split()[3:])
        break
cnf = CNF(from_file=path)
t0 = time.time()
sols, first, censored = 0, None, 0
with Solver(name=name, bootstrap_with=cnf.clauses) as S:
    while True:
        S.conf_budget(200000)
        res = S.solve_limited()
        if res is None:
            if time.time() - t0 > limit:
                censored = 1
                break
            continue
        if not res:
            break
        sols += 1
        if first is None:
            st = S.accum_stats()
            first = {"conflicts": st["conflicts"], "decisions": st["decisions"], "sec": round(time.time() - t0, 2)}
        S.add_clause([-v for v in S.get_model() if v > 0 and v in xs])
        if time.time() - t0 > limit:
            censored = 1
            break
    st = S.accum_stats()
print(json.dumps({"board": path.split("/")[-1], "arm": "cadical", "solver": name, "sols": sols, "conflicts": st["conflicts"],
                  "decisions": st["decisions"], "propagations": st["propagations"], "first": first, "censored": censored,
                  "sec": round(time.time() - t0, 2)}))
