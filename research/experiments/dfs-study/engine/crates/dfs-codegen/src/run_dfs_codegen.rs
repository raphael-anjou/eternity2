//! run_dfs_codegen — run the specialised NAIVE-CODEGEN backtracker on one
//! instance and print the same `RESULT` line shape as `run_dfs`, so the grid
//! harness treats it identically.

use std::process::ExitCode;

use e2_io::Instance;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let get = |flag: &str| -> Option<String> {
        args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)).cloned()
    };

    let Some(puzzle) = get("--puzzle") else {
        eprintln!("error: --puzzle <file.json> required");
        return ExitCode::FAILURE;
    };
    let budget_s: f64 = get("--budget-s").and_then(|s| s.parse().ok()).unwrap_or(60.0);
    let seed: u64 = get("--seed").and_then(|s| s.parse().ok()).unwrap_or(1);

    let inst = match Instance::from_site_json(&puzzle) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("error: loading {puzzle}: {e}");
            return ExitCode::FAILURE;
        }
    };

    let result = dfs_codegen::run(&inst, (budget_s * 1000.0) as u64);
    let out = result.output(&inst);
    let nps = result.stats.nodes_per_sec(result.elapsed_s);

    if let Some(path) = get("--emit") {
        let _ = out.write_json(&path);
    }

    println!(
        "RESULT algo=naive-codegen seed={} score={} breaks={} elapsed_s={:.3} nodes={} \
         backtracks={} max_depth={} depth_at_timeout={} nps={:.0} nps_unit=search-nodes/s \
         board_hash={:016x} url={}",
        seed,
        out.score,
        result.stats.breaks,
        result.elapsed_s,
        result.stats.nodes,
        result.stats.backtracks,
        result.stats.max_depth,
        result.stats.depth_at_timeout,
        nps,
        out.board_hash,
        out.url,
    );
    ExitCode::SUCCESS
}
