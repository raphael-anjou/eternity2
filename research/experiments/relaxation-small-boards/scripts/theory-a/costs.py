"""Derived cost table (no board is built for 6x6 and above; counts from the theory memo's formulas,
uniform colours, no pin). Models, per evaluation of value and gradient, rank r:
  today      4 P^2 r + 3 P^2 + 4 K r               (bm.py's own formula)
  equal      35 P r                                (L V, L^T(.), sphere map; nnz(L) = 6 P)
  S1 exact   3 P^2 r + 3 P^2 + equal               (all-pairs nonnegativity, no pair list)
  S2 blocks  6 Kd r + equal                        (nonnegativity on cell, tile and adjacent blocks only)
  S3 moments 20 P r^2 + equal                      (sum of squares of the zeros through r x r moments)
  S4 factor  equal                                 (nonnegative factor: study B's object)
Memory: L-BFGS state 8 (2 m + 4) P r with m = 10, plus the multipliers of each form.
Usage: costs.py"""
import json


def counts(n, cf, ci):
    E, I = 4 * (n - 2), (n - 2) ** 2
    c2 = lambda a: a * (a - 1) // 2
    P = 16 + E * E + 4 * I * I
    same_cell = 4 * c2(4) + E * c2(E) + I * c2(4 * I)
    same_tile = 4 * c2(4) + E * c2(E) + I * (c2(4 * I) - 6 * I)
    adj_ii = 2 * (n - 2) * (n - 3) * (16 * I * I - 16 * I)
    adj_ei = 4 * (n - 2) * (4 * E * I)
    adj_ee = 4 * (n - 3) * (E * E - E)
    adj_ce = 32 * E
    mis = adj_ii * (1 - 1 / ci) + adj_ei * (1 - 1 / ci) + adj_ee * (1 - 1 / cf) + adj_ce * (1 - 1 / cf)
    K = same_cell + same_tile + mis
    Kd = same_cell + same_tile + adj_ii + adj_ei + adj_ee + adj_ce
    edges_rim = 4 * (n - 1); edges_in = 2 * n * (n - 1) - edges_rim
    G = edges_rim * cf + edges_in * ci
    return dict(P=P, K=K, Kd=Kd, F=P * (P - 1) / 2 - K, G=G, m=2 * n * n + G)


out = []
for (n, cf, ci) in [(5, 3, 4), (6, 4, 5), (8, 4, 9), (16, 5, 17)]:
    c = counts(n, cf, ci)
    P, K, Kd, F, G, m = c["P"], c["K"], c["Kd"], c["F"], c["G"], c["m"]
    for r in (16, 64):
        eq = 35 * P * r
        fl = dict(today=4 * P * P * r + 3 * P * P + 4 * K * r, S1=3 * P * P * r + 3 * P * P + eq,
                  S2=6 * Kd * r + eq, S3=20 * P * r * r + eq, S4=eq)
        lb = 8 * 24 * P * r
        mem = dict(today=lb + 32 * K + 16 * 0.77 * F, S1=lb + 8 * P * P / 2 + 8 * m * r,
                   S1_no_multipliers=lb + 8 * m * r, S2=lb + 8 * Kd + 8 * m * r,
                   S3=lb + 16 * G * r * r + 8 * m * r, S4=lb + 8 * m * r)
        out.append(dict(board=[n, cf, ci], r=r, **{k: (round(v) if isinstance(v, float) else v) for k, v in c.items()},
                        flops={k: f"{v:.2e}" for k, v in fl.items()},
                        factor_vs_today={k: round(fl["today"] / v, 1) for k, v in fl.items()},
                        mem_GB={k: round(v / 1e9, 3) for k, v in mem.items()}))
print(json.dumps(out, indent=1))
