#!/usr/bin/env python3
"""FROSTLINE reproduction: Bethe free energy of the last-row residual factor
graph as a finishability discriminator (vol-233 R2, topic
tail-finishability-frostline).

Pipeline (all on the official 16x16 piece set bundled with the starter kit):
  1. Produce full boards with a seeded randomized greedy row-major filler.
  2. For each board: top = rows 0..14 frozen; self-pool = the 16 pieces the
     board placed in row 15. Dedupe tops by tail piece-multiset.
  3. Label: exact tail optimum (max matched edges in row 15: 15 horizontal +
     16 seam edges) by bitmask DP over (used-piece mask, left colour). Exact.
  4. Signal: sum-product BP on the residual chain with hard border-legality
     domains and a SOFT seam factor exp(beta * [seam colours agree]); read
     Bethe free energy, Bethe entropy, mean_maxprob, frozen_frac, mean_domain,
     and a min-sum frozen fraction (null control).
  5. Stats: Spearman rho, tercile AUC, bootstrap SD, partial correlation vs
     the board's own raw score, disjoint seed-half cross-validation.

Conventions: edges are URDL quads, colour 0 = grey border. Scoring counts
interior matched edges only (rim-excluding), identical to the kit's
score_cells. At R=15 the residual graph is a chain, so BP is exact and the
Bethe free energy is the true -log Z of the residual model.
"""

from __future__ import annotations

import argparse
import json
import math
import random
from pathlib import Path

import numpy as np

KIT_DATA = Path(__file__).resolve().parents[3] / "starter-kit" / "data" / "official.json"
N = 16
CELLS = N * N


def rot(e: tuple[int, int, int, int], r: int) -> tuple[int, int, int, int]:
    """Rotate a URDL edge quad clockwise r quarter-turns."""
    return tuple(e[(i - r) % 4] for i in range(4))  # type: ignore[return-value]


def load_pieces() -> list[tuple[int, int, int, int]]:
    doc = json.loads(KIT_DATA.read_text())
    assert doc["width"] == N and doc["height"] == N
    return [tuple(p) for p in doc["pieces"]]


# ---------------------------------------------------------------- producer --


def border_legal(e: tuple[int, int, int, int], row: int, col: int) -> bool:
    """Hard border legality: grey (0) exactly on rim-facing edges."""
    up, right, down, left = e
    if (up == 0) != (row == 0):
        return False
    if (down == 0) != (row == N - 1):
        return False
    if (left == 0) != (col == 0):
        return False
    if (right == 0) != (col == N - 1):
        return False
    return True


def greedy_board(pieces, seed: int) -> list[tuple[int, int]] | None:
    """Seeded randomized greedy row-major fill. Returns [(piece_id, rot)] per
    cell or None on deadlock (rare; the caller skips that seed)."""
    rng = random.Random(seed)
    used = [False] * len(pieces)
    board: list[tuple[int, int]] = []
    down_color = [0] * N  # colour facing down from the row above, per column
    for cell in range(CELLS):
        row, col = divmod(cell, N)
        left_color = board[-1][2] if col > 0 else None  # right colour of left nb
        best: tuple[int, float, int, int, tuple] | None = None
        for pid, base in enumerate(pieces):
            if used[pid]:
                continue
            for r in range(4):
                e = rot(base, r)
                if not border_legal(e, row, col):
                    continue
                m = 0
                if row > 0 and e[0] == down_color[col]:
                    m += 1
                if col > 0 and e[3] == left_color:
                    m += 1
                key = (m, rng.random(), pid, r, e)
                if best is None or key > best:
                    best = key
        if best is None:
            return None
        _, _, pid, r, e = best
        used[pid] = True
        # store (pid, rot) but keep the placed quad handy via parallel updates
        board.append((pid, r, e[1], e[2]))  # pid, rot, right colour, down colour
        down_color[col] = e[2]
    # normalize: keep (pid, rot) only; quads are re-derived when needed
    return [(pid, r) for pid, r, _rc, _dc in board]


# ------------------------------------------------------------ exact label --


def tail_label(pieces, board, betas_unused=None) -> tuple[int, list, list]:
    """Exact tail optimum for row 15 with the board's own row-15 pieces.

    Returns (tail_opt, seam_colors, pool) where seam_colors[c] is the colour
    facing down from row 14 at column c and pool is the 16 (pid, base_quad).
    DP over (columns left to right, used-piece bitmask, previous right colour).
    """
    seam_colors = []
    for col in range(N):
        pid, r = board[(N - 2) * N + col]
        seam_colors.append(rot(pieces[pid], r)[2])
    pool = [board[(N - 1) * N + col][0] for col in range(N)]

    # candidates[col] = list of (pool_index, left_colour, right_colour, seam_match_bonus_base_up)
    cands: list[list[tuple[int, int, int, int]]] = []
    for col in range(N):
        opts = []
        for k, pid in enumerate(pool):
            for r in range(4):
                e = rot(pieces[pid], r)
                if not border_legal(e, N - 1, col):
                    continue
                opts.append((k, e[3], e[1], e[0]))
        cands.append(opts)

    from functools import lru_cache

    @lru_cache(maxsize=None)
    def dp(col: int, mask: int, prev_right: int) -> int:
        if col == N:
            return 0
        best = -1
        for k, lc, rc, up in cands[col]:
            if mask & (1 << k):
                continue
            gain = int(up == seam_colors[col])
            if col > 0:
                gain += int(lc == prev_right)
            sub = dp(col + 1, mask | (1 << k), rc)
            if sub >= 0 and gain + sub > best:
                best = gain + sub
        return best

    opt = dp(0, 0, 0)
    dp.cache_clear()
    return opt, seam_colors, pool


# -------------------------------------------------------------------- BP  --


def frostline_r15(pieces, seam_colors, pool, beta: float) -> dict:
    """Sum-product BP on the R=15 residual chain, soft seam, hard border.

    Domains: pool (piece, rotation) labels border-legal for each column; the
    one-piece-once matching constraint is NOT imposed (as in the source). At
    R=15 the graph is a chain so BP is exact: bethe_F == -log Z exactly.
    """
    doms = []  # per column: (n_states, up[], left[], right[])
    for col in range(N):
        ups, lefts, rights = [], [], []
        for pid in pool:
            base = pieces[pid]
            for r in range(4):
                e = rot(base, r)
                if border_legal(e, N - 1, col):
                    ups.append(e[0])
                    lefts.append(e[3])
                    rights.append(e[1])
        doms.append((np.array(ups), np.array(lefts), np.array(rights)))

    # log unary potentials (soft seam)
    log_phi = [beta * (doms[c][0] == seam_colors[c]).astype(float) for c in range(N)]

    def log_psi(c: int) -> np.ndarray:
        """log pairwise potential between columns c and c+1, shape (nc, nc1)."""
        return beta * (doms[c][2][:, None] == doms[c + 1][1][None, :]).astype(float)

    psis = [log_psi(c) for c in range(N - 1)]

    def norm(v: np.ndarray) -> tuple[np.ndarray, float]:
        m = v.max()
        w = np.exp(v - m)
        s = w.sum()
        return np.log(w / s + 1e-300), m + math.log(s)

    # forward/backward messages in log space (chain => one pass each, exact)
    fwd = [np.zeros(0)] * N  # fwd[c] = message from c-1 into c
    bwd = [np.zeros(0)] * N  # bwd[c] = message from c+1 into c
    fwd[0] = np.zeros(len(log_phi[0]))
    for c in range(1, N):
        pre = log_phi[c - 1] + fwd[c - 1]
        msg = np.logaddexp.reduce(pre[:, None] + psis[c - 1], axis=0)
        fwd[c], _ = norm(msg)
    bwd[N - 1] = np.zeros(len(log_phi[N - 1]))
    for c in range(N - 2, -1, -1):
        pre = log_phi[c + 1] + bwd[c + 1]
        msg = np.logaddexp.reduce(psis[c] + pre[None, :], axis=1)
        bwd[c], _ = norm(msg)

    beliefs = []
    for c in range(N):
        b = log_phi[c] + fwd[c] + bwd[c]
        b -= np.logaddexp.reduce(b)
        beliefs.append(np.exp(b))

    # Bethe free energy via the standard decomposition (exact on the chain).
    log_z = 0.0
    exp_matches = 0.0
    edge_entropy = 0.0
    node_entropy = 0.0
    energy_term = 0.0
    for c in range(N - 1):
        lb = (log_phi[c] + fwd[c])[:, None] + psis[c] + (log_phi[c + 1] + bwd[c + 1])[None, :]
        lb -= np.logaddexp.reduce(lb, axis=None)
        pb = np.exp(lb)
        edge_entropy -= float((pb * lb).sum())
        energy_term += float((pb * psis[c]).sum())
        exp_matches += float((pb * (psis[c] > 0)).sum())
    for c in range(N):
        b = beliefs[c]
        h = -float((b * np.log(b + 1e-300)).sum())
        node_entropy += h
        energy_term += float((b * log_phi[c]).sum())
        exp_matches += float((b * (log_phi[c] > 0)).sum())
    degree = [1 if c in (0, N - 1) else 2 for c in range(N)]
    bethe_entropy = edge_entropy - sum(
        (degree[c] - 1) * -float((beliefs[c] * np.log(beliefs[c] + 1e-300)).sum())
        for c in range(N)
    )
    log_z = energy_term + bethe_entropy

    maxp = np.array([b.max() for b in beliefs])

    # min-sum (zero temperature) frozen fraction: null control
    GAP = 1e-9
    f_ms = [np.zeros(len(log_phi[0]))]
    for c in range(1, N):
        pre = log_phi[c - 1] + f_ms[c - 1]
        f_ms.append((pre[:, None] + psis[c - 1]).max(axis=0))
    b_ms = [np.zeros(len(log_phi[N - 1]))]
    for c in range(N - 2, -1, -1):
        pre = log_phi[c + 1] + b_ms[0]
        b_ms.insert(0, (psis[c] + pre[None, :]).max(axis=1))
    ms_frozen = 0
    for c in range(N):
        mm = log_phi[c] + f_ms[c] + b_ms[c]
        top2 = np.sort(mm)[-2:] if len(mm) > 1 else np.array([mm[0] - 1, mm[0]])
        ms_frozen += int(top2[1] - top2[0] > 0.5)

    return {
        "bethe_F": -log_z,
        "bethe_S": bethe_entropy,
        "exp_matches": exp_matches,
        "mean_maxprob": float(maxp.mean()),
        "frozen_frac": float((maxp > 0.99).mean()),
        "mean_domain": float(np.mean([len(log_phi[c]) for c in range(N)])),
        "minsum_frozen_frac": ms_frozen / N,
    }


# ------------------------------------------------------------------ stats --


def ranks(x: np.ndarray) -> np.ndarray:
    order = np.argsort(x, kind="stable")
    r = np.empty(len(x))
    r[order] = np.arange(len(x), dtype=float)
    # average ties
    for v in np.unique(x):
        m = x == v
        r[m] = r[m].mean()
    return r


def spearman(x, y) -> float:
    x, y = np.asarray(x, float), np.asarray(y, float)
    if np.ptp(x) == 0 or np.ptp(y) == 0:
        return float("nan")
    rx, ry = ranks(x), ranks(y)
    return float(np.corrcoef(rx, ry)[0, 1])


def partial_spearman(x, y, z) -> float:
    rxy, rxz, ryz = spearman(x, y), spearman(x, z), spearman(y, z)
    den = math.sqrt((1 - rxz**2) * (1 - ryz**2))
    return (rxy - rxz * ryz) / den if den > 0 else float("nan")


def tercile_auc(signal, label) -> float:
    """AUC of `signal` separating top-tercile from bottom-tercile `label`."""
    lab = np.asarray(label, float)
    sig = np.asarray(signal, float)
    lo, hi = np.quantile(lab, [1 / 3, 2 / 3])
    top, bot = sig[lab >= hi], sig[lab <= lo]
    if len(top) == 0 or len(bot) == 0:
        return float("nan")
    wins = sum((t > b) + 0.5 * (t == b) for t in top for b in bot)
    return float(wins / (len(top) * len(bot)))


def bootstrap_sd(x, y, n_boot, seed=0) -> float:
    rng = np.random.default_rng(seed)
    n = len(x)
    vals = []
    for _ in range(n_boot):
        idx = rng.integers(0, n, n)
        vals.append(spearman(np.asarray(x)[idx], np.asarray(y)[idx]))
    return float(np.nanstd(vals))


# ------------------------------------------------------------------- main --


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--seeds", type=int, default=24, help="producer seeds to try")
    ap.add_argument("--betas", type=str, default="1,2,3,4")
    ap.add_argument("--bootstrap", type=int, default=0, help="bootstrap resamples for rho SD")
    ap.add_argument("--out", type=str, default=None, help="write results JSON here")
    args = ap.parse_args()
    betas = [float(b) for b in args.betas.split(",")]

    pieces = load_pieces()
    tops = []  # (seed, board, raw_score, tail_opt)
    seen_pools: set[tuple] = set()
    for seed in range(args.seeds):
        board = greedy_board(pieces, seed)
        if board is None:
            continue
        # raw full-board score (rim-excluding matched edges), same convention
        # as the kit's score_cells
        quads = [rot(pieces[pid], r) for pid, r in board]
        raw = 0
        for cell in range(CELLS):
            row, col = divmod(cell, N)
            if col + 1 < N and quads[cell][1] == quads[cell + 1][3]:
                raw += 1
            if row + 1 < N and quads[cell][2] == quads[cell + N][0]:
                raw += 1
        pool_key = tuple(sorted(board[(N - 1) * N + c][0] for c in range(N)))
        if pool_key in seen_pools:
            continue
        seen_pools.add(pool_key)
        opt, seam, pool = tail_label(pieces, board)
        tops.append({"seed": seed, "raw": raw, "tail_opt": opt,
                     "seam": seam, "pool": pool})
    if len(tops) < 8:
        raise SystemExit(f"only {len(tops)} distinct tops; increase --seeds")

    print(f"n = {len(tops)} distinct tops | raw score min/med/max = "
          f"{min(t['raw'] for t in tops)}/"
          f"{int(np.median([t['raw'] for t in tops]))}/"
          f"{max(t['raw'] for t in tops)} | tail_opt min/med/max = "
          f"{min(t['tail_opt'] for t in tops)}/"
          f"{int(np.median([t['tail_opt'] for t in tops]))}/"
          f"{max(t['tail_opt'] for t in tops)}")

    label = [t["tail_opt"] for t in tops]
    raw = [t["raw"] for t in tops]
    results = {"n": len(tops), "raw_summary": [min(raw), float(np.median(raw)), max(raw)],
               "label_summary": [min(label), float(np.median(label)), max(label)],
               "baseline_rho_raw_vs_tail": spearman(raw, label),
               "betas": {}}
    print(f"baseline rho(raw_score, tail_opt) = {results['baseline_rho_raw_vs_tail']:+.3f}")
    hdr = f"{'beta':>5} {'param':<18} {'rho':>7} {'AUC':>6}"
    if args.bootstrap:
        hdr += f" {'bootSD':>7}"
    print(hdr)
    for beta in betas:
        obs = [frostline_r15(pieces, t["seam"], t["pool"], beta) for t in tops]
        row = {}
        for name in ["bethe_F", "bethe_S", "mean_maxprob", "frozen_frac",
                     "exp_matches", "mean_domain", "minsum_frozen_frac"]:
            sig = [o[name] for o in obs]
            rho = spearman(sig, label)
            auc = tercile_auc(sig, label)
            entry = {"rho": rho, "auc": auc}
            if args.bootstrap:
                entry["boot_sd"] = bootstrap_sd(sig, label, args.bootstrap)
            row[name] = entry
            line = f"{beta:>5.1f} {name:<18} {rho:>7.3f} {auc:>6.3f}"
            if args.bootstrap:
                line += f" {entry['boot_sd']:>7.3f}"
            print(line)
        fe = [o["bethe_F"] for o in obs]
        row["partial_rho_FE_given_raw"] = partial_spearman(fe, label, raw)
        row["partial_rho_raw_given_FE"] = partial_spearman(raw, label, fe)
        print(f"      partial rho(FE, tail_opt | raw) = "
              f"{row['partial_rho_FE_given_raw']:+.3f} | "
              f"partial rho(raw, tail_opt | FE) = "
              f"{row['partial_rho_raw_given_FE']:+.3f}")
        # disjoint seed-half cross-validation
        ev = [i for i, t in enumerate(tops) if t["seed"] % 2 == 0]
        od = [i for i, t in enumerate(tops) if t["seed"] % 2 == 1]
        row["cv_even_odd"] = [
            spearman([fe[i] for i in ev], [label[i] for i in ev]) if len(ev) > 4 else None,
            spearman([fe[i] for i in od], [label[i] for i in od]) if len(od) > 4 else None,
        ]
        print(f"      seed-half CV rho(FE): even {row['cv_even_odd'][0]} / odd {row['cv_even_odd'][1]}")
        results["betas"][str(beta)] = row

    if args.out:
        Path(args.out).write_text(json.dumps(results, indent=2))
        print(f"wrote {args.out}")


if __name__ == "__main__":
    main()
