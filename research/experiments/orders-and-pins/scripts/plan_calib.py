"""Calibration plan (boards/calib only, disjoint from every test board and
from the official set): rep-to-rep spread per order and per pinned
configuration at N = 1e5 and 1e6, and enumeration cost of pinned trees.

    python3 -I scripts/plan_calib.py > plans/calib.jsonl
"""
import json

BIN = "engine/target/release/rpp"
B = "boards/calib/board-100149.txt"
CL = [8 * 16 + 7, 2 * 16 + 2, 2 * 16 + 13, 13 * 16 + 2, 13 * 16 + 13]
GREEDY = [(11, 1), (11, 3), (11, 5), (13, 4), (14, 1), (13, 6), (14, 3), (14, 5), (14, 7), (14, 9),
          (14, 11), (14, 14), (15, 2), (15, 4), (15, 6), (15, 8)]


def fx(extra=()):
    return ",".join(str(p) for p in CL + [r * 16 + c for (r, c) in extra])


units = []
seed = 50000
for o in ["rows-b-lr", "antidiag", "spiral-out", "frame-then-rows", "spiral-in", "rows-t-rl"]:
    for r in range(10):
        seed += 1
        units.append({"unit": f"calib:{o}:np1e5:r{r}", "cmd": [BIN, "tsmc", "--board", B, "--fixed", fx(),
                      "--order", f"orders/16/{o}.txt", "--np", "100000", "--seed0", str(seed), "--label", f"calib-{o}"]})
for r in range(3):
    seed += 1
    units.append({"unit": f"calib:rows-b-lr:np1e6:r{r}", "cmd": [BIN, "tsmc", "--board", B, "--fixed", fx(),
                  "--order", "orders/16/rows-b-lr.txt", "--np", "1000000", "--seed0", str(seed), "--label", "calib-rows-np1e6"]})
for o in ["antidiag", "spiral-in"]:
    for r in range(3):
        seed += 1
        units.append({"unit": f"calib:{o}:np1e6:r{r}", "cmd": [BIN, "tsmc", "--board", B, "--fixed", fx(),
                      "--order", f"orders/16/{o}.txt", "--np", "1000000", "--seed0", str(seed), "--label", f"calib-{o}-np1e6"]})
for k in (1, 4, 12):
    for r in range(10):
        seed += 1
        units.append({"unit": f"calib:greedy{k}:np1e5:r{r}", "cmd": [BIN, "tsmc", "--board", B, "--fixed", fx(GREEDY[:k]),
                      "--order", "orders/16/rows-b-lr.txt", "--np", "100000", "--seed0", str(seed), "--label", f"calib-greedy{k}"]})
for k in (12, 16):
    units.append({"unit": f"calib:greedy{k}:enum", "cmd": [BIN, "tenum", "--board", B, "--fixed", fx(GREEDY[:k]),
                  "--order", "orders/16/rows-b-lr.txt", "--cap", "1e10", "--label", f"calib-greedy{k}-enum"]})
# added after the first calibration pass: PPD cost and agreement at k = 0 and k = 4
for tag, extra in (("k0", []), ("greedy4", GREEDY[:4])):
    for r in range(3):
        seed += 1
        units.append({"unit": f"calib:{tag}:ppd:r{r}", "cmd": [BIN, "tppd", "--board", B, "--fixed", fx(extra),
                      "--order", "orders/16/rows-b-lr.txt", "--np", "10000", "--seed0", str(seed), "--label", f"calib-{tag}-ppd"]})
for u in units:
    print(json.dumps(u))
