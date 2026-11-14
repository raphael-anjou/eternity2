"""Driver for the gap-6a linearisation check (TOY BOARDS; not Eternity II progress).

  python3 -I run.py check      code checks on 3x3 against brute force and against ref_xl.py
  python3 -I run.py controls   negative and positive controls
  python3 -I run.py main SIZE DEGREES [FIELDS] [ENCODINGS] [NBOARDS] [MODE] [SEEDS]   the measurement, appended to results.jsonl
Each xl run is one process on one core under nice -n 10 with a 840 s CPU limit and a 7.5 GB limit.
"""
import sys, os, json, subprocess
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from boards import gen, solve, pin_tile
import xlsys
SYS = os.path.join(HERE, "systems"); os.makedirs(SYS, exist_ok=True)
COLOURS = {3: (3, 3), 4: (3, 4), 5: (3, 6)}     # fewest colours with median pinned solution count 1 (scan_*.out)
FIELDS = {"GF(2)": "xl_2", "GF(2^31-1)": "xl_p"}

def xl(path, D, field, mode="sparse-cdesc", tlim=840, mem=7.5):
    out = subprocess.run(["nice", "-n", "10", os.path.join(HERE, FIELDS[field]), path, str(D), "2" if mode in ("direct", "transposed") and field == "GF(2)" else "p", mode, str(tlim), str(mem)],
                         capture_output=True, text=True)
    return json.loads(out.stdout)

def board(n, seed, want=None, cap=2000):
    cf, ci = COLOURS[n]
    tiles = gen(n, cf, ci, seed)
    sols = solve(n, tiles, cap=cap)
    return tiles, sols

def system(n, seed, tiles, sols, enriched, pins=(0,), tag=""):
    sd = xlsys.build(n, tiles, sols[0], list(pins), enriched)
    path = os.path.join(SYS, f"b{n}_s{seed}_{'enr' if enriched else 'min'}{tag}.txt")
    xlsys.write(sd, path)
    return sd, path

def summarise(o, sd, sols):
    """Soundness against every solution, and the forced count."""
    vecs = [xlsys.solvec_of(sd, s) for s in sols]
    forced = o.get("forced")
    if forced is None: return dict(forced=None, determined=None, sound=None)
    sound = all(vec[a] == v for a, v in forced for vec in vecs) and not o["inconsistent"]
    const = sum(1 for a in range(len(sd["vars"])) if len({vec[a] for vec in vecs}) == 1)
    return dict(forced=len(forced), sound=sound, determined=(len(forced) == len(sd["vars"])), constant_on_variety=const,
                forced_set=sorted(a for a, v in forced))

def unique_seeds(n, k):
    out = []; s = 0
    while len(out) < k:
        t, sols = board(n, s)
        if len(sols) == 1: out.append(s)
        s += 1
    return out

def check():
    ok = True
    print("3x3 code checks, boards from seeds 0..7 with colours", COLOURS[3], "(any number of solutions)")
    for seed in range(8):
        tiles, sols = board(3, seed)
        for enriched in (False, True):
            sd, path = system(3, seed, tiles, sols, enriched)
            for field, p in (("GF(2)", 2), ("GF(2^31-1)", 2147483647)):
                for D in (1, 2, 3):
                    a = xl(path, D, field, "sparse-cdesc"); b = xl(path, D, field, "direct"); c = xl(path, D, field, "transposed")
                    r = json.loads(subprocess.run([sys.executable, "-I", os.path.join(HERE, "ref_xl.py"), path, str(D), str(p)], capture_output=True, text=True).stdout)
                    same = (a["rank_full"] == b["rank_full"] == c["rank_full"] == r["rank_full"]) and \
                           (sorted(a["forced"]) == sorted(b["forced"]) == sorted(c["forced"]) == r["forced"])
                    s = summarise(a, sd, sols)
                    ok &= same and s["sound"]
                    print(f"seed {seed} sols {len(sols)} {'enriched' if enriched else 'minimal '} {field:11s} D={D}: rank {a['rank_full']:.0f} "
                          f"(ref {r['rank_full']}) forced {s['forced']}/{len(sd['vars'])} constant on variety {s['constant_on_variety']} "
                          f"three engines and reference agree: {same}; sound: {s['sound']}")
                if enriched:
                    D = 8   # one placement per free cell: the linearisation is complete (see memo)
                    a = xl(path, D, field)
                    s = summarise(a, sd, sols)
                    good = (a["nullity"] == len(sols)) and s["forced"] == s["constant_on_variety"] and s["sound"]
                    ok &= good
                    print(f"seed {seed} sols {len(sols)} enriched {field:11s} D=8: monomials {a['monomials_nonzero']} nullity {a['nullity']} "
                          f"(brute force: {len(sols)} solutions) forced {s['forced']} (brute force: {s['constant_on_variety']} constant) -> {'OK' if good else 'MISMATCH'}")
    print("ALL CHECKS PASS" if ok else "SOME CHECK FAILED")

def controls():
    n = 4
    # negative: first seed with exactly two pinned solutions
    s = 0
    while True:
        tiles, sols = board(n, s)
        if len(sols) == 2: break
        s += 1
    print(f"negative control: 4x4 seed {s}, 2 pinned solutions")
    for enriched in (False, True):
        sd, path = system(n, s, tiles, sols, enriched)
        for field in FIELDS:
            for D in (1, 2, 3):
                o = xl(path, D, field); r = summarise(o, sd, sols)
                print(f"  {'enriched' if enriched else 'minimal '} {field:11s} D={D}: forced {r['forced']}/{len(sd['vars'])}, constant on both solutions {r['constant_on_variety']}, "
                      f"determined {r['determined']} (must be False), sound {r['sound']} (must be True), {o['seconds']} s")
    # positive: first unique 4x4 board, all cells pinned to the solution except 5, 10, 15
    s = unique_seeds(n, 1)[0]
    tiles, sols = board(n, s)
    free = (5, 10, 15)
    pins = [x for x in range(n * n) if x not in free]
    print(f"positive control: 4x4 seed {s}, every cell pinned to the solution except cells {free}")
    for enriched in (False, True):
        sd, path = system(n, s, tiles, sols, enriched, pins=pins, tag="_pos")
        for field in FIELDS:
            o = xl(path, 1, field); r = summarise(o, sd, sols)
            print(f"  {'enriched' if enriched else 'minimal '} {field:11s} D=1: variables left {len(sd['vars'])}, forced {r['forced']}, determined {r['determined']} (must be True), sound {r['sound']}")

def main(n, degrees, fields, encodings, nboards, mode="sparse-cdesc", only=None):
    seeds = [s for s in unique_seeds(n, nboards) if only is None or s in only]
    res = open(os.path.join(HERE, "results.jsonl"), "a")
    for seed in seeds:
        tiles, sols = board(n, seed)
        for enc in encodings:
            sd, path = system(n, seed, tiles, sols, enc == "enriched")
            for field in fields:
                for D in degrees:
                    o = xl(path, D, field, mode); r = summarise(o, sd, sols)
                    o.pop("forced", None)
                    rec = dict(size=n, seed=seed, colours=COLOURS[n], encoding=enc, field_name=field, vars_before_pin=sd["full_vars"], **o, **r)
                    res.write(json.dumps(rec) + "\n"); res.flush()
                    print(f"{n}x{n} seed {seed} {enc:8s} {field:11s} D={D}: monomials {o['monomials_all']:.0f} (nonzero {o['monomials_nonzero']}) "
                          + (f"rank {o['rank_full']:.0f} nullity {o['nullity']} forced {r['forced']}/{o['n']} determined {r['determined']} sound {r['sound']}" if "rank_full" in o
                             else f"rows of spread <= {o['class_reached']} of {o['class_max']}: forced {r['forced']}/{o['n']} determined {r['determined']} sound {r['sound']}" if o.get("partial")
                             else f"ABORTED: {o['aborted']}")
                          + f" {o['seconds']} s", flush=True)

if __name__ == "__main__":
    if sys.argv[1] == "check": check()
    elif sys.argv[1] == "controls": controls()
    else:
        n = int(sys.argv[2]); degrees = [int(v) for v in sys.argv[3].split(",")]
        fields = sys.argv[4].split(",") if len(sys.argv) > 4 else list(FIELDS)
        encs = sys.argv[5].split(",") if len(sys.argv) > 5 else ["minimal", "enriched"]
        nb = int(sys.argv[6]) if len(sys.argv) > 6 else {3: 10, 4: 10, 5: 5}[n]
        main(n, degrees, fields, encs, nb, sys.argv[7] if len(sys.argv) > 7 else "sparse-cdesc",
             [int(v) for v in sys.argv[8].split(",")] if len(sys.argv) > 8 else None)
