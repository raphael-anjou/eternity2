//! dfs-convert — the study's IO utility, exposing the shared converters so a
//! puzzle or board can move between the formats the blog's engines use.
//!
//!   dfs-convert site2csv  in.json  out.csv     # site JSON  -> legacy CSV
//!   dfs-convert csv2site  in.csv   out.json    # legacy CSV  -> site JSON
//!   dfs-convert info      in.json               # summarise an instance
//!
//! All conversions round-trip through the one `Instance` type, so any blog
//! engine's input can be fed to any other without a bespoke script.

use std::process::ExitCode;

use dfs_io::{load_puzzle_csv, Instance};

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
         dfs-convert csv2site in.csv out.json\n  dfs-convert info in.json"
    );
    ExitCode::FAILURE
}
