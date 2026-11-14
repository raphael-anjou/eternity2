"""Near-critical interior colour count for the generated framed boards (toys).
First-moment count of fillings of a random (unplanted) board with cf frame colours and ci interior colours, one corner tile fixed:
  E = 3! * (4(n-2))! * ((n-2)^2)! * 4^((n-2)^2) / ( cf^(4(n-1)) * ci^(2(n-1)(n-2)) )
ci_real solves E = 1; the ladder uses ceil(ci_real), the smallest whole count with E under 1.
Run: python3 -I crit.py"""
from math import lgamma, log, exp, ceil
def L(n, cf, ci):
    e = 4 * (n - 2); i = (n - 2) ** 2
    return lgamma(4) + lgamma(e + 1) + lgamma(i + 1) + i * log(4) - 4 * (n - 1) * log(cf) - 2 * (n - 1) * (n - 2) * log(ci)
for n in range(5, 13):
    cf = 4
    num = L(n, cf, 1); ci_real = exp(num / (2 * (n - 1) * (n - 2)))
    c = ceil(ci_real)
    print("n=%2d cf=%d  critical ci %.2f  ladder ci %d (log10 E %.2f)  one below: log10 E %.2f" % (n, cf, ci_real, c, L(n, cf, c) / log(10), L(n, cf, c - 1) / log(10)))
# the official puzzle with the same uniform formula, for reference: 5 frame colours, 17 interior, no clue (corner fixed)
n = 16
print("16x16 uniform formula, cf=5: critical ci %.2f (the puzzle has 17)" % exp(L(16, 5, 1) / (2 * 15 * 14)))
