use eternity2_engine::{generate, build_path, Solver, Status};
use std::io::Write;
fn main() {
    let dir = "../engine-side-quests/engine-brainfuck/data";
    for &(s, c, seed) in &[(4u8,4u8,11u32),(4,4,3u32)] {
        let p = generate(s, c, seed);
        let mut bytes = vec![s];
        for e in &p.pieces { for &x in e { bytes.push(x); } }
        let stem = format!("{dir}/bf_{s}_{s}_{seed}");
        std::fs::File::create(format!("{stem}.bin")).unwrap().write_all(&bytes).unwrap();
        let path = build_path("row-major", s, s, 0).unwrap();
        let mut sv = Solver::new(&p, &path, true, false, 0).unwrap();
        loop { let r = sv.step(50_000_000); if r.status != Status::Running { break; } }
        let board: Vec<String> = sv.board().iter().map(|c| c.to_string()).collect();
        std::fs::write(format!("{stem}.board.txt"), board.join(" ")).unwrap();
        println!("4x4 seed {seed}: board=[{}]", board.join(" "));
    }
}
