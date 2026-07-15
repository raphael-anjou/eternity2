//: entry-population generator for the BANDSAW 10×10 testbed.
// Perfect-walk DFS over rows 0..R of a generated N×N instance, hints
// inside the prefix forced, ALL hint pieces bucket-excluded elsewhere
// (poisoning lesson), randomized candidate order per restart,
// banked on visit at depth R*N, deduped by the H7 key
// (row-(R-1) south vector, remaining-pool mask).
//
//   mini_gen --size 10 --colors 8 --seed 101 --rows 6 --secs 60 \
// --cap 200 --restart-ms 250 --out output/run/inst_101

#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use eternity2_bench_audit::mini::{hint_cells, Mini};
use eternity2_generator::SplitMix64;

fn main() {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let mut size = 10usize;
    let mut colors = 8u32;
    let mut seed = 101u64;
    let mut rows = 6usize;
    let mut secs = 60u64;
    let mut cap = 200usize;
    let mut restart_ms = 250u64;
    let mut per_restart = 10usize;
    let mut out = PathBuf::from("output/mini_gen_out");
    let mut i = 0;
    while i < raw.len() {
        match raw[i].as_str() {
            "--size" => { size = raw[i + 1].parse().unwrap(); i += 2; }
            "--colors" => { colors = raw[i + 1].parse().unwrap(); i += 2; }
            "--seed" => { seed = raw[i + 1].parse().unwrap(); i += 2; }
            "--rows" => { rows = raw[i + 1].parse().unwrap(); i += 2; }
            "--secs" => { secs = raw[i + 1].parse().unwrap(); i += 2; }
            "--cap" => { cap = raw[i + 1].parse().unwrap(); i += 2; }
            "--restart-ms" => { restart_ms = raw[i + 1].parse().unwrap(); i += 2; }
            "--per-restart" => { per_restart = raw[i + 1].parse().unwrap(); i += 2; }
            "--out" => { out = PathBuf::from(&raw[i + 1]); i += 2; }
            other => panic!("unknown arg {other}"),
        }
    }
    std::fs::create_dir_all(&out).expect("mkdir out");
    let hc = hint_cells(size);
    let m = Mini::from_seed(size, colors, seed, &hc);
    let n = m.n;
    let prefix_len = rows * n;

    // structural candidate lists per prefix cell over non-hint pieces
    let mut avail0 = vec![true; n * n];
    for &(_, p, _) in &m.hints {
        avail0[p as usize] = false;
    }
    let base_cands: Vec<Vec<(u16, u8)>> = (0..prefix_len)
        .map(|pos| {
            let (r, c) = (pos / n, pos % n);
            match m.hints.iter().find(|&&(hp, _, _)| hp == pos) {
                Some(&(_, p, rt)) => vec![(p, rt)],
                None => m.cands_at(r, c, &avail0),
            }
        })
        .collect();

    let t0 = Instant::now();
    let deadline = t0 + Duration::from_secs(secs);
    let mut rng_seed = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    let mut seen: HashSet<(Vec<u8>, u128)> = HashSet::new();
    let mut entries: Vec<Vec<(u16, u8)>> = Vec::new(); // placements, prefix only
    let mut restarts = 0u64;
    let mut visits = 0u64;
    let mut nodes_total = 0u64;

    while Instant::now() < deadline && entries.len() < cap {
        restarts += 1;
        let mut banked_this_restart = 0usize;
        rng_seed = rng_seed.wrapping_add(0xA076_1D64_78BD_642F);
        let mut rng = SplitMix64::new(rng_seed);
        let mut cands = base_cands.clone();
        for list in &mut cands {
            rng.shuffle(list);
        }
        let restart_deadline =
            (t0 + Duration::from_millis(restart_ms * restarts)).min(deadline);

        // iterative DFS, perfect walk
        let mut used = vec![false; n * n];
        let mut grid: Vec<Option<(u16, u8)>> = vec![None; n * n];
        let mut cursor = vec![0usize; prefix_len + 1];
        let mut depth = 0usize;
        let mut tick = 0u64;
        loop {
            tick += 1;
            if (tick & 0xFFF) == 0 && Instant::now() >= restart_deadline {
                break;
            }
            let mut advanced = false;
            while cursor[depth] < cands[depth].len() {
                let (p, rt) = cands[depth][cursor[depth]];
                cursor[depth] += 1;
                if used[p as usize] {
                    continue;
                }
                let o = m.rot[p as usize][rt as usize];
                let (r, c) = (depth / n, depth % n);
                if r > 0 {
                    let (p2, rt2) = grid[depth - n].unwrap();
                    if m.rot[p2 as usize][rt2 as usize][2] != o[0] {
                        continue;
                    }
                }
                if c > 0 {
                    let (p2, rt2) = grid[depth - 1].unwrap();
                    if m.rot[p2 as usize][rt2 as usize][1] != o[3] {
                        continue;
                    }
                }
                nodes_total += 1;
                used[p as usize] = true;
                grid[depth] = Some((p, rt));
                depth += 1;
                advanced = true;
                break;
            }
            if advanced {
                if depth == prefix_len {
                    visits += 1;
                    // H7 key: frontier souths + pool mask
                    let frontier: Vec<u8> = (0..n)
                        .map(|c| {
                            let (p, rt) = grid[(rows - 1) * n + c].unwrap();
                            m.rot[p as usize][rt as usize][2]
                        })
                        .collect();
                    let mut mask = 0u128;
                    for (pid, &u) in used.iter().enumerate() {
                        if !u {
                            mask |= 1u128 << pid;
                        }
                    }
                    if seen.insert((frontier, mask)) {
                        let entry: Vec<(u16, u8)> =
                            grid[..prefix_len].iter().map(|x| x.unwrap()).collect();
                        // rescore before banking
                        let breaks = m.breaks_of(&grid);
                        assert_eq!(breaks, 0, "banked entry must be perfect");
                        for &(hp, p, rt) in &m.hints {
                            if hp < prefix_len {
                                assert_eq!(grid[hp], Some((p, rt)), "hint violated");
                            } else {
                                assert!(!used[p as usize], "endgame hint piece consumed");
                            }
                        }
                        entries.push(entry);
                        banked_this_restart += 1;
                        if entries.len() >= cap || banked_this_restart >= per_restart {
                            break;
                        }
                    }
                    // force backtrack: we only want depth-R*N states
                    depth -= 1;
                    let (p, _) = grid[depth].unwrap();
                    used[p as usize] = false;
                    grid[depth] = None;
                } else {
                    cursor[depth] = 0;
                }
            } else {
                if depth == 0 {
                    break; // subtree exhausted: restart
                }
                cursor[depth] = 0;
                depth -= 1;
                let (p, _) = grid[depth].unwrap();
                used[p as usize] = false;
                grid[depth] = None;
            }
        }
    }

    let elapsed = t0.elapsed().as_secs_f64();
    let mut f = std::fs::File::create(out.join("entries.jsonl")).expect("entries file");
    for (idx, entry) in entries.iter().enumerate() {
        let cells: Vec<String> = entry
            .iter()
            .enumerate()
            .map(|(pos, &(p, rt))| {
                format!("{{\"pos\":{pos},\"piece_id\":{p},\"rotation\":{rt}}}")
            })
            .collect();
        writeln!(
            f,
            "{{\"idx\":{idx},\"placement\":[{}]}}",
            cells.join(",")
        )
        .unwrap();
    }
    let meta = format!(
        "{{\"size\":{size},\"colors\":{colors},\"seed\":{seed},\"rows\":{rows},\"secs\":{secs},\"cap\":{cap},\"restart_ms\":{restart_ms},\"entries\":{},\"restarts\":{restarts},\"visits\":{visits},\"nodes\":{nodes_total},\"elapsed_s\":{elapsed:.1}}}",
        entries.len()
    );
    std::fs::write(out.join("meta.json"), &meta).expect("meta");
    println!("{meta}");
}
