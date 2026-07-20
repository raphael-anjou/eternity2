#!/usr/bin/env python3
"""Convert a site-JSON puzzle (the format run_dfs_codegen_jit reads) into
McGavin's genbody.c `.puz` + `.hnt` pair, so the same board can be fed to both
engines for a like-for-like throughput comparison.

Usage:
    python3 to_mcgavin.py bench-hard.json out_prefix
      -> writes out_prefix.puz and out_prefix.hnt

Two conventions, learned by reading genbody.c (both verified against a board
that solves identically in both engines):

  * PIECES. Our JSON stores each piece's four edges in URDL order
    [up, right, down, left]. genbody.c reads a `.puz` line as `N E S W`
    (north, east, south, west) into edge[0..3]. URDL == NESW, so the four
    integers copy across verbatim, in order. No reordering.

  * HINTS (the gotcha). genbody.c reads a hint line as `piece x y rotate`,
    1-indexed, and decrements internally. BUT the shipped source has a
    `#define TRANSPOSE` path that swaps x and y on read. The boards here were
    produced for the transposed reading, so we emit `piece y x rotate`
    (1-indexed). If your genbody.c has TRANSPOSE *disabled*, drop the swap
    (pass --no-transpose). If the board does not solve in genbody but does in
    our engine, you have the transpose backwards — flip this flag.

The colour count on the `.puz` header line is max(edge colour) as genbody
expects (it calls that `maxedgetype`).
"""
import json
import sys

def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    transpose = "--no-transpose" not in sys.argv
    if len(args) != 2:
        sys.exit("usage: to_mcgavin.py <board.json> <out_prefix> [--no-transpose]")
    src, out = args
    d = json.load(open(src))
    W, H = d["width"], d["height"]
    pieces = d["pieces"]                 # each [N,E,S,W] == [U,R,D,L]
    maxc = max(e for p in pieces for e in p)

    with open(out + ".puz", "w") as f:
        f.write(f"{W} {H} {maxc}\n")
        for p in pieces:
            f.write(" ".join(str(e) for e in p) + "\n")   # N E S W, verbatim

    hints = d.get("hints", [])
    with open(out + ".hnt", "w") as f:
        f.write(f"{len(hints)}\n")
        for h in hints:
            pos = h["pos"]; y, x = divmod(pos, W); r = h["rot"]; pid = h["piece"]
            a, b = (y, x) if transpose else (x, y)         # transpose gotcha
            f.write(f"{pid + 1} {a + 1} {b + 1} {r}\n")    # 1-indexed

    print(f"wrote {out}.puz ({len(pieces)} pieces) and {out}.hnt "
          f"({len(hints)} hints, transpose={transpose})")

if __name__ == "__main__":
    main()
