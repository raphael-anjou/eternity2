#!/usr/bin/env python3
"""Write a McGavin .hnt from a dfs-study variant JSON.

McGavin reads `count` then `piece_1idx x y rot` per hint, where x = column+1 and
y = row+1 (1-indexed), decremented internally; rotation matches the variant's.
The variant stores hints as {pos, piece(0-idx), rot} with pos = row*16 + col.
"""
import json
import sys

def main(variant_json, out_hnt):
    v = json.load(open(variant_json))
    hints = v["hints"]
    lines = [str(len(hints))]
    for h in hints:
        pos = h["pos"]
        row, col = divmod(pos, 16)
        lines.append(f"{h['piece'] + 1} {col + 1} {row + 1} {h['rot']}")
    open(out_hnt, "w").write("\n".join(lines) + "\n")
    print(f"wrote {out_hnt} ({len(hints)} hints)")

if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
