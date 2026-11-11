#!/usr/bin/env python3
"""Markdown tables for README.md from results/summary.json and Stage 1."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
s = json.loads((ROOT / "results" / "summary.json").read_text())


def f(x, d=0):
    if x is None:
        return "> cap"
    return f"{x:,.{d}f}"


def ci(c):
    return "[" + ", ".join("+inf" if v is None else f"{v:.3f}" for v in c) + "]"


for rung, R in s["rungs"].items():
    if R.get("incomplete"):
        print(f"\n### {rung}: incomplete, missing {R['missing_solvers']}")
        continue
    print(f"\n### {rung} ({R['n']}x{R['n']}, ladder {R['ladder']}, seeds {R['seeds'][0]}..{R['seeds'][-1]})\n")
    print("| solver | solved | Wilson 95% | KM q1 | KM median | KM q3 | median wall ms | total wall s | propagations | time-censored |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    for name, st in R["solvers"].items():
        w = st["wilson95"]
        print(f"| `{name}` | {st['solved']}/{st['runs']} | [{w[0]:.2f}, {w[1]:.2f}] | {f(st['km_q1'])} | {f(st['km_median'])} | "
              f"{f(st['km_q3'])} | {st['median_wall_ms']:,.0f} | {st['total_wall_s']:,.1f} | {st['propagation_calls']:,} | {st['time_censored']} |")
    print("\n| variant | baseline | median ratio | 95% CI median log-ratio | p (boot) | Holm | label |")
    print("|---|---|---|---|---|---|---|")
    for b, cmp in R["g1"].items():
        for v, c in cmp.items():
            r = "+inf" if c["median_ratio"] is None else f"{c['median_ratio']:.3g}"
            print(f"| `{v}` | `{b}` | {r} | {ci(c['ci95'])} | {c['p_boot']:.4f} | {'reject' if c['holm_reject'] else 'no'} | {c['label']} |")
    print(f"\nLabel vs strongest dynamic baseline: " + ", ".join(f"`{v}` {l}" for v, l in R["label_vs_strongest"].items()))
print("\nKill:", s["kill"])
