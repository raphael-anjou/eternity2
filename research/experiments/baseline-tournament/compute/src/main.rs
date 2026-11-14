//! Baseline tournament runner (Track 5, night shift 2026-11-12; see ../PREREG.md).
//!
//! Every entrant is a hard-ladder `SolverEntry` built from its own experiment's
//! crate, and every run goes through `hard_ladder::run_one` (the ladder's
//! unchanged runner and board check), so all entrants share one counter
//! (`NodeCounter::try_assign`) and one cap.
//!
//! tournament list
//! tournament run --entrant NAME [--seeds 1,2,3] [--rungs R6,R7,R8] [--threads 4] [--out-dir DIR]
//!
//! Output: `<out-dir>/<entrant>/s<seed>-<rung>.jsonl`, one hard-ladder
//! `RunRecord` per instance plus tournament metadata. A file is written only
//! when all 30 rows of that (seed, rung) are done; existing files are skipped.

use std::cell::Cell;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Duration;

use backdoor_branching::{learned_rule, single_rule, BbSolver, Rule as BbRule};
use eternity2_solver_trait::Solver;
use hard_ladder::cli::load_instances;
use hard_ladder::solvers::scb::Scb;
use hard_ladder::{run_one, Ladder, LadderInstance, LadderSolver, Role, SolverEntry, TEST_CAP};
use ktam::{Rule as KRule, Unassign};
use mrv_baseline::scb::GuardScb;
use mrv_baseline::{MrvSolver, Order};

// Machine description, recorded in every row. Defaults are the x86 laptop's; the
// m1 runs set TOURNAMENT_MACHINE, TOURNAMENT_CPU and TOURNAMENT_LOAD.
fn env_or(k: &str, d: &str) -> String {
    std::env::var(k).unwrap_or_else(|_| d.to_string())
}
fn machine() -> String {
    env_or("TOURNAMENT_MACHINE", "x86")
}
fn cpu() -> String {
    env_or(
        "TOURNAMENT_CPU",
        "Intel Core Ultra 7 165U (14 threads), 4 threads assigned",
    )
}
fn load() -> String {
    env_or(
        "TOURNAMENT_LOAD",
        "8 other single-thread runs (Track 1) shared the machine; wall clock indicative only",
    )
}

const A22_GUARD: Duration = Duration::from_secs(600);
const A16_GUARD: Duration = Duration::from_secs(240);

thread_local! {
    /// Instance seed of the run on this thread (A15's per-instance seed formula).
    static CUR_SEED: Cell<u64> = const { Cell::new(0) };
}
fn ktam_seed(run_seed: u64) -> u64 {
    run_seed * 1_000_003 + CUR_SEED.with(Cell::get)
}

fn mrvb_ad() -> Box<dyn Solver> {
    Box::new(mrvb_solver_engine::EngineSolver::gacolor_ac3_random().with_alldiff(true))
}
fn mrvb_off() -> Box<dyn Solver> {
    Box::new(mrvb_solver_engine::EngineSolver::gacolor_ac3_random())
}

macro_rules! cmp {
    ($name:literal, $scope:literal, $period:literal) => {
        SolverEntry {
            name: $name,
            description: "cut-multiset-pruning (A17) fork, gacolor_ac3_random + Hall",
            run_seed: 1,
            make: |seed| -> Box<dyn LadderSolver> {
                Box::new(Scb {
                    make: || -> Box<dyn Solver> {
                        Box::new(
                            cmp_solver_engine::EngineSolver::gacolor_ac3_random().with_hall(
                                cmp_solver_engine::HallMode {
                                    scope: $scope,
                                    period: $period,
                                },
                            ),
                        )
                    },
                    seed,
                })
            },
        }
    };
}

macro_rules! scbp {
    ($name:literal, $ctor:ident) => {
        SolverEntry {
            name: $name,
            description: "single-core-benchmark solver-engine preset via hard-ladder Scb",
            run_seed: 0,
            make: |seed| -> Box<dyn LadderSolver> {
                Box::new(Scb {
                    make: || -> Box<dyn Solver> {
                        Box::new(eternity2_solver_engine::EngineSolver::$ctor())
                    },
                    seed,
                })
            },
        }
    };
}

/// Entrant metadata: (source, stochastic, complete, guard seconds).
struct Entrant {
    entry: SolverEntry,
    source: &'static str,
    stoch: bool,
    complete: bool,
    guard_s: Option<u64>,
}

fn hl(name: &str) -> SolverEntry {
    *hard_ladder::registry::BUILTIN
        .iter()
        .find(|e| e.name == name)
        .expect("hard-ladder builtin")
}
fn pc(name: &str) -> SolverEntry {
    *probcut::SOLVERS
        .iter()
        .find(|e| e.name == name)
        .expect("probcut solver")
}

fn engine_dfs(name: &'static str) -> SolverEntry {
    // Same wrapper as hard-ladder's engine_dfs! macro (path seed 0).
    macro_rules! e {
        ($k:literal) => {
            |_| -> Box<dyn LadderSolver> {
                Box::new(hard_ladder::solvers::engine_dfs::EngineDfs { path_kind: $k })
            }
        };
    }
    let make: fn(u64) -> Box<dyn LadderSolver> = match name {
        "engine-dfs-column-major" => e!("column-major"),
        "engine-dfs-spiral-out" => e!("spiral-out"),
        "engine-dfs-diagonal" => e!("diagonal"),
        "engine-dfs-double-snake" => e!("double-snake"),
        "engine-dfs-random" => e!("random"),
        _ => unreachable!(),
    };
    SolverEntry {
        name,
        description: "main engine plain DFS via hard-ladder EngineDfs",
        run_seed: 0,
        make,
    }
}

#[allow(clippy::too_many_lines)]
fn entrants() -> Vec<Entrant> {
    let mut v = Vec::new();
    let mut add = |entry: SolverEntry,
                   source: &'static str,
                   stoch: bool,
                   complete: bool,
                   guard_s: Option<u64>| {
        v.push(Entrant {
            entry,
            source,
            stoch,
            complete,
            guard_s,
        });
    };
    add(
        SolverEntry {
            name: "bb-mrv",
            description: "A22 binding arm = A16 BbSolver Rule::Mrv, guard 600 s",
            run_seed: 0,
            make: |_| -> Box<dyn LadderSolver> {
                Box::new(BbSolver {
                    rule: BbRule::Mrv,
                    guard: A22_GUARD,
                })
            },
        },
        "mrv-baseline (A22) / backdoor-branching (A16)",
        false,
        true,
        Some(600),
    );
    add(
        hl("scb-gacolor_ac3_random"),
        "hard-ladder registry",
        true,
        true,
        None,
    );
    add(hl("hp-dfs"), "hard-ladder registry", false, true, None);
    for n in [
        "engine-dfs-spiral-in",
        "engine-dfs-border-first",
        "engine-dfs-snake",
        "scb-naive_rowmajor",
        "scb-naive_spiral",
        "scb-border_first_lcv",
        "scb-border_first_full",
    ] {
        add(hl(n), "hard-ladder registry", false, true, None);
    }
    add(
        hl("scb-border_first_random"),
        "hard-ladder registry",
        true,
        true,
        None,
    );
    for n in [
        "scb-rare_color_first",
        "example-dfs",
        "scb-gacolor_ac3",
        "scb-gacolor_ac3_ns1",
        "scb-gacolor_ac3_lcv",
    ] {
        add(hl(n), "hard-ladder registry", false, true, None);
    }
    add(
        SolverEntry {
            name: "mrv-nohs",
            description: "A22: bb-mrv without hidden singles",
            run_seed: 0,
            make: |_| -> Box<dyn LadderSolver> {
                Box::new(MrvSolver {
                    order: Order::Free,
                    random: None,
                    hs: false,
                    guard: A22_GUARD,
                })
            },
        },
        "mrv-baseline (A22)",
        false,
        true,
        Some(600),
    );
    add(
        SolverEntry {
            name: "mrv-bf",
            description: "A22: bb-mrv with border-first order",
            run_seed: 0,
            make: |_| -> Box<dyn LadderSolver> {
                Box::new(MrvSolver {
                    order: Order::BorderFirst,
                    random: None,
                    hs: true,
                    guard: A22_GUARD,
                })
            },
        },
        "mrv-baseline (A22)",
        false,
        true,
        Some(600),
    );
    add(
        SolverEntry {
            name: "mrv-rand",
            description: "A22: bb-mrv with random value order",
            run_seed: 1,
            make: |s| -> Box<dyn LadderSolver> {
                Box::new(MrvSolver {
                    order: Order::Free,
                    random: Some(s),
                    hs: true,
                    guard: A22_GUARD,
                })
            },
        },
        "mrv-baseline (A22)",
        true,
        true,
        Some(600),
    );
    add(
        SolverEntry {
            name: "scb-ad",
            description: "A22: scb + A3 piece all-different (engine fork)",
            run_seed: 1,
            make: |s| -> Box<dyn LadderSolver> {
                Box::new(GuardScb {
                    make: mrvb_ad,
                    seed: s,
                    guard: A22_GUARD,
                })
            },
        },
        "mrv-baseline (A22)",
        true,
        true,
        Some(600),
    );
    add(pc("fc-dfs"), "probcut (A12)", false, true, None);
    add(pc("rm-dfs"), "probcut (A12)", false, true, None);
    add(pc("fc-luby"), "probcut (A12)", true, true, None);
    add(
        pc("probcut-v1"),
        "probcut (A12), cut thresholds calibrated on planted prefixes of calibration seeds",
        false,
        true,
        None,
    );
    add(
        pc("probcut-v2"),
        "probcut (A12), cut thresholds calibrated on planted prefixes of calibration seeds",
        false,
        true,
        None,
    );
    add(
        SolverEntry {
            name: "ktam-b3",
            description: "A15 B3 random eviction b3-rinf-k2",
            run_seed: 1,
            make: |s| -> Box<dyn LadderSolver> {
                Box::new(Unassign::new(KRule::Random { r: None, k: 2 }, ktam_seed(s)))
            },
        },
        "ktam-unassignment (A15)",
        true,
        false,
        None,
    );
    add(
        SolverEntry {
            name: "ktam-v1",
            description: "A15 V1 kTAM detachment v1-beta2-rinf-k1",
            run_seed: 1,
            make: |s| -> Box<dyn LadderSolver> {
                Box::new(Unassign::new(
                    KRule::Ktam {
                        beta: 2.0,
                        r: None,
                        k: 1,
                    },
                    ktam_seed(s),
                ))
            },
        },
        "ktam-unassignment (A15)",
        true,
        false,
        None,
    );
    add(
        SolverEntry {
            name: "ktam-v2",
            description: "A15 V2 IMS blocker eviction v2-t0-n0.1",
            run_seed: 1,
            make: |s| -> Box<dyn LadderSolver> {
                Box::new(Unassign::new(
                    KRule::Ims {
                        tabu: 0,
                        noise: 0.1,
                    },
                    ktam_seed(s),
                ))
            },
        },
        "ktam-unassignment (A15)",
        true,
        false,
        None,
    );
    add(
        cmp!("cmp-hall-frontier", 1, 1),
        "cut-multiset-pruning (A17)",
        true,
        true,
        None,
    );
    add(
        cmp!("cmp-hall-all-k4", 2, 4),
        "cut-multiset-pruning (A17)",
        true,
        true,
        None,
    );
    add(
        cmp!("cmp-hall-all", 2, 1),
        "cut-multiset-pruning (A17)",
        true,
        true,
        None,
    );
    add(
        cmp!("cmp-hall-frontier-k4", 1, 4),
        "cut-multiset-pruning (A17)",
        true,
        true,
        None,
    );
    add(
        SolverEntry {
            name: "bb-impact",
            description: "A16 impact branching, guard 240 s",
            run_seed: 0,
            make: |_| -> Box<dyn LadderSolver> {
                Box::new(BbSolver {
                    rule: BbRule::Impact,
                    guard: A16_GUARD,
                })
            },
        },
        "backdoor-branching (A16)",
        false,
        true,
        Some(240),
    );
    add(
        SolverEntry {
            name: "bb-learned",
            description:
                "A16 learned backdoor score (model trained on A3 planted labels), guard 240 s",
            run_seed: 0,
            make: |_| -> Box<dyn LadderSolver> {
                Box::new(BbSolver {
                    rule: learned_rule(),
                    guard: A16_GUARD,
                })
            },
        },
        "backdoor-branching (A16), model trained on A3 planted backdoor labels",
        false,
        true,
        Some(240),
    );
    add(
        SolverEntry {
            name: "bb-single",
            description: "A16 best single feature, guard 240 s",
            run_seed: 0,
            make: |_| -> Box<dyn LadderSolver> {
                Box::new(BbSolver {
                    rule: single_rule(),
                    guard: A16_GUARD,
                })
            },
        },
        "backdoor-branching (A16)",
        false,
        true,
        Some(240),
    );
    for n in [
        "engine-dfs-column-major",
        "engine-dfs-spiral-out",
        "engine-dfs-diagonal",
        "engine-dfs-double-snake",
        "engine-dfs-random",
    ] {
        add(
            engine_dfs(n),
            "main engine (engine/src/solver.rs) via hard-ladder EngineDfs",
            false,
            true,
            None,
        );
    }
    let src = "single-core-benchmark solver-engine preset via hard-ladder Scb";
    add(
        scbp!("scb-border_first_parity", border_first_parity),
        src,
        false,
        true,
        None,
    );
    add(
        scbp!("scb-border_first_gacolor", border_first_gacolor),
        src,
        false,
        true,
        None,
    );
    add(
        scbp!("scb-chess_gacolor", chess_gacolor),
        src,
        false,
        true,
        None,
    );
    add(
        scbp!("scb-gacolor_symbreak", gacolor_symbreak),
        src,
        false,
        true,
        None,
    );
    add(
        scbp!("scb-chess_gacolor_ac3", chess_gacolor_ac3),
        src,
        false,
        true,
        None,
    );
    add(
        scbp!("scb-joe_depth150", joe_depth150),
        src,
        false,
        true,
        None,
    );
    add(
        scbp!("scb-joe_depth150_bp", joe_depth150_bp),
        src,
        false,
        true,
        None,
    );
    add(
        scbp!("scb-joe_depth150_bp_rec", joe_depth150_bp_rec),
        src,
        false,
        true,
        None,
    );
    add(
        scbp!(
            "scb-joe_depth150_bp_rec_layered",
            joe_depth150_bp_rec_layered
        ),
        src,
        false,
        true,
        None,
    );
    add(
        scbp!("scb-verhaard_preferred", verhaard_preferred),
        src,
        false,
        true,
        None,
    );
    add(
        SolverEntry {
            name: "scb-fork-off",
            description: "A22 check arm: engine fork, all-different hook off",
            run_seed: 1,
            make: |s| -> Box<dyn LadderSolver> {
                Box::new(GuardScb {
                    make: mrvb_off,
                    seed: s,
                    guard: A22_GUARD,
                })
            },
        },
        "mrv-baseline (A22) check arm",
        true,
        true,
        Some(600),
    );
    add(
        cmp!("cmp-fork-off", 0, 1),
        "cut-multiset-pruning (A17) check arm",
        true,
        true,
        None,
    );
    v
}

fn load_probcut_params() -> Result<(), String> {
    let p =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../probcut/results/calibration/params.json");
    let text = fs::read_to_string(&p).map_err(|e| format!("{}: {e}", p.display()))?;
    let params: probcut::Params = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    probcut::LUBY_UNIT.store(params.luby_unit, Ordering::Relaxed);
    let _ = probcut::PARAMS.set(params);
    Ok(())
}

fn rung_instances(rung: &str) -> Result<Vec<LadderInstance>, String> {
    let prim = || load_instances(Ladder::Primary, Role::Test);
    let v: Vec<LadderInstance> = match rung {
        "R6" => prim()?.into_iter().filter(|i| i.n == 6).collect(),
        "R7" => prim()?.into_iter().filter(|i| i.n == 7).collect(),
        "R8" => {
            let mut a: Vec<LadderInstance> = load_instances(Ladder::EightC10, Role::Test)?
                .into_iter()
                .filter(|i| (21..=30).contains(&i.seed))
                .collect();
            a.extend(
                load_instances(Ladder::EightC10Fresh, Role::Test)?
                    .into_iter()
                    .filter(|i| (31..=50).contains(&i.seed)),
            );
            a
        }
        _ => return Err(format!("bad rung {rung}")),
    };
    if v.len() != 30 {
        return Err(format!("{rung}: {} instances, expected 30", v.len()));
    }
    Ok(v)
}

fn arg<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
}

fn run(args: &[String]) -> Result<(), String> {
    let name = arg(args, "--entrant").ok_or("--entrant required")?;
    let all = entrants();
    let ent = all
        .iter()
        .find(|e| e.entry.name == name)
        .ok_or_else(|| format!("unknown entrant {name}"))?;
    let seeds: Vec<u64> = if ent.stoch {
        arg(args, "--seeds")
            .unwrap_or("1")
            .split(',')
            .map(|s| s.parse().map_err(|_| "bad --seeds"))
            .collect::<Result<_, _>>()?
    } else {
        vec![ent.entry.run_seed]
    };
    let rungs: Vec<String> = arg(args, "--rungs")
        .unwrap_or("R6,R7,R8")
        .split(',')
        .map(String::from)
        .collect();
    let threads: usize = arg(args, "--threads")
        .unwrap_or("4")
        .parse()
        .map_err(|_| "bad --threads")?;
    let out_dir = PathBuf::from(arg(args, "--out-dir").unwrap_or("results/raw")).join(name);
    fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;

    // Jobs: (seed, rung, instance); groups whose file exists are skipped.
    let mut groups: Vec<(u64, String, Vec<LadderInstance>, PathBuf)> = Vec::new();
    for &s in &seeds {
        for r in &rungs {
            let f = out_dir.join(format!("s{s}-{r}.jsonl"));
            if f.exists() {
                eprintln!("skip {}", f.display());
                continue;
            }
            groups.push((s, r.clone(), rung_instances(r)?, f));
        }
    }
    let mut jobs: Vec<(usize, usize)> = Vec::new();
    for (g, grp) in groups.iter().enumerate() {
        for i in 0..grp.2.len() {
            jobs.push((g, i));
        }
    }
    // Largest boards first for load balance.
    jobs.sort_by_key(|&(g, i)| std::cmp::Reverse(groups[g].2[i].n));
    let results: Mutex<Vec<Vec<Option<serde_json::Value>>>> =
        Mutex::new(groups.iter().map(|g| vec![None; g.2.len()]).collect());
    let next = AtomicUsize::new(0);
    std::thread::scope(|sc| {
        for _ in 0..threads.max(1) {
            sc.spawn(|| loop {
                let k = next.fetch_add(1, Ordering::Relaxed);
                let Some(&(g, i)) = jobs.get(k) else { break };
                let (seed, rung, insts, path) = &groups[g];
                let inst = &insts[i];
                CUR_SEED.with(|c| c.set(u64::from(inst.seed)));
                let entry = SolverEntry {
                    run_seed: *seed,
                    ..ent.entry
                };
                let rec = run_one(&entry, inst, TEST_CAP);
                let time_censored = rec.status == "censored" && rec.nodes_counted < rec.cap;
                eprintln!(
                    "{} s{} {} {} {} nodes={} wall_ms={:.0}{}",
                    name,
                    seed,
                    rung,
                    rec.instance,
                    rec.status,
                    rec.nodes,
                    rec.wall_ms,
                    if time_censored { " TIME-CENSORED" } else { "" }
                );
                let mut v = serde_json::to_value(&rec).expect("json");
                let o = v.as_object_mut().expect("object");
                o.insert("entrant".into(), name.into());
                o.insert("rung".into(), rung.as_str().into());
                o.insert("source".into(), ent.source.into());
                o.insert("complete".into(), ent.complete.into());
                o.insert("stochastic".into(), ent.stoch.into());
                o.insert("guard_s".into(), ent.guard_s.into());
                o.insert("time_censored".into(), time_censored.into());
                o.insert("machine".into(), machine().into());
                o.insert("cpu".into(), cpu().into());
                o.insert("load".into(), load().into());
                let mut res = results.lock().expect("lock");
                res[g][i] = Some(v);
                if res[g].iter().all(Option::is_some) {
                    let mut f = fs::File::create(path).expect("create");
                    for r in &res[g] {
                        writeln!(
                            f,
                            "{}",
                            serde_json::to_string(r.as_ref().expect("row")).expect("json")
                        )
                        .expect("write");
                    }
                    eprintln!("wrote {}", path.display());
                }
            });
        }
    });
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(e) = load_probcut_params() {
        eprintln!("error: {e}");
        return ExitCode::FAILURE;
    }
    let res = match args.first().map(String::as_str) {
        Some("run") => run(&args),
        Some("list") => {
            for e in entrants() {
                println!(
                    "{:34} stoch={} complete={} guard={:?} | {}",
                    e.entry.name, e.stoch, e.complete, e.guard_s, e.source
                );
            }
            Ok(())
        }
        _ => Err("usage: tournament run|list (see src/main.rs)".into()),
    };
    match res {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
