/* Second analysis script (gap 1 review): plain row-scan backtracking on a TOY board, whole tree.
   No propagation, no learning. Rows top to bottom, columns left to right.
   A placement is valid iff its rim sides are grey (0), its other sides are not grey,
   and it matches the cell on its left and the cell above. Cell 0 is pinned.
   stdin: n, then n*n lines "top right bottom left", then "pin_piece pin_rot".
   stdout: nodes dead_ends solutions   (a node is a valid placement; a dead end is a
   node that is not a full board and has no child).
   build: cc -O2 -o rowscan rowscan.c */
#include <stdio.h>
#include <stdlib.h>
#define MAXC 32
static int n, np, pc[400][4], used[400], bot[400], rgt[400], pinp, pinr;
/* oriented placements by (left colour, top colour): piece, right, bottom */
static int cnt[MAXC][MAXC], *lst[MAXC][MAXC];
static unsigned long long nodes, dead, sols, cap;
static int rec(int pos) {
  int kids = 0, y = pos / n, x = pos % n;
  if (pos == np) { sols++; return 1; }
  int left = x ? rgt[pos - 1] : 0, top = y ? bot[pos - n] : 0;
  int *l = lst[left][top], c = cnt[left][top];
  for (int i = 0; i < c; i++) {
    int p = l[3 * i], ri = l[3 * i + 1], bo = l[3 * i + 2];
    if (used[p]) continue;
    if ((x == n - 1) != (ri == 0)) continue;
    if ((y == n - 1) != (bo == 0)) continue;
    if (pos == 0 && p != pinp) continue;
    nodes++; kids++;
    if (nodes > cap) { printf("CAP\n"); exit(0); }
    used[p] = 1; rgt[pos] = ri; bot[pos] = bo;
    if (!rec(pos + 1)) dead++;
    used[p] = 0;
  }
  return kids;
}
int main(int argc, char **argv) {
  cap = argc > 1 ? strtoull(argv[1], 0, 10) : 400000000000ULL;
  if (scanf("%d", &n) != 1) return 1;
  np = n * n;
  for (int i = 0; i < np; i++) for (int j = 0; j < 4; j++) if (scanf("%d", &pc[i][j]) != 1) return 1;
  if (scanf("%d %d", &pinp, &pinr) != 2) return 1;
  for (int a = 0; a < MAXC; a++) for (int b = 0; b < MAXC; b++) lst[a][b] = malloc(sizeof(int) * 3 * 4 * np);
  for (int p = 0; p < np; p++) {
    int seen[4][4], ns = 0;
    for (int r = 0; r < 4; r++) {
      int e[4], dup = 0;
      for (int i = 0; i < 4; i++) e[i] = pc[p][(i + 4 - r) % 4];
      for (int s = 0; s < ns; s++)
        if (seen[s][0] == e[0] && seen[s][1] == e[1] && seen[s][2] == e[2] && seen[s][3] == e[3]) dup = 1;
      if (dup) continue;
      for (int i = 0; i < 4; i++) seen[ns][i] = e[i];
      ns++;
      int *l = lst[e[3]][e[0]], k = cnt[e[3]][e[0]]++;
      l[3 * k] = p; l[3 * k + 1] = e[1]; l[3 * k + 2] = e[2];
    }
  }
  rec(0);
  printf("%llu %llu %llu\n", nodes, dead, sols);
  return 0;
}
