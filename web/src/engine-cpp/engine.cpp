// =============================================================================
// Eternity II engine — C++ ISO port, compiled to freestanding wasm32.
//
// This is a literal, line-for-line translation of the Rust engine
// (engine/src/{types,generator,paths,solver,official}.rs) following the
// authoritative language-agnostic description in
//   engine-side-quests/ALGORITHM.md
// Read that document alongside this file; the names, steps and order here
// mirror it exactly so the two engines agree byte-for-byte (verified against
// engine-side-quests/engine-lua/golden.txt by web/src/engine-cpp/parity.mjs).
//
// Build target: `clang++ --target=wasm32 -nostdlib -fno-exceptions -fno-rtti`.
// There is no libc and no STL heap. Everything lives in fixed-size static
// buffers sized for the official 16x16 board (256 pieces / cells), which is the
// largest puzzle the website ever runs. See build.sh and README.md.
//
// The same source also compiles natively (clang++ -std=c++20 engine.cpp) with a
// `#ifndef __wasm__` main at the bottom that re-emits golden.txt, used as a
// debugging aid during development.
// =============================================================================

// ---- Freestanding integer typedefs (no <cstdint> without libc) --------------
using u8  = unsigned char;
using u16 = unsigned short;
using u32 = unsigned int;
using i32 = int;
using u64 = unsigned long long;

static_assert(sizeof(u32) == 4, "u32 must be 32-bit");
static_assert(sizeof(u64) == 8, "u64 must be 64-bit");

// Board / puzzle sizing. The official set is the upper bound the site runs.
static constexpr int MAX_SIZE   = 16;            // max board edge
static constexpr int MAX_CELLS  = MAX_SIZE * MAX_SIZE;   // 256
static constexpr int MAX_PIECES = MAX_CELLS;            // 256
static constexpr u8  BORDER     = 0;             // ALGORITHM.md §2.2

// =============================================================================
// types.rs — rotation convention (ALGORITHM.md §2.3)
// =============================================================================

// A piece's four edge colours, URDL (up, right, down, left). ALGORITHM.md §2.1.
struct Edges {
  u8 e[4];
};

// rotated(e, r): clockwise quarter-turns. new[i] = old[(i + 4 - r) % 4].
// Mirrors types.rs::rotated.
static inline Edges rotated(Edges e, u8 r) {
  u8 rr = r & 3;
  Edges out;
  out.e[0] = e.e[(4 - rr) & 3];
  out.e[1] = e.e[(5 - rr) & 3];
  out.e[2] = e.e[(6 - rr) & 3];
  out.e[3] = e.e[(7 - rr) & 3];
  return out;
}

static inline bool edges_eq(Edges a, Edges b) {
  return a.e[0] == b.e[0] && a.e[1] == b.e[1] && a.e[2] == b.e[2] && a.e[3] == b.e[3];
}

// =============================================================================
// generator.rs — XorShift RNG (ALGORITHM.md §3)
// =============================================================================

struct XorShift {
  u64 state;

  // splitmix64 seeding (ALGORITHM.md §3.1). All math is u64 wrapping.
  static XorShift make(u32 seed) {
    u64 z = (u64)seed + 0x9E3779B97F4A7C15ULL;
    z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9ULL;
    z = (z ^ (z >> 27)) * 0x94D049BB133111EBULL;
    z = (z ^ (z >> 31)) | 1ULL;   // force non-zero (xorshift dies on zero)
    XorShift x;
    x.state = z;
    return x;
  }

  // xorshift64 step, returning the high 32 bits (ALGORITHM.md §3.2).
  u32 next_u32() {
    u64 x = state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    state = x;
    return (u32)(x >> 32);
  }

  // below(n) → uniform in [0, n)  (ALGORITHM.md §3.3).
  u32 below(u32 n) { return next_u32() % n; }
};

// Fisher–Yates over a u8 array, last index down to 1 (ALGORITHM.md §3.3).
static void shuffle_u8(XorShift* rng, u8* a, int len) {
  for (int i = len - 1; i >= 1; --i) {
    u32 j = rng->below((u32)i + 1);
    u8 tmp = a[i];
    a[i] = a[(int)j];
    a[(int)j] = tmp;
  }
}

// Fisher–Yates over a u16 array (used for piece_order and the `random` path).
static void shuffle_u16(XorShift* rng, u16* a, int len) {
  for (int i = len - 1; i >= 1; --i) {
    u32 j = rng->below((u32)i + 1);
    u16 tmp = a[i];
    a[i] = a[(int)j];
    a[(int)j] = tmp;
  }
}

// Fisher–Yates over an Edges array (the scramble in `generate`).
static void shuffle_edges(XorShift* rng, Edges* a, int len) {
  for (int i = len - 1; i >= 1; --i) {
    u32 j = rng->below((u32)i + 1);
    Edges tmp = a[i];
    a[i] = a[(int)j];
    a[(int)j] = tmp;
  }
}

// interior_edge_count(size) = 2·n·(n−1).
static inline u32 interior_edge_count(u8 size) {
  return 2u * (u32)size * ((u32)size - 1u);
}

// max_colors(size) = min(2·n·(n−1), 22).
static u8 max_colors(u8 size) {
  u32 e = interior_edge_count(size);
  return (u8)(e < 22u ? e : 22u);
}

// =============================================================================
// A Puzzle, in flat static-buffer form (mirrors types.rs::Puzzle).
// =============================================================================
struct Hint {
  u16 pos;
  u16 piece;
  u8  rot;
};

struct Puzzle {
  u8    width;
  u8    height;
  u8    num_colors;
  int   n_pieces;
  Edges pieces[MAX_PIECES];
  int   n_hints;
  Hint  hints[8];
};

// generate_solved(size, colors, seed) — ALGORITHM.md §4 steps 1-3.
// Builds a solved board by painting interior adjacencies, then deriving the
// piece edges. Pieces stay in solution order (piece i at cell i, rot 0).
static void generate_solved(Puzzle* p, u8 size, u8 colors, u32 seed) {
  // colors.clamp(1, max_colors(size))
  u8 mc = max_colors(size);
  if (colors < 1) colors = 1;
  if (colors > mc) colors = mc;

  int s = (int)size;
  int n_edges = (int)interior_edge_count(size);
  XorShift rng = XorShift::make(seed);

  // Step 1: one colour per interior adjacency, every colour present once.
  u8 palette[2 * MAX_SIZE * (MAX_SIZE - 1)];
  for (int i = 0; i < n_edges; ++i) {
    if (i < (int)colors) {
      palette[i] = (u8)(i + 1);
    } else {
      palette[i] = (u8)(rng.below((u32)colors) + 1);
    }
  }
  shuffle_u8(&rng, palette, n_edges);

  // Step 2: split palette into vertical / horizontal adjacency colours.
  //   vert[y*(s-1)+x]  = colour between (x,y) and (x+1,y)
  //   horiz[y*s + x]   = colour between (x,y) and (x,y+1)
  const u8* vert  = palette;                 // first s*(s-1) entries
  const u8* horiz = palette + (s * (s - 1)); // rest

  // Step 3: derive each piece from its four surrounding adjacencies.
  int idx = 0;
  for (int y = 0; y < s; ++y) {
    for (int x = 0; x < s; ++x) {
      u8 up    = (y == 0)     ? BORDER : horiz[(y - 1) * s + x];
      u8 down  = (y == s - 1) ? BORDER : horiz[y * s + x];
      u8 left  = (x == 0)     ? BORDER : vert[y * (s - 1) + (x - 1)];
      u8 right = (x == s - 1) ? BORDER : vert[y * (s - 1) + x];
      p->pieces[idx].e[0] = up;
      p->pieces[idx].e[1] = right;
      p->pieces[idx].e[2] = down;
      p->pieces[idx].e[3] = left;
      ++idx;
    }
  }

  p->width = size;
  p->height = size;
  p->num_colors = colors;
  p->n_pieces = s * s;
  p->n_hints = 0;
}

// generate(size, colors, seed) — ALGORITHM.md §4 step 4 (scramble).
static void generate(Puzzle* p, u8 size, u8 colors, u32 seed) {
  generate_solved(p, size, colors, seed);
  XorShift rng = XorShift::make(seed ^ 0xA5A5A5A5u);
  shuffle_edges(&rng, p->pieces, p->n_pieces);
  for (int i = 0; i < p->n_pieces; ++i) {
    p->pieces[i] = rotated(p->pieces[i], (u8)rng.below(4));
  }
}

// =============================================================================
// official.rs — the official Eternity II set, embedded pre-parsed.
//
// The Rust port parses data/official_eternity2.csv (16-bit binary colour
// words; 65535 = border = 0; nonzero x/y/rot ⇒ a clue/hint). Freestanding wasm
// has no string parsing, so the parse is done once at build time and the
// resulting URDL edge multiset + the five hints are embedded directly. The
// values are identical to what official.rs produces (verified by parity).
// =============================================================================
static const u8 OFFICIAL_PIECES[256][4] = {
    {0,0,1,3},{0,0,1,4},{0,0,2,3},{0,0,3,2},{0,1,6,1},{0,1,7,2},{0,1,9,1},{0,1,9,5},
    {0,1,12,3},{0,1,14,4},{0,1,15,2},{0,1,19,4},{0,1,19,5},{0,1,21,4},{0,2,7,1},{0,2,8,3},
    {0,2,10,5},{0,2,13,5},{0,2,14,2},{0,2,15,2},{0,2,16,4},{0,2,17,1},{0,2,17,5},{0,2,18,1},
    {0,2,21,1},{0,3,6,2},{0,3,6,3},{0,3,7,3},{0,3,8,3},{0,3,14,5},{0,3,15,2},{0,3,18,3},
    {0,3,19,2},{0,3,19,4},{0,3,20,5},{0,3,22,4},{0,4,8,1},{0,4,11,5},{0,4,12,5},{0,4,13,2},
    {0,4,13,3},{0,4,15,1},{0,4,15,2},{0,4,15,3},{0,4,16,1},{0,4,18,4},{0,4,19,4},{0,4,20,4},
    {0,5,6,5},{0,5,7,1},{0,5,7,2},{0,5,9,1},{0,5,14,4},{0,5,16,4},{0,5,16,5},{0,5,19,3},
    {0,5,20,1},{0,5,20,5},{0,5,21,2},{0,5,22,3},{6,6,9,8},{6,6,10,14},{6,7,7,11},{6,8,6,19},
    {6,8,8,22},{6,8,10,10},{6,8,12,7},{6,8,18,9},{6,8,22,19},{6,11,11,14},{6,11,14,17},{6,12,10,8},
    {6,12,15,16},{6,12,18,15},{6,12,19,11},{6,13,10,15},{6,13,13,15},{6,14,11,20},{6,14,18,11},{6,14,20,21},
    {6,15,13,8},{6,16,8,8},{6,16,12,16},{6,17,8,13},{6,17,9,10},{6,17,19,17},{6,17,20,18},{6,18,6,21},
    {6,18,9,22},{6,18,16,20},{6,19,12,17},{6,19,13,15},{6,19,13,16},{6,19,16,21},{6,19,17,10},{6,21,21,11},
    {6,22,16,13},{6,22,18,19},{6,22,21,9},{6,22,22,21},{7,7,17,15},{7,7,17,20},{7,7,20,13},{7,7,22,9},
    {7,8,16,15},{7,9,11,19},{7,9,13,19},{7,9,16,15},{7,9,20,12},{7,10,15,17},{7,10,17,15},{7,11,18,13},
    {7,11,18,20},{7,12,10,16},{7,12,14,17},{7,12,17,12},{7,12,22,20},{7,13,11,21},{7,14,20,17},{7,15,19,22},
    {7,16,10,22},{7,18,9,20},{7,18,10,13},{7,18,18,15},{7,19,17,22},{7,19,21,15},{7,20,10,9},{7,20,13,21},
    {7,20,16,11},{7,20,18,19},{7,21,9,18},{7,21,17,10},{7,22,10,20},{7,22,12,16},{7,22,16,11},{7,22,20,18},
    {8,8,18,14},{8,9,9,11},{8,9,9,12},{8,9,9,17},{8,9,13,21},{8,9,15,9},{8,9,20,17},{8,9,21,21},
    {8,10,11,16},{8,11,8,17},{8,11,8,22},{8,11,11,10},{8,11,15,17},{8,13,9,12},{8,13,16,22},{8,14,12,12},
    {8,14,12,13},{8,14,22,20},{8,16,14,14},{8,16,14,17},{8,16,22,14},{8,18,14,20},{8,18,19,9},{8,19,21,21},
    {8,20,9,18},{8,20,10,18},{8,22,15,12},{8,22,16,20},{9,10,11,16},{9,10,16,19},{9,12,11,11},{9,13,12,15},
    {9,13,13,21},{9,14,16,19},{9,14,18,20},{9,14,21,12},{9,15,15,15},{9,15,17,18},{9,16,14,21},{9,17,14,13},
    {9,18,10,16},{9,19,17,20},{9,19,19,15},{9,20,14,20},{9,21,12,20},{9,21,14,12},{10,10,13,19},{10,11,22,14},
    {10,12,13,17},{10,12,19,19},{10,13,21,14},{10,14,10,21},{10,14,11,13},{10,14,20,13},{10,15,11,11},{10,15,15,18},
    {10,16,12,14},{10,17,21,12},{10,17,21,22},{10,17,22,15},{10,18,12,22},{10,18,13,19},{10,18,18,18},{10,19,13,11},
    {10,20,21,19},{10,20,22,14},{10,21,17,13},{10,21,17,19},{10,21,20,11},{10,21,22,21},{11,11,22,14},{11,12,13,22},
    {11,12,19,15},{11,14,12,13},{11,14,20,15},{11,15,11,20},{11,16,19,19},{11,17,11,18},{11,17,16,22},{11,17,22,21},
    {11,18,13,15},{11,20,16,17},{11,21,12,16},{11,22,12,20},{11,22,21,21},{11,22,21,22},{12,12,17,22},{12,12,18,14},
    {12,12,20,15},{12,14,13,15},{12,14,17,17},{12,16,13,19},{12,18,14,22},{12,18,20,19},{12,19,17,18},{12,19,17,21},
    {13,13,13,18},{13,14,20,16},{13,16,14,16},{13,16,14,18},{13,16,17,15},{13,17,16,20},{13,18,15,22},{13,18,21,15},
    {13,19,14,21},{13,19,16,21},{14,15,15,17},{14,15,18,19},{14,16,22,18},{14,21,20,22},{15,15,21,22},{15,16,17,16},
    {15,17,16,21},{15,18,20,21},{16,16,22,19},{16,17,19,17},{17,19,20,18},{18,20,21,20},{18,22,20,22},{19,22,21,22},
};

// The five clue pieces: {pos, piece_id, rotation}. pos = y*16 + x.
static const u16 OFFICIAL_HINTS[5][3] = {
    {135, 138, 0}, {210, 180, 1}, {34, 207, 1}, {221, 248, 2}, {45, 254, 1},
};

static void official_puzzle(Puzzle* p) {
  p->width = 16;
  p->height = 16;
  p->n_pieces = 256;
  u8 maxc = 0;
  for (int i = 0; i < 256; ++i) {
    for (int k = 0; k < 4; ++k) {
      u8 c = OFFICIAL_PIECES[i][k];
      p->pieces[i].e[k] = c;
      if (c > maxc) maxc = c;
    }
  }
  p->num_colors = maxc;  // 22
  p->n_hints = 5;
  for (int i = 0; i < 5; ++i) {
    p->hints[i].pos   = OFFICIAL_HINTS[i][0];
    p->hints[i].piece = OFFICIAL_HINTS[i][1];
    p->hints[i].rot   = (u8)OFFICIAL_HINTS[i][2];
  }
}

// =============================================================================
// paths.rs — cell-visit orders (ALGORITHM.md §6)
// =============================================================================
enum PathKind {
  PK_ROW_MAJOR = 0,
  PK_SNAKE,
  PK_COLUMN_MAJOR,
  PK_SPIRAL_IN,
  PK_SPIRAL_OUT,
  PK_DIAGONAL,
  PK_BORDER_FIRST,
  PK_DOUBLE_SNAKE,
  PK_RANDOM,
  PK_COUNT,
};

// Build a path into `out` (length w*h). Returns the count, or -1 on bad kind.
// Mirrors paths.rs::build_path, branch for branch.
static int build_path(int kind, u8 width, u8 height, u32 seed, u16* out) {
  int w = (int)width, h = (int)height;
  int n = 0;
  auto idx = [&](int x, int y) -> u16 { return (u16)(y * w + x); };

  switch (kind) {
    case PK_ROW_MAJOR:
      for (int y = 0; y < h; ++y)
        for (int x = 0; x < w; ++x) out[n++] = idx(x, y);
      break;
    case PK_SNAKE:
      for (int y = 0; y < h; ++y) {
        if (y % 2 == 0) {
          for (int x = 0; x < w; ++x) out[n++] = idx(x, y);
        } else {
          for (int x = w - 1; x >= 0; --x) out[n++] = idx(x, y);
        }
      }
      break;
    case PK_COLUMN_MAJOR:
      for (int x = 0; x < w; ++x)
        for (int y = 0; y < h; ++y) out[n++] = idx(x, y);
      break;
    case PK_SPIRAL_IN:
    case PK_SPIRAL_OUT: {
      int x0 = 0, y0 = 0, x1 = w, y1 = h;
      while (x0 < x1 && y0 < y1) {
        for (int x = x0; x < x1; ++x) out[n++] = idx(x, y0);
        for (int y = y0 + 1; y < y1; ++y) out[n++] = idx(x1 - 1, y);
        if (y1 > y0 + 1)
          for (int x = x1 - 2; x >= x0; --x) out[n++] = idx(x, y1 - 1);
        if (x1 > x0 + 1)
          for (int y = y1 - 2; y >= y0 + 1; --y) out[n++] = idx(x0, y);
        x0++; y0++; x1--; y1--;
      }
      if (kind == PK_SPIRAL_OUT) {
        for (int i = 0; i < n / 2; ++i) {
          u16 t = out[i]; out[i] = out[n - 1 - i]; out[n - 1 - i] = t;
        }
      }
      break;
    }
    case PK_DIAGONAL:
      for (int d = 0; d < w + h - 1; ++d)
        for (int y = 0; y < h; ++y)
          if (d >= y && d - y < w) out[n++] = idx(d - y, y);
      break;
    case PK_BORDER_FIRST:
      for (int x = 0; x < w; ++x) out[n++] = idx(x, 0);
      for (int y = 1; y < h; ++y) out[n++] = idx(w - 1, y);
      if (h > 1)
        for (int x = w - 2; x >= 0; --x) out[n++] = idx(x, h - 1);
      if (w > 1)
        for (int y = h - 2; y >= 1; --y) out[n++] = idx(0, y);
      {
        int yl = (h >= 1) ? h - 1 : 0;  // h.saturating_sub(1)
        int xl = (w >= 1) ? w - 1 : 0;
        for (int y = 1; y < yl; ++y)
          for (int x = 1; x < xl; ++x) out[n++] = idx(x, y);
      }
      break;
    case PK_DOUBLE_SNAKE: {
      int y = 0;
      while (y < h) {
        int rows[2]; int nrows;
        if (y + 1 < h) { rows[0] = y; rows[1] = y + 1; nrows = 2; }
        else { rows[0] = y; nrows = 1; }
        if ((y / 2) % 2 == 0) {
          for (int x = 0; x < w; ++x)
            for (int r = 0; r < nrows; ++r) out[n++] = idx(x, rows[r]);
        } else {
          for (int x = w - 1; x >= 0; --x)
            for (int r = 0; r < nrows; ++r) out[n++] = idx(x, rows[r]);
        }
        y += 2;
      }
      break;
    }
    case PK_RANDOM: {
      for (int i = 0; i < w * h; ++i) out[i] = (u16)i;
      n = w * h;
      XorShift rng = XorShift::make(seed);
      shuffle_u16(&rng, out, n);
      break;
    }
    default:
      return -1;
  }
  return n;
}

// =============================================================================
// solver.rs — step-able backtracking DFS (ALGORITHM.md §7)
// =============================================================================
enum Status { ST_RUNNING = 0, ST_SOLVED = 1, ST_EXHAUSTED = 2 };

struct Frame {
  u16 pos;
  u32 cursor;   // next (piece_order_index * 4 + rotation) to try
  u32 placed;   // currently-placed candidate row, or NONE
};
static constexpr u32 NONE = 0xFFFFFFFFu;

struct Solver {
  int   width;
  int   height;
  int   n_pieces;
  int   n_cells;

  Edges table[MAX_PIECES * 4];      // rotated edges for (piece, rot)
  bool  distinct[MAX_PIECES * 4];   // false ⇒ rotation duplicates a lower one
  u16   piece_order[MAX_PIECES];

  i32   board[MAX_CELLS];           // cell -> piece*4+rot, or -1
  u64   used[(MAX_PIECES + 63) / 64];   // availability bitset

  Frame frames[MAX_CELLS];
  int   n_frames;
  int   depth;
  u32   hint_count;

  Status status;
  u64    nodes;
  u64    attempts;
  u64    backtracks;
  u32    best_placed;
  i32    best_board[MAX_CELLS];

  // Saved constructor args, for reset().
  Puzzle puzzle;
  u16    path[MAX_CELLS];
  int    path_len;
  bool   use_hints;
  bool   shuffle_pieces;
  u32    seed;
  bool   in_use;   // pool slot occupied?
};

static inline bool used_contains(const Solver* s, int i) {
  return ((s->used[i >> 6] >> (i & 63)) & 1ULL) == 1ULL;
}
static inline void used_insert(Solver* s, int i) {
  s->used[i >> 6] |= (1ULL << (i & 63));
}
static inline void used_remove(Solver* s, int i) {
  s->used[i >> 6] &= ~(1ULL << (i & 63));
}

// (Re)initialise a solver from its saved puzzle/path/options. Mirrors
// solver.rs::Solver::new. Returns false on an invalid configuration.
static bool solver_init(Solver* s) {
  const Puzzle* p = &s->puzzle;
  int n_cells = (int)p->width * (int)p->height;
  int n_pieces = p->n_pieces;
  if (n_pieces != n_cells) return false;
  if (s->path_len != n_cells) return false;

  s->width = p->width;
  s->height = p->height;
  s->n_pieces = n_pieces;
  s->n_cells = n_cells;

  // Rotation table + distinct-rotation mask (ALGORITHM.md §7.1).
  for (int id = 0; id < n_pieces; ++id) {
    for (u8 r = 0; r < 4; ++r) {
      Edges re = rotated(p->pieces[id], r);
      s->table[id * 4 + r] = re;
      s->distinct[id * 4 + r] = true;
      for (u8 prev = 0; prev < r; ++prev) {
        if (edges_eq(s->table[id * 4 + prev], re)) {
          s->distinct[id * 4 + r] = false;
          break;
        }
      }
    }
  }

  // Piece order, optionally seed-shuffled.
  for (int i = 0; i < n_pieces; ++i) s->piece_order[i] = (u16)i;
  if (s->shuffle_pieces) {
    XorShift rng = XorShift::make(s->seed);
    shuffle_u16(&rng, s->piece_order, n_pieces);
  }

  for (int i = 0; i < n_cells; ++i) s->board[i] = -1;
  for (int i = 0; i < (int)((MAX_PIECES + 63) / 64); ++i) s->used[i] = 0;

  // Pre-place hints.
  s->hint_count = 0;
  if (s->use_hints) {
    for (int i = 0; i < p->n_hints; ++i) {
      int pos = (int)p->hints[i].pos;
      int piece = (int)p->hints[i].piece;
      if (pos >= n_cells || piece >= n_pieces || used_contains(s, piece)) return false;
      s->board[pos] = piece * 4 + (i32)(p->hints[i].rot & 3);
      used_insert(s, piece);
      s->hint_count++;
    }
  }

  // One frame per non-hint cell, in path order.
  s->n_frames = 0;
  for (int i = 0; i < s->path_len; ++i) {
    int c = (int)s->path[i];
    if (s->board[c] == -1) {
      Frame f;
      f.pos = (u16)c;
      f.cursor = 0;
      f.placed = NONE;
      s->frames[s->n_frames++] = f;
    }
  }

  for (int i = 0; i < n_cells; ++i) s->best_board[i] = s->board[i];
  s->depth = 0;
  s->status = ST_RUNNING;
  s->nodes = 0;
  s->attempts = 0;
  s->backtracks = 0;
  s->best_placed = s->hint_count;
  return true;
}

static inline u32 solver_placed(const Solver* s) {
  return s->hint_count + (u32)s->depth;
}

// fits(pos, edges) — ALGORITHM.md §7.3. Mirrors solver.rs::fits.
static bool solver_fits(const Solver* s, int pos, Edges e) {
  int w = s->width, h = s->height;
  int x = pos % w, y = pos / w;
  u8 top = e.e[0], right = e.e[1], bottom = e.e[2], left = e.e[3];

  // Rim rule: edge is grey iff that side is on the outer rim.
  if ((y == 0)     != (top == BORDER))    return false;
  if ((y == h - 1) != (bottom == BORDER)) return false;
  if ((x == 0)     != (left == BORDER))   return false;
  if ((x == w - 1) != (right == BORDER))  return false;

  // Neighbour rule: match already-placed neighbours in all four directions.
  if (y > 0) {
    i32 nb = s->board[pos - w];
    if (nb >= 0 && s->table[nb].e[2] != top) return false;
  }
  if (y < h - 1) {
    i32 nb = s->board[pos + w];
    if (nb >= 0 && s->table[nb].e[0] != bottom) return false;
  }
  if (x > 0) {
    i32 nb = s->board[pos - 1];
    if (nb >= 0 && s->table[nb].e[1] != left) return false;
  }
  if (x < w - 1) {
    i32 nb = s->board[pos + 1];
    if (nb >= 0 && s->table[nb].e[3] != right) return false;
  }
  return true;
}

// step(budget) — ALGORITHM.md §7.2. Mirrors solver.rs::step.
static void solver_step(Solver* s, u32 budget) {
  u32 remaining = budget;
  while (remaining > 0 && s->status == ST_RUNNING) {
    remaining -= 1;

    if (s->depth == s->n_frames) {
      s->status = ST_SOLVED;
      s->best_placed = solver_placed(s);
      for (int i = 0; i < s->n_cells; ++i) s->best_board[i] = s->board[i];
      break;
    }

    Frame* frame = &s->frames[s->depth];
    int pos = (int)frame->pos;
    u32 cursor = frame->cursor;
    u32 limit = (u32)(s->n_pieces * 4);
    u32 placed_row = NONE;

    while (cursor < limit) {
      int oi = (int)(cursor / 4);
      u32 r = cursor % 4;
      cursor += 1;
      int pid = (int)s->piece_order[oi];
      if (used_contains(s, pid)) {
        cursor = (cursor + 3) & ~3u;  // skip remaining rotations
        continue;
      }
      int row = pid * 4 + (int)r;
      if (!s->distinct[row]) continue;
      s->attempts += 1;
      if (solver_fits(s, pos, s->table[row])) {
        placed_row = (u32)row;
        break;
      }
    }

    if (placed_row != NONE) {
      int pid = (int)(placed_row / 4);
      s->board[pos] = (i32)placed_row;
      used_insert(s, pid);
      Frame* f = &s->frames[s->depth];
      f->cursor = cursor;
      f->placed = placed_row;
      s->depth += 1;
      s->nodes += 1;
      u32 placed = solver_placed(s);
      if (placed > s->best_placed) {
        s->best_placed = placed;
        for (int i = 0; i < s->n_cells; ++i) s->best_board[i] = s->board[i];
      }
    } else {
      // Dead end: reset this frame and undo the previous placement.
      s->frames[s->depth].cursor = 0;
      if (s->depth == 0) {
        s->status = ST_EXHAUSTED;
        break;
      }
      s->depth -= 1;
      Frame* prev = &s->frames[s->depth];
      u32 row = prev->placed;
      prev->placed = NONE;
      s->board[(int)prev->pos] = -1;
      used_remove(s, (int)(row / 4));
      s->backtracks += 1;
    }
  }
}

// score_board — ALGORITHM.md §8. Mirrors solver.rs::score_board.
static u32 score_board(const Puzzle* p, const i32* board) {
  int w = p->width, h = p->height;
  u32 score = 0;
  for (int y = 0; y < h; ++y) {
    for (int x = 0; x < w; ++x) {
      i32 cell = board[y * w + x];
      Edges here;
      bool here_ok = false;
      if (cell >= 0) {
        int pid = cell / 4; u8 r = (u8)(cell % 4);
        if (pid < p->n_pieces) { here = rotated(p->pieces[pid], r); here_ok = true; }
      }
      if (x + 1 < w) {
        i32 rc = board[y * w + x + 1];
        if (here_ok && rc >= 0) {
          int pid = rc / 4; u8 r = (u8)(rc % 4);
          if (pid < p->n_pieces) {
            Edges b = rotated(p->pieces[pid], r);
            if (here.e[1] == b.e[3] && here.e[1] != BORDER) score += 1;
          }
        }
      }
      if (y + 1 < h) {
        i32 dc = board[(y + 1) * w + x];
        if (here_ok && dc >= 0) {
          int pid = dc / 4; u8 r = (u8)(dc % 4);
          if (pid < p->n_pieces) {
            Edges b = rotated(p->pieces[pid], r);
            if (here.e[2] == b.e[0] && here.e[2] != BORDER) score += 1;
          }
        }
      }
    }
  }
  return score;
}

// =============================================================================
// WASM ABI
//
// The JS glue passes data through shared linear memory. To keep the boundary
// tiny we expose a handful of exports plus a single shared scratch buffer that
// JS reads/writes:
//
//   - Puzzles are returned by writing into a scratch region the JS side then
//     reads (width, height, num_colors, n_pieces, then 4 bytes per piece,
//     n_hints, then 3 shorts per hint).
//   - Paths and boards are written into / read from int regions.
//   - A small solver pool (E2_MAX_SOLVERS slots) backs createSolver/free.
// =============================================================================

// Scratch for one puzzle's wire form. Layout written by e2_*_puzzle:
//   [0] width [1] height [2] num_colors  (each as i32)
//   [3] n_pieces
//   then n_pieces * 4 colour bytes packed into i32s? — simpler: i32 per colour.
// We keep it byte-simple: see glue.ts for the matching reader.
static constexpr int SCRATCH_I32 = 4096;
static i32 g_scratch[SCRATCH_I32];

// A puzzle staging area: JS fills this before calling solver/score, and the
// engine fills it when generating. Flat ints, large enough for 16x16.
//   wire[0]=width wire[1]=height wire[2]=num_colors wire[3]=n_pieces
//   wire[4 .. 4+n_pieces*4) = edges (u/r/d/l per piece)
//   wire[4+n_pieces*4]      = n_hints
//   then 3 ints per hint (pos, piece, rot)
static i32 g_wire[8 + MAX_PIECES * 4 + 8 * 3];

static void puzzle_to_wire(const Puzzle* p) {
  g_wire[0] = p->width;
  g_wire[1] = p->height;
  g_wire[2] = p->num_colors;
  g_wire[3] = p->n_pieces;
  int o = 4;
  for (int i = 0; i < p->n_pieces; ++i) {
    g_wire[o++] = p->pieces[i].e[0];
    g_wire[o++] = p->pieces[i].e[1];
    g_wire[o++] = p->pieces[i].e[2];
    g_wire[o++] = p->pieces[i].e[3];
  }
  g_wire[o++] = p->n_hints;
  for (int i = 0; i < p->n_hints; ++i) {
    g_wire[o++] = p->hints[i].pos;
    g_wire[o++] = p->hints[i].piece;
    g_wire[o++] = p->hints[i].rot;
  }
}

static void wire_to_puzzle(Puzzle* p) {
  p->width = (u8)g_wire[0];
  p->height = (u8)g_wire[1];
  p->num_colors = (u8)g_wire[2];
  p->n_pieces = g_wire[3];
  int o = 4;
  for (int i = 0; i < p->n_pieces; ++i) {
    p->pieces[i].e[0] = (u8)g_wire[o++];
    p->pieces[i].e[1] = (u8)g_wire[o++];
    p->pieces[i].e[2] = (u8)g_wire[o++];
    p->pieces[i].e[3] = (u8)g_wire[o++];
  }
  p->n_hints = g_wire[o++];
  for (int i = 0; i < p->n_hints; ++i) {
    p->hints[i].pos   = (u16)g_wire[o++];
    p->hints[i].piece = (u16)g_wire[o++];
    p->hints[i].rot   = (u8)g_wire[o++];
  }
}

static constexpr int E2_MAX_SOLVERS = 8;
static Solver g_solvers[E2_MAX_SOLVERS];

// Force each ABI function to be kept and exported under its own name. Using an
// explicit export_name makes wasm-ld emit the export and survives a later
// `wasm-opt` pass (plain visibility can be stripped as dead code). The
// export_name attribute only applies to the wasm target; on a native build
// (the debug `main` below) it is meaningless, so we drop it there.
#ifdef __wasm__
#define E2_EXPORT(name) __attribute__((used, visibility("default"), export_name(#name))) name
#else
#define E2_EXPORT(name) name
#endif

extern "C" {

// Pointer to the wire puzzle buffer (JS reads/writes ints here).
i32* E2_EXPORT(e2_wire_ptr)() { return g_wire; }
// Pointer to the generic int scratch (paths, boards).
i32* E2_EXPORT(e2_scratch_ptr)() { return g_scratch; }
int  E2_EXPORT(e2_scratch_len)() { return SCRATCH_I32; }

// ---- Puzzle factories: fill g_wire, return n_pieces -------------------------
int E2_EXPORT(e2_official)() {
  Puzzle p;
  official_puzzle(&p);
  puzzle_to_wire(&p);
  return p.n_pieces;
}
int E2_EXPORT(e2_generate)(int size, int colors, u32 seed) {
  Puzzle p;
  generate(&p, (u8)size, (u8)colors, seed);
  puzzle_to_wire(&p);
  return p.n_pieces;
}
int E2_EXPORT(e2_generate_solved)(int size, int colors, u32 seed) {
  Puzzle p;
  generate_solved(&p, (u8)size, (u8)colors, seed);
  puzzle_to_wire(&p);
  return p.n_pieces;
}
int E2_EXPORT(e2_max_colors)(int size) { return (int)max_colors((u8)size); }

int E2_EXPORT(e2_path_count)() { return (int)PK_COUNT; }

// Build a path into g_scratch; returns length, or -1 on bad kind.
int E2_EXPORT(e2_build_path)(int kind, int width, int height, u32 seed) {
  if (width * height > SCRATCH_I32) return -1;
  u16 tmp[MAX_CELLS];
  int n = build_path(kind, (u8)width, (u8)height, seed, tmp);
  if (n < 0) return -1;
  for (int i = 0; i < n; ++i) g_scratch[i] = (i32)tmp[i];
  return n;
}

// Score the puzzle currently in g_wire against a board in g_scratch (n cells).
int E2_EXPORT(e2_score_board)(int n_cells) {
  (void)n_cells;  // board length lives implicitly in the puzzle dimensions
  Puzzle p;
  wire_to_puzzle(&p);
  return (int)score_board(&p, g_scratch);
}

// ---- Solver pool ------------------------------------------------------------
// Create a solver from g_wire (puzzle) + g_scratch (path of path_len cells).
// Returns a slot handle >= 0, or -1 if no slot / invalid config.
int E2_EXPORT(e2_solver_new)(int path_len, int use_hints, int shuffle_pieces, u32 seed) {
  int slot = -1;
  for (int i = 0; i < E2_MAX_SOLVERS; ++i) {
    if (!g_solvers[i].in_use) { slot = i; break; }
  }
  if (slot < 0) return -1;
  Solver* s = &g_solvers[slot];
  wire_to_puzzle(&s->puzzle);
  s->path_len = path_len;
  for (int i = 0; i < path_len; ++i) s->path[i] = (u16)g_scratch[i];
  s->use_hints = use_hints != 0;
  s->shuffle_pieces = shuffle_pieces != 0;
  s->seed = seed;
  if (!solver_init(s)) return -1;
  s->in_use = true;
  return slot;
}

void E2_EXPORT(e2_solver_free)(int slot) {
  if (slot < 0 || slot >= E2_MAX_SOLVERS) return;
  g_solvers[slot].in_use = false;
}

void E2_EXPORT(e2_solver_reset)(int slot) {
  if (slot < 0 || slot >= E2_MAX_SOLVERS) return;
  solver_init(&g_solvers[slot]);
}

void E2_EXPORT(e2_solver_step)(int slot, u32 budget) {
  if (slot < 0 || slot >= E2_MAX_SOLVERS) return;
  solver_step(&g_solvers[slot], budget);
}

// Report fields, individually (avoids a struct ABI). All return doubles via
// separate getters so counters that exceed 2^32 stay exact (ALGORITHM.md §7.4).
int    E2_EXPORT(e2_solver_status)(int slot)      { return (int)g_solvers[slot].status; }
double E2_EXPORT(e2_solver_nodes)(int slot)       { return (double)g_solvers[slot].nodes; }
double E2_EXPORT(e2_solver_attempts)(int slot)    { return (double)g_solvers[slot].attempts; }
double E2_EXPORT(e2_solver_backtracks)(int slot)  { return (double)g_solvers[slot].backtracks; }
int    E2_EXPORT(e2_solver_placed)(int slot)      { return (int)solver_placed(&g_solvers[slot]); }
int    E2_EXPORT(e2_solver_best_placed)(int slot) { return (int)g_solvers[slot].best_placed; }

// Copy the current board into g_scratch; returns n_cells.
int E2_EXPORT(e2_solver_board)(int slot) {
  Solver* s = &g_solvers[slot];
  for (int i = 0; i < s->n_cells; ++i) g_scratch[i] = s->board[i];
  return s->n_cells;
}
// Copy the best board into g_scratch; returns n_cells.
int E2_EXPORT(e2_solver_best_board)(int slot) {
  Solver* s = &g_solvers[slot];
  for (int i = 0; i < s->n_cells; ++i) g_scratch[i] = s->best_board[i];
  return s->n_cells;
}
// Score helpers operating on the solver's own puzzle (so JS needn't reship it).
int E2_EXPORT(e2_solver_score)(int slot) {
  Solver* s = &g_solvers[slot];
  return (int)score_board(&s->puzzle, s->board);
}
int E2_EXPORT(e2_solver_best_score)(int slot) {
  Solver* s = &g_solvers[slot];
  return (int)score_board(&s->puzzle, s->best_board);
}

}  // extern "C"

// =============================================================================
// Native debugging aid: re-emit golden.txt to stdout when built natively.
// Not compiled for wasm. Use: clang++ -std=c++20 -DE2_NATIVE_MAIN engine.cpp
// =============================================================================
#if !defined(__wasm__) && defined(E2_NATIVE_MAIN)
#include <cstdio>

static void print_gen(int size, int colors, u32 seed) {
  Puzzle p;
  generate(&p, (u8)size, (u8)colors, seed);
  printf("GEN %d %d %u", size, colors, seed);
  for (int i = 0; i < p.n_pieces; ++i)
    printf(" %d,%d,%d,%d", p.pieces[i].e[0], p.pieces[i].e[1], p.pieces[i].e[2], p.pieces[i].e[3]);
  printf("\n");
}

static const char* PK_NAMES[] = {
    "row-major", "snake", "column-major", "spiral-in", "spiral-out",
    "diagonal", "border-first", "double-snake", "random"};

int main() {
  print_gen(4, 4, 7);
  print_gen(5, 6, 42);
  print_gen(3, 3, 1);
  print_gen(6, 5, 99);

  Puzzle off; official_puzzle(&off);
  printf("OFF pieces=%d colors=%d hints=%d\n", off.n_pieces, off.num_colors, off.n_hints);
  for (int i = 0; i < off.n_hints; ++i)
    printf("OFFHINT %d %d %d\n", off.hints[i].pos, off.hints[i].piece, off.hints[i].rot);

  for (int k = 0; k < PK_COUNT; ++k) {
    for (int sz = 3; sz <= 5; ++sz) {
      u16 tmp[MAX_CELLS];
      int n = build_path(k, (u8)sz, (u8)sz, 1, tmp);
      printf("PATH %s %d %d", PK_NAMES[k], sz, sz);
      for (int i = 0; i < n; ++i) printf(" %d", tmp[i]);
      printf("\n");
    }
  }

  for (int k = 0; k < PK_COUNT; ++k) {
    Puzzle p; generate(&p, 4, 4, 11);
    Solver* s = &g_solvers[0];
    wire_to_puzzle(&s->puzzle);  // not used; init from p directly:
    s->puzzle = p;
    u16 tmp[MAX_CELLS];
    int n = build_path(k, 4, 4, 0, tmp);
    s->path_len = n;
    for (int i = 0; i < n; ++i) s->path[i] = tmp[i];
    s->use_hints = true; s->shuffle_pieces = false; s->seed = 0;
    solver_init(s);
    while (s->status == ST_RUNNING) solver_step(s, 5000000);
    u32 sc = score_board(&p, s->board);
    const char* st = s->status == ST_SOLVED ? "solved" : s->status == ST_EXHAUSTED ? "exhausted" : "running";
    printf("SOLVE %s status=%s placed=%u score=%u nodes=%llu attempts=%llu backtracks=%llu\n",
           PK_NAMES[k], st, solver_placed(s), sc, s->nodes, s->attempts, s->backtracks);
  }

  {
    Puzzle p; official_puzzle(&p);
    Solver* s = &g_solvers[1];
    s->puzzle = p;
    u16 tmp[MAX_CELLS];
    int n = build_path(PK_ROW_MAJOR, 16, 16, 0, tmp);
    s->path_len = n;
    for (int i = 0; i < n; ++i) s->path[i] = tmp[i];
    s->use_hints = true; s->shuffle_pieces = false; s->seed = 0;
    solver_init(s);
    solver_step(s, 50000);
    const char* st = s->status == ST_SOLVED ? "solved" : s->status == ST_EXHAUSTED ? "exhausted" : "running";
    printf("OFFICIALRUN budget=50000 status=%s placed=%u best=%u nodes=%llu attempts=%llu backtracks=%llu\n",
           st, solver_placed(s), s->best_placed, s->nodes, s->attempts, s->backtracks);
  }
  return 0;
}
#endif
