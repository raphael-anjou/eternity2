"""Validity verdicts for the refuters, from the generated-board runs in
work/stage0; prints a JSON summary.

  python3 -I stage0_check.py WORK_STAGE0_DIR > results/stage0/summary.json
"""
import glob
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common  # noqa: E402

REFUTERS = ["R0-rows", "R0-spiral", "R1-rows", "R1-spiral", "R2-glucose"]


def rows(path):
    return [json.loads(line) for line in open(path) if line.strip()]


def main():
    w = sys.argv[1]
    out = {"small": {}, "agreement": {}, "cores": {}, "counts": {}, "true_frame": {}}
    per = {r: {"instances": 0, "right": 0, "wrong": 0, "errors": [], "capped": 0} for r in REFUTERS}
    agree = {"pairs": 0, "disagree": 0, "undecided": 0}
    cores = {"checked": 0, "unsound": 0, "by_source": {}}
    boards = sorted(os.path.basename(p)[:-4] for p in glob.glob(os.path.join(w, "g*.puz")))
    for g in boards:
        puz = common.load_puz(os.path.join(w, g + ".puz"))
        brute = json.load(open(os.path.join(w, g + ".puz.brute.json")))
        sol_frames = [{(p, pid) for p, pid, _ in f} for f in brute["frames"]]
        right_set = {tuple(sorted((p, pid) for p, pid, _ in f)) for f in brute["frames"]}
        frames = dict(common.load_frames(os.path.join(w, g + ".frames")))
        verdicts = {}
        for path in glob.glob(os.path.join(w, g, "ref", "*.jsonl")):
            for d in rows(path):
                fr = frames[d["id"]]
                assert not common.check_frame(puz, fr), (g, d["id"])
                is_right = tuple(sorted((p, pid) for p, pid, _ in fr)) in right_set
                r = d["refuter"]
                per[r]["instances"] += 1
                per[r]["right" if is_right else "wrong"] += 1
                verdicts.setdefault(d["id"], {})[r] = d["verdict"]
                if d["verdict"] == "capped":
                    per[r]["capped"] += 1
                elif d["verdict"] == "unsat" and is_right:
                    per[r]["errors"].append([g, d["id"], "unsat on a right frame"])
                elif d["verdict"] == "sat":
                    board = [tuple(x) for x in d["solution"]]
                    probs = common.check_board(puz, board, fr)
                    if probs or not is_right:
                        per[r]["errors"].append([g, d["id"], "sat but " + (probs[0] if probs else "frame not in brute set")])
                if d["verdict"] == "unsat" and not r.startswith("R1"):
                    cells = [p for p, _, _ in d["core_placements"]] if r.startswith("R2") else d["read_border"]
                    sub = {(p, pid) for p, pid, _ in fr if p in cells}
                    src = r
                    cores["by_source"].setdefault(src, {"checked": 0, "unsound": 0})
                    cores["by_source"][src]["checked"] += 1
                    cores["checked"] += 1
                    if any(sub <= sf for sf in sol_frames):
                        cores["unsound"] += 1
                        cores["by_source"][src]["unsound"] += 1
        for fid, v in verdicts.items():
            a, b = v.get("R2-glucose"), v.get("R0-rows")
            if a is None or b is None or "capped" in (a, b):
                agree["undecided"] += 1
                continue
            agree["pairs"] += 1
            if a != b:
                agree["disagree"] += 1
        out["small"][g] = {"solutions": brute["solutions"], "solution_frames": len(brute["frames"]),
                           "frames_tested": len(frames)}
    # partial frames (bottom arcs): unsat iff no brute-force solution agrees
    arc = {"R2-glucose": {"instances": 0, "errors": [], "capped": 0}, "R0-rowsup-partial": {"instances": 0, "errors": [], "capped": 0}}
    for g in boards:
        puz = common.load_puz(os.path.join(w, g + ".puz"))
        brute = json.load(open(os.path.join(w, g + ".puz.brute.json")))
        sol_frames = [{(p, pid) for p, pid, _ in f} for f in brute["frames"]]
        arcs = dict(common.load_frames(os.path.join(w, g + ".arcs")))
        for path in glob.glob(os.path.join(w, g, "arcref", "*.jsonl")):
            for d in rows(path):
                sub = {(p, pid) for p, pid, _ in arcs[d["id"]]}
                extendable = any(sub <= sf for sf in sol_frames)
                a = arc[d["refuter"]]
                a["instances"] += 1
                if d["verdict"] == "capped":
                    a["capped"] += 1
                elif (d["verdict"] == "unsat") == extendable:
                    a["errors"].append([g, d["id"], d["verdict"], "extendable" if extendable else "not extendable"])
                elif d["verdict"] == "sat":
                    probs = common.check_board(puz, [tuple(x) for x in d["solution"]], arcs[d["id"]])
                    if probs:
                        a["errors"].append([g, d["id"], "sat but " + probs[0]])
    out["arcs"] = arc
    out["refuters"] = per
    out["agreement"] = agree
    out["cores"] = cores
    # SMC count against exact enumeration on the 6x6 boards
    errs = []
    for ex in glob.glob(os.path.join(w, "g6-*.subsets.exact.jsonl")):
        g = os.path.basename(ex).split(".")[0]
        exact = {d["id"]: d["log10"] for d in rows(ex)}
        smc = {d["id"]: d["log10_mean"] for d in rows(os.path.join(w, g, "counts.jsonl"))}
        smc["empty"] = rows(os.path.join(w, g, "base.jsonl"))[0]["log10_mean"]
        for k, v in exact.items():
            if k in smc and v is not None and smc[k] is not None:
                errs.append(abs(smc[k] - v))
            elif k in smc:
                errs.append(float("inf") if (v is None) != (smc[k] is None) else 0.0)
    errs.sort()
    out["counts"] = {"subsets": len(errs), "median_abs_log10_err": errs[len(errs) // 2] if errs else None,
                     "max_abs_log10_err": errs[-1] if errs else None}
    # (i) true frames of planted boards
    for path in sorted(glob.glob(os.path.join(w, "t*.true.*.jsonl"))):
        b = os.path.basename(path).split(".")[0]
        puz = common.load_puz(os.path.join(w, b + ".puz"))
        fr = common.load_frames(os.path.join(w, b + ".true"))[0][1]
        for d in rows(path):
            ok = None
            if d["verdict"] == "sat":
                ok = not common.check_board(puz, [tuple(x) for x in d["solution"]], fr)
            out["true_frame"].setdefault(b, {})[d["refuter"]] = {
                "verdict": d["verdict"], "verified": ok, "secs": d["secs"],
                "nodes": d.get("nodes"), "conflicts": d.get("conflicts")}
            if d["verdict"] == "unsat":
                per[d["refuter"]]["errors"].append([b, "true frame", "unsat on the planted frame"])
    out["pass"] = {r: (not per[r]["errors"]) for r in REFUTERS}
    out["pass"]["sat_vs_plain"] = agree["disagree"] == 0 and agree["pairs"] >= 200
    out["pass"]["cores_sound"] = cores["unsound"] == 0
    out["pass"]["R0-arc"] = not arc["R0-rowsup-partial"]["errors"]
    out["pass"]["R2-arc"] = not arc["R2-glucose"]["errors"]
    json.dump(out, sys.stdout, indent=1)
    print()


if __name__ == "__main__":
    main()
