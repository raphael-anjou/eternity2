# The Eternity II engine, explained once

This is the **single authoritative description of how the engine works**. The
website ships a Rust implementation (compiled to WebAssembly), and there are
several from-scratch ports in other languages (TypeScript, C, C++, Python, Lua,
COBOL, Brainfuck). They are all *the same algorithm*. Rather than re-explain it
in every language's README, the logic lives here, once, and each port points
back to this file.

If you are a maintainer who has never seen this code: **read this document
top-to-bottom and you will understand every port.** The code in each language is
a faithful, literal translation of what is written below — same names, same
steps, same order.

There is no magic and no cleverness that isn't explained here. The engine is
about 350 lines of real logic. It is small on purpose.

---

## 1. What the engine does

Eternity II is an **edge-matching puzzle**: square tiles whose four sides are
coloured, placed on a grid so that touching sides share a colour, and the border
of the board must be a specific "grey" colour. The engine does four things:

1. **Generates** solvable puzzles of any size (for the playground / demos).
2. Holds the **official** 16×16 Eternity II piece set.
3. **Solves** a puzzle by backtracking search — and does it *step by step*, so a
   browser can animate the search as it happens.
4. **Scores** a board: how many interior edges currently match.

Everything else is plumbing.

---

## 2. Conventions (memorise these four — everything depends on them)

These are shared by *every* port and every data file. Get them right and the
ports agree byte-for-byte; get them wrong and nothing lines up.

### 2.1 Edges are URDL

Every piece has 4 edge colours, always listed **Up, Right, Down, Left** —
clockwise from the top. This matches the encoding used by the community board
viewer at `e2.bucas.name`, so our boards interoperate with theirs.

```
        up
       ┌────┐
  left │    │ right       edges = [up, right, down, left]
       └────┘                       0    1     2     3
       down
```

### 2.2 Colour 0 is the border

Interior colours are `1..=22`. Colour **0** is the grey "border" colour that may
only appear on the outside rim of the board. A piece on the top row must have
`up == 0`; a piece in the interior must have *no* zero edges; and so on.

### 2.3 Rotation `r` is clockwise quarter-turns

A piece can be placed in one of 4 rotations. Rotating a URDL tuple **clockwise**
by `r` quarter-turns: the edge that faced up now faces right. Concretely:

```
rotated([u, r, d, l], 1) = [l, u, r, d]   # 90° CW: left edge moves to the top
rotated(e, r) :  new[i] = old[(i + 4 - r) mod 4]
```

(Equivalently, rotation 1 = `[l,u,r,d]`, rotation 2 = `[d,l,u,r]`, rotation 3 =
`[r,d,l,u]`.) Every port has a `rotated` / `rotateEdges` function that is exactly
this.

### 2.4 A board cell holds `piece*4 + rotation`, or `-1` when empty

The board is a flat array of integers, one per cell, in **row-major** order
(`cell index = y*width + x`). Each entry encodes *both* which piece is there and
how it's turned:

```
cell value  = piece_id * 4 + rotation    (>= 0, piece placed)
cell value  = -1                          (empty)
```

So `cell >> 2` (or `cell / 4`) is the piece id and `cell & 3` (or `cell % 4`) is
the rotation. This single-integer encoding is why the ports can compare boards
directly — the numbers line up across all languages.

---

## 3. The random number generator (XorShift + splitmix64 seeding)

Puzzle generation must be **deterministic**: the same `(size, colors, seed)`
must produce exactly the same puzzle on every platform and in every language.
We cannot use a language's built-in RNG (they all differ), so the engine carries
its own tiny generator. Reproducing it *exactly* is the first thing every port
must get right, because everything downstream (which colours, which piece order,
which rotations) flows from it.

It has two parts.

### 3.1 Seeding with one round of splitmix64

A raw 32-bit seed makes a poor 64-bit state (lots of zero bits), so we run it
through one round of the well-known **splitmix64** mixer first:

```
state = seed + 0x9E3779B97F4A7C15
state = (state XOR (state >> 30)) * 0xBF58476D1CE4E5B9
state = (state XOR (state >> 27)) * 0x94D049BB133111EB
state =  state XOR (state >> 31)
state =  state OR 1                  # force non-zero (xorshift dies on zero)
```

All arithmetic is **unsigned 64-bit with wraparound** (multiplication overflows
and wraps; that is intentional and must be preserved). Languages without native
u64 wrapping (JavaScript) must use `BigInt` masked to 64 bits.

### 3.2 The xorshift step

The generator produces 32-bit numbers by a standard xorshift64 step and
returning the **high 32 bits**:

```
x  = state
x ^= x << 13
x ^= x >> 7
x ^= x << 17
state = x                 # (kept as 64-bit)
return (x >> 32) as u32   # the high half
```

Again, 64-bit wrapping throughout.

### 3.3 Two helpers built on top

- `below(n)` → a number in `[0, n)`, computed as `next_u32() % n`.
- `shuffle(array)` → an in-place **Fisher–Yates** shuffle, iterating **from the
  last index down to 1**, swapping `i` with `below(i+1)`:

  ```
  for i = len-1 down to 1:
      j = below(i + 1)
      swap(array[i], array[j])
  ```

The descending direction and the `below(i+1)` call order matter: change either
and the shuffle produces a different permutation, breaking parity.

> **Parity note.** This is the single most fragile piece to port. The Lua, TS,
> Python, C and C++ ports each reproduce the exact 64-bit math above, and the
> test suite checks generated puzzles against Rust-produced golden data, so any
> RNG drift is caught immediately.

---

## 4. Generating a solvable puzzle

`generate(size, colors, seed)` builds an `n×n` puzzle that is **solvable by
construction**. The trick: build a *solved* board first, then scramble the
pieces. Steps:

1. **Paint the interior adjacencies.** An `n×n` board has `2·n·(n−1)` interior
   edges (shared sides between neighbours). Make a `palette` array of that many
   colours: the first `colors` entries are `1, 2, …, colors` (guaranteeing every
   colour appears at least once), the rest are random in `1..=colors`. Then
   `shuffle` the palette. (The optional *framed* variant paints this step
   differently — with **balanced** colour counts, closer to real Eternity II —
   see §4.1.)

2. **Split the palette** into vertical and horizontal adjacency colours and lay
   them out on the grid: `vert[y][x]` is the colour between cell `(x,y)` and
   `(x+1,y)`; `horiz[y][x]` is the colour between `(x,y)` and `(x,y+1)`.

3. **Derive each piece** from its four surrounding adjacencies. A cell's `up`
   edge is the horizontal colour above it (or `0` on the top row), its `right`
   edge is the vertical colour to its right (or `0` on the right column), etc.
   This yields `generate_solved`: piece `i` sits at cell `i`, rotation 0, and the
   identity board is a perfect solution.

4. **Scramble** (only in `generate`, not `generate_solved`): make a *second* RNG
   seeded with `seed XOR 0xA5A5A5A5`, shuffle the piece list, and give each piece
   a random rotation. Now the solution order is hidden but a solution still
   exists.

The two distinct seeds (`seed` for the layout, `seed XOR 0xA5A5A5A5` for the
scramble) keep the puzzle's *shape* and its *shuffle* independent.

`max_colors(size) = min(2·n·(n−1), 22)` — you can't ask for more colours than
there are interior edges, nor more than the 22 renderable motifs.

### 4.1 The framed variant (real-Eternity-II colour layout)

The default painter above is uniform: any colour can appear on any interior
adjacency, and each colour's *count* is left to chance. Real Eternity II is not
like that — its 22 motifs split into a handful that only touch the **frame band**
(edges adjacent to the grey rim) and the rest that live in the **deep interior**,
and each motif appears roughly the same number of times. The framed variant
(`generate_framed` / `generate_solved_framed`, flag off ⇒ byte-for-byte identical
to the default) reproduces both properties. It only differs in **step 1** — the
painting; steps 2–4 are unchanged.

- **Two pools, not one.** With `frame_count = min(5, colors−1)`, colours
  `1..=frame_count` are the frame colours and `frame_count+1..=colors` the
  interior colours. Partition the interior adjacencies (ascending slot index)
  into a *frame-band* pool (a slot at least one of whose two cells lies on the
  outer ring) and a *deep-interior* pool (both cells off the rim).

- **Fill each pool by *cycling*, then shuffle.** Fill the frame-band pool with
  `1, 2, …, frame_count, 1, 2, …` (repeat the cycle to the pool's length), so
  every frame colour appears as equally often as the slot count allows — each
  `⌈slots/frame_count⌉` or `⌊slots/frame_count⌋` times, never more than one apart.
  Fill the deep-interior pool the same way with its colours. Then `shuffle` each
  pool **independently** and drop the results onto their slots. The even counts
  give the real-E2 look; the shuffle keeps the board fully random.

  > This balanced fill uses **only `shuffle`** (no `below` draws inside a pool),
  > so it stays a pure function of the two shuffles and every port reproduces it
  > byte-for-byte. Thanks to Vasily (VV) for pointing out that cycling the pools
  > — rather than drawing colours at random — is what gives the equal counts.

- **Fallback.** The split needs at least one interior colour (`colors ≥ 2`) and at
  least one deep-interior adjacency (only exists for `size ≥ 4`); below that
  threshold the framed path equals the unrestricted one, so every requested
  colour still appears.

`frame_count = min(5, colors−1)` mirrors the real set (5 of its 22 colours are
frame-only). The website's Viewer surfaces this variant as a switch.

---

## 5. The official Eternity II set

`official_puzzle()` returns the real 16×16 puzzle (256 pieces, 22 colours, 5
clue/hint pieces). It is parsed from a canonical CSV (`data/official_eternity2.csv`)
where each colour is written as a 16-bit binary word (`65535` = grey border =
colour 0), and any piece with a non-zero `(x, y, rotation)` is one of the five
**hints** — pieces pinned to a known cell and rotation. The ports that need it
embed the same CSV and parse it the same way.

---

## 6. Cell-visit orders ("paths")

A **path** is the order in which the solver fills cells — a permutation of the
cell indices `0..w*h`. This is a genuine search hyperparameter: the *same* puzzle
can take wildly different amounts of work depending on the fill order. The engine
ships **9** of them:

| Name | Order |
| --- | --- |
| `row-major` | left-to-right, top-to-bottom |
| `snake` | row-major but alternate rows go right-to-left (a continuous snake) |
| `column-major` | top-to-bottom, left-to-right |
| `spiral-in` | rim first, spiralling inward to the centre |
| `spiral-out` | `spiral-in` reversed (centre outward) |
| `diagonal` | anti-diagonals, top-left to bottom-right |
| `border-first` | the whole rim clockwise, then the interior row-major |
| `double-snake` | two rows at a time, zig-zagging (keeps a compact frontier) |
| `random` | a seeded `shuffle` of all cells |

`build_path(kind, width, height, seed)` returns the permutation. The exact
construction of each is given in the code (every port mirrors it line for line);
`random` is the only one that touches the RNG, via `seed`.

The solver does **not** require the path to be contiguous — it checks *all four*
neighbours of each cell when testing a fit, so even a `random` path produces
correct solutions. Path order only affects *how fast*.

---

## 7. The solver: step-able backtracking DFS

This is the heart of the engine. It is a depth-first backtracking search, but
written as an **explicit state machine** rather than a recursive function, so the
caller can run it a bounded number of steps at a time and inspect the board in
between. That is exactly what an animated browser UI needs: `step(budget)` runs
up to `budget` operations and returns; the UI draws the board; repeat.

### 7.1 Setup (done once, in the constructor)

- **Rotation table.** Precompute `table[piece*4 + r]` = the piece's edges rotated
  by `r`, for every piece and every `r` in `0..4`. Placement then never has to
  rotate on the fly.
- **Distinct-rotation mask.** Some pieces are symmetric: two of their rotations
  give identical edges. `distinct[piece*4 + r]` is `false` if that rotation
  duplicates an earlier rotation of the same piece. The search **skips**
  non-distinct rotations — trying them would just re-explore the same board.
- **Piece order.** The order in which pieces are *tried* at each cell. By default
  `0, 1, 2, …`; if `shuffle_pieces` is set, it's a seeded `shuffle` of that.
- **Hints.** If `use_hints` is on, pre-place each hint piece at its pinned cell
  and rotation, and mark it used. Hinted cells are never searched.
- **Frames.** One `frame` per *non-hint* cell, in path order. A frame holds:
  - `pos` — which board cell this frame fills,
  - `cursor` — the next `(piece_order_index, rotation)` combination to try here,
    encoded as a single number `index*4 + rotation`,
  - `placed` — which combination is currently placed here (or "none").

  `depth` is the index of the frame we are currently working on.

- **`used`** — a bitset: bit `p` set means piece `p` is already on the board.

### 7.2 One step

`step(budget)` loops up to `budget` times; each iteration is **one placement or
one backtrack**:

1. If `depth == number of frames`, every cell is filled → **Solved**. Stop.
2. Otherwise look at the current frame. Scan candidates starting at its `cursor`:
   - decode `cursor` into a piece-order index and a rotation;
   - if that **piece is already used**, skip all four of its rotations at once;
   - if that **rotation is not distinct**, skip just it;
   - otherwise count an **attempt** and test whether the piece *fits* at this
     cell (see §7.3).
   - The first candidate that fits wins; remember the cursor position *after* it.
3. **If a candidate fit:** place it (write `piece*4+rot` into the board, mark the
   piece used, save the cursor and placement into the frame), advance `depth`,
   and count a **node**. Track the deepest board ever reached (`best_placed` /
   `best_board`) — that's what the UI shows as "best so far".
4. **If nothing fit:** dead end. Reset this frame's cursor. If we're at the top
   frame (`depth == 0`), the search is **Exhausted** (no solution). Otherwise
   step back one frame, **undo** its placement (clear the board cell, free the
   piece), count a **backtrack**, and continue. The frame we backtracked to keeps
   its old cursor, so it resumes *after* the candidate that led to the dead end.

That's the entire search. Placement and backtracking, one at a time, bounded by
`budget`.

### 7.3 Does a piece "fit" at a cell?

`fits(pos, edges)` returns true iff:

- **Rim rule.** On each of the four sides, the piece's edge is grey (`0`) *iff*
  that side is on the board's outer rim. Top row ⇒ `up` must be 0; not top row ⇒
  `up` must not be 0; and so on for all four sides.
- **Neighbour rule.** For each of the four neighbours that is *already placed*,
  the shared edge colours must be equal. (We check all four because the path may
  visit cells in any order — a neighbour in any direction might already be down.)

Note a grey/grey contact between two rim cells is allowed by the rim rule but
**does not score** (see §8); it's a legal placement, not a matched edge.

### 7.4 The three counters (why we report them)

- **nodes** — successful placements.
- **attempts** — `fits` calls on distinct rotations of available pieces.
- **backtracks** — undone placements.

These are reported alongside the board. They matter for tests: two engines that
produce the same final board but *different* counts are exploring the tree
differently. The parity suite checks the counts **exactly**, which proves the
ports don't just coincidentally solve — they walk the identical search tree.

---

## 8. Scoring a board

`score_board(puzzle, board)` counts **matched interior edges**: for every pair of
horizontally- or vertically-adjacent placed cells, score 1 if the shared edge
colours are equal **and not grey**. Grey/grey rim contacts don't count. The
maximum for an `n×n` board is `2·n·(n−1)` (every interior edge matched) — that's
`480` for the official 16×16, hence "Eternity II / 480."

---

## 9. The public surface (what each port exposes)

Every port — and the WASM module the website loads — offers the same set of
operations:

| Operation | Meaning |
| --- | --- |
| `officialPuzzle()` | the real 16×16 set |
| `generatePuzzle(size, colors, seed)` | a scrambled solvable puzzle |
| `generateSolvedPuzzle(size, colors, seed)` | same, but in solution order (identity board solves it) |
| `maxColors(size)` | largest usable colour count |
| `pathKinds()` | the 9 path names |
| `buildPath(kind, w, h, seed)` | a cell-visit permutation |
| `scoreBoard(puzzle, board)` | matched interior edges |
| a **Solver** object | `new(puzzle, path, useHints, shufflePieces, seed)`, then `step(budget)`, `report()`, `board()`, `bestBoard()`, `score()`, `bestScore()`, `reset()` |

The full ports (TypeScript, C/WASM, C++/WASM) implement *all* of these — the
complete surface of the canonical Rust engine. The smaller studies (Python, Lua,
COBOL, Brainfuck) implement as much as makes sense for a command-line demo. Each
is validated against the same Rust **golden data**. None are build-selectable:
the website always runs the Rust/WASM engine in `../engine`.

---

## 10. How parity is proven

The Rust engine emits reference outputs (`engine/src/bin/golden.rs` →
`golden.txt`): generated-puzzle pieces (RNG + construction), the official set
summary, every path permutation at several sizes, and full solver runs (final
status, score, and the three counters) on a 4×4 across all 9 paths plus a fixed
50,000-step probe of the official set.

Each port has a test that reads that same `golden.txt` and asserts its own output
matches **field for field, count for count**. If a port drifts in the RNG, a path
construction, the rotation convention, the distinct-rotation handling, or the
backtracking order, a golden assertion fails. That is the safety net that lets us
claim these are genuinely the same engine, not lookalikes.

---

*Start here, then read any single port alongside this document. They were written
to be read together.*
