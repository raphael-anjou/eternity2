"""First-moment model of the real 16x16 tree (uniform colour model with q_in from the real
inner side counts, q_frame = 1/5), five clues pre-placed. Used for RELATIVE prices:
 (a) row scan, calibrated against the measured 10^40.457;
 (b) 2x2 block scan (blocks row-major, cells TL TR BL BR inside a block), all levels and
     block-boundary levels only (the "super-tile table" variant, where intra-block nodes
     become table lookups);
 (c) solutions with a hole: the row-scan tree with a set H of cells never filled
     (leftover tiles unused). Reports E_H (expected random hole-solutions, planted excluded),
     T_H (total nodes), and T_H / (E_H + 1), the nodes per hole-solution found.
Width of a filled set S = P(4,c) P(56,e) P(191,i) 4^i q_in^J_in q_fr^J_fr  (i = free inner).
Run: python3 -I 03_first_moment_orders_holes.py   (stdlib, seconds)"""
import math

N = 16
CLUES = {(2, 2), (2, 13), (13, 2), (13, 13), (8, 7)}
L10 = math.log10
q_in = 0.058901   # from 01_blocks_2x2.out (inner sides); 1/17 = 0.058824
q_fr = 1 / 5


def lff(n, k):  # log10 of falling factorial n!/(n-k)!
    return (math.lgamma(n + 1) - math.lgamma(n - k + 1)) / math.log(10)


def cls(r, c):
    rim_r = r in (0, N - 1); rim_c = c in (0, N - 1)
    return "corner" if rim_r and rim_c else ("edge" if rim_r or rim_c else "inner")


def joint_q(a, b):
    # joint between two rim cells along the ring carries a frame colour
    if cls(*a) != "inner" and cls(*b) != "inner" and (a[0] == b[0] in (0, N - 1) or a[1] == b[1] in (0, N - 1)):
        return q_fr
    return q_in


def profile(order, hole=frozenset()):
    """log10 widths along `order` (list of cells, clue cells excluded), clue cells pre-filled."""
    filled = set(CLUES)
    c = e = i = 0
    lw = 0.0  # log10 of q-product
    out = []
    # joints among clue cells (none adjacent here)
    for cell in order:
        if cell in hole:
            continue
        k = cls(*cell)
        if k == "corner": c += 1
        elif k == "edge": e += 1
        else: i += 1
        r, cc = cell
        for nb in ((r - 1, cc), (r + 1, cc), (r, cc - 1), (r, cc + 1)):
            if nb in filled:
                lw += L10(joint_q(cell, nb))
        filled.add(cell)
        out.append(lff(4, c) + lff(56, e) + lff(191, i) + i * L10(4) + lw)
    return out


def lsum(xs):
    m = max(xs)
    return m + L10(sum(10 ** (x - m) for x in xs))


row = [(r, c) for r in range(N) for c in range(N) if (r, c) not in CLUES]
blk = [(2 * br + dr, 2 * bc + dc) for br in range(8) for bc in range(8) for dr in (0, 1) for dc in (0, 1)]
blk = [x for x in blk if x not in CLUES]

pr = profile(row)
pb = profile(blk)
T_row = lsum([0.0] + pr)
print(f"MODEL row scan, five clues: log10 total {T_row:.3f} (measured 40.457), peak {max(pr):.2f} at free depth {pr.index(max(pr))+1} (measured 39.16 at 158)")
print(f"  model E[solutions] five clues (random instance): log10 {pr[-1]:.3f}  (independently re-checked figure 4.1e-8 = -7.39)")
T_blk = lsum([0.0] + pb)
# block-boundary levels: depths where a whole block is complete
done, ends = set(CLUES), []
for idx, cell in enumerate(blk):
    done.add(cell)
    br, bc = cell[0] // 2, cell[1] // 2
    if all((2 * br + dr, 2 * bc + dc) in done for dr in (0, 1) for dc in (0, 1)):
        ends.append(idx)
T_blk_tab = lsum([0.0] + [pb[j] for j in ends])
print(f"MODEL 2x2 block scan, all levels: log10 total {T_blk:.3f} ({(T_blk-T_row)/L10(2):+.2f} bits vs row scan), peak {max(pb):.2f}")
print(f"MODEL 2x2 block scan, block levels only (table lookups inside a block): log10 {T_blk_tab:.3f} ({(T_blk_tab-T_row)/L10(2):+.2f} bits vs row scan)")
pr_tab = lsum([0.0] + [pr[j] for j in range(len(pr)) if (j + 1) % 4 == 0])
print(f"MODEL row scan, every 4th level only (same lookup trick on 4-cell runs): log10 {pr_tab:.3f} ({(pr_tab-T_row)/L10(2):+.2f} bits)")

print("\nSolutions with a hole (row scan, hole cells never filled)")
print("hole                         k  log10E_H  log10T_H  log10(T_H/(E_H+1))  bits of T_H vs no hole")
rows_out = []


def report(name, H):
    p = profile(row, frozenset(H))
    T = lsum([0.0] + p)
    E = p[-1]
    per = T - L10(10 ** E + 1)
    rows_out.append((name, len(H), E, T, per))
    print(f"{name:28s} {len(H):3d}  {E:8.2f}  {T:8.2f}  {per:10.2f}  {(T-T_row)/L10(2):+8.2f}")


# late hole: the last m scanned cells
for m in (16, 32, 48, 64, 80, 96):
    report(f"last {m} cells", row[-m:])
# compact square inner holes, placed late (bottom-right of the interior) and early (top-left) and at the peak
for s in (1, 2, 3, 4, 5, 6):
    late = [(r, c) for r in range(14 - s + 1, 15) for c in range(14 - s + 1, 15)]
    early = [(r, c) for r in range(1, 1 + s) for c in range(3, 3 + s)]
    mid = [(r, c) for r in range(9, 9 + s) for c in range(5, 5 + s)]
    for nm, H in (("late", late), ("early", early), ("peak rows", mid)):
        if any(x in CLUES for x in H):
            continue
        report(f"square {s}x{s} {nm}", H)

# smallest late hole (last m cells) with nodes per hole-solution <= 10^15
for m in range(1, 200):
    p = profile(row, frozenset(row[-m:]))
    T = lsum([0.0] + p); E = p[-1]
    if T - L10(10 ** E + 1) <= 15:
        print(f"\nsmallest late hole with <= 1e15 nodes per hole-solution: last {m} cells, log10 E_H {E:.2f}; "
              f"chance a found one is the planted board minus the hole: 10^{-L10(10**E+1):.2f}")
        break
