//! `run_dfs_codegen_jit` — the true-codegen path. Emits a puzzle-specialised
//! Rust program, compiles it with `rustc -O`, runs it, and forwards its `RESULT`
//! line. This is the direct analogue of McGavin's `genbody -DG` → link → run.
//!
//! Usage: run_dfs_codegen_jit --puzzle P.json [--budget-s N] [--emit-src out.rs]
//!        [--keep] [--opt native]

use std::process::{Command, Stdio};

use dfs_codegen::emit::{emit_program, emit_program_chain};
use e2_io::Instance;

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let get = |k: &str| -> Option<String> {
        args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned()
    };
    let Some(puzzle) = get("--puzzle") else {
        eprintln!("error: --puzzle <file.json> required");
        return std::process::ExitCode::FAILURE;
    };
    let budget_s: f64 = get("--budget-s").and_then(|s| s.parse().ok()).unwrap_or(60.0);
    let keep = args.iter().any(|a| a == "--keep");
    let native = get("--opt").as_deref() == Some("native");

    let inst = match Instance::from_site_json(&puzzle) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("error: loading {puzzle}: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };

    let chain = args.iter().any(|a| a == "--chain");
    let budget_ms = (budget_s * 1000.0) as u64;
    let src = if chain {
        emit_program_chain(&inst, budget_ms)
    } else {
        emit_program(&inst, budget_ms)
    };

    // Write the generated source, optionally to a caller-named path.
    let dir = std::env::temp_dir();
    let src_path = if let Some(p) = get("--emit-src") {
        std::path::PathBuf::from(p)
    } else {
        dir.join(format!("e2_codegen_{}.rs", std::process::id()))
    };
    if let Err(e) = std::fs::write(&src_path, &src) {
        eprintln!("error: writing source {}: {e}", src_path.display());
        return std::process::ExitCode::FAILURE;
    }

    // Compile: rustc -O (optionally target-cpu=native), no external crates.
    let bin_path = dir.join(format!("e2_codegen_{}", std::process::id()));
    let mut cc = Command::new("rustc");
    cc.arg("-O")
        .arg("--edition")
        .arg("2021")
        .arg("-C")
        .arg("panic=abort")
        .arg("-C")
        .arg("opt-level=3") // `-O` alone is opt-level=2; force 3 (McGavin uses -Ofast)
        .arg("-C")
        .arg("codegen-units=1");
    if native {
        cc.arg("-C").arg("target-cpu=native");
    }
    cc.arg("-o").arg(&bin_path).arg(&src_path);
    let compile_start = std::time::Instant::now();
    let status = cc.status().expect("spawn rustc");
    let compile_s = compile_start.elapsed().as_secs_f64();
    if !status.success() {
        eprintln!("error: rustc failed to compile the generated program");
        eprintln!("       kept source at {}", src_path.display());
        return std::process::ExitCode::FAILURE;
    }
    eprintln!("# compiled generated engine in {compile_s:.2}s");

    // Run it; forward its stdout (the RESULT line).
    let out = Command::new(&bin_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("run generated engine");

    if !keep {
        let _ = std::fs::remove_file(&bin_path);
        if get("--emit-src").is_none() {
            let _ = std::fs::remove_file(&src_path);
        }
    }
    if out.success() {
        std::process::ExitCode::SUCCESS
    } else {
        std::process::ExitCode::FAILURE
    }
}
