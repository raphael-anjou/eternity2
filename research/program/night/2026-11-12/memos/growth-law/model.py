"""First-moment model of DFS tree size on planted framed edge-matching boards.

Stdlib only. Import, or run `python3 -I model.py` for a self-test table.

Notation (see ../growth-law.md, section 2):
  n        board side, N = n*n cells, w = n-2, P = w*w interior pieces
  c        interior colours, cf frame colours (5 in the ladder generator)
  order    list of cells (x, y), the static fill order
  b_t      expected number of consistent candidates for the t-th cell
  A(d)     prod_{t<=d} b_t, annealed count of consistent partial boards
  H_j(d)   expected nodes at depth d hanging off the planted path at depth j
"""
import math
from functools import lru_cache


# ---------------------------------------------------------------- geometry
def cell_class(n, x, y):
    k = (x in (0, n - 1)) + (y in (0, n - 1))
    return ("interior", "edge", "corner")[k]


def edge_kind(n, a, b):
    """'F' for a seam joining two rim cells along the rim, else 'I'."""
    (x1, y1), (x2, y2) = a, b
    if y1 == y2 and y1 in (0, n - 1):
        return "F"
    if x1 == x2 and x1 in (0, n - 1):
        return "F"
    return "I"


def neighbours(n, x, y):
    for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
        if 0 <= x + dx < n and 0 <= y + dy < n:
            yield (x + dx, y + dy)


def order_rowmajor(n):
    return [(x, y) for y in range(n) for x in range(n)]


def ring(n, k):
    """Cells of the k-th ring (k = 0 is the frame), clockwise from top-left."""
    lo, hi = k, n - 1 - k
    if lo > hi:
        return []
    if lo == hi:
        return [(lo, lo)]
    out = [(x, lo) for x in range(lo, hi + 1)]
    out += [(hi, y) for y in range(lo + 1, hi + 1)]
    out += [(x, hi) for x in range(hi - 1, lo - 1, -1)]
    out += [(lo, y) for y in range(hi - 1, lo, -1)]
    return out


def order_border_first(n):
    """Frame clockwise, then the interior row-major."""
    return ring(n, 0) + [(x, y) for y in range(1, n - 1) for x in range(1, n - 1)]


def order_spiral_in(n):
    out = []
    for k in range((n + 1) // 2):
        out += ring(n, k)
    return out


def order_diagonal(n):
    """Anti-diagonal sweeps from the top-left corner."""
    return sorted(((x, y) for y in range(n) for x in range(n)), key=lambda p: (p[0] + p[1], p[1]))


ORDERS = {
    "rowmajor": order_rowmajor,
    "border-first": order_border_first,
    "spiral-in": order_spiral_in,
    "diagonal": order_diagonal,
}


def steps(n, order):
    """Per step: (class, remaining pieces of the class, rotations, new I edges, new F edges)."""
    left = {"corner": 4, "edge": 4 * (n - 2), "interior": (n - 2) ** 2}
    placed = set()
    out = []
    for cell in order:
        cls = cell_class(n, *cell)
        a = f = 0
        for nb in neighbours(n, *cell):
            if nb in placed:
                if edge_kind(n, cell, nb) == "F":
                    f += 1
                else:
                    a += 1
        out.append((cls, left[cls], 4 if cls == "interior" else 1, a, f))
        left[cls] -= 1
        placed.add(cell)
    return out


def edge_totals(n):
    w = n - 2
    return 2 * w * (w + 1), 4 * (n - 1)  # interior-colour seams, frame-colour seams


# ---------------------------------------------------------------- match probabilities
def colour_counts(edges, colours):
    """The generator cycles colours over the seams of a band: counts differ by at most 1."""
    base, extra = divmod(edges, colours)
    return [base + (1 if i < extra else 0) for i in range(colours)]


@lru_cache(maxsize=None)
def urn_log(edges, colours, kmax=None):
    """ln U(k), k = 0..edges: probability that k disjoint pairs of half-edges,
    drawn without replacement from 2*edges half-edges (2*E_i of colour i), are all
    monochromatic. U(k) = k! [z^k] prod_i sum_j (h_i)_{2j} z^j / j!  /  (H)_{2k}."""
    kmax = edges if kmax is None else kmax
    counts = colour_counts(edges, colours)
    T = [1] + [0] * kmax  # T[k] = sum over (k_i) of multinomial * prod (h_i)_{2 k_i}
    binom = [[1]]
    for k in range(1, kmax + 1):
        row = [1] * (k + 1)
        for j in range(1, k):
            row[j] = binom[k - 1][j - 1] + binom[k - 1][j]
        binom.append(row)
    for e_i in counts:
        h = 2 * e_i
        ff = [1]
        for j in range(1, min(kmax, e_i) + 1):
            ff.append(ff[-1] * (h - 2 * j + 2) * (h - 2 * j + 1))
        new = [0] * (kmax + 1)
        for k in range(kmax + 1):
            s = 0
            for j in range(0, min(k, len(ff) - 1) + 1):
                if T[k - j]:
                    s += binom[k][j] * T[k - j] * ff[j]
            new[k] = s
        T = new
    H = 2 * edges
    out = []
    lden = 0.0
    for k in range(kmax + 1):
        if k:
            lden += math.log(H - 2 * k + 2) + math.log(H - 2 * k + 1)
        out.append(math.log(T[k]) - lden if T[k] else -math.inf)
    return tuple(out)


def log_match(n, c, cf, variant):
    """Return (LI, LF): LI[k] = ln P(k interior-colour seams all match)."""
    EI, EF = edge_totals(n)
    if variant == "naive":
        return ([-k * math.log(c) for k in range(EI + 1)],
                [-k * math.log(cf) for k in range(EF + 1)])
    if variant == "urn":
        return list(urn_log(EI, c)), list(urn_log(EF, cf))
    raise ValueError(variant)


# ---------------------------------------------------------------- the profile
class Profile:
    """ln b_t, ln A(d) and the planted-ensemble quantities for one (n, c, order).

    A planted board has four solutions for free (the board turned by quarter
    turns). `holders[t]` is the number of candidates at step t that are sure
    to lead to a solution on the planted path: 4 at the first cell, 1 after."""

    def __init__(self, n, c, order="rowmajor", cf=5, variant="naive", deplete=True):
        self.n, self.c, self.cf = n, c, cf
        EI, EF = edge_totals(n)
        self.N = n * n
        cells = ORDERS[order](n) if isinstance(order, str) else list(order)
        st = steps(n, cells)
        LI, LF = log_match(n, c, cf, variant)
        kI = kF = 0
        self.lnb, self.lnwrong = [], []
        for t, (cls, r, rho, a, f) in enumerate(st):
            lm = LI[kI + a] - LI[kI] + LF[kF + f] - LF[kF]
            kI += a
            kF += f
            self.lnb.append(math.log(r * rho) + lm)
            h = 4 if t == 0 else 1
            # A wrong candidate next to planted neighbours: the planted partner
            # holds one of the E/c seams of the wanted colour, so a seam matches
            # with probability (E/c - 1) / (E - 1) instead of 1/c.
            lw = a * math.log(max(EI / c - 1.0, 1e-9) / (EI - 1.0)) + f * math.log(max(EF / cf - 1.0, 1e-9) / (EF - 1.0)) if deplete else lm
            self.lnwrong.append(math.log(r * rho - h) + lw if r * rho > h else -math.inf)
        self.lnA = [0.0]
        for v in self.lnb:
            self.lnA.append(self.lnA[-1] + v)
        self.st = st
        self.cells = cells

    def A(self, d):
        """Annealed count of consistent partial boards at depth d (random ensemble)."""
        return math.exp(self.lnA[d])

    def lnH(self, j, d):
        """ln of expected nodes at depth d > j that follow one planted path for j cells and leave it at cell j+1."""
        return self.lnwrong[j] + self.lnA[d] - self.lnA[j + 1]

    def H(self, j, d):
        return math.exp(self.lnH(j, d)) if self.lnwrong[j] > -math.inf else 0.0

    def Z_planted(self, d):
        """Expected consistent partial boards at depth d >= 1 in the planted ensemble."""
        return 4.0 * (1.0 + sum(self.H(j, d) for j in range(1, d))) + self.H(0, d)

    def W(self, j):
        """Expected size of the hanging tree at depth j of one planted path (all wrong siblings)."""
        return sum(self.H(j, d) for d in range(j + 1, self.N + 1))

    def other_solutions(self, j=1):
        """Expected solutions that share the first j >= 1 cells of one planted path, the planted one excluded."""
        return sum(self.H(i, self.N) for i in range(j, self.N))

    def solutions_planted(self):
        return 4.0 * (1.0 + self.other_solutions(1)) + self.H(0, self.N)

    def full_tree(self):
        return sum(self.Z_planted(d) for d in range(1, self.N + 1))

    def peak(self):
        d = max(range(self.N + 1), key=lambda t: self.lnA[t])
        return d, self.lnA[d]

    # ------------------------------------------------ first-solution cost
    def first_solution_terms(self):
        """Per depth j of the planted path: (holders, b'_j, w_j, p_j, w_sf_j).
        b' expected wrong consistent siblings, w mean subtree size of one of them,
        p probability that such a subtree holds a solution, w_sf mean size of a
        solution-free one."""
        N = self.N
        Wj = [self.W(j) for j in range(N)]
        tail = [0.0] * (N + 2)  # full planted tree below depth j, one copy
        for j in range(N - 1, 0, -1):
            tail[j] = 1.0 + Wj[j] + tail[j + 1]
        out = []
        for j in range(N):
            h = 4 if j == 0 else 1
            if self.lnwrong[j] == -math.inf:
                out.append((h, 0.0, 0.0, 0.0, 0.0))
                continue
            bw = math.exp(self.lnwrong[j])
            w = Wj[j] / bw
            sigma = math.exp(self.lnA[N] - self.lnA[j + 1])  # E[solutions below a generic depth-(j+1) node]
            p = min(1.0, sigma / (1.0 + self.other_solutions(j + 1)))
            V = tail[j + 1] + 1.0  # size of a subtree known to hold a solution
            wsf = (w - p * V) / (1.0 - p) if p < 1.0 else 1.0
            wsf = min(max(wsf, min(1.0, w)), w)
            out.append((h, bw, w, p, wsf))
        return out

    def first_solution_mean(self):
        """Mean nodes to a first solution, value order uniformly random.
        Per level: K wrong siblings (Poisson b'), G of them hold a solution
        (binomial p), plus h sure holders. DFS explores the solution-free
        siblings that precede the first holder: (K - G) / (G + h + 1) on average."""
        tot = float(self.N)
        for h, bw, w, p, wsf in self.first_solution_terms():
            if bw <= 0:
                continue
            tot += bw * (1.0 - p) * _e_inv(bw * p, h + 1) * wsf
        return tot

    def first_solution_sample(self, rng, size=2001):
        """Sorted samples of the first-solution cost over value orders (mean subtree sizes)."""
        terms = self.first_solution_terms()
        out = []
        for _ in range(size):
            tot = float(self.N)
            for h, bw, w, p, wsf in terms:
                if bw <= 0:
                    continue
                K = _poisson(rng, bw)
                if K == 0:
                    continue
                G = sum(1 for _ in range(K) if rng.random() < p) if K < 200 else int(round(K * p))
                free = K - G
                if free == 0:
                    continue
                if free < 200:
                    before = _before(rng, free, G + h)
                else:
                    before = int(free * _beta_min(rng, G + h))
                tot += before * wsf
            out.append(tot)
        out.sort()
        return out


def _e_inv(lam, a):
    """E[1 / (G + a)] for G ~ Poisson(lam)."""
    if lam < 1e-12:
        return 1.0 / a
    if lam > 700:
        return 1.0 / (lam + a)
    term = math.exp(-lam)
    tot = term / a
    g = 0
    top = int(lam + 12 * math.sqrt(lam) + 40)
    while g < top:
        g += 1
        term *= lam / g
        tot += term / (g + a)
    return tot


def _poisson(rng, lam):
    if lam > 50:
        return max(0, int(round(rng.gauss(lam, math.sqrt(lam)))))
    L = math.exp(-lam)
    k, p = 0, 1.0
    while True:
        p *= rng.random()
        if p <= L:
            return k
        k += 1


def _beta_min(rng, g):
    """Position (as a fraction) of the first of g uniform points."""
    return 1.0 - rng.random() ** (1.0 / g)


def _before(rng, free, holders):
    """Number of `free` items that precede the first of `holders` items in a random order."""
    cut = _beta_min(rng, holders)
    return sum(1 for _ in range(free) if rng.random() < cut)


def owen(n):
    P = (n - 2) ** 2
    return math.exp((math.lgamma(P + 1) + P * math.log(4)) / (2 * P))


def ln_solutions(n, c, cf=5, variant="naive"):
    """ln of the annealed expected number of solutions of the whole board (random ensemble, all four turns counted)."""
    return Profile(n, c, "rowmajor", cf, variant).lnA[n * n]


def critical_c(n, cf=5):
    """Real c at which the naive whole-board first moment equals 1."""
    w = n - 2
    P = w * w
    lnF = math.log(24) + math.lgamma(4 * w + 1) - 4 * (n - 1) * math.log(cf)
    lnI = math.lgamma(P + 1) + P * math.log(4)
    return math.exp((lnF + lnI) / (2 * w * (w + 1)))


if __name__ == "__main__":
    print("n  Owen   whole-board critical c (naive)")
    for n in (6, 7, 8, 9, 10, 12, 16):
        print(n, round(owen(n), 3), round(critical_c(n), 3))
