/* learning-real-prefixes: plain row scan below a prefix, whole subtree. After ../learning-vs-propagation/rowscan.c
   (same node definition), changed to read the dyn.c file format with pin lines, to respect pinned cells anywhere
   (the clues ahead of the frontier), and to stop on WALL time and print its counts on any exit.
   No propagation, no learning. Free cells in row order. A placement is valid iff grey exactly on the rim, it
   matches the cell on its left and the cell above, and the cells on its right and below when those are pinned.
   node = valid placement on a free cell; dead = node that is not a full board and has no child.
   usage: rowscan_real <file.dyn> <wall_seconds>      build: cc -O2 -o rowscan_real rowscan_real.c */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <signal.h>
#include <unistd.h>
#include <time.h>
#include <sys/time.h>
#define NP 256
#define MAXCOL 32
static int n, np, pc[NP][4], used[NP], bot[NP], rgt[NP], lftc[NP], topc[NP], pinned[NP], freec[NP], nfree;
static int cnt[MAXCOL][MAXCOL], *lst[MAXCOL][MAXCOL];
static volatile unsigned long long nodes, dead, sols;
static struct timespec w0;
static void emit(int cens) {
  struct timespec w1; clock_gettime(CLOCK_MONOTONIC, &w1); char b[400];
  int k = snprintf(b, sizeof b, "{\"arm\":\"T\",\"nodes\":%llu,\"dead\":%llu,\"sols\":%llu,\"censored\":%d,\"wall\":%.4f,\"cpu\":%.4f}\n",
    nodes, dead, sols, cens, (w1.tv_sec - w0.tv_sec) + 1e-9 * (w1.tv_nsec - w0.tv_nsec), (double)clock() / CLOCKS_PER_SEC);
  if (write(1, b, k) < 0) {}
}
static void on_sig(int s) { emit(s == SIGALRM ? 2 : 3); _exit(0); }
static int rec(int k) {
  if (k == nfree) { sols++; return 1; }
  int pos = freec[k], kids = 0, y = pos / n, x = pos % n;
  int left = x ? rgt[pos - 1] : 0, top = y ? bot[pos - n] : 0;
  int nr = (x < n - 1 && pinned[pos + 1]) ? lftc[pos + 1] : -1, nb = (y < n - 1 && pinned[pos + n]) ? topc[pos + n] : -1;
  int *l = lst[left][top], c = cnt[left][top];
  for (int i = 0; i < c; i++) {
    int p = l[3 * i], ri = l[3 * i + 1], bo = l[3 * i + 2];
    if (used[p]) continue;
    if ((x == n - 1) != (ri == 0)) continue;
    if ((y == n - 1) != (bo == 0)) continue;
    if (nr >= 0 && ri != nr) continue;
    if (nb >= 0 && bo != nb) continue;
    nodes++; kids++;
    used[p] = 1; rgt[pos] = ri; bot[pos] = bo;
    if (!rec(k + 1)) dead++;
    used[p] = 0;
  }
  return kids;
}
int main(int argc, char **argv) {
  if (argc < 3) return 1;
  clock_gettime(CLOCK_MONOTONIC, &w0);
  FILE *f = fopen(argv[1], "r"); if (!f) { fprintf(stderr, "no file\n"); return 1; }
  char line[256]; int got = 0, id = 0, pinc[NP], pinv[NP], npin = 0;
  while (fgets(line, sizeof line, f)) { if (line[0] == '#') continue;
    if (line[0] == 'P') { if (sscanf(line + 1, "%d %d", &pinc[npin], &pinv[npin]) == 2) npin++; continue; }
    if (!got) { n = atoi(line); np = n * n; got = 1; if (np > NP) return 1; continue; }
    if (sscanf(line, "%d %d %d %d", &pc[id][0], &pc[id][1], &pc[id][2], &pc[id][3]) == 4) id++; }
  fclose(f); if (id != np) { fprintf(stderr, "bad file\n"); return 1; }
  for (int a = 0; a < MAXCOL; a++) for (int b = 0; b < MAXCOL; b++) lst[a][b] = malloc(sizeof(int) * 3 * 4 * np);
  for (int p = 0; p < np; p++) {
    int seen[4][4], ns = 0;
    for (int r = 0; r < 4; r++) {
      int e[4], dup = 0;
      for (int i = 0; i < 4; i++) e[i] = pc[p][(i + 4 - r) % 4];
      for (int s = 0; s < ns; s++) if (seen[s][0] == e[0] && seen[s][1] == e[1] && seen[s][2] == e[2] && seen[s][3] == e[3]) dup = 1;
      if (dup) continue;
      for (int i = 0; i < 4; i++) seen[ns][i] = e[i];
      ns++;
      int *l = lst[e[3]][e[0]], k = cnt[e[3]][e[0]]++;
      l[3 * k] = p; l[3 * k + 1] = e[1]; l[3 * k + 2] = e[2];
    }
  }
  for (int i = 0; i < npin; i++) { int pos = pinc[i], p = pinv[i] / 4, r = pinv[i] % 4, e[4];
    for (int j = 0; j < 4; j++) e[j] = pc[p][(j + 4 - r) % 4];
    if (pinned[pos] || used[p]) { fprintf(stderr, "pin clash\n"); return 2; }
    pinned[pos] = 1; used[p] = 1; topc[pos] = e[0]; rgt[pos] = e[1]; bot[pos] = e[2]; lftc[pos] = e[3]; }
  for (int pos = 0; pos < np; pos++) { int y = pos / n, x = pos % n;
    if (!pinned[pos]) { freec[nfree++] = pos; continue; }
    if ((topc[pos] == 0) != (y == 0) || (rgt[pos] == 0) != (x == n - 1) || (bot[pos] == 0) != (y == n - 1) || (lftc[pos] == 0) != (x == 0)) { fprintf(stderr, "pin off rim rule at %d\n", pos); return 2; }
    if (x < n - 1 && pinned[pos + 1] && rgt[pos] != lftc[pos + 1]) { fprintf(stderr, "pins mismatch at %d\n", pos); return 2; }
    if (y < n - 1 && pinned[pos + n] && bot[pos] != topc[pos + n]) { fprintf(stderr, "pins mismatch at %d\n", pos); return 2; } }
  /* a pinned cell whose left or upper neighbour is a free cell BEHIND it in scan order is covered by nr / nb above */
  signal(SIGALRM, on_sig); signal(SIGTERM, on_sig); signal(SIGINT, on_sig);
  struct itimerval it; memset(&it, 0, sizeof it); double lim = atof(argv[2]); it.it_value.tv_sec = (long)lim; it.it_value.tv_usec = (long)((lim - (long)lim) * 1e6); setitimer(ITIMER_REAL, &it, 0);
  rec(0);
  emit(0);
  return 0;
}
