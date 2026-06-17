#!/usr/bin/env python3
"""The Eternity II backtracking solver, authored against the bfc API and
compiled to solver.bf.

Same explicit-stack depth-first search as ../../engine/src/solver.rs, sized for
small boards so it actually runs in interpreted Brainfuck. Row-major visit
order, no hints (generated puzzles carry none).

Table building and I/O are unrolled over the fixed board size N (known at
compile time). The candidate scan and the backtrack are genuine runtime loops
(they iterate over a cursor / depth that is only known at run time), expressed
with the tested while/if/array primitives.

Input  (stdin, raw bytes): SIZE byte, then N pieces * 4 edge bytes (URDL).
Output (stdout, raw bytes): the solved board, one byte per cell = piece*4+rot.

Build:  python3 build.py            # writes solver.bf
Run:    python3 bf.py solver.bf < data/bf_3_3_2.bin
"""

import os

from bfc import BF

# Board edge length this build targets. Override with E2_BF_SIZE (build.py
# passes it through) to compile a solver for a different small board. 3 and 4
# finish quickly; 5 runs but takes longer (the linear-scan array access and the
# search both grow with the board).
SIZE = int(os.environ.get("E2_BF_SIZE", "3"))
N = SIZE * SIZE
NROWS = N * 4

# rotated(URDL, r): per-rotation edge order (see types.rs / eternity2.lua).
#   r=0: u r d l   r=1: l u r d   r=2: d l u r   r=3: r d l u
_ROT = {
    0: ("eu", "er", "ed", "el"),
    1: ("el", "eu", "er", "ed"),
    2: ("ed", "el", "eu", "er"),
    3: ("er", "ed", "el", "eu"),
}


def build() -> str:
    b = BF()

    b.reg("n", "nrows")
    b.reg("i", "tmp", "tmp2", "tmp3")
    b.reg("depth", "pos", "cursor", "limit", "placed_row", "found")
    b.reg("oi", "rr", "pid", "row", "rowp1", "used_f", "dist_f")
    b.reg("x", "y", "fits", "nbrow", "nb")
    b.reg("eu", "er", "ed", "el")
    b.reg("cu", "cr", "cd", "cl")            # candidate (placed) edges
    b.reg("byte", "running", "is_solved", "dcmp")
    b.reg("k4", "kmod", "size_m1")

    b.array("PIECE", N * 4)
    # Rotated-edge table, split by direction into four NROWS-cell arrays (TU/TR/
    # TD/TL) instead of one NROWS*4 array. A runtime `aload` costs O(array
    # length), and `_check_fit` is almost entirely TABLE reads, so quartering
    # each array's length is a ~4x speedup on the hot path.
    b.array("TU", NROWS)
    b.array("TR", NROWS)
    b.array("TD", NROWS)
    b.array("TL", NROWS)
    b.array("DISTINCT", NROWS)
    b.array("BOARD", N)
    b.array("USED", N)
    b.array("FCURSOR", N)
    b.array("FPLACED", N)

    b.set("n", N)
    b.set("nrows", NROWS)
    b.set("size_m1", SIZE - 1)

    # --- 1. read SIZE (ignored; fixed build) then N*4 piece edges ---
    b.read("byte")
    for idx in range(N * 4):
        b.read("byte")
        b.ainit_const("PIECE", idx, "byte", "tmp")

    # --- 2. build TABLE + DISTINCT (unrolled; all indices compile-time) ---
    for p in range(N):
        # load base edges (constant index -> direct cell read, no scan)
        for e, name in enumerate(("eu", "er", "ed", "el")):
            b.aget_const("PIECE", p * 4 + e, name, "tmp")
        for r in range(4):
            row = p * 4 + r
            ru, rr_, rd, rl = _ROT[r]
            # store rotated edges into the four per-direction tables at [row]
            for arr, src in (("TU", ru), ("TR", rr_), ("TD", rd), ("TL", rl)):
                b.copy(src, "byte", "tmp")
                b.ainit_const(arr, row, "byte", "tmp")
            # DISTINCT[row] = 0 if this rotation duplicates a lower one of p
            b.set("dist_f", 1)
            for prev in range(r):
                _mark_dup_if_equal(b, row, p * 4 + prev)
            b.copy("dist_f", "byte", "tmp")
            b.ainit_const("DISTINCT", row, "byte", "tmp")

    # --- 3. init search state ---
    b.set("byte", 0)
    for c in range(N):
        b.ainit_const("BOARD", c, "byte", "tmp")
        b.ainit_const("USED", c, "byte", "tmp")
        b.ainit_const("FCURSOR", c, "byte", "tmp")
        b.ainit_const("FPLACED", c, "byte", "tmp")
    b.set("depth", 0)
    b.set("running", 1)
    b.set("is_solved", 0)

    # --- 4. DFS loop ---
    # E2_BF_TRACE=1 emits one byte = (depth + 1) after every step, so the
    # search's progress (and any oscillation) is visible on stdout. Off by
    # default so normal output is just the board.
    trace = os.environ.get("E2_BF_TRACE") == "1"
    with b.while_nz("running"):
        b.eq("depth", "n", "dcmp", "tmp")
        with b.ifnz_copy("dcmp"):
            b.set("is_solved", 1)
            b.set("running", 0)
        b.copy("running", "tmp2", "tmp")
        with b.ifnz_copy("tmp2"):
            _dfs_step(b)
            if trace:
                b.copy("depth", "byte", "tmp")
                b.inc("byte")
                b.write("byte")

    # --- 5. output solved board (row = BOARD[cell] - 1) ---
    for c in range(N):
        b.set("i", c)
        b.aload("BOARD", "i", "byte")
        b.dec("byte")
        b.write("byte")

    return b.code()


def _mark_dup_if_equal(b: BF, row: int, prow: int) -> None:
    """If TABLE[row] equals TABLE[prow] (all 4 edges), set dist_f = 0."""
    for r in ("_du_all", "_du_eq", "_du_a", "_du_b", "_du_s"):
        if r not in b.off:
            b.reg(r)
    b.set("_du_all", 1)                      # running "all four edges equal"
    for e in range(4):
        arr = ("TU", "TR", "TD", "TL")[e]
        b.aget_const(arr, row, "_du_a", "_du_s")
        b.aget_const(arr, prow, "_du_b", "_du_s")
        b.eq("_du_a", "_du_b", "_du_eq", "_du_s")
        _and_into_named(b, "_du_all", "_du_eq", "_du_s")
    with b.ifnz_copy("_du_all"):
        b.set("dist_f", 0)


def _and_into_named(b: BF, acc: str, x: str, scratch: str) -> None:
    """acc = acc AND x (acc,x in {0,1}); x preserved; `scratch` is private to
    the caller so this never collides with the surrounding computation."""
    for r in (scratch, scratch + "_z"):
        if r not in b.off:
            b.reg(r)
    b.copy(x, scratch, scratch + "_z")
    b.logical_not(scratch, scratch + "_z")  # scratch_z = (x == 0)
    with b.ifnz_copy(scratch + "_z"):
        b.set(acc, 0)


def _dfs_step(b: BF) -> None:
    """One placement-or-backtrack, mirroring solver.rs step() body once."""
    # pos = FCURSOR is per-depth; pos = path[depth] = depth (row-major).
    b.copy("depth", "pos", "tmp")           # row-major: cell index == depth
    b.aload("FCURSOR", "depth", "cursor")
    b.copy("nrows", "limit", "tmp")
    b.set("found", 0)
    b.set("placed_row", 0)

    # while (cursor < limit) and (found == 0): scan one candidate.
    # Dedicated scratch regs (nothing else touches these) so the loop-condition
    # computation cannot be clobbered by the helpers it calls.
    for r in ("scan", "_sc_lt", "_sc_nf", "_sc_fc"):
        if r not in b.off:
            b.reg(r)
    b.set("scan", 1)
    with b.while_nz("scan"):
        _lt(b, "cursor", "limit", "_sc_lt")  # _sc_lt = (cursor < limit)
        b.copy("found", "_sc_fc", "_sc_nf")
        b.logical_not("_sc_fc", "_sc_nf")    # _sc_nf = (found == 0)
        # scan = _sc_lt AND _sc_nf
        b.copy("_sc_lt", "scan", "_sc_fc")
        _and_into_named(b, "scan", "_sc_nf", "_sc_fc")
        with b.ifnz_copy("scan"):
            _scan_one(b)

    # if found: place; else backtrack
    b.copy("found", "tmp2", "tmp")
    with b.ifnz_copy("tmp2"):
        _place(b)
    b.copy("found", "tmp2", "tmp")
    b.logical_not("tmp2", "tmp")
    with b.ifnz_copy("tmp"):
        _backtrack(b)


def _scan_one(b: BF) -> None:
    """Examine candidate at `cursor`: oi=cursor/4, rr=cursor%4, pid=oi
    (identity piece order). Skip used pieces and non-distinct rotations;
    on a fit set found=1, placed_row=row+1."""
    _divmod4(b, "cursor", "oi", "rr")
    b.copy("oi", "pid", "tmp")              # piece order = identity
    # advance cursor by 1 (default)
    b.inc("cursor")
    # if USED[pid]: skip rest of this piece's rotations: cursor = (cursor+3)&~3
    b.aload("USED", "pid", "used_f")
    with b.ifnz_copy("used_f"):
        _align4_up(b, "cursor")
    # else examine row = pid*4 + rr
    b.copy("used_f", "tmp2", "tmp")
    b.logical_not("tmp2", "tmp")            # tmp = (not used)
    with b.ifnz_copy("tmp"):
        _row_from(b, "pid", "rr", "row")    # row = pid*4 + rr
        # Stash row+1 NOW: _check_fit clobbers `row` (its neighbour scan reuses
        # the register), so we must capture the candidate before checking fit.
        if "_so_rp1" not in b.off:
            b.reg("_so_rp1")
        b.copy("row", "_so_rp1", "tmp")
        b.inc("_so_rp1")                     # _so_rp1 = row + 1
        b.aload("DISTINCT", "row", "dist_f")
        with b.ifnz_copy("dist_f"):
            _check_fit(b)                    # sets fits (clobbers `row`)
            with b.ifnz_copy("fits"):
                b.set("found", 1)
                b.copy("_so_rp1", "placed_row", "tmp")


def _check_fit(b: BF) -> None:
    """Does TABLE[row] fit at pos? rim/interior + placed neighbors. fits=0/1."""
    # candidate edges
    for off, name in enumerate(("cu", "cr", "cd", "cl")):
        _table_edge(b, "row", off, name)
    _xy(b, "pos", "x", "y")
    b.set("fits", 1)
    # rim/interior: edge==0 iff on that border.
    #   top border iff y==0 ; need cu==0 iff y==0
    _border_check(b, "y", 0, "cu")          # y==0 <-> cu==0
    _border_check(b, "y", SIZE - 1, "cd")   # y==size-1 <-> cd==0
    _border_check(b, "x", 0, "cl")          # x==0 <-> cl==0
    _border_check(b, "x", SIZE - 1, "cr")   # x==size-1 <-> cr==0
    # neighbor matches (only when that neighbor exists and is placed)
    # above: y>0 ; neighbor cell pos-size ; its down (edge2) must == cu
    _neighbor_check(b, "y", 0, "above")
    _neighbor_check(b, "y", SIZE - 1, "below")
    _neighbor_check(b, "x", 0, "left")
    _neighbor_check(b, "x", SIZE - 1, "right")


def _place(b: BF) -> None:
    b.copy("placed_row", "rowp1", "tmp")    # row+1
    b.copy("rowp1", "byte", "tmp")
    b.astore("BOARD", "pos", "byte")        # BOARD[pos] = row+1
    # pid = (row)/4 where row = rowp1-1
    b.copy("rowp1", "row", "tmp"); b.dec("row")
    _div4(b, "row", "pid")
    b.set("byte", 1)
    b.astore("USED", "pid", "byte")
    # FCURSOR[depth] = cursor ; FPLACED[depth] = rowp1
    b.copy("cursor", "byte", "tmp"); b.astore("FCURSOR", "depth", "byte")
    b.copy("rowp1", "byte", "tmp"); b.astore("FPLACED", "depth", "byte")
    b.inc("depth")


def _backtrack(b: BF) -> None:
    # FCURSOR[depth] = 0
    b.set("byte", 0); b.astore("FCURSOR", "depth", "byte")
    # if depth == 0 -> exhausted (running=0). else step back.
    b.reg("dz") if "dz" not in b.off else None
    b.copy("depth", "tmp2", "tmp")
    b.logical_not("tmp2", "dz")             # dz = (depth==0)
    with b.ifnz_copy("dz"):
        b.set("running", 0)
    b.copy("dz", "tmp2", "tmp")
    b.logical_not("tmp2", "tmp")            # tmp = (depth!=0)
    with b.ifnz_copy("tmp"):
        b.dec("depth")
        # pos_prev = depth (row-major)
        b.copy("depth", "pos", "tmp")
        b.aload("FPLACED", "depth", "rowp1")  # row+1 placed there
        b.set("byte", 0); b.astore("FPLACED", "depth", "byte")
        b.copy("rowp1", "row", "tmp"); b.dec("row")
        _div4(b, "row", "pid")
        b.set("byte", 0); b.astore("BOARD", "pos", "byte")
        b.set("byte", 0); b.astore("USED", "pid", "byte")


# ---------- arithmetic / comparison helpers (all 8-bit, small values) -------

def _divmod4(b: BF, src: str, q: str, rmod: str) -> None:
    """q = src // 4, rmod = src % 4 (src preserved, src < 256). Uses only
    private scratch so it never collides with the caller's q/rmod choice."""
    for r in ("_dm_w", "_dm_c", "_dm_z"):
        if r not in b.off:
            b.reg(r)
    b.copy(src, "_dm_w", "_dm_c")           # work copy of src
    b.set(q, 0); b.set(rmod, 0)
    with b.while_nz("_dm_w"):
        b.dec("_dm_w")
        b.inc(rmod)
        # if rmod == 4: rmod = 0, q += 1
        b.copy(rmod, "_dm_c", "_dm_z")
        b.dec("_dm_c", 4)
        b.logical_not("_dm_c", "_dm_z")     # _dm_z = (rmod==4)
        with b.ifnz_copy("_dm_z"):
            b.set(rmod, 0)
            b.inc(q)


def _div4(b: BF, src: str, q: str) -> None:
    _divmod4(b, src, q, "byte")             # byte clobbered as remainder sink


def _align4_up(b: BF, name: str) -> None:
    """name = ((name + 3) // 4) * 4. (round up to multiple of 4)"""
    for r in ("_al_q", "_al_r"):
        if r not in b.off:
            b.reg(r)
    b.inc(name, 3)
    _divmod4(b, name, "_al_q", "_al_r")     # _al_q = (name+3)//4
    # name = _al_q * 4 = _al_q + _al_q*3
    b.copy("_al_q", name, "tmp")
    b.addto(name, "_al_q", "tmp")
    b.addto(name, "_al_q", "tmp")
    b.addto(name, "_al_q", "tmp")


def _row_from(b: BF, pid: str, rmod: str, dst: str) -> None:
    """dst = pid*4 + rmod."""
    b.copy(pid, dst, "tmp")
    # dst *= 4
    b.copy(dst, "tmp2", "tmp")
    b.addto(dst, "tmp2", "tmp"); b.addto(dst, "tmp2", "tmp"); b.addto(dst, "tmp2", "tmp")
    b.addto(dst, rmod, "tmp")


def _lt(b: BF, a: str, c: str, dst: str) -> None:
    """dst = (a < c), a,c preserved, small non-negative values.

    Decrement copies of a and c together. The first to reach 0 decides:
    if `ta` empties while `tc` is still non-zero, a < c. We stop the instant
    either hits 0 using a single `run` guard that the loop body lowers."""
    for r in ("_lta", "_ltc", "_ltrun", "_ltz"):
        if r not in b.off:
            b.reg(r)
    b.copy(a, "_lta", "tmp")
    b.copy(c, "_ltc", "tmp")
    b.set(dst, 0)
    b.set("_ltrun", 1)
    with b.while_nz("_ltrun"):
        # za = (ta == 0)?
        b.copy("_lta", "tmp2", "tmp")
        b.logical_not("tmp2", "_ltz")       # _ltz = (ta==0)
        with b.ifnz_copy("_ltz"):
            # ta==0: a<c iff tc!=0
            b.copy("_ltc", "tmp2", "tmp")
            b.boolify("tmp2", dst)
            b.set("_ltrun", 0)
        # still running? then check tc==0 (=> a>=c, dst stays 0, stop)
        b.copy("_ltrun", "tmp3", "tmp")
        with b.ifnz_copy("tmp3"):
            b.copy("_ltc", "tmp2", "tmp")
            b.logical_not("tmp2", "_ltz")   # _ltz = (tc==0)
            with b.ifnz_copy("_ltz"):
                b.set("_ltrun", 0)
        # still running? decrement both and loop
        b.copy("_ltrun", "tmp3", "tmp")
        with b.ifnz_copy("tmp3"):
            b.dec("_lta")
            b.dec("_ltc")


def _xy(b: BF, src: str, xo: str, yo: str) -> None:
    """xo = src % size, yo = src // size."""
    b.copy(src, "tmp", "tmp2")
    b.set(xo, 0); b.set(yo, 0)
    b.reg("_xyf") if "_xyf" not in b.off else None
    with b.while_nz("tmp"):
        b.dec("tmp"); b.inc(xo)
        b.copy(xo, "tmp2", "tmp3")
        b.dec("tmp2", SIZE)
        b.logical_not("tmp2", "_xyf")       # _xyf = (xo==size)
        with b.ifnz_copy("_xyf"):
            b.set(xo, 0); b.inc(yo)


def _table_edge(b: BF, row: str, off: int, dst: str) -> None:
    """dst = edge `off` (0=U,1=R,2=D,3=L) of table row `row`. `off` is always a
    compile-time constant, so this is a single direct-array `aload` over the
    quartered per-direction table — no index arithmetic, 4x cheaper than the
    old combined TABLE."""
    arr = ("TU", "TR", "TD", "TL")[off]
    b.aload(arr, row, dst)


def _border_check(b: BF, coord: str, edge_val: int, cedge: str) -> None:
    """If (coord==edge_val) XOR (cedge==0) -> fits=0. Border rule."""
    b.reg("_oncoord") if "_oncoord" not in b.off else None
    b.reg("_isbord") if "_isbord" not in b.off else None
    # on = (coord == edge_val)
    b.copy(coord, "tmp2", "tmp3")
    b.dec("tmp2", edge_val)
    b.logical_not("tmp2", "_oncoord")
    # isbord = (cedge == 0)
    b.copy(cedge, "tmp2", "tmp3")
    b.logical_not("tmp2", "_isbord")
    # mismatch = on != isbord  -> if differ, fits=0
    b.reg("_mm") if "_mm" not in b.off else None
    b.eq("_oncoord", "_isbord", "_mm", "tmp3")
    b.logical_not("_mm", "tmp")             # tmp = (they differ)
    with b.ifnz_copy("tmp"):
        b.set("fits", 0)


def _neighbor_check(b: BF, coord: str, edge_val: int, side: str) -> None:
    """If a neighbor on `side` exists (coord != edge_val) and is placed, its
    facing edge must equal our facing edge; else fits=0.
      above: neighbor pos-size, its edge2(down) vs cu
      below: neighbor pos+size, its edge0(up)   vs cd
      left : neighbor pos-1,    its edge1(right)vs cl
      right: neighbor pos+1,    its edge3(left) vs cr
    """
    spec = {
        "above": (-SIZE, 2, "cu"),
        "below": (+SIZE, 0, "cd"),
        "left":  (-1, 1, "cl"),
        "right": (+1, 3, "cr"),
    }[side]
    delta, nedge, myedge = spec
    b.reg("_exists") if "_exists" not in b.off else None
    # exists = (coord != edge_val)
    b.copy(coord, "tmp2", "tmp3")
    b.dec("tmp2", edge_val)
    b.boolify("tmp2", "_exists")
    with b.ifnz_copy("_exists"):
        # nb = pos + delta
        b.copy("pos", "nb", "tmp")
        if delta >= 0:
            b.inc("nb", delta)
        else:
            b.dec("nb", -delta)
        b.aload("BOARD", "nb", "nbrow")     # row+1 or 0
        with b.ifnz_copy("nbrow"):
            # neighbor placed: compare TABLE[(nbrow-1)*4 + nedge] vs myedge
            b.copy("nbrow", "row", "tmp"); b.dec("row")
            _table_edge(b, "row", nedge, "byte")
            b.reg("_neq") if "_neq" not in b.off else None
            b.eq("byte", myedge, "_neq", "tmp3")
            b.logical_not("_neq", "tmp")    # tmp = (not equal)
            with b.ifnz_copy("tmp"):
                b.set("fits", 0)
            # restore row register isn't needed (recomputed each use)


if __name__ == "__main__":
    print(build())
