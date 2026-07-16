#!/usr/bin/env python3
"""Parse Blackwood's best_board.txt into URDL cells.

best_board.txt is a 16x16 grid of `pieceId/rot` tokens (`---/-` = empty), and it
ends with a bucas viewer URL carrying the same board as a board_edges= string.

Blackwood numbers pieces in its own motif order (see the `puzzle=Joshua_Blackwood`
tag on the URL) and its rotation sense differs from the DFS-study piece file, so
rebuilding edges from pieceId+rot against variant_00.json's pieces does NOT line
up. The board_edges string the engine itself prints IS the canonical URDL board,
already in the exact alphabet the blog scores. So we take the edge COLOURS from
that string and take only the empty-cell MASK from the grid (`---/-`). That way a
partial or collapsed board is scored with the true placed-tile rule (empty cells
never credited) while the colours come straight from the engine's own output.
"""

import os
import re

from parse_bucas_url import cells_from_edges, edges_from_url

W = H = 16
HERE = os.path.dirname(os.path.abspath(__file__))


def empty_mask_from_grid(text):
    """256-bool mask, True where the grid token is `---/-` (empty)."""
    rows = []
    for ln in text.splitlines():
        toks = ln.split()
        if len(toks) == W and all(re.match(r"^(\d+|---)/(\d|-)$", t) for t in toks):
            rows.append(toks)
    if len(rows) < H:
        raise ValueError(f"parsed {len(rows)} board rows, need {H}")
    rows = rows[:H]
    mask = [False] * (W * H)
    for r in range(H):
        for c in range(W):
            pid = rows[r][c].split("/")[0]
            mask[r * W + c] = pid == "---"
    return mask


def parse_board_text(text):
    """Parse best_board.txt into 256 URDL cells (None for empty)."""
    edges = edges_from_url(text)
    mask = empty_mask_from_grid(text)
    return cells_from_edges(edges, empty_mask=mask)


def parse_file(path):
    with open(path, encoding="latin-1") as fh:
        return parse_board_text(fh.read())


if __name__ == "__main__":
    from canonical_rescore import score_cells

    sample = os.path.join(HERE, "engines", "blackwood", "best_board.txt")
    text = open(sample, encoding="latin-1").read()
    cells = parse_file(sample)
    placed = sum(1 for c in cells if c is not None)
    score = score_cells(cells)
    # Consistency: the number of empty grid tokens must equal 256 - placed.
    empties = sum(empty_mask_from_grid(text))
    assert empties + placed == W * H, f"{empties}+{placed} != {W*H}"
    print(f"parse_blackwood self-test: {placed} placed cells, canonical score {score}")
    assert placed > 0
