"""TOY. One board: the SDP lift, its LP shadow, plain AC and probe B's flow, all measured the same way.
Usage: run_board.py n cf ci seed controls.json out.json which[sdp,lp,base] eps time_limit
  which: comma list of the parts to run (each part is written to out.json under its key; an
  existing out.json is updated, so the parts can run as separate processes, one at a time).
"""
import os, sys, json, time, resource
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
from board import Board
from lift import build, solve, assemble_Z
from measure import check_rank_one, round_board, hull_distance, bits_block, kills

n, cf, ci, seed = map(int, sys.argv[1:5])
ctrl_path, out_path, which = sys.argv[5], sys.argv[6], sys.argv[7].split(",")
eps = float(sys.argv[8]); tlim = float(sys.argv[9])
B = Board(n, cf, ci, seed)
res = json.load(open(out_path)) if os.path.exists(out_path) else {}
res.update(board=[n, cf, ci, seed], n_sols=len(B.sols), P=B.P, log2_dfs_nodes=round(float(np.log2(B.dfs_nodes)), 2),
           pin=dict(tile=int(B.pin[0]), rotation=int(B.pin[1]), cell="0 (top-left)", sides_NESW=list(B.pin[2])))
_ctrl = {}


def get_controls():
    """Loaded lazily (the 6x6 controls may still be generating while a solve runs); waits up to 1 h."""
    if not _ctrl:
        for _ in range(720):
            if os.path.exists(ctrl_path):
                break
            time.sleep(5)
        C = json.load(open(ctrl_path))
        assert C["board"] == [n, cf, ci, seed]
        _ctrl.update(annealed=C["annealed"], sampled=C["sampled"],
                     random=[s for s in B.ctrl_idx if (s >= 0).all()])
        res.update(n_annealed=len(C["annealed"]), n_sampled=len(C["sampled"]))
    return _ctrl
t0 = time.time()
alive = B.arc_consistency()
t_ac = time.time() - t0


def rss_gb():
    return round(resource.getrusage(resource.RUSAGE_SELF).ru_maxrss / 2**30, 2)   # bytes on macOS


def point_report(x, Zfun=None):
    r = {}
    r.update(bits_block(B, x, get_controls()))
    r.update(kills(B, x, alive))
    r["hull_distance_Linf"] = round(hull_distance(B, x), 4)
    r["rounding_on_x"] = round_board(B, x)
    return r


def xfile(part):
    return os.path.join(os.path.dirname(os.path.abspath(out_path)), f"x_{part}_{n}_{cf}_{ci}_{seed}.npy")


for part in which:
    if part in ("sdp", "lp"):
        psd = part == "sdp"
        tb = time.time()
        data, cone, lay = build(B, psd=psd)
        tbuild = time.time() - tb
        chk = [check_rank_one(B, data, cone, lay, s) for s in B.sol_idx]
        worst = dict(zero_max_abs=max(c_["zero_max_abs"] for c_ in chk),
                     nonneg_min=min(c_["nonneg_min"] for c_ in chk),
                     exp_max_violation=max(c_["exp_max_violation"] for c_ in chk))
        if psd:
            worst["psd_min_eig"] = min(c_["psd_min_eig"] for c_ in chk)
        sol, secs = solve(data, cone, eps=eps, time_limit=tlim)
        rss_after_main = rss_gb()
        info = sol["info"]
        x = np.clip(sol["x"][lay["ix"]], 0, 1)
        r = dict(free_pairs=int(lay["F"]), rows=int(data["A"].shape[0]), cols=int(data["A"].shape[1]),
                 nnz=int(data["A"].nnz), build_seconds=round(tbuild, 1),
                 all_solutions_rank_one_feasible=worst, status=info["status"], iters=int(info["iter"]),
                 solve_seconds=round(secs, 1), res_pri=info["res_pri"], res_dual=info["res_dual"],
                 gap=info["gap"], pobj=info["pobj"], max_rss_gb_after_main_solve=rss_after_main)
        h = -(x[x > 0] * np.log2(x[x > 0])).sum() / B.N
        r["entropy_bits_per_cell_of_x"] = round(float(h), 3)
        r.update(point_report(x))
        if psd:
            Z = assemble_Z(lay, sol["x"])
            ev = np.linalg.eigvalsh(Z)[::-1]
            r["top_eigs"] = [round(float(e), 4) for e in ev[:8]]
            r["min_eig"] = float(ev[-1])
            r["rank@1e-2"] = int((ev > 1e-2 * ev[0]).sum())
            r["rank@1e-3"] = int((ev > 1e-3 * ev[0]).sum())
            # rank of the solutions' own span (the rank a mixture of solutions would have)
            S = np.zeros((len(B.sol_idx), B.P + 1)); S[:, 0] = 1
            for k, s in enumerate(B.sol_idx):
                S[k, 1 + s] = 1
            r["rank_of_solution_span"] = int(np.linalg.matrix_rank(S))
            # leading-eigenvector rounding
            w, V = np.linalg.eigh(Z)
            v = V[:, -1] * np.sign(V[0, -1] or 1.0)
            r["rounding_on_top_eigvec"] = round_board(B, np.clip(v[1:] / max(v[0], 1e-12), 0, None))
        np.save(xfile(part), x)
        r["max_rss_gb_so_far"] = rss_gb()
        res[part] = r
    elif part in ("sdpcert", "lpcert"):
        # forcing certificates: maximise the sum of a candidate set K of x over the same relaxation
        psd = part == "sdpcert"
        x = np.load(xfile(part[:-4]))
        r = res[part[:-4]]
        sol_used = np.zeros(B.P, bool)
        for s_ in B.sol_idx:
            sol_used[s_] = True
        r["certificates"] = {}
        tc = time.time()
        # leak first (one solve): if no feasible point puts mass off the solution placements, every
        # non-solution placement AC keeps is certified dead. Then the iterative kill certificate, rounds
        # bounded by a wall-clock budget (CERT_BUDGET seconds, default 650) to stay under timeout 900.
        budget = float(os.environ.get("CERT_BUDGET", "650"))
        for cname, K0 in (("leak_off_solutions", alive & ~sol_used), ("kills_beyond_ac", alive & (x < 1e-4))):
            K = K0.copy(); hist = []
            for rnd in range(40):
                if not K.any() or time.time() - tc > budget:
                    break
                dl, cl, ll = build(B, psd=psd, obj="lin", w=K.astype(float))
                sl, sc = solve(dl, cl, eps=eps, time_limit=tlim)
                xl = np.clip(sl["x"][ll["ix"]], 0, 1)
                val = float(xl[K].sum())
                hist.append(dict(size=int(K.sum()), max_sum=round(val, 6), status=sl["info"]["status"],
                                 iters=int(sl["info"]["iter"]), seconds=round(sc, 1)))
                if val <= 1e-3 or cname == "leak_off_solutions":
                    break
                K &= ~(xl > 1e-4)
            r["certificates"][cname] = dict(candidates=int(K0.sum()),
                                            certified_zero=int(K.sum()) if hist and hist[-1]["max_sum"] <= 1e-3 else 0,
                                            finished=bool(hist and (hist[-1]["max_sum"] <= 1e-3 or cname == "leak_off_solutions")),
                                            rounds=hist)
        r["cert_max_rss_gb"] = rss_gb()
        res[part[:-4]] = r
    elif part == "base":
        X_ac = B.sinkhorn(alive.astype(float) + 0.0)
        r = dict(seconds=round(t_ac, 2))
        r.update(point_report(X_ac))
        res["ac"] = r
        tf = time.time()
        pts = B.flow_point(runs=4)
        fl = []
        for z_, X, beta in pts:
            q = bits_block(B, X, get_controls())
            q["beta"] = round(beta, 2)
            fl.append(q)
        keys = [k for k in fl[0] if k.startswith("z_vs") or k.startswith("bits_")]
        res["flow"] = dict(runs=fl, median={k: float(np.median([f[k] for f in fl if f[k] is not None])) for k in keys
                                            if all(f[k] is not None for f in fl)},
                           seconds=round(time.time() - tf, 1))
    json.dump(res, open(out_path, "w"), indent=1, default=lambda o: o.tolist() if hasattr(o, "tolist") else str(o))
    print(part, "done", flush=True)
print(json.dumps({k: res[k] for k in res if k not in ("flow",)}, default=str)[:3000])
