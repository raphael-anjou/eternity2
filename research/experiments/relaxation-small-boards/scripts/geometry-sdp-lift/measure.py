"""TOY. Measures shared by validate.py and run_board.py."""
import numpy as np
from scipy.optimize import linear_sum_assignment, linprog

from board import bits

FLOOR = 1e-5          # floor on x before taking logs (SCS tolerance is 1e-6)
TAUS = (1e-3, 1e-4, 1e-5)


def check_rank_one(B, data, cone, layout, js):
    """Residuals of the rank-one lift of board js in every cone. Returns a dict of worst violations."""
    from lift import point_vector
    v = point_vector(B, layout, js)
    s = data["b"] - data["A"] @ v
    z, l, ep = cone["z"], cone["l"], cone.get("ep", 0)
    out = dict(zero_max_abs=float(np.abs(s[:z]).max()), nonneg_min=float(s[z:z + l].min()))
    ns = cone["s"][0] * (cone["s"][0] + 1) // 2 if "s" in cone else 0
    e = s[z + l + ns:z + l + ns + 3 * ep].reshape(-1, 3)
    # closure of the exp cone {(r,s,u): s e^{r/s} <= u, s > 0} plus {r <= 0, s = 0, u >= 0}
    viol = 0.0
    for r_, s_, u_ in e:
        if s_ > 0:
            viol = max(viol, s_ * np.exp(r_ / s_) - u_)
        else:
            viol = max(viol, r_, -u_, abs(s_))
    out["exp_max_violation"] = float(viol)
    if "s" in cone:
        m = cone["s"][0]
        sv = s[z + l:z + l + ns]
        Z = np.zeros((m, m)); k = 0
        for j in range(m):
            col = sv[k:k + m - j]; k += m - j
            Z[j:, j] = col; Z[j, j:] = col
        off = ~np.eye(m, dtype=bool)
        Z[off] /= np.sqrt(2.0)
        out["psd_min_eig"] = float(np.linalg.eigvalsh(Z).min())
    return out


def round_board(B, X):
    """probe B's rounding: max-weight assignment of tiles to cells on X, best rotation per pair."""
    N, T, n = B.N, B.T, B.n
    W = np.full((N, T), -1e9)
    best_k = {}
    for j in np.argsort(X):
        W[B.cellOf[j], B.tileOf[j]] = X[j]
        best_k[(B.cellOf[j], B.tileOf[j])] = j
    r, c = linear_sum_assignment(-W)
    js = np.full(N, -1)
    for x, t in zip(r, c):
        js[x] = best_k.get((x, t), -1)
    if (js < 0).any():
        return dict(valid=False)
    m = B.mismatches(js.tolist())
    return dict(valid=True, mismatches=int(m), is_solution=bool(m == 0),
                dist_to_nearest_solution=B.dist_to_sols(js))


def hull_distance(B, x):
    """min over convex combinations lam of the solutions of max_p |x_p - (S lam)_p| (LP, HiGHS)."""
    S = np.zeros((B.P, len(B.sol_idx)))
    for k, s in enumerate(B.sol_idx):
        S[s, k] = 1.0
    K = S.shape[1]
    # vars: lam (K), e (1). min e. x - S lam <= e ; S lam - x <= e ; sum lam = 1
    c = np.zeros(K + 1); c[-1] = 1
    A_ub = np.block([[-S, -np.ones((B.P, 1))], [S, -np.ones((B.P, 1))]])
    b_ub = np.concatenate([-x, x])
    A_eq = np.concatenate([np.ones(K), [0]])[None, :]
    r = linprog(c, A_ub=A_ub, b_ub=b_ub, A_eq=A_eq, b_eq=[1.0], bounds=[(0, None)] * (K + 1), method="highs")
    return float(r.x[-1])


def bits_block(B, X, controls):
    """bits on solutions and on each control family, z = (mean_sol - mean_ctrl) / sd_ctrl."""
    out = {}
    bs = bits(X, B.X0, B.sol_idx, FLOOR)
    out["bits_sol_mean"] = round(float(bs.mean()), 2)
    out["bits_sol_min"] = round(float(bs.min()), 2)
    out["bits_sol_max"] = round(float(bs.max()), 2)
    for name, boards in controls.items():
        if len(boards) == 0:
            out[f"bits_{name}_mean"] = None; out[f"z_vs_{name}"] = None
            continue
        bc = bits(X, B.X0, [np.array(b_) for b_ in boards], FLOOR)
        out[f"bits_{name}_mean"] = round(float(bc.mean()), 2)
        out[f"bits_{name}_max"] = round(float(bc.max()), 2)
        out[f"bits_{name}_sd"] = round(float(bc.std()), 2)
        out[f"z_vs_{name}"] = round(float((bs.mean() - bc.mean()) / max(1e-9, bc.std())), 2)
        # how many of each board's cells sit on a placement this point (nearly) kills
        out[f"cells_below_1e-4_{name}"] = round(float(np.mean([(X[np.array(b_)] < 1e-4).sum() for b_ in boards])), 2)
    out["cells_below_1e-4_sol"] = round(float(np.mean([(X[s] < 1e-4).sum() for s in B.sol_idx])), 2)
    return out


def kills(B, x, alive):
    """(c): placements with x below tau that AC keeps, and the reverse."""
    sol_used = np.zeros(B.P, bool)
    for s in B.sol_idx:
        sol_used[s] = True
    out = dict(ac_alive=int(alive.sum()), placements=int(B.P), in_some_solution=int(sol_used.sum()),
               min_x_on_solution_placements=float(x[sol_used].min()),
               mass_off_solution_placements_per_cell=float(x[~sol_used].sum() / B.N))
    for tau in TAUS:
        out[f"killed_ac_keeps@{tau:g}"] = int(((x < tau) & alive).sum())
        out[f"kept_ac_kills@{tau:g}"] = int(((x >= tau) & ~alive).sum())
        out[f"alive@{tau:g}"] = int((x >= tau).sum())
        out[f"killed_but_in_a_solution@{tau:g}"] = int(((x < tau) & sol_used).sum())
    return out
