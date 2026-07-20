//! Generate hint-geometry variants for the "where you place the hints beats how
//! many" experiment (eternity2.dev/research/why/hint-geometry).
//!
//! One solved 16x16 board is generated to Eternity II's colour recipe (5 border
//! colours + 17 interior). Because `generate_solved_framed` lays piece `i` at
//! cell `i` at rotation 0, the identity placement IS the solution, so a hint for
//! any cell `pos` is just `{pos, piece: pos, rot: 0}`. We emit several hint sets
//! that differ only in geometry / count, all pinning true-solution pieces, as
//! site-schema JSON that the dfs-study `run_dfs` binary consumes unchanged.
//!
//! Usage: eternity2-engine-hint-variants <out_dir> [board_seed]

use std::fs;
use std::path::{Path, PathBuf};

use eternity2_engine::{generate_solved_framed, Puzzle, BORDER};

const SIZE: usize = 16;
const COLORS: u8 = 22; // 5 border + 17 interior, the official E2 recipe

/// A solved board with piece IDs relabelled by a random permutation. The board
/// (the physical tiling and its edges) is unchanged; only the integer names of
/// the pieces move, so cell `pos` is now solved by piece `sol[pos]` at rot 0
/// instead of trivially by piece `pos`. Without this the naive row-major solver
/// walks the identity solution in ~240 nodes and the hints do nothing.
struct Scrambled {
    /// `pieces[id]` = edges (URDL, rot 0) of the piece now labelled `id`.
    pieces: Vec<[u8; 4]>,
    /// `sol[pos]` = the piece id that solves cell `pos` at rot 0.
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

/// Relabel the solved board's piece IDs by a seeded permutation.
fn scramble_ids(base: &Puzzle, seed: u64) -> Scrambled {
    let n = base.pieces.len();
    // new_id[old_index] = the id the piece originally at index `old_index` takes.
    let mut new_id: Vec<usize> = (0..n).collect();
    let mut rng = SplitMix(seed);
    for i in (1..n).rev() {
        let j = (rng.next() % (i as u64 + 1)) as usize;
        new_id.swap(i, j);
    }
    // pieces[new_id[old]] = base.pieces[old]; sol[cell] = new_id[cell] (cell was
    // solved by original piece `cell` at rot 0 on the identity board).
    let mut pieces = vec![[0u8; 4]; n];
    for old in 0..n {
        let e = base.pieces[old];
        pieces[new_id[old]] = [e[0], e[1], e[2], e[3]];
    }
    let sol = (0..n).map(|cell| new_id[cell] as u16).collect();
    Scrambled { pieces, sol }
}

/// Encode one edge colour as producer's 16-bit zero-padded binary word.
/// The engine's `BORDER` (0) maps to producer's grey border `1111111111111111`
/// (65535); every interior colour id is written as its 16-bit binary.
fn color_word(c: u8) -> String {
    if c == BORDER {
        "1".repeat(16)
    } else {
        format!("{c:016b}")
    }
}

/// Emit the same variant as producer's legacy CSV. Layout: line 1 = board size;
/// then one line per piece id (line `id+2`) `top,right,bottom,left,x,y,rot`.
/// A hint pins cell `pos` to piece `sol[pos]`, so it is written on THAT piece's
/// row as `x=pos%16, y=pos/16, rot`. This is the identical hint path every
/// benchmark solver reads (`load_puzzle_with_hints`), so producer honours the
/// pins for the whole beam search.
fn write_variant_csv(dir: &Path, name: &str, s: &Scrambled, hint_cells: &[usize]) {
    // piece id -> (x, y, rot) if pinned. URDL edges: producer wants top,right,
    // bottom,left, which is exactly our [U,R,D,L] order.
    let mut pin: std::collections::HashMap<u16, (usize, usize, u8)> = std::collections::HashMap::new();
    for &pos in hint_cells {
        let piece = s.sol[pos];
        pin.insert(piece, (pos % SIZE, pos / SIZE, 0));
    }
    let mut out = String::new();
    out.push_str(&format!("{SIZE}\n"));
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
    let path = dir.join(format!("{name}.csv"));
    fs::write(&path, out).expect("write csv variant");
}

/// Site-schema JSON is byte-compatible with the engine `Puzzle` serde derive,
/// so we build a `Puzzle` clone carrying only the chosen hints and serialise it.
/// Also emits the producer-format CSV of the same variant.
fn write_variant(dir: &Path, name: &str, base: &Puzzle, s: &Scrambled, hint_cells: &[usize]) {
    use eternity2_engine::Hint;
    let hints = hint_cells
        .iter()
        .map(|&pos| Hint {
            pos: pos as u16,
            piece: s.sol[pos], // scrambled solution: cell pos is solved by sol[pos]
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
    let path = dir.join(format!("{name}.json"));
    fs::write(&path, json).expect("write variant");
    write_variant_csv(dir, name, s, hint_cells);
    println!("wrote {} ({} hints, +csv)", path.display(), hint_cells.len());
}

fn is_border(cell: usize) -> bool {
    let (x, y) = (cell % SIZE, cell / SIZE);
    x == 0 || y == 0 || x == SIZE - 1 || y == SIZE - 1
}

/// Scattered lattice: every `stride`-th cell in both axes, offset by 1 so we do
/// not sit exactly on the rim. Returns cells in row-major order.
fn lattice(stride: usize, offset: usize) -> Vec<usize> {
    let mut cells = Vec::new();
    let mut y = offset;
    while y < SIZE {
        let mut x = offset;
        while x < SIZE {
            cells.push(y * SIZE + x);
            x += stride;
        }
        y += stride;
    }
    cells
}

/// Contiguous block: the first `n` cells in row-major order (fills top rows).
fn contiguous(n: usize) -> Vec<usize> {
    (0..n).collect()
}

/// Interior cells only, sampled on a lattice, taking the first `n`.
fn interior_lattice(stride: usize, n: usize) -> Vec<usize> {
    lattice(stride, 1)
        .into_iter()
        .filter(|&c| !is_border(c))
        .take(n)
        .collect()
}

/// Border cells only (the rim ring), taking the first `n` in row-major order.
fn border_cells(n: usize) -> Vec<usize> {
    (0..SIZE * SIZE).filter(|&c| is_border(c)).take(n).collect()
}

/// McGavin's ACTUAL 18-hint board, decoded from his posted bucas URL (groups.io
/// msg 11746): rows {1,3,5} x columns {0,3,6,9,12,15}. This is the real "18
/// scattered hints" that solved Joe's puzzle in <15 min. Note it sits entirely
/// in the top third -- it is spread horizontally, NOT reaching the deep endgame
/// (which is worth noting against the page's "reach the endgame" wording).
fn mcgavin_real_18() -> Vec<usize> {
    let cols = [0usize, 3, 6, 9, 12, 15]; // every third column
    let rows = [1usize, 3, 5]; // the real rows from Peter's board
    let mut cells = Vec::new();
    for &y in &rows {
        for &x in &cols {
            cells.push(y * SIZE + x);
        }
    }
    cells
}

/// A genuinely DEEP 18-hint set: same every-third-column lattice, but on rows
/// {1,7,13} that span top-to-bottom -- what the page's PROSE describes ("dotted
/// down into the region the search reaches last"). McGavin's real board does not
/// do this; this variant tests whether the page's stated causal mechanism
/// (reach the endgame) would actually help MORE than his shallow board did.
fn deep_18() -> Vec<usize> {
    let cols = [0usize, 3, 6, 9, 12, 15];
    let rows = [1usize, 7, 13];
    let mut cells = Vec::new();
    for &y in &rows {
        for &x in &cols {
            cells.push(y * SIZE + x);
        }
    }
    cells
}

fn main() {
    let mut args = std::env::args().skip(1);
    let out_dir = PathBuf::from(args.next().unwrap_or_else(|| {
        eprintln!("usage: hint-variants <out_dir> [board_seed]");
        std::process::exit(2);
    }));
    let board_seed: u32 = args.next().map(|s| s.parse().unwrap()).unwrap_or(1);
    fs::create_dir_all(&out_dir).expect("mkdir out_dir");

    // One solved board, held fixed across every variant.
    let board = generate_solved_framed(SIZE as u8, COLORS, board_seed, true);
    let border_colors = board
        .pieces
        .iter()
        .flat_map(|p| p.iter())
        .filter(|&&c| c == BORDER)
        .count();
    // Relabel piece IDs so the naive row-major solver can't walk the identity
    // solution for free. The tiling is unchanged; hints pin the true (relabelled)
    // pieces. Derived from board_seed so the whole instance is reproducible.
    let scr = scramble_ids(&board, (board_seed as u64).wrapping_mul(0x1000_0001) ^ 0xE2);
    println!(
        "solved board seed={board_seed}: {} pieces, {} border-edge slots, ids scrambled",
        board.pieces.len(),
        border_colors
    );

    // --- Geometry contrast at (roughly) matched count -----------------------
    // Scattered lattice stride 4, offset 1 -> a 4x4 grid = 16 hints reaching
    // deep into the board (the page's "winner").
    let scattered = lattice(4, 1);
    write_variant(&out_dir, "scattered_lattice_16", &board, &scr, &scattered);

    // Contiguous block of the same count, piled into the top rows (the "loser").
    write_variant(&out_dir, "contiguous_16", &board, &scr, &contiguous(scattered.len()));

    // Interior-only vs border-only at the same count.
    write_variant(
        &out_dir,
        "interior_only_16",
        &board,
        &scr,
        &interior_lattice(3, scattered.len()),
    );
    write_variant(
        &out_dir,
        "border_only_16",
        &board,
        &scr,
        &border_cells(scattered.len()),
    );

    // --- Count sweep at fixed geometry (the commenter's 18 -> 15 -> ...) -----
    // Scattered geometry, decreasing count, so we isolate count from placement.
    for n in [24usize, 21, 18, 15, 12, 9, 6, 3] {
        // Take the first `n` lattice cells over a denser stride-3 grid so the
        // counts are reachable while staying scattered.
        let cells: Vec<usize> = lattice(3, 1).into_iter().take(n).collect();
        write_variant(&out_dir, &format!("sweep_scattered_{n:02}"), &board, &scr, &cells);
    }

    // --- McGavin's REAL 18-hint board vs. contiguous, and vs. a deep variant --
    // The actual scattered board Peter solved (rows 1/3/5, decoded from msg 11746).
    write_variant(&out_dir, "mcgavin_real_18", &board, &scr, &mcgavin_real_18());
    // A genuinely deep 18 (rows 1/7/13): what the page's PROSE describes, to test
    // whether "reaching the endgame" helps more than Peter's shallow real board.
    write_variant(&out_dir, "deep_18", &board, &scr, &deep_18());
    // The article's actual contrast: 18 piled into contiguous top rows.
    write_variant(&out_dir, "contiguous_18", &board, &scr, &contiguous(18));

    // Zero-hint baseline (unpinned): how the solver fares with no help at all.
    write_variant(&out_dir, "sweep_scattered_00", &board, &scr, &[]);

    // Full solution: every cell pinned. Emitting the whole board as hints gives
    // any downstream tool the complete piece+rotation-per-cell solution (used to
    // convert to other solvers' formats).
    let all_cells: Vec<usize> = (0..SIZE * SIZE).collect();
    write_variant(&out_dir, "full_solution", &board, &scr, &all_cells);

    println!("done -> {}", out_dir.display());
}
