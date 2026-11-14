/* Re-check's solver for the gap-4 review. Written from scratch (not derived from the memo's toy_orders.c).
 * Framed edge-matching boards: GENERATED toys, D5 board files, or the official 16x16 piece file with its clues.
 * Compile-time: -DMAXC=<cells> -DW=<ceil(4*cells/64)>.
 *
 * usage: dyn <board> <PROP> <RULE> exact <cap> <cpu_seconds>
 *        dyn <board> <PROP> <RULE> smc <M> <reps> <rseed> <cpu_seconds>
 *   board: gen:n:cf:ci:seed[:pins]   pins = comma list of cells fixed to their planted tile ("c" = cell 0)
 *          file:<path>               D5 format (n, then n*n lines top right bottom left, 0 = wall)
 *          e2:<path>                 official csv (top,right,bottom,left as binary words, x,y,rot), clues fixed
 *          e2r:<path>                the same board turned by a half turn (so ROW scans the official board from its last cell)
 *   PROP : PLAIN (no lookahead), FC (fail when any empty cell has no candidate),
 *          AC (arc consistency on joint colours to a fixpoint, plus: a cell with one value removes that tile elsewhere)
 *   RULE : ROW (row scan from cell 0), ROWR (row scan from the last cell),
 *          DOM (smallest domain among ALL empty cells, ties to the cell with most decided neighbours or walls),
 *          DOMF (same, restricted to empty cells that touch a decided cell),
 *          WDEG (smallest domain / weighted degree, weights learned from wipeouts; AC and exact mode only)
 *          Yk   (the memo's staircase rule: smallest domain among empty cells whose upper and left neighbours are decided
 *                or walls, within the first k unfinished rows; Y1 = ROW)
 *          Bk   (smallest domain among ALL empty cells of the first k unfinished rows, same tie rule as DOM)
 *          Lk   (one-step lookahead: take the 6 best cells of Bk, count for each the children that survive
 *                propagation, branch on the cell with fewest surviving children; the extra propagations are not counted)
 * exact: full enumeration of all solutions. tried = branches tried (same measure as the memo),
 *        ctried = branches tried at cells that had two or more candidates, alive = children that survived propagation.
 * smc  : population estimate of the same counts. Level k holds up to M states drawn uniformly without replacement
 *        from the children of the level k-1 sample; n_k is estimated by n_{k-1} * (alive children / states).
 *        Levels with at most M nodes are enumerated exactly. The estimate of every level count is unbiased.
 *        With PROFILE set in the environment, stderr gets one line per level: rep, level, decided cells, log2 n_k.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>
#include <time.h>
#ifndef MAXC
#define MAXC 64
#endif
#ifndef W
#define W 4
#endif
typedef struct { uint64_t w[W]; } bs;
typedef struct { bs dom[MAXC]; uint32_t cm[MAXC][4]; uint8_t dec[MAXC], fix[MAXC]; int ndec; } St;
enum { PLAIN, FC, AC };
enum { ROW, ROWR, DOM, DOMF, WDEG, YK, BK, LA };
static int KOPEN;
static int n, NC, NV, NCOL, PROP, RULE;
static int col[4 * MAXC][4];
static bs bycol[4][40];
static int nb[MAXC][4], walls[MAXC];
static int planted[MAXC];              /* planted value of each cell (generated boards), -1 if unknown */
static int pins[MAXC], pinval[MAXC], npins;
static uint32_t Q[1 << 17]; static int qn; static int dead;
static double wt[MAXC][4]; static int learn;
static uint64_t rs;
static uint64_t rnd(void) { uint64_t z = (rs += 0x9E3779B97F4A7C15ULL); z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9ULL; z = (z ^ (z >> 27)) * 0x94D049BB133111EBULL; return z ^ (z >> 31); }
static inline int bs_empty(const bs *a) { uint64_t x = 0; for (int i = 0; i < W; i++) x |= a->w[i]; return x == 0; }
static inline int bs_cnt(const bs *a) { int c = 0; for (int i = 0; i < W; i++) c += __builtin_popcountll(a->w[i]); return c; }
static inline int bs_has(const bs *a, const bs *b) { for (int i = 0; i < W; i++) if (a->w[i] & b->w[i]) return 1; return 0; }
static inline int bs_first(const bs *a) { for (int i = 0; i < W; i++) if (a->w[i]) return i * 64 + __builtin_ctzll(a->w[i]); return -1; }

static void remove_tile(St *s, int t, int except);
static void on_removed(St *s, int x, const bs *rem) {
  for (int i = 0; i < W; i++) { uint64_t b = rem->w[i];
    while (b) { int v = i * 64 + __builtin_ctzll(b); b &= b - 1;
      for (int d = 0; d < 4; d++) { int k = col[v][d];
        if (((s->cm[x][d] >> k) & 1) && !bs_has(&s->dom[x], &bycol[d][k])) { s->cm[x][d] &= ~(1u << k); if (nb[x][d] >= 0) Q[qn++] = ((uint32_t)x << 16) | (d << 8) | k; } } } }
  if (!s->fix[x] && bs_cnt(&s->dom[x]) == 1) { s->fix[x] = 1; remove_tile(s, bs_first(&s->dom[x]) >> 2, x); }
}
static void remove_tile(St *s, int t, int except) {
  int wi = (4 * t) >> 6; uint64_t m = 0xFULL << ((4 * t) & 63);
  for (int z = 0; z < NC; z++) { if (z == except) continue;
    uint64_t r = s->dom[z].w[wi] & m; if (!r) continue;
    s->dom[z].w[wi] &= ~m; if (bs_empty(&s->dom[z])) { dead = 1; return; }
    bs rem; memset(&rem, 0, sizeof rem); rem.w[wi] = r; on_removed(s, z, &rem); if (dead) return; }
}
static int run_queue(St *s) {
  int qh = 0;
  while (qh < qn && !dead) { uint32_t e = Q[qh++]; int y = e >> 16, d = (e >> 8) & 3, k = e & 255; int x = nb[y][d], od = (d + 2) & 3;
    bs rem; uint64_t any = 0; for (int i = 0; i < W; i++) { rem.w[i] = s->dom[x].w[i] & bycol[od][k].w[i]; any |= rem.w[i]; s->dom[x].w[i] &= ~rem.w[i]; }
    if (!any) continue;
    if (bs_empty(&s->dom[x])) { dead = 1; if (learn) { wt[x][od] += 1; wt[y][d] += 1; } break; }
    on_removed(s, x, &rem); }
  return !dead;
}
static int assign(St *s, int x, int v) {
  if (PROP == AC) {
    qn = 0; dead = 0; bs rem = s->dom[x]; rem.w[v >> 6] &= ~(1ULL << (v & 63));
    memset(&s->dom[x], 0, sizeof(bs)); s->dom[x].w[v >> 6] = 1ULL << (v & 63); s->dec[x] = 1; s->ndec++;
    on_removed(s, x, &rem); if (dead) return 0; return run_queue(s);
  }
  memset(&s->dom[x], 0, sizeof(bs)); s->dom[x].w[v >> 6] = 1ULL << (v & 63); s->dec[x] = 1; s->ndec++;
  int t = v >> 2, wi = (4 * t) >> 6; uint64_t m = 0xFULL << ((4 * t) & 63);
  for (int z = 0; z < NC; z++) { if (s->dec[z]) continue; if (s->dom[z].w[wi] & m) { s->dom[z].w[wi] &= ~m; if (PROP == FC && bs_empty(&s->dom[z])) return 0; } }
  for (int d = 0; d < 4; d++) { int y = nb[x][d]; if (y < 0 || s->dec[y]) continue; int k = col[v][d], od = (d + 2) & 3; uint64_t any = 0;
    for (int i = 0; i < W; i++) { s->dom[y].w[i] &= bycol[od][k].w[i]; any |= s->dom[y].w[i]; }
    if (PROP == FC && !any) return 0; }
  return 1;
}
static int root_init(St *s) {
  memset(s, 0, sizeof *s);
  for (int x = 0; x < NC; x++) for (int v = 0; v < NV; v++) { int ok = 1; for (int d = 0; d < 4; d++) if ((nb[x][d] < 0) != (col[v][d] == 0)) ok = 0; if (ok) s->dom[x].w[v >> 6] |= 1ULL << (v & 63); }
  if (PROP == AC) {
    qn = 0; dead = 0;
    for (int x = 0; x < NC; x++) { if (bs_empty(&s->dom[x])) return 0;
      for (int d = 0; d < 4; d++) { s->cm[x][d] = 0; for (int k = 0; k <= NCOL; k++) if (bs_has(&s->dom[x], &bycol[d][k])) s->cm[x][d] |= 1u << k; } }
    for (int x = 0; x < NC; x++) for (int d = 0; d < 4; d++) if (nb[x][d] >= 0) for (int k = 0; k <= NCOL; k++) if (!((s->cm[x][d] >> k) & 1)) Q[qn++] = ((uint32_t)x << 16) | (d << 8) | k;
    for (int x = 0; x < NC && !dead; x++) if (!s->fix[x] && bs_cnt(&s->dom[x]) == 1) { s->fix[x] = 1; remove_tile(s, bs_first(&s->dom[x]) >> 2, x); }
    if (dead || !run_queue(s)) return 0;
  }
  for (int i = 0; i < npins; i++) { int x = pins[i], v = pinval[i]; if (!((s->dom[x].w[v >> 6] >> (v & 63)) & 1)) return 0; if (!assign(s, x, v)) return 0; }
  return 1;
}
static int assign(St *s, int x, int v);
static St la_tmp;
#define LAC 6
static int select_cell(const St *s) {
  if (RULE == ROW) { for (int x = 0; x < NC; x++) if (!s->dec[x]) return x; return -1; }
  if (RULE == ROWR) { for (int x = NC - 1; x >= 0; x--) if (!s->dec[x]) return x; return -1; }
  int bx = -1; double bk = 1e300; int pass_frontier = (RULE == DOMF);
  int r0 = 0, r1 = n - 1;
  int cand[LAC]; double ckey[LAC]; int ncand = 0;
  if (RULE == YK || RULE == BK || RULE == LA) { for (int x = 0; x < NC; x++) if (!s->dec[x]) { r0 = x / n; break; } r1 = r0 + KOPEN - 1; if (r1 > n - 1) r1 = n - 1; }
  for (;;) {
    for (int x = r0 * n; x < (r1 + 1) * n; x++) { if (s->dec[x]) continue;
      if (RULE == YK) { int u = nb[x][0], l = nb[x][3]; if ((u >= 0 && !s->dec[u]) || (l >= 0 && !s->dec[l])) continue; }
      int known = 0; double wd = 1;
      for (int d = 0; d < 4; d++) { int y = nb[x][d]; if (y < 0) continue; if (s->dec[y] || (PROP == AC && s->fix[y])) known++; else wd += wt[x][d]; }
      if (pass_frontier && !known) continue;
      int c = bs_cnt(&s->dom[x]); double key;
      if (RULE == WDEG) key = (c <= 1) ? -1.0 + c : (double)c / wd;
      else key = c * 8.0 + (4 - known - walls[x]);
      if (key < bk) { bk = key; bx = x; }
      if (RULE == LA) { int j = -1; if (ncand < LAC) j = ncand++; else if (key < ckey[LAC - 1]) j = LAC - 1;
        if (j >= 0) { cand[j] = x; ckey[j] = key; for (; j > 0 && ckey[j - 1] > ckey[j]; j--) { int tc = cand[j]; cand[j] = cand[j - 1]; cand[j - 1] = tc; double tk = ckey[j]; ckey[j] = ckey[j - 1]; ckey[j - 1] = tk; } } } }
    if (bx >= 0 || !pass_frontier) break; pass_frontier = 0; }
  if (RULE == LA && ncand > 1) { double best = 1e300; int sv = learn; learn = 0;
    for (int j = 0; j < ncand; j++) { int x = cand[j], al = 0; bs D = s->dom[x];
      for (int i = 0; i < W; i++) { uint64_t b = D.w[i]; while (b) { int v = i * 64 + __builtin_ctzll(b); b &= b - 1; la_tmp = *s; if (assign(&la_tmp, x, v)) al++; } }
      double k2 = al * 4096.0 + ckey[j]; if (k2 < best) { best = k2; bx = x; } }
    learn = sv; }
  return bx;
}
/* ---------- boards ---------- */
static void finish_board(void) {
  NV = 4 * NC; memset(bycol, 0, sizeof bycol);
  for (int v = 0; v < NV; v++) for (int d = 0; d < 4; d++) bycol[d][col[v][d]].w[v >> 6] |= 1ULL << (v & 63);
  for (int x = 0; x < NC; x++) { int r = x / n, c = x % n; nb[x][0] = r > 0 ? x - n : -1; nb[x][1] = c < n - 1 ? x + 1 : -1; nb[x][2] = r < n - 1 ? x + n : -1; nb[x][3] = c > 0 ? x - 1 : -1;
    walls[x] = 0; for (int d = 0; d < 4; d++) if (nb[x][d] < 0) walls[x]++; }
}
/* tile t with edges e[0..3] = top,right,bottom,left ; value 4t+r is the tile turned r quarter turns clockwise */
static void set_tile(int t, const int *e) { for (int r = 0; r < 4; r++) for (int d = 0; d < 4; d++) col[4 * t + r][d] = e[(d - r + 4) & 3]; }
static void gen(int cf, int ci, uint64_t seed) {
  static int t[MAXC][4]; rs = seed * 0xD1B54A32D192ED03ULL + 77;
  for (;;) {
    memset(t, 0, sizeof t);
    for (int r = 0; r < n; r++) for (int c = 0; c < n; c++) { int x = r * n + c;
      if (c + 1 < n) { int fr = (r == 0 || r == n - 1); int k = fr ? 1 + (int)(rnd() % cf) : cf + 1 + (int)(rnd() % ci); t[x][1] = k; t[x + 1][3] = k; }
      if (r + 1 < n) { int fr = (c == 0 || c == n - 1); int k = fr ? 1 + (int)(rnd() % cf) : cf + 1 + (int)(rnd() % ci); t[x][2] = k; t[x + n][0] = k; } }
    int ok = 1;
    for (int a = 0; a < NC && ok; a++) {
      for (int ro = 1; ro < 4 && ok; ro++) { int same = 1; for (int d = 0; d < 4; d++) if (t[a][d] != t[a][(d + ro) & 3]) same = 0; if (same) ok = 0; }
      for (int b = a + 1; b < NC && ok; b++) for (int ro = 0; ro < 4 && ok; ro++) { int same = 1; for (int d = 0; d < 4; d++) if (t[a][d] != t[b][(d + ro) & 3]) same = 0; if (same) ok = 0; } }
    if (ok) break;
  }
  int perm[MAXC]; for (int i = 0; i < NC; i++) perm[i] = i;        /* perm[id] = cell whose tile gets this id */
  for (int i = NC - 1; i > 0; i--) { int j = (int)(rnd() % (i + 1)); int z = perm[i]; perm[i] = perm[j]; perm[j] = z; }
  for (int id = 0; id < NC; id++) { int cell = perm[id]; int ro = (int)(rnd() & 3); int e[4];
    for (int d = 0; d < 4; d++) e[d] = t[cell][(d + ro) & 3];     /* stored turned back by ro: value 4id+ro restores it */
    set_tile(id, e); planted[cell] = 4 * id + ro; }
  NCOL = cf + ci;
}
static int load_file(const char *p) {
  FILE *f = fopen(p, "r"); if (!f) return 0; char line[256]; int got = 0, id = 0, mx = 0;
  while (fgets(line, sizeof line, f)) { if (line[0] == '#') continue;
    if (!got) { n = atoi(line); NC = n * n; got = 1; continue; }
    int e[4]; if (sscanf(line, "%d %d %d %d", &e[0], &e[1], &e[2], &e[3]) != 4) continue;
    for (int d = 0; d < 4; d++) if (e[d] > mx) mx = e[d]; set_tile(id++, e); }
  fclose(f); NCOL = mx; for (int i = 0; i < NC; i++) planted[i] = -1; return id == NC;
}
static int load_e2(const char *p, int turn) {
  FILE *f = fopen(p, "r"); if (!f) return 0; char line[512]; int got = 0, id = 0, mx = 0;
  while (fgets(line, sizeof line, f)) {
    if (!got) { n = atoi(line); NC = n * n; got = 1; continue; }
    char a[4][32]; int x, y, r; if (sscanf(line, "%31[01],%31[01],%31[01],%31[01],%d,%d,%d", a[0], a[1], a[2], a[3], &x, &y, &r) != 7) continue;
    int e[4]; for (int d = 0; d < 4; d++) { long v = strtol(a[d], 0, 2); e[d] = (v == 65535) ? 0 : (int)v + 1; if (e[d] > mx) mx = e[d]; }
    set_tile(id, e); if (x || y || r) { pins[npins] = turn ? NC - 1 - (y * n + x) : y * n + x; pinval[npins] = 4 * id + ((r + (turn ? 2 : 0)) & 3); npins++; } id++; }
  fclose(f); NCOL = mx; for (int i = 0; i < NC; i++) planted[i] = -1; return id == NC;
}
/* ---------- exact ---------- */
static uint64_t tried, ctried, alive, sols, cap; static int censored; static double tlimit; static clock_t t0;
static St *stk; static uint64_t *prof;
static void search(int depth) {
  St *s = &stk[depth]; if (censored) return;
  if (s->ndec == NC) { sols++; return; }
  int x = select_cell(s); bs D = s->dom[x]; int c = bs_cnt(&D);
  for (int i = 0; i < W; i++) { uint64_t b = D.w[i];
    while (b) { int v = i * 64 + __builtin_ctzll(b); b &= b - 1;
      if (++tried > cap) { censored = 1; return; }
      if ((tried & 0xFFFF) == 0 && (double)(clock() - t0) / CLOCKS_PER_SEC > tlimit) { censored = 2; return; }
      if (c >= 2) ctried++; prof[depth]++;
      stk[depth + 1] = *s; if (assign(&stk[depth + 1], x, v)) { alive++; search(depth + 1); if (censored) return; } } }
}
int main(int argc, char **argv) {
  if (argc < 5) { fprintf(stderr, "see header\n"); return 1; }
  char *b = argv[1]; char desc[256]; snprintf(desc, sizeof desc, "%s", b);
  if (!strncmp(b, "gen:", 4)) { int cf, ci; unsigned long long seed; char pb[128] = ""; sscanf(b + 4, "%d:%d:%d:%llu:%127s", &n, &cf, &ci, &seed, pb); NC = n * n; if (NC > MAXC) { fprintf(stderr, "MAXC too small\n"); return 1; }
    gen(cf, ci, seed);
    for (char *tk = strtok(pb, ","); tk; tk = strtok(0, ",")) { int c = (tk[0] == 'c') ? 0 : atoi(tk); pins[npins] = c; pinval[npins] = planted[c]; npins++; } }
  else if (!strncmp(b, "file:", 5)) { if (!load_file(b + 5)) { fprintf(stderr, "bad file\n"); return 1; } }
  else if (!strncmp(b, "e2r:", 4)) { if (!load_e2(b + 4, 1)) { fprintf(stderr, "bad csv\n"); return 1; } }
  else if (!strncmp(b, "e2:", 3)) { if (!load_e2(b + 3, 0)) { fprintf(stderr, "bad csv\n"); return 1; } }
  else return 1;
  if (NC > MAXC || 4 * NC > 64 * W || NCOL > 31) { fprintf(stderr, "compile-time limits too small (NC %d NCOL %d)\n", NC, NCOL); return 1; }
  finish_board();
  const char *pn[] = { "PLAIN", "FC", "AC" }, *rn[] = { "ROW", "ROWR", "DOM", "DOMF", "WDEG" };
  PROP = -1; RULE = -1; for (int i = 0; i < 3; i++) if (!strcmp(argv[2], pn[i])) PROP = i; for (int i = 0; i < 5; i++) if (!strcmp(argv[3], rn[i])) RULE = i;
  if (argv[3][0] == 'Y' && argv[3][1] >= '1' && argv[3][1] <= '9') { RULE = YK; KOPEN = atoi(argv[3] + 1); }
  if (argv[3][0] == 'L' && argv[3][1] >= '1' && argv[3][1] <= '9') { RULE = LA; KOPEN = atoi(argv[3] + 1); }
  if (argv[3][0] == 'B' && argv[3][1] >= '1' && argv[3][1] <= '9') { RULE = BK; KOPEN = atoi(argv[3] + 1); }
  if (PROP < 0 || RULE < 0) return 1;
  t0 = clock();
  if (!strcmp(argv[4], "exact")) {
    cap = strtoull(argv[5], 0, 10); tlimit = atof(argv[6]); learn = (RULE == WDEG);
    stk = malloc(sizeof(St) * (NC + 2)); prof = calloc(NC + 2, sizeof(uint64_t));
    if (root_init(&stk[0])) search(0);
    int pk = 0; for (int d = 0; d <= NC; d++) if (prof[d] > prof[pk]) pk = d;
    printf("{\"board\":\"%s\",\"n\":%d,\"prop\":\"%s\",\"rule\":\"%s\",\"mode\":\"exact\",\"tried\":%llu,\"ctried\":%llu,\"alive\":%llu,\"sols\":%llu,\"censored\":%d,\"peak_depth\":%d,\"sec\":%.2f}\n",
      desc, n, argv[2], argv[3], (unsigned long long)tried, (unsigned long long)ctried, (unsigned long long)alive, (unsigned long long)sols, censored, pk, (double)(clock() - t0) / CLOCKS_PER_SEC);
    return 0;
  }
  /* smc */
  int M = atoi(argv[5]), reps = atoi(argv[6]); uint64_t rseed = strtoull(argv[7], 0, 10); tlimit = atof(argv[8]);
  St *A = malloc(sizeof(St) * (size_t)M), *B = malloc(sizeof(St) * (size_t)M), tmp;
  size_t lcap = 1 << 16; uint32_t *li = malloc(lcap * 4), *lv = malloc(lcap * 4), *lx = malloc(lcap * 4);
  for (int rep = 0; rep < reps; rep++) {
    rs = (rseed + rep) * 0x2545F4914F6CDD1DULL + 12345; clock_t r0 = clock();
    int Mk = 0; double nk = 1.0, T = 0, CT = 0, NA = 0, S = 0, peak = 0; int pk = 0, cens = 0, level = 0, minpop = M;
    if (root_init(&A[0])) Mk = 1;
    for (; Mk > 0; level++) {
      if ((double)(clock() - t0) / CLOCKS_PER_SEC > tlimit) { cens = 2; break; }
      size_t na = 0; double sd = 0, sc = 0; int complete = 0;
      for (int i = 0; i < Mk; i++) { St *s = &A[i]; if (s->ndec == NC) { complete++; continue; }
        int x = select_cell(s); bs D = s->dom[x]; int c = bs_cnt(&D); sd += c; if (c >= 2) sc += c;
        for (int j = 0; j < W; j++) { uint64_t bb = D.w[j];
          while (bb) { int v = j * 64 + __builtin_ctzll(bb); bb &= bb - 1; tmp = *s;
            if (assign(&tmp, x, v)) { if (na == lcap) { lcap *= 2; li = realloc(li, lcap * 4); lv = realloc(lv, lcap * 4); lx = realloc(lx, lcap * 4); } li[na] = i; lv[na] = v; lx[na] = x; na++; } } } }
      T += nk * sd / Mk; CT += nk * sc / Mk; S += nk * complete / Mk;
      if (nk * sd / Mk > peak) { peak = nk * sd / Mk; pk = level; }
      if (!na) break;
      nk = nk * (double)na / Mk; NA += nk;
      if (getenv("PROFILE")) fprintf(stderr, "P %d %d %d %.4f\n", rep, level + 1, A[li[0]].ndec + 1, log2(nk));
      int m = na < (size_t)M ? (int)na : M;
      for (int j = 0; j < m; j++) { size_t r = j + (size_t)(rnd() % (na - j)); uint32_t z;
        z = li[j]; li[j] = li[r]; li[r] = z; z = lv[j]; lv[j] = lv[r]; lv[r] = z; z = lx[j]; lx[j] = lx[r]; lx[r] = z;
        B[j] = A[li[j]]; assign(&B[j], lx[j], lv[j]); }
      St *z = A; A = B; B = z; Mk = m; if (level > 8 && m < minpop) minpop = m;
    }
    printf("{\"board\":\"%s\",\"n\":%d,\"prop\":\"%s\",\"rule\":\"%s\",\"mode\":\"smc\",\"M\":%d,\"rep\":%d,\"log2_tried\":%.3f,\"log2_ctried\":%.3f,\"log2_alive\":%.3f,\"sols\":%.3g,\"censored\":%d,\"levels\":%d,\"peak_level\":%d,\"log2_peak\":%.3f,\"sec\":%.2f}\n",
      desc, n, argv[2], argv[3], M, rep, log2(T), CT > 0 ? log2(CT) : 0.0, NA > 0 ? log2(NA) : 0.0, S, cens, level, pk, peak > 0 ? log2(peak) : 0.0, (double)(clock() - r0) / CLOCKS_PER_SEC);
    fflush(stdout);
  }
  return 0;
}
