"""pin-map driver (see ../PREREG.md). One stage per call, resumable: every unit
is one jsonl row in results/<stage>.jsonl keyed by `label`; finished labels
are skipped. Run through run.sh (prereg guard, nice).
Usage: drive.py <stage>"""
import json
import math
import os
import random
import subprocess
import sys

EXP = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BIN = os.path.join(EXP, "engine/target/release/pinmap")
RES = os.path.join(EXP, "results")
CAP_RS = "2e10"
CAP_MRV = "1e5"

SETS = {
    "M8": dict(n=8, cf=4, ci=11, seeds=list(range(1, 21)), clues=[]),
    "M8C": dict(n=8, cf=4, ci=11, seeds=list(range(1, 21)), clues=[18, 21, 42, 45, 28]),
    "M10": dict(n=10, cf=5, ci=18, seeds=list(range(1, 6)), clues=[]),
    "N8": dict(n=8, cf=4, ci=10, seeds=list(range(21, 26)), clues=[]),
}
KMAX = {"M8": 16, "M8C": 16, "M10": 24}
MRV_K = [0, 1, 2, 3, 4, 6, 8, 12, 16]
MRV_SEEDS = list(range(1, 11))
MRV_DRAWS = 5
DRAWS = 20


def board(setname, seed):
    s = SETS[setname]
    return os.path.join(EXP, f"boards/n{s['n']}-f{s['cf']}-c{s['ci']}-s{seed}.txt")


def rows(stage):
    p = os.path.join(RES, f"{stage}.jsonl")
    if not os.path.exists(p):
        return []
    return [json.loads(l) for l in open(p) if l.strip()]


def done(stage):
    return {r["label"] for r in rows(stage)}


def run_bin(args, jobs, stage):
    """Feed job lines; append each output row to results/<stage>.jsonl as it
    streams, return the rows."""
    out = open(os.path.join(RES, f"{stage}.jsonl"), "a")
    p = subprocess.Popen([BIN] + args, stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
    p.stdin.write("".join(jobs))
    p.stdin.close()
    got = []
    for line in p.stdout:
        if line.startswith("{"):
            out.write(line)
            out.flush()
            got.append(json.loads(line))
    if p.wait() != 0:
        sys.exit(f"binary failed: {args}")
    return got


def fx(cells):
    return ",".join(map(str, cells)) if cells else "-"


# ---- pin-set policies (PREREG item 3) ----

def ring(n):
    out = [(0, c) for c in range(n)] + [(r, n - 1) for r in range(1, n)]
    out += [(n - 1, c) for c in range(n - 2, -1, -1)] + [(r, 0) for r in range(n - 2, 0, -1)]
    return [r * n + c for r, c in out]


def nearest_free(n, pos, taken, allowed):
    r0, c0 = divmod(pos, n)
    best = None
    for q in sorted(allowed):
        if q in taken:
            continue
        r, c = divmod(q, n)
        d = abs(r - r0) + abs(c - c0)
        if best is None or d < best[0]:
            best = (d, q)
    return best[1]


def lattice(n, k, size, off, allowed):
    a = math.ceil(math.sqrt(k))
    b = math.ceil(k / a)
    pts = [(math.floor((i + 0.5) * size / a) + off) * n + math.floor((j + 0.5) * size / b) + off
           for i in range(a) for j in range(b)]
    if len(pts) > k:
        pts = [pts[math.floor((t + 0.5) * len(pts) / k)] for t in range(k)]
    out = []
    for q in pts:
        out.append(q if (q in allowed and q not in out) else nearest_free(n, q, set(out), allowed))
    return out


def policy_set(name, n, k, clues, rowrank=None, cellmean=None):
    cells = set(range(n * n)) - set(clues)
    N = n * n
    if name == "b-scan":
        out = []
        for i in range(k):
            q = math.floor((i + 0.5) * N / k)
            while q in clues or q in out:
                q = (q + 1) % N
            out.append(q)
        return out
    if name == "c-lattice":
        return lattice(n, k, n, 0, cells)
    if name == "d-cluster":
        order = []
        for r in rowrank:
            order += sorted([q for q in range(r * n, r * n + n) if q in cells], key=lambda q: (-cellmean[q], q))
        return order[:k]
    if name == "e-frame":
        rg = [q for q in ring(n) if q in cells]
        return [rg[math.floor((i + 0.5) * len(rg) / k)] for i in range(k)]
    if name == "f-interior":
        inner = {q for q in cells if 0 < q // n < n - 1 and 0 < q % n < n - 1}
        return lattice(n, k, n - 2, 1, inner)
    raise ValueError(name)


DET = ["b-scan", "c-lattice", "d-cluster", "e-frame", "f-interior"]


def random_set(n, k, clues, seed, draw):
    free = [q for q in range(n * n) if q not in clues]
    return random.Random(seed * 100000 + k * 100 + draw).sample(free, k)


def single_maps(sstage):
    """Per board (seed): {cell: nodes} from the fwd singles pass, and base."""
    out = {}
    for r in rows(sstage):
        if r["label"].endswith(":fwd"):
            seed = int(r["label"].split(":")[1])
            out[seed] = (r["base_nodes"], dict(zip(r["cand"], r["nodes"])))
    return out


def rank_info(setname, sstage):
    n = SETS[setname]["n"]
    clues = SETS[setname]["clues"]
    maps = single_maps(sstage)
    cellmean = {}
    for q in range(n * n):
        if q in clues:
            continue
        cellmean[q] = sum(math.log2(b) - math.log2(m[q]) for b, m in maps.values()) / len(maps)
    rowmean = {}
    for r in range(n):
        v = [cellmean[q] for q in range(r * n, r * n + n) if q in cellmean]
        rowmean[r] = sum(v) / len(v)
    rowrank = sorted(range(n), key=lambda r: (-rowmean[r], r))
    return rowrank, cellmean


# ---- stages ----

def st_boards():
    os.makedirs(os.path.join(EXP, "boards"), exist_ok=True)
    out = open(os.path.join(RES, "boards.jsonl"), "a")
    have = done("boards")
    for name, s in SETS.items():
        if name == "M8C":
            continue
        for seed in s["seeds"]:
            lab = f"{name}:{seed}"
            if lab in have:
                continue
            r = subprocess.run([BIN, "gen", "--n", str(s["n"]), "--cf", str(s["cf"]), "--ci", str(s["ci"]),
                                "--seed", str(seed), "--out", board(name, seed)],
                               capture_output=True, text=True, check=True).stdout
            d = json.loads(r)
            d["label"] = lab
            d["path"] = os.path.relpath(d["path"], EXP)
            out.write(json.dumps(d) + "\n")
            out.flush()


def st_singles(stage, setname, orders):
    s = SETS[setname]
    have = done(stage)
    for seed in s["seeds"]:
        for o in orders:
            lab = f"{stage}:{seed}:{o}"
            if lab in have:
                continue
            r = run_bin(["multi", "--board", board(setname, seed), "--order", o, "--cap", CAP_RS],
                        [f"{lab}\t{fx(s['clues'])}\tall\n"], stage)[0]
            print(lab, r["base_nodes"], r["seconds"], flush=True)


def st_greedy(stage, setname):
    s = SETS[setname]
    have = {r["label"]: r for r in rows(stage)}
    for seed in s["seeds"]:
        chosen = []
        for k in range(1, KMAX[setname] + 1):
            lab = f"{stage}:{seed}:k{k}"
            if lab in have:
                r = have[lab]
            else:
                base = s["clues"] + chosen
                r = run_bin(["multi", "--board", board(setname, seed), "--order", "fwd", "--cap", CAP_RS],
                            [f"{lab}\t{fx(base)}\tall\n"], stage)[0]
            assert r["complete"]
            best = min(range(len(r["cand"])), key=lambda i: (r["nodes"][i], r["cand"][i]))
            chosen.append(r["cand"][best])
            if lab not in have:
                print(lab, "pick", r["cand"][best], "nodes", r["nodes"][best], r["seconds"], flush=True)


def st_policies(stage, setname, sstage):
    s = SETS[setname]
    n, clues = s["n"], s["clues"]
    rowrank, cellmean = rank_info(setname, sstage)
    maps = single_maps(sstage)
    have = done(stage)
    out = open(os.path.join(RES, f"{stage}.jsonl"), "a")
    for seed in s["seeds"]:
        jobs = []
        for k in range(1, KMAX[setname] + 1):
            sets = [(p, policy_set(p, n, k, clues, rowrank, cellmean)) for p in DET]
            sets += [(f"g-random{d}", random_set(n, k, clues, seed, d)) for d in range(DRAWS)]
            for pname, cells in sets:
                lab = f"{stage}:{seed}:{pname}:k{k}"
                if lab in have:
                    continue
                assert len(set(cells)) == k and not set(cells) & set(clues)
                if k == 1:
                    # the singles pass already holds this exact tree
                    base, m = maps[seed]
                    row = {"label": lab, "solver": "rs-fwd", "pins": cells, "complete": True,
                           "nodes": m[cells[0]], "from": f"{sstage}:{seed}:fwd"}
                    out.write(json.dumps(row) + "\n")
                    out.flush()
                    continue
                jobs.append(f"{lab}\t{fx(clues + cells)}\n")
        if jobs:
            got = run_bin(["rs", "--board", board(setname, seed), "--order", "fwd", "--cap", CAP_RS], jobs, stage)
            print(stage, seed, len(got), "trees", round(sum(g["seconds"] for g in got), 1), "s", flush=True)


def st_pairs():
    stage, setname = "pairs8", "M8"
    have = done(stage)
    for seed in range(1, 6):
        for p in range(63):
            lab = f"{stage}:{seed}:{p}"
            if lab in have:
                continue
            cand = ",".join(str(q) for q in range(p + 1, 64))
            run_bin(["multi", "--board", board(setname, seed), "--order", "fwd", "--cap", CAP_RS],
                    [f"{lab}\t{p}\t{cand}\n"], stage)
        print(stage, seed, flush=True)


def st_mrv():
    stage, setname = "mrv8", "M8"
    s = SETS[setname]
    n = s["n"]
    rowrank, cellmean = rank_info(setname, "s8")
    greedy = {}
    for r in rows("g8"):
        _, seed, k = r["label"].split(":")
        best = min(range(len(r["cand"])), key=lambda i: (r["nodes"][i], r["cand"][i]))
        greedy[(int(seed), int(k[1:]))] = r["cand"][best]
    have = done(stage)
    for seed in MRV_SEEDS:
        jobs = []
        for k in MRV_K:
            if k == 0:
                sets = [("none", [])]
            else:
                sets = [("a-greedy", [greedy[(seed, j)] for j in range(1, k + 1)])]
                sets += [(p, policy_set(p, n, k, [], rowrank, cellmean)) for p in DET]
                sets += [(f"g-random{d}", random_set(n, k, [], seed, d)) for d in range(MRV_DRAWS)]
            for pname, cells in sets:
                lab = f"{stage}:{seed}:{pname}:k{k}"
                if lab not in have:
                    jobs.append(f"{lab}\t{fx(cells)}\n")
        if jobs:
            got = run_bin(["mrv", "--board", board(setname, seed), "--cap", CAP_MRV], jobs, stage)
            print(stage, seed, len(got), "runs", round(sum(g["seconds"] for g in got), 1), "s", flush=True)


def main():
    os.makedirs(RES, exist_ok=True)
    st = sys.argv[1]
    if st == "boards":
        st_boards()
    elif st == "s8":
        st_singles("s8", "M8", ["fwd", "rev"])
    elif st == "s8c":
        st_singles("s8c", "M8C", ["fwd"])
    elif st == "s10":
        st_singles("s10", "M10", ["fwd", "rev"])
    elif st == "n8":
        st_singles("n8", "N8", ["fwd"])
    elif st == "g8":
        st_greedy("g8", "M8")
    elif st == "g8c":
        st_greedy("g8c", "M8C")
    elif st == "g10":
        st_greedy("g10", "M10")
    elif st == "p8":
        st_policies("p8", "M8", "s8")
    elif st == "p8c":
        st_policies("p8c", "M8C", "s8c")
    elif st == "p10":
        st_policies("p10", "M10", "s10")
    elif st == "pairs8":
        st_pairs()
    elif st == "mrv8":
        st_mrv()
    else:
        sys.exit(f"unknown stage {st}")


if __name__ == "__main__":
    main()
