//! CLI: `ncs instances N C S0 S1`, `ncs annealed N C SEED`,
//! `ncs stage3 N C SEED` (one JSON object per line on stdout).

use near_codeword_spectrum::{anneal, b_ann, break_dfs, greedy_null, overlap_bin, annealed, Board, Instance, Tally};
use serde_json::json;

// PREREG_AMENDMENTS.md A3: 16 runs per sampler (prereg: 32).
const RUNS: u32 = 16;
const DFS_STEPS: u64 = 20_000_000;
const SA_ATTEMPTS: u64 = 20_000_000;

fn arg<T: std::str::FromStr>(v: &[String], i: usize) -> T {
    v.get(i).and_then(|s| s.parse().ok()).unwrap_or_else(|| {
        eprintln!("usage: ncs instances N C S0 S1 | annealed N C SEED | stage3 N C SEED");
        std::process::exit(2)
    })
}

fn tally_json(inst: &Instance, sampler: &str, run: u32, solved: Option<bool>, t: &Tally) -> serde_json::Value {
    json!({
        "n": inst.n, "colors": inst.colors, "seed": inst.seed, "sampler": sampler, "run": run,
        "solved": solved,
        "best_breaks": if t.best.0 == u32::MAX { None } else { Some(t.best.0) },
        "best_a": if t.best.0 == u32::MAX { None } else { Some(t.best.1) },
        "best_overlap_bin": if t.best.0 == u32::MAX { None } else { Some(overlap_bin(t.best.1, t.n2)) },
        "low_hist": t.low_hist(),
        "low_by_a": t.low_by_a(),
        "min_b_by_a": t.min_b_by_a,
    })
}

fn main() {
    let v: Vec<String> = std::env::args().collect();
    let cmd = v.get(1).map(String::as_str).unwrap_or("");
    let n: usize = arg(&v, 2);
    let c: u8 = arg(&v, 3);
    match cmd {
        "instances" => {
            let (s0, s1): (u32, u32) = (arg(&v, 4), arg(&v, 5));
            for seed in s0..=s1 {
                let inst = Instance::generate(n, c, seed);
                let (pf, pi) = inst.collision_probs();
                let (q, mu) = inst.mu();
                println!(
                    "{}",
                    json!({"n": n, "colors": c, "seed": seed, "pieces": inst.pieces,
                           "rotation_duplicates": inst.rotation_duplicates(),
                           "p_f": pf, "p_i": pi, "q_int": q, "mu": mu})
                );
            }
        }
        "annealed" => {
            let inst = Instance::generate(n, c, arg(&v, 4));
            let (pf, pi) = inst.collision_probs();
            let e = annealed(n, pf, pi);
            let log10: Vec<Vec<Option<f64>>> = e
                .iter()
                .map(|r| r.iter().map(|&x| if x > 0.0 { Some(x.log10()) } else { None }).collect())
                .collect();
            println!(
                "{}",
                json!({"n": n, "colors": c, "seed": inst.seed, "p_f": pf, "p_i": pi,
                       "b_ann": b_ann(&e), "log10_E": log10})
            );
        }
        "stage3" => {
            let seed: u32 = arg(&v, 4);
            let inst = Instance::generate(n, c, seed);
            let base = 1_000_003u32.wrapping_mul(seed).wrapping_add(1009 * n as u32);
            for r in 0..RUNS {
                let mut t = Tally::new(n);
                let board = break_dfs(&inst, base.wrapping_add(r), DFS_STEPS);
                if let Some(cells) = &board {
                    let b = Board::new(&inst, cells.clone());
                    t.see(b.breaks, b.distance());
                }
                println!("{}", tally_json(&inst, "break_dfs", r, Some(board.is_some()), &t));
            }
            for r in 0..RUNS {
                let t = anneal(&inst, base.wrapping_add(r + 100), SA_ATTEMPTS, 2.0, 0.2);
                println!("{}", tally_json(&inst, "anneal", r, None, &t));
            }
            for r in 0..RUNS {
                let t = greedy_null(&inst, base.wrapping_add(r + 200));
                println!("{}", tally_json(&inst, "greedy_null", r, None, &t));
            }
        }
        "bench" => {
            let inst = Instance::generate(n, c, arg(&v, 4));
            let steps: u64 = arg(&v, 5);
            let t = std::time::Instant::now();
            let solved = break_dfs(&inst, 1, steps).is_some();
            eprintln!("dfs {steps} steps: {:.2} s solved {solved}", t.elapsed().as_secs_f64());
            let t = std::time::Instant::now();
            let ta = anneal(&inst, 1, steps, 2.0, 0.2);
            eprintln!("sa {steps} attempts: {:.2} s best {:?}", t.elapsed().as_secs_f64(), ta.best);
            let t = std::time::Instant::now();
            let tg = greedy_null(&inst, 1);
            eprintln!("greedy: {:.2} s best {:?}", t.elapsed().as_secs_f64(), tg.best);
        }
        _ => {
            eprintln!("unknown command");
            std::process::exit(2)
        }
    }
}
