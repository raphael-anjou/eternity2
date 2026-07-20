//! Read a board in any format and print it in every other — one lossless hub.
//!
//!   cargo run --release --example convert -- "<eternity2.dev / bucas URL>"
//!   cargo run --release --example convert -- path/to/board.json    # site-schema
//!
//! Everything routes through `e2-io`'s converters, the same code the site's
//! format page uses, so the outputs here match the site exactly. From a URL we
//! have only edges (piece identities are ambiguous without a piece set); from a
//! site-JSON instance we have the full piece set and can emit CSV too.

use e2_kit::{
    bucas_url, parse_board_edges, score_cells, viewer_url, Instance, SiteInstance,
};
use e2_io::format::{cells_to_e2pieces, cells_to_edges};

fn main() {
    let Some(input) = std::env::args().nth(1) else {
        eprintln!("usage: cargo run --release --example convert -- \"<URL>\" | <board.json>");
        std::process::exit(2);
    };

    if input.ends_with(".json") {
        convert_site_json(&input);
    } else {
        convert_url(&input);
    }
}

fn convert_url(url: &str) {
    let Some(cells) = parse_board_edges(url) else {
        eprintln!("could not parse board_edges out of that input");
        std::process::exit(1);
    };
    let n = cells.len();
    let size = (n as f64).sqrt() as u8;
    println!("read {n} cells ({size}×{size}) from a URL (edges only)\n");
    // Edges alone don't carry piece identities, so board_pieces is blank.
    let codes: Vec<i32> = vec![-1; cells.len()];
    print_edge_forms(&cells, size, &codes);
    // Without a piece set we cannot number pieces, so no CSV / piece codes here.
    println!("\n(for board_pieces and CSV, pass a site-schema .json with its piece set)");
}

fn convert_site_json(path: &str) {
    let instance = match Instance::from_site_json(path) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("cannot read {path}: {e}");
            std::process::exit(1);
        }
    };
    // The solved (identity) placement, so we can show a full board's every form.
    let mut board = e2_kit::Board::new();
    for id in 0..instance.pieces.len() {
        board.place(id, id as u16, 0);
    }
    let out = instance.finish(&board);
    println!(
        "read instance '{}' ({}×{}, {} colours, {} hints)\n",
        instance.name,
        instance.width,
        instance.height,
        instance.num_colors,
        instance.hints.len()
    );
    print_edge_forms(&out.cells, instance.width, &out.board);
    println!("\nboard_pieces (piece*4+rot codes):");
    println!("{}", e2_io::format::codes_to_pieces(&out.board));
    println!("\nCSV (row per piece, community engines read this):");
    print!("{}", instance.to_csv());
    println!("\nsite-schema JSON round-trips via SiteInstance:");
    let site: SiteInstance = instance.to_site();
    println!("  {} pieces, {} hints", site.pieces.len(), site.hints.len());
}

fn print_edge_forms(cells: &[[u8; 4]], size: u8, codes: &[i32]) {
    println!("score (canonical, rim excluded): {}", score_cells(cells));
    println!("\nboard_edges:\n{}", cells_to_edges(cells));
    println!("\ne2pieces.txt:\n{}", cells_to_e2pieces(cells));
    let _ = codes; // codes feed board_pieces below; the URL rides in edges + hints
    println!("\neternity2.dev viewer URL:\n{}", viewer_url("converted", size, cells, &[]));
    println!("\ne2.bucas.name URL:\n{}", bucas_url("converted", size, size, cells));
}
