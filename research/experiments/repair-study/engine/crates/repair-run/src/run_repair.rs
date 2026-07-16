//! run_repair — run one (variant, instance) pair under a time budget and print a
//! single `RESULT` line the grid harness scrapes. The line carries every stat
//! the study raises: score, the lift over the starting board, iterations, the
//! iteration the best last improved (the stall axis), accept rate, mean destroy
//! size, restarts, iters/sec, plus the bucas url and board hash.
//!
//! Usage:
//!   run_repair --list
//!   run_repair --matrix-json
//!   run_repair --puzzle P.json --algo greedy-mismatch --seed 1 --budget-s 60 [--emit out.url]

use std::process::ExitCode;

use e2_io::Instance;
use repair_engine::{all_specs, find, run, RunConfig};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--list") {
        for s in all_specs() {
            println!(
                "{:<20} {:<9} start={:<12} destroy={:<16} repair={:<14} accept={:<14} restart={}",
                s.name,
                s.family.tag(),
                s.start.tag(),
                s.destroy.tag(),
                s.repair.tag(),
                s.accept.tag(),
                s.restart.tag(),
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
                    "  {{\"name\":{:?},\"display\":{:?},\"family\":{:?},\"parent\":{},\
                     \"delta\":{:?},\"start\":{:?},\"destroy\":{:?},\"repair\":{:?},\
                     \"accept\":{:?},\"restart\":{:?}}}",
                    s.name,
                    s.display,
                    s.family.tag(),
                    s.parent.map_or("null".to_string(), |p| format!("{p:?}")),
                    s.delta,
                    s.start.tag(),
                    s.destroy.tag(),
                    s.repair.tag(),
                    s.accept.tag(),
                    s.restart.tag(),
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
    let algo = get("--algo").unwrap_or_else(|| "greedy-mismatch".to_string());
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

    let cfg = RunConfig { budget_ms: (budget_s * 1000.0) as u64, seed };
    let result = run(&inst, &spec, cfg);
    let out = result.output(&inst);
    let ips = result.stats.iters_per_sec(result.elapsed_s);
    let lift = i64::from(result.best_score) - i64::from(result.stats.start_score);

    if let Some(path) = get("--emit") {
        let _ = std::fs::write(&path, &out.bucas_url);
    }
    // The convergence curve, emitted to a sidecar file if requested.
    if let Some(path) = get("--emit-curve") {
        let curve: Vec<String> = result.stats.curve.iter().map(std::string::ToString::to_string).collect();
        let _ = std::fs::write(&path, curve.join(","));
    }

    println!(
        "RESULT algo={} seed={} score={} start_score={} lift={} iterations={} \
         last_best_iter={} accepts={} accept_rate={:.4} improvements={} best_improvements={} \
         mean_destroy={:.2} restarts={} elapsed_s={:.3} ips={:.0} ips_unit=repair-iters/s \
         board_hash={:016x} url={}",
        spec.name,
        seed,
        out.score,
        result.stats.start_score,
        lift,
        result.stats.iterations,
        result.stats.last_best_iter,
        result.stats.accepts,
        result.stats.accept_rate(),
        result.stats.improvements,
        result.stats.best_improvements,
        result.stats.mean_destroy(),
        result.stats.restarts,
        result.elapsed_s,
        ips,
        out.board_hash,
        out.bucas_url,
    );
    ExitCode::SUCCESS
}
