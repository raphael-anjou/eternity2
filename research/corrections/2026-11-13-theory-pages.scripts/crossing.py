"""First-moment solution counts of planted N x N boards, 17 interior + 5 frame colours.

Stdlib only:  python3 -I crossing.py

Three columns per N:
  page    the formula formerly on how-hard-is-this-instance: m! 4^m / 17^j,
          m = (N-2)^2 interior pieces, j = 2(N-2)(N-3) interior-interior joints
          (no seam joints, no frame).
  naive   every joint an independent coin: 4! e! m! 4^m (1/5)^ring (1/17)^(j+e),
          e = 4(N-2) edge pieces (= seam joints), ring = 4(N-1) frame joints.
  census  the census-exact ensemble used for the canonical 14,702 on the
          solution-counting page: uniform pairing of half-edges with fixed
          colour counts, as balanced as the totals allow (even interior
          counts; frame colours split evenly between the two directions).
  census5 the same with five interior cells pinned (piece and rotation known).
Check line: the census formula on the official counts reproduces 1.153e7
(no clue) and 14,702 (centre clue).
"""
import math

L10 = math.log(10)


def lf(k):
    return math.lgamma(k + 1) / L10


def ldf(k):
    """log10 of k!! for odd k >= -1"""
    if k <= 0:
        return 0.0
    # (2r-1)!! = (2r)! / (2^r r!)
    r = (k + 1) // 2
    return lf(2 * r) - r * math.log10(2) - lf(r)


def census(N, ring_counts=None, interior_counts=None, pinned=0):
    m = (N - 2) ** 2
    e = 4 * (N - 2)
    ring = 4 * (N - 1)
    J = 2 * (N - 2) * (N - 1)  # interior joints, seams included
    if ring_counts is None:
        q, r = divmod(ring, 5)
        ring_counts = [q + 1] * r + [q] * (5 - r)
    if interior_counts is None:
        pairs, r = divmod(J, 17)  # n_c = 2 * pairs_c
        interior_counts = [2 * (pairs + 1)] * r + [2 * pairs] * (17 - r)
    assert sum(ring_counts) == ring and sum(interior_counts) == 2 * J
    arr = lf(4) + lf(e) + (lf(m) - lf(m - pinned)) * 0 + lf(m - pinned) + (m - pinned) * math.log10(4)
    ringf = sum(lf(k) for k in ring_counts) - lf(ring)
    intf = sum(ldf(n - 1) for n in interior_counts) - ldf(2 * J - 1)
    return arr + ringf + intf


print("check, official counts (12 per frame colour each way; interior 48 x5, 50 x12):")
off_int = [48] * 5 + [50] * 12
print("  no clue   10^%.3f = %.4g" % (census(16, [12] * 5, off_int), 10 ** census(16, [12] * 5, off_int)))
print("  starter   10^%.3f = %.1f" % (census(16, [12] * 5, off_int, 1), 10 ** census(16, [12] * 5, off_int, 1)))
print("  five clues 10^%.3f" % census(16, [12] * 5, off_int, 5))
print()
print(" N |   page |  naive | census | census, five clues   (log10 expected solutions)")
for N in range(10, 17):
    m = (N - 2) ** 2
    e = 4 * (N - 2)
    j = 2 * (N - 2) * (N - 3)
    page = lf(m) + m * math.log10(4) - j * math.log10(17)
    naive = lf(4) + lf(e) + lf(m) + m * math.log10(4) - (j + e) * math.log10(17) - 4 * (N - 1) * math.log10(5)
    print("%2d | %6.1f | %6.1f | %6.1f | %6.1f" % (N, page, naive, census(N), census(N, pinned=5)))
