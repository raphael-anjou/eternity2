#!/usr/bin/env python3
"""Diff two sweep run directories — the Python twin of `e2kit compare`.

    python3 scripts/compare.py runs/<A> runs/<B>

Reads each run's results.jsonl, pairs cells by seed, and reports the paired
score delta with its spread. Same verdict logic as the Rust CLI: a mean shift
smaller than twice the standard error is not distinguishable from noise on this
puzzle, so it says so rather than letting you over-read a one-point difference.

Pure standard library; no dependencies.
"""
import json
import math
import sys
from pathlib import Path


def read_results(run_dir: str):
    path = Path(run_dir) / "results.jsonl"
    cells = {}
    with open(path) as fh:
        for line in fh:
            line = line.strip()
            if not line:
                continue
            row = json.loads(line)
            cells[row["seed"]] = row
    return cells


def is_achieved(row):
    """A cell whose score is an achieved board score (not a bound/exhaustion).
    Rows written before the outcome field existed default to achieved."""
    outcome = row.get("outcome")
    if outcome is None:
        return True
    return outcome.get("type") in ("complete", "improved")


def mean(xs):
    return sum(xs) / len(xs) if xs else 0.0


def sd(xs):
    if not xs:
        return 0.0
    m = mean(xs)
    return math.sqrt(sum((x - m) ** 2 for x in xs) / len(xs))


def main():
    if len(sys.argv) != 3:
        print("usage: compare.py <runA> <runB>", file=sys.stderr)
        sys.exit(2)
    a_dir, b_dir = sys.argv[1], sys.argv[2]
    a, b = read_results(a_dir), read_results(b_dir)

    # Only pair seeds where both sides achieved a score; a bound or an exhaustion
    # result is not a score and must not be averaged as one.
    shared_all = set(a) & set(b)
    shared = sorted(s for s in shared_all if is_achieved(a[s]) and is_achieved(b[s]))
    skipped = len(shared_all) - len(shared)
    if not shared:
        print("no shared seeds with an achieved score on both sides — nothing comparable.",
              file=sys.stderr)
        sys.exit(1)

    deltas = [b[s]["score"] - a[s]["score"] for s in shared]
    n = len(shared)
    mean_a = mean([c["score"] for c in a.values() if is_achieved(c)])
    mean_b = mean([c["score"] for c in b.values() if is_achieved(c)])
    md = mean(deltas)
    sdd = sd(deltas)
    stderr = sdd / math.sqrt(n) if n else 0.0
    wins = sum(1 for d in deltas if d > 0)
    losses = sum(1 for d in deltas if d < 0)
    ties = n - wins - losses

    print(f"A: {a_dir}")
    print(f"B: {b_dir}")
    print(f"paired on {n} shared seeds with an achieved score on both sides")
    if skipped:
        print(f"  ({skipped} shared seed(s) skipped: a bound or exhaustion, not a score)")
    print()
    print(f"  mean A      {mean_a:.2f}   (achieved cells only)")
    print(f"  mean B      {mean_b:.2f}   (achieved cells only)")
    print(f"  mean Δ(B−A) {md:+.2f}   sd {sdd:.2f}")
    print(f"  B wins {wins}   losses {losses}   ties {ties}")
    print(f"\n  standard error of the mean Δ: {stderr:.2f}")
    if md == 0:
        print("  → identical scores (mean Δ = 0). The two runs match.")
    elif abs(md) < 2 * stderr:
        print("  → |mean Δ| < 2·SE: NOT significant. Sweep more seeds before claiming a win.")
    elif md > 0:
        print("  → B looks genuinely better (mean Δ > 2·SE).")
    else:
        print("  → B looks genuinely worse (mean Δ < −2·SE).")
    if n < 40:
        print(f"  (n={n} is small; aim for 40+ paired seeds for a trustworthy verdict.)")


if __name__ == "__main__":
    main()
