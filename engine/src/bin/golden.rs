// Emit golden reference outputs for the Lua port to validate against.
use eternity2_engine::{generate, official_puzzle, build_path, PATH_KINDS, score_board, Solver, Status};

fn main() {
    // 1. Generated puzzle pieces (parity of RNG + construction).
    for &(s, c, seed) in &[(4u8,4u8,7u32),(5,6,42),(3,3,1),(6,5,99)] {
        let p = generate(s, c, seed);
        print!("GEN {s} {c} {seed}");
        for e in &p.pieces { print!(" {},{},{},{}", e[0],e[1],e[2],e[3]); }
        println!();
    }
    // 2. Official puzzle summary.
    let off = official_puzzle();
    println!("OFF pieces={} colors={} hints={}", off.pieces.len(), off.num_colors, off.hints.len());
    for h in &off.hints { print!("OFFHINT {} {} {}\n", h.pos, h.piece, h.rot); }
    // 3. Paths (full permutation for a few sizes).
    for &kind in PATH_KINDS {
        for &(w,h) in &[(3u8,3u8),(4,4),(5,5)] {
            let path = build_path(kind, w, h, 1).unwrap();
            print!("PATH {kind} {w} {h}");
            for c in &path { print!(" {c}"); }
            println!();
        }
    }
    // 4. Solver: solve generated puzzles on every path; report final stats + score.
    for &kind in PATH_KINDS {
        let p = generate(4, 4, 11);
        let path = build_path(kind, 4, 4, 0).unwrap();
        let mut sv = Solver::new(&p, &path, true, false, 0).unwrap();
        loop {
            let r = sv.step(5_000_000);
            if r.status != Status::Running { break; }
        }
        let r = sv.report();
        let sc = score_board(&p, sv.board());
        let st = match r.status { Status::Solved=>"solved", Status::Exhausted=>"exhausted", Status::Running=>"running" };
        println!("SOLVE {kind} status={st} placed={} score={} nodes={} attempts={} backtracks={}",
            r.placed, sc, r.nodes as u64, r.attempts as u64, r.backtracks as u64);
    }
    // 5. Official partial run (deterministic node/attempt counts on a fixed budget).
    {
        let p = official_puzzle();
        let path = build_path("row-major", 16, 16, 0).unwrap();
        let mut sv = Solver::new(&p, &path, true, false, 0).unwrap();
        let r = sv.step(50_000);
        let st = match r.status { Status::Solved=>"solved", Status::Exhausted=>"exhausted", Status::Running=>"running" };
        println!("OFFICIALRUN budget=50000 status={st} placed={} best={} nodes={} attempts={} backtracks={}",
            r.placed, r.best_placed, r.nodes as u64, r.attempts as u64, r.backtracks as u64);
    }
}
