//! ```text
//! ktam gen | check-instances
//! ktam calibrate --arm B3|V1|V2 [--threads 8]
//! ktam run --arm B1|B2|B3|V1|V2 --rung R6|R7|R8 [--threads 8]
//! ```
//! Outputs go under `results/` of the experiment directory.

use std::fs;
use std::process::ExitCode;

use hard_ladder::registry::find;
use hard_ladder::solvers::scb::CapOverrun;
use hard_ladder::{run_all, Role, TEST_CAP};
use ktam::driver::{from_ladder, instances, r8_jsonl, r8_path, root, run_jobs, write_rows, EVENT_BUDGET};
use ktam::{grid, Rule};

fn arg<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.iter().position(|a| a == key).and_then(|i| args.get(i + 1)).map(String::as_str)
}

fn selected(arm: &str) -> Result<Rule, String> {
    let p = root().join("results/calibration/selection.json");
    let text = fs::read_to_string(&p).map_err(|e| format!("{}: {e} (run calibration first)", p.display()))?;
    let v: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let name = v["selected"][arm].as_str().ok_or_else(|| format!("no selection for {arm}"))?;
    Rule::parse(name).ok_or_else(|| format!("bad config {name}"))
}

fn real_main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let threads: usize = arg(&args, "--threads").map_or(Ok(8), str::parse).map_err(|e| format!("{e}"))?;
    match args.get(1).map(String::as_str) {
        Some("gen") => {
            fs::create_dir_all(root().join("instances")).map_err(|e| e.to_string())?;
            for role in [Role::Test, Role::Calibration] {
                fs::write(r8_path(role), r8_jsonl(role)).map_err(|e| e.to_string())?;
                eprintln!("wrote {}", r8_path(role).display());
            }
            Ok(())
        }
        Some("check-instances") => {
            for role in [Role::Test, Role::Calibration] {
                let committed = fs::read_to_string(r8_path(role)).map_err(|e| e.to_string())?;
                if committed != r8_jsonl(role) {
                    return Err(format!("{} differs from the generator", r8_path(role).display()));
                }
                for inst in instances("R8", role)? {
                    hard_ladder::verify_board(&inst.puzzle, &inst.planted).map_err(|e| format!("{}: {e}", inst.id))?;
                }
            }
            for rung in ["R6", "R7"] {
                for role in [Role::Test, Role::Calibration] {
                    for inst in instances(rung, role)? {
                        let (p, planted) = hard_ladder::instance::generate(inst.n, inst.c, inst.seed);
                        if p.pieces != inst.puzzle.pieces || planted != inst.planted {
                            return Err(format!("ladder instance {} differs from the generator", inst.id));
                        }
                    }
                }
            }
            eprintln!("instances match the generator; planted boards verify");
            Ok(())
        }
        Some("calibrate") => {
            let arm = arg(&args, "--arm").ok_or("--arm")?;
            let insts: Vec<_> = ["R6", "R7"]
                .iter()
                .map(|r| instances(r, Role::Calibration).map(|v| (*r, v)))
                .collect::<Result<_, _>>()?;
            for rule in grid(arm) {
                let jobs: Vec<_> =
                    insts.iter().flat_map(|(r, v)| v.iter().map(move |i| (rule, i, *r))).collect();
                // Node cap (E + n^2) / 2: every solve within E events is observed.
                let rows = run_jobs(arm, &jobs, |n| (EVENT_BUDGET + u64::from(n) * u64::from(n)) / 2, threads);
                write_rows(&root().join(format!("results/calibration/{arm}/{}.jsonl", rule.name())), &rows)?;
            }
            Ok(())
        }
        Some("run") => {
            let arm = arg(&args, "--arm").ok_or("--arm")?;
            let rung = arg(&args, "--rung").ok_or("--rung")?;
            let insts = instances(rung, Role::Test)?;
            let t = std::time::Instant::now();
            let rows = match arm {
                "B1" | "B2" => {
                    let name = if arm == "B1" { "scb-gacolor_ac3_random" } else { "hp-dfs" };
                    let entry = find(&[], name).ok_or("baseline missing")?;
                    run_all(entry, &insts, TEST_CAP, threads).iter().map(|r| from_ladder(arm, r, rung)).collect()
                }
                _ => {
                    let rule = selected(arm)?;
                    let jobs: Vec<_> = insts.iter().map(|i| (rule, i, rung)).collect();
                    run_jobs(arm, &jobs, |_| TEST_CAP, threads)
                }
            };
            let out = root().join(format!("results/test/{rung}/{arm}.jsonl"));
            write_rows(&out, &rows)?;
            eprintln!(
                "{arm} {rung}: {}/{} within E, {} errors, {:.1} s -> {}",
                rows.iter().filter(|r| r.within_e).count(),
                rows.len(),
                rows.iter().filter(|r| r.status == "error").count(),
                t.elapsed().as_secs_f64(),
                out.display()
            );
            Ok(())
        }
        Some("explore-cpu") => {
            // Exploratory, non-binding: see PREREG_AMENDMENTS.md E1.
            let arm = arg(&args, "--arm").ok_or("--arm")?;
            let rung = arg(&args, "--rung").ok_or("--rung")?;
            let cap: u64 = arg(&args, "--node-cap").map_or(Ok(TEST_CAP), str::parse).map_err(|e| format!("{e}"))?;
            let tag = arg(&args, "--tag").unwrap_or("timing");
            let insts = instances(rung, Role::Test)?;
            let rule = if matches!(arm, "B1" | "B2") { None } else { Some(selected(arm)?) };
            let rows = ktam::explore::run_cpu(arm, rule, &insts, rung, cap, threads);
            let out = root().join(format!("results/exploratory/cpu/{rung}/{tag}-{arm}.jsonl"));
            fs::create_dir_all(out.parent().expect("dir")).map_err(|e| e.to_string())?;
            let text: String = rows.iter().map(|r| serde_json::to_string(r).expect("json") + "\n").collect();
            fs::write(&out, text).map_err(|e| e.to_string())?;
            eprintln!("-> {}", out.display());
            Ok(())
        }
        _ => Err("usage: ktam gen | check-instances | calibrate --arm A | run --arm A --rung R".into()),
    }
}

fn main() -> ExitCode {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if !info.payload().is::<CapOverrun>() {
            default_hook(info);
        }
    }));
    match real_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
