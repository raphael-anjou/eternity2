"""Interior colour count c*(N) at which the full first moment of an N x N framed board (5 frame colours, no hints)
equals one, against Owen's interior-only criterion. Same formula as 03_cross_checks.py part (a)."""
import math
lg10f = lambda k: math.lgamma(k + 1) / math.log(10)
def log10_df(k2):
    k = k2 / 2; return (k * math.log(2) + math.lgamma(k + 0.5) - 0.5 * math.log(math.pi)) / math.log(10)
def full(N, c):
    m = (N - 2) ** 2; e = 4 * (N - 2); ring = 4 * (N - 1); J = 2 * (N - 2) * (N - 3) + e
    return math.log10(24) + lg10f(e) + lg10f(m) + m * math.log10(4) + 5 * lg10f(ring / 5) - lg10f(ring) + c * log10_df(2 * J / c) - log10_df(2 * J)
for N in (6, 8, 10, 12, 13, 14, 16):
    lo, hi = 2.0, 40.0
    for _ in range(60):
        mid = (lo + hi) / 2
        if full(N, mid) > 0: lo = mid
        else: hi = mid
    m = (N - 2) ** 2
    owen = math.exp((math.lgamma(m + 1) + m * math.log(4)) / (2 * m))
    print(f"N={N:2d}: c*(N) = {lo:5.2f} interior colours (full first moment = 1);  Owen (m! 4^m)^(1/2m) = {owen:5.2f};  log10 E at c=17: {full(N, 17):6.1f}")
