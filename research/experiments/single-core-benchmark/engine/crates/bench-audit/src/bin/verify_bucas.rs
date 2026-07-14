// verify_bucas — independently rescore & diff bucas board_edges URLs.
//
// Usage: verify_bucas <puzzle.csv> <url1> [url2 url3 ...]
//
// For each URL: decode via board_edges, rescore matched-edges with our
// own scorer, check the 5 canonical hints, count placed cells.
// Then print a pairwise tile-Hamming matrix across all boards.

use std::path::Path;

use eternity2_core::{Board, Puzzle, BORDER};
use eternity2_export::{decode_bucas_board, verify};
use eternity2_puzzle_io::load_puzzle_with_hints;

// Count matched interior+border edges (BORDER=0 never matches).
fn score(puzzle: &Puzzle, b: &Board) -> u32 {
    let w = puzzle.width as usize;
    let h = puzzle.height as usize;
    let mut edges_at: Vec<[u8; 4]> = Vec::with_capacity(w * h);
    for pos in 0..(w * h) as u32 {
        match b.get(pos) {
            Some((id, rot)) => {
                edges_at.push(puzzle.piece(id).unwrap().edges.rotated(rot).as_array());
            }
            None => edges_at.push([0; 4]),
        }
    }
    let mut m = 0u32;
    for y in 0..h {
        for x in 0..w {
            let p = y * w + x;
            let e = edges_at[p];
            if x + 1 < w {
                let er = edges_at[p + 1];
                if e[1] != BORDER && e[1] == er[3] {
                    m += 1;
                }
            }
            if y + 1 < h {
                let eb = edges_at[p + w];
                if e[2] != BORDER && e[2] == eb[0] {
                    m += 1;
                }
            }
        }
    }
    m
}

// Row-major break attribution: for each cell p (in scan order), a break is
// charged when the piece's TOP edge != bottom-of-neighbour-above, or its LEFT
// edge != right-of-neighbour-left. Returns (total_breaks, n_break_cells,
// n_double_break_cells, per-row break counts). A double-break cell pays 2.
struct BreakProfile {
    total: u32,
    cells_with_break: u32,
    double_break_cells: u32,
    per_row: Vec<u32>,
}

fn break_profile(puzzle: &Puzzle, b: &Board) -> BreakProfile {
    let w = puzzle.width as usize;
    let h = puzzle.height as usize;
    let mut edges_at: Vec<[u8; 4]> = Vec::with_capacity(w * h);
    for pos in 0..(w * h) as u32 {
        match b.get(pos) {
            Some((id, rot)) => edges_at.push(puzzle.piece(id).unwrap().edges.rotated(rot).as_array()),
            None => edges_at.push([0; 4]),
        }
    }
    let mut total = 0u32;
    let mut cells = 0u32;
    let mut doubles = 0u32;
    let mut per_row = vec![0u32; h];
    for y in 0..h {
        for x in 0..w {
            let p = y * w + x;
            let e = edges_at[p];
            let mut cell_breaks = 0u32;
            // TOP: compare our top to the bottom of the cell above.
            if y > 0 {
                let up = edges_at[p - w];
                // A break only counts on a real (non-border) required edge:
                // interior adjacency where colors disagree.
                if !(e[0] == up[2] && e[0] != BORDER) && !(e[0] == BORDER && up[2] == BORDER) {
                    cell_breaks += 1;
                }
            }
            // LEFT: compare our left to the right of the cell to the left.
            if x > 0 {
                let lf = edges_at[p - 1];
                if !(e[3] == lf[1] && e[3] != BORDER) && !(e[3] == BORDER && lf[1] == BORDER) {
                    cell_breaks += 1;
                }
            }
            if cell_breaks > 0 {
                cells += 1;
                total += cell_breaks;
                per_row[y] += cell_breaks;
                if cell_breaks == 2 {
                    doubles += 1;
                }
            }
        }
    }
    BreakProfile { total, cells_with_break: cells, double_break_cells: doubles, per_row }
}

// Per-cell break grid: '.' = 0 breaks, '1' = one break, '2' = double break.
fn print_break_grid(puzzle: &Puzzle, b: &Board) {
    let w = puzzle.width as usize;
    let h = puzzle.height as usize;
    let mut edges_at: Vec<[u8; 4]> = Vec::with_capacity(w * h);
    for pos in 0..(w * h) as u32 {
        match b.get(pos) {
            Some((id, rot)) => edges_at.push(puzzle.piece(id).unwrap().edges.rotated(rot).as_array()),
            None => edges_at.push([0; 4]),
        }
    }
    println!("           break grid (row 0 = top):");
    for y in 0..h {
        let mut line = String::from("           ");
        for x in 0..w {
            let p = y * w + x;
            let e = edges_at[p];
            let mut cb = 0u32;
            if y > 0 {
                let up = edges_at[p - w];
                if !(e[0] == up[2] && e[0] != BORDER) && !(e[0] == BORDER && up[2] == BORDER) { cb += 1; }
            }
            if x > 0 {
                let lf = edges_at[p - 1];
                if !(e[3] == lf[1] && e[3] != BORDER) && !(e[3] == BORDER && lf[1] == BORDER) { cb += 1; }
            }
            line.push(match cb { 0 => '.', 1 => '1', _ => '2' });
        }
        println!("{line}");
    }
}

// Tile-level Hamming: number of positions where (piece_id) differs.
fn hamming_tiles(a: &Board, b: &Board, puzzle: &Puzzle) -> u32 {
    let mut d = 0u32;
    for p in 0..puzzle.cell_count() {
        let pa = a.get(p).map(|(id, _)| id);
        let pb = b.get(p).map(|(id, _)| id);
        if pa != pb {
            d += 1;
        }
    }
    d
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: verify_bucas <puzzle.csv> <url1> [url2 ...]");
        std::process::exit(2);
    }
    let (puzzle, hints) = load_puzzle_with_hints(Path::new(&args[1])).expect("load puzzle+hints");
    eprintln!(
        "puzzle: {}x{}, {} pieces, {} hints",
        puzzle.width,
        puzzle.height,
        puzzle.pieces().len(),
        hints.hints.len()
    );

    let mut boards: Vec<Board> = Vec::new();
    let mut had_failure = false;
    for (i, url) in args[2..].iter().enumerate() {
        match decode_bucas_board(&puzzle, url) {
            Ok(decoded) => {
                let b = decoded.board;
                let independent_score = score(&puzzle, &b);
                let report = verify(&puzzle, &hints, &b);
                let physical = report.placed_count == report.total_cells
                    && report.unique_pieces == report.placed_count
                    && report.duplicate_pieces.is_empty()
                    && report.border_violations.is_empty();
                let center_pos = puzzle.position(puzzle.width / 2 - 1, puzzle.height / 2);
                let center_ok = hints
                    .hints
                    .iter()
                    .find(|h| h.position == center_pos)
                    .is_some_and(|h| b.get(h.position) == Some((h.piece_id, h.rotation)));
                let strict = report.hint_compliance.matched == report.hint_compliance.total;
                let bp = break_profile(&puzzle, &b);
                println!(
                    "board[{i}]: score={independent_score}/{} canonical_score={}/{} \
                     placed={}/{} uniq={} borders={} hints={}/{} center={} strict5={} \
                     physical={} piece_numbers={} motif={} breaks={} \
                     (identity max-score={}) break_cells={} double_break_cells={}",
                    report.total_adjacencies,
                    report.matched,
                    report.total_adjacencies,
                    report.placed_count,
                    report.total_cells,
                    report.unique_pieces,
                    report.border_violations.len(),
                    report.hint_compliance.matched,
                    report.hint_compliance.total,
                    center_ok,
                    strict,
                    physical,
                    decoded.used_piece_numbers,
                    decoded.motif_order.as_deref().unwrap_or("default"),
                    bp.total,
                    report.total_adjacencies.saturating_sub(independent_score),
                    bp.cells_with_break,
                    bp.double_break_cells
                );
                if independent_score != report.matched || !physical || !center_ok {
                    had_failure = true;
                    eprintln!(
                        "board[{i}] AUDIT FAILURE: independent/canonical score agreement={}, \
                         physical={}, mandatory_center={}",
                        independent_score == report.matched,
                        physical,
                        center_ok
                    );
                }
                let rows: Vec<String> = bp.per_row.iter().enumerate()
                    .filter(|(_, &c)| c > 0)
                    .map(|(r, &c)| format!("r{r}:{c}"))
                    .collect();
                println!("           break rows: [{}]", rows.join(" "));
                if std::env::var("BREAK_GRID").is_ok() {
                    print_break_grid(&puzzle, &b);
                }
                boards.push(b);
            }
            Err(error) => {
                had_failure = true;
                println!("board[{i}]: DECODE FAILED: {error}");
            }
        }
    }

    if boards.len() > 1 {
        println!("\npairwise tile-Hamming (piece-id differs):");
        print!("      ");
        for j in 0..boards.len() {
            print!("  b{j:<3}");
        }
        println!();
        for i in 0..boards.len() {
            print!("  b{i:<3}");
            for j in 0..boards.len() {
                let d = hamming_tiles(&boards[i], &boards[j], &puzzle);
                print!("  {d:<4}");
            }
            println!();
        }
    }
    if had_failure {
        std::process::exit(1);
    }
}
