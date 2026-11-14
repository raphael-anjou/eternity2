#!/usr/bin/env python3
"""First-moment counting model for edge matching with breaks. Stdlib only.

Run:  python3 -I model.py            (about one minute, one core)
Writes model_results.json next to this file and prints the tables the memo cites.

Ensemble. The piece SHAPES are fixed (4 corners, border pieces, interior pieces).
The colours are shuffled: the interior-colour half-edges uniformly over all
interior-colour sides, the ring half-edges uniformly over the clockwise-facing
ring sides and, separately, over the anticlockwise-facing ring sides, keeping the
colour counts of the instance. For a FIXED placement of pieces, the induced
pairing of half-edges across joints is then a uniformly random perfect matching
(interior) and a uniformly random bijection (ring), so every expectation below is
an exact first moment over that ensemble. Piece distinctness is exact (placements
are permutations). Neglected: that the real instance is planted, not a typical
draw; all correlations beyond the first moment.
"""
import json
import math
import os
import sys
from math import comb, factorial, lgamma, log, exp

LN10 = math.log(10.0)


def dfact(n):
    """n!! for odd n >= -1."""
    r = 1
    while n > 1:
        r *= n
        n -= 2
    return r


def poly_mul(a, b):
    out = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        if x:
            for j, y in enumerate(b):
                out[i + j] += x * y
    return out


def binomial_inversion(G, E):
    """G[m] = sum_s C(s, m) X[s]  ->  X[s] (exact integers)."""
    X = [0] * (E + 1)
    for s in range(E + 1):
        tot = 0
        sign = 1
        for m in range(s, E + 1):
            tot += sign * comb(m, s) * G[m]
            sign = -sign
        X[s] = tot
    return X


def interior_dist(counts):
    """D[k] = number of perfect matchings of labelled half-edges (colour counts
    `counts`) with exactly k bichromatic pairs. Exact integers.
    Method: A[m] = number of ways to mark m disjoint monochromatic pairs;
    G[m] = A[m] * (M-2m-1)!! counts (matching, m marked mono pairs) = sum_s C(s,m) X[s]
    with X[s] the number of matchings with exactly s mono pairs."""
    M = sum(counts)
    E = M // 2
    A = [1]
    for n in counts:
        poly = [factorial(n) // (factorial(j) * factorial(n - 2 * j) * 2 ** j) for j in range(n // 2 + 1)]
        A = poly_mul(A, poly)
    A += [0] * (E + 1 - len(A))
    G = [A[m] * dfact(M - 2 * m - 1) for m in range(E + 1)]
    X = binomial_inversion(G, E)
    D = [X[E - k] for k in range(E + 1)]
    assert sum(D) == dfact(M - 1)
    assert all(d >= 0 for d in D)
    return D


def ring_dist(cw, ccw):
    """D[k] = number of bijections cw-side -> ccw-side half-edges with exactly k
    bichromatic pairs (rook polynomial of the same-colour blocks). Exact."""
    n = sum(cw)
    R = [1]
    for x, y in zip(cw, ccw):
        R = poly_mul(R, [comb(x, i) * comb(y, i) * factorial(i) for i in range(min(x, y) + 1)])
    R += [0] * (n + 1 - len(R))
    G = [R[j] * factorial(n - j) for j in range(n + 1)]
    X = binomial_inversion(G, n)
    D = [X[n - k] for k in range(n + 1)]
    assert sum(D) == factorial(n)
    assert all(d >= 0 for d in D)
    return D


def lcomb(n, k):
    if k < 0 or k > n:
        return -math.inf
    return lgamma(n + 1) - lgamma(k + 1) - lgamma(n - k + 1)


def logsumexp(xs):
    xs = [x for x in xs if x > -math.inf]
    if not xs:
        return -math.inf
    m = max(xs)
    return m + log(sum(exp(x - m) for x in xs))


class JointLaw:
    """Law of the number of broken joints among e specified joints of one class,
    from the exact full-board distribution D (hypergeometric thinning: exact)."""

    def __init__(self, D):
        self.E = len(D) - 1
        tot = sum(D)
        self.logp = [(log(d) - log(tot)) if d > 0 else -math.inf for d in D]
        self._cache = {}

    def lp(self, e, j):
        """ln P(exactly j of e specified joints are broken)."""
        key = (e, j)
        v = self._cache.get(key)
        if v is None:
            E = self.E
            terms = []
            if 0 <= j <= e <= E:
                base = -lcomb(E, e)
                for k in range(j, E - (e - j) + 1):
                    lp = self.logp[k]
                    if lp > -math.inf:
                        terms.append(lp + lcomb(k, j) + lcomb(E - k, e - j) + base)
            v = logsumexp(terms)
            self._cache[key] = v
        return v

    def lspec(self, e, j):
        """ln P(a SPECIFIC set of j joints among e is exactly the broken set)."""
        return self.lp(e, j) - lcomb(e, j)


def balanced(total_joints, colours):
    """Joint counts per colour as the generator paints them (cycled, so equal up to 1)."""
    base, extra = divmod(total_joints, colours)
    return [base + (1 if i < extra else 0) for i in range(colours)]


class Board:
    def __init__(self, n, ring_joint_counts, int_joint_counts, pinned_corner=False, pinned_interior=0):
        self.n = n
        self.ring = JointLaw(ring_dist(ring_joint_counts, ring_joint_counts))
        self.inner = JointLaw(interior_dist([2 * c for c in int_joint_counts]))
        self.Er = sum(ring_joint_counts)
        self.Ei = sum(int_joint_counts)
        assert self.Er == 4 * (n - 1) and self.Ei == 2 * (n - 1) * (n - 2)
        self.n_corner, self.n_edge, self.n_int = 4, 4 * (n - 2), (n - 2) ** 2
        self.pinned_corner = pinned_corner
        self.pinned_interior = pinned_interior
        # row scan: per cell (class, [types of the joints it closes])
        self.cells = []
        for y in range(n):
            for x in range(n):
                bx, by = x in (0, n - 1), y in (0, n - 1)
                cls = 'c' if (bx and by) else ('e' if (bx or by) else 'i')
                joints = []
                if x > 0:
                    joints.append('r' if by else 'i')
                if y > 0:
                    joints.append('r' if bx else 'i')
                self.cells.append((cls, joints))

    def log_w(self):
        """ln of the number of class-respecting placements of the full board."""
        c = 3 if self.pinned_corner else 4
        ni = self.n_int - self.pinned_interior
        return lgamma(c + 1) + lgamma(self.n_edge + 1) + lgamma(ni + 1) + ni * log(4)

    def log_n_exact(self, k):
        """ln E[number of complete boards with exactly k broken joints]."""
        terms = [self.ring.lp(self.Er, kr) + self.inner.lp(self.Ei, k - kr) for kr in range(0, min(k, self.Er) + 1)]
        return self.log_w() + logsumexp(terms)

    def prefix_stats(self):
        """Per depth d = 0..n^2: ln A(d) (ordered class-respecting choices of the
        first d pieces), ring joints closed, interior joints closed."""
        out = [(0.0, 0, 0)]
        la, er, ei = 0.0, 0, 0
        left = {'c': self.n_corner, 'e': self.n_edge, 'i': self.n_int}
        for cls, joints in self.cells:
            la += log(left[cls]) + (log(4) if cls == 'i' else 0.0)
            left[cls] -= 1
            er += joints.count('r')
            ei += joints.count('i')
            out.append((la, er, ei))
        return out

    def tree(self, K, cap_at, per_cell=1, may_break=None):
        """First-moment search tree of a row-scan DFS that tolerates breaks.
        cap_at(d) = largest total number of breaks allowed once d pieces are placed.
        per_cell = largest number of breaks one placement may create.
        may_break(d) = whether the d-th placement may create a break at all (default: yes).
        Returns per depth d the dict {j: ln E[number of partial boards at depth d with j breaks]}."""
        stats = self.prefix_stats()
        pat = {(0, 0): 1}  # (interior breaks, ring breaks) -> number of allowed break patterns
        levels = [{0: 0.0}]
        for d, (cls, joints) in enumerate(self.cells, start=1):
            cap = min(K, cap_at(d))
            new = {}
            ni, nr = joints.count('i'), joints.count('r')
            for (ji, jr), cnt in pat.items():
                for bi in range(ni + 1):
                    for br in range(nr + 1):
                        b = bi + br
                        if b > per_cell or ji + jr + b > cap:
                            continue
                        if b and may_break is not None and not may_break(d):
                            continue
                        key = (ji + bi, jr + br)
                        new[key] = new.get(key, 0) + cnt * comb(ni, bi) * comb(nr, br)
            pat = new
            la, er, ei = stats[d]
            lev = {}
            for (ji, jr), cnt in pat.items():
                v = la + log(cnt) + self.inner.lspec(ei, ji) + self.ring.lspec(er, jr)
                lev.setdefault(ji + jr, []).append(v)
            levels.append({j: logsumexp(v) for j, v in lev.items()})
        return levels


def even_split(total_half_edges, colours):
    """Even half-edge counts per colour, as balanced as evenness allows."""
    pairs = total_half_edges // 2
    return [2 * c for c in balanced(pairs, colours)]


def tail_yield(board, d0, K, cap_at, int_colours, ring_colours):
    """ln E[number of completions of ONE perfect prefix of depth d0] when the half-edges
    left for the tail (unused piece sides plus the frontier) are spread evenly over
    int_colours interior colours and ring_colours ring colours. Same break rule as tree()."""
    stats = board.prefix_stats()
    la0, er0, ei0 = stats[d0]
    la1, er1, ei1 = stats[-1]
    er, ei = er1 - er0, ei1 - ei0
    inner = JointLaw(interior_dist(even_split(2 * ei, int_colours)))
    rc = balanced(er, ring_colours)
    ring = JointLaw(ring_dist(rc, rc))
    pat = {(0, 0): 1}
    for d, (cls, joints) in enumerate(board.cells, start=1):
        if d <= d0:
            continue
        cap = min(K, cap_at(d))
        new = {}
        ni, nr = joints.count('i'), joints.count('r')
        for (ji, jr), cnt in pat.items():
            for bi in range(ni + 1):
                for br in range(nr + 1):
                    if bi + br > 1 or ji + jr + bi + br > cap:
                        continue
                    key = (ji + bi, jr + br)
                    new[key] = new.get(key, 0) + cnt * comb(ni, bi) * comb(nr, br)
        pat = new
    terms = [log(cnt) + inner.lspec(ei, ji) + ring.lspec(er, jr) for (ji, jr), cnt in pat.items()]
    return (la1 - la0) + logsumexp(terms), ei, er


def tail_first_moment(board, depth, int_colours, ring_colours, bmax=60):
    """For the tail that starts after `depth` placed cells (row scan), with its half-edges
    spread evenly over int_colours and ring_colours colours, breaks allowed anywhere:
    ln E[#completions with <= b breaks] for b = 0..bmax, (a) with the real pool, each piece
    once (exact first moment), (b) when every tail cell may take ANY remaining piece of its
    class, reuse allowed (independent-edge approximation, p = 1 / colours).
    Returns (list_distinct, list_reuse, interior joints, ring joints)."""
    stats = board.prefix_stats()
    la0, er0, ei0 = stats[depth]
    la1, er1, ei1 = stats[-1]
    er, ei = er1 - er0, ei1 - ei0
    ci, cr = min(int_colours, max(1, ei)), min(ring_colours, max(1, er))
    inner = JointLaw(interior_dist(even_split(2 * ei, ci)))
    rc = balanced(er, cr)
    ring = JointLaw(ring_dist(rc, rc))
    left = {'c': 0, 'e': 0, 'i': 0}
    for cls, _ in board.cells[depth:]:
        left[cls] += 1
    la_rel = sum(log(left[cls]) + (log(4) if cls == 'i' else 0.0) for cls, _ in board.cells[depth:])
    pi, pr = 1.0 / ci, 1.0 / cr
    dist, reuse = [], []
    cd, cu = [], []
    for k in range(0, bmax + 1):
        td, tu = [], []
        for kr in range(0, min(k, er) + 1):
            ki = k - kr
            if ki > ei:
                continue
            td.append(inner.lp(ei, ki) + ring.lp(er, kr))
            lqi = log(1 - pi) if pi < 1 else -math.inf
            lqr = log(1 - pr) if pr < 1 else -math.inf
            a = lcomb(ei, ki) + (ei - ki) * log(pi) + (ki * lqi if ki else 0.0)
            c = lcomb(er, kr) + (er - kr) * log(pr) + (kr * lqr if kr else 0.0)
            tu.append(a + c)
        cd.append((la1 - la0) + logsumexp(td))
        cu.append(la_rel + logsumexp(tu))
        dist.append(logsumexp(cd))
        reuse.append(logsumexp(cu))
    return dist, reuse, ei, er


def reuse_log_counts(board, depth, int_colours, ring_colours, bmax):
    """ln E[#completions of the tail after `depth` with <= b breaks, pieces reusable], b = 0..bmax
    (independent-edge approximation). Cheap version of the second list of tail_first_moment."""
    stats = board.prefix_stats()
    er = stats[-1][1] - stats[depth][1]
    ei = stats[-1][2] - stats[depth][2]
    left = {'c': 0, 'e': 0, 'i': 0}
    for cls, _ in board.cells[depth:]:
        left[cls] += 1
    la = sum(log(left[cls]) + (log(4) if cls == 'i' else 0.0) for cls, _ in board.cells[depth:])
    lpi, lqi = log(1.0 / int_colours), log(1 - 1.0 / int_colours)
    lpr, lqr = log(1.0 / ring_colours), log(1 - 1.0 / ring_colours)
    out, cum = [], []
    for k in range(bmax + 1):
        t = [lcomb(ei, k - kr) + (ei - k + kr) * lpi + (k - kr) * lqi + lcomb(er, kr) + (er - kr) * lpr + kr * lqr
             for kr in range(0, min(k, er) + 1) if k - kr <= ei]
        cum.append(la + logsumexp(t))
        out.append(logsumexp(cum))
    return out


def first_at_least_one(xs):
    for k, v in enumerate(xs):
        if v >= 0:
            return k
    return None


def summarise(levels):
    tot_by_depth = [logsumexp(list(l.values())) for l in levels]
    total = logsumexp(tot_by_depth[1:])
    final = tot_by_depth[-1]
    peak_d = max(range(1, len(levels)), key=lambda d: tot_by_depth[d])
    return {
        'log10_nodes': total / LN10,
        'log10_boards': final / LN10,
        'log10_nodes_per_board': (total - final) / LN10,
        'peak_depth': peak_d,
        'log10_peak': tot_by_depth[peak_d] / LN10,
        'by_depth_log10': [t / LN10 for t in tot_by_depth],
    }


def odd_colour_law(colours, jmax):
    """Number of colours with an odd number of half-edges in broken joints after j
    breaks, each break a uniform unordered pair of distinct colours (Markov chain)."""
    pairs = comb(colours, 2)
    law = [{0: 1.0}]
    for _ in range(jmax):
        cur, nxt = law[-1], {}
        for o, p in cur.items():
            for o2, w in ((o - 2, comb(o, 2)), (o + 2, comb(colours - o, 2)), (o, o * (colours - o))):
                if w:
                    nxt[o2] = nxt.get(o2, 0.0) + p * w / pairs
        law.append(nxt)
    return law


def main():
    res = {}
    here = os.path.dirname(os.path.abspath(__file__))

    # ---------- official 16x16: counts from web/content/research/why/the-470-wall.mdx ----------
    # ring: 5 colours x 12 joints; interior: five colours on 48 half-edges, twelve on 50.
    E2 = Board(16, [12] * 5, [24] * 5 + [25] * 12)
    print('== 16x16, exact first moment over the colour-shuffle ensemble ==')
    w_none = E2.log_w()
    w_start = w_none - log(196 * 4)
    w_five = lgamma(5) + lgamma(57) + lgamma(192) + 191 * log(4)
    ind = []  # independent-edge model of the 470-wall page, for comparison
    lp_r, lq_r = log(0.2), log(0.8)
    pi = sum((n / 840.0) ** 2 for n in [48] * 5 + [50] * 12)
    lp_i, lq_i = log(pi), log(1 - pi)
    rows = []
    print(' k score  log10N(no clue)  starter  five clues | indep-edge  exact/indep | N(k)/N(k-1)')
    prev = None
    for k in range(0, 21):
        ln = E2.log_n_exact(k)
        li = w_none + logsumexp([lcomb(60, kr) + (60 - kr) * lp_r + kr * lq_r + lcomb(420, k - kr) + (420 - k + kr) * lp_i + (k - kr) * lq_i
                                 for kr in range(0, k + 1)])
        ratio = (ln - prev) / LN10 if (prev is not None and prev > -math.inf and ln > -math.inf) else None
        rows.append({'k': k, 'score': 480 - k, 'log10_N_noclue': ln / LN10, 'log10_N_starter': (ln - w_none + w_start) / LN10,
                     'log10_N_fiveclues': (ln - w_none + w_five) / LN10, 'log10_N_indep': li / LN10,
                     'log10_ratio_prev': ratio})
        f = lambda v: '   -inf' if v == -math.inf else '%7.2f' % (v / LN10)
        print('%2d  %3d   %s        %s   %s   |  %7.2f     %s  | %s' % (
            k, 480 - k, f(ln), f(ln - w_none + w_start), f(ln - w_none + w_five), li / LN10,
            f(ln - li), '      -' if ratio is None else '%7.2f' % ratio))
        prev = ln
    res['e2_counts'] = rows
    print('N(467)/N(480) = 10^%.2f (Owen, msg 6412, as quoted on build/reduce/edge-slipping.mdx: 2.05e35 = 10^35.31)' % (
        (E2.log_n_exact(13) - E2.log_n_exact(0)) / LN10))
    print('Max formula 2^(N-1) C(420,N): 10^%.2f at N=10, 10^%.2f at N=13' % (
        (9 * log(2) + lcomb(420, 10)) / LN10, (12 * log(2) + lcomb(420, 13)) / LN10))
    # interior-only parity factor: P(defect multigraph on colours is Eulerian | k random bichromatic pairs)
    law = odd_colour_law(17, 20)
    res['p_eulerian_17'] = [law[j].get(0, 0.0) for j in range(21)]
    print('P(all 17 interior colours even | j breaks):', ' '.join('%d:%.2e' % (j, law[j].get(0, 0.0)) for j in (2, 4, 6, 8, 9, 10, 11, 12, 14, 20)))

    # ---------- search trees on the 16x16 ----------
    print('\n== 16x16 row-scan search tree (first moment, no clue) ==')
    N2 = 256
    strict = summarise(E2.tree(0, lambda d: 0))
    print('strict: log10 nodes %.2f, peak depth %d (log10 %.2f), log10 boards %.2f, log10 nodes/board %.2f' % (
        strict['log10_nodes'], strict['peak_depth'], strict['log10_peak'], strict['log10_boards'], strict['log10_nodes_per_board']))
    bd = strict['by_depth_log10']
    print('strict tree width log10 at depths 50,100,150,170,180,190,200,210,220,230,240,250,256:',
          ' '.join('%.1f' % bd[d] for d in (50, 100, 150, 170, 180, 190, 200, 210, 220, 230, 240, 250, 256)))
    res['strict'] = {k: v for k, v in strict.items()}

    blackwood = [201, 206, 211, 216, 221, 225, 229, 233, 237, 239]

    def cumulative(unlocks):
        return lambda d: sum(1 for u in unlocks if d >= u)

    def late_share(levels, d0):
        tot = [logsumexp(list(l.values())) for l in levels]
        return exp(logsumexp(tot[d0:]) - logsumexp(tot[1:]))

    sched = {}

    def run(name, K, cap_at, per_cell=1, may_break=None):
        lv = E2.tree(K, cap_at, per_cell, may_break)
        s = summarise(lv)
        s['late_share_from_201'] = late_share(lv, 201)
        sched[name] = (lv, s)
        print('%-34s K=%2d  log10 boards %7.2f  log10 nodes %7.2f  log10 nodes/board %7.2f  peak d=%3d  share of nodes at d>=201: %.3g' % (
            name, K, s['log10_boards'], s['log10_nodes'], s['log10_nodes_per_board'], s['peak_depth'], s['late_share_from_201']))
        return s

    print('-- budget 10, different rules for where breaks may go --')
    run('anywhere (<=2 per cell)', 10, lambda d: 10, per_cell=2)
    run('anywhere (<=1 per cell)', 10, lambda d: 10)
    run('early only (first 56 cells)', 10, lambda d: 10, may_break=lambda d: d <= 56)
    run('free from depth 151', 10, lambda d: 10 if d >= 151 else 0)
    run('free from depth 201', 10, lambda d: 10 if d >= 201 else 0)
    run('free from depth 231', 10, lambda d: 10 if d >= 231 else 0)
    run('Blackwood list', 10, cumulative(blackwood))
    run('Blackwood list shifted -40', 10, cumulative([u - 40 for u in blackwood]))
    run('Blackwood list shifted -20', 10, cumulative([u - 20 for u in blackwood]))
    run('Blackwood list shifted +8', 10, cumulative([u + 8 for u in blackwood]))

    print('-- price per point: nodes per board against the budget K --')
    price = []
    for K in range(0, 15):
        a = summarise(E2.tree(K, lambda d: K, per_cell=2))
        f = summarise(E2.tree(K, lambda d, K=K: K if d >= 201 else 0))
        # Blackwood-style list of length K: keep the last K unlock points of an evenly extended list
        ext = [181, 186, 191, 196] + blackwood
        b = summarise(E2.tree(K, cumulative(ext[len(ext) - K:] if K else [])))
        price.append({'K': K, 'score': 480 - K, 'anywhere': a['log10_nodes_per_board'], 'free201': f['log10_nodes_per_board'],
                      'cumulative_list': b['log10_nodes_per_board'], 'boards_anywhere': a['log10_boards'],
                      'boards_free201': f['log10_boards'], 'boards_list': b['log10_boards'],
                      'nodes_anywhere': a['log10_nodes'], 'nodes_free201': f['log10_nodes'], 'nodes_list': b['log10_nodes']})
        print('K=%2d score %3d | anywhere: nodes/board %6.2f (boards %6.2f nodes %6.2f) | free>=201: %6.2f (boards %6.2f nodes %6.2f) | cumulative list: %6.2f (boards %6.2f nodes %6.2f)' % (
            K, 480 - K, a['log10_nodes_per_board'], a['log10_boards'], a['log10_nodes'], f['log10_nodes_per_board'], f['log10_boards'],
            f['log10_nodes'], b['log10_nodes_per_board'], b['log10_boards'], b['log10_nodes']))
    res['price'] = price

    # coordinate descent on the unlock depths, K = 9, 10, 11 (model optimum of a cumulative list)
    print('-- model-optimal cumulative unlock lists (coordinate descent, step 2) --')
    opt = {}
    for K in (7, 8, 9, 10, 11, 12, 13):
        ext = [181, 186, 191, 196] + blackwood
        cur = ext[len(ext) - K:]
        best = summarise(E2.tree(K, cumulative(cur)))['log10_nodes_per_board']
        improved = True
        sweeps = 0
        while improved and sweeps < 3:
            improved = False
            sweeps += 1
            for i in range(K):
                for step in (-8, -2, 2, 8):
                    cand = sorted(cur[:i] + [min(255, max(2, cur[i] + step))] + cur[i + 1:])
                    v = summarise(E2.tree(K, cumulative(cand)))['log10_nodes_per_board']
                    if v < best - 1e-6:
                        best, cur, improved = v, cand, True
        s = summarise(E2.tree(K, cumulative(cur)))
        opt[K] = {'unlocks': cur, 'log10_nodes_per_board': best, 'log10_boards': s['log10_boards'], 'log10_nodes': s['log10_nodes']}
        print('K=%2d best list %s  log10 nodes/board %.2f (boards %.2f, nodes %.2f)' % (K, cur, best, s['log10_boards'], s['log10_nodes']))
    res['optimised_lists'] = opt

    # value of the colour composition of the tail pool (what a colour quota buys)
    print('-- completions per perfect depth-d0 prefix against the number of colours left in the tail (K=10, Blackwood list) --')
    ty = {}
    for d0 in (155, 180, 200):
        for ci, cr in ((17, 5), (16, 5), (15, 5), (15, 4), (14, 4), (13, 4)):
            v, ei, er = tail_yield(E2, d0, 10, cumulative(blackwood), ci, cr)
            ty['%d:%d:%d' % (d0, ci, cr)] = v / LN10
            print('d0=%d tail joints %d interior + %d ring, %2d interior colours, %d ring colours: log10 completions per prefix %.2f' % (d0, ei, er, ci, cr, v / LN10))
        lvb = sched['Blackwood list'][0]
        print('   ensemble average for comparison: log10 (boards / depth-%d strict prefixes) = %.2f' % (d0, (logsumexp(list(lvb[-1].values())) - lvb[d0][0]) / LN10))
    res['tail_yield_log10'] = ty

    # how loose is a bound that drops distinctness, as a function of the cells left
    print('-- tail of the 16x16 after depth d0: fewest breaks with E[#completions] >= 1, real pool against pool with reuse --')
    loose = []
    for d0 in (120, 150, 170, 184, 192, 200, 208, 216, 224, 232, 240, 244, 248, 252):
        row = {'d0': d0, 'cells_left': 256 - d0}
        for tag, ci, cr in (('17+5', 17, 5), ('15+4', 15, 4)):
            dist, reuse, ei, er = tail_first_moment(E2, d0, ci, cr)
            row[tag] = {'true': first_at_least_one(dist), 'reuse': first_at_least_one(reuse),
                        'log10_distinct_at_0': dist[0] / LN10, 'log10_reuse_at_0': reuse[0] / LN10,
                        'log10_reuse_at_10': reuse[10] / LN10, 'log10_distinct_at_10': dist[10] / LN10}
        loose.append(row)
        a, b = row['17+5'], row['15+4']
        print('d0=%3d cells left %3d | 17+5 colours: true %2s  reuse %2s  (log10 E reuse at b=0: %6.1f, distinct at b=10: %6.1f) | 15+4 colours: true %2s  reuse %2s' % (
            d0, 256 - d0, a['true'], a['reuse'], a['log10_reuse_at_0'], a['log10_distinct_at_10'], b['true'], b['reuse']))
    res['looseness'] = loose

    # an exact-frontier reuse bound at zero cost: node (d, j) survives with probability
    # min(1, E[#reuse completions with <= K - j breaks]) (Markov bound, typical pool).
    print('-- ideal reuse bound (frontier exact, distinctness dropped, free of charge) inside the search, K=10 --')
    K = 10
    rl = {d: reuse_log_counts(E2, d, 17, 5, K) for d in range(1, 256)}
    for d in (192, 200, 208, 216, 224, 232, 240, 248):
        print('   d=%d: log10 E[reuse completions with <= b breaks], b=2,4,6,8,10: %s' % (
            d, ' '.join('%.1f' % (rl[d][b] / LN10) for b in (2, 4, 6, 8, 10))))

    def bounded_cost(unlocks):
        lv = E2.tree(K, cumulative(unlocks))
        boards = logsumexp(list(lv[-1].values()))
        plain = logsumexp([logsumexp(list(l.values())) for l in lv[1:]])
        kept = []
        for d in range(1, 256):
            for j, v in lv[d].items():
                kept.append(v + min(0.0, rl[d][K - j]))
        kept.append(boards)
        return (plain - boards) / LN10, (logsumexp(kept) - boards) / LN10

    rb = {}
    for name, ul in (('Blackwood list', blackwood), ('model-optimal list without bound', opt[10]['unlocks'])):
        a, b_ = bounded_cost(ul)
        rb[name] = {'plain': a, 'bounded': b_}
        print('   %-34s log10 nodes/board: plain %.2f, with the ideal reuse bound %.2f' % (name, a, b_))
    cur = list(opt[10]['unlocks'])
    best = bounded_cost(cur)[1]
    improved, sweeps = True, 0
    while improved and sweeps < 4:
        improved = False
        sweeps += 1
        for i in range(K):
            for step in (-16, -8, -2, 2, 8):
                cand = sorted(cur[:i] + [min(255, max(2, cur[i] + step))] + cur[i + 1:])
                v = bounded_cost(cand)[1]
                if v < best - 1e-6:
                    best, cur, improved = v, cand, True
    rb['re-optimised with bound'] = {'unlocks': cur, 'bounded': best, 'plain': bounded_cost(cur)[0]}
    print('   list re-optimised WITH the ideal reuse bound: %s  log10 nodes/board %.2f (the same list without the bound: %.2f)' % (cur, best, rb['re-optimised with bound']['plain']))
    res['ideal_reuse_bound'] = rb

    # a PERFECT bound from depth d0, with the breaks then left free after d0
    print('-- perfect bound from depth d0 and all 10 breaks free after d0: log10 nodes per board --')
    sb = strict['by_depth_log10']
    orc = {}
    for d0 in (141, 151, 161, 171, 181, 191, 201, 211, 221, 231):
        lvf = E2.tree(10, lambda d, d0=d0: 10 if d >= d0 else 0)
        boards = logsumexp(list(lvf[-1].values()))
        nodes = logsumexp([x * LN10 for x in sb[1:d0]] + [boards + log(256 - d0 + 1)])
        orc[d0] = {'log10_boards': boards / LN10, 'log10_nodes_per_board': (nodes - boards) / LN10}
        print('d0=%d: boards 10^%.2f, nodes per board 10^%.2f' % (d0, boards / LN10, (nodes - boards) / LN10))
    res['oracle_free'] = orc

    # what a perfect bound from depth d0 would remove, and what the parity bound removes
    print('-- bounds on the Blackwood-list tree (K=10) --')
    lv, s = sched['Blackwood list']
    tot = [logsumexp(list(l.values())) for l in lv]
    total = logsumexp(tot[1:])
    oracle = {}
    for d0 in (170, 201, 216, 229, 240):
        kept = logsumexp(tot[1:d0] + [tot[-1] + log(N2 - d0 + 1)])
        oracle[d0] = (total - kept) / LN10
        print('perfect bound from depth %d: nodes divided by 10^%.2f' % (d0, oracle[d0]))
    res['oracle_gain_log10'] = oracle
    law = odd_colour_law(17, 12)
    K = 10
    kept_terms = []
    by_depth_kept = []
    for d in range(1, N2 + 1):
        terms = []
        for j, v in lv[d].items():
            surv = sum(p for o, p in law[j].items() if o <= 2 * (K - j))
            if surv > 0:
                terms.append(v + log(surv))
        by_depth_kept.append(logsumexp(terms))
    kept = logsumexp(by_depth_kept)
    print('parity bound (odd colours <= 2 x remaining budget), interior breaks only: nodes divided by 10^%.2f' % ((total - kept) / LN10))
    late_all = logsumexp(tot[201:])
    late_kept = logsumexp(by_depth_kept[200:])
    print('   same bound, nodes at depth >= 201 only: divided by 10^%.2f' % ((late_all - late_kept) / LN10))
    res['parity_gain_late_log10'] = (late_all - late_kept) / LN10
    for d in (215, 225, 235, 245):
        print('   depth %d: share of nodes kept by the parity bound %.3g' % (d, exp(by_depth_kept[d - 1] - tot[d])))
    res['parity_gain_log10'] = (total - kept) / LN10
    # where the Blackwood tree lives
    print('Blackwood-list tree width log10 at depths 170,190,200,205,210,215,220,225,230,235,240,245,250,256:',
          ' '.join('%.1f' % (tot[d] / LN10) for d in (170, 190, 200, 205, 210, 215, 220, 225, 230, 235, 240, 245, 250, 256)))
    res['blackwood_by_depth_log10'] = [t / LN10 for t in tot]
    res['schedules'] = {k: {kk: vv for kk, vv in v[1].items() if kk != 'by_depth_log10'} for k, v in sched.items()}

    # ---------- small generated boards: threshold where E[N(k)] crosses 1 ----------
    print('\n== small framed boards (balanced painter), corner pinned: first k with E[N(<=k)] >= 1 ==')
    small = []
    for n, total_colours in ((5, 13), (5, 9), (6, 14), (6, 10), (7, 15), (6, 10 + 0), (7, 11), (8, 13)):
        ci = total_colours - 5
        b = Board(n, balanced(4 * (n - 1), 5), balanced(2 * (n - 1) * (n - 2), ci), pinned_corner=True)
        cum, kstar, vals = [], None, []
        for k in range(0, 16):
            cum.append(b.log_n_exact(k))
            c = logsumexp(cum)
            vals.append(c / LN10)
            if kstar is None and c >= 0:
                kstar = k
        small.append({'n': n, 'colours': total_colours, 'interior_colours': ci, 'k_star': kstar, 'log10_cumN': vals})
        print('%dx%d C=%2d (interior %2d): k* = %s ; log10 E[N(<=k)] k=0..10: %s' % (
            n, n, total_colours, ci, kstar, ' '.join('%.1f' % v for v in vals[:11])))
    res['small_boards'] = small

    with open(os.path.join(here, 'model_results.json'), 'w') as fh:
        json.dump(res, fh, indent=1)


if __name__ == '__main__':
    main()
