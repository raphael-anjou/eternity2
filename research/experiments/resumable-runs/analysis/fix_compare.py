#!/usr/bin/env python3
"""Fix round (PREREG_AMENDMENTS.md A1), stdlib only. Writes results/fix/compare.json.

1. Every rerun row in results/fix/raw (G-a, G-b, G-c) against the committed
   version 1 row of the same file and instance.
2. Gate G-e from results/fix/tamper/tamper-*.jsonl.
"""
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
OLD = HERE / "results" / "raw"
NEW = HERE / "results" / "fix" / "raw"
TAMPER = HERE / "results" / "fix" / "tamper"
FIELDS = ["status", "nodes", "backtracks", "propagation", "board", "hash", "hashed",
          "closed_share", "deepest"]
SEG_FIELDS = FIELDS + ["segment", "nodes_segment", "backtracks_segment", "propagation_segment",
                       "depth", "resumable"]


def rows(p):
    return [json.loads(l) for l in p.read_text().splitlines() if l.strip()]


def cross_version():
    out = {"files": 0, "rows": 0, "mismatch": [], "missing_old": [], "row_count_differs": []}
    for p in sorted(NEW.glob("g[ab]-*.jsonl")):
        old_p = OLD / p.name
        if not old_p.exists():
            out["missing_old"].append(p.name)
            continue
        new, old = rows(p), rows(old_p)
        out["files"] += 1
        if len(new) != len(old):
            out["row_count_differs"].append(p.name)
        old_by = {r["instance"]: r for r in old}
        for r in new:
            o = old_by.get(r["instance"])
            out["rows"] += 1
            if p.name.startswith("ga-"):
                pairs = [("fast", KEYS7), ("resumable", FIELDS)]
                bad = o is None or any(r[s][k] != o[s][k] for s, ks in pairs for k in ks)
            else:
                bad = o is None or any(r[k] != o[k] for k in SEG_FIELDS)
            if bad:
                out["mismatch"].append(f"{p.name}:{r['instance']}")
    return out


KEYS7 = FIELDS[:7]


def gate_e():
    res = {}
    for p in sorted(TAMPER.glob("tamper-*.jsonl")):
        rs = rows(p)
        alt = [r for r in rs if r["kind"] == "altered"]
        forged = [r for r in rs if r["kind"] == "forged-digest"]
        unalt = [r for r in rs if r["kind"] == "unaltered"]
        res[p.stem.removeprefix("tamper-")] = {
            "tried": len(alt),
            "refused": sum(r["refused"] for r in alt),
            "accepted": [r["alteration"] for r in alt if not r["refused"]],
            "unaltered_identical": bool(unalt) and all(r["identical"] for r in unalt),
            "forged_tried": len(forged),
            "forged_refused": sum(r["refused"] for r in forged),
            "forged_accepted": sorted({r["alteration"] for r in forged if not r["refused"]}),
        }
    total = {k: sum(v[k] for v in res.values()) for k in ("tried", "refused", "forged_tried", "forged_refused")}
    passed = bool(res) and len(res) == 3 and all(
        v["refused"] == v["tried"] and v["unaltered_identical"] for v in res.values())
    return {"checkpoints": res, "total": total, "pass": passed}


def main():
    out = {"cross_version": cross_version(), "gate_e": gate_e()}
    (HERE / "results" / "fix" / "compare.json").write_text(json.dumps(out, indent=1) + "\n")
    cv = out["cross_version"]
    print("cross-version:", cv["files"], "files", cv["rows"], "rows,", len(cv["mismatch"]), "mismatches",
          "missing", cv["missing_old"], "count differs", cv["row_count_differs"])
    print("G-e:", json.dumps(out["gate_e"]["total"]), "pass", out["gate_e"]["pass"])
    for k, v in out["gate_e"]["checkpoints"].items():
        print(" ", k, v["refused"], "/", v["tried"], "unaltered identical", v["unaltered_identical"],
              "forged", v["forged_refused"], "/", v["forged_tried"], "forged accepted:", v["forged_accepted"])


if __name__ == "__main__":
    main()
