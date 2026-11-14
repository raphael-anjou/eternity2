/* TOY. C port of probe B's two near-solution generators (probe_b_birkhoff_flow.py: anneal() and
 * dfs_near()), same moves, schedule and filters; only speed and the RNG differ. The board model
 * (placements, neighbours, solutions, the 200 random start boards) is read from a file written by
 * controls_c.py.
 *   anneal: start from one of the 200 random boards, swap two same-kind tiles (or re-rotate an
 *           interior tile, prob 0.3), Metropolis on mismatched inner edges, temperature
 *           1.5 * (1 - s/steps) + 0.05. Keep the end board if it has <= maxm mismatches and is
 *           >= 8 cells (placement indices) from every solution.
 *   dfs:    randomised row-scan DFS that tolerates <= maxm mismatches; a complete board >= 8 cells
 *           from every solution is kept and the DFS restarts.
 * usage: nearctl model.txt anneal|dfs steps_or_budget seconds want maxm seed
 * prints one kept board per line (placement indices, cell order), then a line "# stats ...". */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <time.h>

static int N, P, T, n, NS, NC;
static int *cellOf, *tileOf, (*cols)[4], (*nb)[4], *sols, *ctrl;
static int **opt; static int *optn;   /* opt[(cell*T+tile)] list of placements */
static int *bycell[4096]; static int bycn[4096];
static unsigned long long rs;
static unsigned long long rnd(void) { rs ^= rs >> 12; rs ^= rs << 25; rs ^= rs >> 27; return rs * 2685821657736338717ULL; }
static double urand(void) { return (rnd() >> 11) * (1.0 / 9007199254740992.0); }
static int irand(int k) { return (int)(urand() * k); }

static int mism_cell(int x, int *js) {
    int m = 0, *cx = cols[js[x]];
    for (int d = 0; d < 4; d++) { int y = nb[x][d]; if (y >= 0 && cx[d] != cols[js[y]][(d + 2) % 4]) m++; }
    return m;
}
static int mism_total(int *js) { int m = 0; for (int x = 0; x < N; x++) m += mism_cell(x, js); return m / 2; }
static int dist_sols(int *js) {
    int best = 1 << 30;
    for (int s = 0; s < NS; s++) { int d = 0; for (int x = 0; x < N; x++) d += sols[s * N + x] != js[x]; if (d < best) best = d; }
    return best;
}
static void emit(int *js) { for (int x = 0; x < N; x++) printf("%d%c", js[x], x == N - 1 ? '\n' : ' '); fflush(stdout); }

static int kind_cells[3][4096], kind_n[3];

static int anneal(int *js, long steps) {
    memcpy(js, ctrl + irand(NC) * N, sizeof(int) * N);
    for (long st = 0; st < steps; st++) {
        double temp = 1.5 * (1.0 - (double)st / steps) + 0.05;
        int k = irand(3);
        if (kind_n[k] < 2) continue;
        int a = irand(kind_n[k]), b = irand(kind_n[k] - 1); if (b >= a) b++;
        int x = kind_cells[k][a], y = kind_cells[k][b];
        if (k == 0 && urand() < 0.3) y = x;
        int aff[10], na = 0;
        aff[na++] = x; if (y != x) aff[na++] = y;
        for (int d = 0; d < 4; d++) { if (nb[x][d] >= 0) aff[na++] = nb[x][d]; if (nb[y][d] >= 0) aff[na++] = nb[y][d]; }
        /* dedupe */
        int u[10], nu = 0;
        for (int i = 0; i < na; i++) { int seen = 0; for (int j = 0; j < nu; j++) if (u[j] == aff[i]) seen = 1; if (!seen) u[nu++] = aff[i]; }
        int before = 0; for (int i = 0; i < nu; i++) before += mism_cell(u[i], js);
        int ox = js[x], oy = js[y], tx = tileOf[js[x]], ty = tileOf[js[y]];
        if (x == y) { int key = x * T + tx; js[x] = opt[key][irand(optn[key])]; }
        else {
            int k1 = x * T + ty, k2 = y * T + tx;
            js[x] = opt[k1][irand(optn[k1])]; js[y] = opt[k2][irand(optn[k2])];
        }
        int after = 0; for (int i = 0; i < nu; i++) after += mism_cell(u[i], js);
        double delta = (after - before) / 2.0;
        if (!(delta <= 0 || urand() < exp(-delta / temp))) { js[x] = ox; js[y] = oy; }
    }
    return mism_total(js);
}

static long nodes, budget; static int maxm_g, found_flag;
static int used[4096], cur[4096];
static void rec(int x, int mm) {
    if (found_flag || nodes > budget) return;
    nodes++;
    if (x == N) { if (dist_sols(cur) >= 8) { emit(cur); found_flag = 1; } return; }
    int r = x / n, c = x % n, m = bycn[x];
    int cand[4096]; memcpy(cand, bycell[x], sizeof(int) * m);
    for (int i = m - 1; i > 0; i--) { int j = irand(i + 1); int t = cand[i]; cand[i] = cand[j]; cand[j] = t; }
    for (int i = 0; i < m; i++) {
        int j = cand[i], t = tileOf[j];
        if (used[t]) continue;
        int add = 0;
        if (r > 0 && cols[j][0] != cols[cur[x - n]][2]) add++;
        if (c > 0 && cols[j][3] != cols[cur[x - 1]][1]) add++;
        if (mm + add > maxm_g) continue;
        used[t] = 1; cur[x] = j;
        rec(x + 1, mm + add);
        used[t] = 0;
        if (found_flag || nodes > budget) return;
    }
}

int main(int argc, char **argv) {
    if (argc < 8) { fprintf(stderr, "usage\n"); return 1; }
    FILE *f = fopen(argv[1], "r");
    int pin_p;
    if (fscanf(f, "%d %d %d %d %d %d %d", &N, &P, &T, &n, &NS, &NC, &pin_p) != 7) return 2;
    cellOf = malloc(sizeof(int) * P); tileOf = malloc(sizeof(int) * P); cols = malloc(sizeof(int[4]) * P);
    nb = malloc(sizeof(int[4]) * N); sols = malloc(sizeof(int) * NS * N); ctrl = malloc(sizeof(int) * NC * N);
    for (int p = 0; p < P; p++) if (fscanf(f, "%d %d %d %d %d %d", &cellOf[p], &tileOf[p], &cols[p][0], &cols[p][1], &cols[p][2], &cols[p][3]) != 6) return 3;
    for (int x = 0; x < N; x++) if (fscanf(f, "%d %d %d %d", &nb[x][0], &nb[x][1], &nb[x][2], &nb[x][3]) != 4) return 4;
    for (int i = 0; i < NS * N; i++) if (fscanf(f, "%d", &sols[i]) != 1) return 5;
    for (int i = 0; i < NC * N; i++) if (fscanf(f, "%d", &ctrl[i]) != 1) return 6;
    int nkind; if (fscanf(f, "%d", &nkind) != 1) return 7;
    for (int k = 0; k < 3; k++) { if (fscanf(f, "%d", &kind_n[k]) != 1) return 8; for (int i = 0; i < kind_n[k]; i++) if (fscanf(f, "%d", &kind_cells[k][i]) != 1) return 9; }
    fclose(f);
    opt = calloc((size_t)N * T, sizeof(int *)); optn = calloc((size_t)N * T, sizeof(int));
    for (int p = 0; p < P; p++) { int key = cellOf[p] * T + tileOf[p]; opt[key] = realloc(opt[key], sizeof(int) * (optn[key] + 1)); opt[key][optn[key]++] = p; }
    for (int x = 0; x < N; x++) { bycell[x] = malloc(sizeof(int) * P); bycn[x] = 0; }
    for (int p = 0; p < P; p++) bycell[cellOf[p]][bycn[cellOf[p]]++] = p;
    const char *mode = argv[2]; long steps = atol(argv[3]); double secs = atof(argv[4]);
    int want = atoi(argv[5]); maxm_g = atoi(argv[6]); rs = 0x9E3779B97F4A7C15ULL ^ (unsigned long long)atol(argv[7]);
    time_t t0 = time(NULL);
    int kept = 0; long tries = 0; long hist[64] = {0}; long near_close = 0;
    int *js = malloc(sizeof(int) * N);
    if (!strcmp(mode, "anneal")) {
        while (kept < want && difftime(time(NULL), t0) < secs) {
            tries++;
            int m = anneal(js, steps); hist[m < 63 ? m : 63]++;
            if (m <= maxm_g) { if (dist_sols(js) >= 8) { emit(js); kept++; } else near_close++; }
        }
        printf("# stats mode=anneal steps=%ld tries=%ld kept=%d rejected_close=%ld seconds=%.0f hist", steps, tries, kept, near_close, difftime(time(NULL), t0));
        for (int i = 0; i < 64; i++) if (hist[i]) printf(" %d:%ld", i, hist[i]);
        printf("\n");
    } else {
        budget = steps; nodes = 0;
        while (kept < want && nodes <= budget && difftime(time(NULL), t0) < secs) {
            memset(used, 0, sizeof(used)); used[tileOf[pin_p]] = 1; cur[0] = pin_p; found_flag = 0;
            rec(1, 0);
            if (found_flag) kept++;
            tries++;
        }
        printf("# stats mode=dfs budget=%ld nodes=%ld restarts=%ld kept=%d seconds=%.0f\n", budget, nodes, tries, kept, difftime(time(NULL), t0));
    }
    return 0;
}
