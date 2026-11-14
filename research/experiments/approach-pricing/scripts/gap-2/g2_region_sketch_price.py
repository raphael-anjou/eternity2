"""Gap 2, script 3: price of a sound sketch DP for the bottom REGION (not the whole complement) on the real 16x16.
Family priced: track t of the 256 tiles exactly, let the other u = 256 - t repeat (the cheapest sound oblivious sketch found
per state bit in script 1 section 4 and in the toy runs). First-moment model, real census, five clues, pools merged for the
inflation factor (all 256 tiles treated alike; kinds ignored, which favours the sketch).
For a bottom region of r = 256 - d cells with strict levels N_i (i = 1..r) and seam widths F_i:
  inflation I_i(t) = [sum_k C(i,k) t!/(t-k)! u^(i-k)] / [256!/(256-i)!]   (accepted tile sequences per strict sequence)
  states at layer i = min(N_i * I_i, 2^F_i * C(t, k_i)) with k_i the typical number of tracked tiles used
  work X(t) = max over layers of states; the table at the seam holds NB * I_r entries.
Run: python3 -I g2_region_sketch_price.py <official_eternity2.csv>"""
import sys, os, io, contextlib
from math import lgamma, log, log2
HERE = os.path.dirname(os.path.abspath(__file__))
sys.argv = [sys.argv[0], sys.argv[1]]
buf = io.StringIO()
with contextlib.redirect_stdout(buf):
    ns = {"__file__": os.path.join(HERE, "g2_real_tradeoff.py"), "__name__": "g2"}
    exec(open(os.path.join(HERE, "g2_real_tradeoff.py")).read(), ns)
A, Bq, cut, seam_bits, TOT = ns["A"], ns["Bq"], ns["cut"], ns["seam_bits"], ns["TOT"]
L2 = log(2)
def lf(n): return lgamma(n + 1) / L2
def lch(n, k): return lf(n) - lf(k) - lf(n - k) if 0 <= k <= n else float("-inf")
def infl(t, i):
    u = 256 - t; terms = []
    for k in range(0, min(i, t) + 1):
        if u == 0 and k < i: continue
        terms.append((lch(i, k) + lf(t) - lf(t - k) + ((i - k) * log2(u) if i > k else 0.0), k))
    m, km = max(terms)
    return m + log2(sum(2 ** (x - m) for x, _ in terms)) - (lf(256) - lf(256 - i)), km
SB = {j: seam_bits(256 - j) for j in range(1, 256)}
print("model tree 2^%.1f ; two-list join with both lists generated strictly: 2^128.7 at d=137" % TOT)
print("\n   d   r | NB strict | strict widest level | best t | work X(t) | table entries at the seam NB*I | X(t) for t = 0, 32, 64, 128, 192, 224, 240, 256")
for d in (64, 96, 112, 128, 137, 144, 161, 176, 192, 208, 224):
    r = 256 - d; nb = Bq[r - 1]; res = {}
    for t in range(0, 257, 4):
        X = 0.0
        for i in range(1, r + 1):
            I, km = infl(t, i)
            X = max(X, min(Bq[i - 1] + I, SB[i] + lch(t, km)))
        Ir = infl(t, r)[0]
        res[t] = (max(X, nb + Ir), nb + Ir)
    bt = min(res, key=lambda t: res[t][0])
    print("%4d %3d | %6.1f | %6.1f | %3d | %6.1f | %6.1f | %s" % (d, r, nb, max(Bq[:r]), bt, res[bt][0], res[bt][1], " ".join("%.1f" % res[t][0] for t in (0, 32, 64, 128, 192, 224, 240, 256))))
print("\nfull search with a sketch filter at depth d: cost >= max(top tree's widest level up to d, cheapest bottom table)")
best = (1e9, 0, 0)
for d in range(8, 249):
    r = 256 - d; nb = Bq[r - 1]; top = max(A[:d])
    for t in list(range(0, 257, 8)):
        X = 0.0
        for i in range(1, r + 1):
            I, km = infl(t, i)
            X = max(X, min(Bq[i - 1] + I, SB[i] + lch(t, km)))
            if X > best[0]: break
        X = max(X, nb + infl(t, r)[0], top)
        if X < best[0] - 1e-9: best = (X, d, t)
print(" best over d = 8..248 and t = 0, 8, ..., 256: 2^%.1f at d=%d with t=%d tracked tiles (t = 256 is the strict list)" % best)
