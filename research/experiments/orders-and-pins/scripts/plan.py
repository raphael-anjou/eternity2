"""Write every plan file (plans/<stage>.jsonl) from the preregistered design.

    python3 -I scripts/plan.py

Deterministic: reads orders/manifest.json, pins/model.json and the board
files, draws random cells with random.Random(fixed seeds). Every unit gets
its own SMC seed from one counter per stage (stage base + running index).
"""
import json
import os
import random
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
EXP = os.path.dirname(HERE)
sys.path.insert(0, HERE)
from model import Model  # noqa: E402
from orders import CLUES16, CLUES6, load_official  # noqa: E402
from pins_model import read_order  # noqa: E402

BIN = "engine/target/release/rpp"
CSV = "../single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv"
LADDER6 = "../hard-ladder/instances/primary-test.jsonl"
CL16 = [r * 16 + c for (r, c) in CLUES16]
CL6 = [r * 6 + c for (r, c) in CLUES6]
NULL_BOARDS = ["boards/null/board-200115.txt", "boards/null/board-200139.txt", "boards/null/board-200181.txt"]
PIN_BOARDS = ["boards/pins/board-300051.txt", "boards/pins/board-300061.txt", "boards/pins/board-300097.txt",
              "boards/pins/board-300117.txt", "boards/pins/board-300131.txt"]
B_ORDER = "rows-b-lr"

# sample sizes (PREREG.md, fixed from results/calib)
A_OFFICIAL = (1_000_000, 5)
A_LADDER = (100_000, 5)        # A4 (M2): stability ladder, seeds 705001 onward
GB = ([88, 72, 56], 3e9, (100_000, 10), (10_000, 10))   # A4 (M6)
G8 = (100, 10, 200, int(4e8))  # A4 (M7): SMC N, PPD N, reps, enumeration cap
BL = (100_000, 5)              # A4 (M11): leave one clue out
CALIB_BOARDS = ["boards/calib/board-100149.txt", "boards/calib/board-100244.txt"]
SETS8 = {
    "none": [],
    "clustered8": [(5, 1), (5, 3), (5, 5), (6, 2), (6, 4), (7, 1), (7, 3), (7, 5)],
    "spread8": [(6, 2), (6, 5), (4, 2), (4, 5), (3, 3), (2, 2), (2, 5), (1, 3)],
    "clustered4": [(5, 1), (5, 4), (6, 2), (6, 5)],
    "spread4": [(6, 3), (4, 4), (3, 2), (1, 4)],
}
A_NULL = (100_000, 5)
G6 = (200, 200)
GT = (100_000, 10, 3e9, 9.0)   # N, reps, enumeration cap, model log10 budget for the depth K
B0 = (1_000_000, 5)
B0_PPD = (10_000, 5)
B1 = (100_000, 4)
B1_PPD = (10_000, 3)
BK_SMC = (100_000, 5)          # multi-pin sets, plain SMC
BK_PPD = (10_000, 4)           # multi-pin sets, planted-path decomposition
B4_CAP = 2e10
GREEDY_K = [4, 12]  # A4 (M10): cut to two k
SPREAD_K = [1, 2, 4, 6, 8, 10, 12, 16, 24, 32, 40, 48]
RANDOM_K = [1, 2, 4, 8, 16, 24, 32, 48]
RANDOM_SETS = 2
B4_SPREAD_K = [40, 48]


def spread(k, lo=16, hi=160):
    """Spread policy: k pins at evenly spaced free depths of the row scan
    between lo and hi (the region before the model's peak at 158; depth 16
    is the end of the bottom frame row), each moved forward along the order
    to the first cell that is a middle cell not adjacent to a clue or to an
    earlier pin. k = 1: depth (lo + hi) / 2."""
    order = [(r, c) for r in range(15, -1, -1) for c in range(16)]
    free = [x for x in order if x not in CLUES16]
    pins = []
    for i in range(k):
        d = lo + round(i * (hi - lo) / (k - 1)) if k > 1 else (lo + hi) // 2
        j = d - 1
        while True:
            r, c = free[j]
            bad = (r in (0, 15) or c in (0, 15) or (r, c) in pins
                   or any(abs(r - a) + abs(c - b) == 1 for (a, b) in CLUES16 + pins))
            if not bad:
                break
            j += 1
        pins.append(free[j])
    return pins


def fx(cells):
    return ",".join(str(p) for p in cells)


def pos(cells):
    return [r * 16 + c for (r, c) in cells]


def out(stage, units):
    os.makedirs(os.path.join(EXP, "plans"), exist_ok=True)
    ids = [u["unit"] for u in units]
    assert len(ids) == len(set(ids)), stage
    with open(os.path.join(EXP, "plans", f"{stage}.jsonl"), "w") as fh:
        for u in units:
            fh.write(json.dumps(u) + "\n")
    print(stage, len(units), "units")


def tsmc(unit, src, fixed, order, np_, seed, label):
    return {"unit": unit, "cmd": [BIN, "tsmc", *src, "--fixed", fx(fixed), "--order", order,
                                  "--np", str(np_), "--seed0", str(seed), "--label", label]}


def ppd(unit, board, fixed, order, np_, seed, label):
    return {"unit": unit, "cmd": [BIN, "tppd", "--board", board, "--fixed", fx(fixed), "--order", order,
                                  "--np", str(np_), "--seed0", str(seed), "--label", label]}


def main():
    man = json.load(open(os.path.join(EXP, "orders", "manifest.json")))
    pm = json.load(open(os.path.join(EXP, "pins", "model.json")))
    orders = list(man["orders"])
    named = [o for o in orders if man["orders"][o]["kind"] == "named"]
    M = Model(16, load_official())

    # gate6: named orders, 6x6 analogues, without and with the five clue analogues
    units = []
    for o in named:
        for tag, fixed in (("noclue", []), ("clue5", CL6)):
            cmd = [BIN, "gate6", "--jsonl", LADDER6, "--order", f"orders/6/{o}.txt", "--np", str(G6[0]),
                   "--reps", str(G6[1]), "--label", f"g6-{o}-{tag}"]
            if fixed:
                cmd += ["--fixed", fx(fixed)]
            units.append({"unit": f"g6:{o}:{tag}", "cmd": cmd})
    out("gate6", units)

    # GT: every order on the official set, tree truncated at free depth K
    # (K = the largest depth whose model cumulative node count is at most
    # 10^8.5), exact by enumeration and estimated by SMC.
    units = []
    seed = 600000
    gt_k = {}
    for o in orders:
        prof = M.profile(read_order(o), CLUES16)
        cum, K = 0.0, 0
        for d, lw in enumerate(prof, start=1):
            cum += 10 ** lw
            if cum > 10 ** GT[3]:
                break
            K = d
        gt_k[o] = K
        units.append({"unit": f"gt:{o}:enum", "cmd": [BIN, "tenumd", "--csv", CSV, "--fixed", fx(CL16), "--order",
                      f"orders/16/{o}.txt", "--maxd", str(K), "--cap", str(int(GT[2])), "--label", f"gt-{o}"]})
        for r in range(GT[1]):
            seed += 1
            units.append(tsmc(f"gt:{o}:r{r}", ["--csv", CSV], CL16, f"orders/16/{o}.txt", GT[0], seed, f"gt-{o}"))
    out("gt", units)
    with open(os.path.join(EXP, "plans", "gt-depths.json"), "w") as fh:
        json.dump(gt_k, fh, indent=1)

    # Part A, official set, every order
    units, seed = [], 700000
    for o in orders:
        for r in range(A_OFFICIAL[1]):
            seed += 1
            units.append(tsmc(f"a:{o}:r{r}", ["--csv", CSV], CL16, f"orders/16/{o}.txt", A_OFFICIAL[0], seed, f"a-{o}"))
    for o in orders:
        for r in range(A_LADDER[1]):
            seed_l = 705001 + len([u for u in units if u["unit"].startswith("a:") and ":np1e5:" in u["unit"]])
            units.append(tsmc(f"a:{o}:np1e5:r{r}", ["--csv", CSV], CL16, f"orders/16/{o}.txt", A_LADDER[0], seed_l,
                              f"a-{o}-np1e5"))
    out("a-official", units)

    # Part A null: three census-painted boards, five clues from their planted boards
    units, seed = [], 800000
    for b in NULL_BOARDS:
        bid = os.path.basename(b)[6:-4]
        for o in orders:
            for r in range(A_NULL[1]):
                seed += 1
                units.append(tsmc(f"n:{bid}:{o}:r{r}", ["--board", b], CL16, f"orders/16/{o}.txt", A_NULL[0], seed,
                                  f"n-{bid}-{o}"))
    out("a-null", units)

    free = [(r, c) for r in range(16) for c in range(16) if (r, c) not in CLUES16]
    # Part B
    border = f"orders/16/{B_ORDER}.txt"
    greedy = [tuple(x) for x in pm["greedy"]]
    ranked_best = [(x[0], x[1]) for x in pm["ranked_best10"][:3]]
    ranked_worst = [(x[0], x[1]) for x in pm["ranked_worst10"][-3:]]
    grid = [(r, c) for r in (0, 3, 6, 9, 12, 15) for c in (0, 3, 6, 9, 12, 15)]
    mids = [(0, 7), (15, 7), (7, 0), (7, 15)]
    s1 = []
    for cell in grid + mids + ranked_best + ranked_worst + greedy[:8] + spread(8):
        if cell not in s1 and cell not in CLUES16:
            s1.append(cell)
    s1_ppd = []
    for cell in greedy[:2] + spread(8)[:2]:
        if cell not in s1_ppd and cell not in CLUES16:
            s1_ppd.append(cell)

    def both(stage, tag, b, pins, seedc):
        us = []
        bid = os.path.basename(b)[6:-4]
        for r in range(BK_SMC[1]):
            seedc[0] += 1
            us.append(tsmc(f"{stage}:{bid}:{tag}:smc:r{r}", ["--board", b], CL16 + pos(pins), border, BK_SMC[0],
                           seedc[0], f"{stage}-{bid}-{tag}-smc"))
        for r in range(BK_PPD[1]):
            seedc[0] += 1
            us.append(ppd(f"{stage}:{bid}:{tag}:ppd:r{r}", b, CL16 + pos(pins), border, BK_PPD[0], seedc[0],
                          f"{stage}-{bid}-{tag}-ppd"))
        return us

    units, sc = [], [900000]
    for b in PIN_BOARDS:
        bid = os.path.basename(b)[6:-4]
        for r in range(B0[1]):
            sc[0] += 1
            units.append(tsmc(f"b0:{bid}:k0:smc:r{r}", ["--board", b], CL16, border, B0[0], sc[0], f"b0-{bid}-smc"))
        for r in range(B0_PPD[1]):
            sc[0] += 1
            units.append(ppd(f"b0:{bid}:k0:ppd:r{r}", b, CL16, border, B0_PPD[0], sc[0], f"b0-{bid}-ppd"))
    out("b0", units)
    units, sc = [], [910000]
    for b in PIN_BOARDS:
        bid = os.path.basename(b)[6:-4]
        for cell in s1:
            tag = f"{cell[0]},{cell[1]}"
            for r in range(B1[1]):
                sc[0] += 1
                units.append(tsmc(f"b1:{bid}:{tag}:smc:r{r}", ["--board", b], CL16 + pos([cell]), border, B1[0],
                                  sc[0], f"b1-{bid}-{tag}-smc"))
            if cell in s1_ppd:
                for r in range(B1_PPD[1]):
                    sc[0] += 1
                    units.append(ppd(f"b1:{bid}:{tag}:ppd:r{r}", b, CL16 + pos([cell]), border, B1_PPD[0], sc[0],
                                     f"b1-{bid}-{tag}-ppd"))
    out("b1", units)
    units, sc = [], [920000]
    for b in PIN_BOARDS:
        for k in GREEDY_K:
            units += both("b2", f"k{k}", b, greedy[:k], sc)
    out("b2", units)
    units, sc = [], [925000]
    for b in PIN_BOARDS:
        for k in SPREAD_K:
            units += both("bs", f"k{k}", b, spread(k), sc)
    out("bs", units)
    units, sc = [], [930000]
    rsets = {}
    for bi, b in enumerate(PIN_BOARDS):
        bid = os.path.basename(b)[6:-4]
        for k in RANDOM_K:
            for j in range(RANDOM_SETS):
                rng = random.Random(1000 * (bi + 1) + 10 * k + j)
                cells = rng.sample(free, k)
                rsets[f"{bid}:k{k}:s{j}"] = [list(c) for c in cells]
                units += both("b3", f"k{k}-s{j}", b, cells, sc)
    out("b3", units)
    units = []
    for b in PIN_BOARDS:
        bid = os.path.basename(b)[6:-4]
        for k in B4_SPREAD_K:
            units.append({"unit": f"b4:{bid}:k{k}", "cmd": [BIN, "tenum", "--board", b, "--fixed",
                          fx(CL16 + pos(spread(k))), "--order", border, "--cap", str(int(B4_CAP)),
                          "--label", f"b4-{bid}-spread{k}"]})
    out("b4", units)

    # A4 (M6) gate GB: exact truncated trees under pins at full size
    units, sc = [], [940000]
    gb_sets = {"greedy12": greedy[:12], "greedy16": greedy[:16], "spread16": spread(16)}
    for b in CALIB_BOARDS:
        bid = os.path.basename(b)[6:-4]
        for name, pins in gb_sets.items():
            for D in GB[0]:
                units.append({"unit": f"gb:{bid}:{name}:enum{D}", "cmd": [BIN, "tenumd", "--board", b, "--fixed",
                              fx(CL16 + pos(pins)), "--order", border, "--maxd", str(D), "--cap", str(int(GB[1])),
                              "--label", f"gb-{bid}-{name}-d{D}"]})
            for r in range(GB[2][1]):
                sc[0] += 1
                units.append(tsmc(f"gb:{bid}:{name}:smc:r{r}", ["--board", b], CL16 + pos(pins), border, GB[2][0],
                                  sc[0], f"gb-{bid}-{name}-smc"))
            for r in range(GB[3][1]):
                sc[0] += 1
                units.append(ppd(f"gb:{bid}:{name}:ppd:r{r}", b, CL16 + pos(pins), border, GB[3][0], sc[0],
                                 f"gb-{bid}-{name}-ppd"))
    out("gb", units)

    # A4 (M7) 8x8 implementation gate for both estimators with pins
    units = []
    os.makedirs(os.path.join(EXP, "boards", "b8"), exist_ok=True)
    os.makedirs(os.path.join(EXP, "orders", "8"), exist_ok=True)
    o8 = "orders/8/rows-b-lr.txt"
    with open(os.path.join(EXP, o8), "w") as fh:
        fh.write("# rows-b-lr, 8x8\n" + " ".join(str(r * 8 + c) for r in range(7, -1, -1) for c in range(8)) + "\n")
    for li, line in enumerate(open(os.path.join(EXP, "../hard-ladder/instances/8-c10-test.jsonl"))):
        d = json.loads(line)
        bp = f"boards/b8/{d['id']}.txt"
        with open(os.path.join(EXP, bp), "w") as fh:
            fh.write(f"# {d['id']} from hard-ladder/instances/8-c10-test.jsonl (puzzle and planted)\n8\n")
            for pc in d["puzzle"]["pieces"]:
                fh.write(" ".join(map(str, pc)) + "\n")
            for v in d["planted"]:
                fh.write(f"{v // 4} {v % 4}\n")
        for name, cells in SETS8.items():
            fixed = ["--fixed", fx([r * 8 + c for (r, c) in cells])] if cells else []
            base = ["--board", bp, "--order", o8] + fixed
            units.append({"unit": f"g8:{d['id']}:{name}:enum", "cmd": [BIN, "tenum", *base, "--cap", str(G8[3]),
                          "--label", f"g8-{d['id']}-{name}"]})
            units.append({"unit": f"g8:{d['id']}:{name}:smc", "cmd": [BIN, "tsmc", *base, "--np", str(G8[0]), "--reps",
                          str(G8[2]), "--seed0", str(1_000_000 + 1000 * li), "--label", f"g8-{d['id']}-{name}-smc"]})
            units.append({"unit": f"g8:{d['id']}:{name}:ppd", "cmd": [BIN, "tppd", *base, "--np", str(G8[1]), "--reps",
                          str(G8[2]), "--seed0", str(2_000_000 + 1000 * li), "--label", f"g8-{d['id']}-{name}-ppd"]})
    out("g8", units)

    # A4 (M11) bl: leave one clue out, official set and the five Part B boards
    units, sc = [], [950000]
    for src, bid in [(["--csv", CSV], "official")] + [(["--board", b], os.path.basename(b)[6:-4]) for b in PIN_BOARDS]:
        for drop in CL16:
            for r in range(BL[1]):
                sc[0] += 1
                units.append(tsmc(f"bl:{bid}:drop{drop}:r{r}", src, [c for c in CL16 if c != drop], border, BL[0],
                                  sc[0], f"bl-{bid}-drop{drop}"))
    out("bl", units)
    with open(os.path.join(EXP, "plans", "partb-sets.json"), "w") as fh:
        json.dump({"order": B_ORDER, "s1": [list(c) for c in s1], "s1_ppd": [list(c) for c in s1_ppd],
                   "greedy": [list(c) for c in greedy],
                   "spread": {k: [list(c) for c in spread(k)] for k in SPREAD_K},
                   "random_sets": rsets}, fh, indent=1)


if __name__ == "__main__":
    main()
