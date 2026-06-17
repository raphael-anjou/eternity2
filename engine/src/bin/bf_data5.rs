use eternity2_engine::{generate, build_path, Solver, Status};
use std::io::Write;
fn main() {
    let dir = "../engine-side-quests/engine-brainfuck/data";
    for &(s, c, seed) in &[(5u8,5u8,3u32),(5,6,42)] {
        let p = generate(s, c, seed);
        let mut bytes = vec![s];
        for e in &p.pieces { for &x in e { bytes.push(x); } }
        let stem = format!("{dir}/bf_{s}_{s}_{seed}");
        std::fs::File::create(format!("{stem}.bin")).unwrap().write_all(&bytes).unwrap();
        let path = build_path("row-major", s, s, 0).unwrap();
        let mut sv = Solver::new(&p, &path, true, false, 0).unwrap();
        let mut steps=0u64;
        loop { let r = sv.step(50_000_000); steps+=1; if r.status != Status::Running { break; } if steps>100 {break;} }
        let board: Vec<String> = sv.board().iter().map(|c| c.to_string()).collect();
        std::fs::write(format!("{stem}.board.txt"), board.join(" ")).unwrap();
        let r = sv.report();
        println!("wrote {stem}: attempts={} nodes={} backtracks={} board=[{}]",
            r.attempts as u64, r.nodes as u64, r.backtracks as u64, board.join(" "));
    }
}
