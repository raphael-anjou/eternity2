"""Counting for the degree-2 lift as a function of the side n and the colour counts (stdlib only).
No pins (a pin changes P by under 1 percent at n >= 6). Uniform colours assumed on each edge class.
Prints, per board: placements P, cells N, placements per cell k, the number of structural zeros
(forbidden pairs) split by source, their share of all pairs, the 'noise floor' ratio
sqrt(P (1 - rho) / rho) / N, the Barvinok-Pataki rank for the equality-only (theta) form, and the
memory and flop formulas of section 4.
"""
import math, sys


def board(n, cf, ci):
    E, I = 4 * (n - 2), (n - 2) ** 2
    N = n * n
    P = 16 + E * E + 4 * I * I
    c2 = lambda a: a * (a - 1) // 2
    same_cell = 4 * c2(4) + E * c2(E) + I * c2(4 * I)
    same_tile = 4 * c2(4) + E * c2(E) + I * (c2(4 * I) - I * c2(4))
    adj = (2 * (n - 2) * (n - 3) * (16 * I * I - 16 * I) * (1 - 1 / ci)
           + 4 * (n - 2) * (E * 4 * I) * (1 - 1 / ci)
           + 4 * (n - 3) * (E * E - E) * (1 - 1 / cf)
           + 8 * (4 * E) * (1 - 1 / cf))
    forb = same_cell + same_tile + adj
    pairs = c2(P)
    rho = forb / pairs
    return dict(n=n, cf=cf, ci=ci, N=N, P=P, k=P / N, same_cell=same_cell, same_tile=same_tile, adj=adj,
                forb=forb, pairs=pairs, rho=rho, six_over_N=6 / N,
                noise_ratio=math.sqrt(P * (1 - rho) / rho) / N,
                one_over_T_interior=1 / max(I, 1), deep_cells=max(n - 4, 0) ** 2,
                tile_rule_share_of_zeros=same_tile / forb)


rows = [board(4, 2, 4), board(5, 3, 4), board(6, 4, 5), board(7, 4, 7), board(8, 5, 8), board(10, 5, 10),
        board(12, 5, 12), board(16, 5, 17)]
print("n cf ci |      P    k | zeros(forbidden pairs)  share  6/N  | tile-rule share of zeros | noise ratio | deep cells")
for r in rows:
    print(f"{r['n']:2d} {r['cf']} {r['ci']:2d} | {r['P']:6d} {r['k']:6.1f} | {r['forb']:14.4g} {r['rho']:.4f} {r['six_over_N']:.4f} |"
          f" {r['tile_rule_share_of_zeros']:.3f} | {r['noise_ratio']:.2f} | {r['deep_cells']}")
print()
print("cost table (bytes, flops), equality-only theta form: m = zeros + P + 1")
for r in rows:
    P, forb = r["P"], r["forb"]
    m = forb + P + 1
    rbp = math.ceil((math.sqrt(8 * m + 1) - 1) / 2)
    dense = 8 * (P + 1) ** 2
    eig = 10 * P ** 3
    I = (r["n"] - 2) ** 2
    bag_tile = 4 * (I * I / 2 + I / 2)              # blow-up of the rook's graph K_I x K_I, interior only
    bag_notile = (r["n"] - 1) * 4 * I                # one row of interior cells plus one, no tile rule
    print(f"n={r['n']:2d}: m={m:.3g} r_BP={rbp:6d} | dense Z {dense/2**30:9.3g} GiB, eig 10P^3 = {eig:.2g} flops |"
          f" factor V at r_BP {8*P*rbp/2**30:8.3g} GiB, at r=64 {8*P*64/2**30:7.3g} GiB |"
          f" gradient 2 m r: r_BP {2*m*rbp:.2g}, r=64 {2*m*64:.2g} | nonneg pairs {r['pairs']-forb:.3g} |"
          f" chordal bag with tile rule >= {bag_tile:.0f} of {P}, without {bag_notile}")
