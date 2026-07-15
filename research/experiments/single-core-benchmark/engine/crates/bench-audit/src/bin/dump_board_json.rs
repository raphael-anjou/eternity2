// dump_board_json — decode a bucas URL (or load a partial JSON) and emit a
// full JSON description of the puzzle catalog + board placement, for external
// (Python) consumers such as the worm-update experiment.
//
// Output JSON:
//   {
//     "width": 16, "height": 16,
//     "pieces": [ { "id": u16, "edges": [t,r,b,l] (base rotation) }, ... ],
//     "hints":  [ { "pos": u32, "piece_id": u16, "rotation": u8 }, ... ],
//     "board":  [ { "pos": u32, "piece_id": u16, "rotation": u8 } | null, ... ]
//   }
// Board is dense, length width*height; empty cells are null.

use std::path::PathBuf;

use eternity2_benchmark::loader::load_puzzle_with_hints;
use eternity2_core::Board;
use eternity2_export::{decode_bucas_board, load_board};
use serde_json::json;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: dump_board_json <bucas-url-or-board.json>");
        std::process::exit(1);
    }
    let input = &args[1];
    let puzzle_path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/puzzles/size_16_official_eternity.csv"));
    let (puzzle, hints) = load_puzzle_with_hints(&puzzle_path).expect("load puzzle");

    // Decide: bucas URL/blob or a JSON file path on disk.
    let looks_like_url = input.contains("bucas.name")
        || input.contains("board_edges")
        || (input.chars().all(|c| c.is_ascii_lowercase()) && input.len() == puzzle.cell_count() as usize * 4);

    let board: Board = if looks_like_url {
        decode_bucas_board(&puzzle, input).expect("decode bucas").board
    } else {
        let path = PathBuf::from(input);
        load_board(&path, &puzzle).expect("load board json")
    };

    let pieces: Vec<_> = puzzle
        .pieces()
        .iter()
        .map(|p| {
            let e = p.edges.as_array();
            json!({ "id": p.id, "edges": [e[0], e[1], e[2], e[3]] })
        })
        .collect();

    let hints_json: Vec<_> = hints
        .hints
        .iter()
        .map(|h| json!({ "pos": h.position, "piece_id": h.piece_id, "rotation": h.rotation.as_u8() }))
        .collect();

    let n = puzzle.cell_count();
    let mut cells = Vec::with_capacity(n as usize);
    for pos in 0..n {
        match board.get(pos) {
            Some((pid, rot)) => {
                cells.push(json!({ "pos": pos, "piece_id": pid, "rotation": rot.as_u8() }));
            }
            None => cells.push(serde_json::Value::Null),
        }
    }

    let out = json!({
        "width": puzzle.width,
        "height": puzzle.height,
        "color_count": puzzle.color_count,
        "pieces": pieces,
        "hints": hints_json,
        "board": cells,
    });
    println!("{}", serde_json::to_string(&out).unwrap());
}
