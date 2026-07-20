//! Generate many boards at once — the "give me 100 boards of this shape, these
//! seeds, these hints pinned" tool.
//!
//!   cargo run --release --example generate_batch -- --n 100 --out runs/boards
//!   cargo run --release --example generate_batch -- --seeds 1..50 --pins 5 --framed
//!   cargo run --release --example generate_batch -- --n 20 --size 16 --colors 22 --jsonl
//!
//! Flags:
//!   --n <count>          generate seeds 1..=count (mutually exclusive with --seeds)
//!   --seeds <A..B>       explicit inclusive seed range, e.g. 1..100
//!   --size <n>           board side length (default 16)
//!   --colors <n>         interior colour count (default 22)
//!   --framed             frame-restricted colours (the real-Eternity-II look)
//!   --pins <k>           pin k solution cells as hints on each board (like clues)
//!   --out <dir>          write one <seed>.json per board (site-schema, reusable)
//!   --jsonl              instead, print one compact board record per line to stdout
//!
//! The per-board JSON is the site's `SiteInstance` schema, the same shape the
//! benchmark's `variant_NN.json` files use — so any of them can be read straight
//! back with `Instance::from_site_json`, including by the sweep runner.

use std::path::PathBuf;

use e2_kit::{generator, instance_from_generated, pin_solution_hints};

struct Args {
    seeds: Vec<u32>,
    size: u8,
    colors: u8,
    framed: bool,
    pins: u32,
    out: Option<PathBuf>,
    jsonl: bool,
}

fn main() {
    let args = parse_args();
    if let Some(dir) = &args.out {
        if let Err(e) = std::fs::create_dir_all(dir) {
            eprintln!("cannot create {}: {e}", dir.display());
            std::process::exit(1);
        }
    }

    let mut written = 0usize;
    for &seed in &args.seeds {
        let puzzle = generator::generate_framed(args.size, args.colors, seed, args.framed);
        let name = format!("gen-{}x{}-c{}-s{}", args.size, args.size, args.colors, seed);
        let mut instance = instance_from_generated(&name, &puzzle);
        if args.pins > 0 {
            instance = pin_solution_hints(
                instance, args.size, args.colors, seed, args.framed, args.pins,
            );
        }

        let site = instance.to_site();
        if args.jsonl {
            println!("{}", serde_json::to_string(&site).expect("serialize"));
        } else if let Some(dir) = &args.out {
            let path = dir.join(format!("{seed}.json"));
            let json = serde_json::to_string_pretty(&site).expect("serialize");
            std::fs::write(&path, json).expect("write board");
        }
        written += 1;
    }

    if let Some(dir) = &args.out {
        eprintln!("wrote {written} boards to {}", dir.display());
        eprintln!("read one back with: Instance::from_site_json(path)");
    } else if !args.jsonl {
        eprintln!(
            "generated {written} boards (nothing written — pass --out <dir> or --jsonl)"
        );
    }
}

fn parse_args() -> Args {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let mut seeds: Option<Vec<u32>> = None;
    let mut size = 16u8;
    let mut colors = 22u8;
    let mut framed = false;
    let mut pins = 0u32;
    let mut out = None;
    let mut jsonl = false;

    let mut i = 0;
    while i < raw.len() {
        match raw[i].as_str() {
            "--n" => {
                let n: u32 = next(&raw, &mut i, "--n");
                seeds = Some((1..=n).collect());
            }
            "--seeds" => {
                let spec: String = next(&raw, &mut i, "--seeds");
                seeds = Some(parse_range(&spec));
            }
            "--size" => size = next(&raw, &mut i, "--size"),
            "--colors" => colors = next(&raw, &mut i, "--colors"),
            "--framed" => framed = true,
            "--pins" => pins = next(&raw, &mut i, "--pins"),
            "--out" => out = Some(PathBuf::from(next::<String>(&raw, &mut i, "--out"))),
            "--jsonl" => jsonl = true,
            other => {
                eprintln!("unknown flag: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }

    Args {
        seeds: seeds.unwrap_or_else(|| (1..=10).collect()),
        size,
        colors,
        framed,
        pins,
        out,
        jsonl,
    }
}

fn next<T: std::str::FromStr>(raw: &[String], i: &mut usize, flag: &str) -> T {
    *i += 1;
    raw.get(*i)
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| {
            eprintln!("{flag} needs a valid value");
            std::process::exit(2);
        })
}

/// Parse an inclusive range like `1..100` into the seeds `1,2,…,100`.
fn parse_range(spec: &str) -> Vec<u32> {
    if let Some((a, b)) = spec.split_once("..") {
        let a: u32 = a.parse().unwrap_or(1);
        let b: u32 = b.parse().unwrap_or(a);
        (a..=b).collect()
    } else if let Ok(n) = spec.parse::<u32>() {
        vec![n]
    } else {
        eprintln!("bad --seeds range: {spec} (expected A..B)");
        std::process::exit(2);
    }
}
