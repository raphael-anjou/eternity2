#!/usr/bin/env python3
"""How loose is a bound that drops piece distinctness? Small planted boards. Stdlib only.

Run:  python3 -I gap_check.py        (about two to three minutes, one core)

For framed planted boards (5 ring colours, c interior colours, balanced painter in
the style of engine/src/generator.rs, NOT byte-identical to it), sample perfect
prefixes with a randomised strict depth-first search, stop when `rows` rows are
left, and compute for the tail:

  true   = fewest broken joints of any completion with the real remaining pieces
           (each used once), by iterative deepening;
  relax  = the same when a remaining piece may be reused any number of times
           (frontier colours and geometry exact). This is the limit W -> infinity
           of a decision diagram that keeps the frontier and drops distinctness;
  assign = fewest breaks against the fixed frontier when the next n cells receive
           DISTINCT remaining pieces and every joint between two empty cells is
           ignored (assignment relaxation: keeps distinctness, drops geometry).

gap = true - relax. Also prints the first-moment prediction of true and relax.
"""
import importlib.util
import math
import os
import random
import sys
import time

here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location('model', os.path.join(here, 'model.py'))
model = importlib.util.module_from_spec(spec)
spec.loader.exec_module(model)


def make_board(n, ci, rng, ring_colours=5):
    """Planted framed board: pieces[cell] = (top, right, bottom, left), 0 on the rim."""
    hj, vj = {}, {}
    ring_slots, int_slots = [], []
    for y in range(n):
        for x in range(n - 1):
            (ring_slots if y in (0, n - 1) else int_slots).append(('h', x, y))
    for y in range(n - 1):
        for x in range(n):
            (ring_slots if x in (0, n - 1) else int_slots).append(('v', x, y))
    rc = [(k % ring_colours) + 1 for k in range(len(ring_slots))]
    ic = [ring_colours + (k % ci) + 1 for k in range(len(int_slots))]
    rng.shuffle(rc)
    rng.shuffle(ic)
    col = dict(zip(ring_slots, rc))
    col.update(zip(int_slots, ic))
    pieces = []
    for y in range(n):
        for x in range(n):
            top = col[('v', x, y - 1)] if y > 0 else 0
            bottom = col[('v', x, y)] if y < n - 1 else 0
            left = col[('h', x - 1, y)] if x > 0 else 0
            right = col[('h', x, y)] if x < n - 1 else 0
            pieces.append((top, right, bottom, left))
    return pieces


def fits(n, pieces):
    """Per cell: list of (pid, rotated tuple) whose grey sides are exactly the rim sides."""
    out = []
    for y in range(n):
        for x in range(n):
            rim = (y == 0, x == n - 1, y == n - 1, x == 0)
            cand = []
            for pid, p in enumerate(pieces):
                seen = set()
                for r in range(4):
                    q = p[r:] + p[:r]
                    if q in seen:
                        continue
                    if all((q[s] == 0) == rim[s] for s in range(4)):
                        seen.add(q)
                        cand.append((pid, q))
            out.append(cand)
    return out


def strict_prefix(n, cand, depth, rng, cap=400000):
    """First perfect partial board of `depth` cells met by a randomised strict DFS."""
    board = [None] * (n * n)
    used = set()
    nodes = [0]

    def rec(d):
        if d == depth:
            return True
        x, y = d % n, d // n
        order = cand[d][:]
        rng.shuffle(order)
        for pid, q in order:
            if pid in used:
                continue
            if x > 0 and board[d - 1][1] != q[3]:
                continue
            if y > 0 and board[d - n][2] != q[0]:
                continue
            nodes[0] += 1
            if nodes[0] > cap:
                return False
            used.add(pid)
            board[d] = q
            if rec(d + 1):
                return True
            used.discard(pid)
        board[d] = None
        return False

    return (board, used) if rec(0) else None


class Cap(Exception):
    pass


class Tail:
    """Completion of cells depth.. of a board whose first `depth` cells are fixed."""

    def __init__(self, n, cand, board, used, depth):
        self.n, self.N, self.depth = n, n * n, depth
        self.pool = [[(pid, q) for pid, q in cand[d] if pid not in used] for d in range(n * n)]
        self.board = board[:]
        self.fail = {}      # reuse search: frontier state -> largest budget proved insufficient
        self.nodes = 0
        self.cap = 0

    def _costs(self, d, q):
        n, board = self.n, self.board
        c = 0
        if d % n > 0 and board[d - 1][1] != q[3]:
            c += 1
        if d >= n and board[d - n][2] != q[0]:
            c += 1
        return c

    def reuse_ok(self, d, b):
        """Can cells d.. be completed with at most b breaks when pieces may be reused?
        Uses only the frontier of self.board, so it is valid at any node of the distinct search."""
        if d == self.N:
            return True
        n, board = self.n, self.board
        key = (d, tuple(t[2] for t in board[d - n:d]), board[d - 1][1] if d % n else 0)
        if self.fail.get(key, -1) >= b:
            return False
        saved = board[d]
        ok = False
        for want in range(0, min(b, 2) + 1):
            for pid, q in self.pool[d]:
                if self._costs(d, q) != want:
                    continue
                self.nodes_reuse += 1
                if self.nodes_reuse > 3000000:
                    board[d] = saved
                    raise Cap()
                board[d] = q
                if self.reuse_ok(d + 1, b - want):
                    ok = True
                    break
            if ok:
                break
        board[d] = saved
        if not ok:
            self.fail[key] = max(self.fail.get(key, -1), b)
        return ok

    def reuse_min(self, bmax=60):
        self.nodes_reuse = 0
        for b in range(bmax):
            if self.reuse_ok(self.depth, b):
                return b
        return bmax

    def distinct_min(self, cap, use_bound, bmax=60):
        """Fewest breaks with each remaining piece used once. Iterative deepening.
        use_bound: prune a node when the reuse relaxation of its subtree exceeds the budget left.
        Returns (value, nodes, censored). If censored, the true value is >= value."""
        self.nodes_reuse = 0
        nodes = [0]
        taken = set()
        board, n, N = self.board, self.n, self.N

        def rec(d, b):
            if d == N:
                return True
            for want in range(0, min(b, 2) + 1):
                for pid, q in self.pool[d]:
                    if pid in taken or self._costs(d, q) != want:
                        continue
                    nodes[0] += 1
                    if nodes[0] > cap:
                        raise Cap()
                    board[d] = q
                    if use_bound and not self.reuse_ok(d + 1, b - want):
                        continue
                    taken.add(pid)
                    ok = rec(d + 1, b - want)
                    taken.discard(pid)
                    if ok:
                        return True
            board[d] = None
            return False

        start = self.reuse_min() if use_bound else 0
        for b in range(start, bmax):
            try:
                if rec(self.depth, b):
                    return b, nodes[0], False
            except Cap:
                board[self.depth:] = [None] * (N - self.depth)
                return b, nodes[0], True
        return bmax, nodes[0], True


def assign_bound(n, cand, board, used, depth):
    """Min-cost assignment of distinct unused pieces to the next n cells, cost = breaks
    against already placed neighbours only (branch and bound; tiny)."""
    N = n * n
    cells = list(range(depth, min(N, depth + n)))
    costs = []
    for d in cells:
        x, y = d % n, d // n
        best = {}
        for pid, q in cand[d]:
            if pid in used:
                continue
            c = 0
            if d == depth and x > 0 and board[d - 1][1] != q[3]:
                c += 1
            if y > 0 and board[d - n] is not None and board[d - n][2] != q[0]:
                c += 1
            if c < best.get(pid, 9):
                best[pid] = c
        costs.append(sorted(best.items(), key=lambda kv: kv[1]))
    best_total = [10 ** 9]
    taken = set()

    def rec(i, acc):
        if acc >= best_total[0]:
            return
        if i == len(cells):
            best_total[0] = acc
            return
        for pid, c in costs[i]:
            if pid in taken:
                continue
            if acc + c >= best_total[0]:
                break
            taken.add(pid)
            rec(i + 1, acc + c)
            taken.discard(pid)

    rec(0, 0)
    return best_total[0]


def run(n, ci, rows, n_prefix, seed, cap):
    rng = random.Random(seed)
    depth = n * n - rows * n
    out = []
    t0 = time.time()
    tries = 0
    while len(out) < n_prefix and tries < 6 * n_prefix:
        tries += 1
        pieces = make_board(n, ci, rng)
        cand = fits(n, pieces)
        sp = strict_prefix(n, cand, depth, rng)
        if sp is None:
            continue
        board, used = sp
        t = Tail(n, cand, board, used, depth)
        try:
            rel = t.reuse_min()
            tru, nodes_b, cens = t.distinct_min(cap, use_bound=True)
        except Cap:
            continue  # reuse search itself too large: prefix dropped, counted in `tries`
        nodes_plain = None
        if not cens:
            t2 = Tail(n, cand, board, used, depth)
            v2, nodes_plain, cens2 = t2.distinct_min(cap, use_bound=False)
            if cens2:
                nodes_plain = None
            else:
                assert v2 == tru
        asg = assign_bound(n, cand, board, used, depth)
        out.append({'true': tru, 'reuse': rel, 'assign': asg, 'censored': cens, 'nodes_bound': nodes_b, 'nodes_plain': nodes_plain})
    fm_d, fm_u, ei, er = model.tail_first_moment(
        model.Board(n, model.balanced(4 * (n - 1), 5), model.balanced(2 * (n - 1) * (n - 2), ci)), depth, ci, 5)
    closed = [r for r in out if not r['censored']]

    def mean(xs):
        return sum(xs) / len(xs) if xs else float('nan')

    def med(xs):
        xs = sorted(xs)
        return xs[len(xs) // 2] if xs else float('nan')

    gaps = [r['true'] - r['reuse'] for r in closed]
    both = [r for r in closed if r['nodes_plain']]
    ratio = mean([math.log10(r['nodes_plain'] / max(1, r['nodes_bound'])) for r in both])
    print('%dx%d c=%d rows left %d (%2d cells): prefixes %d, closed %d | true mean %.2f median %s | reuse mean %.2f | assign mean %.2f | gap mean %.2f median %s min %s max %s | first moment true %s reuse %s | log10 node ratio plain/bounded %.2f (n=%d) | %.0fs' % (
        n, n, ci, rows, rows * n, len(out), len(closed), mean([r['true'] for r in closed]), med([r['true'] for r in closed]),
        mean([r['reuse'] for r in closed]), mean([r['assign'] for r in closed]), mean(gaps), med(gaps),
        min(gaps) if gaps else '-', max(gaps) if gaps else '-',
        model.first_at_least_one(fm_d), model.first_at_least_one(fm_u), ratio, len(both), time.time() - t0))
    cens = [(r['true'], r['reuse']) for r in out if r['censored']]
    if cens:
        print('    censored (true >= first number, reuse): %s' % cens)
    sys.stdout.flush()
    return out


if __name__ == '__main__':
    quick = len(sys.argv) > 1 and sys.argv[1] == 'quick'
    k = 3 if quick else 8
    cap = 150000 if quick else 200000
    import json
    res = {}
    for n, ci, rows, kk in ((6, 5, 1, k), (6, 5, 2, k), (6, 5, 3, max(3, 3 * k // 4)), (7, 6, 1, max(3, k // 2)), (7, 6, 2, max(3, 5 * k // 8))):
        res['%dx%d_c%d_rows%d' % (n, n, ci, rows)] = run(n, ci, rows, kk, 1000 * n + rows, cap)
    if not quick:
        with open(os.path.join(here, 'gap_results.json'), 'w') as fh:
            json.dump(res, fh, indent=1)
