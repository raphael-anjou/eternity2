"""learning-real-prefixes: check of the pipeline on TOY boards before any real prefix is run. Boards of the toy part
(toy generator), written in the dyn format with pins; all five arms must report the same number of completions.
Case A: 5x5 seed 1, corner pinned (toy part: 80 solutions, tree 219645 nodes with the pinned cell counted, AC 16294
tried). Cases B: 6x6 seeds 1 to 4 with the corner, the first row and two inner cells pinned to the planted solution
(pins ahead of the frontier, at least one completion). Cases C: same boards, one prefix cell changed to a wrong but
consistent placement where one exists. usage: python3 -I validate.py"""
import json, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
os.chdir(HERE)
sys.path.insert(0, HERE)
from toylib import generate, rot
UV = os.environ.get("UV", "uv").split()
os.makedirs("tmp", exist_ok=True)


def write(name, n, pieces, pins):
    with open("tmp/%s.dyn" % name, "w") as f:
        f.write("# TOY validation board\n%d\n" % n + "".join("%d %d %d %d\n" % p for p in pieces) + "".join("P %d %d\n" % (c, 4 * p + r) for c, (p, r) in pins.items()))


def run(name):
    d, c = "tmp/%s.dyn" % name, "tmp/%s.cnf" % name
    out = {}
    o = lambda cmd: subprocess.run(cmd, capture_output=True, text=True).stdout
    out["ENC"] = o([sys.executable, "-I", "enc_real.py", d, c]).strip()
    j = json.loads(o(["bin/rowscan_real", d, "600"]))
    out["T"] = (j["sols"], j["nodes"])
    j = json.loads(o(["bin/dyn16", "file:" + d, "AC", "ROW", "exact", "1000000000000", "600"]))
    out["AC"] = (j["sols"], j["tried"], j["failed"])
    j = json.loads(o(UV + ["run", "-q", "--with", "python-sat==1.9.dev22", "python3", "-I", "cdcl_pysat.py", c, "600"]).strip().splitlines()[-1])
    out["K"] = (j["sols"], j["conflicts"])
    for arm, opt in (("C", []), ("U0", ["--no-lookback", "--lookahead=no", "--heuristic=None"])):
        t = o(["bin/clasp", "-q", "--stats=2", "--time-limit=600", "-n", "0"] + opt + [c])
        g = lambda k: int(re.search(r"^c %s\s*:\s*(\d+)" % k, t, re.M).group(1))
        out[arm] = (g("Models"), g("Conflicts"))
    ok = len({v[0] for k, v in out.items() if k != "ENC"}) == 1
    print(name, "AGREE" if ok else "DISAGREE", out, flush=True)
    return ok


good = True
pieces, pin = generate(5, 3, 4, 1)
write("valA", 5, pieces, {0: pin})
good &= run("valA")
for seed in range(1, 5):
    n = 6
    pieces, where = generate(n, 3, 5, seed, full=True)
    cells = list(range(n)) + [15, 28]
    pins = {c: where[c] for c in cells}
    write("valB%d" % seed, n, pieces, pins)
    good &= run("valB%d" % seed)
    # wrong prefix: change cell 5 (top right corner) or cell 4 to another consistent placement if any
    done = False
    for c in (4, 3, 2):
        if done:
            break
        left = rot(pieces[pins[c - 1][0]], pins[c - 1][1])[1]
        usedp = {p for p, _ in pins.values()}
        for p, e in enumerate(pieces):
            for r in range(4):
                o = rot(e, r)
                if p not in usedp and o[0] == 0 and o[3] == left and o[1] != 0 and o[2] != 0 and not done:
                    pins2 = {k: v for k, v in pins.items() if k < c or k in (15, 28)}
                    pins2[c] = (p, r)
                    write("valC%d" % seed, n, pieces, pins2)
                    good &= run("valC%d" % seed)
                    done = True
print("ALL AGREE" if good else "FAILURE")
