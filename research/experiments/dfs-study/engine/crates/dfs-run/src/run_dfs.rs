//! run_dfs — run one (variant, instance) pair under a time budget and print a
//! single `RESULT` line the grid harness scrapes. The line carries EVERY stat
//! the study raises (score, nps, nodes, backtracks, max-depth, depth-at-timeout,
//! breaks, url, hash), unlike the sibling benchmark which drops most of them.
//!
//! Usage:
//!   run_dfs --list
//!   run_dfs --puzzle P.json --algo naive-clean --seed 1 --budget-s 60 [--emit out.json]

use std::process::ExitCode;

use dfs_engine::{find, all_specs, SpecKind};
use dfs_engine::{run, RunConfig};
use e2_io::Instance;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--list") {
        for s in all_specs() {
            println!(
                "{:<20} {:<10} path={:?} value={} prop={} breaks={}",
                s.name,
                s.family.tag(),
                s.path,
                s.value.tag(),
                s.propagate.tag(),
                s.breaks.tag(),
            );
        }
        return ExitCode::SUCCESS;
    }

    // Machine-readable variant matrix: the registry, as JSON, so the harness and
    // the site render the "what stacks on what" story from the code, not by hand.
    if args.iter().any(|a| a == "--matrix-json") {
        let rows: Vec<String> = all_specs()
            .into_iter()
            .map(|s| {
                format!(
                    "  {{\"name\":{:?},\"display\":{:?},\"family\":{:?},\"kind\":{:?},\
                     \"parent\":{},\"delta\":{:?},\"path\":\"{:?}\",\"value\":{:?},\
                     \"propagate\":{:?},\"breaks\":{:?},\"allows_breaks\":{},\"note\":{:?}}}",
                    s.name,
                    s.display,
                    s.family.tag(),
                    match s.kind {
                        SpecKind::Engine => "engine",
                        SpecKind::Codegen => "codegen",
                        SpecKind::External => "external",
                    },
                    s.parent.map_or("null".to_string(), |p| format!("{p:?}")),
                    s.delta,
                    s.path,
                    s.value.tag(),
                    s.propagate.tag(),
                    s.breaks.tag(),
                    s.allows_breaks(),
                    s.external_note,
                )
            })
            .collect();
        println!("[\n{}\n]", rows.join(",\n"));
        return ExitCode::SUCCESS;
    }

    let get = |flag: &str| -> Option<String> {
        args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)).cloned()
    };

    let Some(puzzle) = get("--puzzle") else {
        eprintln!("error: --puzzle <file.json> required (or --list)");
        return ExitCode::FAILURE;
    };
    let algo = get("--algo").unwrap_or_else(|| "naive-clean".to_string());
    let seed: u64 = get("--seed").and_then(|s| s.parse().ok()).unwrap_or(1);
    let budget_s: f64 = get("--budget-s").and_then(|s| s.parse().ok()).unwrap_or(60.0);

    let Some(spec) = find(&algo) else {
        eprintln!("error: unknown algo {algo:?}; try --list");
        return ExitCode::FAILURE;
    };

    let inst = match Instance::from_site_json(&puzzle) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("error: loading {puzzle}: {e}");
            return ExitCode::FAILURE;
        }
    };

    match spec.kind {
        SpecKind::Engine => {}
        SpecKind::Codegen => {
            eprintln!("error: {algo} is a codegen variant, run it via run_dfs_codegen");
            return ExitCode::FAILURE;
        }
        SpecKind::External => {
            eprintln!("error: {algo} is an external community engine, run it via its own wrapper");
            return ExitCode::FAILURE;
        }
    }

    // Optional --max-nodes caps search nodes for deterministic, wall-clock-
    // independent A/B testing of node-loop changes.
    let max_nodes = get("--max-nodes").and_then(|s| s.parse::<u64>().ok());
    let cfg = RunConfig { budget_ms: (budget_s * 1000.0) as u64, seed, max_nodes };
    let result = run(&inst, &spec, cfg);
    let out = result.output(&inst);
    let nps = result.stats.nodes_per_sec(result.elapsed_s);

    if let Some(path) = get("--emit") {
        // The one canonical artifact per board: a self-describing .json holding
        // the score, placement vector, hash, both letter blobs, and a ready-to-
        // open eternity2.dev viewer URL.
        let _ = out.write_json(&path);
    }

    // The single machine-readable line.
    println!(
        "RESULT algo={} seed={} score={} breaks={} elapsed_s={:.3} nodes={} backtracks={} \
         max_depth={} depth_at_timeout={} nodes_to_solution={} secs_to_solution={:.3} \
         nps={:.0} nps_unit=search-nodes/s board_hash={:016x} url={}",
        spec.name,
        seed,
        out.score,
        result.stats.breaks,
        result.elapsed_s,
        result.stats.nodes,
        result.stats.backtracks,
        result.stats.max_depth,
        result.stats.depth_at_timeout,
        result.stats.nodes_to_solution,
        result.stats.secs_to_solution,
        nps,
        out.board_hash,
        out.url,
    );
    ExitCode::SUCCESS
}
