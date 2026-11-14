"""learning-real-prefixes: CaDiCaL 1.5.3 through python-sat, all solutions by blocking clauses over the placement
variables until UNSAT. After ../learning-vs-propagation/cdcl_pysat.py, changed: the clauses are streamed into the
solver, the conflict budget per call is 5,000 (was 200,000) so the WALL limit, counted from process start, is checked
every few seconds, a line of counts so far is printed (flushed) at most every 5 s and on SIGTERM, and wall and
processor seconds of the solve phase are recorded. The last line of output holds the counts; "final":1 marks a clean
end. usage: uv run --with python-sat python3 -I cdcl_pysat.py board.cnf LIMIT_WALL_SECONDS [budget]"""
import json, signal, sys, time
T0 = time.monotonic()
from pysat.solvers import Solver
path, limit = sys.argv[1], float(sys.argv[2])
budget = int(sys.argv[3]) if len(sys.argv) > 3 else 5000
stop = []
signal.signal(signal.SIGTERM, lambda *a: stop.append(1))
S = Solver(name="cadical153")
nx, empty, ncl = 0, False, 0
for line in open(path):
    c = line[0]
    if c == "c":
        if line.startswith("c nplacement"):
            nx = int(line.split()[2])
        continue
    if c == "p":
        continue
    lits = [int(t) for t in line.split()]
    lits.pop()
    ncl += 1
    if lits:
        S.add_clause(lits)
    else:
        empty = True
t1, c1 = time.monotonic(), time.process_time()
sols, censored, last = 0, 0, [t1]
st = {"conflicts": 0, "decisions": 0, "propagations": 0, "restarts": 0}


def emit(final):
    print(json.dumps({"arm": "K", "solver": "cadical153", "board": path.split("/")[-1], "clauses": ncl, "sols": sols, "conflicts": st["conflicts"],
                      "decisions": st["decisions"], "propagations": st["propagations"], "censored": censored, "final": final,
                      "load_wall": round(t1 - T0, 3), "wall": round(time.monotonic() - t1, 4), "cpu": round(time.process_time() - c1, 4),
                      "wall_total": round(time.monotonic() - T0, 3)}), flush=True)


while not empty:
    S.conf_budget(budget)
    res = S.solve_limited()
    st = S.accum_stats()
    over = time.monotonic() - T0 > limit or bool(stop)
    if res is None:
        if over:
            censored = 1
            break
        if time.monotonic() - last[0] > 5:
            last[0] = time.monotonic()
            emit(0)
        continue
    if not res:
        break
    sols += 1
    S.add_clause([-v for v in S.get_model() if 0 < v <= nx])
    if over:
        censored = 1
        break
emit(1)
S.delete()
