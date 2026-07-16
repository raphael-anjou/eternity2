// MOSAIC — window/block composition solver for Eternity II.
//
// Composes EXACT block fills (per-block MaxScore via small DFS+B&B) into a full
// board, with scarcity-aware piece reservation (the piece-theft fix) and
// SOFT boundaries (blocks never go infeasible — pay for mismatches). Hints are
// hard pins (strict-canonical compatible).
//
// The Python PoC (scripts/v206_mosaic) reaches 448/480 from scratch; this Rust
// port runs orders of magnitude more block-solves to enable block-backtracking
// and long multi-restart runs.
//
// Usage:
//   mosaic [--bs 4] [--reserve 0.08] [--order row|spiral|corners] [--hints]
//          [--seed N] [--restarts R] [--out DIR]

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

use eternity2_benchmark::loader::load_puzzle_with_hints;
use eternity2_core::{Board, Color, Puzzle, Rotation, BORDER};
use eternity2_export::{save_board, viewer_url, BoardMetadata};

const SIDE_N: usize = 0;
const SIDE_E: usize = 1;
const SIDE_S: usize = 2;
const SIDE_W: usize = 3;

#[inline]
fn rot_edges(base: [Color; 4], r: u8) -> [Color; 4] {
    // new[i] = base[(i - r) mod 4]  (matches core Edges::rotated; verified)
    let r = r as usize;
    [
        base[(SIDE_N + 4 - r) % 4],
        base[(SIDE_E + 4 - r) % 4],
        base[(SIDE_S + 4 - r) % 4],
        base[(SIDE_W + 4 - r) % 4],
    ]
}

struct Inst {
    size: usize,
    np: usize,
    base: Vec<[Color; 4]>,            // base edges [N,E,S,W] per piece
    border_sides: Vec<Vec<usize>>,    // for each piece, sorted set of sides==BORDER (base)
    scarcity: Vec<u32>,               // piece scarcity (min (N,W)-pair server count)
    hints: HashMap<usize, (usize, u8)>, // pos -> (piece, rot)
}

impl Inst {
    fn from_puzzle(puzzle: &Puzzle, hints_in: &[(usize, usize, u8)]) -> Self {
        let size = puzzle.width as usize;
        let np = puzzle.pieces().len();
        let mut base = vec![[0 as Color; 4]; np];
        for (i, p) in puzzle.pieces().iter().enumerate() {
            base[i] = p.edges.as_array();
        }
        // scarcity: per (N,W) interior pair, distinct pieces presenting it
        let mut supply: HashMap<(Color, Color), std::collections::HashSet<usize>> = HashMap::new();
        for p in 0..np {
            for r in 0..4u8 {
                let e = rot_edges(base[p], r);
                if e[SIDE_N] != BORDER && e[SIDE_W] != BORDER {
                    supply.entry((e[SIDE_N], e[SIDE_W])).or_default().insert(p);
                }
            }
        }
        let mut scarcity = vec![99u32; np];
        for p in 0..np {
            let mut b = 99u32;
            for r in 0..4u8 {
                let e = rot_edges(base[p], r);
                if e[SIDE_N] != BORDER && e[SIDE_W] != BORDER {
                    let s = supply[&(e[SIDE_N], e[SIDE_W])].len() as u32;
                    if s < b { b = s; }
                }
            }
            scarcity[p] = b;
        }
        let border_sides: Vec<Vec<usize>> = (0..np)
            .map(|p| (0..4).filter(|&s| base[p][s] == BORDER).collect())
            .collect();
        let hints = hints_in.iter().map(|&(pos, pid, rot)| (pos, (pid, rot))).collect();
        Inst { size, np, base, border_sides, scarcity, hints }
    }

    #[inline]
    fn must_border(&self, cell: usize) -> Vec<usize> {
        let (x, y) = (cell % self.size, cell / self.size);
        let mut s = Vec::new();
        if y == 0 { s.push(SIDE_N); }
        if y == self.size - 1 { s.push(SIDE_S); }
        if x == 0 { s.push(SIDE_W); }
        if x == self.size - 1 { s.push(SIDE_E); }
        s.sort_unstable(); // compare against ascending border-side sets
        s
    }
}

// A candidate placement at a cell: (piece, rot, edges)
#[derive(Clone, Copy)]
struct Cand { p: usize, r: u8, e: [Color; 4] }

// Soft boundary demand on a cell: list of (side, color) the cell SHOULD match.
type Boundary = HashMap<usize, Vec<(usize, Color)>>;

/// Solve one block to MaxScore (max internal + boundary matches) via DFS+B&B.
/// `avail`: bool per piece (usable). `pins`: cell->(p,r) hard.
/// Returns (assignment cell->(p,r), matched_total) or None if a cell has no cand.
fn solve_block(
    inst: &Inst,
    cells: &[usize],
    avail: &[bool],
    boundary: &Boundary,
    pins: &HashMap<usize, (usize, u8)>,
    seed: u64,
) -> Option<(Vec<(usize, usize, u8)>, i32)> {
    let n = cells.len();
    let cellset: std::collections::HashSet<usize> = cells.iter().copied().collect();
    // deterministic per-cell shuffle key (SplitMix64-ish) for seed diversity:
    // permuting candidate iteration order yields different equally-optimal block
    // fills -> different piece consumption -> structurally distinct basins.
    let mix = |mut z: u64| -> u64 {
        z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    };
    // candidate list per cell-index
    let mut cand: Vec<Vec<Cand>> = Vec::with_capacity(n);
    for &c in cells {
        if let Some(&(pid, rot)) = pins.get(&c) {
            cand.push(vec![Cand { p: pid, r: rot, e: rot_edges(inst.base[pid], rot) }]);
            continue;
        }
        let mb = inst.must_border(c);
        // a piece can fit cell c iff its NUMBER of border sides matches |mb| AND
        // some rotation lands the BORDER sides exactly on mb. (Don't pre-filter on
        // BASE border sides — rotation moves them.)
        let mut lst = Vec::new();
        for p in 0..inst.np {
            if !avail[p] { continue; }
            if inst.border_sides[p].len() != mb.len() { continue; } // class size (corner/edge/interior)
            for r in 0..4u8 {
                let e = rot_edges(inst.base[p], r);
                let bs: Vec<usize> = (0..4).filter(|&s| e[s] == BORDER).collect();
                if bs != mb { continue; }
                lst.push(Cand { p, r, e });
            }
        }
        if lst.is_empty() { return None; }
        if seed != 0 {
            // shuffle candidate order deterministically by (seed, cell, piece, rot)
            lst.sort_by_key(|cd| mix(seed ^ ((c as u64) << 20) ^ ((cd.p as u64) << 2) ^ cd.r as u64));
        }
        cand.push(lst);
    }
    // internal edges among block cells: (ci, side_a, cj, side_b)
    // We index cells by their block index. Build adjacency on block indices.
    let pos_of: HashMap<usize, usize> = cells.iter().enumerate().map(|(i, &c)| (c, i)).collect();
    let mut internal: Vec<(usize, usize, usize, usize)> = Vec::new();
    for (i, &c) in cells.iter().enumerate() {
        let (x, y) = (c % inst.size, c / inst.size);
        if x + 1 < inst.size && cellset.contains(&(c + 1)) {
            internal.push((i, SIDE_E, pos_of[&(c + 1)], SIDE_W));
        }
        if y + 1 < inst.size && cellset.contains(&(c + inst.size)) {
            internal.push((i, SIDE_S, pos_of[&(c + inst.size)], SIDE_N));
        }
    }
    // boundary demands per block index
    let mut bnd: Vec<Vec<(usize, Color)>> = vec![Vec::new(); n];
    for (i, &c) in cells.iter().enumerate() {
        if let Some(d) = boundary.get(&c) { bnd[i] = d.clone(); }
    }
    // max possible additional matches from cell index i onward (for B&B):
    // each internal edge with max endpoint >= i can still match; plus boundary.
    // Precompute suffix max-bound.
    let total_int = internal.len();
    let total_bnd: usize = bnd.iter().map(|v| v.len()).sum();
    let max_possible = (total_int + total_bnd) as i32;

    // DFS over block indices 0..n in order.
    let mut used_local: Vec<bool> = vec![false; inst.np];
    let mut chosen: Vec<usize> = vec![usize::MAX; n]; // candidate index per cell
    let mut best: i32 = -1;
    let mut best_choice: Vec<usize> = vec![0; n];

    // For bounding, precompute for each block index, the count of internal edges
    // whose BOTH endpoints are >= that index would still be open — simpler: bound
    // = matched_so_far + (edges not yet both-decided) + (boundary not yet decided).
    // We approximate with max_possible - decided_matches_lost; cheap admissible.

    // Edge ownership for incremental scoring: edge counted when its LATER endpoint
    // is placed.
    // Map: for block index j, edges that close at j (both endpoints <= j and one == j)
    let mut edges_closing_at: Vec<Vec<(usize, usize, usize, usize)>> = vec![Vec::new(); n];
    for &(a, sa, b, sb) in &internal {
        let later = a.max(b);
        edges_closing_at[later].push((a, sa, b, sb));
    }

    #[allow(clippy::too_many_arguments)]
    fn dfs(
        inst: &Inst,
        idx: usize,
        n: usize,
        cand: &Vec<Vec<Cand>>,
        bnd: &Vec<Vec<(usize, Color)>>,
        edges_closing_at: &Vec<Vec<(usize, usize, usize, usize)>>,
        chosen: &mut Vec<usize>,
        used_local: &mut Vec<bool>,
        matched: i32,
        remaining_bound: i32,
        best: &mut i32,
        best_choice: &mut Vec<usize>,
        nodes: &mut u64,
        node_cap: u64,
    ) {
        if *nodes >= node_cap { return; }
        if matched + remaining_bound <= *best {
            return;
        }
        if idx == n {
            if matched > *best {
                *best = matched;
                for i in 0..n { best_choice[i] = chosen[i]; }
            }
            return;
        }
        // remaining edges/boundary that will be decided from idx onward
        // recompute a cheap upper bound for the child: subtract edges closing
        // before reaching them — but we pass remaining_bound decremented per level.
        let cl = &edges_closing_at[idx];
        let nb_here = bnd[idx].len() as i32;
        let edges_here = cl.len() as i32;
        let child_rem = remaining_bound - edges_here - nb_here; // these get resolved at this level
        for ci in 0..cand[idx].len() {
            let c = cand[idx][ci];
            if used_local[c.p] { continue; }
            // score gained at this level: boundary matches + internal edges closing here
            let mut gain = 0i32;
            for &(side, col) in &bnd[idx] {
                if c.e[side] == col { gain += 1; }
            }
            // for edges closing at idx, the other endpoint is already chosen
            let mut ok_edges = 0i32;
            for &(a, sa, b, sb) in cl {
                let (me_side, other_idx, other_side) = if a == idx { (sa, b, sb) } else { (sb, a, sa) };
                let oc = &cand[other_idx][chosen[other_idx]];
                if c.e[me_side] == oc.e[other_side] { ok_edges += 1; }
            }
            gain += ok_edges;
            *nodes += 1;
            chosen[idx] = ci;
            used_local[c.p] = true;
            dfs(inst, idx + 1, n, cand, bnd, edges_closing_at, chosen, used_local,
                matched + gain, child_rem, best, best_choice, nodes, node_cap);
            used_local[c.p] = false;
            if *nodes >= node_cap { break; }
        }
        chosen[idx] = usize::MAX;
    }

    // node cap: keep the FIRST block (free boundary, huge tree) tractable while
    // staying exact for constrained later blocks (which finish well under the cap).
    let node_cap: u64 = std::env::var("MOSAIC_NODECAP").ok()
        .and_then(|s| s.parse().ok()).unwrap_or(50_000_000);
    let mut nodes: u64 = 0;
    dfs(inst, 0, n, &cand, &bnd, &edges_closing_at, &mut chosen, &mut used_local,
        0, max_possible, &mut best, &mut best_choice, &mut nodes, node_cap);

    if best < 0 {
        if std::env::var("MOSAIC_DEBUG").is_ok() {
            let counts: Vec<usize> = cand.iter().map(|v| v.len()).collect();
            eprintln!("solve_block FAIL: n={n} cand_counts={:?} internal={} bnd={}",
                counts, internal.len(), total_bnd);
        }
        return None;
    }
    let assign: Vec<(usize, usize, u8)> = (0..n)
        .map(|i| { let c = cand[i][best_choice[i]]; (cells[i], c.p, c.r) })
        .collect();
    Some((assign, best))
}

fn block_order(nb: usize, order: &str) -> Vec<(usize, usize)> {
    let mut cells: Vec<(usize, usize)> = (0..nb).flat_map(|br| (0..nb).map(move |bc| (br, bc))).collect();
    match order {
        "spiral" => {
            let mut res = Vec::new();
            let mut seen = std::collections::HashSet::new();
            let (mut lo, mut hi) = (0i64, nb as i64 - 1);
            while lo <= hi {
                for c in lo..=hi {
                    for rc in [(lo, c), (hi, c)] {
                        let rc = (rc.0 as usize, rc.1 as usize);
                        if seen.insert(rc) { res.push(rc); }
                    }
                }
                for r in lo..=hi {
                    for rc in [(r, lo), (r, hi)] {
                        let rc = (rc.0 as usize, rc.1 as usize);
                        if seen.insert(rc) { res.push(rc); }
                    }
                }
                lo += 1; hi -= 1;
            }
            res
        }
        "corners" => {
            let center = (nb as f64 - 1.0) / 2.0;
            cells.sort_by(|a, b| {
                let da = (a.0 as f64 - center).abs() + (a.1 as f64 - center).abs();
                let db = (b.0 as f64 - center).abs() + (b.1 as f64 - center).abs();
                db.partial_cmp(&da).unwrap()
            });
            cells
        }
        _ => cells,
    }
}

#[allow(clippy::too_many_arguments)]
fn run(inst: &Inst, puzzle: &Puzzle, bs: usize, order: &str, reserve_frac: f64, use_hints: bool, seed: u64) -> (i32, Board, f64) {
    let size = inst.size;
    let nb = size / bs;
    let blocks = block_order(nb, order);
    let hintmap: HashMap<usize, (usize, u8)> = if use_hints { inst.hints.clone() } else { HashMap::new() };
    let hint_pieces: std::collections::HashSet<usize> = hintmap.values().map(|&(p, _)| p).collect();
    let mut place: Vec<Option<(usize, u8)>> = vec![None; size * size];
    let mut pe: Vec<Option<[Color; 4]>> = vec![None; size * size];
    let mut used = vec![false; inst.np];
    for &p in &hint_pieces { used[p] = true; } // hint pieces only placed via pins
    let t0 = Instant::now();
    let nblk = blocks.len();
    for (bi, &(br, bc)) in blocks.iter().enumerate() {
        let cells: Vec<usize> = (0..bs).flat_map(|dy| (0..bs).map(move |dx| (dy, dx)))
            .map(|(dy, dx)| (br * bs + dy) * size + (bc * bs + dx)).collect();
        let cellset: std::collections::HashSet<usize> = cells.iter().copied().collect();
        // boundary from placed neighbors outside block (all 4 sides)
        let mut boundary: Boundary = HashMap::new();
        for &c in &cells {
            let (x, y) = (c % size, c / size);
            let mut v = Vec::new();
            if y > 0 && !cellset.contains(&(c - size)) { if let Some(e) = pe[c - size] { v.push((SIDE_N, e[SIDE_S])); } }
            if y + 1 < size && !cellset.contains(&(c + size)) { if let Some(e) = pe[c + size] { v.push((SIDE_S, e[SIDE_N])); } }
            if x > 0 && !cellset.contains(&(c - 1)) { if let Some(e) = pe[c - 1] { v.push((SIDE_W, e[SIDE_E])); } }
            if x + 1 < size && !cellset.contains(&(c + 1)) { if let Some(e) = pe[c + 1] { v.push((SIDE_E, e[SIDE_W])); } }
            if !v.is_empty() { boundary.insert(c, v); }
        }
        let pins: HashMap<usize, (usize, u8)> = cells.iter().filter_map(|&c| hintmap.get(&c).map(|&pr| (c, pr))).collect();
        // pool = available non-hint pieces, minus reserved scarcest (non-final blocks)
        let mut avail = used.iter().map(|&u| !u).collect::<Vec<bool>>();
        for &p in &hint_pieces { avail[p] = false; }
        if reserve_frac > 0.0 && bi + 1 < nblk {
            let mut pool: Vec<usize> = (0..inst.np).filter(|&p| avail[p]).collect();
            pool.sort_by_key(|&p| inst.scarcity[p]); // scarcest first
            let nres = (pool.len() as f64 * reserve_frac) as usize;
            for &p in pool.iter().take(nres) { avail[p] = false; }
        }
        let mut res = solve_block(inst, &cells, &avail, &boundary, &pins, seed);
        if res.is_none() {
            // fallback: full available pool (reservation over-constrained)
            let mut avail2 = used.iter().map(|&u| !u).collect::<Vec<bool>>();
            for &p in &hint_pieces { avail2[p] = false; }
            res = solve_block(inst, &cells, &avail2, &boundary, &pins, seed);
        }
        if res.is_none() {
            // diagnose: which cell has no available border-class piece?
            let avail2 = used.iter().map(|&u| !u).collect::<Vec<bool>>();
            for &c in &cells {
                if pins.contains_key(&c) { continue; }
                let mb = inst.must_border(c);
                let cnt = (0..inst.np).filter(|&p| avail2[p]
                    && (0..4u8).any(|r| {
                        let e = rot_edges(inst.base[p], r);
                        (0..4).filter(|&s| e[s]==BORDER).collect::<Vec<_>>() == mb
                    })).count();
                if cnt == 0 {
                    eprintln!("DEAD: block ({br},{bc}) cell {c} border-class {:?} has 0 available pieces", mb);
                }
            }
        }
        let (assign, _m) = res.expect("block infeasible even with full pool");
        for (c, p, r) in assign {
            place[c] = Some((p, r));
            pe[c] = Some(rot_edges(inst.base[p], r));
            used[p] = true;
        }
    }
    // build Board + score
    let mut board = Board::empty(puzzle);
    for c in 0..size * size {
        if let Some((p, r)) = place[c] {
            board.place(c as u32, p as u16, Rotation::from_u8(r).unwrap());
        }
    }
    let mut matched = 0i32;
    for c in 0..size * size {
        let (x, y) = (c % size, c / size);
        if x + 1 < size {
            if let (Some(a), Some(b)) = (pe[c], pe[c + 1]) { if a[SIDE_E] == b[SIDE_W] { matched += 1; } }
        }
        if y + 1 < size {
            if let (Some(a), Some(b)) = (pe[c], pe[c + size]) { if a[SIDE_S] == b[SIDE_N] { matched += 1; } }
        }
    }
    (matched, board, t0.elapsed().as_secs_f64())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut bs = 4usize;
    let mut reserve = 0.08f64;
    let mut order = "row".to_string();
    let mut use_hints = false;
    let mut out = String::new();
    let mut seed = 0u64;
    let mut puzzle_path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/puzzles/size_16_official_eternity.csv"));
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--bs" => { bs = args[i + 1].parse().unwrap(); i += 2; }
            "--reserve" => { reserve = args[i + 1].parse().unwrap(); i += 2; }
            "--order" => { order = args[i + 1].clone(); i += 2; }
            "--hints" => { use_hints = true; i += 1; }
            "--seed" => { seed = args[i + 1].parse().unwrap(); i += 2; }
            "--out" => { out = args[i + 1].clone(); i += 2; }
            "--puzzle" => { puzzle_path = PathBuf::from(&args[i + 1]); i += 2; }
            other => { eprintln!("unknown arg {other}"); i += 1; }
        }
    }
    let (puzzle, hints) = load_puzzle_with_hints(&puzzle_path).expect("load puzzle");
    let hv: Vec<(usize, usize, u8)> = hints.hints.iter()
        .map(|h| (h.position as usize, h.piece_id as usize, h.rotation.as_u8()))
        .collect();
    let inst = Inst::from_puzzle(&puzzle, &hv);
    eprintln!("MOSAIC-rust bs={bs} reserve={reserve} order={order} hints={use_hints} seed={seed} size={} np={}", inst.size, inst.np);
    let (matched, board, secs) = run(&inst, &puzzle, bs, &order, reserve, use_hints, seed);
    // hints obeyed
    let mut ho = 0;
    for &(pos, pid, rot) in &hv {
        if board.get(pos as u32) == Some((pid as u16, Rotation::from_u8(rot).unwrap())) { ho += 1; }
    }
    println!("matched={matched}/480 hints_obeyed={ho}/5 solve={secs:.1}s");
    let url = viewer_url(&puzzle, &board, "mosaic");
    println!("viewer: {url}");
    if !out.is_empty() {
        let dir = PathBuf::from(&out);
        std::fs::create_dir_all(&dir).ok();
        let path = dir.join(format!("mosaic_bs{bs}_rf{reserve}_{order}_s{seed}_m{matched}{}.json", if use_hints {"_HINTED"} else {""}));
        let md = BoardMetadata { source: Some(format!("mosaic bs={bs} rf={reserve} order={order}")), ..Default::default() };
        save_board(&path, &puzzle, &board, &md).expect("save");
        println!("saved -> {}", path.display());
    }
}
