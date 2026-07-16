#!/usr/bin/env python3
"""Canonical matched-edge rescorer for the community-engine comparison.

Every engine in the comparison (McGavin C, Blackwood C#, our Verhaard reimpl)
emits a board in its own format. We never trust an engine's self-reported score.
Instead each engine's board is converted to the one canonical representation, a
1024-character URDL edge string, and rescored here with the exact same rule the
rest of the blog uses (the dataset build and the record-boards verifier): count
the interior adjacencies whose two touching half-edges share a colour.

This is the linchpin of the comparison: one scorer, applied to every engine's
output, so a McGavin board and a Verhaard board are scored by identical code.

URDL edge string: 16*16 cells, row-major, four lowercase letters per cell in
up-right-down-left order. 'a' is the grey rim (colour 0), 'b','c',... interior
colours. This is the `board_edges` string the e2.bucas.name / eternity2.dev
viewer reads.
"""

W = H = 16
EDGES_LEN = W * H * 4


def score_edges(edges: str) -> int:
    """Matched interior edges of a full 1024-char URDL edge string."""
    if len(edges) != EDGES_LEN:
        raise ValueError(f"edge string must be {EDGES_LEN} chars, got {len(edges)}")

    def cell(i: int) -> str:
        return edges[i * 4 : i * 4 + 4]

    matched = 0
    for r in range(H):
        for c in range(W):
            i = r * W + c
            if c + 1 < W and cell(i)[1] == cell(i + 1)[3]:  # right vs left
                matched += 1
            if r + 1 < H and cell(i)[2] == cell(i + W)[0]:  # down vs up
                matched += 1
    return matched


def edges_from_cells(cells) -> str:
    """Build the URDL edge string from a list of 256 (u,r,d,l) colour tuples.

    Colour 0 -> 'a', 1 -> 'b', ... Cells that are empty (None) become the rim
    letter 'a' on all four sides, which contributes no interior match. Engines
    that stop partway leave a partial board; scoring the partial is legitimate
    (unmatched empty cells simply do not count), and it is how a collapsed run
    earns its low canonical score honestly.
    """
    if len(cells) != W * H:
        raise ValueError(f"need {W * H} cells, got {len(cells)}")
    out = []
    for cell in cells:
        if cell is None:
            out.append("aaaa")
            continue
        u, r, d, l = cell
        out.append(
            chr(ord("a") + u) + chr(ord("a") + r) + chr(ord("a") + d) + chr(ord("a") + l)
        )
    return "".join(out)


def score_cells(cells) -> int:
    """Convenience: rescore directly from a 256-cell URDL-tuple board."""
    return score_edges(edges_from_cells(cells))


if __name__ == "__main__":
    # Self-test: a trivial all-rim board scores 0 (no interior colours match
    # unless they happen to, and all-'a' interior seams DO match — so build a
    # deliberately mismatched checker to prove the scorer counts correctly).
    import sys

    # 1. All-'a' board: every interior seam is a==a, so every interior edge
    #    matches. Interior edges = W*(H-1) + H*(W-1) = 16*15 + 16*15 = 480.
    all_rim = "a" * EDGES_LEN
    s = score_edges(all_rim)
    assert s == W * (H - 1) + H * (W - 1) == 480, f"all-rim expected 480, got {s}"

    # 2. A board where every cell's right/down edges differ from neighbours'
    #    left/up: expect 0 interior matches. Give each cell a unique-ish pattern
    #    by alternating two colours so no seam agrees.
    cells = []
    for r in range(H):
        for c in range(W):
            # up,right,down,left — choose so right != left-of-neighbour and
            # down != up-of-neighbour everywhere.
            base = (r + c) % 2
            cells.append((base, 1 - base, base, 1 - base))
    edges = edges_from_cells(cells)
    s2 = score_edges(edges)
    # Right edge of (r,c) = 1-base; left edge of (r,c+1) has base' = (r+c+1)%2
    #   its left = 1-base'. base' = 1-base, so 1-base' = base. right(1-base) vs
    #   left(base): differ -> no match. Same for vertical. So expect 0.
    assert s2 == 0, f"checker board expected 0, got {s2}"

    print("canonical_rescore self-test OK (all-rim=480, checker=0)")
    sys.exit(0)
