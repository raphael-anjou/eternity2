/* Gap 3: shape of the exact row-scan tree. One board file from g3_prep.py (scan is always
 * rows top-down in the file's frame; pinned cells are on the board from the start and are
 * not nodes, as in Track 10's tree.rs).
 *
 *   g3tree <board> <P> <seed> <dmax> <m> <cap> <d1,d2,...> <nprobe> <mlive> <nfr> <rcap> [<PS>]
 *
 * With the optional PS, only the sibling mode runs (planted boards): one line "SB j size" per
 * sibling of the planted path, size estimated by an SMC of population PS from that sibling.
 *
 * Prints, one record per line:
 *   PATH b_0 b_1 ...           children of the planted prefix at each free step (planted boards)
 *   PS d log2Wstar S capped    Knuth weight and exact subtree size below the planted prefix
 *   LV k M E zero mean var log10N meanlog2W log2sumInvW
 *                              population SMC (uniform resampling without replacement):
 *                              level statistics before step k, the width estimate after it,
 *                              the population mean of log2 Knuth weight and
 *                              log2(N * mean(1/W)), an estimate of the probability that a
 *                              uniform-child probe survives to this depth
 *   US d log2W S capped        m population members at depth d: Knuth weight, subtree size
 *   MU d surv meanlen          nprobe uniform-child probes from the root
 *   PL d log2Wlive ncap        planted prefix: product over its ancestors of the number of
 *                              children that have a descendant at depth d ("live" children;
 *                              a search capped at rcap nodes counts as live, ncap of them)
 *   UL d log2Wlive ncap log2W  the same for mlive population members at depth d
 *   FR d cost S capped         (capped = -1: the search itself hit cap before depth d) nfr runs of depth-first search with random value order from
 *                              the root, stopped at the first node of depth d: nodes spent,
 *                              then the subtree size below that node (to cap)
 * Subtree size = number of valid partial boards strictly below the prefix, counted to cap.
 */
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define NN 16
#define CELLS 256
#define MAXC 32

typedef struct { int pid; uint8_t e[4]; } Ori;
static Ori ori[1024];
static int nori;
static uint16_t *lst[MAXC][MAXC];
static int llen[MAXC][MAXC];
static uint8_t B[CELLS][4];
static int isfixed[CELLS], order[CELLS], nfree;
static int pieces[CELLS][4];
static int solpid[CELLS], sole[CELLS][4], havesol;

static uint64_t rs[4];
static uint64_t rnext(void) {
    uint64_t r = ((rs[1] * 5) << 7 | (rs[1] * 5) >> 57) * 9, t = rs[1] << 17;
    rs[2] ^= rs[0]; rs[3] ^= rs[1]; rs[1] ^= rs[2]; rs[0] ^= rs[3]; rs[2] ^= t;
    rs[3] = rs[3] << 45 | rs[3] >> 19;
    return r;
}
static void rseed(uint64_t s) {
    for (int i = 0; i < 4; i++) {
        s += 0x9E3779B97F4A7C15ULL;
        uint64_t x = s;
        x = (x ^ (x >> 30)) * 0xBF58476D1CE4E5B9ULL;
        x = (x ^ (x >> 27)) * 0x94D049BB133111EBULL;
        rs[i] = x ^ (x >> 31);
    }
}
static double runit(void) { return (rnext() >> 11) / 9007199254740992.0; }

#define USED(m, p) ((m)[(p) >> 6] >> ((p)&63) & 1)

/* valid children at free step k; B must hold the cells above and left of it */
static inline int children(int k, const uint64_t *mask, uint16_t *out) {
    int p = order[k], y = p / NN, x = p % NN;
    int top = y ? B[p - NN][2] : 0, left = x ? B[p - 1][1] : 0;
    int rmode = x == NN - 1 ? 0 : (isfixed[p + 1] ? B[p + 1][3] : -1);
    int bmode = y == NN - 1 ? 0 : (isfixed[p + NN] ? B[p + NN][0] : -1);
    const uint16_t *l = lst[top][left];
    int n = llen[top][left], c = 0;
    for (int i = 0; i < n; i++) {
        const Ori *o = &ori[l[i]];
        if (USED(mask, o->pid)) continue;
        if (rmode < 0 ? o->e[1] == 0 : o->e[1] != rmode) continue;
        if (bmode < 0 ? o->e[2] == 0 : o->e[2] != bmode) continue;
        out[c++] = l[i];
    }
    return c;
}

static uint64_t dfs_nodes, dfs_cap, dfs_sols;
static uint64_t dmask[4];
static void dfs(int k) {
    if (k == nfree) { dfs_sols++; return; }
    uint16_t ch[128];
    int n = children(k, dmask, ch), p = order[k];
    for (int i = 0; i < n && dfs_nodes < dfs_cap; i++) {
        const Ori *o = &ori[ch[i]];
        dfs_nodes++;
        memcpy(B[p], o->e, 4);
        dmask[o->pid >> 6] |= 1ULL << (o->pid & 63);
        dfs(k + 1);
        dmask[o->pid >> 6] &= ~(1ULL << (o->pid & 63));
    }
}
static uint64_t subtree(int k, const uint64_t *mask, uint64_t cap) {
    memcpy(dmask, mask, 32);
    dfs_nodes = 0; dfs_cap = cap; dfs_sols = 0;
    dfs(k);
    return dfs_nodes;
}

/* does the state at step k have a descendant at depth d? 1 yes, 0 no, 2 cap reached */
static uint64_t r_nodes, r_cap;
static int reach_rec(int k, int d) {
    if (k >= d) return 1;
    uint16_t ch[128];
    int n = children(k, dmask, ch), p = order[k];
    for (int i = 0; i < n; i++) {
        const Ori *o = &ori[ch[i]];
        if (r_nodes >= r_cap) return 2;
        r_nodes++;
        memcpy(B[p], o->e, 4);
        dmask[o->pid >> 6] |= 1ULL << (o->pid & 63);
        int r = reach_rec(k + 1, d);
        dmask[o->pid >> 6] &= ~(1ULL << (o->pid & 63));
        if (r) return r;
    }
    return 0;
}
/* log2 of the product of live branching along the path path[0..d-1] (oriented indices) */
static double live_weight(const uint16_t *path, int d, const uint64_t *fixmask, uint64_t rcap, int *ncap) {
    uint64_t mask[4];
    memcpy(mask, fixmask, 32);
    double l2 = 0;
    uint16_t ch[128];
    *ncap = 0;
    for (int j = 0; j < d; j++) {
        int p = order[j];
        int c = children(j, mask, ch), live = 1;
        for (int i = 0; i < c; i++) {
            if (ch[i] == path[j]) continue;
            const Ori *o = &ori[ch[i]];
            memcpy(B[p], o->e, 4);
            memcpy(dmask, mask, 32);
            dmask[o->pid >> 6] |= 1ULL << (o->pid & 63);
            r_nodes = 0; r_cap = rcap;
            int r = reach_rec(j + 1, d);
            if (r) live++;
            if (r == 2) (*ncap)++;
        }
        l2 += log2((double)live);
        const Ori *o = &ori[path[j]];
        memcpy(B[p], o->e, 4);
        mask[o->pid >> 6] |= 1ULL << (o->pid & 63);
    }
    return l2;
}
/* randomised depth-first search to the first node of depth d; returns 1 when reached */
static uint64_t fr_nodes, fr_cap;
static int fr_rec(int k, int d) {
    if (k >= d) return 1;
    uint16_t ch[128];
    int n = children(k, dmask, ch), p = order[k];
    for (int i = n - 1; i > 0; i--) { int j = (int)(rnext() % (uint64_t)(i + 1)); uint16_t t = ch[i]; ch[i] = ch[j]; ch[j] = t; }
    for (int i = 0; i < n; i++) {
        const Ori *o = &ori[ch[i]];
        if (fr_nodes >= fr_cap) return 2;
        fr_nodes++;
        memcpy(B[p], o->e, 4);
        dmask[o->pid >> 6] |= 1ULL << (o->pid & 63);
        int r = fr_rec(k + 1, d);
        if (r) return r;
        dmask[o->pid >> 6] &= ~(1ULL << (o->pid & 63));
    }
    return 0;
}

/* Estimated number of nodes strictly below the current state at step k0 (B holds every cell of
 * steps < k0), by population SMC with at most P members; exact whenever no level exceeds P. */
static int pos2step_g[CELLS];
static double smc_size(int k0, const uint64_t *mask0, long P, int dmax) {
    static uint16_t *sq, *sq2;
    static uint64_t (*mk)[4], (*mk2)[4];
    static int *cnt;
    static long cap_alloc;
    if (cap_alloc < P) {
        sq = malloc((size_t)P * nfree * 2); sq2 = malloc((size_t)P * nfree * 2);
        mk = malloc((size_t)P * 32); mk2 = malloc((size_t)P * 32); cnt = malloc(sizeof(int) * P);
        cap_alloc = P;
    }
    long M = 1;
    double N = 1, total = 0;
    uint16_t ch[128];
    memcpy(mk[0], mask0, 32);
    for (int k = k0; k < dmax && M > 0; k++) {
        int p = order[k], y = p / NN, x = p % NN, w = k - k0;
        int su = (y && !isfixed[p - NN]) ? pos2step_g[p - NN] - k0 : -1;
        int sl = (x && !isfixed[p - 1]) ? pos2step_g[p - 1] - k0 : -1;
        long E = 0;
        for (long i = 0; i < M; i++) {
            uint16_t *q = sq + (size_t)i * nfree;
            if (su >= 0) memcpy(B[p - NN], ori[q[su]].e, 4);
            if (sl >= 0) memcpy(B[p - 1], ori[q[sl]].e, 4);
            cnt[i] = children(k, mk[i], ch);
            E += cnt[i];
        }
        if (!E) break;
        N *= (double)E / M;
        total += N;
        long need = E < P ? E : P, left = E, M2 = 0;
        for (long i = 0; i < M && need > 0; i++) {
            if (!cnt[i]) continue;
            uint16_t *q = sq + (size_t)i * nfree;
            if (su >= 0) memcpy(B[p - NN], ori[q[su]].e, 4);
            if (sl >= 0) memcpy(B[p - 1], ori[q[sl]].e, 4);
            int c = children(k, mk[i], ch);
            for (int j = 0; j < c && need > 0; j++, left--) {
                if (need < left && runit() * left >= need) continue;
                need--;
                uint16_t *dq = sq2 + (size_t)M2 * nfree;
                memcpy(dq, q, (size_t)w * 2);
                dq[w] = ch[j];
                memcpy(mk2[M2], mk[i], 32);
                int pid = ori[ch[j]].pid;
                mk2[M2][pid >> 6] |= 1ULL << (pid & 63);
                M2++;
            }
        }
        { uint16_t *t = sq; sq = sq2; sq2 = t; }
        { uint64_t (*t)[4] = mk; mk = mk2; mk2 = t; }
        M = M2;
    }
    return total;
}

static int indepth(int d, const int *dl, int nd) {
    for (int i = 0; i < nd; i++) if (dl[i] == d) return 1;
    return 0;
}

int main(int argc, char **argv) {
    if (argc < 12) { fprintf(stderr, "usage\n"); return 2; }
    long mlive = atol(argv[9]), nfr = atol(argv[10]);
    uint64_t rcap = strtoull(argv[11], 0, 10);
    FILE *f = fopen(argv[1], "r");
    long P = atol(argv[2]);
    uint64_t seed = strtoull(argv[3], 0, 10);
    int dmax = atoi(argv[4]);
    long m = atol(argv[5]);
    uint64_t cap = strtoull(argv[6], 0, 10);
    int dl[64], nd = 0;
    for (char *t = strtok(argv[7], ","); t; t = strtok(0, ",")) dl[nd++] = atoi(t);
    long nprobe = atol(argv[8]);
    int n, F, S;
    if (!f || fscanf(f, "%d", &n) != 1 || n != NN) return 3;
    for (int i = 0; i < CELLS; i++) fscanf(f, "%d %d %d %d", &pieces[i][0], &pieces[i][1], &pieces[i][2], &pieces[i][3]);
    uint64_t fixmask[4] = {0, 0, 0, 0};
    fscanf(f, "%d", &F);
    for (int i = 0; i < F; i++) {
        int pos, pid, e[4];
        fscanf(f, "%d %d %d %d %d %d", &pos, &pid, &e[0], &e[1], &e[2], &e[3]);
        isfixed[pos] = 1;
        for (int s = 0; s < 4; s++) B[pos][s] = (uint8_t)e[s];
        fixmask[pid >> 6] |= 1ULL << (pid & 63);
    }
    fscanf(f, "%d", &S);
    havesol = S == CELLS;
    for (int i = 0; i < S; i++) {
        int pos, pid, e[4];
        fscanf(f, "%d %d %d %d %d %d", &pos, &pid, &e[0], &e[1], &e[2], &e[3]);
        solpid[pos] = pid;
        for (int s = 0; s < 4; s++) sole[pos][s] = e[s];
    }
    fclose(f);
    for (int p = 0; p < CELLS; p++) if (!isfixed[p]) order[nfree++] = p;
    for (int pid = 0; pid < CELLS; pid++) {
        if (USED(fixmask, pid)) continue;
        int seen[4][4], ns = 0;
        for (int r = 0; r < 4; r++) {
            int e[4];
            for (int s = 0; s < 4; s++) e[s] = pieces[pid][(s - r + 4) % 4];
            int dup = 0;
            for (int q = 0; q < ns; q++) if (!memcmp(seen[q], e, sizeof e)) dup = 1;
            if (dup) continue;
            memcpy(seen[ns++], e, sizeof e);
            ori[nori].pid = pid;
            for (int s = 0; s < 4; s++) ori[nori].e[s] = (uint8_t)e[s];
            nori++;
        }
    }
    for (int pass = 0; pass < 2; pass++) {
        if (pass) for (int a = 0; a < MAXC; a++) for (int b = 0; b < MAXC; b++) {
            lst[a][b] = malloc(sizeof(uint16_t) * (llen[a][b] + 1));
            llen[a][b] = 0;
        }
        for (int i = 0; i < nori; i++) {
            int a = ori[i].e[0], b = ori[i].e[3];
            if (pass) lst[a][b][llen[a][b]] = (uint16_t)i;
            llen[a][b]++;
        }
    }
    rseed(seed);
    if (dmax > nfree) dmax = nfree;

    for (int k = 0; k < nfree; k++) pos2step_g[order[k]] = k;
    /* SB j size: every sibling of the planted path (child of the planted prefix of depth j that
     * is not planted), with the estimated number of nodes strictly below it */
    if (havesol && argc > 12) {
        long PS = atol(argv[12]);
        uint64_t mask[4], m2[4];
        memcpy(mask, fixmask, 32);
        uint16_t ch[128];
        for (int k = 0; k < nfree; k++) {
            int c = children(k, mask, ch), p = order[k];
            for (int i = 0; i < c; i++) {
                const Ori *o = &ori[ch[i]];
                if (o->pid == solpid[p] && o->e[0] == sole[p][0] && o->e[1] == sole[p][1] && o->e[2] == sole[p][2] && o->e[3] == sole[p][3]) continue;
                memcpy(B[p], o->e, 4);
                memcpy(m2, mask, 32);
                m2[o->pid >> 6] |= 1ULL << (o->pid & 63);
                printf("SB %d %.6g\n", k, smc_size(k + 1, m2, PS, nfree));
            }
            for (int q = 0; q < 4; q++) B[p][q] = (uint8_t)sole[p][q];
            mask[solpid[p] >> 6] |= 1ULL << (solpid[p] & 63);
        }
        fflush(stdout);
        return 0;
    }

    /* planted path */
    if (havesol) {
        uint64_t mask[4];
        memcpy(mask, fixmask, 32);
        double l2w = 0;
        uint16_t ch[128];
        printf("PATH");
        static double l2wat[CELLS + 1];
        static uint64_t maskat[CELLS + 1][4];
        for (int k = 0; k < nfree; k++) {
            l2wat[k] = l2w;
            memcpy(maskat[k], mask, 32);
            int c = children(k, mask, ch), p = order[k], found = 0;
            for (int i = 0; i < c; i++) {
                const Ori *o = &ori[ch[i]];
                if (o->pid == solpid[p] && o->e[0] == sole[p][0] && o->e[1] == sole[p][1] && o->e[2] == sole[p][2] && o->e[3] == sole[p][3]) found = 1;
            }
            if (!found) { fprintf(stderr, "planted child missing at step %d\n", k); return 4; }
            printf(" %d", c);
            l2w += log2((double)c);
            for (int s = 0; s < 4; s++) B[p][s] = (uint8_t)sole[p][s];
            mask[solpid[p] >> 6] |= 1ULL << (solpid[p] & 63);
        }
        printf("\n");
        for (int i = 0; i < nd; i++) {
            int d = dl[i];
            if (d >= nfree) continue;
            /* B holds the whole planted board; dfs overwrites cells from step d on, restore after */
            uint64_t s = subtree(d, maskat[d], cap);
            printf("PS %d %.4f %llu %d %llu\n", d, l2wat[d], (unsigned long long)s, s >= cap, (unsigned long long)dfs_sols);
            for (int k = d; k < nfree; k++) for (int q = 0; q < 4; q++) B[order[k]][q] = (uint8_t)sole[order[k]][q];
        }
        if (mlive >= 0) {
            uint16_t path[CELLS];
            for (int k = 0; k < nfree; k++) {
                int p = order[k];
                path[k] = 65535;
                for (int i = 0; i < nori; i++)
                    if (ori[i].pid == solpid[p] && ori[i].e[0] == sole[p][0] && ori[i].e[1] == sole[p][1] && ori[i].e[2] == sole[p][2] && ori[i].e[3] == sole[p][3]) path[k] = (uint16_t)i;
            }
            for (int i = 0; i < nd; i++) {
                int nc, d = dl[i];
                double l2 = live_weight(path, d, fixmask, rcap, &nc);
                printf("PL %d %.4f %d\n", d, l2, nc);
            }
        }
        fflush(stdout);
    }

    /* randomised depth-first search to the first node of each depth */
    for (int i = 0; i < nd && nfr > 0; i++) {
        int d = dl[i];
        for (long t = 0; t < nfr; t++) {
            memcpy(dmask, fixmask, 32);
            fr_nodes = 0; fr_cap = cap;
            if (fr_rec(0, d) != 1) { printf("FR %d %llu 0 -1\n", d, (unsigned long long)fr_nodes); continue; }
            uint64_t cost = fr_nodes, mask[4];
            memcpy(mask, dmask, 32);
            uint64_t sz = subtree(d, mask, cap);
            printf("FR %d %llu %llu %d\n", d, (unsigned long long)cost, (unsigned long long)sz, sz >= cap);
        }
        fflush(stdout);
    }

    /* uniform-child probes from the root */
    if (nprobe > 0) {
        static double surv[CELLS + 1], len[CELLS + 1];
        uint16_t ch[128];
        for (long t = 0; t < nprobe; t++) {
            uint64_t mask[4];
            memcpy(mask, fixmask, 32);
            int k = 0;
            for (; k < dmax; k++) {
                int c = children(k, mask, ch);
                if (!c) break;
                const Ori *o = &ori[ch[rnext() % (uint64_t)c]];
                memcpy(B[order[k]], o->e, 4);
                mask[o->pid >> 6] |= 1ULL << (o->pid & 63);
            }
            for (int d = 0; d <= dmax; d++) { if (k >= d) surv[d] += 1; len[d] += k < d ? k : d; }
        }
        for (int d = 0; d <= dmax; d += 8) printf("MU %d %.6g %.4f\n", d, surv[d] / nprobe, len[d] / nprobe);
        fflush(stdout);
    }

    /* population SMC */
    if (P > 0) {
        size_t rec = (size_t)nfree * 2;
        uint16_t *seq = malloc(rec * P), *seq2 = malloc(rec * P);
        uint64_t (*mk)[4] = malloc(32 * P), (*mk2)[4] = malloc(32 * P);
        double *lw = malloc(8 * P), *lw2 = malloc(8 * P);
        int *cnt = malloc(sizeof(int) * P);
        long M = 1;
        memcpy(mk[0], fixmask, 32);
        lw[0] = 0;
        double log10N = 0;
        uint16_t ch[128];
        int *pos2step = malloc(sizeof(int) * CELLS);
        for (int k = 0; k < nfree; k++) pos2step[order[k]] = k;
        for (int k = 0; k < dmax && M > 0; k++) {
            int p = order[k], y = p / NN, x = p % NN;
            long E = 0, zero = 0;
            double s2 = 0;
            for (long i = 0; i < M; i++) {
                uint16_t *sq = seq + (size_t)i * nfree;
                if (y && !isfixed[p - NN]) memcpy(B[p - NN], ori[sq[pos2step[p - NN]]].e, 4);
                if (x && !isfixed[p - 1]) memcpy(B[p - 1], ori[sq[pos2step[p - 1]]].e, 4);
                int c = children(k, mk[i], ch);
                cnt[i] = c; E += c; zero += !c; s2 += (double)c * c;
            }
            double mean = (double)E / M, var = s2 / M - mean * mean;
            long keep = E < P ? E : P, need = keep, left = E, M2 = 0;
            for (long i = 0; i < M && need > 0; i++) {
                if (!cnt[i]) continue;
                uint16_t *sq = seq + (size_t)i * nfree;
                if (y && !isfixed[p - NN]) memcpy(B[p - NN], ori[sq[pos2step[p - NN]]].e, 4);
                if (x && !isfixed[p - 1]) memcpy(B[p - 1], ori[sq[pos2step[p - 1]]].e, 4);
                int c = children(k, mk[i], ch);
                for (int j = 0; j < c && need > 0; j++, left--) {
                    if (runit() * left >= need) continue;
                    need--;
                    uint16_t *dq = seq2 + (size_t)M2 * nfree;
                    memcpy(dq, sq, (size_t)k * 2);
                    dq[k] = ch[j];
                    memcpy(mk2[M2], mk[i], 32);
                    int pid = ori[ch[j]].pid;
                    mk2[M2][pid >> 6] |= 1ULL << (pid & 63);
                    lw2[M2] = lw[i] + log2((double)c);
                    M2++;
                }
            }
            if (E > 0) log10N += log10(mean); else log10N = -INFINITY;
            { uint16_t *t = seq; seq = seq2; seq2 = t; }
            { uint64_t (*t)[4] = mk; mk = mk2; mk2 = t; }
            { double *t = lw; lw = lw2; lw2 = t; }
            double ml = 0, mx = -1e300, si = 0;
            for (long i = 0; i < M2; i++) { ml += lw[i]; if (-lw[i] > mx) mx = -lw[i]; }
            for (long i = 0; i < M2; i++) si += exp2(-lw[i] - mx);
            printf("LV %d %ld %ld %.6f %.6f %.6f %.6f %.4f %.4f\n", k, M, E, (double)zero / M, mean, var, log10N,
                   M2 ? ml / M2 : 0.0, M2 ? log10N * log2(10.0) + mx + log2(si / M2) : 0.0);
            M = M2;
            if (indepth(k + 1, dl, nd) && M > 0) {
                long mm = m; /* the i-th listed depth gets m * 4^i members (deeper subtrees are smaller), at most 64000 */
                for (int q = 0; q < nd && dl[q] != k + 1; q++) mm *= 4;
                if (mm > 64000) mm = 64000;
                if (mm > M) mm = M;
                for (long t = 0; t < mm; t++) {
                    long i = (long)(rnext() % (uint64_t)M);
                    uint16_t *sq = seq + (size_t)i * nfree;
                    for (int q = 0; q <= k; q++) memcpy(B[order[q]], ori[sq[q]].e, 4);
                    uint64_t s = subtree(k + 1, mk[i], cap);
                    printf("US %d %.4f %llu %d\n", k + 1, lw[i], (unsigned long long)s, s >= cap);
                }
                for (long t = 0; t < mlive && t < M; t++) {
                    long i = (long)(rnext() % (uint64_t)M);
                    int nc;
                    double l2 = live_weight(seq + (size_t)i * nfree, k + 1, fixmask, rcap, &nc);
                    printf("UL %d %.4f %d %.4f\n", k + 1, l2, nc, lw[i]);
                }
            }
            fflush(stdout);
        }
    }
    return 0;
}
