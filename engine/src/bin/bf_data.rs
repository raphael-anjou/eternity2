// Emit raw-byte input files for the Brainfuck solver and the expected board,
// for every bundled test size (3x3, 4x4, 5x5).
// <name>.bin : SIZE byte, then N pieces * 4 edge bytes (URDL).
// <name>.board.txt : space-separated expected board (cell -> piece*4+rot).
use eternity2_engine::{build_path, generate, Solver, Status};
use std::io::Write;

/// (size, colors, seed) for every board the Brainfuck port is validated on.
const CASES: &[(u8, u8, u32)] = &[
    (3, 3, 1),
    (3, 3, 2),
    (3, 3, 5),
    (3, 3, 7),
    (4, 4, 11),
    (4, 4, 3),
    (5, 5, 3),
    (5, 6, 42),
];

fn main() {
    let dir = "../engine-side-quests/engine-brainfuck/data";
    std::fs::create_dir_all(dir).unwrap();
    for &(s, c, seed) in CASES {
        let p = generate(s, c, seed);
        let mut bytes = vec![s];
        for e in &p.pieces {
            for &x in e {
                bytes.push(x);
            }
        }
        let stem = format!("{dir}/bf_{s}_{s}_{seed}");
        std::fs::File::create(format!("{stem}.bin"))
            .unwrap()
            .write_all(&bytes)
            .unwrap();
        let path = build_path("row-major", s, s, 0).unwrap();
        let mut sv = Solver::new(&p, &path, true, false, 0).unwrap();
        // Safety cap: the naive solver clears every bundled case well within
        // this many step batches; the bound just prevents an accidental hang.
        let mut steps = 0u64;
        loop {
            let r = sv.step(50_000_000);
            steps += 1;
            if r.status != Status::Running || steps > 100 {
                break;
            }
        }
        let board: Vec<String> = sv.board().iter().map(|c| c.to_string()).collect();
        std::fs::write(format!("{stem}.board.txt"), board.join(" ")).unwrap();
        let r = sv.report();
        println!(
            "wrote {stem}: attempts={} nodes={} backtracks={} board=[{}]",
            r.attempts as u64,
            r.nodes as u64,
            r.backtracks as u64,
            board.join(" ")
        );
    }
}
