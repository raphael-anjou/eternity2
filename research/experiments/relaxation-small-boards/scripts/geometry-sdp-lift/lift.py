"""TOY. The semidefinite lift (and its LP shadow) of one generated board, written straight in SCS's
conic form (no mox86ing layer, so the 6x6 matrix fits in memory).

Variables: x[p] for each placement p of board.Board (rim-consistent, pin applied), y[p,q] for each
FREE pair p < q, and an entropy epigraph t[p]. A pair is free when p and q are in different
cells, use different tiles, and, if their cells are adjacent, their facing sides match. Every other
off-diagonal entry of Y is the constant 0 (the gangster and mismatch zeros are not variables).

  Z = [[1, x^T], [x, Y]],  Y[p,p] = x[p],  Z PSD                        (dropped for the LP)
  sum_{p in cell c} x[p] = 1,  sum_{p of tile t} x[p] = 1
  sum_{q in cell c} Y[p,q] = x[p]  for every p and every cell c != cell(p)    (row sums)
  sum_{q of tile t} Y[p,q] = x[p]  for every p and every tile t != tile(p)
  x >= 0, y >= 0 entrywise
  objective (obj=):
    "l2"      minimise ||x||^2 (the point used for every reading);
    "lin"     maximise w.x for a 0/1 weight w (the forcing certificates of measure (c));
    "entropy" maximise sum_p entr(x[p]) with exp cones (t, x, 1) (tried first, see below).

Why ||x||^2: it is the same fixed function of x on every board, symmetric under any relabelling of
cells, tiles and rotations, and it knows nothing of the solutions or of colours (matching enters
only through the constraints). It is strictly convex in x, so the reported x is unique. It pulls
mass toward spreading over every placement the constraints allow, like the entropy, but it does
not guarantee x[p] > 0 for every placement some feasible point uses; so measure (c) never reads a
zero off this point alone, it confirms it with a "lin" solve (maximise the sum of the candidate
zeros: an optimum of 0 means every feasible point has them all at 0).
Why not entropy: tried first. On the 4x4 check SCS never met its dual tolerance in 200,000
iterations (the optimum sits where most x are 0 and the entropy's slope is infinite, so the dual
is unbounded), while the l2 solve converged in about 150 iterations to the same x.
Minimising the trace would not do: trace(Y) = sum x = number of cells on the whole feasible set.
"""
import time

import numpy as np
import scipy.sparse as sp


def free_pairs(B, alive=None):
    P = B.P
    c, t = B.cellOf, B.tileOf
    same = (c[:, None] == c[None, :]) | (t[:, None] == t[None, :])
    ok = ~same
    # adjacency mismatch
    for d in range(4):
        nbc = B.nb[c, d]                                  # neighbour cell of p in direction d
        adj = (nbc[:, None] == c[None, :])                # q sits in that neighbour cell
        mis = B.cols[:, d][:, None] != B.cols[:, (d + 2) % 4][None, :]
        ok &= ~(adj & mis)
    if alive is not None:
        ok &= alive[:, None] & alive[None, :]
    iu = np.triu_indices(P, 1)
    keep = ok[iu]
    return iu[0][keep], iu[1][keep]


def build(B, psd=True, alive=None, obj="l2", w=None):
    """Return (data, cone, layout). Variables: [x (P), y (F)] plus t (P) for obj="entropy"."""
    P, N, T = B.P, B.N, B.T
    fp, fq = free_pairs(B, alive)
    F = len(fp)
    ix = np.arange(P); iy = P + np.arange(F); it = P + F + np.arange(P)
    ent = obj == "entropy"
    nvar = P + F + (P if ent else 0)
    rows, cols, vals, b = [], [], [], []
    r = 0

    def add(rr, cc, vv):
        rows.append(np.asarray(rr)); cols.append(np.asarray(cc)); vals.append(np.asarray(vv, float))

    # ---- zero cone -------------------------------------------------------------------------
    # cell sums, tile sums
    add(r + B.cellOf, ix, np.ones(P)); b.append(np.ones(N)); r += N
    add(r + B.tileOf, ix, np.ones(P)); b.append(np.ones(T)); r += T
    # dead placements (only when an AC mask is given): x = 0
    if alive is not None:
        dead = np.where(~alive)[0]
        add(r + np.arange(len(dead)), dead, np.ones(len(dead))); b.append(np.zeros(len(dead))); r += len(dead)
    # row sums over cells: row id (p, c) for c != cell(p)
    #   + y[p,q] for q in c ; - x[p]
    pc_id = np.full((P, N), -1)
    mask = np.ones((P, N), bool); mask[np.arange(P), B.cellOf] = False
    pc_id[mask] = r + np.arange(mask.sum()); r += mask.sum()
    add(pc_id[mask], np.repeat(ix, N - 1), -np.ones(mask.sum()))
    add(pc_id[fp, B.cellOf[fq]], iy, np.ones(F))
    add(pc_id[fq, B.cellOf[fp]], iy, np.ones(F))
    pt_id = np.full((P, T), -1)
    mask = np.ones((P, T), bool); mask[np.arange(P), B.tileOf] = False
    pt_id[mask] = r + np.arange(mask.sum()); r += mask.sum()
    add(pt_id[mask], np.repeat(ix, T - 1), -np.ones(mask.sum()))
    add(pt_id[fp, B.tileOf[fq]], iy, np.ones(F))
    add(pt_id[fq, B.tileOf[fp]], iy, np.ones(F))
    nz = r
    b.append(np.zeros(nz - sum(len(v) for v in b)))
    # ---- nonneg cone: x >= 0, y >= 0 ------------------------------------------------------
    add(r + np.arange(P + F), np.arange(P + F), -np.ones(P + F)); b.append(np.zeros(P + F)); r += P + F
    nl = P + F
    # ---- PSD cone on Z, svec lower triangle column-major, off-diagonals scaled by sqrt 2 ----
    s_dims = []
    if psd:
        m = P + 1
        s2 = np.sqrt(2.0)

        def pos(i, j):            # i >= j
            return j * m - j * (j - 1) // 2 + (i - j)
        bs = np.zeros(m * (m + 1) // 2)
        bs[pos(0, 0)] = 1.0
        add(r + pos(ix + 1, 0), ix, -s2 * np.ones(P))          # Z[i,0] = x
        add(r + pos(ix + 1, ix + 1), ix, -np.ones(P))          # Z[i,i] = x
        add(r + pos(fq + 1, fp + 1), iy, -s2 * np.ones(F))     # Z[q+1,p+1] = y
        b.append(bs); r += len(bs)
        s_dims = [m]
    # SCS cone order is zero, nonneg, PSD, exp: the exp block goes last
    # ---- exp cones (t_p, x_p, 1) ----------------------------------------------------------
    if ent:
        base = r + 3 * np.arange(P)
        add(base, it, -np.ones(P)); add(base + 1, ix, -np.ones(P))
        be = np.zeros(3 * P); be[2::3] = 1.0; b.append(be); r += 3 * P
    A = sp.csc_matrix((np.concatenate(vals), (np.concatenate(rows), np.concatenate(cols))), shape=(r, nvar))
    bvec = np.concatenate(b)
    assert len(bvec) == r
    c = np.zeros(nvar)
    data = dict(A=A, b=bvec, c=c)
    if ent:
        c[it] = -1.0
    elif obj == "l2":
        data["P"] = sp.csc_matrix((np.ones(P), (ix, ix)), shape=(nvar, nvar))
    elif obj == "lin":
        c[ix] = -np.asarray(w, float)
    cone = dict(z=int(nz), l=int(nl))
    if ent:
        cone["ep"] = int(P)
    if psd:
        cone["s"] = s_dims
    layout = dict(ent=ent, P=P, F=F, fp=fp, fq=fq, ix=ix, iy=iy, it=it)
    return data, cone, layout


def assemble_Z(layout, v):
    P = layout["P"]
    Z = np.zeros((P + 1, P + 1))
    Z[0, 0] = 1.0
    x = v[layout["ix"]]
    Z[0, 1:] = x; Z[1:, 0] = x
    Z[np.arange(1, P + 1), np.arange(1, P + 1)] = x
    y = v[layout["iy"]]
    Z[layout["fp"] + 1, layout["fq"] + 1] = y
    Z[layout["fq"] + 1, layout["fp"] + 1] = y
    return Z


def point_vector(B, layout, js):
    """The rank-one lift of a board js (placement index per cell) as a variable vector."""
    P = layout["P"]
    x = np.zeros(P); x[js] = 1.0
    y = x[layout["fp"]] * x[layout["fq"]]
    t = np.zeros(P)                       # entr(0) = entr(1) = 0
    return np.concatenate([x, y, t]) if layout["ent"] else np.concatenate([x, y])


def solve(data, cone, eps=1e-6, max_iters=200000, time_limit=600, verbose=False, linear_solver="accelerate"):
    """SCS 3.3. Apple Accelerate's sparse LDL is the linear-system backend: on 5x5 seed 1 it ran
    2.5 times faster per iteration than the default QDLDL (solver_settings_test.py)."""
    import os
    import scs
    verbose = verbose or os.environ.get("SCS_VERBOSE") == "1"
    if verbose and os.environ.get("SCS_LINEBUF") == "1":
        # line-buffer the C library's stdout so SCS's iteration log reaches the log file as it runs
        import ctypes
        libc = ctypes.CDLL(None)
        stdout_p = ctypes.c_void_p.in_dll(libc, "stdout")
        libc.setvbuf(stdout_p, None, 1, 0)          # _IOLBF = 1 on glibc
    linear_solver = os.environ.get("SCS_LINEAR_SOLVER", linear_solver)   # "qdldl" on Linux (x86)
    t0 = time.time()
    solver = scs.SCS(data, cone, eps_abs=eps, eps_rel=eps, max_iters=max_iters,
                     time_limit_secs=time_limit, verbose=verbose, linear_solver=linear_solver)
    sol = solver.solve()
    return sol, time.time() - t0
