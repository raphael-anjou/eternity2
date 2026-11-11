//! Cut-multiset pruning on the hard ladder: the ladder's best baseline
//! (single-core-benchmark `gacolor_ac3_random`, forked verbatim) plus a Hall
//! matching check, run through the hard-ladder harness. See `../PREREG.md`.
//!
//! Besides the hard-ladder CLI (`run`, `list`, ...), this binary owns the
//! 8x8 c = 10 rung (`R8`, ladder file `8-c10`):
//!
//! ```text
//! cut_multiset gen-c10                 # write ../instances/8-c10-{test,calibration}.jsonl
//! cut_multiset check-c10               # committed files == generator output
//! cut_multiset run-c10 --solver NAME [--role test|calibration] [--threads 8]
//! ```

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use cmp_solver_engine::{EngineSolver, HallMode};
use eternity2_solver_trait::Solver;
use hard_ladder::instance::{generate, owen_criterion, FRAME_COLOURS};
use hard_ladder::registry::find;
use hard_ladder::solvers::scb::Scb;
use hard_ladder::{run_all, LadderInstance, LadderSolver, Role, SolverEntry, CALIBRATION_CAP, TEST_CAP};

macro_rules! fork {
    ($name:literal, $scope:literal, $period:literal, $desc:literal) => {
        SolverEntry {
            name: $name,
            description: $desc,
            run_seed: 1,
            make: |seed| -> Box<dyn LadderSolver> {
                Box::new(Scb {
                    make: || -> Box<dyn Solver> {
                        Box::new(
                            EngineSolver::gacolor_ac3_random()
                                .with_hall(HallMode { scope: $scope, period: $period }),
                        )
                    },
                    seed,
                })
            },
        }
    };
}

pub const ENTRIES: &[SolverEntry] = &[
    fork!("cmp-fork-off", 0, 1, "forked gacolor_ac3_random, Hall check off (must equal scb-gacolor_ac3_random)"),
    fork!("cmp-hall-frontier", 1, 1, "V1: + Hall matching of frontier cells at every node"),
    fork!("cmp-hall-all", 2, 1, "+ Hall matching of every empty cell at every node"),
    fork!("cmp-hall-frontier-k4", 1, 4, "+ Hall matching of frontier cells every 4th node"),
    fork!("cmp-hall-all-k4", 2, 4, "+ Hall matching of every empty cell every 4th node"),
];

const R8_N: u8 = 8;
const R8_C: u8 = 10;
const R8_LADDER: &str = "8-c10";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn c10_path(role: Role) -> PathBuf {
    root().join("instances").join(format!("{R8_LADDER}-{}.jsonl", role.name()))
}

/// R8 instances: exactly the ladder's generator call and scramble at n = 8, c = 10.
fn c10_instances(role: Role) -> Vec<LadderInstance> {
    role.seeds()
        .into_iter()
        .map(|seed| {
            let (puzzle, planted) = generate(R8_N, R8_C, seed);
            LadderInstance {
                id: format!("n{R8_N}-c{R8_C}-s{seed}"),
                ladder: R8_LADDER.into(),
                role: role.name().into(),
                n: R8_N,
                c: R8_C,
                seed,
                owen_criterion: owen_criterion(R8_N),
                generator: format!(
                    "generate_solved_framed_bc({R8_N}, {}, {seed}, true, {FRAME_COLOURS}) + XorShift({seed} ^ 0xA5A5A5A5) scramble",
                    R8_C + FRAME_COLOURS
                ),
                puzzle,
                planted,
            }
        })
        .collect()
}

fn jsonl<T: serde::Serialize>(rows: &[T]) -> String {
    rows.iter().map(|r| serde_json::to_string(r).expect("json") + "\n").collect()
}

fn arg<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.iter().position(|a| a == key).and_then(|i| args.get(i + 1)).map(String::as_str)
}

fn c10(cmd: &str, args: &[String]) -> Result<(), String> {
    const ROLES: [Role; 2] = [Role::Test, Role::Calibration];
    match cmd {
        "gen-c10" => {
            fs::create_dir_all(root().join("instances")).map_err(|e| e.to_string())?;
            for r in ROLES {
                fs::write(c10_path(r), jsonl(&c10_instances(r))).map_err(|e| e.to_string())?;
                eprintln!("wrote {}", c10_path(r).display());
            }
            Ok(())
        }
        "check-c10" => {
            for r in ROLES {
                let committed = fs::read_to_string(c10_path(r)).map_err(|e| e.to_string())?;
                if committed != jsonl(&c10_instances(r)) {
                    return Err(format!("{} differs from the generator", c10_path(r).display()));
                }
            }
            eprintln!("8-c10 instances match the generator");
            Ok(())
        }
        _ => {
            let name = arg(args, "--solver").ok_or("--solver required")?;
            let entry = find(ENTRIES, name).ok_or_else(|| format!("unknown solver {name}"))?;
            let role = Role::parse(arg(args, "--role").unwrap_or("test")).ok_or("bad --role")?;
            let threads: usize = arg(args, "--threads").map_or(Ok(8), str::parse).map_err(|e| format!("{e}"))?;
            let cap = if role == Role::Test { TEST_CAP } else { CALIBRATION_CAP };
            let text = fs::read_to_string(c10_path(role)).map_err(|e| e.to_string())?;
            let insts: Vec<LadderInstance> = text
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(|l| serde_json::from_str(l).map_err(|e| e.to_string()))
                .collect::<Result<_, _>>()?;
            let t = std::time::Instant::now();
            let recs = run_all(entry, &insts, cap, threads);
            let out = PathBuf::from(format!("results/{}/{R8_LADDER}/{name}.jsonl", role.name()));
            fs::create_dir_all(out.parent().expect("parent")).map_err(|e| e.to_string())?;
            fs::write(&out, jsonl(&recs)).map_err(|e| e.to_string())?;
            let solved = recs.iter().filter(|r| r.status == "solved").count();
            let errors = recs.iter().filter(|r| r.status == "error").count();
            eprintln!(
                "{name}: {solved}/{} solved, {errors} errors, cap {cap}, {:.1} s -> {}",
                recs.len(),
                t.elapsed().as_secs_f64(),
                out.display()
            );
            Ok(())
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some(cmd @ ("gen-c10" | "check-c10" | "run-c10")) => match c10(cmd, &args) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::FAILURE
            }
        },
        _ => hard_ladder::cli::main_with(ENTRIES),
    }
}
