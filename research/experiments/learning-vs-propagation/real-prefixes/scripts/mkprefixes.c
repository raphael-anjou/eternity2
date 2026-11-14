/* learning-real-prefixes: sample wrong prefixes of the row scan tree on the OFFICIAL Eternity II pieces.
   Row scan from cell 0 (top left), rows top to bottom, columns left to right; the five clues are on the board from
   the start. A prefix at free depth d is a consistent placement of the first d free (non-clue) cells in scan
   order, the convention of the real-puzzle profile. (A first version of this
   file used a reversed definition, 251-d free cells placed; its pilot was discarded. See ../wrong-depth-pilot/.) Consistent = grey exactly on the rim, every joint between two filled cells matches
   (clue cells ahead of the frontier included).
   Sampler: one population descent per prefix (the scheme of dyn.c smc and of the real-puzzle profile). Level k
   holds up to M = 20000 nodes; all valid children of the population are listed and M of them are kept, drawn
   uniformly without replacement (Fisher-Yates, splitmix64 seeded with the prefix seed); at the target level one
   member of the population is drawn uniformly and is the prefix. Every node of a level has nearly the same chance
   of being in the population (exactly the same given the populations above it), so the prefix is close to a
   uniform draw from the level; the residual bias is of order 1/M and favours nothing I can name. Each prefix has
   its own population, so prefixes share no ancestor by construction.
   A single random descent (uniform child at each step, restart on a dead end) was the instruction; it dies before
   level 139 with probability near 1, and a randomised depth-first search from the root thrashes under cell 18 (the
   cell above the first clue), so neither could produce the prefixes. See ../notes/measurement.md.
   Writes prefixes/d<d>_<ii>.dyn (dyn.c file format: n, 256 pieces "top right bottom left", pin lines
   "P cell 4*piece+turn", clues first, then the prefix) and appends to prefixes.tsv:
   id, free depth, index, seed, children listed by the sampler, placements cell:piece:turn.
   usage: mkprefixes official.csv COUNT d1 d2 ...     seed of prefix i at depth d = 1000*d + i
   build: cc -O2 -o bin/mkprefixes mkprefixes.c */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#define N 16
#define NP 256
static int pc[NP][4], used[NP], has[NP], bd[NP][4], clue_c[8], clue_v[8], nclue, freec[NP], nfree;
static int cnt[32][32], lst[32][32][NP * 4][2];
static int pl_p[NP], pl_r[NP];
static uint64_t rs, nodes;
static uint64_t rnd(void) { uint64_t z = (rs += 0x9E3779B97F4A7C15ULL); z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9ULL; z = (z ^ (z >> 27)) * 0x94D049BB133111EBULL; return z ^ (z >> 31); }
static void put(int pos, int p, int r) { for (int j = 0; j < 4; j++) bd[pos][j] = pc[p][(j + 4 - r) % 4]; has[pos] = 1; used[p] = 1; }
#define M 20000
static int target;
static unsigned char (*A)[NP][2], (*B)[NP][2];      /* populations: placement (piece, turn) per free index */
static int *li, *lp, *lr; static size_t lcap;
static void rebuild(unsigned char st[NP][2], int k) {
  memset(used, 0, sizeof used); memset(has, 0, sizeof has);
  for (int c = 0; c < nclue; c++) put(clue_c[c], clue_v[c] / 4, clue_v[c] % 4);
  for (int j = 0; j < k; j++) put(freec[j], st[j][0], st[j][1]);
}
static int descend(void) {
  int Mk = 1;
  for (int k = 0; k < target; k++) {
    int pos = freec[k], y = pos / N, x = pos % N; size_t na = 0;
    for (int s = 0; s < Mk; s++) { rebuild(A[s], k);
      int left = x ? bd[pos - 1][1] : 0, top = y ? bd[pos - N][2] : 0;
      for (int i = 0; i < cnt[left][top]; i++) { int p = lst[left][top][i][0], r = lst[left][top][i][1];
        if (used[p]) continue;
        int ri = pc[p][(1 + 4 - r) % 4], bo = pc[p][(2 + 4 - r) % 4];
        if ((ri == 0) != (x == N - 1) || (bo == 0) != (y == N - 1)) continue;
        if (x < N - 1 && has[pos + 1] && bd[pos + 1][3] != ri) continue;
        if (y < N - 1 && has[pos + N] && bd[pos + N][0] != bo) continue;
        if (na == lcap) { lcap = lcap ? 2 * lcap : 1 << 16; li = realloc(li, lcap * sizeof(int)); lp = realloc(lp, lcap * sizeof(int)); lr = realloc(lr, lcap * sizeof(int)); }
        li[na] = s; lp[na] = p; lr[na] = r; na++; } }
    nodes += na;
    if (!na) return 0;
    int m = na < M ? (int)na : M;
    for (int j = 0; j < m; j++) { size_t q = j + (size_t)(rnd() % (uint64_t)(na - j)); int t;
      t = li[j]; li[j] = li[q]; li[q] = t; t = lp[j]; lp[j] = lp[q]; lp[q] = t; t = lr[j]; lr[j] = lr[q]; lr[q] = t;
      memcpy(B[j], A[li[j]], sizeof B[j]); B[j][k][0] = (unsigned char)lp[j]; B[j][k][1] = (unsigned char)lr[j]; }
    void *z = A; A = B; B = z; Mk = m;
  }
  int pick = (int)(rnd() % (uint64_t)Mk);
  for (int k = 0; k < target; k++) { pl_p[k] = A[pick][k][0]; pl_r[k] = A[pick][k][1]; }
  return 1;
}
int main(int argc, char **argv) {
  if (argc < 4) return 1;
  FILE *f = fopen(argv[1], "r"); char line[512]; int id = 0, got = 0;
  while (fgets(line, sizeof line, f)) { if (!got) { got = 1; continue; }
    char a[4][32]; int x, y, r; if (sscanf(line, "%31[01],%31[01],%31[01],%31[01],%d,%d,%d", a[0], a[1], a[2], a[3], &x, &y, &r) != 7) continue;
    for (int d = 0; d < 4; d++) { long v = strtol(a[d], 0, 2); pc[id][d] = (v == 65535) ? 0 : (int)v + 1; }
    if (x || y || r) { clue_c[nclue] = y * N + x; clue_v[nclue] = 4 * id + r; nclue++; } id++; }
  fclose(f); if (id != NP || nclue != 5) { fprintf(stderr, "bad csv\n"); return 1; }
  for (int p = 0; p < NP; p++) for (int r = 0; r < 4; r++) { int l = pc[p][(3 + 4 - r) % 4], t = pc[p][(0 + 4 - r) % 4]; lst[l][t][cnt[l][t]][0] = p; lst[l][t][cnt[l][t]][1] = r; cnt[l][t]++; }
  int count = atoi(argv[2]); FILE *tsv = fopen("prefixes.tsv", "a");
  for (int a = 3; a < argc; a++) { int d = atoi(argv[a]);
    for (int i = 0; i < count; i++) {
      memset(used, 0, sizeof used); memset(has, 0, sizeof has); nfree = 0; nodes = 0;
      for (int c = 0; c < nclue; c++) put(clue_c[c], clue_v[c] / 4, clue_v[c] % 4);
      for (int pos = 0; pos < NP; pos++) if (!has[pos]) freec[nfree++] = pos;
      target = d; uint64_t seed = 1000ULL * d + i; rs = seed;
      if (!A) { A = malloc(M * sizeof *A); B = malloc(M * sizeof *B); }
      int tries = 1; while (!descend()) { tries++; if (tries > 20) { fprintf(stderr, "population extinct 20 times\n"); return 2; } } /* an extinct population is redrawn with the same generator running on */
      if (tries > 1) fprintf(stderr, "d%d_%02d: %d populations\n", d, i, tries);
      char name[64]; snprintf(name, sizeof name, "prefixes/d%d_%02d.dyn", d, i); FILE *o = fopen(name, "w");
      fprintf(o, "# OFFICIAL pieces, five clues, row scan prefix, free depth %d, sampler seed %llu\n16\n", d, (unsigned long long)seed);
      for (int p = 0; p < NP; p++) fprintf(o, "%d %d %d %d\n", pc[p][0], pc[p][1], pc[p][2], pc[p][3]);
      for (int c = 0; c < nclue; c++) fprintf(o, "P %d %d\n", clue_c[c], clue_v[c]);
      for (int k = 0; k < target; k++) fprintf(o, "P %d %d\n", freec[k], 4 * pl_p[k] + pl_r[k]);
      fclose(o);
      fprintf(tsv, "d%d_%02d\t%d\t%d\t%llu\t%llu\t", d, i, d, i, (unsigned long long)seed, (unsigned long long)nodes);
      for (int k = 0; k < target; k++) fprintf(tsv, "%d:%d:%d%s", freec[k], pl_p[k], pl_r[k], k + 1 < target ? " " : "\n");
      fflush(tsv); } }
  fclose(tsv); return 0;
}
