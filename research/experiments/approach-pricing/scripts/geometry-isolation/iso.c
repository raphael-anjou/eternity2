/* TOY BOARDS ONLY (generated edge-matching boards; never Eternity II progress).
 *
 * Are solutions isolated in the space of full boards?  One program, several subcommands.
 *
 * Board space: every cell holds one tile of its class (corner, edge, inner); rim tiles sit in
 * the one rotation that puts grey on the rim, so the rim is always grey; inner tiles take any of
 * four rotations.  Cell 0 holds the pinned corner (first corner tile in painter order, grey on
 * N and W), never moved, which removes the four global rotations exactly as in common.py.
 * Mismatches = unmatched inner edges (2n(n-1) inner edges in all).  Hamming distance between two
 * boards = number of cells whose (tile, rotation) differ.
 *
 * Move sets (all act inside one class, pin excluded):
 *   M1: rotate one inner tile; swap two tiles of one class, each landing in any legal rotation.
 *   M2: M1 plus 3-cycles of tiles of one class, any legal rotations.
 *   M3: M1 plus any permutation of the four tiles of a 2x2 block of inner cells, any rotations
 *       (this contains the rigid rotation of the block).
 *
 * Subcommands (board file from export_boards.py):
 *   iso F enum                          all solutions, DFS nodes
 *   iso F basin MS KMAX CAP             BFS from every solution in the level set {mismatch <= k},
 *                                       k = 0..KMAX, explored set capped at CAP boards
 *   iso F beam W MAXPAIRS               upper bound on the M1 barrier between solution pairs:
 *                                       beam search over monotone paths (each step puts one more
 *                                       cell into the target's placement), minimising the max
 *                                       mismatch along the path
 *   iso F anneal RUNS STEPS SEED SNAPS [T0 T1 TRAPK TRAPCAP]
 *                                       plain annealer on mismatches with M1 moves; with TRAPK, a
 *                                       final board with 1..TRAPK mismatches is BFS-checked for an
 *                                       M1 path to a solution inside {mismatch <= TRAPK}
 *   iso F count K BUDGET                exact count of complete boards with exactly j <= K
 *                                       mismatches (row-scan DFS tolerating K), node budget
 *   iso F knuth K PROBES SEED           Knuth estimator of the same counts (high variance, unused)
 *   iso F hybrid K D PROBES SEED BUDGET random path to depth D, exact tolerant count below it
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>
#include <time.h>

#define MAXN 49
#define MAXT 49

static int n, N, T, E;
static int tiles[MAXT][4];
static int cls_cell[MAXN];          /* 0 inner, 1 edge, 2 corner */
static int cls_tile[MAXT];
static int rotOK[MAXN][MAXT];       /* rim cells: the one legal rotation, -1 if none; inner: 0 */
static int nbr[MAXN][4];
static int pinT, pinR;
static int cells_of[3][MAXN], ncells_of[3];   /* free cells (pin excluded) */
static int tile_[MAXN], rot_[MAXN];           /* working board */

static double now(void) { struct timespec t; clock_gettime(CLOCK_MONOTONIC, &t); return t.tv_sec + 1e-9 * t.tv_nsec; }

static inline int col(int x, int d) { return tiles[tile_[x]][(d + rot_[x]) & 3]; }
static inline int mis(int x, int d) { int y = nbr[x][d]; return y >= 0 && col(x, d) != col(y, (d + 2) & 3); }
static int total_mis(void) {
    int m = 0;
    for (int x = 0; x < N; x++) { m += mis(x, 1); m += mis(x, 2); }
    return m;
}

/* ---------- RNG ---------- */
static uint64_t rs = 88172645463325252ULL;
static inline uint64_t xr(void) { rs ^= rs << 13; rs ^= rs >> 7; rs ^= rs << 17; return rs; }
static inline int rnd(int k) { return (int)(xr() % (uint64_t)k); }
static inline double urand(void) { return (xr() >> 11) * (1.0 / 9007199254740992.0); }

static void load(const char *fn) {
    FILE *f = fopen(fn, "r");
    if (!f) { perror(fn); exit(1); }
    if (fscanf(f, "%d %d", &n, &T) != 2) exit(1);
    for (int i = 0; i < T; i++) for (int d = 0; d < 4; d++) if (fscanf(f, "%d", &tiles[i][d]) != 1) exit(1);
    fclose(f);
    N = n * n; E = 2 * n * (n - 1);
    for (int x = 0; x < N; x++) {
        int r = x / n, c = x % n;
        nbr[x][0] = r > 0 ? x - n : -1; nbr[x][1] = c < n - 1 ? x + 1 : -1;
        nbr[x][2] = r < n - 1 ? x + n : -1; nbr[x][3] = c > 0 ? x - 1 : -1;
        int rim[4] = {r == 0, c == n - 1, r == n - 1, c == 0};
        cls_cell[x] = rim[0] + rim[1] + rim[2] + rim[3];
        for (int t = 0; t < T; t++) {
            rotOK[x][t] = -1;
            for (int k = 0; k < 4; k++) {
                int ok = 1;
                for (int d = 0; d < 4; d++) if ((tiles[t][(d + k) & 3] == 0) != rim[d]) ok = 0;
                if (ok) { rotOK[x][t] = k; break; }
            }
        }
    }
    for (int t = 0; t < T; t++) { int z = 0; for (int d = 0; d < 4; d++) z += tiles[t][d] == 0; cls_tile[t] = z; }
    pinT = -1;
    for (int t = 0; t < T && pinT < 0; t++) if (cls_tile[t] == 2) {
        for (int k = 0; k < 4; k++) if (tiles[t][k & 3] == 0 && tiles[t][(3 + k) & 3] == 0) { pinT = t; pinR = k; break; }
    }
    for (int x = 1; x < N; x++) cells_of[cls_cell[x]][ncells_of[cls_cell[x]]++] = x;
}

/* rotations a tile may take at a cell */
static inline int nrots(int x) { return cls_cell[x] == 0 ? 4 : 1; }
static inline int rotk(int x, int t, int j) { return cls_cell[x] == 0 ? j : rotOK[x][t]; }

/* ---------- encoding ---------- */
static inline void encode(uint8_t *b) { for (int x = 0; x < N; x++) b[x] = (uint8_t)(tile_[x] * 4 + rot_[x]); }
static inline void decode(const uint8_t *b) { for (int x = 0; x < N; x++) { tile_[x] = b[x] >> 2; rot_[x] = b[x] & 3; } }
static inline int hamming(const uint8_t *a, const uint8_t *b) { int h = 0; for (int x = 0; x < N; x++) h += a[x] != b[x]; return h; }

/* ---------- solutions (row-scan DFS, strict) ---------- */
static uint8_t *sols; static int nsol, solcap; static long long dfs_nodes;
static int used_[MAXT];
static void enum_rec(int x) {
    dfs_nodes++;
    if (x == N) {
        if (nsol == solcap) { solcap = solcap ? 2 * solcap : 64; sols = realloc(sols, (size_t)solcap * N); }
        encode(sols + (size_t)nsol * N); nsol++; return;
    }
    int r = x / n, c = x % n;
    int need_n = r > 0 ? col(x - n, 2) : 0, need_w = c > 0 ? col(x - 1, 1) : 0;
    for (int t = 0; t < T; t++) {
        if (used_[t] || cls_tile[t] != cls_cell[x]) continue;
        for (int j = 0; j < nrots(x); j++) {
            int k = rotk(x, t, j);
            if (k < 0) continue;
            if (tiles[t][k & 3] != need_n || tiles[t][(3 + k) & 3] != need_w) continue;
            used_[t] = 1; tile_[x] = t; rot_[x] = k;
            enum_rec(x + 1);
            used_[t] = 0;
        }
    }
}
static void enumerate(void) {
    memset(used_, 0, sizeof used_);
    used_[pinT] = 1; tile_[0] = pinT; rot_[0] = pinR; dfs_nodes = 1;
    enum_rec(1);
}
static int is_solution_idx(const uint8_t *b) {
    for (int s = 0; s < nsol; s++) if (!memcmp(b, sols + (size_t)s * N, N)) return s;
    return -1;
}
static int dist_nearest(const uint8_t *b) {
    int best = N + 1;
    for (int s = 0; s < nsol; s++) { int h = hamming(b, sols + (size_t)s * N); if (h < best) best = h; }
    return best;
}

/* ---------- hashed board set ---------- */
typedef struct { uint8_t *arena; uint8_t *mm; size_t cnt, cap; uint32_t *ht; size_t hmask; int capped; } Set;
static uint64_t hashb(const uint8_t *b) { uint64_t h = 1469598103934665603ULL; for (int i = 0; i < N; i++) { h ^= b[i]; h *= 1099511628211ULL; } return h; }
static void set_init(Set *s, size_t cap) {
    s->cap = cap; s->cnt = 0; s->capped = 0;
    s->arena = malloc(cap * N); s->mm = malloc(cap);
    size_t hs = 1; while (hs < 2 * cap) hs <<= 1;
    s->ht = malloc(hs * sizeof(uint32_t)); memset(s->ht, 0xff, hs * sizeof(uint32_t)); s->hmask = hs - 1;
}
static void set_clear(Set *s) { s->cnt = 0; s->capped = 0; memset(s->ht, 0xff, (s->hmask + 1) * sizeof(uint32_t)); }
/* returns 1 if newly inserted, 0 if present, -1 if cap reached */
static int set_add(Set *s, const uint8_t *b, int mm) {
    size_t h = hashb(b) & s->hmask;
    while (s->ht[h] != 0xffffffffu) {
        if (!memcmp(s->arena + (size_t)s->ht[h] * N, b, N)) return 0;
        h = (h + 1) & s->hmask;
    }
    if (s->cnt >= s->cap) { s->capped = 1; return -1; }
    memcpy(s->arena + s->cnt * N, b, N); s->mm[s->cnt] = (uint8_t)mm;
    s->ht[h] = (uint32_t)s->cnt; s->cnt++;
    return 1;
}

/* ---------- generic move: place tiles ts[] on cells xs[] with all legal rotations ---------- */
static int mv_m, mv_xs[4], mv_ts[4], mv_idx[MAXN], mv_base, mv_K;
static void (*mv_leaf)(int total);
static void mv_dfs(int i, int partial) {
    if (i == mv_m) { mv_leaf(mv_base + partial); return; }
    int x = mv_xs[i], t = mv_ts[i];
    for (int j = 0; j < nrots(x); j++) {
        int k = rotk(x, t, j);
        rot_[x] = k;
        int c = 0;
        for (int d = 0; d < 4; d++) {
            int y = nbr[x][d];
            if (y < 0) continue;
            if (mv_idx[y] < 0 || mv_idx[y] < i) c += col(x, d) != col(y, (d + 2) & 3);
        }
        if (mv_base + partial + c <= mv_K) mv_dfs(i + 1, partial + c);
    }
}
static void try_move(int m, const int *xs, const int *ts, int curM) {
    int ot[4], orr[4];
    mv_m = m;
    for (int i = 0; i < m; i++) { mv_xs[i] = xs[i]; mv_ts[i] = ts[i]; mv_idx[xs[i]] = i; ot[i] = tile_[xs[i]]; orr[i] = rot_[xs[i]]; }
    int local = 0;
    for (int i = 0; i < m; i++) {
        int x = xs[i];
        for (int d = 0; d < 4; d++) { int y = nbr[x][d]; if (y >= 0 && (mv_idx[y] < 0 || mv_idx[y] < i)) local += mis(x, d); }
    }
    mv_base = curM - local;
    if (mv_base <= mv_K) {
        for (int i = 0; i < m; i++) tile_[xs[i]] = ts[i];
        mv_dfs(0, 0);
    }
    for (int i = 0; i < m; i++) { tile_[xs[i]] = ot[i]; rot_[xs[i]] = orr[i]; mv_idx[xs[i]] = -1; }
}
static const int PERM4[24][4] = {
    {0,1,2,3},{0,1,3,2},{0,2,1,3},{0,2,3,1},{0,3,1,2},{0,3,2,1},{1,0,2,3},{1,0,3,2},{1,2,0,3},{1,2,3,0},{1,3,0,2},{1,3,2,0},
    {2,0,1,3},{2,0,3,1},{2,1,0,3},{2,1,3,0},{2,3,0,1},{2,3,1,0},{3,0,1,2},{3,0,2,1},{3,1,0,2},{3,1,2,0},{3,2,0,1},{3,2,1,0}};
static void neighbours(int ms, int curM) {
    int xs[4], ts[4];
    for (int i = 0; i < ncells_of[0]; i++) { xs[0] = cells_of[0][i]; ts[0] = tile_[xs[0]]; try_move(1, xs, ts, curM); }
    for (int c = 0; c < 3; c++) {
        int m = ncells_of[c];
        for (int a = 0; a < m; a++) for (int b = a + 1; b < m; b++) {
            xs[0] = cells_of[c][a]; xs[1] = cells_of[c][b];
            ts[0] = tile_[xs[1]]; ts[1] = tile_[xs[0]];
            try_move(2, xs, ts, curM);
            if (ms == 2) for (int e = b + 1; e < m; e++) {
                xs[2] = cells_of[c][e];
                int A = tile_[xs[0]], B = tile_[xs[1]], C = tile_[xs[2]];
                ts[0] = B; ts[1] = C; ts[2] = A; try_move(3, xs, ts, curM);
                ts[0] = C; ts[1] = A; ts[2] = B; try_move(3, xs, ts, curM);
            }
        }
    }
    if (ms == 3) {
        for (int r = 1; r + 1 < n - 1; r++) for (int c = 1; c + 1 < n - 1; c++) {
            int b[4] = {r * n + c, r * n + c + 1, (r + 1) * n + c, (r + 1) * n + c + 1};
            int bt[4] = {tile_[b[0]], tile_[b[1]], tile_[b[2]], tile_[b[3]]};
            for (int p = 0; p < 24; p++) {
                for (int i = 0; i < 4; i++) ts[i] = bt[PERM4[p][i]];
                try_move(4, b, ts, curM);
            }
        }
    }
}

/* ---------- basin BFS ---------- */
static Set S; static uint8_t tmpb[MAXN];
static int bfs_found[4096], bfs_nfound; static uint8_t *bfs_src; static int bfs_maxh;
static void bfs_leaf(int total) {
    encode(tmpb);
    int r = set_add(&S, tmpb, total);
    if (r == 1) {
        int h = hamming(tmpb, bfs_src); if (h > bfs_maxh) bfs_maxh = h;
        if (total == 0) { int s = is_solution_idx(tmpb); if (s >= 0 && bfs_nfound < 4096) bfs_found[bfs_nfound++] = s; }
    }
}
static void cmd_basin(int ms, int KMAX, size_t cap) {
    /* One BFS per component: a solution already met in an earlier explored set at the same k
     * shares that component and is not re-run (its max Hamming distance is computed over the
     * stored explored set, exact when uncapped, a lower bound when capped). */
    set_init(&S, cap);
    for (int x = 0; x < N; x++) mv_idx[x] = -1;
    mv_leaf = bfs_leaf;
    int *done = malloc(sizeof(int) * nsol);
    for (int k = 0; k <= KMAX; k++) {
        mv_K = k;
        for (int s = 0; s < nsol; s++) done[s] = 0;
        for (int s = 0; s < nsol; s++) {
            if (done[s]) continue;
            double t0 = now();
            set_clear(&S); bfs_nfound = 0; bfs_maxh = 0; bfs_src = sols + (size_t)s * N;
            set_add(&S, bfs_src, 0); bfs_found[bfs_nfound++] = s;
            size_t head = 0; long hist[8] = {0};
            while (head < S.cnt && !S.capped) {
                decode(S.arena + head * N);
                int M = S.mm[head]; head++;
                neighbours(ms, M);
            }
            for (size_t i = 0; i < S.cnt; i++) hist[S.mm[i] < 7 ? S.mm[i] : 7]++;
            printf("{\"cmd\":\"basin\",\"ms\":%d,\"k\":%d,\"sol\":%d,\"size\":%zu,\"capped\":%d,\"expanded\":%zu,"
                   "\"N\":%d,\"by_mm\":[%ld,%ld,%ld,%ld,%ld],\"sols_in\":[",
                   ms, k, s, S.cnt, S.capped, head, N, hist[0], hist[1], hist[2], hist[3], hist[4]);
            for (int i = 0; i < bfs_nfound; i++) printf(i ? ",%d" : "%d", bfs_found[i]);
            printf("],\"maxh\":[");
            for (int i = 0; i < bfs_nfound; i++) {
                const uint8_t *src = sols + (size_t)bfs_found[i] * N; int mh = 0;
                for (size_t j = 0; j < S.cnt; j++) { int h = hamming(S.arena + j * N, src); if (h > mh) mh = h; }
                printf(i ? ",%d" : "%d", mh); done[bfs_found[i]] = 1;
            }
            printf("],\"sec\":%.2f}\n", now() - t0);
            fflush(stdout);
        }
    }
    free(done);
}

/* ---------- beam upper bound on the M1 barrier ---------- */
typedef struct { uint8_t b[MAXN]; int pmax, cur; } BS;
static int bs_cmp(const void *a, const void *b) {
    const BS *p = a, *q = b;
    if (p->pmax != q->pmax) return p->pmax - q->pmax;
    return p->cur - q->cur;
}
static int beam_pair(const uint8_t *A, const uint8_t *B, int W) {
    BS *beam = malloc(sizeof(BS) * W), *cand = malloc(sizeof(BS) * (size_t)W * N * 8);
    int nb = 1; memcpy(beam[0].b, A, N); beam[0].pmax = 0; beam[0].cur = 0;
    int Bt[MAXN], Br[MAXN];
    for (int x = 0; x < N; x++) { Bt[x] = B[x] >> 2; Br[x] = B[x] & 3; }
    Set seen; set_init(&seen, (size_t)W * N * 8 + 16);
    for (int step = 0; step < N; step++) {
        int nc = 0;
        for (int i = 0; i < nb; i++) {
            if (!memcmp(beam[i].b, B, N)) { int r = beam[i].pmax; free(beam); free(cand); free(seen.arena); free(seen.mm); free(seen.ht); return r; }
            decode(beam[i].b);
            int where[MAXT]; for (int x = 0; x < N; x++) where[tile_[x]] = x;
            for (int x = 1; x < N; x++) {
                if (tile_[x] == Bt[x] && rot_[x] == Br[x]) continue;
                int ot = tile_[x], orr = rot_[x];
                if (tile_[x] == Bt[x]) {
                    rot_[x] = Br[x];
                    cand[nc].cur = total_mis(); encode(cand[nc].b);
                    cand[nc].pmax = beam[i].pmax > cand[nc].cur ? beam[i].pmax : cand[nc].cur; nc++;
                    rot_[x] = orr;
                } else {
                    int y = where[Bt[x]];
                    int yr = rot_[y];
                    tile_[x] = Bt[x]; rot_[x] = Br[x]; tile_[y] = ot;
                    for (int j = 0; j < nrots(y); j++) {
                        rot_[y] = rotk(y, ot, j);
                        cand[nc].cur = total_mis(); encode(cand[nc].b);
                        cand[nc].pmax = beam[i].pmax > cand[nc].cur ? beam[i].pmax : cand[nc].cur; nc++;
                    }
                    tile_[x] = ot; rot_[x] = orr; tile_[y] = Bt[x]; rot_[y] = yr;
                }
            }
        }
        qsort(cand, nc, sizeof(BS), bs_cmp);
        set_clear(&seen); nb = 0;
        for (int i = 0; i < nc && nb < W; i++) if (set_add(&seen, cand[i].b, 0) == 1) beam[nb++] = cand[i];
    }
    int r = -1;
    for (int i = 0; i < nb; i++) if (!memcmp(beam[i].b, B, N)) r = beam[i].pmax;
    free(beam); free(cand); free(seen.arena); free(seen.mm); free(seen.ht);
    return r;
}
static void cmd_beam(int W, int maxpairs) {
    int done = 0;
    for (int i = 0; i < nsol && done < maxpairs; i++) for (int j = i + 1; j < nsol && done < maxpairs; j++) {
        const uint8_t *A = sols + (size_t)i * N, *B = sols + (size_t)j * N;
        int u = beam_pair(A, B, W), v = beam_pair(B, A, W);
        int best = (u < 0) ? v : (v < 0 ? u : (u < v ? u : v));
        printf("{\"cmd\":\"beam\",\"W\":%d,\"i\":%d,\"j\":%d,\"ham\":%d,\"ub\":%d}\n", W, i, j, hamming(A, B), best);
        fflush(stdout); done++;
    }
}

/* ---------- annealer ---------- */
static void random_board(void) {
    tile_[0] = pinT; rot_[0] = pinR;
    for (int c = 0; c < 3; c++) {
        int pool[MAXT], np = 0;
        for (int t = 0; t < T; t++) if (cls_tile[t] == c && t != pinT) pool[np++] = t;
        for (int i = np - 1; i > 0; i--) { int j = rnd(i + 1); int z = pool[i]; pool[i] = pool[j]; pool[j] = z; }
        for (int i = 0; i < ncells_of[c]; i++) { int x = cells_of[c][i]; tile_[x] = pool[i]; rot_[x] = c == 0 ? rnd(4) : rotOK[x][pool[i]]; }
    }
}
static int local_mis2(int x, int y) {   /* mismatches on edges touching x or y, each once */
    int m = 0;
    for (int d = 0; d < 4; d++) m += mis(x, d);
    if (y != x) for (int d = 0; d < 4; d++) { int z = nbr[y][d]; if (z >= 0 && z != x) m += mis(y, d); }
    return m;
}
static int trapK = 0; static size_t trapCap = 1000000;
static void cmd_anneal(int runs, long steps, int snaps, double T0, double T1) {
    if (trapK > 0) { set_init(&S, trapCap); for (int x = 0; x < N; x++) mv_idx[x] = -1; }
    uint8_t b[MAXN];
    int nfree = ncells_of[0] + ncells_of[1] + ncells_of[2];
    int allc[MAXN]; int q = 0; for (int c = 0; c < 3; c++) for (int i = 0; i < ncells_of[c]; i++) allc[q++] = cells_of[c][i];
    /* random-board baseline */
    for (int i = 0; i < 1000; i++) {
        random_board(); encode(b);
        printf("{\"cmd\":\"random\",\"mm\":%d,\"dist\":%d}\n", total_mis(), dist_nearest(b));
    }
    for (int run = 0; run < runs; run++) {
        random_board();
        int M = total_mis(), best = M; long hit = -1;
        double lr = log(T1 / T0);
        long every = steps / snaps;
        printf("{\"cmd\":\"traj\",\"run\":%d,\"pts\":[", run);
        for (long st = 0; st < steps; st++) {
            if (every && st % every == 0) { encode(b); printf("%s[%ld,%d,%d]", st ? "," : "", st, M, dist_nearest(b)); }
            double temp = T0 * exp(lr * (double)st / (double)steps);
            int x = allc[rnd(nfree)], y;
            int c = cls_cell[x];
            int ox = tile_[x], oxr = rot_[x], oy, oyr;
            if (c == 0 && urand() < 0.3) {
                y = x; oy = ox; oyr = oxr;
                int before = local_mis2(x, x);
                rot_[x] = (oxr + 1 + rnd(3)) & 3;
                int delta = local_mis2(x, x) - before;
                if (delta <= 0 || urand() < exp(-delta / temp)) M += delta; else rot_[x] = oxr;
            } else {
                if (ncells_of[c] < 2) continue;
                do y = cells_of[c][rnd(ncells_of[c])]; while (y == x);
                oy = tile_[y]; oyr = rot_[y];
                int before = local_mis2(x, y);
                tile_[x] = oy; tile_[y] = ox;
                if (c == 0) { rot_[x] = rnd(4); rot_[y] = rnd(4); } else { rot_[x] = rotOK[x][oy]; rot_[y] = rotOK[y][ox]; }
                int delta = local_mis2(x, y) - before;
                if (delta <= 0 || urand() < exp(-delta / temp)) M += delta;
                else { tile_[x] = ox; rot_[x] = oxr; tile_[y] = oy; rot_[y] = oyr; }
            }
            if (M < best) best = M;
            if (M == 0) { hit = st; break; }
        }
        encode(b);
        int d = dist_nearest(b);
        printf("],\"final_mm\":%d,\"final_dist\":%d,\"best_mm\":%d,\"solved_at\":%ld,\"check\":%d", M, d, best, hit, total_mis());
        if (trapK > 0 && M > 0 && M <= trapK) {
            /* trap check: is this near-solution in the same M1 component as a solution, in the
             * level set {mismatch <= trapK}?  BFS capped at trapCap boards. */
            uint8_t src[MAXN]; memcpy(src, b, N);
            int sv_t[MAXN], sv_r[MAXN]; memcpy(sv_t, tile_, sizeof sv_t); memcpy(sv_r, rot_, sizeof sv_r);
            mv_leaf = bfs_leaf; mv_K = trapK;
            set_clear(&S); bfs_nfound = 0; bfs_maxh = 0; bfs_src = src;
            set_add(&S, src, M);
            size_t head = 0;
            while (head < S.cnt && !S.capped && bfs_nfound == 0) {
                decode(S.arena + head * N); int MM = S.mm[head]; head++;
                neighbours(1, MM);
            }
            printf(",\"trapK\":%d,\"trap_reached_sol\":%d,\"trap_size\":%zu,\"trap_capped\":%d", trapK, bfs_nfound > 0, S.cnt, S.capped);
            memcpy(tile_, sv_t, sizeof sv_t); memcpy(rot_, sv_r, sizeof sv_r);
        }
        printf("}\n");
        fflush(stdout);
    }
}

/* ---------- counting boards with exactly j mismatches ---------- */
static long long cnt[8], cnodes, cbudget; static int cK, cabort;
static void count_rec(int x, int mm) {
    if (cabort) return;
    if (++cnodes > cbudget) { cabort = 1; return; }
    if (x == N) { cnt[mm]++; return; }
    int r = x / n, c = x % n;
    int need_n = r > 0 ? col(x - n, 2) : 0, need_w = c > 0 ? col(x - 1, 1) : 0;
    for (int t = 0; t < T; t++) {
        if (used_[t] || cls_tile[t] != cls_cell[x]) continue;
        for (int j = 0; j < nrots(x); j++) {
            int k = rotk(x, t, j);
            if (k < 0) continue;
            int add = (r > 0 && tiles[t][k & 3] != need_n) + (c > 0 && tiles[t][(3 + k) & 3] != need_w);
            if (mm + add > cK) continue;
            used_[t] = 1; tile_[x] = t; rot_[x] = k;
            count_rec(x + 1, mm + add);
            used_[t] = 0;
        }
    }
}
static void cmd_count(int K, long long budget) {
    memset(used_, 0, sizeof used_); memset(cnt, 0, sizeof cnt);
    cK = K; cbudget = budget; cnodes = 0; cabort = 0;
    used_[pinT] = 1; tile_[0] = pinT; rot_[0] = pinR;
    double t0 = now();
    count_rec(1, 0);
    printf("{\"cmd\":\"count\",\"K\":%d,\"aborted\":%d,\"nodes\":%lld,\"counts\":[", K, cabort, cnodes);
    for (int j = 0; j <= K; j++) printf(j ? ",%lld" : "%lld", cnt[j]);
    printf("],\"sec\":%.1f}\n", now() - t0);
}
static void cmd_knuth(int K, long long probes) {
    double sum[8] = {0}, sum2[8] = {0};
    double t0 = now();
    for (long long p = 0; p < probes; p++) {
        memset(used_, 0, sizeof used_);
        used_[pinT] = 1; tile_[0] = pinT; rot_[0] = pinR;
        double w = 1; int mm = 0, x;
        for (x = 1; x < N; x++) {
            int r = x / n, c = x % n;
            int need_n = r > 0 ? col(x - n, 2) : 0, need_w = c > 0 ? col(x - 1, 1) : 0;
            int ct[MAXT * 4], ck[MAXT * 4], ca[MAXT * 4], nc = 0;
            for (int t = 0; t < T; t++) {
                if (used_[t] || cls_tile[t] != cls_cell[x]) continue;
                for (int j = 0; j < nrots(x); j++) {
                    int k = rotk(x, t, j);
                    if (k < 0) continue;
                    int add = (r > 0 && tiles[t][k & 3] != need_n) + (c > 0 && tiles[t][(3 + k) & 3] != need_w);
                    if (mm + add > K) continue;
                    ct[nc] = t; ck[nc] = k; ca[nc] = add; nc++;
                }
            }
            if (!nc) break;
            int pick = rnd(nc);
            w *= nc; used_[ct[pick]] = 1; tile_[x] = ct[pick]; rot_[x] = ck[pick]; mm += ca[pick];
        }
        for (int j = 0; j <= K; j++) { double v = (x == N && mm == j) ? w : 0; sum[j] += v; sum2[j] += v * v; }
    }
    printf("{\"cmd\":\"knuth\",\"K\":%d,\"probes\":%lld,\"est\":[", K, probes);
    for (int j = 0; j <= K; j++) printf(j ? ",%.4g" : "%.4g", sum[j] / probes);
    printf("],\"se\":[");
    for (int j = 0; j <= K; j++) { double m = sum[j] / probes, v = sum2[j] / probes - m * m; printf(j ? ",%.4g" : "%.4g", sqrt(v > 0 ? v : 0) / sqrt((double)probes)); }
    printf("],\"sec\":%.1f}\n", now() - t0);
}

/* Hybrid estimator: a uniform random path (Knuth) down to depth D, then an exact tolerant count
 * of the whole subtree below it; unbiased, much lower variance than plain Knuth. */
static void cmd_hybrid(int K, int D, long long probes, long long budget) {
    double sum[8] = {0}, sum2[8] = {0}; long long aborted = 0;
    double t0 = now();
    for (long long p = 0; p < probes; p++) {
        memset(used_, 0, sizeof used_);
        used_[pinT] = 1; tile_[0] = pinT; rot_[0] = pinR;
        double w = 1; int mm = 0, x, dead = 0;
        for (x = 1; x < D; x++) {
            int r = x / n, c = x % n;
            int need_n = r > 0 ? col(x - n, 2) : 0, need_w = c > 0 ? col(x - 1, 1) : 0;
            int ct[MAXT * 4], ck[MAXT * 4], ca[MAXT * 4], nc = 0;
            for (int t = 0; t < T; t++) {
                if (used_[t] || cls_tile[t] != cls_cell[x]) continue;
                for (int j = 0; j < nrots(x); j++) {
                    int k = rotk(x, t, j);
                    if (k < 0) continue;
                    int add = (r > 0 && tiles[t][k & 3] != need_n) + (c > 0 && tiles[t][(3 + k) & 3] != need_w);
                    if (mm + add > K) continue;
                    ct[nc] = t; ck[nc] = k; ca[nc] = add; nc++;
                }
            }
            if (!nc) { dead = 1; break; }
            int pick = rnd(nc);
            w *= nc; used_[ct[pick]] = 1; tile_[x] = ct[pick]; rot_[x] = ck[pick]; mm += ca[pick];
        }
        double v[8] = {0};
        if (!dead) {
            memset(cnt, 0, sizeof cnt); cK = K; cbudget = budget; cnodes = 0; cabort = 0;
            count_rec(D, mm);
            if (cabort) aborted++;
            for (int j = 0; j <= K; j++) v[j] = w * (double)cnt[j];
        }
        for (int j = 0; j <= K; j++) { sum[j] += v[j]; sum2[j] += v[j] * v[j]; }
    }
    printf("{\"cmd\":\"hybrid\",\"K\":%d,\"D\":%d,\"probes\":%lld,\"aborted_probes\":%lld,\"est\":[", K, D, probes, aborted);
    for (int j = 0; j <= K; j++) printf(j ? ",%.4g" : "%.4g", sum[j] / probes);
    printf("],\"se\":[");
    for (int j = 0; j <= K; j++) { double m = sum[j] / probes, vv = sum2[j] / probes - m * m; printf(j ? ",%.4g" : "%.4g", sqrt(vv > 0 ? vv : 0) / sqrt((double)probes)); }
    printf("],\"sec\":%.1f}\n", now() - t0);
}

int main(int argc, char **argv) {
    if (argc < 3) { fprintf(stderr, "usage: see header\n"); return 1; }
    load(argv[1]);
    const char *cmd = argv[2];
    if (!strcmp(cmd, "count")) { cmd_count(atoi(argv[3]), atoll(argv[4])); return 0; }
    if (!strcmp(cmd, "hybrid")) { rs ^= (uint64_t)atoll(argv[6]) * 0x9E3779B97F4A7C15ULL; cmd_hybrid(atoi(argv[3]), atoi(argv[4]), atoll(argv[5]), atoll(argv[7])); return 0; }
    if (!strcmp(cmd, "knuth")) { rs ^= (uint64_t)atoll(argv[5]) * 0x9E3779B97F4A7C15ULL; cmd_knuth(atoi(argv[3]), atoll(argv[4])); return 0; }
    double t0 = now();
    enumerate();
    printf("{\"cmd\":\"enum\",\"n\":%d,\"T\":%d,\"E\":%d,\"sols\":%d,\"dfs_nodes\":%lld,\"sec\":%.1f}\n", n, T, E, nsol, dfs_nodes, now() - t0);
    for (int s = 0; s < nsol; s++) { decode(sols + (size_t)s * N); if (total_mis()) { fprintf(stderr, "bad solution\n"); return 1; } }
    fflush(stdout);
    if (!strcmp(cmd, "enum")) return 0;
    if (!strcmp(cmd, "basin")) cmd_basin(atoi(argv[3]), atoi(argv[4]), (size_t)atoll(argv[5]));
    else if (!strcmp(cmd, "beam")) cmd_beam(atoi(argv[3]), atoi(argv[4]));
    else if (!strcmp(cmd, "anneal")) {
        rs ^= (uint64_t)atoll(argv[5]) * 0x9E3779B97F4A7C15ULL;
        if (argc > 9) trapK = atoi(argv[9]);
        if (argc > 10) trapCap = (size_t)atoll(argv[10]);
        cmd_anneal(atoi(argv[3]), atol(argv[4]), atoi(argv[6]), argc > 7 ? atof(argv[7]) : 2.0, argc > 8 ? atof(argv[8]) : 0.05);
    }
    return 0;
}
