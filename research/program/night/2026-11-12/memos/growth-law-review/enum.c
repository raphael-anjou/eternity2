/* Referee enumerator for framed edge-matching boards (review of growth-law.md).
   Row-major depth-first search, one node per consistent placement, candidates in
   piece-index then rotation order, identical orientations of a piece skipped
   unless ALLROT is set. Reads one instance per line on stdin:
       tag n e0 e1 e2 e3 ...      (n*n pieces, URDL, 0 = rim)
   Usage: enum MODE CAP [allrot]
     first   nodes to the first solution
     full    whole tree: nodes, solutions, per-depth counts
     census  whole tree plus, per depth, how many nodes have no strict
             descendant 1, n, 2n cells deeper and how many have no relaxed
             (pieces reusable inside the lookahead) filling of the next 1, n, 2n cells
     ring    number of valid frames (ring seams only)
   Build: cc -O2 -o <scratch>/enum enum.c */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define MAXP 144
#define MAXC 48
typedef struct { int p, r, b; } Cand;
static int n, N, np, pc[MAXP][4];
static Cand store[MAXP * 4];
static int tstart[MAXC][MAXC], tcount[MAXC][MAXC];
static unsigned long long nodes, cap, Z[MAXP + 1], sols;
static unsigned long long sdead[MAXP + 1][3], rdead[MAXP + 1][3];
static int used[MAXP], rgt[MAXP + 1], bot[MAXP + 1], stop, first_mode, census, allrot;

static void build(void) {
    int cnt[MAXC][MAXC]; memset(cnt, 0, sizeof cnt); memset(tcount, 0, sizeof tcount);
    static int ou[MAXP * 4][5]; int m = 0;
    for (int p = 0; p < np; p++) {
        int seen[4][4], ns = 0;
        for (int r = 0; r < 4; r++) {
            int o[4]; for (int i = 0; i < 4; i++) o[i] = pc[p][(i - r + 4) % 4];
            int dup = 0;
            if (!allrot) for (int s = 0; s < ns; s++) if (!memcmp(seen[s], o, sizeof o)) dup = 1;
            if (dup) continue;
            memcpy(seen[ns++], o, sizeof o);
            ou[m][0] = p; memcpy(&ou[m][1], o, sizeof o); m++;
            tcount[o[0]][o[3]]++;
        }
    }
    int at = 0;
    for (int t = 0; t < MAXC; t++) for (int l = 0; l < MAXC; l++) { tstart[t][l] = at; at += tcount[t][l]; }
    for (int i = 0; i < m; i++) {
        int t = ou[i][1], l = ou[i][4];
        Cand c = { ou[i][0], ou[i][2], ou[i][3] };
        store[tstart[t][l] + cnt[t][l]++] = c;
    }
}

/* relaxed filling of cells pos..end-1: pieces used on the current path are excluded,
   but a piece may be reused inside the lookahead */
static int relax(int pos, int end) {
    if (pos >= end || pos >= N) return 1;
    int x = pos % n, y = pos / n, t = y ? bot[pos - n] : 0, l = x ? rgt[pos - 1] : 0;
    Cand *c = store + tstart[t][l];
    int sr = rgt[pos], sb = bot[pos], ok = 0;
    for (int k = tcount[t][l]; k > 0 && !ok; k--, c++) {
        if (used[c->p]) continue;
        if ((c->r == 0) != (x == n - 1)) continue;
        if ((c->b == 0) != (y == n - 1)) continue;
        rgt[pos] = c->r; bot[pos] = c->b;
        ok = relax(pos + 1, end);
    }
    rgt[pos] = sr; bot[pos] = sb;
    return ok;
}

/* node at depth pos (pos cells filled); returns the deepest depth reached below it */
static int dfs(int pos) {
    if (pos == N) { sols++; if (first_mode) stop = 1; return N; }
    int x = pos % n, y = pos / n, t = y ? bot[pos - n] : 0, l = x ? rgt[pos - 1] : 0;
    Cand *c = store + tstart[t][l];
    int best = pos;
    for (int k = tcount[t][l]; k > 0; k--, c++) {
        if (used[c->p]) continue;
        if ((c->r == 0) != (x == n - 1)) continue;
        if ((c->b == 0) != (y == n - 1)) continue;
        nodes++; Z[pos + 1]++;
        if (nodes >= cap && !(pos + 1 == N)) { stop = 2; return best; }
        used[c->p] = 1; rgt[pos] = c->r; bot[pos] = c->b;
        if (census) {
            int L[3] = { 1, n, 2 * n };
            for (int i = 0; i < 3; i++) if (pos + 1 + L[i] <= N && !relax(pos + 1, pos + 1 + L[i])) rdead[pos + 1][i]++;
        }
        int m = dfs(pos + 1);
        used[c->p] = 0;
        if (census && !stop) {
            int L[3] = { 1, n, 2 * n }, reach = m - (pos + 1);
            for (int i = 0; i < 3; i++) if (pos + 1 + L[i] <= N && reach < L[i]) sdead[pos + 1][i]++;
        }
        if (m > best) best = m;
        if (stop) return best;
    }
    return best;
}

/* valid frames: clockwise ring of 4 corners and 4(n-2) edge pieces, ring seams only */
static int rin[MAXP], rout[MAXP], rkind[MAXP], rused[MAXP], slotkind[MAXP], nring, first_in;
static unsigned long long rcount, rnodes;
static void ringdfs(int s, int need) {
    if (s == nring) { if (need == first_in) rcount++; return; }
    for (int p = 0; p < np; p++) {
        if (rkind[p] != slotkind[s] || rused[p]) continue;
        if (s > 0 && rin[p] != need) continue;
        if (s == 0) first_in = rin[p];
        rused[p] = 1; rnodes++;
        ringdfs(s + 1, rout[p]);
        rused[p] = 0;
    }
}
static void ring(void) {
    for (int p = 0; p < np; p++) {
        int z = 0, zi = -1;
        for (int i = 0; i < 4; i++) if (pc[p][i] == 0) z++;
        rkind[p] = z; rused[p] = 0;
        if (z == 1) { for (int i = 0; i < 4; i++) if (pc[p][i] == 0) zi = i;
            rout[p] = pc[p][(zi + 1) % 4]; rin[p] = pc[p][(zi + 3) % 4]; }
        if (z == 2) { for (int i = 0; i < 4; i++) if (pc[p][i] == 0 && pc[p][(i + 3) % 4] == 0) zi = i;
            /* rims at zi (up) and zi-1 (left): canonical (0, r, d, 0), in = d, out = r */
            rout[p] = pc[p][(zi + 1) % 4]; rin[p] = pc[p][(zi + 2) % 4]; }
    }
    nring = 4 * (n - 1);
    for (int s = 0; s < nring; s++) slotkind[s] = (s % (n - 1) == 0) ? 2 : 1;
    rcount = rnodes = 0;
    ringdfs(0, 0);
}

int main(int argc, char **argv) {
    const char *mode = argv[1];
    cap = strtoull(argv[2], 0, 10);
    allrot = argc > 3 && !strcmp(argv[3], "allrot");
    first_mode = !strcmp(mode, "first"); census = !strcmp(mode, "census");
    char tag[64];
    while (scanf("%63s %d", tag, &n) == 2) {
        N = np = n * n;
        for (int p = 0; p < np; p++) for (int i = 0; i < 4; i++) if (scanf("%d", &pc[p][i]) != 1) return 1;
        if (!strcmp(mode, "ring")) { ring(); printf("{\"tag\":\"%s\",\"frames\":%llu,\"ring_nodes\":%llu}\n", tag, rcount, rnodes); continue; }
        build();
        nodes = sols = 0; stop = 0; memset(Z, 0, sizeof Z); memset(used, 0, sizeof used);
        memset(sdead, 0, sizeof sdead); memset(rdead, 0, sizeof rdead);
        dfs(0);
        printf("{\"tag\":\"%s\",\"n\":%d,\"nodes\":%llu,\"solutions\":%llu,\"capped\":%s,\"Z\":[", tag, n, nodes, sols, stop == 2 ? "true" : "false");
        for (int d = 0; d <= N; d++) printf("%s%llu", d ? "," : "", Z[d]);
        printf("]");
        if (census) for (int i = 0; i < 3; i++) {
            printf(",\"strict%d\":[", i); for (int d = 0; d <= N; d++) printf("%s%llu", d ? "," : "", sdead[d][i]); printf("]");
            printf(",\"relaxed%d\":[", i); for (int d = 0; d <= N; d++) printf("%s%llu", d ? "," : "", rdead[d][i]); printf("]");
        }
        printf("}\n");
    }
    return 0;
}
