//! Generate hint-geometry variants for the **Hint Study** lab section
//! (eternity2.dev/research/lab/experiments/raphael-anjou/hint-study).
//!
//! One solved N×N board is generated to Eternity II's colour recipe (border
//! colours confined to the frame band + interior colours). Because
//! `generate_solved_framed` lays piece `i` at cell `i` at rotation 0, the
//! identity placement IS the solution, so a hint for any cell `pos` is just
//! `{pos, piece: pos, rot: 0}`. We relabel piece IDs by a seeded permutation so
//! a naive solver cannot walk the identity, then emit several hint sets that
//! differ only in geometry / count, as site-schema JSON (consumed by the
//! dfs-study `run_dfs`) and producer CSV (consumed by `producer_trie`).
//!
//! Parametric in board size and colour count so the study can measure the
//! placement effect as a function of N (the scaling axis). The colour recipe
//! defaults follow the density-preserving rule derived in the study method:
//! border colours target multiplicity ~12, interior ~24, matching E2 exactly at
//! N=16 (5 border + ~17 interior) while holding constraint density fixed across
//! sizes so the size trend is not confounded by density.
//!
//! Usage:
//!   hint_variants --out <dir> [--size N] [--colors C] [--seed S]
//!   (legacy positional form `hint_variants <out_dir> [seed]` still works, N=16)

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use eternity2_engine::generator::{frame_color_count, interior_edge_count};
use eternity2_engine::{generate_solved_framed, Puzzle, BORDER};

/// The board size for a run, threaded through the layout helpers (which used to
/// close over a `SIZE` const). Kept in one struct so every geometry function is
/// explicit about the grid it addresses.
#[derive(Clone, Copy)]
struct Grid {
    n: usize,
}

impl Grid {
    fn cell(&self, x: usize, y: usize) -> usize {
        y * self.n + x
    }
    fn xy(&self, cell: usize) -> (usize, usize) {
        (cell % self.n, cell / self.n)
    }
    fn is_border(&self, cell: usize) -> bool {
        let (x, y) = self.xy(cell);
        x == 0 || y == 0 || x == self.n - 1 || y == self.n - 1
    }
    fn ncells(&self) -> usize {
        self.n * self.n
    }
}

/// Density-preserving colour recipe for size `n`: returns `colors` (total) such
/// that `frame_color_count(colors)` border colours each land near multiplicity
/// 12 and the interior colours near multiplicity 24 — matching E2's structure at
/// n=16 while holding constraint density constant across sizes. See the study
/// method page for the derivation. The engine clamps `colors` to
/// `max_colors(n)`, and `frame_color_count` caps border colours at 5.
fn faithful_colors(n: usize) -> u8 {
    let n_u8 = n as u8;
    let frame = frame_band_slot_count(n) as f64;
    let interior = (interior_edge_count(n_u8) as usize - frame_band_slot_count(n)) as f64;
    // Target multiplicities from E2 (frame 60/5=12, interior 420/17≈24.7).
    let bc = ((frame / 12.0).round() as usize).max(1).min(5);
    let ic = ((interior / 24.0).round() as usize).max(1);
    (bc + ic).min(22) as u8
}

/// Count of along-frame seams (border band) on an n×n board — mirrors the
/// engine's `slot_is_frame_band` partition, recomputed here so the study's
/// recipe derivation is self-contained.
fn frame_band_slot_count(n: usize) -> usize {
    let s = n;
    let n_vert = s * (s - 1);
    let mut frame = 0;
    for i in 0..n_vert {
        let y = i / (s - 1);
        if y == 0 || y == s - 1 {
            frame += 1;
        }
    }
    for j in 0..(s * (s - 1)) {
        let x = j % s;
        if x == 0 || x == s - 1 {
            frame += 1;
        }
    }
    frame
}

/// A solved board with piece IDs relabelled by a random permutation. The board
/// (the physical tiling and its edges) is unchanged; only the integer names of
/// the pieces move, so cell `pos` is now solved by piece `sol[pos]` at rot 0
/// instead of trivially by piece `pos`. Without this the naive row-major solver
/// walks the identity solution in ~N² nodes and the hints do nothing.
struct Scrambled {
    pieces: Vec<[u8; 4]>,
    sol: Vec<u16>,
}

/// splitmix64 — a tiny deterministic PRNG for the ID permutation.
struct SplitMix(u64);
impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

fn scramble_ids(base: &Puzzle, seed: u64) -> Scrambled {
    let n = base.pieces.len();
    let mut new_id: Vec<usize> = (0..n).collect();
    let mut rng = SplitMix(seed);
    for i in (1..n).rev() {
        let j = (rng.next() % (i as u64 + 1)) as usize;
        new_id.swap(i, j);
    }
    let mut pieces = vec![[0u8; 4]; n];
    for old in 0..n {
        let e = base.pieces[old];
        pieces[new_id[old]] = [e[0], e[1], e[2], e[3]];
    }
    let sol = (0..n).map(|cell| new_id[cell] as u16).collect();
    Scrambled { pieces, sol }
}

/// Encode one edge colour as producer's 16-bit zero-padded binary word.
fn color_word(c: u8) -> String {
    if c == BORDER {
        "1".repeat(16)
    } else {
        format!("{c:016b}")
    }
}

fn write_variant_csv(dir: &Path, name: &str, g: Grid, s: &Scrambled, hint_cells: &[usize]) {
    let mut pin: HashMap<u16, (usize, usize, u8)> = HashMap::new();
    for &pos in hint_cells {
        let piece = s.sol[pos];
        let (x, y) = g.xy(pos);
        pin.insert(piece, (x, y, 0));
    }
    let mut out = String::new();
    out.push_str(&format!("{}\n", g.n));
    for (id, e) in s.pieces.iter().enumerate() {
        let (x, y, rot) = pin.get(&(id as u16)).copied().unwrap_or((0, 0, 0));
        out.push_str(&format!(
            "{},{},{},{},{},{},{}\n",
            color_word(e[0]),
            color_word(e[1]),
            color_word(e[2]),
            color_word(e[3]),
            x,
            y,
            rot
        ));
    }
    fs::write(dir.join(format!("{name}.csv")), out).expect("write csv variant");
}

fn write_variant(dir: &Path, name: &str, g: Grid, base: &Puzzle, s: &Scrambled, hint_cells: &[usize]) {
    use eternity2_engine::Hint;
    let hints = hint_cells
        .iter()
        .map(|&pos| Hint {
            pos: pos as u16,
            piece: s.sol[pos],
            rot: 0,
        })
        .collect::<Vec<_>>();
    let variant = Puzzle {
        name: name.to_string(),
        width: base.width,
        height: base.height,
        num_colors: base.num_colors,
        pieces: s.pieces.clone(),
        hints,
    };
    let json = serde_json::to_string_pretty(&variant).expect("serialise variant");
    fs::write(dir.join(format!("{name}.json")), json).expect("write variant");
    write_variant_csv(dir, name, g, s, hint_cells);
    println!("  wrote {name:28} ({:>3} hints)", hint_cells.len());
}

// ---------------------------------------------------------------------------
// Geometry helpers — all parametric in the grid.
// ---------------------------------------------------------------------------

/// Scattered lattice: every `stride`-th cell in both axes, offset so we do not
/// sit exactly on the rim. Row-major order.
fn lattice(g: Grid, stride: usize, offset: usize) -> Vec<usize> {
    let mut cells = Vec::new();
    let mut y = offset;
    while y < g.n {
        let mut x = offset;
        while x < g.n {
            cells.push(g.cell(x, y));
            x += stride;
        }
        y += stride;
    }
    cells
}

/// A lattice with exactly `k` evenly-spaced points per row and per column (so
/// `k×k` cells total), centred on the interior with a small inset off the rim.
/// This gives clean "k per line" spread layouts (k=3 → 9, k=6 → 36) that the
/// stride-based `lattice` cannot hit exactly.
fn lattice_per_line(g: Grid, k: usize) -> Vec<usize> {
    if k == 0 {
        return Vec::new();
    }
    // Positions: inset 1 from each rim, k points spread across the usable span.
    let lo = 1usize;
    let hi = g.n - 2; // last usable interior-ish index
    let span = hi.saturating_sub(lo);
    let coord = |i: usize| -> usize {
        if k == 1 {
            g.n / 2
        } else {
            lo + (span * i) / (k - 1)
        }
    };
    let mut cells = std::collections::BTreeSet::new();
    for iy in 0..k {
        for ix in 0..k {
            cells.insert(g.cell(coord(ix), coord(iy)));
        }
    }
    cells.into_iter().collect()
}

/// Contiguous block: the first `n` cells in row-major order (fills top rows).
fn contiguous(n: usize) -> Vec<usize> {
    (0..n).collect()
}

fn interior_lattice(g: Grid, stride: usize, n: usize) -> Vec<usize> {
    lattice(g, stride, 1)
        .into_iter()
        .filter(|&c| !g.is_border(c))
        .take(n)
        .collect()
}

fn border_cells(g: Grid, n: usize) -> Vec<usize> {
    (0..g.ncells()).filter(|&c| g.is_border(c)).take(n).collect()
}

/// A `k×k` solid block of cells with its top-left corner at (x0, y0).
fn block(g: Grid, x0: usize, y0: usize, k: usize) -> Vec<usize> {
    let mut cells = Vec::new();
    for dy in 0..k {
        for dx in 0..k {
            let (x, y) = (x0 + dx, y0 + dy);
            if x < g.n && y < g.n {
                cells.push(g.cell(x, y));
            }
        }
    }
    cells
}

/// The "clustered" end of the count ladder: five `k×k` solid blocks, one near
/// each corner and one at the centre. This is a *clustered geometry* for the
/// clustered-vs-spread comparison; it is deliberately five compact blocks, not
/// the real five official clue cells (those are single cells, tested on the path
/// axis via `e2_clue_shape`). The five-block layout is what banks a large
/// pinned-seam floor.
fn clustered_blocks(g: Grid, k: usize) -> Vec<usize> {
    let m = g.n;
    // Anchor cells: 4 offset from corners + 1 centre.
    let off = 1;
    let anchors = [
        (off, off),
        (m - off - k, off),
        (off, m - off - k),
        (m - off - k, m - off - k),
        (m / 2 - k / 2, m / 2 - k / 2),
    ];
    let mut set = std::collections::BTreeSet::new();
    for &(x0, y0) in &anchors {
        for c in block(g, x0, y0, k) {
            set.insert(c);
        }
    }
    set.into_iter().collect()
}

/// A genuinely spread interior lattice whose count is *close* to `target`. We
/// pick the stride whose FULL offset-1 lattice count is nearest to the target and
/// return that lattice unmodified — never a within-lattice downsample, which was
/// found to collapse low counts into near-linear rows (a geometry change
/// masquerading as a count change). The trade-off is that the achievable counts
/// are the lattice counts (⌈(n-1)/stride⌉², e.g. 9, 16, 25 on a 16-grid), so the
/// "count sweep" is a sweep over those; the caller labels each variant by its
/// ACTUAL count. Every returned set has the same uniform-grid character, so a
/// difference across the sweep is a difference in count, not in shape.
fn spread_count(g: Grid, target: usize) -> Vec<usize> {
    let mut best: Vec<usize> = lattice(g, 2, 1);
    let mut best_gap = (best.len() as i64 - target as i64).abs();
    for stride in 2..=g.n {
        let cells = lattice(g, stride, 1);
        if cells.is_empty() {
            continue;
        }
        let gap = (cells.len() as i64 - target as i64).abs();
        if gap < best_gap {
            best = cells;
            best_gap = gap;
        }
    }
    best
}

/// The official Eternity II 5-clue SHAPE, generalized to an n×n board: one
/// central anchor plus four anchors set in from the corners, in the same
/// arrangement as E2's five pre-placed clues (a centre clue and four spread
/// toward the quadrants). Only the geometric SHAPE is borrowed from the puzzle;
/// the pieces and board are our own generated instance. Returns 5 single cells.
///
/// At 16×16 these are the EXACT official Eternity II clue cells, in `(col,row)`
/// 0-indexed form: the mandatory centre clue at I8 = (7,8), and four clue-puzzle
/// placements inset three from each corner at C3 (2,2), C14 (13,2), N3 (2,13),
/// N14 (13,13). For other board sizes the same pattern is scaled: the four
/// corner clues sit `inset` cells in from each corner, and the centre clue sits
/// one short of the middle (mirroring I8's offset from the true centre H8/(8,8)).
fn e2_clue_shape(g: Grid) -> Vec<usize> {
    let m = g.n;
    if m == 16 {
        // The real official cells, exactly.
        return vec![
            g.cell(7, 8),   // I8, mandatory centre
            g.cell(2, 2),   // C3
            g.cell(13, 2),  // C14
            g.cell(2, 13),  // N3
            g.cell(13, 13), // N14
        ];
    }
    // Faithful generalization for the scaling axis: inset three from the corners
    // (clamped for small boards), centre one short of the middle.
    let inset = 3.min((m.saturating_sub(1)) / 2);
    let mid = m / 2;
    let centre = mid.saturating_sub(1);
    vec![
        g.cell(centre, mid),
        g.cell(inset, inset),
        g.cell(m - 1 - inset, inset),
        g.cell(inset, m - 1 - inset),
        g.cell(m - 1 - inset, m - 1 - inset),
    ]
}

fn main() {
    // Parse both the new flag form and the legacy positional form.
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let flag = |name: &str| -> Option<String> {
        argv.iter().position(|a| a == name).and_then(|i| argv.get(i + 1)).cloned()
    };

    let out_dir = flag("--out")
        .or_else(|| argv.iter().find(|a| !a.starts_with("--")).cloned())
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            eprintln!("usage: hint_variants --out <dir> [--size N] [--colors C] [--seed S]");
            std::process::exit(2);
        });

    let size: usize = flag("--size").and_then(|s| s.parse().ok()).unwrap_or(16);
    let seed: u32 = flag("--seed")
        .and_then(|s| s.parse().ok())
        .or_else(|| {
            // legacy positional seed
            argv.iter().filter(|a| !a.starts_with("--")).nth(1).and_then(|s| s.parse().ok())
        })
        .unwrap_or(1);
    let colors: u8 = flag("--colors")
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| faithful_colors(size));

    let g = Grid { n: size };
    fs::create_dir_all(&out_dir).expect("mkdir out_dir");

    let board = generate_solved_framed(size as u8, colors, seed, true);
    let border_edges = board
        .pieces
        .iter()
        .flat_map(|p| p.iter())
        .filter(|&&c| c == BORDER)
        .count();
    let scr = scramble_ids(&board, (seed as u64).wrapping_mul(0x1000_0001) ^ 0xE2);

    println!(
        "board N={size} colors={colors} (frame={} interior={}) seed={seed}: {} pieces, {} border-edge slots",
        frame_color_count(colors),
        colors - frame_color_count(colors),
        board.pieces.len(),
        border_edges,
    );

    // --- Axis 1: geometry at matched count -----------------------------------
    // Scattered lattice reaching deep into the board vs. the same count piled
    // contiguously, plus interior-only and border-only at that count.
    let scattered = lattice(g, 4, 1);
    let matched = scattered.len();
    write_variant(&out_dir, "geom_scattered", g, &board, &scr, &scattered);
    write_variant(&out_dir, "geom_contiguous", g, &board, &scr, &contiguous(matched));
    write_variant(&out_dir, "geom_interior", g, &board, &scr, &interior_lattice(g, 3, matched));
    write_variant(&out_dir, "geom_border", g, &board, &scr, &border_cells(g, matched));

    // --- Axis 2: the clustering ladder ---------------------------------------
    // Spread vs clustered at increasing counts. The clustered end is five k×k
    // blocks (corners + centre); the spread end is a clean k-per-line lattice.
    // Reports whether MORE clustered hints can lose to FEWER spread hints, and how
    // score moves as spread density rises (k per line = 2..6 → 4,9,16,25,36).
    let mut seen_spread = std::collections::BTreeSet::new();
    for k in 2..=6 {
        let cells = lattice_per_line(g, k);
        if cells.len() >= 4 && seen_spread.insert(cells.len()) {
            write_variant(&out_dir, &format!("ladder_spread_{:02}", cells.len()), g, &board, &scr, &cells);
        }
    }
    for k in 2..=4 {
        let cells = clustered_blocks(g, k);
        write_variant(&out_dir, &format!("ladder_clustered_k{k}_{:02}", cells.len()), g, &board, &scr, &cells);
    }

    // --- Axis 3 support: count sweep at fixed (spread) geometry ---------------
    let dense = lattice(g, 3, 1);
    for n in [24usize, 21, 18, 15, 12, 9, 6, 3] {
        if n <= dense.len() {
            let cells: Vec<usize> = dense.iter().copied().take(n).collect();
            write_variant(&out_dir, &format!("sweep_{n:02}"), g, &board, &scr, &cells);
        }
    }

    // --- Axis 3 support: same count (18), shallow vs deep vertical spread -----
    // Two 18-hint scattered lattices of the SHAPE discussed on the eternity2 list
    // (every-third-column on three rows): one confined to the top third (shallow),
    // one spanning top-to-bottom (deep). Tests whether reaching the endgame — the
    // stated causal mechanism — actually helps a backtracker more than a beam.
    // These are our own generated boards and layouts; only the lattice SHAPE is
    // borrowed from the list discussion, not any board, puzzle, or engine.
    if g.n >= 12 {
        let cols: Vec<usize> = (0..g.n).step_by(3).collect();
        let mk = |rows: &[usize]| -> Vec<usize> {
            let mut c = Vec::new();
            for &y in rows {
                for &x in &cols {
                    c.push(g.cell(x, y));
                }
            }
            c
        };
        let top = g.n / 8;
        write_variant(&out_dir, "shallow_18", g, &board, &scr, &mk(&[top, top + 2, top + 4]));
        write_variant(&out_dir, "deep_18", g, &board, &scr, &mk(&[top, g.n / 2, g.n - 3]));
    }

    // --- Axis: the E2 5-clue SHAPE (sparse anchors) on our board --------------
    // The official puzzle's own gift: 5 anchors in the E2 arrangement. The path
    // articles measure how different fill orders exploit — or waste — exactly this
    // geometry. Shape borrowed from the puzzle; board and pieces are ours.
    write_variant(&out_dir, "clue_shape_5", g, &board, &scr, &e2_clue_shape(g));

    // --- The hint-geometry-page layouts, on OUR boards ------------------------
    // The /research/why/hint-geometry page contrasts an 18-hint SCATTERED lattice
    // (the shape Peter McGavin used: rows {1,3,5} × cols {0,3,6,9,12,15}, "every
    // third column on a few odd rows") against 18 CONTIGUOUS hints piled in the
    // top rows, and claims scattered-18 solves easily while contiguous needs 80+.
    // We test the SAME SHAPES on OUR generated boards + solvers to confirm or
    // qualify that claim fairly (his was on Joe's puzzle with his engine). 16×16.
    if g.n == 16 {
        let cols = [0usize, 3, 6, 9, 12, 15];
        let rows = [1usize, 3, 5];
        let mut scattered18 = Vec::new();
        for &y in &rows {
            for &x in &cols {
                scattered18.push(g.cell(x, y));
            }
        }
        write_variant(&out_dir, "hintgeo_scattered_18", g, &board, &scr, &scattered18);
        write_variant(&out_dir, "hintgeo_contiguous_18", g, &board, &scr, &contiguous(18));
    }

    // Zero-hint baseline + full solution reference.
    write_variant(&out_dir, "baseline_00", g, &board, &scr, &[]);
    write_variant(&out_dir, "full_solution", g, &board, &scr, &(0..g.ncells()).collect::<Vec<_>>());

    println!("done -> {}", out_dir.display());
}
