"""Review item 2: a bound on sound sketch tables that does not need 'tracked subsets are extremal'.
Lookahead lemma (review, section 2): let the bottom table be built by a sketch DP whose state is (frontier, s), s a function of
the tile ids chosen without looking at colours. If at bottom layer j the DP holds at most theta * N_B(j) states, then at least
about (1 - theta) of the strict nodes at depth D = 256 - j have an ancestor at the cut that passes the table. Hence
   cost >= max( top levels up to the cut , max over D beyond the cut of  N_A(D) N_B(256-D) / (N_A(D) + N_B(256-D)) ).
The right-hand side is at least half of  m* = max over D of min(N_A(D), N_B(256-D)),  whatever the cut and the sketch."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from r_common import *
A = levels(ROWS_BU); Bq = levels(ROWS_BU[::-1])
def hm(D): return A[D - 1] + Bq[255 - D] - lsum([A[D - 1], Bq[255 - D]])
m = [(min(A[D - 1], Bq[255 - D]), D) for D in range(1, 256)]
print("two-sided level m(D) = min(N_A(D), N_B(256-D)): largest 2^%.2f at D=%d" % max(m))
best = None
print("   cut d | top widest level to d | largest harmonic term beyond d (at D) | bound | strict join at this cut (widest levels)")
for d in range(1, 256):
    top = max(A[:d]); h = max((hm(D), D) for D in range(d, 256))
    bound = max(top, h[0]); strict = max(top, max(Bq[:256 - d]))
    if best is None or bound < best[0]: best = (bound, d)
    if d % 16 == 0 or d in (137, 139, 150): print("%8d | %6.1f | %6.1f (%d) | %6.1f | %6.1f" % (d, top, h[0], h[1], bound, strict))
print("smallest bound over cuts: 2^%.2f at d=%d ; strict two-list join 2^128.71 ; tree widest level 2^%.2f" % (*best, max(A)))
print("so one oblivious sound sketch table, of any kind, saves at most %.1f bits on the strict join and %.1f bits on the tree, in widest-level terms" % (128.71 - best[0], max(A) - best[0]))
