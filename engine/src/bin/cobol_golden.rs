// Reference values for the COBOL port: generate small puzzles, solve row-major,
// print pieces + solver stats + score. COBOL validates against these.
use eternity2_engine::{generate, build_path, score_board, Solver, Status};
fn main() {
    for &(s, c, seed) in &[(4u8,4u8,11u32),(5,5,3),(5,6,42),(3,3,1)] {
        let p = generate(s, c, seed);
        // pieces, space-separated u,r,d,l
        print!("PIECES {s} {c} {seed}:");
        for e in &p.pieces { print!(" {} {} {} {}", e[0],e[1],e[2],e[3]); }
        println!();
        let path = build_path("row-major", s, s, 0).unwrap();
        let mut sv = Solver::new(&p, &path, true, false, 0).unwrap();
        loop { let r = sv.step(20_000_000); if r.status != Status::Running { break; } }
        let r = sv.report();
        let st = match r.status { Status::Solved=>"SOLVED", Status::Exhausted=>"EXHAUSTED", _=>"RUNNING" };
        println!("RESULT {s} {c} {seed}: status={st} placed={} score={} nodes={} attempts={} backtracks={}",
            r.placed, score_board(&p, sv.board()), r.nodes as u64, r.attempts as u64, r.backtracks as u64);
    }
}
