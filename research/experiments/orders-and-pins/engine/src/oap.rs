//! orders-and-pins additions (see ../../PREREG.md). The tree, the SMC
//! estimator and the enumerator are Track 10's (`tree.rs`), unchanged except
//! for two additions there: a step with no rim side and no known neighbour
//! scans every oriented piece, and the enumerator also reports the node count
//! at the first complete board.
//!
//! Subcommands (dispatched from main.rs):
//!   paint-boards  census-painted 16x16 boards, piece ids shuffled and rotated
//!   tsmc          SMC repetitions for (board or official csv, fixed cells, order file)
//!   tenum         exact enumeration of the same configuration, with a cap
//!   gate6         exact against SMC on the 30 hard-ladder 6x6 instances, any order file

use crate::puzzle::{load_csv, rotate, Edges, Puzzle, Rng};
use crate::tree::{smc, State, Tree};
use std::collections::HashMap;
use std::fmt::Write as _;
use std::time::Instant;

pub struct Board {
    pub puzzle: Puzzle,
    /// planted[pos] = (pid, rot): the solution the board was painted from.
    pub planted: Vec<(u16, u8)>,
}

/// Board file: line 1 `n`; n*n lines `t r b l` (pieces by id); n*n lines
/// `pid rot` (planted placement by cell, pos = row * n + col).
pub fn load_board(path: &str) -> Board {
    let raw = std::fs::read_to_string(path).expect("read board");
    let mut it = raw.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#'));
    let n: usize = it.next().unwrap().trim().parse().unwrap();
    let mut pieces = Vec::with_capacity(n * n);
    for _ in 0..n * n {
        let v: Vec<u8> = it
            .next()
            .unwrap()
            .split_whitespace()
            .map(|t| t.parse().unwrap())
            .collect();
        pieces.push([v[0], v[1], v[2], v[3]]);
    }
    let mut planted = Vec::with_capacity(n * n);
    for _ in 0..n * n {
        let v: Vec<usize> = it
            .next()
            .unwrap()
            .split_whitespace()
            .map(|t| t.parse().unwrap())
            .collect();
        planted.push((v[0] as u16, v[1] as u8));
    }
    let b = Board {
        puzzle: Puzzle {
            n,
            pieces,
            hints: vec![],
        },
        planted,
    };
    verify_planted(&b);
    b
}

/// The planted placement is a valid complete board: every joint matches, grey
/// exactly on the rim, every piece used once.
pub fn verify_planted(b: &Board) {
    let n = b.puzzle.n;
    let e: Vec<Edges> = b
        .planted
        .iter()
        .map(|&(pid, r)| rotate(b.puzzle.pieces[pid as usize], r))
        .collect();
    let mut used = vec![false; n * n];
    for (pos, &(pid, _)) in b.planted.iter().enumerate() {
        assert!(!used[pid as usize], "piece used twice");
        used[pid as usize] = true;
        let (r, c) = (pos / n, pos % n);
        assert_eq!(e[pos][0] == 0, r == 0);
        assert_eq!(e[pos][2] == 0, r == n - 1);
        assert_eq!(e[pos][3] == 0, c == 0);
        assert_eq!(e[pos][1] == 0, c == n - 1);
        if c + 1 < n {
            assert_eq!(e[pos][1], e[pos + 1][3]);
        }
        if r + 1 < n {
            assert_eq!(e[pos][2], e[pos + n][0]);
        }
    }
}

fn write_board(path: &str, pieces: &[Edges], planted: &[(u16, u8)], n: usize, note: &str) {
    let mut s = String::new();
    let _ = writeln!(s, "# {note}");
    let _ = writeln!(s, "{n}");
    for p in pieces {
        let _ = writeln!(s, "{} {} {} {}", p[0], p[1], p[2], p[3]);
    }
    for &(pid, r) in planted {
        let _ = writeln!(s, "{pid} {r}");
    }
    std::fs::write(path, s).expect("write board");
}

/// Paint `count` boards from the official census with Track 10's painter
/// (`crate::paint`, judged), trying painter seeds seed0, seed0+1, ...; each
/// accepted board's piece ids are shuffled and each piece is rotated by a
/// random quarter turn (the tree does not depend on ids or stored rotation;
/// the shuffle only keeps the enumerator's value order from following the
/// painted cell order). Writes <out>/board-<seed>.txt.
pub fn paint_boards(m: &HashMap<String, String>) {
    let official = load_csv(m.get("csv").expect("--csv"));
    let seed0: u64 = m.get("seed0").map(|v| v.parse().unwrap()).unwrap_or(1);
    let count: usize = m.get("count").map(|v| v.parse().unwrap()).unwrap_or(1);
    let out = m.get("out").expect("--out");
    std::fs::create_dir_all(out).unwrap();
    let mut seed = seed0;
    let mut done = 0;
    let mut rejected = 0;
    while done < count {
        if let Some(p) = crate::paint(&official, seed) {
            let n = p.n;
            let mut rng = Rng::new(seed ^ 0x0A9_5EED);
            let mut perm: Vec<usize> = (0..n * n).collect();
            rng.shuffle(&mut perm);
            let mut pieces = vec![[0u8; 4]; n * n];
            let mut planted = vec![(0u16, 0u8); n * n];
            for (pos, &pid) in perm.iter().enumerate() {
                let k = rng.below(4) as u8;
                pieces[pid] = rotate(p.pieces[pos], k);
                planted[pos] = (pid as u16, (4 - k) % 4);
            }
            let b = Board {
                puzzle: Puzzle {
                    n,
                    pieces: pieces.clone(),
                    hints: vec![],
                },
                planted: planted.clone(),
            };
            verify_planted(&b);
            let path = format!("{out}/board-{seed}.txt");
            write_board(
                &path,
                &pieces,
                &planted,
                n,
                &format!("census painter seed {seed} (Track 10 paint), ids shuffled"),
            );
            println!("{{\"seed\":{seed},\"path\":\"{path}\"}}");
            done += 1;
        } else {
            rejected += 1;
        }
        seed += 1;
    }
    eprintln!("# painted {done}, rejected {rejected}");
}

fn parse_list(s: &str) -> Vec<usize> {
    s.split(|c: char| c == ',' || c.is_whitespace())
        .filter(|t| !t.is_empty())
        .map(|t| t.parse().unwrap())
        .collect()
}

pub fn load_order(path: &str, cells: usize) -> Vec<usize> {
    let raw = std::fs::read_to_string(path).expect("read order file");
    let body: String = raw
        .lines()
        .filter(|l| !l.starts_with('#'))
        .collect::<Vec<_>>()
        .join(" ");
    let v = parse_list(&body);
    let mut seen = vec![false; cells];
    for &p in &v {
        assert!(p < cells && !seen[p], "order file is not a permutation");
        seen[p] = true;
    }
    assert_eq!(v.len(), cells, "order file must list every cell once");
    v
}

/// (puzzle, fixed cells, free order) from --board or --csv, --fixed (cell
/// list; the pieces are the planted ones, or the official clues for --csv),
/// --order (order file).
pub fn config(m: &HashMap<String, String>) -> (Puzzle, Vec<(usize, u16, u8)>, Vec<usize>) {
    let fixed_pos: Vec<usize> = m.get("fixed").map(|s| parse_list(s)).unwrap_or_default();
    let (p, fixed) = if let Some(b) = m.get("board") {
        let b = load_board(b);
        let fixed: Vec<(usize, u16, u8)> = fixed_pos
            .iter()
            .map(|&pos| (pos, b.planted[pos].0, b.planted[pos].1))
            .collect();
        (b.puzzle, fixed)
    } else {
        let p = load_csv(m.get("csv").expect("--board or --csv"));
        let fixed: Vec<(usize, u16, u8)> = fixed_pos
            .iter()
            .map(|&pos| {
                *p.hints
                    .iter()
                    .find(|h| h.0 == pos)
                    .expect("--csv: a fixed cell must be an official clue")
            })
            .collect();
        (p, fixed)
    };
    let order: Vec<usize> = load_order(m.get("order").expect("--order"), p.n * p.n)
        .into_iter()
        .filter(|q| !fixed.iter().any(|f| f.0 == *q))
        .collect();
    (p, fixed, order)
}

pub fn tsmc(m: &HashMap<String, String>) {
    let (p, fixed, order) = config(m);
    let np = crate::num(m, "np", 1e5) as usize;
    let reps: usize = crate::get(m, "reps", 1usize);
    let seed0: u64 = crate::get(m, "seed0", 1u64);
    let label = m.get("label").cloned().unwrap_or_default();
    let t = Tree::new(&p, &order, &fixed);
    for r in 0..reps {
        let t0 = Instant::now();
        let run = smc(&t, np, seed0 + r as u64);
        println!(
            "{}",
            crate::smc_json(
                &label,
                r,
                seed0 + r as u64,
                np,
                &run,
                t0.elapsed().as_secs_f64()
            )
        );
    }
}

pub fn tenum(m: &HashMap<String, String>) {
    let (p, fixed, order) = config(m);
    let cap = crate::num(m, "cap", 1e10) as u64;
    let label = m.get("label").cloned().unwrap_or_default();
    let t = Tree::new(&p, &order, &fixed);
    let t0 = Instant::now();
    let (w, complete, first) = t.enumerate_first(cap);
    let total: u64 = w[1..].iter().sum();
    println!(
        "{{\"label\":\"{label}\",\"free\":{},\"complete\":{complete},\"total\":{total},\"solutions\":{},\"nodes_at_first_solution\":{},\"seconds\":{:.1},\"widths\":{}}}",
        t.nfree,
        w[t.nfree],
        first.map(|x| x.to_string()).unwrap_or("null".into()),
        t0.elapsed().as_secs_f64(),
        crate::fmt_vec(&w)
    );
}

/// The 30 hard-ladder 6x6 instances (puzzle object; the planted board is used
/// only to read the pieces of the fixed cells), any 36-cell order file, an
/// optional list of fixed cells. One JSON line per instance: exact widths by
/// enumeration (cap) and the SMC per-depth mean and sd.
pub fn gate6(m: &HashMap<String, String>) {
    let path = m.get("jsonl").expect("--jsonl");
    let np = crate::num(m, "np", 200.0) as usize;
    let reps: usize = crate::get(m, "reps", 200usize);
    let cap = crate::num(m, "cap", 2e10) as u64;
    let label = m.get("label").cloned().unwrap_or_default();
    let fixed_pos: Vec<usize> = m.get("fixed").map(|s| parse_list(s)).unwrap_or_default();
    let full = load_order(m.get("order").expect("--order"), 36);
    for (li, line) in std::fs::read_to_string(path).unwrap().lines().enumerate() {
        let (id, n, pieces) = crate::parse_pieces(line);
        if n != 6 {
            continue;
        }
        let planted_raw = line
            .split("\"planted\":[")
            .nth(1)
            .unwrap()
            .split(']')
            .next()
            .unwrap();
        let planted: Vec<(u16, u8)> = parse_list(planted_raw)
            .into_iter()
            .map(|v| ((v / 4) as u16, (v % 4) as u8))
            .collect();
        let b = Board {
            puzzle: Puzzle {
                n,
                pieces,
                hints: vec![],
            },
            planted,
        };
        verify_planted(&b);
        let fixed: Vec<(usize, u16, u8)> = fixed_pos
            .iter()
            .map(|&pos| (pos, b.planted[pos].0, b.planted[pos].1))
            .collect();
        let order: Vec<usize> = full
            .iter()
            .copied()
            .filter(|q| !fixed.iter().any(|f| f.0 == *q))
            .collect();
        let t = Tree::new(&b.puzzle, &order, &fixed);
        println!(
            "{}",
            crate::check_config(
                &format!("{label}:{id}"),
                &t,
                np,
                reps,
                cap,
                1000 * li as u64 + 1
            )
        );
    }
}

/// Track 10's population step (uniform N-subset without replacement of all
/// children, Floyd's algorithm), started from `init` states at free depth k0
/// instead of the root. Returns est[d] for d in 0..=nfree (zero before k0):
/// an unbiased estimate of the number of descendants at depth d of the init
/// states (est[k0] = number of init states).
pub fn smc_from(t: &Tree, init: Vec<State>, k0: usize, np: usize, rng: &mut Rng) -> Vec<f64> {
    let mut est = vec![0.0f64; t.nfree + 1];
    est[k0] = init.len() as f64;
    let mut pop = if init.len() > np {
        let mut v = init;
        rng.shuffle(&mut v);
        est[k0] = v.len() as f64;
        v.truncate(np);
        v
    } else {
        init
    };
    // weight per particle relative to the population size
    let mut counts: Vec<u32> = Vec::new();
    for k in k0..t.nfree {
        if pop.is_empty() {
            break;
        }
        counts.clear();
        let mut e: u64 = 0;
        for st in &pop {
            let mut c = 0u32;
            t.for_each_child(st, k, |_| c += 1);
            counts.push(c);
            e += c as u64;
        }
        est[k + 1] = est[k] * e as f64 / pop.len() as f64;
        if e == 0 {
            break;
        }
        let chosen: Vec<u64> = if e as usize <= np {
            (0..e).collect()
        } else {
            let mut set = std::collections::HashSet::with_capacity(np * 2);
            let mut out = Vec::with_capacity(np);
            for j in (e - np as u64)..e {
                let r = rng.next_u64() % (j + 1);
                let v = if set.contains(&r) { j } else { r };
                set.insert(v);
                out.push(v);
            }
            out.sort_unstable();
            out
        };
        let mut next: Vec<State> = Vec::with_capacity(chosen.len());
        let mut ci = 0usize;
        let mut base = 0u64;
        for (j, st) in pop.iter().enumerate() {
            let c = counts[j] as u64;
            if ci < chosen.len() && chosen[ci] < base + c {
                let mut kids: Vec<u32> = Vec::with_capacity(c as usize);
                t.for_each_child(st, k, |i| kids.push(i));
                for (off, &i) in kids.iter().enumerate() {
                    while ci < chosen.len() && chosen[ci] == base + off as u64 {
                        let mut ns = st.clone();
                        t.apply(&mut ns, k, i);
                        next.push(ns);
                        ci += 1;
                    }
                }
            }
            base += c;
        }
        pop = next;
    }
    est
}

/// Planted-path decomposition (PPD). Every node of the tree either lies on
/// the planted path or has a first step j at which it leaves it. So
///   W_d = 1 + sum_{j < d} D_j(d),
/// where D_j(d) = number of depth-d descendants of the children of the
/// planted state at depth j other than the planted child. Each D_j is
/// estimated by an independent `smc_from` started from those siblings, so the
/// estimate is unbiased whenever SMC is, and the planted path, which plain
/// SMC loses at a bottleneck, is counted exactly. Needs a board with a
/// planted solution.
pub fn ppd(t: &Tree, planted: &[(u16, u8)], pieces: &[Edges], np: usize, seed: u64) -> (Vec<f64>, Vec<f64>) {
    let mut w = vec![0.0f64; t.nfree + 1];
    let mut by_j = vec![0.0f64; t.nfree];
    let mut st = t.root();
    w[0] = 1.0;
    for j in 0..t.nfree {
        let pos = t.step_pos(j);
        let (ppid, prot) = planted[pos];
        let pe = rotate(pieces[ppid as usize], prot);
        let mut kids: Vec<u32> = Vec::new();
        t.for_each_child(&st, j, |i| kids.push(i));
        let mut planted_i = None;
        let mut alts: Vec<State> = Vec::new();
        for &i in &kids {
            let (pid, e) = t.oriented_piece(i);
            if pid == ppid && e == pe && planted_i.is_none() {
                planted_i = Some(i);
            } else {
                let mut ns = st.clone();
                t.apply(&mut ns, j, i);
                alts.push(ns);
            }
        }
        let pi = planted_i.expect("planted child not among the children: board or fixed cells inconsistent");
        if !alts.is_empty() {
            let mut rng = Rng::new(seed.wrapping_mul(1_000_003).wrapping_add(j as u64));
            let est = smc_from(t, alts, j + 1, np, &mut rng);
            for d in j + 1..=t.nfree {
                w[d] += est[d];
                by_j[j] += est[d];
            }
        }
        t.apply(&mut st, j, pi);
        w[j + 1] += 1.0;
    }
    (w, by_j)
}

pub fn tppd(m: &HashMap<String, String>) {
    let bpath = m.get("board").expect("--board (PPD needs a planted solution)");
    let b = load_board(bpath);
    let (p, fixed, order) = config(m);
    let np = crate::num(m, "np", 1e4) as usize;
    let reps: usize = crate::get(m, "reps", 1usize);
    let seed0: u64 = crate::get(m, "seed0", 1u64);
    let label = m.get("label").cloned().unwrap_or_default();
    let t = Tree::new(&p, &order, &fixed);
    for r in 0..reps {
        let t0 = Instant::now();
        let (w, by_j) = ppd(&t, &b.planted, &p.pieces, np, seed0 + r as u64);
        let total: f64 = w[1..].iter().sum();
        let lw: Vec<String> = w
            .iter()
            .map(|&x| if x > 0.0 { format!("{:.6}", x.log10()) } else { "null".into() })
            .collect();
        println!(
            "{{\"label\":\"{label}\",\"estimator\":\"ppd\",\"rep\":{r},\"seed\":{},\"np\":{np},\"total\":{total:.6e},\"seconds\":{:.2},\"log10_width\":[{}],\"by_first_departure\":{}}}",
            seed0 + r as u64,
            t0.elapsed().as_secs_f64(),
            lw.join(","),
            crate::fmt_f(&by_j)
        );
    }
}

/// Exact enumeration of the tree truncated at free depth --maxd (gate GT).
pub fn tenumd(m: &HashMap<String, String>) {
    let (p, fixed, order) = config(m);
    let cap = crate::num(m, "cap", 2e9) as u64;
    let maxd: usize = crate::get(m, "maxd", 20usize);
    let label = m.get("label").cloned().unwrap_or_default();
    let t = Tree::new(&p, &order, &fixed);
    let t0 = Instant::now();
    let (w, complete) = t.enumerate_depth(cap, maxd);
    println!(
        "{{\"label\":\"{label}\",\"maxd\":{maxd},\"complete\":{complete},\"seconds\":{:.1},\"widths\":{}}}",
        t0.elapsed().as_secs_f64(),
        crate::fmt_vec(&w)
    );
}
