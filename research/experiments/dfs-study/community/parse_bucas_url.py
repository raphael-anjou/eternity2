#!/usr/bin/env python3
"""Parse a bucas viewer URL (board_edges=...) into URDL cells.

The e2.bucas.name / eternity2.dev viewer encodes a board as board_edges=, a run
of lowercase letters, four per cell, row-major, in up/right/down/left order.
'a' is colour 0 (the grey rim). This is already the canonical representation the
blog scores, so parsing is a direct letter->int decode.

Used for Verhaard, whose original Win32 engine cannot run on this machine: its
best_so_far.url is the only artefact it leaves, and it is a bucas URL.

An `empty_mask` may be supplied (list of 256 bools, True = empty) so that a
partial board does not credit empty-empty 'a'=='a' seams; without it every cell
is treated as placed, which is correct for a full solved board.
"""

import re

W = H = 16


def edges_from_url(url_or_text):
    m = re.search(r"board_edges=([A-Za-z]+)", url_or_text)
    if not m:
        raise ValueError("no board_edges= found")
    edges = m.group(1).lower()[: W * H * 4]
    if len(edges) != W * H * 4:
        raise ValueError(f"board_edges too short: {len(edges)} chars")
    return edges


def cells_from_edges(edges, empty_mask=None):
    """Decode a 1024-char URDL string to 256 (u,r,d,l) int tuples (None=empty)."""
    cells = []
    for i in range(W * H):
        if empty_mask is not None and empty_mask[i]:
            cells.append(None)
            continue
        quad = edges[i * 4 : i * 4 + 4]
        cells.append(tuple(ord(ch) - ord("a") for ch in quad))
    return cells


def parse_url(url_or_text, empty_mask=None):
    return cells_from_edges(edges_from_url(url_or_text), empty_mask)


def parse_file(path, empty_mask=None):
    with open(path, encoding="latin-1") as fh:
        return parse_url(fh.read(), empty_mask)


if __name__ == "__main__":
    import os
    from canonical_rescore import score_cells, score_edges

    here = os.path.dirname(os.path.abspath(__file__))
    sample = os.path.join(here, "engines", "verhaard", "best_so_far.url")
    text = open(sample, encoding="latin-1").read()
    edges = edges_from_url(text)
    cells = parse_url(text)  # full board (Verhaard reports a complete board)
    placed = sum(1 for c in cells if c is not None)
    s_cells = score_cells(cells)
    s_edges = score_edges(edges)
    print(
        f"parse_bucas_url self-test: {placed} placed cells, "
        f"score_cells={s_cells}, score_edges={s_edges}"
    )
    assert s_cells == s_edges, "full-board score_cells must equal score_edges"
