//! Unit tests for the fix round (PREREG_AMENDMENTS.md A1): checkpoint digest
//! and load checks (D1), exit codes and the too-low-cap row (D2), closed
//! share on a root refutation (D3), the progress directory. Calibration seeds
//! only. Run with `cargo test --release`.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use eternity2_engine::Puzzle;
use hard_ladder::cli::load_instances;
use hard_ladder::{Ladder, LadderInstance, LadderSolver, NodeCounter, Outcome, Role};
use mrv_node_cost::HashHook;
use resumable_runs::{Checkpoint, Config, InstanceMeta, ResumableSolver, VERSION};
use serde_json::{json, Value};

const BIN: &str = env!("CARGO_BIN_EXE_resumable_runs");

fn tmp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("resumable-runs-test-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn cal7() -> LadderInstance {
    load_instances(
        Ladder::parse("primary").unwrap(),
        Role::parse("calibration").unwrap(),
    )
    .unwrap()
    .into_iter()
    .find(|i| i.n == 7 && i.seed == 1001)
    .unwrap()
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

/// Run 7x7 calibration 1001 to `cap` with a checkpoint; return its text.
fn checkpoint_text(dir: &Path, cap: u64) -> String {
    let inst = cal7();
    let p = dir.join("c.ckpt.json");
    let cfg = Config {
        meta: meta(&inst),
        checkpoint: Some(p.clone()),
        cap_total: cap,
        ..Config::default()
    };
    let mut s = ResumableSolver::new(Duration::MAX, HashHook::default(), cfg);
    let out = s.solve(&inst.puzzle, &mut NodeCounter::new(cap));
    assert!(
        matches!(out, Outcome::CapReached),
        "calibration 1001 must not solve by {cap}"
    );
    std::fs::read_to_string(p).unwrap()
}

fn edit(text: &str, f: impl FnOnce(&mut Value)) -> String {
    let mut v: Value = serde_json::from_str(text).unwrap();
    f(&mut v);
    serde_json::to_string(&v).unwrap()
}

#[test]
fn digest_round_trip_and_refusals() {
    let d = tmp("digest");
    let text = checkpoint_text(&d, 50_000);
    let ck = Checkpoint::parse(&text).expect("written checkpoint loads");
    assert_eq!(ck.version, VERSION);
    assert_eq!(ck.cap, 50_000);
    assert_eq!(ck.digest, ck.content_digest());

    let last = ck.stack.len() - 1;
    let edits: Vec<(&str, String)> = vec![
        (
            "last index",
            edit(&text, |v| {
                let i = v["stack"][last]["index"].as_u64().unwrap();
                v["stack"][last]["index"] = json!(if i == 0 { 1 } else { i - 1 });
            }),
        ),
        (
            "nodes",
            edit(&text, |v| v["totals"]["nodes"] = json!(50_001)),
        ),
        (
            "backtracks",
            edit(&text, |v| {
                let b = v["totals"]["backtracks"].as_u64().unwrap();
                v["totals"]["backtracks"] = json!(b + 7);
            }),
        ),
        (
            "hash",
            edit(&text, |v| v["hash"] = json!("0000000000000000")),
        ),
        ("hashed", edit(&text, |v| v["hashed"] = json!(5))),
        ("cap", edit(&text, |v| v["cap"] = json!(60_000))),
        (
            "option",
            edit(&text, |v| v["stack"][0]["options"][0][1] = json!(3)),
        ),
        ("unknown field", edit(&text, |v| v["extra"] = json!(1))),
    ];
    for (what, t) in edits {
        let e = Checkpoint::parse(&t).expect_err(what);
        assert!(
            e.contains("digest mismatch") || e.contains("unknown field"),
            "{what}: {e}"
        );
    }
    let v1 = edit(&text, |v| v["version"] = json!(1));
    assert!(Checkpoint::parse(&v1)
        .unwrap_err()
        .contains("version 1 cannot be resumed"));
    for cut in [0, 1, text.len() / 2, text.len() - 1] {
        assert!(
            Checkpoint::parse(&text[..cut]).is_err(),
            "truncated at {cut}"
        );
    }
}

#[test]
fn checks_without_digest() {
    // A forged digest does not get past the digest-independent checks.
    let d = tmp("forged");
    let text = checkpoint_text(&d, 50_000);
    let forge = |f: &dyn Fn(&mut Checkpoint)| {
        let mut ck = Checkpoint::parse(&text).unwrap();
        f(&mut ck);
        ck.seal();
        Checkpoint::parse(&serde_json::to_string(&ck).unwrap())
    };
    assert!(forge(&|c| c.totals.nodes += 1).is_err()); // propagation = nodes + 1
    assert!(forge(&|c| c.hashed = Some(5)).is_err());
    assert!(forge(&|c| c.cap = c.totals.nodes - 1).is_err());
    assert!(forge(&|c| c.totals.backtracks = c.totals.nodes + 1).is_err());
    assert!(forge(&|c| c.closed_share = 1.0).is_err());
    assert!(forge(&|c| c.stack.clear()).is_err());
    assert!(forge(&|_| {}).is_ok());
}

#[test]
fn root_refutation_has_closed_share_one() {
    // No piece carries the border colour, so no corner cell has an option.
    let puzzle = Puzzle {
        name: "root-refuted".into(),
        width: 3,
        height: 3,
        num_colors: 1,
        pieces: vec![[1, 1, 1, 1]; 9],
        hints: vec![],
    };
    let mut s = ResumableSolver::new(Duration::MAX, HashHook::default(), Config::default());
    let mut counter = NodeCounter::new(1000);
    let out = s.solve(&puzzle, &mut counter);
    assert!(matches!(out, Outcome::Exhausted));
    assert_eq!(counter.nodes(), 0);
    assert_eq!(s.report.closed_share, 1.0);
}

fn bin(dir: &Path, args: &[&str]) -> (i32, Vec<Value>) {
    let out = dir.join("rows.jsonl");
    let _ = std::fs::remove_file(&out);
    let st = Command::new(BIN)
        .args(args)
        .arg("--out")
        .arg(&out)
        .stderr(std::process::Stdio::null())
        .status()
        .unwrap();
    let rows = std::fs::read_to_string(&out)
        .unwrap_or_default()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    (st.code().unwrap(), rows)
}

#[test]
fn exit_codes_and_low_cap_row() {
    let d = tmp("exit");
    let ck = d.join("ck");
    let cks = ck.to_str().unwrap();
    let common = [
        "--ladder",
        "primary",
        "--role",
        "calibration",
        "--sizes",
        "7",
        "--seeds",
        "1001-1001",
    ];
    let mut a = vec!["run", "--cap", "40000", "--checkpoint-dir", cks];
    a.extend(common);
    let (code, rows) = bin(&d, &a);
    assert_eq!(code, 0);
    assert_eq!(rows[0]["status"], "censored");
    let f = ck.join(format!(
        "{}.ckpt.json",
        rows[0]["instance"].as_str().unwrap()
    ));
    let fs_ = f.to_str().unwrap();

    // Cap at or below the checkpoint: error row with the checkpoint's totals, exit 1.
    for cap in ["40000", "30000"] {
        let (code, rows) = bin(&d, &["resume", "--from", fs_, "--cap", cap]);
        assert_eq!(code, 1, "cap {cap}");
        assert_eq!(rows[0]["status"], "error");
        assert_eq!(rows[0]["nodes"], 40_000);
        assert_eq!(rows[0]["segment"], 2);
        assert_eq!(rows[0]["hashed"], 40_000);
    }
    // A valid resume exits 0.
    let (code, rows) = bin(&d, &["resume", "--from", fs_, "--cap", "45000"]);
    assert_eq!((code, rows[0]["status"].as_str()), (0, Some("censored")));
    assert_eq!(rows[0]["nodes"], 45_000);

    // An altered checkpoint: error row and exit 1.
    let text = std::fs::read_to_string(&f).unwrap();
    let bad = d.join("bad.ckpt.json");
    std::fs::write(&bad, edit(&text, |v| v["totals"]["backtracks"] = json!(1))).unwrap();
    let (code, rows) = bin(
        &d,
        &["resume", "--from", bad.to_str().unwrap(), "--cap", "45000"],
    );
    assert_eq!(code, 1);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["status"], "error");
    assert!(rows[0]["error"]
        .as_str()
        .unwrap()
        .contains("digest mismatch"));
}

#[test]
fn progress_directory() {
    let d = tmp("progress");
    // Missing directory: created, records written.
    let p = d.join("a/b/progress.jsonl");
    let (code, _) = bin(
        &d,
        &[
            "run",
            "--ladder",
            "primary",
            "--role",
            "calibration",
            "--sizes",
            "7",
            "--seeds",
            "1001-1001",
            "--cap",
            "20000",
            "--progress",
            p.to_str().unwrap(),
            "--progress-every",
            "5000",
        ],
    );
    assert_eq!(code, 0);
    assert_eq!(std::fs::read_to_string(&p).unwrap().lines().count(), 4);
    // Directory that cannot be created (its parent is a file): error row, exit 1.
    std::fs::write(d.join("file"), "x").unwrap();
    let q = d.join("file/sub/progress.jsonl");
    let (code, rows) = bin(
        &d,
        &[
            "run",
            "--ladder",
            "primary",
            "--role",
            "calibration",
            "--sizes",
            "7",
            "--seeds",
            "1001-1001",
            "--cap",
            "20000",
            "--progress",
            q.to_str().unwrap(),
        ],
    );
    assert_eq!(code, 1);
    assert_eq!(rows[0]["status"], "error");
    assert_eq!(rows[0]["nodes"], 0);
}
