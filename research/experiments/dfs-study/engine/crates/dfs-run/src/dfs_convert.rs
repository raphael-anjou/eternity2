//! dfs-convert — the study's IO utility, exposing the shared converters so a
//! puzzle or board can move between the formats the blog's engines use.
//!
//!   dfs-convert site2csv  in.json  out.csv           # site JSON  -> legacy CSV
//!   dfs-convert csv2site  in.csv   out.json          # legacy CSV  -> site JSON
//!   dfs-convert url2json  in.url   puzzle.csv out.json  # bucas/edges -> canonical JSON
//!   dfs-convert info      in.json                     # summarise an instance
//!
//! All conversions round-trip through the one `Instance` type, so any blog
//! engine's input can be fed to any other without a bespoke script. `url2json`
//! is the migration path: it matches an edge-only viewer URL back against a
//! piece set to recover the full placement, then writes the one canonical board
//! document (with an eternity2.dev URL) that every algorithm now emits.

use std::process::ExitCode;

use e2_io::{load_puzzle_csv, parse_board_edges, parse_puzzle_name, Instance};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("");
    match cmd {
        "site2csv" => {
            let (Some(inp), Some(out)) = (args.get(2), args.get(3)) else {
                return usage();
            };
            match Instance::from_site_json(inp) {
                Ok(inst) => {
                    if let Err(e) = std::fs::write(out, inst.to_csv()) {
                        eprintln!("write {out}: {e}");
                        return ExitCode::FAILURE;
                    }
                    println!("wrote {out} ({} pieces, {} hints)", inst.pieces.len(), inst.hints.len());
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("read {inp}: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        "csv2site" => {
            let (Some(inp), Some(out)) = (args.get(2), args.get(3)) else {
                return usage();
            };
            match load_puzzle_csv(inp) {
                Ok(inst) => {
                    let json = serde_json::to_string_pretty(&inst.to_site()).unwrap();
                    if let Err(e) = std::fs::write(out, json) {
                        eprintln!("write {out}: {e}");
                        return ExitCode::FAILURE;
                    }
                    println!("wrote {out} ({} pieces, {} hints)", inst.pieces.len(), inst.hints.len());
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("read {inp}: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        "url2json" => {
            let (Some(inp), Some(puzzle), Some(out)) = (args.get(2), args.get(3), args.get(4))
            else {
                return usage();
            };
            // The input may be a .url file or a raw URL/edges string on argv.
            let text = std::fs::read_to_string(inp).unwrap_or_else(|_| inp.clone());
            let Some(cells) = parse_board_edges(&text) else {
                eprintln!("no board_edges found in {inp}");
                return ExitCode::FAILURE;
            };
            // Load the piece set to match against (CSV or site JSON).
            let mut inst = match load_puzzle_csv(puzzle) {
                Ok(i) => i,
                Err(_) => match Instance::from_site_json(puzzle) {
                    Ok(i) => i,
                    Err(e) => {
                        eprintln!("read puzzle {puzzle}: {e}");
                        return ExitCode::FAILURE;
                    }
                },
            };
            // Carry the board's own name through to the URL, if it had one.
            if let Some(name) = parse_puzzle_name(&text) {
                inst.name = name;
            }
            let solved = inst.match_board(&cells);
            if let Err(e) = solved.write_json(out) {
                eprintln!("write {out}: {e}");
                return ExitCode::FAILURE;
            }
            println!("wrote {out} (score {}, {} breaks)", solved.score, solved.breaks);
            ExitCode::SUCCESS
        }
        "info" => {
            let Some(inp) = args.get(2) else { return usage() };
            match Instance::from_site_json(inp) {
                Ok(inst) => {
                    println!(
                        "{}: {}x{}, {} pieces, {} interior colours, {} hints, max score {}",
                        inst.name,
                        inst.width,
                        inst.height,
                        inst.pieces.len(),
                        inst.num_colors,
                        inst.hints.len(),
                        inst.max_score(),
                    );
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("read {inp}: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        _ => usage(),
    }
}

fn usage() -> ExitCode {
    eprintln!(
        "usage:\n  dfs-convert site2csv in.json out.csv\n  \
         dfs-convert csv2site in.csv out.json\n  \
         dfs-convert url2json in.url puzzle.csv out.json\n  \
         dfs-convert info in.json"
    );
    ExitCode::FAILURE
}
