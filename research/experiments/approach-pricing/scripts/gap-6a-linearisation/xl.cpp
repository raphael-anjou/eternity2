// XL (linearisation) at a degree bound D for a 0/1 system given as
//   - E linear equations  sum_{a in e} x_a = 1
//   - a list F of forbidden pairs  x_a * x_b = 0
//   - Boolean axioms x^2 = x (monomials are multilinear)
// over GF(PRIME) (compile with -DPRIME=...) or over GF(2) with bit rows (field argument "2").
//
// Plain XL: every equation times every monomial, total degree at most D, linearised.
// A monomial of degree <= D containing a forbidden pair is itself a row (pair times the rest),
// so those columns are removed and counted. What is left is the matrix A:
//   rows    (e, m), m a monomial of degree <= D-1 with no forbidden pair:
//           x^m * (sum_{a in e} x_a - 1) = (|e cap m| - 1) x^m + sum_{a in e \ m, m+a allowed} x^(m+a)
//   columns monomials of degree <= D with no forbidden pair.
// Reported: rank of the whole linearised system, nullity, and the variables x_a for which
// x_a - v lies in the row space (forced to v).
//
// Two equivalent eliminations (argument mode):
//   direct      reduced echelon form of the rows, columns ordered degree >= 2 first, then the
//               variables, then the constant; rows whose pivot is a variable are the linear consequences.
//   transposed  reduced echelon form of the COLUMNS of degree >= 2 (vectors indexed by rows); its
//               orthogonal complement K is the set of row combinations with no term of degree >= 2;
//               the linear consequences are K times the columns of degree <= 1.
//   auto        whichever has the smaller dimension.
//
// Usage: xl system.txt D field(2|p) mode(direct|transposed|auto) [time_limit_s] [mem_limit_gb]
// system.txt: "n E", then n solution values (unused here, kept for the driver), then E lines
// "k v1 .. vk", then "nF" and nF lines "a b".
#include <cstdio>
#include <cstdlib>
#include <cstdint>
#include <cstring>
#include <ctime>
#include <vector>
#include <unordered_map>
#include <algorithm>
#include <string>
typedef uint32_t u32; typedef uint64_t u64;
#ifndef PRIME
#define PRIME 2147483647u
#endif
static const u32 P = PRIME;
static inline u32 red(u64 t) {
#if PRIME == 2147483647u
    t = (t & P) + (t >> 31); t = (t & P) + (t >> 31); return (u32)(t >= P ? t - P : t);
#else
    return (u32)(t % P);
#endif
}
static u32 inv_mod(u32 a) { u64 r = 1, b = a; u32 e = P - 2; while (e) { if (e & 1) r = r * b % P; b = b * b % P; e >>= 1; } return (u32)r; }

static double t0;
static double now() { return (double)clock() / CLOCKS_PER_SEC; }
static double TLIM = 840, MLIM = 7.5e9;
static std::string abort_reason;
typedef std::vector<std::pair<int, u32>> Sparse;

// ---------- reduced echelon basis over GF(P), pivot = first nonzero coordinate ----------
struct EngP {
    int dim = 0, rank = 0; std::vector<u32*> rows; std::vector<int> pivcol, pivof; u32* w = nullptr;
    void init(int d) { dim = d; pivof.assign(d, -1); w = (u32*)calloc(d > 0 ? d : 1, 4); }
    static void axpy(u32* dst, const u32* src, u32 c, int lo, int hi) { // dst -= c * src
        u64 cc = P - c;
        for (int j = lo; j < hi; j++) dst[j] = red(dst[j] + cc * src[j]);
    }
    u32 get(int i, int j) const { return rows[i][j]; }
    double bytes() const { return (double)(rank + 1) * dim * 4; }
    double nnz() const { double c = 0; for (auto r : rows) for (int j = 0; j < dim; j++) c += r[j] != 0; return c; }
    bool insert(const Sparse& v) {
        if (v.empty()) return false;
        int lo = dim;
        for (auto& e : v) { w[e.first] = red((u64)w[e.first] + e.second); if (e.first < lo) lo = e.first; }
        for (auto& e : v) { int j = e.first, i = pivof[j]; if (i >= 0 && w[j]) axpy(w, rows[i], w[j], j, dim); }
        int j = lo; while (j < dim && !w[j]) j++;
        if (j == dim) return false;
        u32 iv = inv_mod(w[j]);
        if (iv != 1) for (int k = j; k < dim; k++) w[k] = red((u64)w[k] * iv);
        for (int i = 0; i < rank; i++) { u32 c = rows[i][j]; if (c) axpy(rows[i], w, c, j, dim); }
        rows.push_back(w); pivcol.push_back(j); pivof[j] = rank++;
        w = (u32*)calloc(dim, 4);
        if (!w) { abort_reason = "calloc failed"; return true; }
        return true;
    }
};
// ---------- the same over GF(2) with bit rows ----------
struct Eng2 {
    int dim = 0, rank = 0, W = 0; std::vector<u64*> rows; std::vector<int> pivcol, pivof; u64* w = nullptr;
    void init(int d) { dim = d; W = (d + 63) / 64; pivof.assign(d, -1); w = (u64*)calloc(W > 0 ? W : 1, 8); }
    u32 get(int i, int j) const { return (rows[i][j >> 6] >> (j & 63)) & 1; }
    double bytes() const { return (double)(rank + 1) * W * 8; }
    double nnz() const { double c = 0; for (auto r : rows) for (int k = 0; k < W; k++) c += __builtin_popcountll(r[k]); return c; }
    bool insert(const Sparse& v) {
        if (v.empty()) return false;
        int lo = dim;
        for (auto& e : v) { if (e.second & 1) w[e.first >> 6] ^= 1ull << (e.first & 63); if (e.first < lo) lo = e.first; }
        for (auto& e : v) {
            int j = e.first, i = pivof[j];
            if (i >= 0 && ((w[j >> 6] >> (j & 63)) & 1)) { const u64* r = rows[i]; for (int k = j >> 6; k < W; k++) w[k] ^= r[k]; }
        }
        int k = lo >> 6; while (k < W && !w[k]) k++;
        if (k == W) return false;
        int j = k * 64 + __builtin_ctzll(w[k]);
        u64 bit = 1ull << (j & 63);
        for (int i = 0; i < rank; i++) { u64* r = rows[i]; if (r[k] & bit) for (int q = k; q < W; q++) r[q] ^= w[q]; }
        rows.push_back(w); pivcol.push_back(j); pivof[j] = rank++;
        w = (u64*)calloc(W, 8);
        if (!w) { abort_reason = "calloc failed"; return true; }
        return true;
    }
};

// ---------- sparse echelon basis (not reduced), any PRIME: used by mode "sparse" ----------
// Each incoming row is fully reduced against the pivots found so far (columns handled in
// increasing order through a heap, so cascades are followed); pivot = first nonzero column.
// Rows are stored sparse in one arena. The basis is an echelon form with distinct leading columns.
struct EngS {
    int dim = 0, rank = 0; std::vector<u64> start; std::vector<u32> cols, vals; std::vector<int> pivcol, pivof;
    std::vector<u32> w, heap; std::vector<uint8_t> inheap; double work = 0;
    void init(int d) { dim = d; pivof.assign(d, -1); w.assign(d, 0); inheap.assign(d, 0); start.push_back(0); }
    double bytes() const { return (double)cols.capacity() * 4 + (double)vals.capacity() * 4 + (double)dim * 9 + (double)start.capacity() * 8; }
    double nnz() const { return (double)cols.size(); }
    inline void touch(u32 j) { if (!inheap[j]) { inheap[j] = 1; heap.push_back(j); std::push_heap(heap.begin(), heap.end(), std::greater<u32>()); } }
    bool insert(const Sparse& v) {
        for (auto& e : v) { w[e.first] = red((u64)w[e.first] + e.second); touch((u32)e.first); }
        size_t base = cols.size();
        while (!heap.empty()) {
            std::pop_heap(heap.begin(), heap.end(), std::greater<u32>()); u32 j = heap.back(); heap.pop_back(); inheap[j] = 0;
            u32 c = w[j]; if (!c) continue; w[j] = 0;
            int i = pivof[j];
            if (i < 0) { cols.push_back(j); if (P != 2) vals.push_back(c); continue; }
            u64 cc = P - c;
            for (u64 q = start[i] + 1; q < start[i + 1]; q++) { u32 col = cols[q]; u32 val = (P == 2) ? 1 : vals[q]; w[col] = red(w[col] + cc * val); touch(col); }
            work += (double)(start[i + 1] - start[i]);
        }
        if (cols.size() == base) return false;
        if (P != 2) { u32 iv = inv_mod(vals[base]); if (iv != 1) for (size_t q = base; q < vals.size(); q++) vals[q] = red((u64)vals[q] * iv); }
        pivcol.push_back((int)cols[base]); pivof[cols[base]] = rank++; start.push_back(cols.size());
        return true;
    }
    // the arena is append only, so the basis can be cut back to an earlier rank
    void rollback(int r) {
        for (int i = rank - 1; i >= r; i--) pivof[pivcol[i]] = -1;
        pivcol.resize(r); cols.resize(start[r]); if (P != 2) vals.resize(start[r]); start.resize(r + 1); rank = r;
    }
};

// ---------- the system ----------
static int n, E, D;
static std::vector<std::vector<int>> eqs, eqs_of;
static std::vector<std::vector<u64>> eqmask, adj; // bitsets over variables
static int NW;
static inline bool tst(const std::vector<u64>& b, int v) { return (b[v >> 6] >> (v & 63)) & 1; }
// monomials: id order = by degree, then lexicographic; id 0 is the constant, ids 1..n the variables
static std::vector<uint16_t> mv; static std::vector<uint8_t> mdeg; static std::vector<int> deg_start;
static std::unordered_map<u64, int> mid;
static std::vector<int> cellof; static int side = 0;
static int NM, Nlow, N2; static bool coldesc = false, rowasc = false;
static int KB = 10; // bits per variable in a monomial key
static u64 key_of(const uint16_t* v, int d) { u64 k = 0; for (int i = 0; i < d; i++) k = (k << KB) | (u64)(v[i] + 1); return k; }
// column positions: monomials of degree >= 2 first (by increasing degree, or by decreasing degree
// when coldesc), then the n variables, then the constant
static int pos_of(int id) {
    if (id == 0) return N2 + n;
    if (id <= n) return N2 + id - 1;
    if (!coldesc) return id - 1 - n;
    int d = mdeg[id]; return (NM - deg_start[d + 1]) + (id - deg_start[d]);
}
static void enumerate() {
    mv.assign(D > 0 ? D : 1, 0); mdeg.assign(1, 0); mid[0] = 0; deg_start.assign(1, 0);
    int begin = 0, end = 1;
    std::vector<u64> allow(NW);
    for (int d = 0; d < D; d++) {
        deg_start.push_back(end);
        for (int id = begin; id < end; id++) {
            uint16_t cur[16]; for (int i = 0; i < d; i++) cur[i] = mv[(size_t)id * D + i];
            for (int q = 0; q < NW; q++) allow[q] = ~0ull;
            for (int i = 0; i < d; i++) for (int q = 0; q < NW; q++) allow[q] &= ~adj[cur[i]][q];
            for (int w = d ? cur[d - 1] + 1 : 0; w < n; w++) if (tst(allow, w)) {
                cur[d] = (uint16_t)w;
                int nid = (int)mdeg.size(); mdeg.push_back((uint8_t)(d + 1));
                size_t off = mv.size(); mv.resize(off + D, 0);
                for (int i = 0; i <= d; i++) mv[off + i] = cur[i];
                mid[key_of(cur, d + 1)] = nid;
            }
        }
        begin = end; end = (int)mdeg.size();
    }
    deg_start.push_back(end);
    NM = end; Nlow = deg_start[D]; N2 = NM - 1 - n;
}
// row (e, m) as a sparse vector over column positions
static void row_of(int e, int m, Sparse& out) {
    out.clear();
    int d = mdeg[m]; const uint16_t* v = &mv[(size_t)m * D];
    int cnt = 0; for (int i = 0; i < d; i++) cnt += tst(eqmask[e], v[i]);
    u32 c = (u32)(((long long)cnt - 1 + P) % P);
    if (c) out.push_back({pos_of(m), c});
    uint16_t cur[16];
    for (int a : eqs[e]) {
        bool ok = true;
        for (int i = 0; i < d && ok; i++) if (v[i] == a || tst(adj[a], v[i])) ok = false;
        if (!ok) continue;
        int k = 0, i = 0; while (i < d && v[i] < a) cur[k++] = v[i++]; cur[k++] = (uint16_t)a; while (i < d) cur[k++] = v[i++];
        out.push_back({pos_of(mid.at(key_of(cur, d + 1))), 1 % P});
    }
}
// column of monomial M as a sparse vector over row indices e * Nlow + m
static void col_of(int M, Sparse& out) {
    out.clear();
    int d = mdeg[M]; const uint16_t* v = &mv[(size_t)M * D];
    uint16_t cur[16];
    for (int i = 0; i < d; i++) {
        int k = 0; for (int q = 0; q < d; q++) if (q != i) cur[k++] = v[q];
        int m = mid.at(key_of(cur, d - 1));
        for (int e : eqs_of[v[i]]) out.push_back({e * Nlow + m, 1 % P});
    }
    if (d <= D - 1) for (int e = 0; e < E; e++) {
        int cnt = 0; for (int i = 0; i < d; i++) cnt += tst(eqmask[e], v[i]);
        u32 c = (u32)(((long long)cnt - 1 + P) % P);
        if (c) out.push_back({e * Nlow + M, c});
    }
}
static bool guard(double bytes) {
    if (now() - t0 > TLIM) { abort_reason = "time limit"; return false; }
    if (bytes > MLIM) { abort_reason = "memory limit"; return false; }
    return abort_reason.empty();
}
// read the linear consequences off a reduced echelon basis whose last n+1 coordinates
// (from position base) are the variables and then the constant
template <class Eng> static void read_forced(const Eng& g, int base, std::vector<int>& fv, std::vector<long long>& fval, bool& incons, int& nlin) {
    nlin = 0;
    for (int i = 0; i < g.rank; i++) {
        int pc = g.pivcol[i]; if (pc < base) continue;
        nlin++;
        if (pc == base + n) { incons = true; continue; }
        bool alone = true;
        for (int j = pc + 1; j < base + n && alone; j++) if (g.get(i, j)) alone = false;
        if (alone) { fv.push_back(pc - base); fval.push_back((P - g.get(i, base + n)) % P); }
    }
}
template <class Eng> static void run(const char* fieldname, std::string mode) {
    enumerate();
    // counts
    double nfull = 0, c = 1; for (int k = 0; k <= D; k++) { nfull += c; c = c * (n - k) / (k + 1); }
    long long R = (long long)E * Nlow;
    if (mode == "auto") mode = (R < NM) ? "transposed" : "direct";
    long long nzrows = 0; Sparse sp;
    for (int e = 0; e < E; e++) for (int m = 0; m < Nlow; m++) { row_of(e, m, sp); if (!sp.empty()) nzrows++; }
    long long rankA = -1, rank2 = -1; std::vector<int> fv; std::vector<long long> fval; bool incons = false; int nlin = 0; double peak = 0, bnnz = 0;
    std::string extra; bool partial = false;
    if (mode == "local") {
        // Rows whose multiplier has degree D-1 are added by classes of increasing spread (largest
        // Manhattan distance between two cells of the multiplier). After each class the rows of all
        // lower multipliers are added, the linear consequences are counted, and unless all n variables
        // are forced (or the class was the last) those lower rows are removed again. A subset of the
        // XL rows spans a subspace of the XL row space, so "determined" at an early class is sound;
        // the rank reported is then the rank of the rows used, not of the whole system.
        coldesc = true;
        if (side == 0) { fprintf(stderr, "mode local needs the cell section in the system file\n"); exit(2); }
        int top0 = deg_start[D - 1];
        std::vector<std::pair<int, int>> top; int maxsp = 0;
        for (int m = top0; m < Nlow; m++) {
            int d = mdeg[m], sp2 = 0; const uint16_t* v = &mv[(size_t)m * D];
            for (int i = 0; i < d; i++) for (int j = i + 1; j < d; j++) {
                int a = cellof[v[i]], b = cellof[v[j]]; int dist = abs(a / side - b / side) + abs(a % side - b % side); if (dist > sp2) sp2 = dist;
            }
            top.push_back({sp2, m}); if (sp2 > maxsp) maxsp = sp2;
        }
        std::stable_sort(top.begin(), top.end());
        EngS g; g.init(NM); size_t k = 0; long long used = 0; extra = "\"classes\":[";
        for (int cls = 0; cls <= maxsp && abort_reason.empty(); cls++) {
            size_t k0 = k;
            for (; k < top.size() && top[k].first == cls && abort_reason.empty(); k++) {
                for (int e = 0; e < E; e++) { row_of(e, top[k].second, sp); g.insert(sp); }
                if (((k - k0) & 0x3ff) == 0 && !guard(g.bytes())) break;
            }
            if (!abort_reason.empty()) break;
            used += (long long)(k - k0) * E;
            int r0 = g.rank; double nnz0 = g.nnz();
            for (int m = top0 - 1; m >= 0 && abort_reason.empty(); m--) { for (int e = 0; e < E; e++) { row_of(e, m, sp); g.insert(sp); } if (!guard(g.bytes())) break; }
            if (!abort_reason.empty()) break;
            int lin = 0; for (int i = r0; i < g.rank; i++) if (g.pivcol[i] >= N2) lin++;
            for (int i = 0; i < r0; i++) if (g.pivcol[i] >= N2) lin++;
            char buf[200]; snprintf(buf, sizeof buf, "%s[%d,%zu,%d,%.0f,%.2f]", cls ? "," : "", cls, k - k0, lin, g.nnz(), now() - t0); extra += buf;
            fprintf(stderr, "  class %d: %zu multipliers, top rank %d nnz %.0f, linear rank %d of %d, t=%.1fs\n", cls, k - k0, r0, nnz0, lin, n, now() - t0);
            bool last = (cls == maxsp);
            if (lin >= n || last) {
                partial = !last; rankA = g.rank; peak = g.bytes(); bnnz = g.nnz();
                EngP h; h.init(n + 1);
                for (int i = 0; i < g.rank; i++) if (g.pivcol[i] >= N2) {
                    sp.clear(); for (u64 q = g.start[i]; q < g.start[i + 1]; q++) sp.push_back({(int)g.cols[q] - N2, (P == 2) ? 1u : g.vals[q]});
                    h.insert(sp);
                }
                read_forced(h, 0, fv, fval, incons, nlin);
                snprintf(buf, sizeof buf, "],\"class_reached\":%d,\"class_max\":%d,\"rows_used\":%lld,", cls, maxsp, used + (long long)top0 * E); extra += buf;
                break;
            }
            g.rollback(r0);
        }
        if (!abort_reason.empty()) { extra += "],"; peak = g.bytes(); bnnz = g.nnz(); fprintf(stderr, "aborted at rank %d nnz %.0f\n", g.rank, g.nnz()); }
    } else if (mode.rfind("sparse", 0) == 0) {
        coldesc = mode.find("-cdesc") != std::string::npos; rowasc = mode.find("-rasc") != std::string::npos;
        EngS g; g.init(NM); long long done = 0;
        for (int mm = 0; mm < Nlow && abort_reason.empty(); mm++) {
            int m = rowasc ? mm : Nlow - 1 - mm;
            for (int e = 0; e < E; e++) { row_of(e, m, sp); g.insert(sp); }
            if (!guard(g.bytes())) break;
            if ((++done & 0x3fff) == 0) fprintf(stderr, "  multipliers %lld/%d rank %d nnz %.0f work %.3g t=%.0fs\n", done, Nlow, g.rank, g.nnz(), g.work, now() - t0);
        }
        peak = g.bytes(); bnnz = g.nnz();
        if (abort_reason.empty()) {
            rankA = g.rank;
            EngP h; h.init(n + 1);
            for (int i = 0; i < g.rank; i++) if (g.pivcol[i] >= N2) {
                sp.clear(); for (u64 q = g.start[i]; q < g.start[i + 1]; q++) sp.push_back({(int)g.cols[q] - N2, (P == 2) ? 1u : g.vals[q]});
                h.insert(sp);
            }
            read_forced(h, 0, fv, fval, incons, nlin);
        } else fprintf(stderr, "aborted at rank %d nnz %.0f\n", g.rank, g.nnz());
    } else if (mode == "direct") {
        Eng g; g.init(NM);
        for (int m = Nlow - 1; m >= 0 && abort_reason.empty(); m--) for (int e = 0; e < E; e++) {
            row_of(e, m, sp); g.insert(sp);
            if (!guard(g.bytes())) break;
        }
        peak = g.bytes(); bnnz = g.nnz();
        if (abort_reason.empty()) { rankA = g.rank; read_forced(g, N2, fv, fval, incons, nlin); }
        else fprintf(stderr, "aborted at rank %d\n", g.rank);
    } else {
        Eng g; g.init((int)R);
        long long done = 0;
        for (int M = NM - 1; M > n && abort_reason.empty(); M--) {
            col_of(M, sp); g.insert(sp); done++;
            if (!guard(g.bytes())) break;
            if ((done & 0xffff) == 0) fprintf(stderr, "  cols %lld/%d rank %d t=%.0fs\n", done, N2, g.rank, now() - t0);
        }
        peak = g.bytes(); bnnz = g.nnz();
        if (abort_reason.empty()) {
            rank2 = g.rank;
            // L[j][c] for the free coordinates j: (e_j - sum_i N[i][j] e_{p_i}) . column c
            std::vector<int> freeidx; for (int j = 0; j < (int)R; j++) if (g.pivof[j] < 0) freeidx.push_back(j);
            std::vector<int> slot(R, -1); for (size_t s = 0; s < freeidx.size(); s++) slot[freeidx[s]] = (int)s;
            std::vector<u32> L(freeidx.size() * (size_t)(n + 1), 0);
            for (int cidx = 0; cidx <= n; cidx++) {
                int M = (cidx == n) ? 0 : cidx + 1;
                col_of(M, sp);
                for (auto& en : sp) {
                    int i = g.pivof[en.first];
                    if (i < 0) { u32& x = L[(size_t)slot[en.first] * (n + 1) + cidx]; x = red((u64)x + en.second); }
                    else for (int j = g.pivcol[i] + 1; j < (int)R; j++) {
                        u32 a = g.get(i, j); if (!a) continue;
                        u32& x = L[(size_t)slot[j] * (n + 1) + cidx]; x = red((u64)x + (u64)(P - red((u64)a * en.second)));
                    }
                }
            }
            EngP h; h.init(n + 1);
            for (size_t s = 0; s < freeidx.size(); s++) {
                sp.clear(); for (int cidx = 0; cidx <= n; cidx++) { u32 x = L[s * (n + 1) + cidx]; if (x) sp.push_back({cidx, x}); }
                h.insert(sp);
            }
            rankA = rank2 + h.rank; read_forced(h, 0, fv, fval, incons, nlin);
        } else fprintf(stderr, "aborted at rank %d\n", g.rank);
    }
    printf("{\"n\":%d,\"E\":%d,\"D\":%d,\"field\":\"%s\",\"mode\":\"%s\",\"monomials_all\":%.0f,\"monomials_nonzero\":%d,"
           "\"rows_nonzero\":%lld,\"row_index_space\":%lld,", n, E, D, fieldname, mode.c_str(), nfull, NM, nzrows, R);
    printf("%s", extra.c_str());
    if (rankA >= 0 && partial) {
        printf("\"partial\":true,\"rank_rows_used\":%lld,\"linear_consequences\":%d,\"inconsistent\":%s,\"forced\":[", rankA, nlin, incons ? "true" : "false");
        for (size_t i = 0; i < fv.size(); i++) printf("%s[%d,%lld]", i ? "," : "", fv[i], fval[i]);
        printf("],");
    } else if (rankA >= 0) {
        printf("\"rank_A\":%lld,\"rank_deg2plus\":%lld,\"rank_full\":%.0f,\"nullity\":%lld,\"linear_consequences\":%d,\"inconsistent\":%s,\"forced\":[",
               rankA, rank2, nfull - NM + rankA, NM - rankA, nlin, incons ? "true" : "false");
        for (size_t i = 0; i < fv.size(); i++) printf("%s[%d,%lld]", i ? "," : "", fv[i], fval[i]);
        printf("],");
    } else printf("\"aborted\":\"%s\",", abort_reason.c_str());
    printf("\"basis_nonzeros\":%.0f,\"peak_bytes\":%.0f,\"seconds\":%.2f}\n", bnnz, peak, now() - t0);
}
int main(int argc, char** argv) {
    if (argc < 5) { fprintf(stderr, "usage: xl system.txt D field(2|p) mode [tlim] [mem_gb]\n"); return 2; }
    t0 = now();
    D = atoi(argv[2]); std::string field = argv[3], mode = argv[4];
    if (argc > 5) TLIM = atof(argv[5]);
    if (argc > 6) MLIM = atof(argv[6]) * 1e9;

    FILE* f = fopen(argv[1], "r"); if (!f) { perror("open"); return 2; }
    if (fscanf(f, "%d %d", &n, &E) != 2) return 2;
    if (n > 1000) { fprintf(stderr, "n too large\n"); return 2; }
    NW = (n + 63) / 64;
    KB = 1; while ((1 << KB) < n + 2) KB++;
    if (D < 1 || D > 16 || D * KB > 64) { fprintf(stderr, "D too large for this n\n"); return 2; }
    for (int i = 0, x; i < n; i++) if (fscanf(f, "%d", &x) != 1) return 2;
    eqs.resize(E); eqs_of.resize(n); eqmask.assign(E, std::vector<u64>(NW, 0)); adj.assign(n, std::vector<u64>(NW, 0));
    for (int e = 0, k; e < E; e++) {
        if (fscanf(f, "%d", &k) != 1) return 2;
        for (int i = 0, v; i < k; i++) { if (fscanf(f, "%d", &v) != 1) return 2; eqs[e].push_back(v); eqs_of[v].push_back(e); eqmask[e][v >> 6] |= 1ull << (v & 63); }
    }
    int nF; if (fscanf(f, "%d", &nF) != 1) return 2;
    for (int i = 0, a, b; i < nF; i++) { if (fscanf(f, "%d %d", &a, &b) != 2) return 2; adj[a][b >> 6] |= 1ull << (b & 63); adj[b][a >> 6] |= 1ull << (a & 63); }
    int sd; if (fscanf(f, "%d", &sd) == 1) { side = sd; cellof.resize(n); for (int i = 0; i < n; i++) if (fscanf(f, "%d", &cellof[i]) != 1) return 2; }
    fclose(f);
    if (mode.rfind("sparse", 0) == 0 || mode == "local") { char nm[64]; snprintf(nm, sizeof nm, "GF(%u)", P); run<EngP>(nm, mode); return 0; }
    if (field == "2" && P != 2) { fprintf(stderr, "bit rows need the -DPRIME=2 build\n"); return 2; }
    if (field == "2") run<Eng2>("GF(2) bit rows", mode);
    else { char nm[64]; snprintf(nm, sizeof nm, "GF(%u)", P); run<EngP>(nm, mode); }
    return 0;
}
