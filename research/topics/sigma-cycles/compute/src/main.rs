//! Sigma-cycles, all-pairs, over the bundled boards.
//!
//! Two complete boards of the same puzzle place the same pieces in (generally)
//! different cells. The position permutation sigma carries board A's arrangement
//! to board B: for the piece sitting at cell i in A, sigma[i] is the cell that
//! same piece occupies in B. sigma decomposes into disjoint cycles; applying a
//! whole cycle moves every piece in it one step toward B. The research question
//! is whether any *proper prefix* of a large cycle (a partial application) can
//! ever raise the matched-edge score above board A's. On the record pairs the
//! page previously quoted by hand (154 / 80 / 85-cell cycles), no prefix helped.
//! This crate turns that three-pair prose into a committed population result:
//! every ordered pair of same-piece-set bundled boards, every large cycle, every
//! proper prefix, scored.
//!
//! Piece identity is taken from the board itself, not from an external id list:
//! a piece is the rotation-invariant signature of its four edge colours (the
//! lexicographically smallest of its four URDL rotations). Two boards "share a
//! piece set" iff their multisets of piece signatures are identical. Under the
//! bucas letter convention the boards fall into exactly two such sets (the plain
//! `board_edges` boards, and the `motifs_order=jblackwood` boards, which use a
//! relabelled colour alphabet); each is analysed on its own.
//!
//! The partial-application scoring mirrors the site's live SigmaCycleLab exactly:
//! applying the first `k` edges of a cycle sets cells `cycle[0..k]` to the piece
//! board B wants there, leaving the rest of board A untouched, so a fully applied
//! cycle reproduces B on those cells. Scores are matched interior edges (the same
//! convention research/topics/record-boards verifies). Exact and deterministic.

use eternity2_engine::official::official_puzzle;

const BOARDS_JSON: &str = include_str!("../boards.json");

const W: usize = 16;
const H: usize = 16;
const N: usize = W * H;

/// Only cycles at least this long get a full prefix scan reported (short cycles
/// are transpositions and 3-cycles whose behaviour is not the "giant loop" the
/// page is about). Every cycle length is still recorded in the structure summary.
const LARGE_CYCLE_MIN: usize = 10;

// --- minimal JSON reading, tailored to the machine-generated boards.json -----

/// Pull the next `"key": "value"` string starting at or after `from`.
fn next_string(s: &str, key: &str, from: usize) -> Option<(String, usize)> {
    let needle = format!("\"{key}\"");
    let k = s[from..].find(&needle)? + from;
    let colon = s[k..].find(':')? + k;
    let q1 = s[colon..].find('"')? + colon + 1;
    let q2 = s[q1..].find('"')? + q1;
    Some((s[q1..q2].to_string(), q2))
}

/// Pull the next `"key": <number|null>` starting at or after `from`.
/// Returns None value for null; the second element is the scan cursor.
fn next_scalar(s: &str, key: &str, from: usize) -> Option<(Option<i64>, usize)> {
    let needle = format!("\"{key}\"");
    let k = s[from..].find(&needle)? + from;
    let colon = s[k..].find(':')? + k;
    let rest = &s[colon + 1..];
    let end = rest.find([',', '}', '\n']).unwrap_or(rest.len());
    let tok = rest[..end].trim();
    let val = if tok == "null" { None } else { tok.parse::<i64>().ok() };
    Some((val, colon + 1 + end))
}

// --- board model -------------------------------------------------------------

/// A board: 256 cells, each a URDL colour tuple (bucas letters mapped to bytes).
#[derive(Clone)]
struct Board {
    id: String,
    score: Option<i64>,
    source: String,
    cells: Vec<[u8; 4]>,
}

/// Rotation-invariant signature of a piece: the lexicographically smallest of its
/// four URDL rotations. Two cells hold the same piece iff their signatures match.
fn piece_sig(c: &[u8; 4]) -> [u8; 4] {
    let mut best = *c;
    for s in 1..4 {
        let r = [c[s % 4], c[(s + 1) % 4], c[(s + 2) % 4], c[(s + 3) % 4]];
        if r < best {
            best = r;
        }
    }
    best
}

impl Board {
    fn parse(edges: &str) -> Option<Vec<[u8; 4]>> {
        let b = edges.as_bytes();
        if b.len() != N * 4 {
            return None;
        }
        let mut cells = Vec::with_capacity(N);
        for i in 0..N {
            cells.push([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]]);
        }
        Some(cells)
    }

    /// Complete solution: no cell is the all-grey ("aaaa") empty marker, and all
    /// 256 pieces are distinct (a genuine full E2 arrangement, not a partial).
    fn is_complete_solution(&self) -> bool {
        if self.cells.iter().any(|c| *c == [b'a'; 4]) {
            return false;
        }
        let mut sigs: Vec<[u8; 4]> = self.cells.iter().map(piece_sig).collect();
        sigs.sort_unstable();
        sigs.windows(2).all(|w| w[0] != w[1])
    }

    /// Sorted piece-signature multiset, the key for "shares a piece set".
    fn piece_multiset(&self) -> Vec<[u8; 4]> {
        let mut sigs: Vec<[u8; 4]> = self.cells.iter().map(piece_sig).collect();
        sigs.sort_unstable();
        sigs
    }

    /// Matched interior edges of a raw cell grid (URDL: 0=up,1=right,2=down,3=left).
    fn score_cells(cells: &[[u8; 4]]) -> u32 {
        let mut matched = 0u32;
        for r in 0..H {
            for c in 0..W {
                let i = r * W + c;
                if c + 1 < W && cells[i][1] == cells[i + 1][3] {
                    matched += 1;
                }
                if r + 1 < H && cells[i][2] == cells[i + W][0] {
                    matched += 1;
                }
            }
        }
        matched
    }

    fn score(&self) -> u32 {
        Self::score_cells(&self.cells)
    }
}

// --- sigma + cycles ----------------------------------------------------------

/// sigma[i] = cell in B holding the piece that sits at cell i in A. Both boards
/// share a piece set (checked by the caller), so every piece has a unique home.
fn sigma_permutation(a: &Board, b: &Board) -> Vec<usize> {
    let mut home: std::collections::HashMap<[u8; 4], usize> = std::collections::HashMap::new();
    for (pos, c) in b.cells.iter().enumerate() {
        home.insert(piece_sig(c), pos);
    }
    a.cells
        .iter()
        .map(|c| *home.get(&piece_sig(c)).expect("shared piece set"))
        .collect()
}

/// Disjoint non-trivial cycles of a permutation (fixed points dropped),
/// longest-first, matching the site's decomposeCycles ordering.
fn decompose_cycles(sigma: &[usize]) -> Vec<Vec<usize>> {
    let mut seen = vec![false; sigma.len()];
    let mut cycles: Vec<Vec<usize>> = Vec::new();
    for i in 0..sigma.len() {
        if seen[i] || sigma[i] == i {
            seen[i] = true;
            continue;
        }
        let mut cyc = Vec::new();
        let mut j = i;
        while !seen[j] {
            seen[j] = true;
            cyc.push(j);
            j = sigma[j];
        }
        if cyc.len() > 1 {
            cycles.push(cyc);
        }
    }
    cycles.sort_by(|x, y| y.len().cmp(&x.len()));
    cycles
}

/// Score board A after applying the first `k` edges of `cycle` toward B: cells
/// cycle[0..k] take the piece B places there. Mirrors the site's applyCycles for
/// a single selected cycle at fraction k/len (rounded to whole edges).
fn score_prefix(a: &Board, b: &Board, cycle: &[usize], k: usize) -> u32 {
    let mut cells = a.cells.clone();
    for &cell in &cycle[..k] {
        cells[cell] = b.cells[cell];
    }
    Board::score_cells(&cells)
}

// --- JSON writing helpers ----------------------------------------------------

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn main() {
    let max = official_puzzle().max_score();

    // Parse every board in boards.json.
    let mut boards: Vec<Board> = Vec::new();
    let mut pos = 0usize;
    while let Some((id, p1)) = next_string(BOARDS_JSON, "id", pos) {
        let (score, p2) = next_scalar(BOARDS_JSON, "score", p1).unwrap();
        let (edges, p3) = next_string(BOARDS_JSON, "edges", p2).unwrap();
        let (source, p4) = next_string(BOARDS_JSON, "source", p3).unwrap();
        if let Some(cells) = Board::parse(&edges) {
            boards.push(Board { id, score, source, cells });
        }
        pos = p4;
    }
    let parsed = boards.len();

    // Keep complete solutions only.
    let solutions: Vec<Board> = boards.into_iter().filter(Board::is_complete_solution).collect();
    let dropped = parsed - solutions.len();

    // Group by shared piece set.
    let mut group_of: Vec<usize> = vec![0; solutions.len()];
    let mut keys: Vec<Vec<[u8; 4]>> = Vec::new();
    for (bi, b) in solutions.iter().enumerate() {
        let ms = b.piece_multiset();
        let g = keys.iter().position(|k| *k == ms).unwrap_or_else(|| {
            keys.push(ms);
            keys.len() - 1
        });
        group_of[bi] = g;
    }
    let group_count = keys.len();

    // Per-group ordered pairs.
    #[derive(Default)]
    struct PairStat {
        pairs: u64,
        large_cycles: u64,
        prefixes_tested: u64,
        prefixes_worse: u64,
        prefixes_ge: u64, // >= score(A) (a violation of "every prefix scores strictly worse")
        violating_pairs: u64,
    }

    // Collect JSON fragments.
    let mut group_json: Vec<String> = Vec::new();
    let mut pair_json: Vec<String> = Vec::new();

    for g in 0..group_count {
        let members: Vec<usize> = (0..solutions.len()).filter(|&i| group_of[i] == g).collect();
        let mut stat = PairStat::default();

        for &ai in &members {
            for &bi in &members {
                if ai == bi {
                    continue;
                }
                let a = &solutions[ai];
                let b = &solutions[bi];
                let score_a = a.score();
                let sigma = sigma_permutation(a, b);
                let cycles = decompose_cycles(&sigma);
                let moved: usize = cycles.iter().map(|c| c.len()).sum();
                let largest = cycles.first().map(|c| c.len()).unwrap_or(0);
                stat.pairs += 1;

                // Per large cycle: scan every proper prefix.
                let mut pair_violates = false;
                let mut curves: Vec<String> = Vec::new();
                for cyc in cycles.iter().filter(|c| c.len() >= LARGE_CYCLE_MIN) {
                    stat.large_cycles += 1;
                    let len = cyc.len();
                    let mut min_delta = i64::MAX; // min over prefixes of (score_prefix - score_a)
                    let mut max_delta = i64::MIN;
                    let mut best_prefix_score = 0u32; // highest prefix score seen
                    let mut worse = 0u64;
                    let mut ge = 0u64;
                    // Sample the curve for the JSON (every prefix for the largest
                    // cycle of the first few pairs would be huge; we store a coarse
                    // sampled curve plus the exact extrema).
                    let step = (len / 32).max(1);
                    let mut sampled: Vec<(usize, u32)> = Vec::new();
                    for k in 1..len {
                        let s = score_prefix(a, b, cyc, k);
                        let d = s as i64 - score_a as i64;
                        if d < min_delta {
                            min_delta = d;
                        }
                        if d > max_delta {
                            max_delta = d;
                        }
                        if s > best_prefix_score {
                            best_prefix_score = s;
                        }
                        if s < score_a {
                            worse += 1;
                        } else {
                            ge += 1; // >= score_a: a partial application that did not lose ground
                        }
                        if k % step == 0 {
                            sampled.push((k, s));
                        }
                    }
                    stat.prefixes_tested += (len - 1) as u64;
                    stat.prefixes_worse += worse;
                    stat.prefixes_ge += ge;
                    if ge > 0 {
                        pair_violates = true;
                    }
                    let sampled_j: Vec<String> = sampled
                        .iter()
                        .map(|(k, s)| format!("[{k},{s}]"))
                        .collect();
                    curves.push(format!(
                        "{{\"cycleLen\":{len},\"minDelta\":{min_delta},\"maxDelta\":{max_delta},\
\"bestPrefixScore\":{best_prefix_score},\"prefixesTested\":{},\"prefixesWorse\":{worse},\
\"prefixesGE\":{ge},\"sampledCurve\":[{}]}}",
                        len - 1,
                        sampled_j.join(",")
                    ));
                }
                if pair_violates {
                    stat.violating_pairs += 1;
                }

                pair_json.push(format!(
                    "{{\"group\":{g},\"from\":\"{}\",\"fromScore\":{},\"to\":\"{}\",\"toScore\":{},\
\"scoreA\":{score_a},\"movedCells\":{moved},\"cycleCount\":{},\"largestCycle\":{largest},\
\"cycleLengths\":[{}],\"violatesProperty\":{},\"largeCycleCurves\":[{}]}}",
                    esc(&a.id),
                    a.score.map(|s| s.to_string()).unwrap_or("null".into()),
                    esc(&b.id),
                    b.score.map(|s| s.to_string()).unwrap_or("null".into()),
                    cycles.len(),
                    cycles.iter().map(|c| c.len().to_string()).collect::<Vec<_>>().join(","),
                    pair_violates,
                    curves.join(",")
                ));
            }
        }

        let member_ids: Vec<String> = members
            .iter()
            .map(|&i| {
                format!(
                    "{{\"id\":\"{}\",\"score\":{},\"source\":\"{}\"}}",
                    esc(&solutions[i].id),
                    solutions[i].score.map(|s| s.to_string()).unwrap_or("null".into()),
                    esc(&solutions[i].source)
                )
            })
            .collect();

        group_json.push(format!(
            "{{\"group\":{g},\"boardCount\":{},\"orderedPairs\":{},\"largeCycles\":{},\
\"prefixesTested\":{},\"prefixesWorse\":{},\"prefixesGE\":{},\"violatingPairs\":{},\
\"boards\":[{}]}}",
            members.len(),
            stat.pairs,
            stat.large_cycles,
            stat.prefixes_tested,
            stat.prefixes_worse,
            stat.prefixes_ge,
            stat.violating_pairs,
            member_ids.join(",")
        ));
    }

    // Top-level summary.
    println!("{{");
    println!("  \"metric\": \"sigma-cycle structure and partial-application matched-edge score curves over all ordered same-piece-set bundled board pairs\",");
    println!("  \"maxScore\": {max},");
    println!("  \"largeCycleMinLen\": {LARGE_CYCLE_MIN},");
    println!("  \"boardsParsed\": {parsed},");
    println!("  \"boardsDropped\": {dropped},");
    println!("  \"completeSolutions\": {},", solutions.len());
    println!("  \"pieceSetGroups\": {group_count},");
    println!("  \"note\": \"A piece is the rotation-invariant signature of its four edge colours; boards share a piece set iff those multisets match. Partial application sets the prefix cells to board B's pieces, mirroring the site SigmaCycleLab. 'prefixesGE' counts proper prefixes scoring >= score(A): 0 across the whole population means every partial application of every large cycle scores strictly worse than the start board.\",");
    println!("  \"groups\": [");
    println!("    {}", group_json.join(",\n    "));
    println!("  ],");
    println!("  \"pairCount\": {},", pair_json.len());
    println!("  \"pairs\": [");
    println!("    {}", pair_json.join(",\n    "));
    println!("  ]");
    println!("}}");
}
