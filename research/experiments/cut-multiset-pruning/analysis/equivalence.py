#!/usr/bin/env python3
"""Fork with the Hall hook off must count node for node like the committed baseline."""
import json
import os
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def load(p):
    with open(p) as f:
        return {r["instance"]: r for r in map(json.loads, filter(str.strip, f))}


fork = load(os.path.join(ROOT, "results/test/primary/cmp-fork-off.jsonl"))
base = load(os.path.join(ROOT, "../hard-ladder/results/test/primary/scb-gacolor_ac3_random.jsonl"))
diff = [i for i, r in fork.items() if (r["status"], r["nodes"], r["backtracks"]) != (base[i]["status"], base[i]["nodes"], base[i]["backtracks"])]
print(f"fork-off vs committed scb-gacolor_ac3_random: {len(fork)} instances, {len(diff)} differ")
sys.exit(1 if diff or not fork else 0)
