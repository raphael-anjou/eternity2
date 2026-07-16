#!/usr/bin/env python3
"""Parse a McGavin engine grid dump (print_state output) into URDL cells.

McGavin's engine prints its board with print_state_2 (genbody.c). For each row
of the board it emits three logical lines:

  north line : one %7d field per column = the NORTH edge colour of that cell
  main  line : leftrim, then per column [WEST edge %3d][piece+1 %4d], then the
               far EAST edge %3d of the last column
  (after the last row) a south line = the SOUTH edge of each bottom cell

Empty cells print as blanks. Colour 0 is the grey rim. Piece ids are printed
1-indexed (piece+1); we do not need them for scoring since the printed edge
colours already encode the placed rotation, but we return them for round-trip
checks.

We read each placed cell's NORTH (up) and WEST (left) colours from the dump and
derive RIGHT = the neighbour's WEST (or the printed far-east rim for the last
column) and DOWN = the next row's NORTH (or the printed south rim for the last
row). The result is a 256-entry list of (u,r,d,l) tuples, None for empty cells,
scored by the one canonical scorer. Matched-seam counting is invariant to a
global transpose, so McGavin's column-major internal frame needs no untangling.
"""

import re
import sys

W = H = 16


def _last_block(text: str) -> str:
    """Return the last full board block (between dashed separators)."""
    # print_state_2 ends each dump with a line of '----' repeated.
    blocks = re.split(r"-{16,}", text)
    # Trailing block after the final separator is usually empty/partial; the
    # last real board is the block just before the last separator.
    for b in reversed(blocks):
        if b.count("\n") >= H * 2:  # a full board is ~ 2 lines per row + trailer
            return b
    return blocks[-2] if len(blocks) >= 2 else text


def _num_or_none(field: str):
    field = field.strip()
    if field == "":
        return None
    try:
        return int(field)
    except ValueError:
        return None


def parse_dump(text: str):
    """Parse the last board dump in `text` into 256 URDL cells (None = empty)."""
    block = _last_block(text)
    # Strip ANSI escape sequences that the interactive display injects.
    block = re.sub(r"\x1b\[[0-9;]*[A-Za-z]", "", block)
    lines = block.split("\n")

    # A north line is 16 right-justified %7d fields (all digits/spaces, no other
    # token) and is immediately followed by a main line beginning with a west
    # rim field then piece ids. We locate the board by finding the run of
    # alternating (north, main) pairs. The most robust key: main lines contain
    # many numbers and are longer; north lines are pure %7d columns.
    #
    # Fixed-width layout of the MAIN line:
    #   col 0..2  : west rim (%3d)  -> seam left of cell 0
    #   then per cell y: [%4d piece][%3d west-of-next]  (7 chars/cell)
    #   final     : %3d east rim of last cell
    # Fixed-width layout of the NORTH line: 16 x %7d.

    n = len(lines)

    def parse_north(ln):
        # 16 fields of width 7; a north line is nothing but digits and spaces.
        if len(ln) < 7:
            return None
        if re.search(r"[^0-9 ]", ln):
            return None
        vals = []
        for k in range(W):
            seg = ln[k * 7 : k * 7 + 7]
            vals.append(_num_or_none(seg))
        # a valid north line has at least one number and no stray trailing token
        if all(v is None for v in vals):
            return None
        rest = ln[W * 7 :].strip()
        if rest != "":
            return None
        return vals

    def parse_main(ln):
        # width-3 west rim, then 16 * (width-4 piece, width-3 west), + width-3 east
        # total = 3 + 16*7 + 3 = 118. Blank (all-space) main lines parse to all
        # None, which is a legitimate empty board row on a partial dump.
        if re.search(r"[^0-9 ]", ln):
            return None
        wests = []
        pieces = []
        pos = 0
        wests.append(_num_or_none(ln[pos : pos + 3]))
        pos += 3
        east = None
        for y in range(W):
            p = _num_or_none(ln[pos : pos + 4])
            pos += 4
            w_next = _num_or_none(ln[pos : pos + 3])
            pos += 3
            pieces.append(p)
            if y < W - 1:
                wests.append(w_next)
            else:
                east = w_next
        return wests, pieces, east

    # Anchor the board: it begins on the first line after the "Solutions:" /
    # "Tiles placed:" status line (or, lacking that, the first north-formatted
    # line). From the anchor we read exactly H row-pairs positionally so that
    # blank rows of a partial board keep their row index instead of collapsing.
    start = None
    for idx, ln in enumerate(lines):
        if "Tiles placed" in ln or "Solutions:" in ln:
            start = idx + 1
    if start is None:
        for idx, ln in enumerate(lines):
            if parse_north(ln) is not None:
                start = idx
                break
    if start is None:
        raise ValueError("no board dump found (no status line, no north row)")

    north_rows = []
    west_rows = []
    piece_rows = []
    east_rim = []
    for r in range(H):
        n_idx = start + r * 2
        m_idx = n_idx + 1
        north = parse_north(lines[n_idx]) if n_idx < n else None
        main = parse_main(lines[m_idx]) if m_idx < n else None
        if north is None:
            north = [None] * W
        if main is None:
            main = ([None] * W, [None] * W, None)
        north_rows.append(north)
        west_rows.append(main[0])
        piece_rows.append(main[1])
        east_rim.append(main[2])

    # South rim of the bottom row: the %7d line just after the 16th row-pair.
    south_vals = [None] * W
    s_idx = start + H * 2
    if s_idx < n:
        cand = parse_north(lines[s_idx])
        if cand is not None:
            south_vals = cand

    cells = [None] * (W * H)
    for r in range(H):
        for c in range(W):
            piece = piece_rows[r][c]
            if piece is None:
                continue
            u = north_rows[r][c]
            l = west_rows[r][c]
            # right seam = west of next cell, or east rim for last col
            if c + 1 < W:
                right = west_rows[r][c + 1]
            else:
                right = east_rim[r]
            # down seam = north of next row's cell, or south rim for last row
            if r + 1 < H:
                down = north_rows[r + 1][c]
            else:
                down = south_vals[c]
            # A placed cell should have all four edges; if a neighbour is empty
            # the seam colour may be missing (None) -> treat as rim 0 so it
            # simply does not match, giving the true partial-board score.
            u = 0 if u is None else u
            l = 0 if l is None else l
            right = 0 if right is None else right
            down = 0 if down is None else down
            cells[r * W + c] = (u, right, down, l)
    return cells


def parse_file(path: str):
    with open(path, encoding="latin-1") as fh:
        return parse_dump(fh.read())


if __name__ == "__main__":
    import os
    from canonical_rescore import score_cells

    here = os.path.dirname(os.path.abspath(__file__))
    # Round-trip self-test against a committed sample run log.
    sample = os.path.join(here, "engines", "mcgavin", "sample_dump.log")
    if not os.path.exists(sample):
        # fall back to the transient scratchpad if the committed sample is not
        # present yet (test still works during development).
        sample = sys.argv[1] if len(sys.argv) > 1 else sample
    cells = parse_file(sample)
    placed = sum(1 for c in cells if c is not None)
    score = score_cells(cells)
    print(f"parse_mcgavin self-test: {placed} placed cells, canonical score {score}")
    assert placed > 0, "no cells parsed"
