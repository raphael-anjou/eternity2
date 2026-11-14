"""Review item 4: the seam width. Three different quantities, all from the real colour counts.
 Fu  = uniform width: log2(17) per interior seam edge, log2(5) per frame seam edge.
 Fr  = collision width of two INDEPENDENT half-edges drawn from the real census: -log2 sum_c (n_c/H)^2 per edge.
 Fc  = width of the seam GIVEN that the two halves use complementary tile sets and every other edge matches:
       -log2 [ P(all edges) / P(all edges but the seam) ], the model's joint probabilities.
 Fm  = the memo's 'implied' width, -log2 [ P(all) / (P(eA) P(eB)) ]: product of marginals, not a conditional probability.
Also: the exact parity identity behind Fc, and its rank on the real tiles."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from r_common import *
ni = sorted(ic.values()); nf = sorted(fc.values())
H = sum(ni); M = sum(nf)
print("interior colours: %d, half-edge counts %s, total %d (edges %d)" % (len(ni), ni, H, H // 2))
print("frame colours: %d, half-edge counts %s, total %d (edges %d)" % (len(nf), nf, M, M // 2))
ci_bits = -log2(sum((n / H) ** 2 for n in ni)); cf_bits = -log2(sum((n / M) ** 2 for n in nf))
ci_wo = -log2(sum(n * (n - 1) for n in ni) / (H * (H - 1)))
print("per interior edge: uniform %.4f bits, real census with replacement %.4f, first edge of the model (without replacement) %.4f" % (log2(17), ci_bits, ci_wo))
print("per frame edge:    uniform %.4f bits, real census %.4f, model first edge %.4f" % (log2(5), cf_bits, -LPF[1]))
def edges(cells):
    cs = set(cells); ei = ef = 0
    for (r, c) in cs:
        for (rr, cc) in ((r + 1, c), (r, c + 1)):
            if (rr, cc) in cs:
                if (r == rr and r in (0, N - 1)) or (c == cc and c in (0, N - 1)): ef += 1
                else: ei += 1
    return ei, ef
print("\n   d | seam edges int, frame | Fu | Fr (real counts, independent halves) | Fc (given complementary sets, rest matched) | Fm (memo) | Fm - Fc = dependence between the two halves' internal matches")
for d in (64, 96, 112, 128, 137, 144, 160, 192):
    ea = edges(ROWS_BU[:d]); eb = edges(ROWS_BU[d:]); si, sf = 420 - ea[0] - eb[0], 60 - ea[1] - eb[1]
    Fu = si * log2(17) + sf * log2(5); Fr = si * ci_bits + sf * cf_bits
    Fc = -(LPI[420] - LPI[420 - si]) - (LPF[60] - LPF[60 - sf])
    Fm = -(LPI[420] - LPI[ea[0]] - LPI[eb[0]]) - (LPF[60] - LPF[ea[1]] - LPF[eb[1]])
    print("%4d | %2d %2d | %5.1f | %5.1f | %5.1f | %5.1f | %5.1f" % (d, si, sf, Fu, Fr, Fc, Fm, Fm - Fc))
print("\nconditional match probability of the k-th last interior edge given all earlier ones matched (model), bits:")
print(" ".join("%d:%.2f" % (k, -(LPI[420 - k + 1] - LPI[420 - k])) for k in (1, 2, 3, 4, 6, 8, 12, 14, 16, 24, 32, 64, 128, 256, 420)))
print("same for frame edges:", " ".join("%d:%.2f" % (k, -(LPF[60 - k + 1] - LPF[60 - k])) for k in (1, 2, 3, 4, 8, 16, 30, 60)))

# Parity identity. In any filling of a region with all internal edges matched, for each colour c:
#   (number of half-edges of colour c on the region's open boundary) = (number on its tiles) mod 2.
# So the seam's colour-parity vector is a GF(2)-linear function of the indicator vector of the used set.
cols = sorted(set(v for p in pieces for v in p if v))
rowsv = []
for p in pieces:
    v = 0
    for x in p:
        if x: v ^= 1 << cols.index(x)
    rowsv.append(v)
def rank(vs):
    vs = list(vs); r = 0
    for bit in range(len(cols)):
        piv = next((i for i in range(r, len(vs)) if vs[i] >> bit & 1), None)
        if piv is None: continue
        vs[r], vs[piv] = vs[piv], vs[r]
        for i in range(len(vs)):
            if i != r and vs[i] >> bit & 1: vs[i] ^= vs[r]
        r += 1
    return r
tot = 0
for v in rowsv: tot ^= v
print("\nparity map tile -> colour-parity vector over GF(2): %d colours, rank %d on the 256 real tiles ; sum over all tiles = %s (every colour count even)" % (len(cols), rank(rowsv), bin(tot)))
from math import comb
def parity_entropy(k, q):
    """entropy (bits) of the parity vector of k uniform draws from q colours"""
    from itertools import product
    # distribution of the number of odd colours: dynamic programme over draws
    dist = {0: 1.0}
    for _ in range(k):
        nd = {}
        for odd, pr in dist.items():
            if odd > 0: nd[odd - 1] = nd.get(odd - 1, 0) + pr * odd / q
            if odd < q: nd[odd + 1] = nd.get(odd + 1, 0) + pr * (q - odd) / q
        dist = nd
    return -sum(pr * log2(pr / comb(q, odd)) for odd, pr in dist.items() if pr > 0)
print("entropy of the seam's parity vector with uniform colours: row seam (14 interior + 2 frame edges) %.1f + %.1f = %.1f bits ; mid-row seam (15 + 2) %.1f bits" % (
    parity_entropy(14, 17), parity_entropy(2, 5), parity_entropy(14, 17) + parity_entropy(2, 5), parity_entropy(15, 17) + parity_entropy(2, 5)))
