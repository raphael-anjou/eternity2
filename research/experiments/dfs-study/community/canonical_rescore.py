#!/usr/bin/env python3
"""Canonical matched-edge rescorer for the community-engine comparison.

Every engine in the comparison (McGavin C, Blackwood C#, our Verhaard reimpl)
emits a board in its own format. We never trust an engine's self-reported score.
Instead each engine's board is converted to the one canonical representation, a
1024-character URDL edge string, and rescored here with the exact same rule the
rest of the blog uses (the engine's bench-grid::score_cells / verify_bucas,
validated to reproduce the known 469 board): count the interior adjacencies whose
two touching half-edges share a NON-RIM colour. Two grey-rim ('a') half-edges
meeting in the interior do not count, so empty regions of a partial board and the
rim colour on a border tile both contribute nothing.

This is the linchpin of the comparison: one scorer, applied to every engine's
output, so a McGavin board and a Verhaard board are scored by identical code.

URDL edge string: 16*16 cells, row-major, four lowercase letters per cell in
up-right-down-left order. 'a' is the grey rim (colour 0), 'b','c',... interior
colours. This is the `board_edges` string the e2.bucas.name / eternity2.dev
viewer reads.
"""

W = H = 16
EDGES_LEN = W * H * 4
RIM = "a"  # colour 0, the grey border; a rim-vs-rim seam is NOT a match


def score_edges(edges: str) -> int:
    """Matched interior edges of a full 1024-char URDL edge string.

    This is THE blog canonical scorer (identical to the engine's
    bench-grid::score_cells and verify_bucas, validated to reproduce the known
    469 board): count an interior seam only when the two touching half-edges
    share a colour AND that colour is not the grey rim ('a'). Two rim edges
    meeting in the interior do not score, so an empty/all-rim region contributes
    nothing.
    """
    if len(edges) != EDGES_LEN:
        raise ValueError(f"edge string must be {EDGES_LEN} chars, got {len(edges)}")

    def cell(i: int) -> str:
        return edges[i * 4 : i * 4 + 4]

    matched = 0
    for r in range(H):
        for c in range(W):
            i = r * W + c
            if c + 1 < W:
                col = cell(i)[1]
                if col == cell(i + 1)[3] and col != RIM:  # right vs left
                    matched += 1
            if r + 1 < H:
                col = cell(i)[2]
                if col == cell(i + W)[0] and col != RIM:  # down vs up
                    matched += 1
    return matched


def edges_from_cells(cells) -> str:
    """Build the URDL edge string from a list of 256 (u,r,d,l) colour tuples.

    Colour 0 -> 'a', 1 -> 'b', ... Cells that are empty (None) become the rim
    letter 'a' on all four sides. Since rim==rim seams are excluded from the
    score, an empty region contributes no matched edge, so a collapsed or partial
    board earns exactly its placed-tile matched-edge count.
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
    """Matched interior edges of a 256-cell URDL-tuple board (None = empty).

    Unlike score_edges (which reads a flat 1024-char string and cannot tell an
    empty cell from a placed border tile whose edge is the grey rim), this counts
    a seam only when BOTH touching cells are placed. That is the correct rule for
    a PARTIAL board: an engine that stalled leaves an empty region, and two
    adjacent empty cells must not be credited as a matched 'a'=='a' seam. On a
    FULL board (no None) this is identical to score_edges.
    """
    if len(cells) != W * H:
        raise ValueError(f"need {W * H} cells, got {len(cells)}")
    matched = 0
    for r in range(H):
        for c in range(W):
            i = r * W + c
            cur = cells[i]
            if cur is None:
                continue
            if c + 1 < W and cells[i + 1] is not None:
                if cur[1] == cells[i + 1][3] and cur[1] != 0:  # right vs left
                    matched += 1
            if r + 1 < H and cells[i + W] is not None:
                if cur[2] == cells[i + W][0] and cur[2] != 0:  # down vs up
                    matched += 1
    return matched


# A known 469/480 board (bucas board_edges), taken from the engine's own
# scorer test (bench-grid `scorer_agrees_with_known_469`). Rescoring it here
# must return exactly 469: this pins our Python scorer to the blog's Rust one.
KNOWN_469_EDGES = (
    "adcaaendadweadhdafwdafgfaemfacqeaencafqeadofaepdaeueaeseadteaacdcnfansnnwuws"
    "hlsuwvvlgsgvmtrsqvgtntqvkkntokvkpllkulplsnhltovncacofqfangwqwwvgstgwvphtgnnpr"
    "qungqmqqkwhuwokvwlwlrqwpnqrhsknvpnscadpfvcawkrvvrnkglorhuqlnvhuusuvmonpwnspou"
    "qnlkvuqmokqhwmkrphlslrdadscpcarqvpntmqopstqrqphmorulwmnkolsntkqoqnuvgoosnqwso"
    "qphssoggkdadgcrfavmkrmupmsthuqqwtoriqwmmrorrmtrgrqrtrgtmrnlktogllsquggtnqdaft"
    "foeaknvopgtnhhlgwushiowumnlorkmngtrktlqtmnwlkounllwougolnkpgfackelfavhnltrwhl"
    "uqrsrtuwvlrlgvvmkigrwpkqlhwwokluisowquiooiqpprocaepfufanhhuwkuhqtjktpjtloupvm"
    "joijjmpppjhhrpkprhskppuvtkitvvrvmteabvfhcahwjhuhmwjnthjronuhhrjqphjskqpgmsrus"
    "grhouptwhtkmtvrtkmhmrbafhcsbajwgsmsowtjisoqtjhmkqpjnmkgrjmpqgsuvpoujuwovummso"
    "tlgmmtplfabtbteagiwtomniiwpmtwvwksmwnplsrilpqwoivkvwjhukvrkhsjprgwvjppvwbaepe"
    "ibawilinqwiphiqvjshmlljlkillqikovjqvijvuksikgikpnigvjnnvomjeabobjfalijjwgiiis"
    "jgsksslqjkiigqisoijuhsjosuslgoirilijurnvijmqlvbaeqfjbajrijingrjminsutmjtrugig"
    "tommihjumsiujggjiimhgujvmijpjlthjeaetbdaaidadgcadidactcadrbacgbabmdabufadubaf"
    "jbabhcabvbacpcabhbaceaab"
)


if __name__ == "__main__":
    import sys

    # 1. All-'a' board scores 0: every interior seam is a rim==rim match, which
    #    the canonical rule excludes.
    assert score_edges("a" * EDGES_LEN) == 0, "all-rim must score 0"

    # 2. A checker of two interior colours where no seam agrees -> 0.
    cells = []
    for r in range(H):
        for c in range(W):
            base = (r + c) % 2
            cells.append((base + 1, 2 - base, base + 1, 2 - base))
    assert score_edges(edges_from_cells(cells)) == 0, "checker must score 0"

    # 3. score_cells == score_edges on a full board, and empties never credited.
    lone = [None] * (W * H)
    lone[0] = (0, 5, 0, 0)
    assert score_cells(lone) == 0, "lone placed cell -> 0"
    pair = [None] * (W * H)
    pair[0] = (0, 5, 0, 0)  # right = 5 (interior colour)
    pair[1] = (0, 0, 0, 5)  # left  = 5 -> one interior match
    assert score_cells(pair) == 1, "matching interior pair -> 1"
    pair_rim = [None] * (W * H)
    pair_rim[0] = (0, 0, 0, 0)  # right = 0 (rim)
    pair_rim[1] = (0, 0, 0, 0)  # left  = 0 -> rim seam, NOT counted
    assert score_cells(pair_rim) == 0, "matching rim pair -> 0 (rim excluded)"

    # 4. THE authoritative check: the known 469 board rescoring to 469.
    assert score_edges(KNOWN_469_EDGES) == 469, (
        f"known 469 board rescored to {score_edges(KNOWN_469_EDGES)}, expected 469"
    )

    print("canonical_rescore self-test OK (all-rim=0, checker=0, pair=0/1, known-469=469)")
    sys.exit(0)
