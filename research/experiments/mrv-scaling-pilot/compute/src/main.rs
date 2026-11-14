//! Pilot instances (PREREG.md): 9x9 on the A22 `plus2` track (c = primary + 2
//! = 11), fresh test seeds 101..108, from the ladder's own generator call
//! `hard_ladder::instance::make_instance(Ladder::PlusTwo, Role::Test, 9, seed)`
//! (= `generate(9, 11, seed)` plus the ladder scramble, amendment A4).
//!
//! pilot_instances write   # writes ../instances/plus2-test.jsonl
//! pilot_instances check   # committed file == generator output, plus anchors

use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use hard_ladder::cli::load_instances;
use hard_ladder::instance::{colours, make_instance};
use hard_ladder::{Ladder, Role};

const N: u8 = 9;
const SEEDS: std::ops::RangeInclusive<u32> = 101..=108;

fn path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../instances/plus2-test.jsonl")
}

fn generated() -> String {
    SEEDS
        .map(|s| {
            serde_json::to_string(&make_instance(Ladder::PlusTwo, Role::Test, N, s)).expect("json")
                + "\n"
        })
        .collect()
}

fn check() -> Result<(), String> {
    if colours(Ladder::PlusTwo, N) != 11 {
        return Err("plus2 colour count at n = 9 is not 11".into());
    }
    let committed = fs::read_to_string(path()).map_err(|e| e.to_string())?;
    if committed != generated() {
        return Err(format!("{} differs from the generator", path().display()));
    }
    // Anchor: the same call reproduces the committed A22 ladder row n9-c11-s1,
    // so seeds 101..108 come from exactly the generator A22 used.
    let a22 = load_instances(Ladder::PlusTwo, Role::Test)?;
    let row = a22
        .iter()
        .find(|i| i.n == N && i.seed == 1)
        .ok_or("A22 row n9 s1 missing")?;
    let again = make_instance(Ladder::PlusTwo, Role::Test, N, 1);
    if serde_json::to_string(row).map_err(|e| e.to_string())?
        != serde_json::to_string(&again).map_err(|e| e.to_string())?
    {
        return Err("generator does not reproduce A22 row n9-c11-s1".into());
    }
    if a22.iter().any(|i| SEEDS.contains(&i.seed)) {
        return Err("a pilot seed is already in the A22 ladder".into());
    }
    eprintln!("pilot instances match the generator (n = 9, c = 11, seeds 101..108); A22 anchor ok");
    Ok(())
}

fn main() -> ExitCode {
    let res = match std::env::args().nth(1).as_deref() {
        Some("write") => fs::write(path(), generated()).map_err(|e| e.to_string()),
        Some("check") => check(),
        _ => Err("usage: pilot_instances write|check".into()),
    };
    match res {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
