"""Gap 3: read g3tree outputs. python3 -I g3_analyze.py <out files...>

Per file and depth d (free cells placed), from the population members (uniform over the
level up to the sampler's noise) and, on planted boards, the planted prefix:

  mean S        mean subtree size, sample and from the width profile (sum of deeper widths / width)
  med, zero     median subtree size, share of members with an empty subtree
  top1%         share of the level's work held by the largest 1% of subtrees
  typ gain      log2( mean S / (2 mean min(S, median S)) ): what "uniform draw, search below,
                restart" would gain IF the solution sat under a median node
  S*/mean       planted subtree over the mean subtree
  rank          share of nodes with S <= S*
  Q             share of work in subtrees with S <= S* (sample)
  gU            log2( mean S / (2 mean min(S, S*)) ): gain of the same scheme on this board
  gCut          log2( mean S / (2 S*) ): same with a fixed budget S* per attempt
  band          share of work in subtrees with S*/2 <= S <= 2 S*; an oracle that knew every
                subtree's size and S* would gain log2(1 / (2 band)) bits
  code          log2 W* - log2 N_d: extra bits a uniform-child probe needs to land on the
                planted prefix, against a uniform draw from the level
  codeU         mean over members of log2 W - log2 N_d (the same for a uniform node)
"""
import math
import sys
from collections import defaultdict


def read(fn):
    us, ps, lv, ml, ps_, mu = defaultdict(list), {}, {0: 0.0}, {}, {}, {}
    for l in open(fn):
        t = l.split()
        if not t:
            continue
        if t[0] == "US":
            us[int(t[1])].append((float(t[2]), int(t[3]), int(t[4])))
        elif t[0] == "PS":
            ps[int(t[1])] = (float(t[2]), int(t[3]), int(t[4]), int(t[5]))
        elif t[0] == "LV":
            lv[int(t[1]) + 1] = float(t[7])
            ml[int(t[1]) + 1] = float(t[8])
            ps_[int(t[1]) + 1] = float(t[9])
        elif t[0] == "MU":
            mu[int(t[1])] = (float(t[2]), float(t[3]))
    return us, ps, lv, ml, ps_, mu


def main():
    B = math.log2(10)
    rows = defaultdict(list)
    for fn in sys.argv[1:]:
        us, ps, lv, ml, psv, mu = read(fn)
        ks = sorted(k for k in lv if lv[k] > -1e300)
        tot = math.log10(sum(10 ** lv[k] for k in ks))
        name = fn.rsplit("/", 1)[-1].replace(".out", "")
        print("%s: tree 10^%.3f = 2^%.1f, widest level %d, last sampled level %d" % (name, tot, tot * B, max(ks, key=lambda k: lv[k]), ks[-1]))
        if len(sys.argv) <= 4:
            # uniform-child probes against a uniform draw from the level: codeU = E_U log2 W - log2 N,
            # log2 Ps = log2 of the probe's survival probability (population estimate; direct where
            # the probes still survive), KL = codeU + log2 Ps = divergence of the surviving probe's
            # law from uniform, the bits a probe-and-restart scheme loses when subtree size and
            # weight are independent
            for d in (16, 32, 40, 48, 64, 80, 96, 112, 128, 144, 158):
                if d in ml and d in ks:
                    cu = ml[d] - lv[d] * B
                    direct = ("%.2f" % math.log2(mu[d][0])) if d in mu and mu[d][0] > 0 else "n/a"
                    print("  probes d=%3d: log10 N %.3f codeU %.2f log2 Ps %.2f (direct %s) KL(U||probe) %.2f bits%s"
                          % (d, lv[d], cu, psv[d], direct, cu + psv[d], ("  mean probe length %.2f" % mu[d][1]) if d in mu else ""))
        for d in sorted(us):
            S = sorted(x[1] for x in us[d])
            n = len(S)
            ncap = sum(x[2] for x in us[d])
            mean = sum(S) / n
            prof = sum(10 ** (lv[k] - lv[d]) for k in ks if k > d)
            med = S[n // 2]
            top = sum(S[n - n // 100:]) / sum(S)
            typ = math.log2(mean / (2 * max(1e-9, sum(min(x, med) for x in S) / n))) if med > 0 else float("inf")
            line = "  d=%3d n=%5d cap=%d | mean S %.4g (profile %.4g) med %d zero %.3f top1%% %.3f typ gain %5.1f" % (d, n, ncap, mean, prof, med, S.count(0) / n, top, typ)
            codeU = ml.get(d, float("nan")) - lv[d] * B
            if d in ps:
                l2w, s, capped, nsol = ps[d]
                rank = sum(1 for x in S if x <= s) / n
                q = sum(x for x in S if x <= s) / sum(S)
                mm = sum(min(x, s) for x in S) / n
                gU = math.log2(mean / (2 * mm))
                gUp = math.log2(prof / (2 * mm))
                gCut = math.log2(prof / (2 * s))
                band = sum(x for x in S if s / 2 <= x <= 2 * s) / sum(S)
                code = l2w - lv[d] * B
                line += " | S* %.4g%s S*/mean %.1f rank %.4f Q %.3f gU %.2f (profile mean %.2f) gCut %.2f band %.3f code %.1f codeU %.1f" % (s, "+" if capped else "", s / prof, rank, q, gU, gUp, gCut, band, code, codeU)
                rows[d].append((s / prof, rank, q, gU, gUp, gCut, code, codeU, typ, top, tot, band, capped))
            else:
                line += " | codeU %.1f" % codeU
                # gain of the scheme as a function of where the solution's subtree ranks in the work
                tw = sum(S)
                acc = 0.0
                out = []
                targets = [0.01, 0.1, 0.5, 0.9]
                for x in S:
                    acc += x
                    while targets and acc / tw >= targets[0]:
                        mm = sum(min(y, x) for y in S) / n
                        out.append("Q=%.2f: S*=%d gU=%.2f" % (targets[0], x, math.log2(mean / (2 * mm))))
                        targets.pop(0)
                line += " | if the planted subtree sat at work share " + "; ".join(out)
            print(line)
    if rows:
        print("\nsummary over planted boards (median [min, max])")
        names = ["S*/mean", "rank", "Q", "gU", "gU(profile mean)", "gCut", "code", "codeU", "typ gain", "top1%", "log10 tree", "band", "capped"]
        for d in sorted(rows):
            v = rows[d]
            parts = []
            for i, nm in enumerate(names):
                xs = sorted(r[i] for r in v)
                parts.append("%s %.3g [%.3g, %.3g]" % (nm, xs[len(xs) // 2], xs[0], xs[-1]))
            print("  d=%3d boards=%d | %s" % (d, len(v), " | ".join(parts)))
            qs = sorted(r[2] for r in v)
            print("         Q sorted: " + " ".join("%.2f" % q for q in qs))


main()
