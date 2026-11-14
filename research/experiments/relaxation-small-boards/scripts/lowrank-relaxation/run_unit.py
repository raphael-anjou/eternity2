"""TOY BOARDS ONLY. One unit = one board, one rank r, a range of random starts (one process, so the
peak memory reported is this unit's). Writes one JSON line per start to out.jsonl.
Usage: run_unit.py n cf ci seed r start0 start1 out.jsonl [time_limit_per_start] [mode]
  mode: "random" (default), "solution" (positive control: start at a true solution's rank-one
  point, plus 1e-3 noise when r > 1), "broken" (negative control: the N side of the first inner
  tile recoloured to another inner colour, so that no solution exists; checked by enumeration).
"""
import os, sys, json, time, resource, platform
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import common  # noqa: F401  (local copy, loaded before board.py)
from common import gen, enumerate_all
import numpy as np
from board import Board
from bm import Structure, solve, init_U, U_of_solution, greedy_round
from measure import round_board, hull_distance

n, cf, ci, seed, r, s0, s1 = map(int, sys.argv[1:8])
out_path = sys.argv[8]
tlim = float(sys.argv[9]) if len(sys.argv) > 9 else 1500
mode = sys.argv[10] if len(sys.argv) > 10 else "random"
label = mode            # "random90": random start, 90-minute limit; "early90": same, plus early stop
early = mode == "early90"   # round every 2 outer steps, stop at the first rounding that is a solution
mode = "random" if mode in ("random90", "early90") else mode

t_board = time.time()
broken_info = None
if mode == "broken":
    # Swapping two sides of one tile leaves these few-colour toys solvable (checked on 5x5 seed 0:
    # every inner tile, 3 to 36 solutions remain). So one side of one inner tile is recoloured to
    # another inner colour instead: that colour then appears an odd number of times on the
    # half-edges, so no board can match every inner edge (parity), and enumeration confirms 0.
    tiles = gen(n, cf, ci, seed)
    for i, t in enumerate(tiles):
        if 0 not in t:
            nt = list(t); nt[0] = cf + 1 if t[0] != cf + 1 else cf + 2; nt = tuple(nt)
            canon = {min(u[k:] + u[:k] for k in range(4)) for j, u in enumerate(tiles) if j != i}
            if min(nt[k:] + nt[:k] for k in range(4)) in canon:
                continue
            tt = tiles[:i] + [nt] + tiles[i + 1:]
            if len(enumerate_all(n, tt)["sols"]) == 0:
                broken_info = dict(tile=i, before=list(t), after=list(nt))
                tiles = tt
                break
    assert broken_info is not None
    B = Board(n, cf, ci, seed, tiles=tiles)
else:
    B = Board(n, cf, ci, seed)
S = Structure(B)
t_board = time.time() - t_board
sol_used = np.zeros(B.P, bool)
for s in B.sol_idx:
    sol_used[s] = True
xs_path = os.path.join(HERE, "raw", f"x_sdp_{n}_{cf}_{ci}_{seed}.npy")
x_sdp = np.load(xs_path) if (os.path.exists(xs_path) and mode != "broken") else None


def rss_mb():
    return round(resource.getrusage(resource.RUSAGE_SELF).ru_maxrss / (2**20 if sys.platform == "darwin" else 2**10), 1)   # bytes on macOS, KiB on Linux


for start in range(s0, s1):
    rng = np.random.default_rng(1000 * r + start)
    if mode == "solution":
        U0 = U_of_solution(B, r, B.sol_idx[start % len(B.sol_idx)])
        if r > 1:      # a small push off the exact point, so the run is not trivially stationary
            U0 = U0 + 1e-3 * rng.standard_normal(U0.shape)
    else:
        U0 = init_U(B, r, rng)
    stop = None
    if early:
        def stop(xx):
            js = greedy_round(B, xx)
            if B.mismatches(js.tolist()) == 0:
                return True
            return bool(round_board(B, np.clip(xx, 0, None)).get("is_solution")) if B.sol_idx else False
    V, info = solve(S, r, U0, time_limit=tlim, stop_check=stop)
    x = V[:, 0]
    sv = np.linalg.svd(np.vstack([np.eye(1, r), V]), compute_uv=False)
    rec = dict(board=[n, cf, ci, seed], host=platform.node(), mode=mode, label=label, early_stop=early, time_limit=tlim, r=r, start=start, P=B.P, K=int(S.K), F=int(S.F),
               n_sols=len(B.sols), status=info["status"], seconds=info["seconds"], outer=info["outer"],
               evals=info["evals"], final_rho=info["final_rho"], viol=info["viol"],
               active_nonneg_multipliers=info["active_nonneg_multipliers"],
               obj=float(x @ x), rank_1e2=int((sv ** 2 > 1e-2 * sv[0] ** 2).sum()),
               rank_1e3=int((sv ** 2 > 1e-3 * sv[0] ** 2).sum()), top_eigs=[round(float(e), 4) for e in (sv ** 2)[:6]],
               peak_rss_mb_process=rss_mb(), board_setup_seconds=round(t_board, 1))
    if broken_info:
        rec["broken"] = broken_info
    js = greedy_round(B, x)
    m = B.mismatches(js.tolist())
    rec["greedy_round"] = dict(mismatches=int(m), is_solution=bool(m == 0))
    if B.sol_idx:
        xc = np.clip(x, 0, None)
        rec["mass_off_solution_placements_per_cell"] = float(xc[~sol_used].sum() / B.N)
        rec["min_x_on_solution_placements"] = float(x[sol_used].min())
        rec["hull_distance_Linf"] = round(hull_distance(B, xc), 5)
        rec["greedy_round"]["dist_to_nearest_solution"] = B.dist_to_sols(js)
        rec["hungarian_round"] = round_board(B, xc)
        if x_sdp is not None:
            rec["linf_vs_full_sdp"] = float(np.abs(x - x_sdp).max())
            rec["obj_full_sdp"] = float(x_sdp @ x_sdp)
    else:
        rec["hungarian_round"] = None   # measure.round_board needs a solution list (none here)
    rec["hist"] = [dict(h, viol=float(f"{h['viol']:.3e}"), obj=round(h["obj"], 4)) for h in info["hist"]]   # every outer step
    xdir = out_path[:-len(".jsonl")] + "_x"
    os.makedirs(xdir, exist_ok=True)
    np.save(os.path.join(xdir, f"x_start{start}.npy"), x)
    with open(out_path, "a") as f:
        f.write(json.dumps(rec) + "\n")
    print(r, start, rec["status"], rec["viol"]["max"], rec.get("mass_off_solution_placements_per_cell"),
          rec["greedy_round"], rec["seconds"], flush=True)
