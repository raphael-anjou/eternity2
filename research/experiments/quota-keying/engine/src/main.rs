//! qk: quota-keying measurements (see ../PREREG.md). One subcommand:
//!   bw   one run of the Blackwood-style port, one JSON line on stdout
//!
//! Options of `bw` (all off by default; off reproduces the Track 10 port):
//!   --csv FILE --unlock LIST --seed S --cap NODES   as in Track 10
//!   --priv a,b,c        privileged colours (default 22,21,20, the port's)
//!   --legalcap N        stop after N legal nodes (snapshot at N/2)
//!   --legalbreak        refuse wrong-class / wrongly rotated pieces in the break path
//!   --ringbreakoff      also refuse breaks on a ring-coloured side (with --legalbreak: Blackwood's rule)
//!   --noisolation       drop the non-adjacency rule for breaks (part 2)
//!   --stage 0|1|2       A0 / A1 / A2 staging of the quota
//!   --noquota --maxbreaks K --label L

mod bw;
mod puzzle;

use puzzle::load_csv;
use std::collections::HashMap;
use std::time::Instant;

fn args_map() -> (String, HashMap<String, String>) {
    let a: Vec<String> = std::env::args().collect();
    let cmd = a.get(1).cloned().unwrap_or_default();
    let mut m = HashMap::new();
    let mut i = 2;
    while i < a.len() {
        let k = a[i].trim_start_matches("--").to_string();
        if i + 1 < a.len() && !a[i + 1].starts_with("--") {
            m.insert(k, a[i + 1].clone());
            i += 2;
        } else {
            m.insert(k, "1".into());
            i += 1;
        }
    }
    (cmd, m)
}

fn get<T: std::str::FromStr>(m: &HashMap<String, String>, k: &str, d: T) -> T {
    m.get(k).and_then(|v| v.parse().ok()).unwrap_or(d)
}

fn num(m: &HashMap<String, String>, k: &str, d: f64) -> f64 {
    m.get(k)
        .map(|v| v.parse::<f64>().expect("number"))
        .unwrap_or(d)
}

fn fmt_vec<T: std::fmt::Display>(v: &[T]) -> String {
    let s: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    format!("[{}]", s.join(","))
}

fn main() {
    let (cmd, m) = args_map();
    if cmd != "bw" {
        eprintln!("usage: qk bw --csv FILE --unlock LIST [--seed S] [--cap N] [--priv a,b,c] [--legalcap N] [--legalbreak] [--stage 0|1|2]");
        std::process::exit(2);
    }
    let p = load_csv(m.get("csv").expect("--csv"));
    let unlock: Vec<usize> = m
        .get("unlock")
        .expect("--unlock")
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let seed: u64 = get(&m, "seed", 1u64);
    let cap = num(&m, "cap", 1e10) as u64;
    let quota = !m.contains_key("noquota");
    let label = m.get("label").cloned().unwrap_or_default();
    let privc: Vec<u8> = m
        .get("priv")
        .map(|s| s.split(',').map(|t| t.parse().unwrap()).collect())
        .unwrap_or_else(|| vec![22, 21, 20]);
    let maxb: u8 = get(&m, "maxbreaks", 10u8);
    let legal_cap = num(&m, "legalcap", u64::MAX as f64) as u64;
    let legal_break = m.contains_key("legalbreak");
    let stage: u8 = get(&m, "stage", 0u8);
    let cps: Vec<u64> = [1e7, 1e8, 1e9, 1e10]
        .iter()
        .map(|&x| x as u64)
        .filter(|&x| x < cap)
        .collect();
    let mut s = bw::Solver::new(&p, &privc, quota, unlock.clone(), maxb, cap, cps);
    if legal_cap != u64::MAX {
        s.set_legal_cap(legal_cap, vec![legal_cap / 2]);
    }
    s.legal_break = legal_break;
    s.ring_break_off = m.contains_key("ringbreakoff");
    s.no_isolation = m.contains_key("noisolation");
    s.set_stage(stage);
    let t0 = Instant::now();
    let handle = std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(move || {
            s.run(seed);
            s
        })
        .unwrap();
    let s = handle.join().unwrap();
    let secs = t0.elapsed().as_secs_f64();
    let snaps: Vec<String> = s
        .snapshots
        .iter()
        .map(|(n, h)| format!("[{},{}]", n, fmt_vec(h)))
        .collect();
    let lsnaps: Vec<String> = s
        .legal_snapshots
        .iter()
        .map(|(nl, n, h, hl)| format!("[{},{},{},{}]", nl, n, fmt_vec(h), fmt_vec(hl)))
        .collect();
    println!(
        "{{\"label\":\"{label}\",\"seed\":{seed},\"priv\":{},\"unlock\":{},\"quota\":{quota},\"cap\":{cap},\"legalcap\":{},\"legalbreak\":{legal_break},\"ringbreakoff\":{},\"noisolation\":{},\"stage\":{stage},\"nodes\":{},\"nodes_legal\":{},\"seconds\":{secs:.1},\"deepest\":{},\"breaks_at_deepest\":{},\"completions\":{},\"best_complete_score\":{},\"c3_hist\":{},\"hist\":{},\"hist_legal\":{},\"checkpoints\":[{}],\"legal_checkpoints\":[{}]}}",
        fmt_vec(&privc),
        fmt_vec(&unlock),
        if legal_cap == u64::MAX { "null".to_string() } else { legal_cap.to_string() },
        s.ring_break_off,
        s.no_isolation,
        s.nodes,
        s.nodes_legal,
        s.deepest,
        s.breaks_at_deepest,
        s.completions,
        s.best_score,
        fmt_vec(&s.c3_hist),
        fmt_vec(&s.hist),
        fmt_vec(&s.hist_legal),
        snaps.join(","),
        lsnaps.join(",")
    );
}
