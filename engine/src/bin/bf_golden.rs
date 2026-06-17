// Reference values + piece bytes for the Brainfuck solver. For each puzzle,
// print the pieces and the row-major solver stats. Also dump pieces as a flat
// space-separated list (the BF program reads 4 bytes per piece from stdin).
use eternity2_engine::{generate, build_path, score_board, Solver, Status};
fn main() {
    for &(s, c, seed) in &[(3u8,3u8,1u32),(3,3,2),(3,3,5),(3,3,7),(4,4,11),(4,4,3)] {
        let p = generate(s, c, seed);
        let path = build_path("row-major", s, s, 0).unwrap();
        let mut sv = Solver::new(&p, &path, true, false, 0).unwrap();
        loop { let r = sv.step(50_000_000); if r.status != Status::Running { break; } }
        let r = sv.report();
        let st = match r.status { Status::Solved=>"SOLVED", Status::Exhausted=>"EXHAUSTED", _=>"RUNNING" };
        // pieces flat
        let mut bytes = String::new();
        for e in &p.pieces { for &x in e { bytes.push_str(&format!("{} ", x)); } }
        println!("PUZZLE {s} {c} {seed} colors_max={}", p.num_colors);
        println!("  PIECES {}", bytes.trim());
        // solved board: cell -> row (piece*4+rot)
        let mut bd = String::new();
        for &cell in sv.board() { bd.push_str(&format!("{} ", cell)); }
        println!("  BOARD {}", bd.trim());
        println!("  RESULT status={st} placed={} score={} nodes={} attempts={} backtracks={}",
            r.placed, score_board(&p, sv.board()), r.nodes as u64, r.attempts as u64, r.backtracks as u64);
    }
}
