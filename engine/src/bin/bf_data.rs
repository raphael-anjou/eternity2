// Emit raw-byte input files for the Brainfuck solver and the expected board.
// <name>.bin : SIZE byte, then N pieces * 4 edge bytes (URDL).
// <name>.board.txt : space-separated expected board (cell -> piece*4+rot).
use eternity2_engine::{generate, build_path, Solver, Status};
use std::io::Write;
fn main() {
    let dir = "../engine-side-quests/engine-brainfuck/data";
    std::fs::create_dir_all(dir).unwrap();
    for &(s, c, seed) in &[(3u8,3u8,1u32),(3,3,2),(3,3,5),(3,3,7)] {
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
        println!("wrote {stem}.bin + .board.txt  board=[{}]", board.join(" "));
    }
}
