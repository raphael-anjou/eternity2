// Emit per-puzzle piece data files for the COBOL solver.
// Format: line 1 = "SIZE COLORS SEED"; then one piece per line "u r d l".
use eternity2_engine::generate;
use std::io::Write;
fn main() {
    for &(s, c, seed) in &[(4u8,4u8,11u32),(5,5,3),(5,6,42),(3,3,1)] {
        let p = generate(s, c, seed);
        let path = format!("../engine-side-quests/engine-cobol/data/p_{s}_{c}_{seed}.txt");
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "{s} {c} {seed}").unwrap();
        for e in &p.pieces { writeln!(f, "{} {} {} {}", e[0],e[1],e[2],e[3]).unwrap(); }
        println!("wrote {path}");
    }
}
