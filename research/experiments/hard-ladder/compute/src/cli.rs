//! Command line shared by the built-in binary and plug-in binaries.
//!
//! ```text
//! hard_ladder list
//! hard_ladder gen                      # (re)write instances/*.jsonl
//! hard_ladder check-instances          # committed files == generator output
//! hard_ladder run --solver NAME [--role test|calibration] [--ladder primary|secondary]
//!                 [--sizes 6,7,8,9,10] [--threads 8] [--out FILE]
//! hard_ladder calibrate [--threads 8]  # every calibration candidate
//! ```
//!
//! Instances are read from `<root>/instances`, where `<root>` is the
//! hard-ladder directory (override with `HARD_LADDER_ROOT`). The cap is fixed
//! by the role (`TEST_CAP`, `CALIBRATION_CAP`) so every solver runs on the same
//! instances under the same caps. The default output is
//! `results/<role>/<ladder>/<solver>.jsonl` relative to the working directory.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::instance::{ladder_instances, Ladder, LadderInstance, Role};
use crate::registry::{find, BUILTIN, CALIBRATION_CANDIDATES};
use crate::runner::{run_all, RunRecord};
use crate::solver::SolverEntry;
use crate::solvers::scb::CapOverrun;
use crate::{CALIBRATION_CAP, TEST_CAP};

#[must_use]
pub fn root() -> PathBuf {
    std::env::var_os("HARD_LADDER_ROOT")
        .map_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(".."), PathBuf::from)
}

#[must_use]
pub fn instance_path(ladder: Ladder, role: Role) -> PathBuf {
    root().join("instances").join(format!("{}-{}.jsonl", ladder.name(), role.name()))
}

pub fn load_instances(ladder: Ladder, role: Role) -> Result<Vec<LadderInstance>, String> {
    let p = instance_path(ladder, role);
    let text = fs::read_to_string(&p).map_err(|e| format!("{}: {e}", p.display()))?;
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).map_err(|e| e.to_string()))
        .collect()
}

fn to_jsonl<T: serde::Serialize>(rows: &[T]) -> String {
    rows.iter().map(|r| serde_json::to_string(r).expect("json") + "\n").collect()
}

const PAIRS: [(Ladder, Role); 3] = [
    (Ladder::Primary, Role::Test),
    (Ladder::Primary, Role::Calibration),
    (Ladder::Secondary, Role::Test),
];

fn arg<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.iter().position(|a| a == key).and_then(|i| args.get(i + 1)).map(String::as_str)
}

fn write_records(out: &Path, recs: &[RunRecord]) -> Result<(), String> {
    if let Some(d) = out.parent() {
        fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    let mut f = fs::File::create(out).map_err(|e| e.to_string())?;
    f.write_all(to_jsonl(recs).as_bytes()).map_err(|e| e.to_string())
}

fn run_cmd(
    entry: &SolverEntry,
    ladder: Ladder,
    role: Role,
    sizes: &[u8],
    threads: usize,
    out: &Path,
) -> Result<(), String> {
    let cap = match role {
        Role::Test => TEST_CAP,
        Role::Calibration => CALIBRATION_CAP,
    };
    let insts: Vec<_> =
        load_instances(ladder, role)?.into_iter().filter(|i| sizes.contains(&i.n)).collect();
    let t = std::time::Instant::now();
    let recs = run_all(entry, &insts, cap, threads);
    write_records(out, &recs)?;
    let solved = recs.iter().filter(|r| r.status == "solved").count();
    let errors = recs.iter().filter(|r| r.status == "error").count();
    eprintln!(
        "{}: {solved}/{} solved, {errors} errors, cap {cap}, {:.1} s -> {}",
        entry.name,
        recs.len(),
        t.elapsed().as_secs_f64(),
        out.display()
    );
    Ok(())
}

/// Entry point. Plug-in binaries pass their own solvers in `extra`; names in
/// `extra` shadow built-ins.
#[must_use]
pub fn main_with(extra: &[SolverEntry]) -> ExitCode {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if !info.payload().is::<CapOverrun>() {
            default_hook(info);
        }
    }));
    let args: Vec<String> = std::env::args().collect();
    let res = match args.get(1).map(String::as_str) {
        Some("list") => {
            for e in extra.iter().chain(BUILTIN) {
                println!("{:28} run_seed={} {}", e.name, e.run_seed, e.description);
            }
            Ok(())
        }
        Some("gen") => {
            let dir = root().join("instances");
            fs::create_dir_all(&dir).map_err(|e| e.to_string()).and_then(|()| {
                for (l, r) in PAIRS {
                    let v = ladder_instances(l, r);
                    fs::write(instance_path(l, r), to_jsonl(&v)).map_err(|e| e.to_string())?;
                    eprintln!("wrote {} ({} instances)", instance_path(l, r).display(), v.len());
                }
                Ok(())
            })
        }
        Some("check-instances") => (|| {
            for (l, r) in PAIRS {
                let committed = fs::read_to_string(instance_path(l, r)).map_err(|e| e.to_string())?;
                if committed != to_jsonl(&ladder_instances(l, r)) {
                    return Err(format!("{} differs from the generator", instance_path(l, r).display()));
                }
            }
            eprintln!("instances match the generator");
            Ok(())
        })(),
        Some(cmd @ ("run" | "calibrate")) => (|| {
            let threads: usize = arg(&args, "--threads").map_or(Ok(8), str::parse).map_err(|e| format!("{e}"))?;
            let sizes: Vec<u8> = arg(&args, "--sizes")
                .unwrap_or("6,7,8,9,10")
                .split(',')
                .map(|s| s.parse().map_err(|e| format!("{e}")))
                .collect::<Result<_, _>>()?;
            if cmd == "calibrate" {
                for name in CALIBRATION_CANDIDATES {
                    let e = find(extra, name).expect("candidate registered");
                    let out = PathBuf::from(format!("results/calibration/primary/{name}.jsonl"));
                    run_cmd(e, Ladder::Primary, Role::Calibration, &sizes, threads, &out)?;
                }
                return Ok(());
            }
            let name = arg(&args, "--solver").ok_or("--solver required")?;
            let entry = find(extra, name).ok_or_else(|| format!("unknown solver {name}; try list"))?;
            let role = Role::parse(arg(&args, "--role").unwrap_or("test")).ok_or("bad --role")?;
            let ladder = Ladder::parse(arg(&args, "--ladder").unwrap_or("primary")).ok_or("bad --ladder")?;
            let out = arg(&args, "--out").map_or_else(
                || PathBuf::from(format!("results/{}/{}/{name}.jsonl", role.name(), ladder.name())),
                PathBuf::from,
            );
            run_cmd(entry, ladder, role, &sizes, threads, &out)
        })(),
        _ => Err("usage: hard_ladder list | gen | check-instances | run --solver NAME [--role R] [--ladder L] [--sizes ..] [--threads N] [--out F] | calibrate".into()),
    };
    match res {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
