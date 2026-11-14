"""Gap 3: the best restart cutoff of depth-first search with random value order, on full-size
planted TOY boards, from the subtree size of every sibling of the planted path
(g3tree sibling mode, lines "SB j size"). python3 -I g3_cutoff.py <sib files...>

Time to the solution under a random value order: T = D + sum over siblings that precede the
planted child of (1 + size). Per depth the planted child's position among its b children is
uniform and the siblings before it are a uniform subset.

Exact for every tree: E[T] = D + (tree - D) / 2.
A run cut at t nodes succeeds with q(t) = P(T <= t); restarting costs l(t) = E[min(T, t)] / q(t)
(Luby, Sinclair, Zuckerman). Gain in bits = log2(E[T] / l(t)).

q(t) is far below what sampling can see for small t, so two closed forms are used:
  upper  q(t) <= prod_j 1 / (1 + number of siblings at depth j larger than t)
         and l(t) >= t (1 - q) / q                 -> an upper bound on the gain
  lower  with t' = D + sum of all siblings of size <= t, a run cut at t' succeeds whenever no
         larger sibling precedes the planted child  -> a gain that is achieved
Sibling sizes at shallow depth are SMC estimates (unbiased, noisy); the bounds use them as given.
"""
import math
import random
import sys
from collections import defaultdict


def main():
    summary = []
    for fn in sys.argv[1:]:
        sib = defaultdict(list)
        for l in open(fn):
            t = l.split()
            if t and t[0] == "SB":
                sib[int(t[1])].append(1.0 + float(t[2]))
        if not sib:
            continue
        D = 251
        tree = D + sum(sum(v) for v in sib.values())
        ET = D + (tree - D) / 2
        sizes = sorted(x for v in sib.values() for x in v)
        best_ub, best_lb = (-99.0, 0, 0), (-99.0, 0, 0)
        curve, curve_ub = {}, {}
        prefix = 0.0
        for idx, s in enumerate(sizes):
            prefix += s
            if idx + 1 < len(sizes) and sizes[idx + 1] == s:
                continue
            # cutoff at which every sibling of size <= s fits and every larger one must be avoided
            lq = -sum(math.log2(1 + sum(1 for x in v if x > s)) for v in sib.values())
            q = 2.0 ** lq
            t_ub = max(s, D)
            if q >= 1:
                continue                      # nothing to avoid: no restart, no gain
            ub = math.log2(ET / t_ub) + lq - math.log2(1 - q)
            t_lb = D + prefix
            lb = math.log2(ET / t_lb) + lq
            if ub > best_ub[0]:
                best_ub = (ub, t_ub, lq)
            if lb > best_lb[0]:
                best_lb = (lb, t_lb, lq)
            for g in (2, 5, 10, 20, 40, 60, 85):
                if t_lb <= tree / 2.0 ** g and (g not in curve or lb > curve[g][0]):
                    curve[g] = (lb, lq)
                if t_ub <= tree / 2.0 ** g and (g not in curve_ub or ub > curve_ub[g]):
                    curve_ub[g] = ub
        # sampled distribution of T / tree and the sampled best cutoff
        rng = random.Random(7)
        T = []
        for _ in range(20000):
            x = D
            for v in sib.values():
                k = rng.randrange(len(v) + 1)
                if k:
                    x += sum(rng.sample(v, k))
            T.append(x)
        T.sort()
        pre = 0.0
        best = None
        for j, tt in enumerate(T):
            pre += tt
            l = (pre + (len(T) - j - 1) * tt) / (j + 1)
            if j >= 19 and (best is None or l < best[0]):      # at least 20 successes of 20,000
                best = (l, tt, (j + 1) / len(T))
        gs = math.log2(ET / best[0])
        nsib = len(sizes)
        dead = sum(1 for x in sizes if x < 1000)
        name = fn.rsplit("/", 1)[-1].replace(".out", "")
        print("%s: tree 2^%.1f, %d siblings (%d below 1,000 nodes) | sampled best cutoff (q >= 0.001): gain %.2f bits at t/tree %.3g, q %.4f | T/tree quantiles 1%% %.3g 10%% %.3g 50%% %.3g"
              % (name, math.log2(tree), nsib, dead, gs, best[1] / tree, best[2], T[200] / tree, T[2000] / tree, T[10000] / tree))
        print("    any cutoff: achieved gain at best %.2f bits (t/tree 2^%.1f, q 2^%.1f) ; upper bound %.2f bits (t/tree 2^%.1f, q 2^%.1f)"
              % (best_lb[0], math.log2(best_lb[1] / tree), best_lb[2], best_ub[0], math.log2(best_ub[1] / tree), best_ub[2]))
        print("    achieved gain with the cutoff at most tree/2^g: " + " ; ".join("g=%d: %.1f (q 2^%.1f)" % (g, curve[g][0], curve[g][1]) for g in sorted(curve)))
        print("    upper bound with the cutoff at most tree/2^g: " + " ; ".join("g=%d: %.1f" % (g, curve_ub[g]) for g in sorted(curve_ub)))
        summary.append((gs, best_lb[0], best_ub[0], curve.get(10, (float("nan"),))[0], curve.get(40, (float("nan"),))[0], curve_ub.get(10, float("nan")), curve_ub.get(40, float("nan")),
                        curve.get(85, (float("nan"),))[0], curve_ub.get(85, float("nan"))))
    if summary:
        print("\n%d board-directions (median [min, max])" % len(summary))
        for i, nm in enumerate(["sampled best cutoff gain", "achieved gain, any cutoff", "upper bound, any cutoff", "best achieved, cutoff <= tree/2^10", "best achieved, cutoff <= tree/2^40", "upper bound, cutoff <= tree/2^10", "upper bound, cutoff <= tree/2^40", "best achieved, cutoff <= tree/2^85", "upper bound, cutoff <= tree/2^85"]):
            xs = sorted(r[i] for r in summary if r[i] == r[i])
            if xs:
                print("  %-36s %.2f [%.2f, %.2f]" % (nm, xs[len(xs) // 2], xs[0], xs[-1]))
        n = len(summary)
        for g in (1, 2, 3, 4, 5):
            print("  achieved gain (any cutoff) >= %d bits: %d of %d (%.2f; 2^-g = %.3f) ; upper bound >= %d bits: %d of %d"
                  % (g, sum(1 for r in summary if r[1] >= g), n, sum(1 for r in summary if r[1] >= g) / n, 2.0 ** -g, g, sum(1 for r in summary if r[2] >= g), n))


main()
