"""Eternity II engine — a faithful, pure-Python ISO port of the Rust engine.

This is one of several from-scratch re-implementations of the website's engine.
It is a *literal* translation of the algorithm described, once and for all, in
``engine-side-quests/ALGORITHM.md`` — read that document first; every function
here points back to a section of it. The canonical source it mirrors is the Rust
crate in ``engine/src/``.

Nothing here is clever. It is the same xorshift RNG, the same generator, the same
nine cell-visit orders, and the same explicit-stack backtracking solver, named
the same way, in the same order. It is validated byte-for-byte against the Rust
engine's golden output (see ``spec.py`` / ``golden.txt``), down to the exact
node / attempt / backtrack counts.

Conventions (ALGORITHM.md §2), shared by every port:
  * Edges are URDL: ``edges = (up, right, down, left)``.
  * Colour 0 is the grey border; interior colours are 1..=22.
  * Rotation ``r`` is clockwise quarter-turns.
  * A board cell holds ``piece*4 + rotation`` (>=0) or ``-1`` when empty;
    cells are row-major, ``index = y*width + x``.

No third-party dependencies. Pure standard library, runs on CPython 3.9+.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Optional

# --- Type aliases (purely for readability) ----------------------------------
# An edge tuple is always 4 colours in URDL order.
Edges = tuple[int, int, int, int]

BORDER = 0  # the grey rim colour (ALGORITHM.md §2.2)

# 64-bit mask: Python ints are unbounded, so every wrapping 64-bit operation is
# followed by ``& U64`` to emulate the Rust ``u64`` overflow exactly.
U64 = 0xFFFFFFFFFFFFFFFF
U32 = 0xFFFFFFFF


# ============================================================================
# §2.3  Rotation
# ============================================================================
def rotated(e: Edges, r: int) -> Edges:
    """Rotate a URDL edge tuple clockwise by ``r`` quarter-turns.

    new[i] = old[(i + 4 - r) % 4].  Rotation 1 = (l, u, r, d): the edge that
    faced up now faces right.  See ALGORITHM.md §2.3.
    """
    r &= 3
    return (e[(4 - r) & 3], e[(5 - r) & 3], e[(6 - r) & 3], e[(7 - r) & 3])


def canonical(e: Edges) -> Edges:
    """Lexicographically minimal rotation — bucas's rotation-independent id."""
    best = e
    for r in range(1, 4):
        c = rotated(e, r)
        if c < best:
            best = c
    return best


# ============================================================================
# §3  The random number generator (XorShift64 + splitmix64 seeding)
# ============================================================================
class XorShift:
    """The engine's own deterministic RNG (ALGORITHM.md §3).

    We carry our own generator so that ``(size, colors, seed)`` reproduces the
    *exact* same puzzle in every language. All arithmetic is unsigned 64-bit
    with wraparound, emulated here with ``& U64``.
    """

    __slots__ = ("state",)

    def __init__(self, seed: int) -> None:
        # §3.1 — one round of splitmix64 turns a weak 32-bit seed into a good
        # 64-bit state. The final ``| 1`` forbids the all-zero state (xorshift
        # is stuck at zero).
        z = (seed + 0x9E3779B97F4A7C15) & U64
        z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & U64
        z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & U64
        z = z ^ (z >> 31)
        self.state = (z | 1) & U64

    def next_u32(self) -> int:
        """§3.2 — one xorshift64 step; return the high 32 bits."""
        x = self.state
        x ^= (x << 13) & U64
        x ^= x >> 7
        x ^= (x << 17) & U64
        self.state = x & U64
        return (x >> 32) & U32

    def below(self, n: int) -> int:
        """Uniform in [0, n) — ``next_u32() % n`` (ALGORITHM.md §3.3)."""
        return self.next_u32() % n

    def shuffle(self, lst: list) -> None:
        """In-place Fisher–Yates, descending, swapping i with below(i+1).

        The direction and call order are part of the contract: changing either
        yields a different permutation and breaks cross-language parity.
        """
        for i in range(len(lst) - 1, 0, -1):
            j = self.below(i + 1)
            lst[i], lst[j] = lst[j], lst[i]


# ============================================================================
# §1 / §9  The Puzzle value type
# ============================================================================
@dataclass
class Hint:
    pos: int
    piece: int
    rot: int


@dataclass
class Puzzle:
    name: str
    width: int
    height: int
    num_colors: int  # interior colours, border (0) excluded
    pieces: list[Edges]  # edges in rotation 0, URDL
    hints: list[Hint] = field(default_factory=list)

    def cell_count(self) -> int:
        return self.width * self.height

    def max_score(self) -> int:
        """Every interior edge matched: 2*w*h - w - h (480 for 16x16)."""
        w, h = self.width, self.height
        return 2 * w * h - w - h


def max_score(width: int, height: int) -> int:
    return 2 * width * height - width - height


# ============================================================================
# §4  Generating a solvable puzzle
# ============================================================================
def interior_edge_count(size: int) -> int:
    """Interior (shared) edges of an n x n board: 2*n*(n-1)."""
    return 2 * size * (size - 1)


def max_colors(size: int) -> int:
    """Largest usable colour count: bounded by interior edges and 22 motifs."""
    return min(interior_edge_count(size), 22)


def generate_solved(size: int, colors: int, seed: int) -> Puzzle:
    """A solvable puzzle in *solution order*: piece i sits at cell i, rot 0, so
    the identity board is a perfect solution.  ALGORITHM.md §4 steps 1-3.
    """
    assert size >= 2, "size must be >= 2"
    colors = min(max(colors, 1), max_colors(size))
    s = size
    n_edges = interior_edge_count(size)
    rng = XorShift(seed)

    # Step 1 — paint one colour per interior adjacency; the first ``colors``
    # entries guarantee every colour appears at least once; then shuffle.
    palette = [
        (i + 1) if i < colors else (rng.below(colors) + 1) for i in range(n_edges)
    ]
    rng.shuffle(palette)

    # Step 2 — split into vertical / horizontal adjacency colours.
    #   vert[y*(s-1)+x]  = colour between (x,y) and (x+1,y)
    #   horiz[y*s+x]     = colour between (x,y) and (x,y+1)
    vert = palette[: s * (s - 1)]
    horiz = palette[s * (s - 1) :]

    def v(x: int, y: int) -> int:
        return vert[y * (s - 1) + x]

    def h(x: int, y: int) -> int:
        return horiz[y * s + x]

    # Step 3 — derive each piece from its four surrounding adjacencies; rim
    # sides are the border colour.
    pieces: list[Edges] = []
    for y in range(s):
        for x in range(s):
            up = BORDER if y == 0 else h(x, y - 1)
            down = BORDER if y == s - 1 else h(x, y)
            left = BORDER if x == 0 else v(x - 1, y)
            right = BORDER if x == s - 1 else v(x, y)
            pieces.append((up, right, down, left))

    return Puzzle(
        name=f"generated_{size}x{size}_c{colors}_s{seed}",
        width=size,
        height=size,
        num_colors=colors,
        pieces=pieces,
        hints=[],
    )


def generate(size: int, colors: int, seed: int) -> Puzzle:
    """A scrambled solvable puzzle (ALGORITHM.md §4 step 4).

    Build the solved puzzle, then a *second* RNG seeded with ``seed ^
    0xA5A5A5A5`` shuffles the piece list and gives each piece a random rotation.
    """
    puzzle = generate_solved(size, colors, seed)
    rng = XorShift(seed ^ 0xA5A5A5A5)
    rng.shuffle(puzzle.pieces)
    puzzle.pieces = [rotated(p, rng.below(4)) for p in puzzle.pieces]
    return puzzle


# ============================================================================
# §5  The official Eternity II set
# ============================================================================
def _parse_color_word(s: str) -> int:
    """Colours in the CSV are 16-bit binary words; 65535 = grey border = 0."""
    v = int(s.strip(), 2)
    return BORDER if v == 65535 else v


def official_puzzle(csv_text: str) -> Puzzle:
    """Parse the canonical official set from its CSV (ALGORITHM.md §5).

    Line 1 is the board size; each later line is
    ``top,right,bottom,left,x,y,rotation``. A piece whose (x,y,rot) are not all
    zero is one of the five pinned clue/hint pieces (at pos = y*size + x).
    """
    lines = [ln for ln in csv_text.splitlines() if ln.strip()]
    size = int(lines[0].strip())

    pieces: list[Edges] = []
    hints: list[Hint] = []
    max_color = 0
    for piece_id, line in enumerate(lines[1:]):
        cols = line.split(",")
        edges = (
            _parse_color_word(cols[0]),
            _parse_color_word(cols[1]),
            _parse_color_word(cols[2]),
            _parse_color_word(cols[3]),
        )
        max_color = max(max_color, *edges)
        pieces.append(edges)
        if len(cols) >= 7:
            x = int(cols[4].strip() or "0")
            y = int(cols[5].strip() or "0")
            rot = int(cols[6].strip() or "0")
            if x != 0 or y != 0 or rot != 0:
                hints.append(Hint(pos=y * size + x, piece=piece_id, rot=rot))

    return Puzzle(
        name="official_eternity2",
        width=size,
        height=size,
        num_colors=max_color,
        pieces=pieces,
        hints=hints,
    )


# ============================================================================
# §6  Cell-visit orders ("paths")
# ============================================================================
PATH_KINDS = [
    "row-major",
    "snake",
    "column-major",
    "spiral-in",
    "spiral-out",
    "diagonal",
    "border-first",
    "double-snake",
    "random",
]


def build_path(kind: str, width: int, height: int, seed: int) -> Optional[list[int]]:
    """Return the cell-visit permutation for ``kind`` (ALGORITHM.md §6).

    Each construction mirrors paths.rs line for line. ``random`` is the only one
    that touches the RNG. Returns ``None`` for an unknown kind.
    """
    w, h = width, height

    def idx(x: int, y: int) -> int:
        return y * w + x

    out: list[int] = []

    if kind == "row-major":
        for y in range(h):
            for x in range(w):
                out.append(idx(x, y))

    elif kind == "snake":
        for y in range(h):
            xs = range(w) if y % 2 == 0 else range(w - 1, -1, -1)
            for x in xs:
                out.append(idx(x, y))

    elif kind == "column-major":
        for x in range(w):
            for y in range(h):
                out.append(idx(x, y))

    elif kind in ("spiral-in", "spiral-out"):
        x0, y0, x1, y1 = 0, 0, w, h
        while x0 < x1 and y0 < y1:
            for x in range(x0, x1):
                out.append(idx(x, y0))
            for y in range(y0 + 1, y1):
                out.append(idx(x1 - 1, y))
            if y1 > y0 + 1:
                for x in range(x1 - 2, x0 - 1, -1):
                    out.append(idx(x, y1 - 1))
            if x1 > x0 + 1:
                for y in range(y1 - 2, y0, -1):
                    out.append(idx(x0, y))
            x0 += 1
            y0 += 1
            x1 -= 1
            y1 -= 1
        if kind == "spiral-out":
            out.reverse()

    elif kind == "diagonal":
        for d in range(w + h - 1):
            for y in range(h):
                if d >= y and d - y < w:
                    out.append(idx(d - y, y))

    elif kind == "border-first":
        # Rim clockwise from (0,0), then the interior row-major.
        for x in range(w):
            out.append(idx(x, 0))
        for y in range(1, h):
            out.append(idx(w - 1, y))
        if h > 1:
            for x in range(w - 2, -1, -1):
                out.append(idx(x, h - 1))
        if w > 1:
            for y in range(h - 2, 0, -1):
                out.append(idx(0, y))
        for y in range(1, max(h - 1, 1)):
            for x in range(1, max(w - 1, 1)):
                out.append(idx(x, y))

    elif kind == "double-snake":
        # Two rows at a time, zig-zagging — keeps a compact frontier.
        y = 0
        while y < h:
            rows = [y, y + 1] if y + 1 < h else [y]
            xs = range(w) if (y // 2) % 2 == 0 else range(w - 1, -1, -1)
            for x in xs:
                for r in rows:
                    out.append(idx(x, r))
            y += 2

    elif kind == "random":
        out = list(range(w * h))
        XorShift(seed).shuffle(out)

    else:
        return None

    assert len(out) == w * h, f"path {kind} wrong length"
    return out


# ============================================================================
# §7  The step-able backtracking solver
# ============================================================================
class Status:
    RUNNING = "running"
    SOLVED = "solved"
    EXHAUSTED = "exhausted"


@dataclass
class Report:
    status: str
    nodes: int  # successful placements
    attempts: int  # fits() calls on distinct rotations of available pieces
    backtracks: int  # undone placements
    placed: int  # pieces currently on the board (hints included)
    best_placed: int  # deepest ever reached


class _Frame:
    """One frame per non-hint cell, in path order (ALGORITHM.md §7.1)."""

    __slots__ = ("pos", "cursor", "placed")

    def __init__(self, pos: int) -> None:
        self.pos = pos
        # Next candidate ``order_index*4 + rotation`` to try at this cell.
        self.cursor = 0
        # Combination currently placed here, or -1 while empty.
        self.placed = -1


class Solver:
    """Depth-first backtracking search written as an explicit state machine, so
    a caller can run it ``step(budget)`` at a time and inspect the board in
    between (exactly what an animated UI needs).  ALGORITHM.md §7.
    """

    def __init__(
        self,
        puzzle: Puzzle,
        path: list[int],
        use_hints: bool = True,
        shuffle_pieces: bool = False,
        seed: int = 0,
    ) -> None:
        n_cells = puzzle.cell_count()
        n_pieces = len(puzzle.pieces)
        if n_pieces != n_cells:
            raise ValueError(f"puzzle has {n_pieces} pieces for {n_cells} cells")
        if len(path) != n_cells:
            raise ValueError(f"path covers {len(path)} of {n_cells} cells")
        seen = [False] * n_cells
        for c in path:
            if c >= n_cells or seen[c]:
                raise ValueError("path is not a permutation of the cells")
            seen[c] = True

        self.width = puzzle.width
        self.height = puzzle.height
        self.n_pieces = n_pieces

        # §7.1 — precomputed rotation table + distinct-rotation mask. A rotation
        # is "not distinct" if it duplicates an earlier rotation of the same
        # piece (a symmetric piece); the search skips those.
        self.table: list[Edges] = [(0, 0, 0, 0)] * (n_pieces * 4)
        self.distinct = [True] * (n_pieces * 4)
        for pid, e in enumerate(puzzle.pieces):
            for r in range(4):
                re = rotated(e, r)
                self.table[pid * 4 + r] = re
                for prev in range(r):
                    if self.table[pid * 4 + prev] == re:
                        self.distinct[pid * 4 + r] = False
                        break

        # Order in which pieces are *tried* at each cell.
        self.piece_order = list(range(n_pieces))
        if shuffle_pieces:
            XorShift(seed).shuffle(self.piece_order)

        # Board + availability bitset (here a plain bytearray for clarity:
        # used[p] == 1 means piece p is placed).
        self.board = [-1] * n_cells
        self.used = bytearray(n_pieces)
        self.hint_count = 0
        if use_hints:
            for hnt in puzzle.hints:
                if (
                    hnt.pos >= n_cells
                    or hnt.piece >= n_pieces
                    or self.used[hnt.piece]
                ):
                    raise ValueError(f"invalid hint at position {hnt.pos}")
                self.board[hnt.pos] = hnt.piece * 4 + (hnt.rot & 3)
                self.used[hnt.piece] = 1
                self.hint_count += 1

        # One frame per still-empty cell, in path order.
        self.frames = [_Frame(c) for c in path if self.board[c] == -1]

        self.depth = 0
        self.status = Status.RUNNING
        self.nodes = 0
        self.attempts = 0
        self.backtracks = 0
        self.best_placed = self.hint_count
        self.best_board = list(self.board)

    def _fits(self, pos: int, e: Edges) -> bool:
        """Does a piece with edges ``e`` legally sit at ``pos``?  §7.3."""
        w, h = self.width, self.height
        x, y = pos % w, pos // w
        top, right, bottom, left = e

        # Rim rule: a side is grey iff it is on the board's outer rim.
        if (y == 0) != (top == BORDER):
            return False
        if (y == h - 1) != (bottom == BORDER):
            return False
        if (x == 0) != (left == BORDER):
            return False
        if (x == w - 1) != (right == BORDER):
            return False

        # Neighbour rule: match every already-placed neighbour (any direction,
        # because the path may visit cells in any order). table[n][k] indexes
        # the neighbour's URDL edges: [2]=down, [0]=up, [1]=right, [3]=left.
        board = self.board
        table = self.table
        if y > 0:
            n = board[pos - w]
            if n >= 0 and table[n][2] != top:
                return False
        if y < h - 1:
            n = board[pos + w]
            if n >= 0 and table[n][0] != bottom:
                return False
        if x > 0:
            n = board[pos - 1]
            if n >= 0 and table[n][1] != left:
                return False
        if x < w - 1:
            n = board[pos + 1]
            if n >= 0 and table[n][3] != right:
                return False
        return True

    def step(self, budget: int) -> Report:
        """Run up to ``budget`` placements/backtracks, then report.  §7.2."""
        remaining = budget
        limit = self.n_pieces * 4
        while remaining > 0 and self.status == Status.RUNNING:
            remaining -= 1

            # 1. All frames filled -> solved.
            if self.depth == len(self.frames):
                self.status = Status.SOLVED
                self.best_placed = self.placed()
                self.best_board = list(self.board)
                break

            frame = self.frames[self.depth]
            pos = frame.pos
            cursor = frame.cursor
            placed_row = -1

            # 2. Scan candidates from the cursor for the first that fits.
            while cursor < limit:
                oi = cursor // 4
                r = cursor % 4
                cursor += 1
                pid = self.piece_order[oi]
                if self.used[pid]:
                    cursor = (cursor + 3) & ~3  # skip this piece's rotations
                    continue
                row = pid * 4 + r
                if not self.distinct[row]:
                    continue
                self.attempts += 1
                if self._fits(pos, self.table[row]):
                    placed_row = row
                    break

            if placed_row != -1:
                # 3. Place it and descend.
                pid = placed_row // 4
                self.board[pos] = placed_row
                self.used[pid] = 1
                frame.cursor = cursor
                frame.placed = placed_row
                self.depth += 1
                self.nodes += 1
                placed = self.placed()
                if placed > self.best_placed:
                    self.best_placed = placed
                    self.best_board = list(self.board)
            else:
                # 4. Dead end: reset this frame; backtrack (or exhaust).
                frame.cursor = 0
                if self.depth == 0:
                    self.status = Status.EXHAUSTED
                    break
                self.depth -= 1
                prev = self.frames[self.depth]
                row = prev.placed
                prev.placed = -1
                self.board[prev.pos] = -1
                self.used[row // 4] = 0
                self.backtracks += 1

        return self.report()

    def placed(self) -> int:
        return self.hint_count + self.depth

    def report(self) -> Report:
        return Report(
            status=self.status,
            nodes=self.nodes,
            attempts=self.attempts,
            backtracks=self.backtracks,
            placed=self.placed(),
            best_placed=self.best_placed,
        )

    def board_cells(self) -> list[int]:
        """Current board: cell -> piece*4+rot, or -1 when empty."""
        return self.board

    def best_board_cells(self) -> list[int]:
        return self.best_board


# ============================================================================
# §8  Scoring a board
# ============================================================================
def score_board(puzzle: Puzzle, board: list[int]) -> int:
    """Matched interior edges (ALGORITHM.md §8). A grey/grey rim contact does
    not score; the maximum is 2*w*h - w - h.
    """
    w, h = puzzle.width, puzzle.height

    def edges_of(cell: int) -> Optional[Edges]:
        if cell < 0:
            return None
        pid = cell // 4
        if pid >= len(puzzle.pieces):
            return None
        return rotated(puzzle.pieces[pid], cell % 4)

    score = 0
    for y in range(h):
        for x in range(w):
            here = edges_of(board[y * w + x])
            if here is None:
                continue
            if x + 1 < w:
                right = edges_of(board[y * w + x + 1])
                if right is not None and here[1] == right[3] and here[1] != BORDER:
                    score += 1
            if y + 1 < h:
                below = edges_of(board[(y + 1) * w + x])
                if below is not None and here[2] == below[0] and here[2] != BORDER:
                    score += 1
    return score
