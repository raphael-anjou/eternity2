//! Gate G-e (PREREG_AMENDMENTS.md A1): alter one checkpoint in every way the
//! enumeration below produces, resume each altered copy in a new process and
//! require a refusal (non-zero exit code and an error row). The unaltered
//! copy must resume to the same row as an uninterrupted run.
//!
//! Alterations, enumerated from the file, not listed by hand:
//! - every JSON leaf (number, string, null) at any depth: integers +1 and,
//!   when above 0, -1; floats to the next f64 up and +1; 16-hex strings with
//!   the last digit changed, other strings with a character appended; null
//!   to 0;
//! - every object key removed, and one unknown key added at each object;
//! - every array (each level's option list, each option, the stack): its
//!   last element removed, its first element duplicated, its first two
//!   elements swapped when they differ;
//! - `version` set to 1 and to 3;
//! - truncation at byte offsets 0, 1, len/4, len/2, 3len/4, len-2, len-1.
//!
//! Each structural or value alteration is also tried a second time with the
//! digest recomputed (a forged digest), to measure what the checks that do
//! not rely on the digest catch on their own. That second count is reported,
//! not gated: totals such as `backtracks` or `elapsed_s` and the node hash
//! cannot be recomputed from the stack.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use resumable_runs::Checkpoint;
use serde_json::{json, Value};

use crate::{arg, find_instance, num, out_file, segment, Config};

fn leaf_alterations(v: &Value) -> Vec<(String, Value)> {
    match v {
        Value::Number(n) => {
            if let Some(u) = n.as_u64() {
                let mut a = vec![("+1".to_string(), json!(u + 1))];
                if u > 0 {
                    a.push(("-1".into(), json!(u - 1)));
                }
                a
            } else {
                let x = n.as_f64().expect("number");
                vec![
                    ("next-up".into(), json!(f64::from_bits(x.to_bits() + 1))),
                    ("+1.0".into(), json!(x + 1.0)),
                ]
            }
        }
        Value::String(s) => {
            let hex = s.len() == 16 && s.bytes().all(|b| b.is_ascii_hexdigit());
            if hex {
                let last = s.as_bytes()[15];
                let d = if last == b'0' { '1' } else { '0' };
                vec![("hex-digit".into(), json!(format!("{}{d}", &s[..15])))]
            } else {
                vec![("append".into(), json!(format!("{s}x")))]
            }
        }
        Value::Null => vec![("null-to-0".into(), json!(0))],
        Value::Bool(b) => vec![("flip".into(), json!(!b))],
        _ => vec![],
    }
}

/// Every single alteration of `root` as (label, altered value).
fn enumerate(root: &Value) -> Vec<(String, Value)> {
    let mut out = Vec::new();
    // Walk paths; at each node, produce alterations of a full copy.
    fn walk(root: &Value, path: &mut Vec<Value>, out: &mut Vec<(String, Value)>) {
        let node = get(root, path).clone();
        let label = |what: &str| {
            let p: Vec<String> = path
                .iter()
                .map(|x| match x {
                    Value::String(s) => s.clone(),
                    other => other.to_string(),
                })
                .collect();
            format!("/{} {what}", p.join("/"))
        };
        match &node {
            Value::Object(m) => {
                for k in m.keys() {
                    let mut c = root.clone();
                    get_mut(&mut c, path)
                        .as_object_mut()
                        .expect("obj")
                        .remove(k);
                    out.push((label(&format!("remove key {k}")), c));
                }
                let mut c = root.clone();
                get_mut(&mut c, path)
                    .as_object_mut()
                    .expect("obj")
                    .insert("unknown_field".into(), json!(0));
                out.push((label("add unknown key"), c));
                for k in m.keys() {
                    path.push(json!(k));
                    walk(root, path, out);
                    path.pop();
                }
            }
            Value::Array(a) => {
                if !a.is_empty() {
                    let mut c = root.clone();
                    get_mut(&mut c, path).as_array_mut().expect("arr").pop();
                    out.push((label("drop last element"), c));
                    let mut c = root.clone();
                    let arr = get_mut(&mut c, path).as_array_mut().expect("arr");
                    let first = arr[0].clone();
                    arr.insert(0, first);
                    out.push((label("duplicate first element"), c));
                }
                if a.len() >= 2 && a[0] != a[1] {
                    let mut c = root.clone();
                    get_mut(&mut c, path)
                        .as_array_mut()
                        .expect("arr")
                        .swap(0, 1);
                    out.push((label("swap first two elements"), c));
                }
                for i in 0..a.len() {
                    path.push(json!(i));
                    walk(root, path, out);
                    path.pop();
                }
            }
            leaf => {
                for (what, nv) in leaf_alterations(leaf) {
                    let mut c = root.clone();
                    *get_mut(&mut c, path) = nv;
                    out.push((label(&what), c));
                }
            }
        }
    }
    walk(root, &mut Vec::new(), &mut out);
    for v in [1, 3] {
        let mut c = root.clone();
        c["version"] = json!(v);
        out.push((format!("/version set to {v}"), c));
    }
    out
}

fn get<'a>(v: &'a Value, path: &[Value]) -> &'a Value {
    path.iter().fold(v, |v, k| match k {
        Value::String(s) => &v[s.as_str()],
        Value::Number(n) => &v[n.as_u64().expect("idx") as usize],
        _ => unreachable!(),
    })
}

fn get_mut<'a>(v: &'a mut Value, path: &[Value]) -> &'a mut Value {
    path.iter().fold(v, |v, k| match k {
        Value::String(s) => &mut v[s.as_str()],
        Value::Number(n) => &mut v[n.as_u64().expect("idx") as usize],
        _ => unreachable!(),
    })
}

/// Resume `file` in a new process to total cap `cap`; (exit code, last row).
fn resume_proc(file: &Path, cap: u64, work: &Path) -> Result<(i32, Value), String> {
    let rows = work.join("rows.jsonl");
    let _ = fs::remove_file(&rows);
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let st = Command::new(exe)
        .args(["resume", "--from"])
        .arg(file)
        .args(["--cap", &cap.to_string(), "--guard", "0", "--out"])
        .arg(&rows)
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| e.to_string())?;
    let text = fs::read_to_string(&rows).unwrap_or_default();
    let row = text.lines().last().map_or(Value::Null, |l| {
        serde_json::from_str(l).unwrap_or(Value::Null)
    });
    Ok((st.code().unwrap_or(-1), row))
}

const SAME: [&str; 9] = [
    "status",
    "nodes",
    "backtracks",
    "propagation",
    "board",
    "hash",
    "hashed",
    "closed_share",
    "deepest",
];

pub fn tamper(args: &[String]) -> Result<usize, String> {
    let from = PathBuf::from(arg(args, "--from").ok_or("--from required")?);
    let extra = num(args, "--extra", 10_000)?;
    let work = PathBuf::from(arg(args, "--work").ok_or("--work required")?);
    fs::create_dir_all(&work).map_err(|e| e.to_string())?;
    let mut f = out_file(args)?;
    let text = fs::read_to_string(&from).map_err(|e| e.to_string())?;
    let ck = Checkpoint::parse(&text)?;
    let cap = ck.totals.nodes + extra;
    let inst = find_instance(&ck.instance)?;
    let src = from.display().to_string();
    let mut failures = 0;

    // Unaltered copy against an uninterrupted run to the same cap.
    let cfg = Config {
        meta: crate::meta(&inst),
        cap_total: cap,
        ..Config::default()
    };
    let unint = segment(&inst, cap, cfg, Duration::MAX, None);
    let copy = work.join("copy.ckpt.json");
    fs::write(&copy, &text).map_err(|e| e.to_string())?;
    let (code, row) = resume_proc(&copy, cap, &work)?;
    let same = code == 0 && SAME.iter().all(|k| row[k] == unint[k]);
    failures += usize::from(!same);
    crate::log_line(
        &mut f,
        &json!({"checkpoint": src, "kind": "unaltered", "exit": code, "status": row["status"],
                "identical": same, "nodes": row["nodes"], "hash": row["hash"],
                "uninterrupted_hash": unint["hash"], "cap": cap}),
    )?;

    let root: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let file = work.join("altered.ckpt.json");
    let mut tried = 0;
    let mut refused = 0;
    let mut forged_tried = 0;
    let mut forged_refused = 0;
    let try_one = |label: &str, kind: &str, body: &str, f: &mut fs::File| -> Result<bool, String> {
        fs::write(&file, body).map_err(|e| e.to_string())?;
        let (code, row) = resume_proc(&file, cap, &work)?;
        let ok = code != 0 && row["status"] == "error";
        crate::log_line(
            f,
            &json!({"checkpoint": src, "kind": kind, "alteration": label, "exit": code,
                    "status": row["status"], "error": row["error"], "refused": ok}),
        )?;
        Ok(ok)
    };
    for (label, v) in enumerate(&root) {
        let body = serde_json::to_string(&v).map_err(|e| e.to_string())?;
        tried += 1;
        if try_one(&label, "altered", &body, &mut f)? {
            refused += 1;
        }
        // Same alteration with the digest recomputed, when it still parses.
        if let Ok(mut c) = serde_json::from_value::<Checkpoint>(v.clone()) {
            c.seal();
            let body = serde_json::to_string(&c).map_err(|e| e.to_string())?;
            if body != text {
                forged_tried += 1;
                if try_one(&label, "forged-digest", &body, &mut f)? {
                    forged_refused += 1;
                }
            }
        }
    }
    let n = text.len();
    let mut offs = vec![0, 1, n / 4, n / 2, 3 * n / 4, n - 2, n - 1];
    offs.dedup();
    for o in offs {
        tried += 1;
        if try_one(
            &format!("truncate at byte {o} of {n}"),
            "altered",
            &text[..o],
            &mut f,
        )? {
            refused += 1;
        }
    }
    failures += tried - refused;
    crate::log_line(
        &mut f,
        &json!({"checkpoint": src, "kind": "summary", "tried": tried, "refused": refused,
                "accepted": tried - refused, "forged_tried": forged_tried,
                "forged_refused": forged_refused, "unaltered_identical": same}),
    )?;
    eprintln!(
        "{src}: {refused}/{tried} refused, unaltered identical {same}; forged digest {forged_refused}/{forged_tried} refused"
    );
    Ok(failures)
}
