//! Dump the embedded official Eternity II puzzle (256 pieces, 5 clue hints)
//! as site-schema JSON, so the runtime-sized `hint_scale` driver can be run on
//! the real board with any path order.
//!
//! Usage: official_json > official_e2.json

use eternity2_engine::official_puzzle;

fn main() {
    let p = official_puzzle();
    println!("{}", serde_json::to_string_pretty(&p).expect("serialise official puzzle"));
}
