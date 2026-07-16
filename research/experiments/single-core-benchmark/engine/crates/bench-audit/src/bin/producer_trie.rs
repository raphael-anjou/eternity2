// producer_trie — frontier/parent-chain beam storage, removing the
// O(B*fanout) full-board-clone RAM ceiling of producer_fast.
//
// Motivation (team-lead directive, citing Frohner et al. 2022 "Parallel
// Beam Search for Combinatorial Optimization" -- they run B up to 20M on
// 46 cores): our beam entries are partial boards that SHARE almost all of
// their placed cells with their siblings and parent at every layer -- at
// depth d, a beam member differs from its parent in exactly ONE cell.
// Storing B full 256-cell board copies per layer (producer_fast's
// packed-but-still-O(B) design) is the RAM wall for B>=131072. This binary
// stores the beam as a FOREST: each node is (parent_index, pos, pid, rot,
// score) -- one placement's delta, not a board. A full board is
// reconstructed only when actually needed (scoring a new cell's <=4
// neighbours, or emitting the final answer), by walking the parent chain
// back to each specific neighbour position via a precomputed
// depth_of[pos] table (the order is fixed upfront, so "which ancestor
// placed position q" is a known number of parent-hops, not a scan).
//
// Frontier design (Frohner-style static 2-buffer, adapted): two arenas
// (Vec<Node>), CURRENT and NEXT, flipped every depth. CURRENT holds this
// layer's B beam nodes (indices are stable for the duration of the layer);
// NEXT accumulates children as they're generated, then becomes CURRENT for
// the next layer. Old arenas are NOT freed (nodes reference into all prior
// layers' arenas via parent_index + parent_layer) -- this is the "trie"
// part: total memory is O(depth * B) NODES (5 small fields each) instead
// of O(depth * B) FULL BOARDS. At 256 depth * 131072 beam * ~16 bytes/node
// that is ~537MB for the whole run's history, vs a naive O(B) full-board
// peak of ~4-5GB for a single layer alone in the flat design.
//
// CORRECTNESS GATE (--threads 1, the default): bit-identical boards to
// producer (and to producer_fast's --serial mode) for
// the same (order, beam, tol, seed) -- the storage change is STORAGE ONLY,
// not the search or the RNG draw sequence. Every published number from this
// binary was recorded on the default single-thread path and reproduces there.
//
// PARALLELISM (--threads N). Frohner et al.'s 46-core/20M-beam
// result is intra-run data parallelism across the beam, and that is what
// --threads N buys. Two independent layers:
//
//   1. SEED-LEVEL (--seed-lo/--seed-hi): each seed already builds its own
//      Rng and its own arenas, so seeds are independent by construction.
//      Fanning the seed loop out does NOT perturb any individual seed's
//      draw stream -- every seed's score is bit-identical to --threads 1 at
//      any thread count. Only stdout row ORDER would change, and we sort
//      rows back into seed order before printing so even that is stable.
//
//   2. INTRA-RUN (the per-layer beam expansion): the serial code threads
//      ONE `rng` through every beam member in index order inside the
//      expansion loop, which is the only true serialization in the layer --
//      resolve_neighbor/candidates_for_cell are pure reads of immutable
//      state. --threads N>1 replaces that single stream with a per-member
//      stream keyed by (seed, depth, member_index) (see `member_rng`).
//
// WHAT --threads N>1 GIVES UP, PRECISELY: the intra-run path's boards are
// NOT bit-identical to the serial stream, because a beam member's tie-break
// draws no longer depend on how many draws its lower-indexed siblings
// happened to consume. It is NOT, however, non-deterministic: the
// per-member key is derived from (seed, depth, member_index) and never from
// thread id, arrival order, or wall-clock, so a given (order, beam, tol,
// seed) is reproducible run-to-run AND ACROSS THREAD COUNTS -- --threads 2
// and --threads 16 produce the same board. The truncation shuffle keeps its
// own single sequential stream (it is O(children) on one thread and not the
// bottleneck), so the beam contents are a pure function of the inputs.
//
// So the guarantee ladder is:
//   --threads 1    == the historical serial stream, bit-for-bit.
//   --threads N>1  != the serial stream, but == itself for any N, always.
//
// Usage: producer, plus [--threads N] (default 1).

use eternity2_core::{Board, Piece, PieceId, Puzzle, Rotation, BORDER};
use eternity2_export::board_to_doc;
use eternity2_puzzle_io::load_puzzle_with_hints;
use rayon::prelude::*;
use std::env;
use std::path::PathBuf;

const W: usize = 16;
const H: usize = 16;
const N: usize = W * H;

// ---------------------------------------------------------------------------
// EMPIRICAL PLACEMENT PRIOR (additive; a prior-weaving beam idea on the
// current-indexing producer).
//
// P[pid][pos] = how often piece `pid` appears at board position `pos` across a
// corpus of our HIGH-SCORE boards. We normalize per POSITION so that
// prior_norm[pid][pos] in [0,1] with 1.0 = the piece most-favored at that
// position. A node accumulates prior_sum = sum over its placed cells of
// prior_norm[pid][pos]. The truncation key blends edge score with the prior:
//
//     effective(node) = score(node) + prior_alpha * prior_sum(node)
//
// so among (near-)equal edge scores, the beam keeps the arrangement our best
// boards historically favor. prior_alpha = 0 is EXACTLY the existing beam
// (the counting-sort path is taken verbatim, bit-identical draw stream).
// Quantization scale for the blended (score + alpha*prior_sum) truncation key.
// Large enough that distinct effective values separate; the round() collapses
// float noise so genuinely-tied keys land in the same shuffle run.
const PRIOR_KEY_SCALE: f32 = 4096.0;

struct Prior {
    // norm[pid][pos] in [0,1]; per-position max-normalized empirical frequency.
    norm: Vec<[f32; N]>,
}

impl Prior {
    #[inline]
    fn get(&self, pid: PieceId, pos: u16) -> f32 {
        self.norm[pid as usize][pos as usize]
    }
}

fn load_prior(path: &str) -> Prior {
    let raw = std::fs::read_to_string(path).expect("read prior file");
    let v: serde_json::Value = serde_json::from_str(&raw).expect("parse prior json");
    let m = v["matrix"].as_array().expect("prior.matrix array");
    assert_eq!(m.len(), 256, "prior expects 256 pieces");
    // Raw counts[pid][pos].
    let mut counts: Vec<[f32; N]> = vec![[0.0f32; N]; 256];
    for (pid, row) in m.iter().enumerate() {
        let cols = row.as_array().expect("prior row array");
        assert_eq!(cols.len(), N, "prior expects 256 positions/row");
        for (pos, c) in cols.iter().enumerate() {
            counts[pid][pos] = c.as_f64().expect("prior count") as f32;
        }
    }
    // Per-position max-normalize: divide each column by its column max.
    let mut colmax = [0.0f32; N];
    for row in &counts {
        for pos in 0..N {
            if row[pos] > colmax[pos] {
                colmax[pos] = row[pos];
            }
        }
    }
    let mut norm: Vec<[f32; N]> = vec![[0.0f32; N]; 256];
    for pid in 0..256 {
        for pos in 0..N {
            norm[pid][pos] = if colmax[pos] > 0.0 { counts[pid][pos] / colmax[pos] } else { 0.0 };
        }
    }
    Prior { norm }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Order {
    RowMajor,
    Spiral,
    BorderFirst,
    // COMB: Verhaard & Max's high-score fill geometry (msg 6112/6126).
    // The top `split_row` rows are filled row-major (the "handle" / scanline
    // part); the remaining rows split_row..H are filled COLUMN-BY-COLUMN, left
    // to right, top to bottom within each column (the vertical "teeth"). Each
    // tooth is a short, near-independent column whose failures are local, so a
    // high-scoring deep frontier is reachable many cheap ways instead of one
    // expensive full-width frontier. Tooth length = H - split_row is tuned to
    // the target score ("the lower the score you aim for, the longer the teeth").
    // Orientation matches the producer's native top-to-bottom row-major baseline
    // so the comb-vs-rowmajor comparison is apples-to-apples (same corner start).
    Comb { split_row: usize },
}

fn parse_order(s: &str) -> Order {
    match s {
        "rowmajor" => Order::RowMajor,
        "spiral" => Order::Spiral,
        "borderfirst" => Order::BorderFirst,
        // "comb" alone defaults to split_row=12 (Max's twelve-rows order);
        // "comb:N" sets split_row=N explicitly.
        "comb" => Order::Comb { split_row: 12 },
        _ => {
            if let Some(rest) = s.strip_prefix("comb:") {
                let split_row: usize = rest.parse().expect("comb:N — N must be an integer");
                assert!(split_row < H, "comb split_row {split_row} must be < {H}");
                Order::Comb { split_row }
            } else {
                panic!("unknown order {s}")
            }
        }
    }
}

fn row_major_order() -> Vec<u32> {
    (0..(W * H) as u32).collect()
}

fn spiral_order() -> Vec<u32> {
    let mut order = Vec::with_capacity(W * H);
    let (mut top, mut bottom, mut left, mut right) = (0i32, H as i32 - 1, 0i32, W as i32 - 1);
    while top <= bottom && left <= right {
        for x in left..=right {
            order.push((top * W as i32 + x) as u32);
        }
        top += 1;
        for y in top..=bottom {
            order.push((y * W as i32 + right) as u32);
        }
        right -= 1;
        if top <= bottom {
            for x in (left..=right).rev() {
                order.push((bottom * W as i32 + x) as u32);
            }
            bottom -= 1;
        }
        if left <= right {
            for y in (top..=bottom).rev() {
                order.push((y * W as i32 + left) as u32);
            }
            left += 1;
        }
    }
    assert_eq!(order.len(), W * H);
    order
}

fn border_first_order() -> Vec<u32> {
    let mut order = Vec::with_capacity(W * H);
    for x in 0..W as i32 {
        order.push((0 * W as i32 + x) as u32);
    }
    for y in 1..H as i32 - 1 {
        order.push((y * W as i32 + (W as i32 - 1)) as u32);
    }
    for x in (0..W as i32).rev() {
        order.push(((H as i32 - 1) * W as i32 + x) as u32);
    }
    for y in (1..H as i32 - 1).rev() {
        order.push((y * W as i32) as u32);
    }
    let mut seen = vec![false; W * H];
    for &p in &order {
        seen[p as usize] = true;
    }
    for y in 1..H as i32 - 1 {
        for x in 1..W as i32 - 1 {
            let p = (y * W as i32 + x) as u32;
            if !seen[p as usize] {
                order.push(p);
                seen[p as usize] = true;
            }
        }
    }
    assert_eq!(order.len(), W * H);
    order
}

/// Comb fill order: rows `0..split_row` in row-major order (the
/// scanline handle), then rows `split_row..H` filled column-by-column (vertical
/// teeth), columns left-to-right and rows top-to-bottom within each column.
/// `split_row == H` degenerates to pure row-major; `split_row == 0` is a pure
/// column scan. Every position appears exactly once.
fn comb_order(split_row: usize) -> Vec<u32> {
    let mut order = Vec::with_capacity(W * H);
    // Handle: top `split_row` rows, row-major (bit-identical prefix to RowMajor).
    for y in 0..split_row {
        for x in 0..W {
            order.push((y * W + x) as u32);
        }
    }
    // Teeth: remaining rows, column-by-column.
    for x in 0..W {
        for y in split_row..H {
            order.push((y * W + x) as u32);
        }
    }
    assert_eq!(order.len(), W * H);
    order
}

fn order_positions(o: Order) -> Vec<u32> {
    match o {
        Order::RowMajor => row_major_order(),
        Order::Spiral => spiral_order(),
        Order::BorderFirst => border_first_order(),
        Order::Comb { split_row } => comb_order(split_row),
    }
}

// xorshift64 seeded RNG, matching the original engine's stream exactly.
struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self {
        Self(if seed == 0 { 0x9E3779B97F4A7C15 } else { seed })
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }
    fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = self.below(i + 1);
            v.swap(i, j);
        }
    }
}

/// Per-beam-member RNG for the `--threads N>1` expansion path.
///
/// The serial path threads one `Rng` through the members of a layer in index
/// order, so member k's draws depend on how many draws members 0..k consumed
/// -- inherently sequential. Here each member instead gets a stream seeded by
/// a SplitMix64 finalizer over `(seed, depth, member_index)`. Every input is a
/// pure function of the search state, so the stream a member sees does not
/// depend on which thread ran it, when it started, or how many threads exist:
/// `--threads 2` and `--threads 16` draw identically. That is what makes N>1
/// non-reproducible-vs-serial but still fully deterministic.
///
/// SplitMix64's finalizer (not xorshift64 seeded directly) because the three
/// inputs are small and highly correlated across members -- consecutive
/// member_index values must not yield correlated streams, and xorshift64 seeds
/// poorly from near-identical values.
#[inline]
fn member_rng(seed: u64, depth: u32, member_index: usize) -> Rng {
    let mut z = seed
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(u64::from(depth).wrapping_mul(0xBF58_476D_1CE4_E5B9))
        .wrapping_add((member_index as u64).wrapping_mul(0x94D0_49BB_1331_11EB));
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    Rng::new(z)
}

fn rim_needs(pos: u32) -> [bool; 4] {
    let y = pos / W as u32;
    let x = pos % W as u32;
    [y == 0, x == W as u32 - 1, y == H as u32 - 1, x == 0]
}

#[derive(Clone, Copy)]
struct LegalPlacement {
    pid: PieceId,
    rot: Rotation,
    edges: [u8; 4],
}

fn edges_of(piece: &Piece, rot: Rotation) -> [u8; 4] {
    piece.edges.rotated(rot).as_array()
}

fn build_legal_table(puzzle: &Puzzle) -> Vec<Vec<LegalPlacement>> {
    let mut table: Vec<Vec<LegalPlacement>> = vec![Vec::new(); N];
    for pos in 0..N as u32 {
        let needs = rim_needs(pos);
        let mut v = Vec::new();
        for piece in puzzle.pieces() {
            for &r in &Rotation::ALL {
                let e = edges_of(piece, r);
                let mut ok = true;
                for s in 0..4 {
                    if needs[s] != (e[s] == BORDER) {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    v.push(LegalPlacement { pid: piece.id, rot: r, edges: e });
                }
            }
        }
        table[pos as usize] = v;
    }
    table
}

fn build_piece_edges(puzzle: &Puzzle) -> Vec<[u8; 4]> {
    let max_id = puzzle.pieces().iter().map(|p| p.id).max().unwrap_or(0);
    let mut table = vec![[0u8; 4]; max_id as usize + 1];
    for piece in puzzle.pieces() {
        table[piece.id as usize] = piece.edges.as_array();
    }
    table
}

#[inline]
fn rotated_edge(piece_edges_r0: &[[u8; 4]], pid: PieceId, rot: Rotation, side: usize) -> u8 {
    let raw_side = (side + 4 - rot.as_u8() as usize) % 4;
    piece_edges_r0[pid as usize][raw_side]
}

// Bitset over 256 piece ids -- carried incrementally alongside each live
// beam entry (BeamRef.used below), NOT stored per historical Node (that
// would add 32 bytes to every one of the O(depth*beam_width) nodes kept
// for the whole run, defeating the RAM win). Only the CURRENT layer's
// live entries need one each, which is the same O(beam_width) cost the
// flat producer already pays for its used-set.
#[derive(Clone, Copy, Default)]
struct UsedSet([u64; 4]);
impl UsedSet {
    #[inline]
    fn get(&self, pid: PieceId) -> bool {
        (self.0[(pid >> 6) as usize] >> (pid & 63)) & 1 == 1
    }
    #[inline]
    fn set(&mut self, pid: PieceId) {
        self.0[(pid >> 6) as usize] |= 1u64 << (pid & 63);
    }
}

/// One beam node: a single placement's delta plus a pointer to the parent
/// node that placed everything else. `parent` indexes into `layers
/// [layer_of_parent]`; `layer_of_parent = depth - 1` for every non-root
/// node (nodes are created exactly one search-depth apart), so a node's
/// full ancestry is `layers[depth-1][parent], layers[depth-2][...], ...`
/// down to the root sentinel at layer 0 (the initial hinted board, itself
/// stored as its own single-element "layer -1").
#[derive(Clone, Copy)]
struct Node {
    parent: u32,
    pos: u16,
    pid: PieceId,
    rot: Rotation,
    score: i32,
}

/// The initial (hinted) board, materialized in full once -- it is shared
/// by every node's ancestry chain as the ultimate root, so it is the one
/// place a full 256-cell array legitimately lives for the whole run.
struct RootBoard {
    cells: Box<[u16; N]>, // 0xFFFF sentinel = empty
}

const EMPTY: u16 = 0xFFFF;

#[inline]
fn pack(pid: PieceId, r: Rotation) -> u16 {
    (pid << 2) | u16::from(r.as_u8())
}
#[inline]
fn unpack(v: u16) -> (PieceId, Rotation) {
    (v >> 2, Rotation::from_u8((v & 3) as u8).unwrap())
}

/// Resolve what piece/rotation sits at `pos` for the partial represented by
/// `(layers, layer_idx, node_idx)`, by walking the parent chain. `depth_of`
/// maps position -> the search depth at which that position is filled
/// (u32::MAX if it's a hint / pre-filled in the root), so the walk knows
/// exactly how many parent-hops to take instead of scanning.
fn resolve_cell(
    layers: &[Vec<Node>],
    mut layer_idx: usize,
    mut node_idx: u32,
    pos: u32,
    depth_of: &[u32],
    root: &RootBoard,
) -> Option<(PieceId, Rotation)> {
    let target_depth = depth_of[pos as usize];
    if target_depth == u32::MAX {
        // Filled in the root (a hint) -- never touched by any node.
        let v = root.cells[pos as usize];
        return if v == EMPTY { None } else { Some(unpack(v)) };
    }
    loop {
        let node = layers[layer_idx][node_idx as usize];
        // layer_idx (0-based into `layers`) corresponds to search depth
        // layer_idx+1 (layer 0 holds the depth-1 nodes, extending the
        // depth-0 root). So this node's OWN placement is at search depth
        // layer_idx+1.
        let this_depth = layer_idx as u32 + 1;
        if node.pos as u32 == pos {
            return Some((node.pid, node.rot));
        }
        if this_depth <= target_depth {
            // We've walked past the point where `pos` could have been
            // placed by an ancestor and it wasn't this node -- means `pos`
            // is not yet placed in this partial (it's later in visiting
            // order than the current node). Only happens if the caller
            // asks about a not-yet-filled neighbour, which callers must
            // guard against via the "already placed" checks below.
            return None;
        }
        if layer_idx == 0 {
            return None; // shouldn't happen given depth_of guards
        }
        node_idx = node.parent;
        layer_idx -= 1;
    }
}

/// Candidates for `pos`, given the resolved (already-placed-only) up/right/
/// bottom/left neighbour edges. Same scoring logic as the flat producer,
/// parameterized over pre-resolved Option<u8> edges so this function has
/// no storage-format dependency.
/// Fill `out` (cleared first) with the usable placements for this cell and
/// their edge-match gain, sorted by gain descending. Takes a caller-owned
/// scratch buffer so the hot beam loop pays no per-call allocation (this fn is
/// ~50% of producer runtime; it runs once per beam node per cell). Behaviour is
/// bit-identical to the previous `Vec`-returning version: same entries, same
/// stable sort order.
#[inline]
fn candidates_for_cell(
    out: &mut Vec<(PieceId, Rotation, i8)>,
    legal: &[LegalPlacement],
    used: &UsedSet,
    top_edge: Option<u8>,
    right_edge: Option<u8>,
    bottom_edge: Option<u8>,
    left_edge: Option<u8>,
) {
    out.clear();
    for lp in legal {
        if used.get(lp.pid) {
            continue;
        }
        let mut gain: i8 = 0;
        if let Some(te) = top_edge {
            if lp.edges[0] == te {
                gain += 1;
            }
        }
        if let Some(re) = right_edge {
            if lp.edges[1] == re {
                gain += 1;
            }
        }
        if let Some(be) = bottom_edge {
            if lp.edges[2] == be {
                gain += 1;
            }
        }
        if let Some(le) = left_edge {
            if lp.edges[3] == le {
                gain += 1;
            }
        }
        out.push((lp.pid, lp.rot, gain));
    }
    out.sort_by_key(|&(_, _, g)| -(g as i32));
}

/// Draw up to `k` picks from the gain-sorted `cands`, at each step choosing
/// uniformly at random within the current top tie-class (gains within `tol` of
/// the max), into `chosen`. Consumes `cands` IN PLACE (it is the caller's reused
/// scratch buffer, not read afterwards) — this avoids the previous
/// `cands.to_vec()` that cloned all ~784 interior candidates every call. The
/// `remove(idx)` order-preserving shifts and the exact `rng.below` draw sequence
/// are unchanged, so the selection is bit-identical to the old version.
fn pick_tie_class(
    cands: &mut Vec<(PieceId, Rotation, i8)>,
    tol: i8,
    rng: &mut Rng,
    k: usize,
    chosen: &mut Vec<(PieceId, Rotation, i8)>,
) {
    chosen.clear();
    let n = k.min(cands.len());
    for _ in 0..n {
        if cands.is_empty() {
            break;
        }
        let top = cands[0].2;
        let tie_len = cands.iter().take_while(|c| top - c.2 <= tol).count();
        let idx = rng.below(tie_len.max(1));
        chosen.push(cands.remove(idx));
        if cands.is_empty() {
            break;
        }
    }
}

/// A "beam entry" reference during the search: an index into the most
/// recently completed layer (or usize::MAX/root for depth 0), plus the
/// running used-set and score cached alongside it -- used-set IS cached
/// per live beam entry (32 bytes) because re-deriving it by a full parent
/// walk on every single cell (not just neighbour lookups) would be O(depth)
/// per beam member per cell, i.e. right back to O(B*depth) work overall.
/// Caching it costs O(B) memory per LIVE layer only (not every historical
/// layer), which is the same order as the flat producer's used-set but
/// full 512B board -- so this is still the intended RAM win.
#[derive(Clone, Copy)]
struct BeamRef {
    node_idx: u32, // index into the just-created layer (or root sentinel)
    used: UsedSet,
    score: i32,
    // Accumulated per-position-normalized prior over this node's placed cells.
    // 0.0 for every node when no prior is loaded (prior_alpha unused then).
    prior_sum: f32,
}

#[allow(clippy::too_many_arguments)]
fn run_one(
    hints: &eternity2_core::Hints,
    order: Order,
    beam_width: usize,
    tol: i8,
    seed: u64,
    legal_table: &[Vec<LegalPlacement>],
    piece_edges: &[[u8; 4]],
    prior: Option<&Prior>,
    prior_alpha: f32,
    // 1 = the historical serial expansion (single shared RNG stream, bit-identical
    // to producer). >1 = per-member RNG streams, expanded in parallel.
    threads: usize,
) -> (i32, Box<[u16; N]>, usize, usize) {
    let mut rng = Rng::new(seed);
    let positions = order_positions(order);
    // Prior guidance is active only with a loaded prior AND positive alpha; the
    // alpha=0 path never touches the counting-sort truncation (bit-identical).
    let use_prior = prior.is_some() && prior_alpha > 0.0;

    let mut root = RootBoard { cells: Box::new([EMPTY; N]) };
    let mut root_used = UsedSet::default();
    for h in &hints.hints {
        root.cells[h.position as usize] = pack(h.piece_id, h.rotation);
        root_used.set(h.piece_id);
    }
    let root_score = score_partial(&root.cells, piece_edges);

    let hint_positions: Vec<bool> = {
        let mut v = vec![false; N];
        for h in &hints.hints {
            v[h.position as usize] = true;
        }
        v
    };

    // depth_of[pos] = the search depth (1-based; matches `this_depth` in
    // resolve_cell) at which `pos` is filled by a NODE, or u32::MAX if
    // it's a hint (filled in the root, depth 0).
    let mut depth_of = vec![u32::MAX; N];
    {
        let mut d = 0u32;
        for &pos in positions.iter().filter(|&&p| !hint_positions[p as usize]) {
            d += 1;
            depth_of[pos as usize] = d;
        }
    }

    // `layers[i]` holds the nodes created at search depth i+1.
    let mut layers: Vec<Vec<Node>> = Vec::new();
    let mut beam: Vec<BeamRef> =
        vec![BeamRef { node_idx: u32::MAX, used: root_used, score: root_score, prior_sum: 0.0 }];

    let fanout = 8usize.min(beam_width.max(1));
    let mut peak_layer_len = 0usize;
    let par = threads > 1;
    // Search depth of the layer about to be built; 1-based, matching depth_of.
    // Only read on the parallel path, as the per-member RNG key.
    let mut cur_depth = 0u32;

    for &pos in positions.iter().filter(|&&p| !hint_positions[p as usize]) {
        cur_depth += 1;
        let legal = &legal_table[pos as usize];
        let y = pos / W as u32;
        let x = pos % W as u32;
        let cur_layer_idx = layers.len(); // this layer's index once pushed

        // Precompute which of the 4 neighbours are already placed and at
        // what (layer_idx, "how many hops back") -- for row-major/border-
        // first these are always within the last 1-2 layers; for spiral
        // they can be further but resolve_cell handles any distance.
        let need_top = y > 0;
        let need_right = x < W as u32 - 1;
        let need_bottom = y < H as u32 - 1;
        let need_left = x > 0;

        let mut children: Vec<Node> = Vec::with_capacity(beam.len() * fanout);
        let mut child_used: Vec<UsedSet> = Vec::with_capacity(beam.len() * fanout);
        let mut child_prior: Vec<f32> = Vec::with_capacity(beam.len() * fanout);

        // The per-member work: resolve <=4 neighbour edges by walking the parent
        // chain, score the legal placements, draw up to `fanout` tie-class picks.
        // Everything it touches outside its own scratch buffers (`layers`, `root`,
        // `legal`, `depth_of`, `piece_edges`) is immutable for the whole layer,
        // so the only thing standing between this and a parallel map is the RNG --
        // which is why the parallel arm hands each member its own stream.
        let expand = |bref: &BeamRef,
                      rng: &mut Rng,
                      cand_buf: &mut Vec<(PieceId, Rotation, i8)>,
                      picks_buf: &mut Vec<(PieceId, Rotation, i8)>,
                      out: &mut Vec<(Node, UsedSet, f32)>| {
            let top_edge = if need_top {
                resolve_neighbor(&layers, cur_layer_idx, bref.node_idx, pos - W as u32, &depth_of, &root)
                    .map(|(pid, r)| rotated_edge(piece_edges, pid, r, 2))
            } else {
                None
            };
            let right_edge = if need_right {
                resolve_neighbor(&layers, cur_layer_idx, bref.node_idx, pos + 1, &depth_of, &root)
                    .map(|(pid, r)| rotated_edge(piece_edges, pid, r, 3))
            } else {
                None
            };
            let bottom_edge = if need_bottom {
                resolve_neighbor(&layers, cur_layer_idx, bref.node_idx, pos + W as u32, &depth_of, &root)
                    .map(|(pid, r)| rotated_edge(piece_edges, pid, r, 0))
            } else {
                None
            };
            let left_edge = if need_left {
                resolve_neighbor(&layers, cur_layer_idx, bref.node_idx, pos - 1, &depth_of, &root)
                    .map(|(pid, r)| rotated_edge(piece_edges, pid, r, 1))
            } else {
                None
            };

            candidates_for_cell(cand_buf, legal, &bref.used, top_edge, right_edge, bottom_edge, left_edge);
            if cand_buf.is_empty() {
                return;
            }
            pick_tie_class(cand_buf, tol, rng, fanout, picks_buf);
            for &(pid, r, gain) in picks_buf.iter() {
                let mut nu = bref.used;
                nu.set(pid);
                let pinc = if use_prior { prior.unwrap().get(pid, pos as u16) } else { 0.0 };
                out.push((
                    Node { parent: bref.node_idx, pos: pos as u16, pid, rot: r, score: bref.score + gain as i32 },
                    nu,
                    bref.prior_sum + pinc,
                ));
            }
        };

        if par {
            // Parallel expansion. Each member produces its own child list keyed by
            // its beam index, and we concatenate those lists IN BEAM-INDEX ORDER
            // (not completion order) below -- so `children` is exactly the vector
            // the serial arm would build from the same per-member draws, and the
            // downstream counting sort / shuffle / truncate see a thread-count-
            // independent input. This is the property that makes --threads 2 and
            // --threads 16 agree.
            let per_member: Vec<Vec<(Node, UsedSet, f32)>> = beam
                .par_iter()
                .enumerate()
                .map(|(mi, bref)| {
                    let mut rng = member_rng(seed, cur_depth, mi);
                    let mut cand_buf: Vec<(PieceId, Rotation, i8)> = Vec::with_capacity(legal.len());
                    let mut picks_buf: Vec<(PieceId, Rotation, i8)> = Vec::with_capacity(fanout);
                    let mut out: Vec<(Node, UsedSet, f32)> = Vec::with_capacity(fanout);
                    expand(bref, &mut rng, &mut cand_buf, &mut picks_buf, &mut out);
                    out
                })
                .collect();
            for m in per_member {
                for (node, used, ps) in m {
                    children.push(node);
                    child_used.push(used);
                    child_prior.push(ps);
                }
            }
        } else {
            // Serial expansion: one shared RNG stream threaded through the members
            // in beam order -- the historical draw sequence, bit-for-bit.
            let mut cand_buf: Vec<(PieceId, Rotation, i8)> = Vec::with_capacity(legal.len());
            let mut picks_buf: Vec<(PieceId, Rotation, i8)> = Vec::with_capacity(fanout);
            let mut out: Vec<(Node, UsedSet, f32)> = Vec::with_capacity(fanout);
            for bref in &beam {
                out.clear();
                expand(bref, &mut rng, &mut cand_buf, &mut picks_buf, &mut out);
                for &(node, used, ps) in out.iter() {
                    children.push(node);
                    child_used.push(used);
                    child_prior.push(ps);
                }
            }
        }

        if children.is_empty() {
            let mut best = 0usize;
            for i in 1..beam.len() {
                if beam[i].score > beam[best].score {
                    best = i;
                }
            }
            let board = materialize(&layers, layers.len().wrapping_sub(1), beam[best].node_idx, &root);
            return (beam[best].score, board, layers.len(), peak_layer_len);
        }

        // Truncate with the SAME algorithm as the original: (stable) sort
        // by score descending, seeded shuffle within every equal-score
        // run, then truncate. Operates on (Node, UsedSet) pairs zipped
        // together so the shuffle permutes both in lockstep.
        //
        // Frohner et al. 2022's central speed trick (team-lead directive):
        // scores are bounded ints (0..=480 for a full E2 board, so a
        // partial's running score is too), so a COUNTING SORT into 481
        // buckets is O(n + 481) instead of O(n log n) -- and, walked
        // bucket-descending with elements appended in original relative
        // order within each bucket, produces EXACTLY the same element
        // ordering a stable descending sort_by_key would (that's the
        // definition of a stable sort: ties keep source order). The
        // subsequent equal-score-run shuffle therefore sees the identical
        // grouping and identical pre-shuffle order it always did --
        // bit-identical to the sort_by_key version, just O(n) instead of
        // O(n log n) to get there.
        let mut idx: Vec<usize> = if !use_prior {
            const MAX_SCORE: usize = 480;
            let mut counts = [0u32; MAX_SCORE + 1];
            for c in &children {
                debug_assert!(c.score >= 0 && c.score as usize <= MAX_SCORE);
                counts[c.score as usize] += 1;
            }
            // Prefix sums over DESCENDING score give each bucket's start
            // offset in the final (highest-score-first) ordering.
            let mut offset = [0u32; MAX_SCORE + 1];
            let mut running = 0u32;
            for s in (0..=MAX_SCORE).rev() {
                offset[s] = running;
                running += counts[s];
            }
            let mut out = vec![0usize; children.len()];
            let mut cursor = offset;
            for (i, c) in children.iter().enumerate() {
                let s = c.score as usize;
                out[cursor[s] as usize] = i;
                cursor[s] += 1;
            }
            out
        } else {
            // PRIOR-GUIDED order: rank by effective = score + alpha*prior_sum,
            // descending. We quantize the effective key to an integer so the
            // subsequent equal-key shuffle (below) groups genuinely-equal keys
            // deterministically. With alpha=0 this branch is never taken.
            let mut out: Vec<usize> = (0..children.len()).collect();
            let keyed: Vec<i64> = children
                .iter()
                .zip(child_prior.iter())
                .map(|(c, &ps)| ((c.score as f32 + prior_alpha * ps) * PRIOR_KEY_SCALE).round() as i64)
                .collect();
            // Stable descending sort by effective key (stable so equal keys keep
            // source order, matching the counting-sort convention before shuffle).
            out.sort_by(|&a, &b| keyed[b].cmp(&keyed[a]));
            out
        };
        // Shuffle within equal-key runs. Key = plain score when no prior; the
        // quantized effective key when prior-guided.
        let key_of = |i: usize| -> i64 {
            if use_prior {
                ((children[i].score as f32 + prior_alpha * child_prior[i]) * PRIOR_KEY_SCALE).round() as i64
            } else {
                children[i].score as i64
            }
        };
        let mut i = 0;
        while i < idx.len() {
            let mut j = i;
            while j < idx.len() && key_of(idx[j]) == key_of(idx[i]) {
                j += 1;
            }
            rng.shuffle(&mut idx[i..j]);
            i = j;
        }
        idx.truncate(beam_width);

        let mut new_layer: Vec<Node> = Vec::with_capacity(idx.len());
        let mut new_beam: Vec<BeamRef> = Vec::with_capacity(idx.len());
        for &i in &idx {
            new_layer.push(children[i]);
            new_beam.push(BeamRef { node_idx: (new_layer.len() - 1) as u32, used: child_used[i], score: children[i].score, prior_sum: child_prior[i] });
        }
        peak_layer_len = peak_layer_len.max(new_layer.len());
        layers.push(new_layer);
        beam = new_beam;
    }

    let mut best = 0usize;
    for i in 1..beam.len() {
        if beam[i].score > beam[best].score {
            best = i;
        }
    }
    let board = materialize(&layers, layers.len() - 1, beam[best].node_idx, &root);
    (beam[best].score, board, layers.len(), peak_layer_len)
}

/// Harvest variant of [`run_one`]: identical search dynamics (same RNG draw
/// sequence, same beam, same truncation) but instead of returning only the
/// single best final board it materializes EVERY surviving member of the
/// FINAL beam and returns `(final_score, board)` for each, sorted by score
/// descending (rank 0 = best). This is the "finishable vs doomed lineage"
/// sample set: the top-scoring members are finishable, the low ones doomed,
/// and all share the identical producer dynamics. Kept as a separate copy so
/// `run_one`'s hot path and bit-identical --emit-best behaviour are untouched.
fn run_one_harvest(
    hints: &eternity2_core::Hints,
    order: Order,
    beam_width: usize,
    tol: i8,
    seed: u64,
    legal_table: &[Vec<LegalPlacement>],
    piece_edges: &[[u8; 4]],
) -> Vec<(i32, Box<[u16; N]>)> {
    let mut rng = Rng::new(seed);
    let positions = order_positions(order);

    let mut root = RootBoard { cells: Box::new([EMPTY; N]) };
    let mut root_used = UsedSet::default();
    for h in &hints.hints {
        root.cells[h.position as usize] = pack(h.piece_id, h.rotation);
        root_used.set(h.piece_id);
    }
    let root_score = score_partial(&root.cells, piece_edges);

    let hint_positions: Vec<bool> = {
        let mut v = vec![false; N];
        for h in &hints.hints {
            v[h.position as usize] = true;
        }
        v
    };

    let mut depth_of = vec![u32::MAX; N];
    {
        let mut d = 0u32;
        for &pos in positions.iter().filter(|&&p| !hint_positions[p as usize]) {
            d += 1;
            depth_of[pos as usize] = d;
        }
    }

    let mut layers: Vec<Vec<Node>> = Vec::new();
    let mut beam: Vec<BeamRef> = vec![BeamRef { node_idx: u32::MAX, used: root_used, score: root_score, prior_sum: 0.0 }];

    let fanout = 8usize.min(beam_width.max(1));

    for &pos in positions.iter().filter(|&&p| !hint_positions[p as usize]) {
        let legal = &legal_table[pos as usize];
        let y = pos / W as u32;
        let x = pos % W as u32;
        let cur_layer_idx = layers.len();

        let need_top = y > 0;
        let need_right = x < W as u32 - 1;
        let need_bottom = y < H as u32 - 1;
        let need_left = x > 0;

        let mut children: Vec<Node> = Vec::with_capacity(beam.len() * fanout);
        let mut child_used: Vec<UsedSet> = Vec::with_capacity(beam.len() * fanout);
        let mut cand_buf: Vec<(PieceId, Rotation, i8)> = Vec::with_capacity(legal.len());
        let mut picks_buf: Vec<(PieceId, Rotation, i8)> = Vec::with_capacity(fanout);

        for bref in &beam {
            let top_edge = if need_top {
                resolve_neighbor(&layers, cur_layer_idx, bref.node_idx, pos - W as u32, &depth_of, &root)
                    .map(|(pid, r)| rotated_edge(piece_edges, pid, r, 2))
            } else {
                None
            };
            let right_edge = if need_right {
                resolve_neighbor(&layers, cur_layer_idx, bref.node_idx, pos + 1, &depth_of, &root)
                    .map(|(pid, r)| rotated_edge(piece_edges, pid, r, 3))
            } else {
                None
            };
            let bottom_edge = if need_bottom {
                resolve_neighbor(&layers, cur_layer_idx, bref.node_idx, pos + W as u32, &depth_of, &root)
                    .map(|(pid, r)| rotated_edge(piece_edges, pid, r, 0))
            } else {
                None
            };
            let left_edge = if need_left {
                resolve_neighbor(&layers, cur_layer_idx, bref.node_idx, pos - 1, &depth_of, &root)
                    .map(|(pid, r)| rotated_edge(piece_edges, pid, r, 1))
            } else {
                None
            };

            candidates_for_cell(&mut cand_buf, legal, &bref.used, top_edge, right_edge, bottom_edge, left_edge);
            if cand_buf.is_empty() {
                continue;
            }
            pick_tie_class(&mut cand_buf, tol, &mut rng, fanout, &mut picks_buf);
            for &(pid, r, gain) in &picks_buf {
                let mut nu = bref.used;
                nu.set(pid);
                children.push(Node { parent: bref.node_idx, pos: pos as u16, pid, rot: r, score: bref.score + gain as i32 });
                child_used.push(nu);
            }
        }

        if children.is_empty() {
            return harvest_final_beam(&layers, &beam, &root);
        }

        let mut idx: Vec<usize> = {
            const MAX_SCORE: usize = 480;
            let mut counts = [0u32; MAX_SCORE + 1];
            for c in &children {
                debug_assert!(c.score >= 0 && c.score as usize <= MAX_SCORE);
                counts[c.score as usize] += 1;
            }
            let mut offset = [0u32; MAX_SCORE + 1];
            let mut running = 0u32;
            for s in (0..=MAX_SCORE).rev() {
                offset[s] = running;
                running += counts[s];
            }
            let mut out = vec![0usize; children.len()];
            let mut cursor = offset;
            for (i, c) in children.iter().enumerate() {
                let s = c.score as usize;
                out[cursor[s] as usize] = i;
                cursor[s] += 1;
            }
            out
        };
        let mut i = 0;
        while i < idx.len() {
            let mut j = i;
            while j < idx.len() && children[idx[j]].score == children[idx[i]].score {
                j += 1;
            }
            rng.shuffle(&mut idx[i..j]);
            i = j;
        }
        idx.truncate(beam_width);

        let mut new_layer: Vec<Node> = Vec::with_capacity(idx.len());
        let mut new_beam: Vec<BeamRef> = Vec::with_capacity(idx.len());
        for &i in &idx {
            new_layer.push(children[i]);
            new_beam.push(BeamRef { node_idx: (new_layer.len() - 1) as u32, used: child_used[i], score: children[i].score, prior_sum: 0.0 });
        }
        layers.push(new_layer);
        beam = new_beam;
    }

    harvest_final_beam(&layers, &beam, &root)
}

/// Materialize every member of the final beam into a full board with its own
/// final score, returned sorted by score descending (rank 0 = best).
fn harvest_final_beam(
    layers: &[Vec<Node>],
    beam: &[BeamRef],
    root: &RootBoard,
) -> Vec<(i32, Box<[u16; N]>)> {
    let last_layer = layers.len().wrapping_sub(1);
    let mut out: Vec<(i32, Box<[u16; N]>)> = beam
        .iter()
        .map(|bref| (bref.score, materialize(layers, last_layer, bref.node_idx, root)))
        .collect();
    out.sort_by_key(|&(score, _)| -score);
    out
}

/// R7-C TRUNK-DIVERSIFIED beam. Identical search dynamics as `run_one` EXCEPT
/// the truncation step enforces a per-trunk quota: after sorting children by
/// score (with the same seeded equal-score shuffle), the beam is filled
/// greedily but at most `cap` members may share the same rows-0..(div_row-1)
/// prefix ("trunk signature"). Once the beam is full of quota-respecting
/// members, remaining high-score children that would overflow a trunk's quota
/// are dropped in favour of members from under-represented (or new) trunks,
/// down to a score floor of `best - div_tol` (so we never keep arbitrarily bad
/// members just for diversity). This forces structurally distinct trunks to
/// survive the truncation that otherwise collapses the beam to one lineage.
///
/// Returns the harvested final beam (score, board) sorted desc, plus the
/// number of DISTINCT rows-0..(div_row-1) trunk signatures present in the
/// final beam (the row-`div_row` trunk count KPI).
#[allow(clippy::too_many_arguments)]
fn run_one_diversify(
    hints: &eternity2_core::Hints,
    order: Order,
    beam_width: usize,
    tol: i8,
    seed: u64,
    legal_table: &[Vec<LegalPlacement>],
    piece_edges: &[[u8; 4]],
    div_row: usize, // trunk signature = positions 0..div_row*16-1 (row-major only)
    cap: usize,     // max beam members per trunk signature
    div_tol: i32,   // keep diversity members within this score gap of the layer best
) -> (Vec<(i32, Box<[u16; N]>)>, usize) {
    assert!(matches!(order, Order::RowMajor), "diversify requires rowmajor order");
    let mut rng = Rng::new(seed);
    let positions = order_positions(order);

    let mut root = RootBoard { cells: Box::new([EMPTY; N]) };
    let mut root_used = UsedSet::default();
    for h in &hints.hints {
        root.cells[h.position as usize] = pack(h.piece_id, h.rotation);
        root_used.set(h.piece_id);
    }
    let root_score = score_partial(&root.cells, piece_edges);
    let hint_positions: Vec<bool> = {
        let mut v = vec![false; N];
        for h in &hints.hints {
            v[h.position as usize] = true;
        }
        v
    };
    let mut depth_of = vec![u32::MAX; N];
    {
        let mut d = 0u32;
        for &pos in positions.iter().filter(|&&p| !hint_positions[p as usize]) {
            d += 1;
            depth_of[pos as usize] = d;
        }
    }
    // The signature becomes "fixed" once the search has placed the last cell of
    // row (div_row-1), i.e. position div_row*16 - 1. The search depth at which
    // that happens:
    let sig_last_pos = (div_row * W).saturating_sub(1) as u32;
    let sig_fixed_depth = depth_of[sig_last_pos as usize]; // may be u32::MAX if a hint (won't be)

    let mut layers: Vec<Vec<Node>> = Vec::new();
    // Carry a trunk-signature hash per beam member (0 = not yet fixed).
    let mut beam: Vec<BeamRef> = vec![BeamRef { node_idx: u32::MAX, used: root_used, score: root_score, prior_sum: 0.0 }];
    let mut beam_sig: Vec<u64> = vec![0];

    let fanout = 8usize.min(beam_width.max(1));
    let mut cur_depth = 0u32;

    for &pos in positions.iter().filter(|&&p| !hint_positions[p as usize]) {
        cur_depth += 1;
        let legal = &legal_table[pos as usize];
        let y = pos / W as u32;
        let x = pos % W as u32;
        let cur_layer_idx = layers.len();
        let need_top = y > 0;
        let need_right = x < W as u32 - 1;
        let need_bottom = y < H as u32 - 1;
        let need_left = x > 0;

        let mut children: Vec<Node> = Vec::with_capacity(beam.len() * fanout);
        let mut child_used: Vec<UsedSet> = Vec::with_capacity(beam.len() * fanout);
        let mut child_psig: Vec<u64> = Vec::with_capacity(beam.len() * fanout); // parent's sig
        let mut cand_buf: Vec<(PieceId, Rotation, i8)> = Vec::with_capacity(legal.len());
        let mut picks_buf: Vec<(PieceId, Rotation, i8)> = Vec::with_capacity(fanout);

        for (bi, bref) in beam.iter().enumerate() {
            let top_edge = if need_top {
                resolve_neighbor(&layers, cur_layer_idx, bref.node_idx, pos - W as u32, &depth_of, &root)
                    .map(|(pid, r)| rotated_edge(piece_edges, pid, r, 2))
            } else { None };
            let right_edge = if need_right {
                resolve_neighbor(&layers, cur_layer_idx, bref.node_idx, pos + 1, &depth_of, &root)
                    .map(|(pid, r)| rotated_edge(piece_edges, pid, r, 3))
            } else { None };
            let bottom_edge = if need_bottom {
                resolve_neighbor(&layers, cur_layer_idx, bref.node_idx, pos + W as u32, &depth_of, &root)
                    .map(|(pid, r)| rotated_edge(piece_edges, pid, r, 0))
            } else { None };
            let left_edge = if need_left {
                resolve_neighbor(&layers, cur_layer_idx, bref.node_idx, pos - 1, &depth_of, &root)
                    .map(|(pid, r)| rotated_edge(piece_edges, pid, r, 1))
            } else { None };

            candidates_for_cell(&mut cand_buf, legal, &bref.used, top_edge, right_edge, bottom_edge, left_edge);
            if cand_buf.is_empty() { continue; }
            pick_tie_class(&mut cand_buf, tol, &mut rng, fanout, &mut picks_buf);
            for &(pid, r, gain) in &picks_buf {
                let mut nu = bref.used;
                nu.set(pid);
                children.push(Node { parent: bref.node_idx, pos: pos as u16, pid, rot: r, score: bref.score + gain as i32 });
                child_used.push(nu);
                child_psig.push(beam_sig[bi]);
            }
        }
        if children.is_empty() {
            let (h, n) = harvest_with_sig(&layers, &beam, &beam_sig, &root);
            return (h, n);
        }

        // Compute each child's trunk signature: if this depth completes the
        // signature region (cur_depth == sig_fixed_depth) fold in this cell's
        // placement; if already past, inherit parent's; if before, stays 0.
        let sig_now = sig_fixed_depth != u32::MAX && cur_depth >= sig_fixed_depth;
        let complete_now = cur_depth == sig_fixed_depth;

        // Stable score-desc order via counting sort (identical to run_one).
        let mut idx: Vec<usize> = {
            const MAX_SCORE: usize = 480;
            let mut counts = [0u32; MAX_SCORE + 1];
            for c in &children { counts[c.score as usize] += 1; }
            let mut offset = [0u32; MAX_SCORE + 1];
            let mut running = 0u32;
            for s in (0..=MAX_SCORE).rev() { offset[s] = running; running += counts[s]; }
            let mut out = vec![0usize; children.len()];
            let mut cursor = offset;
            for (i, c) in children.iter().enumerate() {
                let s = c.score as usize;
                out[cursor[s] as usize] = i; cursor[s] += 1;
            }
            out
        };
        let mut i = 0;
        while i < idx.len() {
            let mut j = i;
            while j < idx.len() && children[idx[j]].score == children[idx[i]].score { j += 1; }
            rng.shuffle(&mut idx[i..j]);
            i = j;
        }

        // Compute per-child signature (only meaningful once sig_now).
        let child_sig: Vec<u64> = (0..children.len()).map(|k| {
            if !sig_now { 0 }
            else if complete_now {
                // fold this cell into parent's sig
                let base = child_psig[k];
                let c = &children[k];
                mix_sig(base, c.pos, c.pid, c.rot)
            } else {
                child_psig[k]
            }
        }).collect();

        // Diversified truncation.
        let (kept, new_beam, new_layer, new_sig): (Vec<usize>, Vec<BeamRef>, Vec<Node>, Vec<u64>) = if !sig_now {
            // Before signature fixed: plain best-B (bit-identical to run_one).
            let mut ki = idx.clone();
            ki.truncate(beam_width);
            let mut nl = Vec::with_capacity(ki.len());
            let mut nb = Vec::with_capacity(ki.len());
            let mut ns = Vec::with_capacity(ki.len());
            for &k in &ki {
                nl.push(children[k]);
                nb.push(BeamRef { node_idx: (nl.len()-1) as u32, used: child_used[k], score: children[k].score, prior_sum: 0.0 });
                ns.push(child_sig[k]);
            }
            (ki, nb, nl, ns)
        } else {
            // Quota fill: walk idx (score-desc), keep a member if its trunk sig
            // has < cap members already kept; stop at beam_width. Track score
            // floor: once beam full, keep replacing only with members from NEW
            // sigs above floor -- simpler: single pass with a spillover list.
            use std::collections::HashMap;
            let best_score = children[idx[0]].score;
            let floor = best_score - div_tol;
            let mut per_sig: HashMap<u64, usize> = HashMap::new();
            let mut kept_idx: Vec<usize> = Vec::with_capacity(beam_width);
            let mut spill: Vec<usize> = Vec::new(); // over-quota but high score, fallback fill
            for &k in &idx {
                if kept_idx.len() >= beam_width { break; }
                let sg = child_sig[k];
                let cnt = per_sig.entry(sg).or_insert(0);
                if *cnt < cap {
                    *cnt += 1;
                    kept_idx.push(k);
                } else if children[k].score >= floor {
                    spill.push(k);
                }
            }
            // If quota starved us below beam_width, top up from spill (best first;
            // spill already in score-desc order because idx was).
            if kept_idx.len() < beam_width {
                for &k in &spill {
                    if kept_idx.len() >= beam_width { break; }
                    kept_idx.push(k);
                }
            }
            let mut nl = Vec::with_capacity(kept_idx.len());
            let mut nb = Vec::with_capacity(kept_idx.len());
            let mut ns = Vec::with_capacity(kept_idx.len());
            for &k in &kept_idx {
                nl.push(children[k]);
                nb.push(BeamRef { node_idx: (nl.len()-1) as u32, used: child_used[k], score: children[k].score, prior_sum: 0.0 });
                ns.push(child_sig[k]);
            }
            (kept_idx, nb, nl, ns)
        };
        let _ = kept;
        layers.push(new_layer);
        beam = new_beam;
        beam_sig = new_sig;
    }

    harvest_with_sig(&layers, &beam, &beam_sig, &root)
}

#[inline]
fn mix_sig(base: u64, pos: u16, pid: PieceId, rot: Rotation) -> u64 {
    // Order-sensitive FNV-ish mix of one placement into a running signature.
    let mut h = base ^ 0x9E3779B97F4A7C15;
    let v = (u64::from(pos) << 32) | (u64::from(pid) << 8) | u64::from(rot.as_u8());
    h = h.wrapping_mul(0x100000001B3);
    h ^= v;
    h = h.wrapping_mul(0x100000001B3);
    h ^= h >> 29;
    h
}

/// Harvest final beam AND count distinct trunk signatures present.
fn harvest_with_sig(
    layers: &[Vec<Node>],
    beam: &[BeamRef],
    beam_sig: &[u64],
    root: &RootBoard,
) -> (Vec<(i32, Box<[u16; N]>)>, usize) {
    use std::collections::HashSet;
    let last_layer = layers.len().wrapping_sub(1);
    let mut out: Vec<(i32, Box<[u16; N]>)> = beam
        .iter()
        .map(|bref| (bref.score, materialize(layers, last_layer, bref.node_idx, root)))
        .collect();
    out.sort_by_key(|&(score, _)| -score);
    let distinct: HashSet<u64> = beam_sig.iter().copied().collect();
    (out, distinct.len())
}

/// Neighbour resolution used inside the per-cell loop: `cur_layer_idx` is
/// the index the NEW layer will get once pushed (i.e. `layers.len()`
/// before pushing), so the node's own ancestry search starts at
/// `cur_layer_idx - 1` (the just-completed layer holding `bref.node_idx`).
fn resolve_neighbor(
    layers: &[Vec<Node>],
    cur_layer_idx: usize,
    node_idx: u32,
    pos: u32,
    depth_of: &[u32],
    root: &RootBoard,
) -> Option<(PieceId, Rotation)> {
    if depth_of[pos as usize] == u32::MAX {
        let v = root.cells[pos as usize];
        return if v == EMPTY { None } else { Some(unpack(v)) };
    }
    // Not yet placed if its depth is >= the depth about to be assigned to
    // the CURRENT cell (cur_layer_idx+1, since layers[cur_layer_idx] does
    // not exist yet and will hold depth cur_layer_idx+1).
    if depth_of[pos as usize] as usize >= cur_layer_idx + 1 {
        return None;
    }
    if cur_layer_idx == 0 {
        return None; // no completed layers yet -- pos must be a hint, handled above
    }
    resolve_cell(layers, cur_layer_idx - 1, node_idx, pos, depth_of, root)
}

fn materialize(layers: &[Vec<Node>], mut layer_idx: usize, mut node_idx: u32, root: &RootBoard) -> Box<[u16; N]> {
    let mut board = root.cells.clone();
    if layers.is_empty() {
        return board;
    }
    loop {
        let node = layers[layer_idx][node_idx as usize];
        board[node.pos as usize] = pack(node.pid, node.rot);
        if layer_idx == 0 {
            break;
        }
        node_idx = node.parent;
        layer_idx -= 1;
    }
    board
}

fn score_partial(board: &[u16; N], piece_edges: &[[u8; 4]]) -> i32 {
    let mut m = 0i32;
    for y in 0..H {
        for x in 0..W {
            let p = y * W + x;
            let v = board[p];
            if v == EMPTY {
                continue;
            }
            let (pid, rot) = (v >> 2, Rotation::from_u8((v & 3) as u8).unwrap());
            if x + 1 < W {
                let vr = board[p + 1];
                if vr != EMPTY {
                    let e = rotated_edge(piece_edges, pid, rot, 1);
                    let er = rotated_edge(piece_edges, vr >> 2, Rotation::from_u8((vr & 3) as u8).unwrap(), 3);
                    if e != BORDER && e == er {
                        m += 1;
                    }
                }
            }
            if y + 1 < H {
                let vb = board[p + W];
                if vb != EMPTY {
                    let e = rotated_edge(piece_edges, pid, rot, 2);
                    let eb = rotated_edge(piece_edges, vb >> 2, Rotation::from_u8((vb & 3) as u8).unwrap(), 0);
                    if e != BORDER && e == eb {
                        m += 1;
                    }
                }
            }
        }
    }
    m
}

/// Write the harvested final beam to `<dir>/beam_seed<S>_beam<B>_order<O>.jsonl`,
/// one JSON line per final-beam member (rank 0 = best). Each line:
/// `{"seed":S,"beam":B,"order":O,"rank":i,"score":sc,"board":[[pid,rot],...256]}`
/// with unplaced (EMPTY) cells encoded as `[-1,-1]`.
fn write_final_beam_jsonl(
    dir: &str,
    seed: u64,
    beam_width: usize,
    order_name: &str,
    harvest: &[(i32, Box<[u16; N]>)],
) {
    std::fs::create_dir_all(dir).expect("create emit-final-beam dir");
    let path = std::path::Path::new(dir).join(format!("beam_seed{seed}_beam{beam_width}_order{order_name}.jsonl"));
    let mut buf = String::new();
    for (rank, (score, board)) in harvest.iter().enumerate() {
        let cells: Vec<[i32; 2]> = board
            .iter()
            .map(|&v| {
                if v == EMPTY {
                    [-1, -1]
                } else {
                    let (pid, r) = unpack(v);
                    [i32::from(pid), i32::from(r.as_u8())]
                }
            })
            .collect();
        let line = serde_json::json!({
            "seed": seed,
            "beam": beam_width,
            "order": order_name,
            "rank": rank,
            "score": score,
            "board": cells,
        });
        buf.push_str(&serde_json::to_string(&line).expect("serialize beam member"));
        buf.push('\n');
    }
    std::fs::write(&path, buf).expect("write final-beam jsonl");
    eprintln!("# emitted final beam ({} members) -> {}", harvest.len(), path.display());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: producer_trie <puzzle.csv> --order O --beam B --tol T [--seed S | --seed-lo A --seed-hi Z] [--threads N] [--emit-best out.json] [--emit-final-beam dir]");
        eprintln!("  --threads 1 (default): historical serial search, bit-identical to producer.");
        eprintln!("  --threads N (0 = all cores): parallel; deterministic for a given seed and equal");
        eprintln!("              across thread counts, but NOT bit-identical to the serial stream.");
        std::process::exit(1);
    }
    let csv_path = PathBuf::from(&args[1]);
    let mut order = Order::RowMajor;
    let mut beam_width: usize = 8;
    let mut tol: i8 = 0;
    let mut seed_lo: u64 = 1;
    let mut seed_hi: u64 = 1;
    let mut emit_best: Option<String> = None;
    let mut emit_final_beam: Option<String> = None;
    let mut div_row: usize = 0; // 0 = diversify off
    let mut div_cap: usize = 1;
    let mut div_tol: i32 = 3;
    let mut prior_file: Option<String> = None;
    let mut prior_alpha: f32 = 0.0;
    let mut threads: usize = 1; // 1 = serial/bit-identical default

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--order" => { order = parse_order(&args[i + 1]); i += 2; }
            "--beam" => { beam_width = args[i + 1].parse().unwrap(); i += 2; }
            "--tol" => { tol = args[i + 1].parse().unwrap(); i += 2; }
            "--seed" => { seed_lo = args[i + 1].parse().unwrap(); seed_hi = seed_lo; i += 2; }
            "--seed-lo" => { seed_lo = args[i + 1].parse().unwrap(); i += 2; }
            "--seed-hi" => { seed_hi = args[i + 1].parse().unwrap(); i += 2; }
            "--emit-best" => { emit_best = Some(args[i + 1].clone()); i += 2; }
            "--emit-final-beam" => { emit_final_beam = Some(args[i + 1].clone()); i += 2; }
            "--diversify-row" => { div_row = args[i + 1].parse().unwrap(); i += 2; }
            "--diversify-cap" => { div_cap = args[i + 1].parse().unwrap(); i += 2; }
            "--diversify-tol" => { div_tol = args[i + 1].parse().unwrap(); i += 2; }
            "--prior-file" => { prior_file = Some(args[i + 1].clone()); i += 2; }
            "--prior-alpha" => { prior_alpha = args[i + 1].parse().unwrap(); i += 2; }
            "--threads" => { threads = args[i + 1].parse().unwrap(); i += 2; }
            other => { eprintln!("unknown arg {other}"); std::process::exit(1); }
        }
    }

    // --threads 0 = "use every core". Resolve it to a concrete count now so the
    // rest of the run (and the log line) talks about a real number.
    if threads == 0 {
        threads = std::thread::available_parallelism().map_or(1, std::num::NonZero::get);
    }
    // Build our own pool rather than touching rayon's global one: the pool size
    // is then exactly what --threads asked for, regardless of RAYON_NUM_THREADS
    // or any other rayon user in the process.
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("build rayon pool");
    if threads > 1 {
        eprintln!(
            "# --threads {threads}: parallel search. Deterministic per seed and identical across \
             thread counts, but NOT bit-identical to --threads 1 (the serial RNG stream)."
        );
    }

    let prior: Option<Prior> = prior_file.as_deref().map(|p| {
        eprintln!("# loading prior from {p} (alpha={prior_alpha})");
        load_prior(p)
    });
    let prior_ref = prior.as_ref();

    let (puzzle, hints) = load_puzzle_with_hints(&csv_path).expect("load puzzle");
    let legal_table = build_legal_table(&puzzle);
    let piece_edges = build_piece_edges(&puzzle);

    let order_name = match order {
        Order::RowMajor => "rowmajor".to_string(),
        Order::Spiral => "spiral".to_string(),
        Order::BorderFirst => "borderfirst".to_string(),
        Order::Comb { split_row } => format!("comb{split_row}"),
    };
    let order_name = order_name.as_str();

    let mut scores: Vec<i32> = Vec::new();
    let mut best_overall: (i32, Option<Box<[u16; N]>>) = (-1, None);
    println!("order,beam,tol,seed,score,layers,peak_layer_len");

    let n_seeds = (seed_hi - seed_lo + 1) as usize;
    // Two ways to spend cores, and they compete for the same pool:
    //
    //   * SEED-PARALLEL: run whole seeds concurrently, each one internally
    //     serial. Perfectly independent, no synchronization per layer, and each
    //     seed stays bit-identical to --threads 1 -- strictly the better deal
    //     whenever there are enough seeds to fill the pool.
    //   * BEAM-PARALLEL: one seed at a time, cores split across the beam. The
    //     only option that makes a SINGLE run faster (and the benchmark harness
    //     runs exactly one seed), but it pays a sync barrier per layer and
    //     departs from the serial stream.
    //
    // So: prefer seed-parallel when there's more than one seed, and fall back to
    // beam-parallel when a single seed has to use the whole machine. Nesting both
    // would just oversubscribe the pool.
    let seed_parallel = threads > 1 && n_seeds > 1 && div_row == 0 && emit_final_beam.is_none();
    let inner_threads = if seed_parallel { 1 } else { threads };

    if seed_parallel {
        // Each seed is independent and internally serial, so this is a plain map
        // -- collected into a Vec that rayon returns in INPUT order, then printed
        // in seed order. Row order on stdout is therefore identical to the serial
        // run's; only the wall-clock changes.
        eprintln!("# {n_seeds} seeds across {threads} threads (per-seed results bit-identical to --threads 1)");
        let results: Vec<(u64, i32, Box<[u16; N]>, usize, usize)> = pool.install(|| {
            (seed_lo..=seed_hi)
                .into_par_iter()
                .map(|seed| {
                    let (score, board, n_layers, peak) = run_one(
                        &hints, order, beam_width, tol, seed, &legal_table, &piece_edges,
                        prior_ref, prior_alpha, 1,
                    );
                    (seed, score, board, n_layers, peak)
                })
                .collect()
        });
        for (seed, score, board, n_layers, peak_layer_len) in results {
            println!("{order_name},{beam_width},{tol},{seed},{score},{n_layers},{peak_layer_len}");
            scores.push(score);
            if score > best_overall.0 {
                best_overall = (score, Some(board));
            }
        }
        finish(&mut scores, order_name, beam_width, tol, best_overall, emit_best.as_deref(), &puzzle);
        return;
    }

    for seed in seed_lo..=seed_hi {
        if div_row > 0 {
            let (harvest, trunk_count) = run_one_diversify(
                &hints, order, beam_width, tol, seed, &legal_table, &piece_edges,
                div_row, div_cap, div_tol,
            );
            let score = harvest.first().map(|(s, _)| *s).unwrap_or(-1);
            println!("{order_name},{beam_width},{tol},{seed},{score},div_row{div_row},cap{div_cap},trunks{trunk_count}");
            scores.push(score);
            if score > best_overall.0 {
                best_overall = (score, Some(harvest[0].1.clone()));
            }
            if let Some(dir) = emit_final_beam.as_deref() {
                write_final_beam_jsonl(dir, seed, beam_width, order_name, &harvest);
            }
            continue;
        }
        let (score, board, n_layers, peak_layer_len) = pool.install(|| {
            run_one(
                &hints, order, beam_width, tol, seed, &legal_table, &piece_edges, prior_ref,
                prior_alpha, inner_threads,
            )
        });
        println!("{order_name},{beam_width},{tol},{seed},{score},{n_layers},{peak_layer_len}");
        scores.push(score);
        if score > best_overall.0 {
            best_overall = (score, Some(board));
        }

        if let Some(dir) = emit_final_beam.as_deref() {
            let harvest = run_one_harvest(&hints, order, beam_width, tol, seed, &legal_table, &piece_edges);
            write_final_beam_jsonl(dir, seed, beam_width, order_name, &harvest);
        }
    }

    finish(&mut scores, order_name, beam_width, tol, best_overall, emit_best.as_deref(), &puzzle);
}

/// Print the run summary and, if asked, write the best board's bucas URL.
/// Shared by the seed-parallel and sequential paths so they cannot drift.
fn finish(
    scores: &mut Vec<i32>,
    order_name: &str,
    beam_width: usize,
    tol: i8,
    best_overall: (i32, Option<Box<[u16; N]>>),
    emit_best: Option<&str>,
    puzzle: &Puzzle,
) {
    scores.sort_unstable();
    let n = scores.len();
    let min = scores[0];
    let max = scores[n - 1];
    let median = if n % 2 == 1 { scores[n / 2] } else { (scores[n / 2 - 1] + scores[n / 2]) / 2 };
    eprintln!("# SUMMARY order={order_name} beam={beam_width} tol={tol} n={n} min={min} median={median} max={max}");

    if let Some(path) = emit_best {
        let best_board = best_overall.1.expect("at least one seed run");
        let mut b = Board::empty(puzzle);
        for (pos, &v) in best_board.iter().enumerate() {
            if v != EMPTY {
                let (pid, r) = unpack(v);
                b.place(pos as u32, pid, r);
            }
        }
        let score = best_overall.0.max(0) as u32;
        board_to_doc(puzzle, &b, "size_16_official_eternity", score)
            .write_json(path)
            .expect("write board json");
        eprintln!("# emitted best board (score={}) -> {path}", best_overall.0);
    }
}
