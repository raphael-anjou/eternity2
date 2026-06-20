/*
 * Eternity II engine — C ISO re-implementation.
 * =============================================
 *
 * This is a faithful, literal port of the Rust engine in engine/src (*.rs),
 * following engine-side-quests/ALGORITHM.md section by section. Names, steps
 * and ordering mirror the Rust source so the two agree byte-for-byte. The
 * parity suite (parity.mjs) checks every line of the Rust-produced
 * engine-lua/golden.txt against this engine's output.
 *
 * Target: freestanding wasm32 (clang --target=wasm32 -nostdlib --no-entry).
 * No libc, no malloc/free: all working memory is static, fixed-size buffers
 * sized for the official 16x16 board (256 pieces / 256 cells), which is the
 * largest puzzle the website ever runs. This is the simplest, allocation-free
 * choice and is documented in README.md.
 *
 * The same file also compiles natively (clang -O2 engine.c, with the
 * #ifndef __wasm__ main below) for an ABI-independent golden diff.
 *
 * ABI: WASM cannot pass structs/strings across the JS boundary, so the
 * exported functions read/write primitives plus a shared scratch region in
 * linear memory. See the "ABI" section at the bottom and glue.ts.
 */

/* ---- Fixed-width integer typedefs (no stdint under -nostdlib). ---------- */
typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef signed   int       i32;
typedef unsigned long long u64;

#define E2_MAX_SIZE   16
#define E2_MAX_CELLS  (E2_MAX_SIZE * E2_MAX_SIZE)   /* 256 */
#define E2_MAX_PIECES E2_MAX_CELLS                  /* 256 */
#define U32_MAX       0xFFFFFFFFu

#include "official_data.h"

/* ======================================================================== *
 * 2. Conventions — rotation (ALGORITHM.md S2.3, types.rs::rotated)          *
 *                                                                           *
 * r = clockwise quarter-turns. new[i] = old[(i + 4 - r) % 4].               *
 * ======================================================================== */
static void rotated(const u8 e[4], u8 r, u8 out[4]) {
    r &= 3;
    out[0] = e[(4 - r) & 3];
    out[1] = e[(5 - r) & 3];
    out[2] = e[(6 - r) & 3];
    out[3] = e[(7 - r) & 3];
}

#define BORDER 0u

/* ======================================================================== *
 * 3. The RNG: XorShift + splitmix64 seeding (generator.rs::XorShift)        *
 *                                                                           *
 * All math is unsigned 64-bit with wraparound — native in C, no masking.    *
 * ======================================================================== */
typedef struct { u64 state; } XorShift;

static void xs_new(XorShift *x, u32 seed) {
    /* 3.1 Seed through one round of splitmix64, then force non-zero. */
    u64 z = (u64)seed + 0x9E3779B97F4A7C15ull;
    z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9ull;
    z = (z ^ (z >> 27)) * 0x94D049BB133111EBull;
    x->state = (z ^ (z >> 31)) | 1ull;
}

static u32 xs_next_u32(XorShift *x) {
    /* 3.2 xorshift64 step; return the high 32 bits. */
    u64 v = x->state;
    v ^= v << 13;
    v ^= v >> 7;
    v ^= v << 17;
    x->state = v;
    return (u32)(v >> 32);
}

/* below(n) -> [0, n) */
static u32 xs_below(XorShift *x, u32 n) { return xs_next_u32(x) % n; }

/* Fisher-Yates, last index down to 1, swapping i with below(i+1). */
static void xs_shuffle_u8(XorShift *x, u8 *a, u32 len) {
    for (u32 i = len - 1; i >= 1; i--) {
        u32 j = xs_below(x, i + 1);
        u8 t = a[i]; a[i] = a[j]; a[j] = t;
        if (i == 1) break; /* unsigned guard: i-- past 1 would wrap */
    }
}

static void xs_shuffle_u16(XorShift *x, u16 *a, u32 len) {
    for (u32 i = len - 1; i >= 1; i--) {
        u32 j = xs_below(x, i + 1);
        u16 t = a[i]; a[i] = a[j]; a[j] = t;
        if (i == 1) break;
    }
}

/* ======================================================================== *
 * 4. Puzzle model + generator (types.rs, generator.rs)                      *
 *                                                                           *
 * A Puzzle is held in static buffers. pieces[i] are the URDL edges in       *
 * rotation 0; hints are {pos, piece, rot}.                                  *
 * ======================================================================== */
typedef struct { u16 pos; u16 piece; u8 rot; } Hint;

typedef struct {
    u8   width;
    u8   height;
    u8   num_colors;
    u16  n_pieces;
    u8   pieces[E2_MAX_PIECES][4];
    u16  n_hints;
    Hint hints[8];
} Puzzle;

static u32 interior_edge_count(u8 size) {
    return 2u * (u32)size * ((u32)size - 1u);
}

/* max_colors(size) = min(2n(n-1), 22) */
static u8 e2_max_colors(u8 size) {
    u32 c = interior_edge_count(size);
    return (u8)(c < 22u ? c : 22u);
}

/*
 * generate_solved (generator.rs::generate_solved): build a solved board by
 * painting interior adjacencies, then derive each piece. Piece i sits at cell
 * i, rotation 0 — the identity board is the solution. The pieces stay in
 * solution order/orientation (no scramble).
 */
static void generate_solved(Puzzle *p, u8 size, u8 colors, u32 seed) {
    /* colors.clamp(1, max_colors(size)) */
    u8 mc = e2_max_colors(size);
    if (colors < 1) colors = 1;
    if (colors > mc) colors = mc;

    u32 s = size;
    u32 n_edges = interior_edge_count(size);
    XorShift rng; xs_new(&rng, seed);

    /* One color per interior adjacency, every color present at least once. */
    static u8 palette[2 * E2_MAX_SIZE * (E2_MAX_SIZE - 1)];
    for (u32 i = 0; i < n_edges; i++) {
        if (i < (u32)colors) palette[i] = (u8)(i + 1);
        else                 palette[i] = (u8)(xs_below(&rng, (u32)colors) + 1);
    }
    xs_shuffle_u8(&rng, palette, n_edges);

    /* palette[..s*(s-1)] = vertical, palette[s*(s-1)..] = horizontal.
     * vert[y*(s-1)+x] = color between (x,y) and (x+1,y);
     * horiz[y*s+x]    = color between (x,y) and (x,y+1).            */
    const u8 *vert  = palette;
    const u8 *horiz = palette + s * (s - 1);

    u16 idx = 0;
    for (u32 y = 0; y < s; y++) {
        for (u32 x = 0; x < s; x++) {
            u8 up    = (y == 0)     ? BORDER : horiz[(y - 1) * s + x];
            u8 down  = (y == s - 1) ? BORDER : horiz[y * s + x];
            u8 left  = (x == 0)     ? BORDER : vert[y * (s - 1) + (x - 1)];
            u8 right = (x == s - 1) ? BORDER : vert[y * (s - 1) + x];
            p->pieces[idx][0] = up;
            p->pieces[idx][1] = right;
            p->pieces[idx][2] = down;
            p->pieces[idx][3] = left;
            idx++;
        }
    }

    p->width = size;
    p->height = size;
    p->num_colors = colors;
    p->n_pieces = (u16)(s * s);
    p->n_hints = 0;
}

/*
 * generate (generator.rs::generate): generate_solved, then scramble with a
 * SECOND rng seeded seed ^ 0xA5A5A5A5 — shuffle the piece list, then give
 * each piece a random rotation. Solvable by construction.
 */
static void generate(Puzzle *p, u8 size, u8 colors, u32 seed) {
    generate_solved(p, size, colors, seed);
    XorShift rng; xs_new(&rng, seed ^ 0xA5A5A5A5u);

    /* Shuffle pieces (Fisher-Yates over rows of 4 edges). */
    u32 n = p->n_pieces;
    for (u32 i = n - 1; i >= 1; i--) {
        u32 j = xs_below(&rng, i + 1);
        for (int k = 0; k < 4; k++) {
            u8 t = p->pieces[i][k]; p->pieces[i][k] = p->pieces[j][k]; p->pieces[j][k] = t;
        }
        if (i == 1) break;
    }
    /* Random rotation per piece. */
    for (u32 i = 0; i < n; i++) {
        u8 out[4];
        rotated(p->pieces[i], (u8)xs_below(&rng, 4), out);
        for (int k = 0; k < 4; k++) p->pieces[i][k] = out[k];
    }
}

/* official_puzzle (official.rs): from the embedded decoded CSV data. */
static void official_puzzle(Puzzle *p) {
    p->width = E2_OFFICIAL_SIZE;
    p->height = E2_OFFICIAL_SIZE;
    p->n_pieces = E2_OFFICIAL_PIECES;
    u8 max_color = 0;
    for (int i = 0; i < E2_OFFICIAL_PIECES; i++) {
        for (int k = 0; k < 4; k++) {
            u8 c = E2_OFFICIAL_EDGES[i][k];
            p->pieces[i][k] = c;
            if (c > max_color) max_color = c;
        }
    }
    p->num_colors = max_color;
    p->n_hints = E2_OFFICIAL_HINTS;
    for (int i = 0; i < E2_OFFICIAL_HINTS; i++) {
        p->hints[i].pos   = (u16)E2_OFFICIAL_HINT[i][0];
        p->hints[i].piece = (u16)E2_OFFICIAL_HINT[i][1];
        p->hints[i].rot   = (u8) E2_OFFICIAL_HINT[i][2];
    }
}

/* ======================================================================== *
 * 6. Cell-visit orders ("paths") — paths.rs::build_path                     *
 *                                                                           *
 * Returns the count, writing the permutation into out[]. Returns -1 for an  *
 * unknown kind. Kind is passed as an index into PATH_KINDS (the glue maps   *
 * the name->index so the WASM ABI stays primitive-only).                    *
 * ======================================================================== */
static const char *PATH_KINDS[] = {
    "row-major", "snake", "column-major", "spiral-in", "spiral-out",
    "diagonal", "border-first", "double-snake", "random",
};
#define PATH_KIND_COUNT 9

/* tiny strcmp (freestanding) */
static int str_eq(const char *a, const char *b) {
    while (*a && (*a == *b)) { a++; b++; }
    return *a == *b;
}

static int path_kind_index(const char *kind) {
    for (int i = 0; i < PATH_KIND_COUNT; i++)
        if (str_eq(kind, PATH_KINDS[i])) return i;
    return -1;
}

/* Build path by kind index. out must hold w*h entries. Returns w*h or -1. */
static i32 build_path(int kind, u8 width, u8 height, u32 seed, u16 *out) {
    int w = width, h = height;
    int n = 0;
    #define PUSH(X, Y) (out[n++] = (u16)((Y) * w + (X)))

    switch (kind) {
    case 0: /* row-major */
        for (int y = 0; y < h; y++) for (int x = 0; x < w; x++) PUSH(x, y);
        break;
    case 1: /* snake */
        for (int y = 0; y < h; y++) {
            if (y % 2 == 0) for (int x = 0; x < w; x++) PUSH(x, y);
            else            for (int x = w - 1; x >= 0; x--) PUSH(x, y);
        }
        break;
    case 2: /* column-major */
        for (int x = 0; x < w; x++) for (int y = 0; y < h; y++) PUSH(x, y);
        break;
    case 3: /* spiral-in */
    case 4: /* spiral-out */
    {
        int x0 = 0, y0 = 0, x1 = w, y1 = h;
        while (x0 < x1 && y0 < y1) {
            for (int x = x0; x < x1; x++) PUSH(x, y0);
            for (int y = y0 + 1; y < y1; y++) PUSH(x1 - 1, y);
            if (y1 > y0 + 1)
                for (int x = x1 - 2; x >= x0; x--) PUSH(x, y1 - 1);
            if (x1 > x0 + 1)
                for (int y = y1 - 2; y >= y0 + 1; y--) PUSH(x0, y);
            x0++; y0++; x1--; y1--;
        }
        if (kind == 4) { /* spiral-out: reverse */
            for (int i = 0, j = n - 1; i < j; i++, j--) {
                u16 t = out[i]; out[i] = out[j]; out[j] = t;
            }
        }
        break;
    }
    case 5: /* diagonal */
        for (int d = 0; d < w + h - 1; d++)
            for (int y = 0; y < h; y++)
                if (d >= y && d - y < w) PUSH(d - y, y);
        break;
    case 6: /* border-first: rim clockwise then interior row-major */
        for (int x = 0; x < w; x++) PUSH(x, 0);
        for (int y = 1; y < h; y++) PUSH(w - 1, y);
        if (h > 1) for (int x = w - 2; x >= 0; x--) PUSH(x, h - 1);
        if (w > 1) for (int y = h - 2; y >= 1; y--) PUSH(0, y);
        for (int y = 1; y < (h > 0 ? h - 1 : 0); y++)
            for (int x = 1; x < (w > 0 ? w - 1 : 0); x++) PUSH(x, y);
        break;
    case 7: /* double-snake: two rows at a time, zig-zagging */
    {
        int y = 0;
        while (y < h) {
            int rows[2]; int nrows;
            if (y + 1 < h) { rows[0] = y; rows[1] = y + 1; nrows = 2; }
            else           { rows[0] = y; nrows = 1; }
            if ((y / 2) % 2 == 0) {
                for (int x = 0; x < w; x++)
                    for (int ri = 0; ri < nrows; ri++) PUSH(x, rows[ri]);
            } else {
                for (int x = w - 1; x >= 0; x--)
                    for (int ri = 0; ri < nrows; ri++) PUSH(x, rows[ri]);
            }
            y += 2;
        }
        break;
    }
    case 8: /* random: seeded shuffle of all cells */
    {
        for (int i = 0; i < w * h; i++) out[i] = (u16)i;
        XorShift rng; xs_new(&rng, seed);
        xs_shuffle_u16(&rng, out, (u32)(w * h));
        n = w * h;
        break;
    }
    default:
        return -1;
    }
    #undef PUSH
    return n;
}

/* ======================================================================== *
 * 7. Step-able backtracking DFS — solver.rs::Solver                         *
 * ======================================================================== */
typedef struct {
    u16 pos;
    u32 cursor;  /* next (order_index*4 + rotation) to try */
    u32 placed;  /* order_index*4+rot placed here, or U32_MAX */
} Frame;

enum { ST_RUNNING = 0, ST_SOLVED = 1, ST_EXHAUSTED = 2 };

typedef struct {
    int width, height;
    int n_pieces;
    u8  table[E2_MAX_PIECES * 4][4];   /* rotated edges per (piece,rot) */
    u8  distinct[E2_MAX_PIECES * 4];   /* false if dup of lower rotation */
    u16 piece_order[E2_MAX_PIECES];
    i32 board[E2_MAX_CELLS];           /* cell -> piece*4+rot, or -1 */
    u64 used[E2_MAX_PIECES / 64 + 1];  /* availability bitset */
    Frame frames[E2_MAX_CELLS];
    int  n_frames;
    int  depth;
    u32  hint_count;
    int  status;
    u64  nodes, attempts, backtracks;
    u32  best_placed;
    i32  best_board[E2_MAX_CELLS];
    const Puzzle *puzzle;
} Solver;

static int bit_get(const u64 *w, int i) { return (int)((w[i >> 6] >> (i & 63)) & 1ull); }
static void bit_set(u64 *w, int i)   { w[i >> 6] |=  (1ull << (i & 63)); }
static void bit_clear(u64 *w, int i) { w[i >> 6] &= ~(1ull << (i & 63)); }

/*
 * Solver::new. Returns 0 on success, -1 on invalid arguments (mirrors the
 * Rust Result; the website only ever passes validated args).
 */
static int solver_new(Solver *s, const Puzzle *p, const u16 *path, int path_len,
                      int use_hints, int shuffle_pieces, u32 seed) {
    int n_cells = (int)p->width * (int)p->height;
    int n_pieces = p->n_pieces;
    if (n_pieces != n_cells) return -1;
    if (path_len != n_cells) return -1;

    /* path must be a permutation */
    static u8 seen[E2_MAX_CELLS];
    for (int i = 0; i < n_cells; i++) seen[i] = 0;
    for (int i = 0; i < path_len; i++) {
        int c = path[i];
        if (c >= n_cells || seen[c]) return -1;
        seen[c] = 1;
    }

    s->puzzle = p;
    s->width = p->width;
    s->height = p->height;
    s->n_pieces = n_pieces;

    /* Rotation table + distinct mask. */
    for (int id = 0; id < n_pieces; id++) {
        for (u8 r = 0; r < 4; r++) {
            u8 re[4];
            rotated(p->pieces[id], r, re);
            int row = id * 4 + r;
            for (int k = 0; k < 4; k++) s->table[row][k] = re[k];
            s->distinct[row] = 1;
            for (u8 prev = 0; prev < r; prev++) {
                int prow = id * 4 + prev;
                if (s->table[prow][0] == re[0] && s->table[prow][1] == re[1] &&
                    s->table[prow][2] == re[2] && s->table[prow][3] == re[3]) {
                    s->distinct[row] = 0;
                    break;
                }
            }
        }
    }

    /* Piece order (optionally seed-shuffled). */
    for (int i = 0; i < n_pieces; i++) s->piece_order[i] = (u16)i;
    if (shuffle_pieces) {
        XorShift rng; xs_new(&rng, seed);
        xs_shuffle_u16(&rng, s->piece_order, (u32)n_pieces);
    }

    /* Board + used + hints. */
    for (int i = 0; i < n_cells; i++) s->board[i] = -1;
    for (unsigned i = 0; i < sizeof(s->used) / sizeof(s->used[0]); i++) s->used[i] = 0;
    s->hint_count = 0;
    if (use_hints) {
        for (int hi = 0; hi < p->n_hints; hi++) {
            const Hint *hn = &p->hints[hi];
            int pos = hn->pos;
            if (pos >= n_cells || hn->piece >= n_pieces || bit_get(s->used, hn->piece))
                return -1;
            s->board[pos] = (i32)hn->piece * 4 + (i32)(hn->rot & 3);
            bit_set(s->used, hn->piece);
            s->hint_count++;
        }
    }

    /* One frame per non-hint cell, in path order. */
    s->n_frames = 0;
    for (int i = 0; i < path_len; i++) {
        int c = path[i];
        if (s->board[c] == -1) {
            Frame *f = &s->frames[s->n_frames++];
            f->pos = (u16)c;
            f->cursor = 0;
            f->placed = U32_MAX;
        }
    }

    for (int i = 0; i < n_cells; i++) s->best_board[i] = s->board[i];
    s->depth = 0;
    s->status = ST_RUNNING;
    s->nodes = s->attempts = s->backtracks = 0;
    s->best_placed = s->hint_count;
    return 0;
}

static u32 solver_placed(const Solver *s) { return s->hint_count + (u32)s->depth; }

/* fits(pos, edges) — solver.rs::fits */
static int fits(const Solver *s, int pos, const u8 e[4]) {
    int w = s->width, h = s->height;
    int x = pos % w, y = pos / w;
    u8 top = e[0], right = e[1], bottom = e[2], left = e[3];

    /* Rim rule: edge grey iff side is on the outer rim. */
    if ((y == 0)     != (top == BORDER))    return 0;
    if ((y == h - 1) != (bottom == BORDER)) return 0;
    if ((x == 0)     != (left == BORDER))   return 0;
    if ((x == w - 1) != (right == BORDER))  return 0;

    /* Neighbour rule: match whichever neighbours are already placed. */
    if (y > 0)     { i32 nb = s->board[pos - w]; if (nb >= 0 && s->table[nb][2] != top)    return 0; }
    if (y < h - 1) { i32 nb = s->board[pos + w]; if (nb >= 0 && s->table[nb][0] != bottom) return 0; }
    if (x > 0)     { i32 nb = s->board[pos - 1]; if (nb >= 0 && s->table[nb][1] != left)   return 0; }
    if (x < w - 1) { i32 nb = s->board[pos + 1]; if (nb >= 0 && s->table[nb][3] != right)  return 0; }
    return 1;
}

/* step(budget) — solver.rs::step. One placement or one backtrack per loop. */
static void solver_step(Solver *s, u32 budget) {
    u32 remaining = budget;
    while (remaining > 0 && s->status == ST_RUNNING) {
        remaining--;

        if (s->depth == s->n_frames) {
            s->status = ST_SOLVED;
            s->best_placed = solver_placed(s);
            for (int i = 0; i < s->width * s->height; i++) s->best_board[i] = s->board[i];
            break;
        }

        Frame *frame = &s->frames[s->depth];
        int pos = frame->pos;
        u32 cursor = frame->cursor;
        u32 limit = (u32)(s->n_pieces * 4);
        u32 placed_row = U32_MAX;

        while (cursor < limit) {
            int oi = (int)(cursor / 4);
            u32 r = cursor % 4;
            cursor++;
            int pid = s->piece_order[oi];
            if (bit_get(s->used, pid)) {
                cursor = (cursor + 3) & ~3u; /* skip remaining rotations */
                continue;
            }
            int row = pid * 4 + (int)r;
            if (!s->distinct[row]) continue;
            s->attempts++;
            if (fits(s, pos, s->table[row])) {
                placed_row = (u32)row;
                break;
            }
        }

        if (placed_row != U32_MAX) {
            int pid = (int)(placed_row / 4);
            s->board[pos] = (i32)placed_row;
            bit_set(s->used, pid);
            frame->cursor = cursor;
            frame->placed = placed_row;
            s->depth++;
            s->nodes++;
            u32 placed = solver_placed(s);
            if (placed > s->best_placed) {
                s->best_placed = placed;
                for (int i = 0; i < s->width * s->height; i++) s->best_board[i] = s->board[i];
            }
        } else {
            frame->cursor = 0;
            if (s->depth == 0) { s->status = ST_EXHAUSTED; break; }
            s->depth--;
            Frame *prev = &s->frames[s->depth];
            u32 row = prev->placed;
            prev->placed = U32_MAX;
            s->board[prev->pos] = -1;
            bit_clear(s->used, (int)(row / 4));
            s->backtracks++;
        }
    }
}

/* ======================================================================== *
 * 8. Scoring — solver.rs::score_board                                      *
 * ======================================================================== */
static int score_edges_of(const Puzzle *p, i32 row, u8 out[4]) {
    if (row < 0) return 0;
    int pid = row / 4, r = row % 4;
    if (pid >= p->n_pieces) return 0;
    rotated(p->pieces[pid], (u8)r, out);
    return 1;
}

static u32 score_board(const Puzzle *p, const i32 *board) {
    int w = p->width, h = p->height;
    u32 score = 0;
    for (int y = 0; y < h; y++) {
        for (int x = 0; x < w; x++) {
            u8 here[4]; int has = score_edges_of(p, board[y * w + x], here);
            if (x + 1 < w) {
                u8 b[4];
                if (has && score_edges_of(p, board[y * w + x + 1], b))
                    if (here[1] == b[3] && here[1] != BORDER) score++;
            }
            if (y + 1 < h) {
                u8 b[4];
                if (has && score_edges_of(p, board[(y + 1) * w + x], b))
                    if (here[2] == b[0] && here[2] != BORDER) score++;
            }
        }
    }
    return score;
}

/* ======================================================================== *
 * ABI for the WASM/JS boundary (see glue.ts).                               *
 *                                                                           *
 * WASM can only pass primitives and pointers, so:                           *
 *  - A single static Puzzle (g_puzzle) is the "current puzzle". The puzzle  *
 *    builders fill it and expose its fields through accessors.              *
 *  - Pieces/paths/boards are read out of fixed scratch regions whose base   *
 *    pointers are returned to JS; JS copies them via typed-array views.     *
 *  - A single static Solver (g_solver) backs createSolver. The site only    *
 *    runs one solver at a time; if it needs more, JS serialises access.     *
 * ======================================================================== */
static Puzzle g_puzzle;          /* puzzle returned by the puzzle builders */
static Solver g_solver;          /* the one active solver                  */
static u16    g_path[E2_MAX_CELLS];  /* scratch for build_path output      */
static i32    g_board_out[E2_MAX_CELLS]; /* scratch for board reads        */

/* --- Exported helpers used by glue.ts (all extern "C"-style by default). */

/* Build a generated puzzle into g_puzzle; returns piece count. */
__attribute__((export_name("e2_generate")))
int e2_generate(int size, int colors, unsigned int seed) {
    generate(&g_puzzle, (u8)size, (u8)colors, seed);
    return g_puzzle.n_pieces;
}

__attribute__((export_name("e2_generate_solved")))
int e2_generate_solved(int size, int colors, unsigned int seed) {
    generate_solved(&g_puzzle, (u8)size, (u8)colors, seed);
    return g_puzzle.n_pieces;
}

__attribute__((export_name("e2_official")))
int e2_official(void) {
    official_puzzle(&g_puzzle);
    return g_puzzle.n_pieces;
}

__attribute__((export_name("e2_puzzle_width")))  int e2_puzzle_width(void)  { return g_puzzle.width; }
__attribute__((export_name("e2_puzzle_height"))) int e2_puzzle_height(void) { return g_puzzle.height; }
__attribute__((export_name("e2_puzzle_colors"))) int e2_puzzle_colors(void) { return g_puzzle.num_colors; }
__attribute__((export_name("e2_puzzle_hints")))  int e2_puzzle_hints(void)  { return g_puzzle.n_hints; }

/* Pointer to g_puzzle.pieces (row-major, 4 u8 per piece). JS reads
 * n_pieces*4 bytes from here. */
__attribute__((export_name("e2_pieces_ptr")))
const u8 *e2_pieces_ptr(void) { return &g_puzzle.pieces[0][0]; }

/* Hint i fields. */
__attribute__((export_name("e2_hint_pos")))   int e2_hint_pos(int i)   { return g_puzzle.hints[i].pos; }
__attribute__((export_name("e2_hint_piece"))) int e2_hint_piece(int i) { return g_puzzle.hints[i].piece; }
__attribute__((export_name("e2_hint_rot")))   int e2_hint_rot(int i)   { return g_puzzle.hints[i].rot; }

__attribute__((export_name("e2_max_colors")))
int e2_max_colors_abi(int size) { return e2_max_colors((u8)size); }

__attribute__((export_name("e2_path_kind_count")))
int e2_path_kind_count(void) { return PATH_KIND_COUNT; }

/* Pointer to the name of path kind i (NUL-terminated C string). */
__attribute__((export_name("e2_path_kind_name")))
const char *e2_path_kind_name(int i) {
    if (i < 0 || i >= PATH_KIND_COUNT) return "";
    return PATH_KINDS[i];
}

/* Build a path by kind index into g_path; returns length, or -1. */
__attribute__((export_name("e2_build_path")))
int e2_build_path(int kind, int width, int height, unsigned int seed) {
    return build_path(kind, (u8)width, (u8)height, seed, g_path);
}

__attribute__((export_name("e2_path_ptr")))
const u16 *e2_path_ptr(void) { return g_path; }

/*
 * Score a board. The caller writes the board (n_cells i32) into g_board_out
 * via e2_board_in_ptr() then calls this. The puzzle is g_puzzle.
 */
__attribute__((export_name("e2_board_in_ptr")))
i32 *e2_board_in_ptr(void) { return g_board_out; }

__attribute__((export_name("e2_score_board")))
int e2_score_board(void) { return (int)score_board(&g_puzzle, g_board_out); }

/*
 * Solver. createSolver first builds the puzzle into g_puzzle (via official/
 * generate above) and a path into g_path, then calls e2_solver_new with the
 * path length. Returns 0 on success, -1 on bad args.
 */
__attribute__((export_name("e2_solver_new")))
int e2_solver_new(int path_len, int use_hints, int shuffle_pieces, unsigned int seed) {
    return solver_new(&g_solver, &g_puzzle, g_path, path_len, use_hints, shuffle_pieces, seed);
}

__attribute__((export_name("e2_solver_step")))
void e2_solver_step(unsigned int budget) { solver_step(&g_solver, budget); }

__attribute__((export_name("e2_solver_status")))    int e2_solver_status(void)    { return g_solver.status; }
/* Counters can exceed 2^32 on long runs; return as f64 (double) like Rust. */
__attribute__((export_name("e2_solver_nodes")))     double e2_solver_nodes(void)      { return (double)g_solver.nodes; }
__attribute__((export_name("e2_solver_attempts")))  double e2_solver_attempts(void)   { return (double)g_solver.attempts; }
__attribute__((export_name("e2_solver_backtracks")))double e2_solver_backtracks(void) { return (double)g_solver.backtracks; }
__attribute__((export_name("e2_solver_placed")))    int e2_solver_placed(void)    { return (int)solver_placed(&g_solver); }
__attribute__((export_name("e2_solver_best_placed")))int e2_solver_best_placed(void){ return (int)g_solver.best_placed; }

/* Copy current/best board into g_board_out; return its pointer. */
__attribute__((export_name("e2_solver_board")))
const i32 *e2_solver_board(void) {
    int n = g_solver.width * g_solver.height;
    for (int i = 0; i < n; i++) g_board_out[i] = g_solver.board[i];
    return g_board_out;
}

__attribute__((export_name("e2_solver_best_board")))
const i32 *e2_solver_best_board(void) {
    int n = g_solver.width * g_solver.height;
    for (int i = 0; i < n; i++) g_board_out[i] = g_solver.best_board[i];
    return g_board_out;
}

__attribute__((export_name("e2_solver_score")))
int e2_solver_score(void) {
    int n = g_solver.width * g_solver.height;
    for (int i = 0; i < n; i++) g_board_out[i] = g_solver.board[i];
    return (int)score_board(&g_puzzle, g_board_out);
}

__attribute__((export_name("e2_solver_best_score")))
int e2_solver_best_score(void) {
    return (int)score_board(&g_puzzle, g_solver.best_board);
}

/*
 * reset(): rebuild the solver from the same puzzle/path. The path is still in
 * g_path and the puzzle in g_puzzle (createSolver keeps them stable), so JS
 * passes back the same parameters.
 */
__attribute__((export_name("e2_solver_reset")))
int e2_solver_reset(int path_len, int use_hints, int shuffle_pieces, unsigned int seed) {
    return solver_new(&g_solver, &g_puzzle, g_path, path_len, use_hints, shuffle_pieces, seed);
}

/* ======================================================================== *
 * Native golden emitter (for ABI-independent debugging).                    *
 * Compiled only when NOT targeting wasm. Mirrors engine/src/bin/golden.rs   *
 * field layout so its output diffs clean against golden.txt.                *
 * ======================================================================== */
#ifndef __wasm__
#include <stdio.h>

static const char *status_name(int st) {
    return st == ST_SOLVED ? "solved" : st == ST_EXHAUSTED ? "exhausted" : "running";
}

static void emit_gen(int size, int colors, u32 seed) {
    generate(&g_puzzle, (u8)size, (u8)colors, seed);
    printf("GEN %d %d %u", size, colors, seed);
    for (int i = 0; i < g_puzzle.n_pieces; i++)
        printf(" %d,%d,%d,%d", g_puzzle.pieces[i][0], g_puzzle.pieces[i][1],
               g_puzzle.pieces[i][2], g_puzzle.pieces[i][3]);
    printf("\n");
}

static void emit_path(const char *kind, int w, int h) {
    int ki = path_kind_index(kind);
    /* golden.rs builds PATH lines with seed 1 (matches the Lua spec). */
    i32 n = build_path(ki, (u8)w, (u8)h, 1, g_path);
    printf("PATH %s %d %d", kind, w, h);
    for (i32 i = 0; i < n; i++) printf(" %d", g_path[i]);
    printf("\n");
}

static void emit_solve(const char *kind) {
    generate(&g_puzzle, 4, 4, 11);
    int ki = path_kind_index(kind);
    i32 n = build_path(ki, 4, 4, 0, g_path);
    solver_new(&g_solver, &g_puzzle, g_path, n, 1, 0, 0);
    do { solver_step(&g_solver, 5000000); } while (g_solver.status == ST_RUNNING);
    int n2 = g_solver.width * g_solver.height;
    for (int i = 0; i < n2; i++) g_board_out[i] = g_solver.board[i];
    u32 sc = score_board(&g_puzzle, g_board_out);
    printf("SOLVE %s status=%s placed=%u score=%u nodes=%llu attempts=%llu backtracks=%llu\n",
           kind, status_name(g_solver.status), solver_placed(&g_solver), sc,
           g_solver.nodes, g_solver.attempts, g_solver.backtracks);
}

int main(void) {
    emit_gen(4, 4, 7);
    emit_gen(5, 6, 42);
    emit_gen(3, 3, 1);
    emit_gen(6, 5, 99);

    official_puzzle(&g_puzzle);
    printf("OFF pieces=%d colors=%d hints=%d\n", g_puzzle.n_pieces, g_puzzle.num_colors, g_puzzle.n_hints);
    for (int i = 0; i < g_puzzle.n_hints; i++)
        printf("OFFHINT %d %d %d\n", g_puzzle.hints[i].pos, g_puzzle.hints[i].piece, g_puzzle.hints[i].rot);

    const char *kinds[] = {"row-major","snake","column-major","spiral-in","spiral-out",
                           "diagonal","border-first","double-snake","random"};
    /* PATH lines exactly as golden.rs emits them: for each kind, sizes 3,4,5. */
    for (int k = 0; k < 9; k++)
        for (int s = 3; s <= 5; s++)
            emit_path(kinds[k], s, s);

    for (int k = 0; k < 9; k++) emit_solve(kinds[k]);

    /* OFFICIALRUN budget=50000 */
    official_puzzle(&g_puzzle);
    int ki = path_kind_index("row-major");
    i32 n = build_path(ki, 16, 16, 0, g_path);
    solver_new(&g_solver, &g_puzzle, g_path, n, 1, 0, 0);
    solver_step(&g_solver, 50000);
    printf("OFFICIALRUN budget=50000 status=%s placed=%u best=%u nodes=%llu attempts=%llu backtracks=%llu\n",
           status_name(g_solver.status), solver_placed(&g_solver), g_solver.best_placed,
           g_solver.nodes, g_solver.attempts, g_solver.backtracks);
    return 0;
}
#endif
