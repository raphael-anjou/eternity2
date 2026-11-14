//! resumable_runs: long bb-mrv-fast runs with progress records, checkpoint
//! and resume, plus the gate and timing stages of `../PREREG.md`.
//!
//! ```text
//! resumable_runs run    --ladder L --role R [--sizes 9] [--seeds 1-8] --cap C --out ROWS.jsonl
//!                       [--progress P.jsonl] [--progress-every 1000000]
//!                       [--checkpoint-dir D] [--checkpoint-every M] [--guard 0]
//! resumable_runs resume --from CKPT.json|DIR --cap C --out ROWS.jsonl
//!                       [--progress P.jsonl] [--progress-every N]
//!                       [--checkpoint-dir D] [--checkpoint-every M] [--guard 0]
//! resumable_runs gate-a --ladder L --role R [--sizes ..] [--seeds ..] --cap C
//!                       --progress P.jsonl --progress-every N --out ROWS.jsonl
//! resumable_runs time   --ladder L --role R [--sizes ..] [--seeds ..] --cap C
//!                       --progress P.jsonl --out ROWS.jsonl
//! resumable_runs tamper --from CKPT.json --extra 10000 --work DIR --out ROWS.jsonl
//! ```
//!
//! `run`, `resume` and `gate-a` exit with code 1 when any row they write has
//! status `error` (and `gate-a` also on any mismatch); `tamper` exits 1 when
//! any alteration was accepted or the unaltered copy did not resume
//! identically.
//!
//! `--cap` is always the total node cap over all segments. A checkpoint is
//! written to `<checkpoint-dir>/<instance>.ckpt.json` at the cap (and every
//! M nodes with `--checkpoint-every`). `--guard 0` disables the wall-clock
//! guard (recommended for long runs: a guard stop is not resumable).

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use hard_ladder::cli::load_instances;
use hard_ladder::{
    run_one, verify_board, Ladder, LadderInstance, LadderSolver, NodeCounter, Outcome, Role,
    SolverEntry,
};
use mrv_node_cost::{FastSolver, HashHook, NoHook};
use resumable_runs::{Checkpoint, Config, InstanceMeta, ResumableSolver};
use serde_json::{json, Value};

mod tamper;

fn arg<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
}

fn num(args: &[String], key: &str, default: u64) -> Result<u64, String> {
    arg(args, key).map_or(Ok(default), |s| {
        s.parse::<f64>()
            .map(|x| x as u64)
            .map_err(|_| format!("bad {key}"))
    })
}

fn guard(args: &[String]) -> Result<Duration, String> {
    let g = num(args, "--guard", 600)?;
    Ok(if g == 0 {
        Duration::MAX
    } else {
        Duration::from_secs(g)
    })
}

fn instances(args: &[String]) -> Result<Vec<LadderInstance>, String> {
    let ladder =
        Ladder::parse(arg(args, "--ladder").ok_or("--ladder required")?).ok_or("bad --ladder")?;
    let role = Role::parse(arg(args, "--role").unwrap_or("test")).ok_or("bad --role")?;
    let mut insts = load_instances(ladder, role)?;
    if let Some(s) = arg(args, "--sizes") {
        let sizes: Vec<u8> = s
            .split(',')
            .map(|x| x.parse().map_err(|_| "bad --sizes"))
            .collect::<Result<_, _>>()?;
        insts.retain(|i| sizes.contains(&i.n));
    }
    if let Some(s) = arg(args, "--seeds") {
        let (a, b) = s.split_once('-').ok_or("--seeds lo-hi")?;
        let (a, b): (u32, u32) = (
            a.parse().map_err(|_| "bad seed")?,
            b.parse().map_err(|_| "bad seed")?,
        );
        insts.retain(|i| (a..=b).contains(&i.seed));
    }
    Ok(insts)
}

fn meta(i: &LadderInstance) -> InstanceMeta {
    InstanceMeta {
        id: i.id.clone(),
        ladder: i.ladder.clone(),
        role: i.role.clone(),
        n: i.n,
        c: i.c,
        seed: i.seed,
    }
}

fn out_file(args: &[String]) -> Result<fs::File, String> {
    let out = arg(args, "--out").ok_or("--out required")?;
    if let Some(d) = Path::new(out).parent() {
        fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(out)
        .map_err(|e| e.to_string())
}

/// (status, board, error) as in Track 0's gate rows.
fn status(inst: &LadderInstance, out: &Outcome, err: Option<&String>) -> (String, Value, Value) {
    if let Some(e) = err {
        return ("error".into(), Value::Null, json!(e));
    }
    match out {
        Outcome::Solved(b) => match verify_board(&inst.puzzle, b) {
            Ok(()) => ("solved".into(), json!(b), Value::Null),
            Err(e) => (
                "error".into(),
                json!(b),
                json!(format!("unverified board: {e}")),
            ),
        },
        Outcome::CapReached => ("censored".into(), Value::Null, Value::Null),
        Outcome::Exhausted => ("error".into(), Value::Null, json!("exhausted")),
    }
}

/// One segment of a resumable run; returns its row.
fn segment(
    inst: &LadderInstance,
    cap_total: u64,
    cfg: Config,
    g: Duration,
    resumed_from: Option<&str>,
) -> Value {
    let base = cfg.resume.as_ref().map_or(0, |c| c.totals.nodes);
    let ck_state = cfg.resume.as_ref().map(|c| {
        (
            c.totals.clone(),
            c.segments,
            c.closed_share,
            c.stack.len() - 1,
            c.deepest,
            c.hash.clone(),
            c.hashed,
        )
    });
    let wall_before = cfg.resume.as_ref().map_or(0.0, |c| c.totals.elapsed_s);
    let seg_cap = cap_total.saturating_sub(base);
    let mut s = ResumableSolver::new(g, HashHook::default(), cfg);
    let mut counter = NodeCounter::new(seg_cap);
    let t = Instant::now();
    let out = if seg_cap == 0 {
        // Nothing to search: report the checkpoint's own state.
        if let Some((t, segs, share, depth, deepest, h, n)) = ck_state {
            s.report.base = t;
            s.report.segment = segs + 1;
            s.report.closed_share = share;
            s.report.depth = depth;
            s.report.deepest = deepest;
            if let (Some(h), Some(n)) = (h, n) {
                s.hook.h = u64::from_str_radix(&h, 16).unwrap_or(0);
                s.hook.n = n;
            }
        }
        s.error = Some("cap is not above the checkpoint's node count".into());
        Outcome::Exhausted
    } else {
        s.solve(&inst.puzzle, &mut counter)
    };
    let wall_ms = t.elapsed().as_secs_f64() * 1000.0;
    let (st, board, err) = status(inst, &out, s.error.as_ref());
    let r = &s.report;
    let cap_write = r.writes.last().filter(|_| r.checkpoint.is_some());
    json!({
        "solver": "bb-mrv-fast-resumable", "instance": inst.id, "ladder": inst.ladder,
        "role": inst.role, "n": inst.n, "c": inst.c, "seed": inst.seed,
        "segment": r.segment, "cap": cap_total, "segment_cap": seg_cap, "status": st,
        "nodes": r.base.nodes + counter.nodes(), "nodes_segment": counter.nodes(),
        "backtracks": r.base.backtracks + counter.backtracks, "backtracks_segment": counter.backtracks,
        "propagation": r.base.propagation + counter.propagation, "propagation_segment": counter.propagation,
        "board": board, "error": err,
        "hash": format!("{:016x}", s.hook.h), "hashed": s.hook.n,
        "closed_share": r.closed_share, "depth": r.depth, "deepest": r.deepest,
        "checkpoint": r.checkpoint, "resumable": r.checkpoint.is_some(),
        "resumed_from": resumed_from, "guard_fired": r.guard_fired,
        "wall_ms": wall_ms, "wall_ms_total": wall_before * 1000.0 + wall_ms,
        "replay_ms": r.replay_ms, "checkpoint_write_ms": cap_write.map(|w| w.ms),
        "checkpoint_bytes": cap_write.map(|w| w.bytes), "checkpoint_writes": r.writes,
        "progress_lines": r.progress_lines, "model_fnv": r.model_fnv,
    })
}

struct Opts {
    progress: Option<PathBuf>,
    progress_every: u64,
    ckdir: Option<PathBuf>,
    checkpoint_every: u64,
}

fn opts(args: &[String]) -> Result<Opts, String> {
    Ok(Opts {
        progress: arg(args, "--progress").map(PathBuf::from),
        progress_every: num(args, "--progress-every", 1_000_000)?,
        ckdir: arg(args, "--checkpoint-dir").map(PathBuf::from),
        checkpoint_every: num(args, "--checkpoint-every", 0)?,
    })
}

fn cfg_for(o: &Opts, inst: &LadderInstance, resume: Option<Checkpoint>, cap: u64) -> Config {
    Config {
        cap_total: cap,
        meta: meta(inst),
        progress: o.progress.clone(),
        progress_every: o.progress_every,
        checkpoint: o
            .ckdir
            .as_ref()
            .map(|d| d.join(format!("{}.ckpt.json", inst.id))),
        checkpoint_every: o.checkpoint_every,
        resume,
    }
}

fn log_line(f: &mut fs::File, v: &Value) -> Result<(), String> {
    writeln!(f, "{v}").map_err(|e| e.to_string())
}

fn is_error(row: &Value) -> bool {
    row["status"] == "error"
}

/// Row for a checkpoint that could not be loaded or matched to an instance.
fn load_error_row(path: &str, cap: u64, e: &str) -> Value {
    json!({
        "solver": "bb-mrv-fast-resumable", "instance": Value::Null, "status": "error",
        "error": e, "cap": cap, "resumed_from": path, "nodes": Value::Null,
        "segment": Value::Null, "resumable": false, "checkpoint": Value::Null,
    })
}

fn log_row(f: &mut fs::File, row: &Value) -> Result<(), String> {
    eprintln!(
        "{} seg {} {} nodes={} share={:.6} depth={} deepest={} ms={:.0}",
        row["instance"].as_str().unwrap_or("?"),
        row["segment"],
        row["status"],
        row["nodes"],
        row["closed_share"].as_f64().unwrap_or(0.0),
        row["depth"],
        row["deepest"],
        row["wall_ms"].as_f64().unwrap_or(0.0)
    );
    writeln!(f, "{row}").map_err(|e| e.to_string())
}

/// Ok(number of error rows written).
fn run(args: &[String]) -> Result<usize, String> {
    let insts = instances(args)?;
    let cap = num(args, "--cap", 1_000_000)?;
    let g = guard(args)?;
    let o = opts(args)?;
    let mut f = out_file(args)?;
    let mut errors = 0;
    for inst in &insts {
        let row = segment(inst, cap, cfg_for(&o, inst, None, cap), g, None);
        errors += usize::from(is_error(&row));
        log_row(&mut f, &row)?;
    }
    Ok(errors)
}

fn find_instance(meta: &InstanceMeta) -> Result<LadderInstance, String> {
    let l = Ladder::parse(&meta.ladder).ok_or("bad ladder in checkpoint")?;
    let r = Role::parse(&meta.role).ok_or("bad role in checkpoint")?;
    load_instances(l, r)?
        .into_iter()
        .find(|i| i.id == meta.id)
        .ok_or_else(|| format!("instance {} not found", meta.id))
}

/// Ok(number of error rows written). A checkpoint that fails to load gets
/// an error row; the other checkpoints of a directory still run.
fn resume(args: &[String]) -> Result<usize, String> {
    let from = PathBuf::from(arg(args, "--from").ok_or("--from required")?);
    let mut files: Vec<PathBuf> = if from.is_dir() {
        fs::read_dir(&from)
            .map_err(|e| e.to_string())?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.to_string_lossy().ends_with(".ckpt.json"))
            .collect()
    } else {
        vec![from]
    };
    files.sort();
    let cap = num(args, "--cap", 1_000_000)?;
    let g = guard(args)?;
    let o = opts(args)?;
    let mut f = out_file(args)?;
    let mut cache: Vec<((String, String), Vec<LadderInstance>)> = Vec::new();
    let mut errors = 0;
    for p in &files {
        let ps = p.display().to_string();
        let found = Checkpoint::load(p).and_then(|ck| {
            let key = (ck.instance.ladder.clone(), ck.instance.role.clone());
            if !cache.iter().any(|(k, _)| *k == key) {
                let l = Ladder::parse(&key.0).ok_or("bad ladder in checkpoint")?;
                let r = Role::parse(&key.1).ok_or("bad role in checkpoint")?;
                cache.push((key.clone(), load_instances(l, r)?));
            }
            let insts = &cache.iter().find(|(k, _)| *k == key).expect("cached").1;
            let inst = insts
                .iter()
                .find(|i| i.id == ck.instance.id)
                .ok_or_else(|| format!("instance {} not found", ck.instance.id))?;
            Ok((ck, inst.clone()))
        });
        let row = match found {
            Ok((ck, inst)) => segment(&inst, cap, cfg_for(&o, &inst, Some(ck), cap), g, Some(&ps)),
            Err(e) => load_error_row(&ps, cap, &e),
        };
        errors += usize::from(is_error(&row));
        log_row(&mut f, &row)?;
    }
    Ok(errors)
}

/// Unmodified bb-mrv-fast with the hash hook (Track 0's gate arm).
fn fast_row(inst: &LadderInstance, cap: u64) -> Value {
    let mut s = FastSolver::<HashHook> {
        guard: Duration::from_secs(600),
        hook: HashHook::default(),
    };
    let mut counter = NodeCounter::new(cap);
    let t = Instant::now();
    let out = s.solve(&inst.puzzle, &mut counter);
    let ms = t.elapsed().as_secs_f64() * 1000.0;
    let (st, board, err) = status(inst, &out, None);
    json!({
        "status": st, "nodes": counter.nodes(), "backtracks": counter.backtracks,
        "propagation": counter.propagation, "board": board, "error": err,
        "hash": format!("{:016x}", s.hook.h), "hashed": s.hook.n, "ms": ms,
    })
}

const KEYS: [&str; 7] = [
    "status",
    "nodes",
    "backtracks",
    "propagation",
    "board",
    "hash",
    "hashed",
];

fn gate_a(args: &[String]) -> Result<usize, String> {
    let insts = instances(args)?;
    let cap = num(args, "--cap", 1_000_000)?;
    let o = opts(args)?;
    let mut f = out_file(args)?;
    let mut bad = 0;
    for inst in &insts {
        let fast = fast_row(inst, cap);
        let res = segment(
            inst,
            cap,
            cfg_for(&o, inst, None, cap),
            Duration::from_secs(600),
            None,
        );
        let eq = KEYS.iter().all(|k| fast[k] == res[k]);
        if !eq || is_error(&fast) || is_error(&res) {
            bad += 1;
        }
        eprintln!(
            "{} {} nodes={} hash={} equal={eq}",
            inst.id, fast["status"], fast["nodes"], fast["hash"]
        );
        let row = json!({
            "instance": inst.id, "ladder": inst.ladder, "role": inst.role, "n": inst.n,
            "c": inst.c, "seed": inst.seed, "cap": cap, "fast": fast, "resumable": res, "equal": eq,
        });
        writeln!(f, "{row}").map_err(|e| e.to_string())?;
    }
    eprintln!(
        "gate-a: {} instances, {bad} mismatches or errors",
        insts.len()
    );
    Ok(bad)
}

// ---- timing: run_one through SolverEntry ----------------------------------

static TIMING_CFG: Mutex<Option<(InstanceMeta, PathBuf)>> = Mutex::new(None);

const ARMS: [SolverEntry; 2] = [
    SolverEntry {
        name: "bb-mrv-fast",
        description: "Track 0 FastSolver, unmodified, no hook",
        run_seed: 0,
        make: |_| -> Box<dyn LadderSolver> {
            Box::new(FastSolver::<NoHook> {
                guard: Duration::from_secs(600),
                hook: NoHook,
            })
        },
    },
    SolverEntry {
        name: "bb-mrv-fast-resumable",
        description: "ResumableSolver, no hook, progress every 1e6 nodes, no checkpoint",
        run_seed: 0,
        make: |_| -> Box<dyn LadderSolver> {
            let (m, p) = TIMING_CFG
                .lock()
                .expect("lock")
                .clone()
                .expect("timing cfg");
            Box::new(ResumableSolver::new(
                Duration::from_secs(600),
                NoHook,
                Config {
                    meta: m,
                    progress: Some(p),
                    progress_every: 1_000_000,
                    checkpoint: None,
                    checkpoint_every: 0,
                    resume: None,
                    cap_total: 0,
                },
            ))
        },
    },
];

/// Per instance, 10 runs in the order A B B A A B B A A B.
fn time(args: &[String]) -> Result<usize, String> {
    let insts = instances(args)?;
    let cap = num(args, "--cap", 1_000_000)?;
    let p = PathBuf::from(arg(args, "--progress").ok_or("--progress required")?);
    let mut f = out_file(args)?;
    const PATTERN: [usize; 10] = [0, 1, 1, 0, 0, 1, 1, 0, 0, 1];
    for inst in &insts {
        *TIMING_CFG.lock().expect("lock") = Some((meta(inst), p.clone()));
        let mut rep = [0usize; 2];
        for (slot, &arm) in PATTERN.iter().enumerate() {
            let rec = run_one(&ARMS[arm], inst, cap);
            let mut v = serde_json::to_value(&rec).map_err(|e| e.to_string())?;
            v["slot"] = json!(slot);
            v["rep"] = json!(rep[arm]);
            rep[arm] += 1;
            writeln!(f, "{v}").map_err(|e| e.to_string())?;
            eprintln!(
                "{} {} slot {slot} {} nodes={} ms={:.1}",
                inst.id, rec.solver, rec.status, rec.nodes, rec.wall_ms
            );
        }
    }
    Ok(0)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let res = match args.first().map(String::as_str) {
        Some("run") => run(&args),
        Some("resume") => resume(&args),
        Some("gate-a") => gate_a(&args),
        Some("time") => time(&args),
        Some("tamper") => tamper::tamper(&args),
        _ => Err("usage: resumable_runs run|resume|gate-a|time|tamper (see src/main.rs)".into()),
    };
    match res {
        Ok(0) => ExitCode::SUCCESS,
        Ok(n) => {
            eprintln!("error: {n} row(s) with status error or mismatch");
            ExitCode::FAILURE
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
