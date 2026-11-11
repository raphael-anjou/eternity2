#!/usr/bin/env python3
"""Stage 1 (PREREG.md): can A3's backdoor cells be recognised at the root?

Reads results/stage1/features.jsonl (bb_features), trains the preregistered
logistic regression on seeds 1..15, evaluates within-instance AUC on seeds
16..30 and writes results/stage1/{model.json,summary.json}. Standard library.
"""
import json
import math
import random
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "results" / "stage1"
BOOT = 10_000
BOOT_SEED = 20261111
L2, STEPS, LR = 1e-3, 3000, 0.5


def zscore(rows):
    k = len(rows)
    out = [list(r) for r in rows]
    for j in range(len(rows[0])):
        mean = sum(r[j] for r in rows) / k
        sd = math.sqrt(sum((r[j] - mean) ** 2 for r in rows) / k)
        for r in out:
            r[j] = (r[j] - mean) / sd if sd > 1e-12 else 0.0
    return out


def auc(scores, labels):
    pos = [s for s, y in zip(scores, labels) if y]
    neg = [s for s, y in zip(scores, labels) if not y]
    if not pos or not neg:
        return None
    w = 0.0
    for p in pos:
        for q in neg:
            w += 1.0 if p > q else 0.5 if p == q else 0.0
    return w / (len(pos) * len(neg))


def boot_mean_ci(vals, seed=BOOT_SEED):
    rng = random.Random(seed)
    n = len(vals)
    means = sorted(sum(vals[rng.randrange(n)] for _ in range(n)) / n for _ in range(BOOT))
    return [means[int(0.025 * BOOT)], means[int(0.975 * BOOT) - 1]]


def fit(xs, ys):
    d = len(xs[0])
    w = [0.0] * d
    b = 0.0
    n = len(xs)
    for _ in range(STEPS):
        gw = [0.0] * d
        gb = 0.0
        for x, y in zip(xs, ys):
            z = b + sum(wi * xi for wi, xi in zip(w, x))
            p = 1.0 / (1.0 + math.exp(-z)) if z >= 0 else math.exp(z) / (1.0 + math.exp(z))
            e = p - y
            gb += e
            for j in range(d):
                gw[j] += e * x[j]
        for j in range(d):
            w[j] -= LR * (gw[j] / n + 2 * L2 * w[j])
        b -= LR * gb / n
    return w, b


def main():
    rows = [json.loads(l) for l in (OUT / "features.jsonl").read_text().splitlines() if l.strip()]
    names = rows[0]["features"]
    jdom = names.index("ln_dom")
    inst = []
    for r in rows:
        f = [c["f"] for c in r["cells"]]
        inst.append({
            "regime": r["regime"], "n": r["n"], "seed": r["seed"], "k_source": r["k_source"],
            "raw": f, "z": zscore(f), "y": [1 if c["label"] else 0 for c in r["cells"]],
            "cell": [c["cell"] for c in r["cells"]],
        })
    train = [i for i in inst if i["seed"] <= 15]
    test = [i for i in inst if i["seed"] >= 16]

    xs = [x for i in train for x in i["z"]]
    ys = [y for i in train for y in i["y"]]
    w, b = fit(xs, ys)

    def score_learned(i):
        return [sum(a * c for a, c in zip(w, z)) for z in i["z"]]

    def aucs(insts, score):
        out = []
        for i in insts:
            a = auc(score(i), i["y"])
            if a is not None:
                out.append(a)
        return out

    def failfirst(i):
        return [-r[jdom] for r in i["raw"]]

    def feat(j, sign=1.0):
        return lambda i: [sign * r[j] for r in i["raw"]]

    def index_control(i):
        return [-c for c in i["cell"]]

    def mean(v):
        return sum(v) / len(v)

    # single features: train AUC, sign chosen on training
    singles = {}
    for j, nm in enumerate(names):
        tr = mean(aucs(train, feat(j)))
        sign = 1.0 if tr >= 0.5 else -1.0
        te = aucs(test, feat(j, sign))
        singles[nm] = {"train_auc_raw": tr, "sign": sign, "train_auc_signed": max(tr, 1 - tr),
                       "test_auc_signed": mean(te), "test_ci": boot_mean_ci(te)}
    best_single = max((nm for nm in names if nm != "ln_dom"), key=lambda nm: (singles[nm]["train_auc_signed"], nm))

    # kill test K1: paired per test instance
    pairs = []
    for i in test:
        a_l = auc(score_learned(i), i["y"])
        a_f = auc(failfirst(i), i["y"])
        if a_l is not None and a_f is not None:
            pairs.append((a_l, a_f, i))
    deltas = [a - f for a, f, _ in pairs]
    delta_ci = boot_mean_ci(deltas)
    learned_te = [a for a, _, _ in pairs]
    ff_te = [f for _, f, _ in pairs]
    ff_signed_sign = singles["ln_dom"]["sign"]
    strict = [auc(score_learned(i), i["y"]) - auc(feat(jdom, ff_signed_sign)(i), i["y"]) for _, _, i in pairs]

    def split(key):
        out = {}
        for v in sorted({p[2][key] for p in pairs}, key=str):
            sel = [p for p in pairs if p[2][key] == v]
            d = [a - f for a, f, _ in sel]
            out[str(v)] = {"instances": len(sel), "auc_learned": mean([a for a, _, _ in sel]),
                           "auc_failfirst": mean([f for _, f, _ in sel]), "delta": mean(d),
                           "delta_ci": boot_mean_ci(d)}
        return out

    k1 = delta_ci[0] <= 0
    summary = {
        "features": names,
        "train_instances": len(train), "test_instances": len(test), "test_pairs": len(pairs),
        "train_cells": len(xs), "train_positive_rate": sum(ys) / len(ys),
        "model": {"weights": w, "bias": b},
        "test_auc_learned": mean(learned_te), "test_auc_learned_ci": boot_mean_ci(learned_te),
        "test_auc_failfirst": mean(ff_te), "test_auc_failfirst_ci": boot_mean_ci(ff_te),
        "delta": mean(deltas), "delta_ci": delta_ci,
        "delta_vs_signed_domain": mean(strict), "delta_vs_signed_domain_ci": boot_mean_ci(strict),
        "train_auc_learned": mean(aucs(train, score_learned)),
        "single_features": singles, "best_single": best_single,
        "index_control_test_auc": mean(aucs(test, index_control)),
        "index_control_test_ci": boot_mean_ci(aucs(test, index_control)),
        "by_k_source": split("k_source"), "by_regime": split("regime"), "by_n": split("n"),
        "K1_fires": k1,
    }
    (OUT / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    model = {"features": names, "weights": w, "bias": b,
             "single": {"feature": best_single, "sign": singles[best_single]["sign"]}}
    (OUT / "model.json").write_text(json.dumps(model, indent=2) + "\n")
    print(f"learned AUC {summary['test_auc_learned']:.4f}  failfirst {summary['test_auc_failfirst']:.4f}  "
          f"delta {summary['delta']:.4f} CI [{delta_ci[0]:.4f}, {delta_ci[1]:.4f}]  K1 fires: {k1}")
    print(f"best single: {best_single} sign {singles[best_single]['sign']:+.0f}")
    for nm, s in singles.items():
        print(f"  {nm:10s} train {s['train_auc_signed']:.4f} (sign {s['sign']:+.0f})  test {s['test_auc_signed']:.4f} "
              f"[{s['test_ci'][0]:.4f}, {s['test_ci'][1]:.4f}]")
    print("  index ctl test", round(summary["index_control_test_auc"], 4))
    print("weights", [round(x, 3) for x in w])


if __name__ == "__main__":
    main()
