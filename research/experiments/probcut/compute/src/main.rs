//! ```text
//! probcut gen | check-instances            # instances/8-c10-{test,calibration}.jsonl
//! probcut stage1 --role R --ladder L [--sizes 6,7] [--planted-only] [--threads 8]
//! probcut run --solver NAME --role R --ladder primary|8-c10 [--sizes 6,7] [--threads 8]
//!             [--luby-unit U] [--out FILE]
//! ```
//! Paths are relative to the probcut directory (`PROBCUT_ROOT` overrides).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use hard_ladder::cli::load_instances;
use hard_ladder::registry::find;
use hard_ladder::solvers::scb::CapOverrun;
use hard_ladder::{run_all, Ladder, LadderInstance, Role, CALIBRATION_CAP, TEST_CAP};
use probcut::{instance_8c10, stage1_rows, ChildRow, Params, LUBY_UNIT, PARAMS, PASS_INFO, SOLVERS};

fn root() -> PathBuf {
    std::env::var_os("PROBCUT_ROOT").map_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(".."), PathBuf::from)
}

fn arg<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.iter().position(|a| a == key).and_then(|i| args.get(i + 1)).map(String::as_str)
}

fn jsonl<T: serde::Serialize>(rows: &[T]) -> String {
    rows.iter().map(|r| serde_json::to_string(r).expect("json") + "\n").collect()
}

fn write(path: &Path, text: &str) -> Result<(), String> {
    if let Some(d) = path.parent() {
        fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

fn r8_path(role: &str) -> PathBuf {
    root().join("instances").join(format!("8-c10-{role}.jsonl"))
}

fn r8_generated(role: Role) -> Vec<LadderInstance> {
    role.seeds().into_iter().map(|s| instance_8c10(role.name(), s)).collect()
}

fn load(ladder: &str, role: Role, sizes: &[u8]) -> Result<Vec<LadderInstance>, String> {
    let v = if ladder == "8-c10" {
        let p = r8_path(role.name());
        let text = fs::read_to_string(&p).map_err(|e| format!("{}: {e}", p.display()))?;
        text.lines().filter(|l| !l.trim().is_empty()).map(|l| serde_json::from_str(l).map_err(|e| e.to_string())).collect::<Result<Vec<_>, _>>()?
    } else {
        let l = Ladder::parse(ladder).ok_or("bad --ladder")?;
        load_instances(l, role)?.into_iter().filter(|i| sizes.contains(&i.n)).collect()
    };
    if v.is_empty() {
        return Err("no instances".into());
    }
    Ok(v)
}

fn load_params() -> Result<(), String> {
    let p = root().join("results/calibration/params.json");
    if let Ok(text) = fs::read_to_string(&p) {
        let params: Params = serde_json::from_str(&text).map_err(|e| e.to_string())?;
        LUBY_UNIT.store(params.luby_unit, Ordering::Relaxed);
        let _ = PARAMS.set(params);
    }
    Ok(())
}

fn par_map<T: Sync, R: Send>(items: &[T], threads: usize, f: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let next = AtomicUsize::new(0);
    let out: Mutex<Vec<Option<R>>> = Mutex::new((0..items.len()).map(|_| None).collect());
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| loop {
                let k = next.fetch_add(1, Ordering::Relaxed);
                let Some(it) = items.get(k) else { break };
                let r = f(it);
                out.lock().expect("lock")[k] = Some(r);
            });
        }
    });
    out.into_inner().expect("lock").into_iter().map(|r| r.expect("ran")).collect()
}

fn real_main(args: &[String]) -> Result<(), String> {
    let role = Role::parse(arg(args, "--role").unwrap_or("test")).ok_or("bad --role")?;
    let ladder = arg(args, "--ladder").unwrap_or("primary");
    let sizes: Vec<u8> = arg(args, "--sizes").unwrap_or("6,7").split(',').map(|s| s.parse().map_err(|_| "bad --sizes")).collect::<Result<_, _>>()?;
    let threads: usize = arg(args, "--threads").unwrap_or("8").parse().map_err(|_| "bad --threads")?;
    match args.get(1).map(String::as_str) {
        Some("gen") => {
            for r in [Role::Test, Role::Calibration] {
                write(&r8_path(r.name()), &jsonl(&r8_generated(r)))?;
            }
            Ok(())
        }
        Some("check-instances") => {
            for r in [Role::Test, Role::Calibration] {
                let committed = fs::read_to_string(r8_path(r.name())).map_err(|e| e.to_string())?;
                if committed != jsonl(&r8_generated(r)) {
                    return Err(format!("{} differs from the generator", r8_path(r.name()).display()));
                }
                for inst in load("8-c10", r, &[])? {
                    hard_ladder::verify_board(&inst.puzzle, &inst.planted)?;
                }
            }
            eprintln!("8-c10 instances match the generator; planted boards verify");
            Ok(())
        }
        Some("stage1") => {
            let insts = load(ladder, role, &sizes)?;
            let planted_only = args.iter().any(|a| a == "--planted-only");
            let rows: Vec<Vec<ChildRow>> = par_map(&insts, threads, |i| {
                let n = i.n as usize;
                stage1_rows(i, &[n, 2 * n, 3 * n], !planted_only)
            });
            let rows: Vec<ChildRow> = rows.into_iter().flatten().collect();
            let tag = if planted_only { "planted" } else { "stage1" };
            let out = root().join(format!("results/{}/{tag}-{ladder}-{}.jsonl", role.name(), sizes.iter().map(u8::to_string).collect::<Vec<_>>().join("")));
            let out = if ladder == "8-c10" { root().join(format!("results/{}/{tag}-8-c10.jsonl", role.name())) } else { out };
            write(&out, &jsonl(&rows))?;
            eprintln!("wrote {} ({} rows)", out.display(), rows.len());
            Ok(())
        }
        Some("run") => {
            load_params()?;
            if let Some(u) = arg(args, "--luby-unit") {
                LUBY_UNIT.store(u.parse().map_err(|_| "bad --luby-unit")?, Ordering::Relaxed);
            }
            let name = arg(args, "--solver").ok_or("--solver required")?;
            let entry = find(SOLVERS, name).ok_or_else(|| format!("unknown solver {name}"))?;
            let insts = load(ladder, role, &sizes)?;
            let cap = if role == Role::Test { TEST_CAP } else { CALIBRATION_CAP };
            let recs = run_all(entry, &insts, cap, threads);
            if let Some(e) = recs.iter().find(|r| r.status == "error") {
                eprintln!("ERROR {}: {:?}", e.instance, e.error);
            }
            let out = arg(args, "--out").map_or_else(|| root().join(format!("results/{}/{ladder}/{name}.jsonl", role.name())), PathBuf::from);
            write(&out, &jsonl(&recs))?;
            if let Some(map) = PASS_INFO.lock().expect("lock").as_ref() {
                let rows: Vec<serde_json::Value> = insts
                    .iter()
                    .filter_map(|i| map.get(&i.puzzle.name).map(|p| serde_json::json!({"instance": i.id, "n": i.n, "info": p})))
                    .collect();
                write(&out.with_extension("passes.jsonl"), &jsonl(&rows))?;
            }
            eprintln!("wrote {}", out.display());
            Ok(())
        }
        _ => Err("usage: probcut gen|check-instances|stage1|run ...".into()),
    }
}

fn main() -> ExitCode {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if !info.payload().is::<CapOverrun>() {
            default_hook(info);
        }
    }));
    let args: Vec<String> = std::env::args().collect();
    match real_main(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
