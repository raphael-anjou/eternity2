//! `e2kit` — the kit's command-line umbrella.
//!
//!   e2kit gen --n 100 --pins 5 --out runs/boards   # batch-generate boards
//!   e2kit compare runs/<A> runs/<B>                 # paired diff of two runs
//!   e2kit score "<board URL>"                       # score one board
//!
//! `gen` and `score` mirror the same-named examples; `compare` is the tool that
//! answers "did my change actually help?" by diffing two run directories the
//! sweep produced. Run a sweep with `cargo run --release --example sweep`.

use std::path::Path;

use e2_kit::{
    generator, instance_from_generated, pin_solution_hints, score_url, CellResult, RunDir,
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().map_or("", String::as_str);
    let rest = &args[args.len().min(1)..];
    match cmd {
        "gen" => cmd_gen(rest),
        "compare" => cmd_compare(rest),
        "score" => cmd_score(rest),
        "" | "-h" | "--help" | "help" => usage(),
        other => {
            eprintln!("unknown command: {other}\n");
            usage();
            std::process::exit(2);
        }
    }
}

fn usage() {
    eprintln!(
        "e2kit — Eternity II starter-kit CLI\n\
         \n\
         USAGE:\n\
         \x20 e2kit gen [--n N | --seeds A..B] [--size S] [--colors C] [--framed] [--pins K] [--out DIR | --jsonl]\n\
         \x20 e2kit compare <runA> <runB>\n\
         \x20 e2kit score \"<board URL>\"\n\
         \n\
         Run a sweep to produce run directories for `compare`:\n\
         \x20 cargo run --release --example sweep -- --n 40\n"
    );
}

fn cmd_score(rest: &[String]) {
    let Some(url) = rest.first() else {
        eprintln!("usage: e2kit score \"<board URL>\"");
        std::process::exit(2);
    };
    if let Some(s) = score_url(url) { println!("{s}") } else {
        eprintln!("could not parse a board out of that URL");
        std::process::exit(1);
    }
}

fn cmd_gen(rest: &[String]) {
    let mut seeds: Vec<u32> = (1..=10).collect();
    let mut size = 16u8;
    let mut colors = 22u8;
    let mut framed = false;
    let mut pins = 0u32;
    let mut out: Option<String> = None;
    let mut jsonl = false;

    let mut i = 0;
    while i < rest.len() {
        match rest[i].as_str() {
            "--n" => {
                let n = val::<u32>(rest, &mut i, "--n");
                seeds = (1..=n).collect();
            }
            "--seeds" => {
                let spec = val::<String>(rest, &mut i, "--seeds");
                seeds = parse_range(&spec);
            }
            "--size" => size = val(rest, &mut i, "--size"),
            "--colors" => colors = val(rest, &mut i, "--colors"),
            "--framed" => framed = true,
            "--pins" => pins = val(rest, &mut i, "--pins"),
            "--out" => out = Some(val(rest, &mut i, "--out")),
            "--jsonl" => jsonl = true,
            other => {
                eprintln!("gen: unknown flag {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }

    if let Some(dir) = &out {
        std::fs::create_dir_all(dir).expect("create out dir");
    }
    let mut written = 0usize;
    for &seed in &seeds {
        let puzzle = generator::generate_framed(size, colors, seed, framed);
        let name = format!("gen-{size}x{size}-c{colors}-s{seed}");
        let mut instance = instance_from_generated(&name, &puzzle);
        if pins > 0 {
            instance = pin_solution_hints(instance, size, colors, seed, framed, pins);
        }
        let site = instance.to_site();
        if jsonl {
            println!("{}", serde_json::to_string(&site).expect("serialize"));
        } else if let Some(dir) = &out {
            let json = serde_json::to_string_pretty(&site).expect("serialize");
            std::fs::write(Path::new(dir).join(format!("{seed}.json")), json).expect("write");
        }
        written += 1;
    }
    if let Some(dir) = &out {
        eprintln!("wrote {written} boards to {dir}");
    } else if !jsonl {
        eprintln!("generated {written} boards (pass --out DIR or --jsonl to keep them)");
    }
}

fn cmd_compare(rest: &[String]) {
    let (Some(a), Some(b)) = (rest.first(), rest.get(1)) else {
        eprintln!("usage: e2kit compare <runA> <runB>");
        std::process::exit(2);
    };
    let ra = RunDir::read_results(a).unwrap_or_else(|e| die(&format!("read {a}: {e}")));
    let rb = RunDir::read_results(b).unwrap_or_else(|e| die(&format!("read {b}: {e}")));

    // Pair by seed so the comparison is apples-to-apples per board.
    let map_b: std::collections::HashMap<u32, &CellResult> =
        rb.iter().map(|c| (c.seed, c)).collect();
    let mut paired: Vec<(&CellResult, &CellResult)> = Vec::new();
    for ca in &ra {
        if let Some(cb) = map_b.get(&ca.seed) {
            paired.push((ca, *cb));
        }
    }
    if paired.is_empty() {
        die("no shared seeds between the two runs — did they sweep the same grid?");
    }

    let n = paired.len();
    let deltas: Vec<f64> = paired
        .iter()
        .map(|(a, b)| f64::from(b.score) - f64::from(a.score))
        .collect();
    let mean_a = ra.iter().map(|c| f64::from(c.score)).sum::<f64>() / ra.len() as f64;
    let mean_b = rb.iter().map(|c| f64::from(c.score)).sum::<f64>() / rb.len() as f64;
    let mean_delta = deltas.iter().sum::<f64>() / n as f64;
    let sd_delta = {
        let var = deltas.iter().map(|d| (d - mean_delta).powi(2)).sum::<f64>() / n as f64;
        var.sqrt()
    };
    let wins = deltas.iter().filter(|&&d| d > 0.0).count();
    let losses = deltas.iter().filter(|&&d| d < 0.0).count();
    let ties = n - wins - losses;

    println!("A: {a}");
    println!("B: {b}");
    println!("paired on {n} shared seeds\n");
    println!("  mean A      {mean_a:.2}");
    println!("  mean B      {mean_b:.2}");
    println!("  mean Δ(B−A) {mean_delta:+.2}   sd {sd_delta:.2}");
    println!("  B wins {wins}   losses {losses}   ties {ties}");

    // Honesty guardrail: on this puzzle the paired-delta sd is large; a mean
    // shift smaller than sd/sqrt(n) is not distinguishable from noise.
    let stderr = if n > 0 { sd_delta / (n as f64).sqrt() } else { 0.0 };
    println!("\n  standard error of the mean Δ: {stderr:.2}");
    if mean_delta == 0.0 {
        println!("  → identical scores (mean Δ = 0). The two runs match.");
    } else if mean_delta.abs() < 2.0 * stderr {
        println!("  → |mean Δ| < 2·SE: NOT significant. Sweep more seeds before claiming a win.");
    } else if mean_delta > 0.0 {
        println!("  → B looks genuinely better (mean Δ > 2·SE).");
    } else {
        println!("  → B looks genuinely worse (mean Δ < −2·SE).");
    }
    if n < 40 {
        println!("  (n={n} is small; aim for 40+ paired seeds for a trustworthy verdict.)");
    }
}

fn val<T: std::str::FromStr>(rest: &[String], i: &mut usize, flag: &str) -> T {
    *i += 1;
    rest.get(*i)
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| die(&format!("{flag} needs a valid value")))
}

fn parse_range(spec: &str) -> Vec<u32> {
    if let Some((a, b)) = spec.split_once("..") {
        let a: u32 = a.parse().unwrap_or(1);
        let b: u32 = b.parse().unwrap_or(a);
        (a..=b).collect()
    } else if let Ok(n) = spec.parse::<u32>() {
        vec![n]
    } else {
        die(&format!("bad range: {spec}"))
    }
}

fn die(msg: &str) -> ! {
    eprintln!("{msg}");
    std::process::exit(1);
}
