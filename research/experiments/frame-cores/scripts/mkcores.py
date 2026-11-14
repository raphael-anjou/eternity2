"""Collect the nogoods found for each refuted frame into a subsets file for
`fc count` (one line per nogood: id = <frame>:<source>, then the border
placements of the nogood).

Sources: the R2 minimised core (SAT failed-assumption set after deletion),
and for every plain-search refutation the set of border cells adjacent to an
interior cell the search reached (the same tree refutes every frame that
agrees on those cells, so it is a nogood with cost = the search's nodes).
R1 is not a source: its matching check reads every unfilled cell that
touches the frame, so its tree depends on the whole frame.

  python3 -I mkcores.py FRAMES RESULTS_DIR OUT_SUBSETS
"""
import glob
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common  # noqa: E402


def main():
    frames = dict(common.load_frames(sys.argv[1]))
    out = open(sys.argv[3], "w")
    for path in sorted(glob.glob(os.path.join(sys.argv[2], "*.jsonl"))):
        for line in open(path):
            d = json.loads(line)
            if d["verdict"] != "unsat" or d["id"] not in frames or d["refuter"].startswith("R1"):
                continue
            fr = {pos: (pid, r) for pos, pid, r in frames[d["id"]]}
            if d["refuter"].startswith("R2"):
                cells = [p for p, _, _ in d["core_placements"]]
            else:
                cells = d["read_border"]
            s = " ".join("%d %d %d" % (p, fr[p][0], fr[p][1]) for p in sorted(cells))
            out.write("%s:%s %s\n" % (d["id"], d["refuter"], s))


if __name__ == "__main__":
    main()
