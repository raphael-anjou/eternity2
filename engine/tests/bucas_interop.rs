//! Cross-validation against e2.bucas.name: decode the real McGavin/Blackwood
//! 469/480 board (bundled in the viewer's init.js, default motif letters) and
//! check that our piece set, rotation convention, hint table, and scoring all
//! agree with it.

use eternity2_engine::{canonical, official_puzzle, rotated, BORDER};

/// board_edges of "JBlackwood+PMcGavin_469": 4 letters (URDL) per cell,
/// row-major, 'a' = 0 = grey. Source: e2.bucas.name init.js (GPL-3.0).
const MCGAVIN_469_EDGES: &str = "adcaaendadweadhdafwdafgfaemfacqeaencafqeadofaepdaeueaeseadteaacdcnfansnnwuwshlsuwvvlgsgvmtrsqvgtntqvkkntokvkpllkulplsnhltovncacofqfangwqwwvgstgwvphtgnnprqungqmqqkwhuwokvwlwlrqwpnqrhsknvpnscadpfvcawkrvvrnkglorhuqlnvhuusuvmonpwnspouqnlkvuqmokqhwmkrphlslrdadscpcarqvpntmqopstqrqphmorulwmnkolsntkqoqnuvgoosnqwsoqphssoggkdadgcrfavmkrmupmsthuqqwtoriqwmmrorrmtrgrqrtrgtmrnlktogllsquggtnqdaftfoeaknvopgtnhhlgwushiowumnlorkmngtrktlqtmnwlkounllwougolnkpgfackelfavhnltrwhluqrsrtuwvlrlgvvmkigrwpkqlhwwokluisowquiooiqpprocaepfufanhhuwkuhqtjktpjtloupvmjoijjmpppjhhrpkprhskppuvtkitvvrvmteabvfhcahwjhuhmwjnthjronuhhrjqphjskqpgmsrusgrhouptwhtkmtvrtkmhmrbafhcsbajwgsmsowtjisoqtjhmkqpjnmkgrjmpqgsuvpoujuwovummsotlgmmtplfabtbteagiwtomniiwpmtwvwksmwnplsrilpqwoivkvwjhukvrkhsjprgwvjppvwbaepeibawilinqwiphiqvjshmlljlkillqikovjqvijvuksikgikpnigvjnnvomjeabobjfalijjwgiiisjgsksslqjkiigqisoijuhsjosuslgoirilijurnvijmqlvbaeqfjbajrijingrjminsutmjtrugigtommihjumsiujggjiimhgujvmijpjlthjeaetbdaaidadgcadidactcadrbacgbabmdabufadubafjbabhcabvbacpcabhbaceaab";

fn decode_cells() -> Vec<[u8; 4]> {
    assert_eq!(MCGAVIN_469_EDGES.len(), 1024);
    MCGAVIN_469_EDGES
        .as_bytes()
        .chunks(4)
        .map(|c| [c[0] - b'a', c[1] - b'a', c[2] - b'a', c[3] - b'a'])
        .collect()
}

fn raw_score(cells: &[[u8; 4]], w: usize, h: usize) -> u32 {
    let mut score = 0;
    for y in 0..h {
        for x in 0..w {
            let a = cells[y * w + x];
            if x + 1 < w {
                let b = cells[y * w + x + 1];
                if a[1] == b[3] && a[1] != BORDER {
                    score += 1;
                }
            }
            if y + 1 < h {
                let b = cells[(y + 1) * w + x];
                if a[2] == b[0] && a[2] != BORDER {
                    score += 1;
                }
            }
        }
    }
    score
}

#[test]
fn mcgavin_469_board_scores_469() {
    assert_eq!(raw_score(&decode_cells(), 16, 16), 469);
}

#[test]
fn piece_multiset_matches_official_set() {
    let cells = decode_cells();
    let mut ours: Vec<[u8; 4]> = official_puzzle().pieces.iter().map(|&e| canonical(e)).collect();
    let mut theirs: Vec<[u8; 4]> = cells.iter().map(|&e| canonical(e)).collect();
    ours.sort_unstable();
    theirs.sort_unstable();
    assert_eq!(ours, theirs);
}

#[test]
fn center_clue_matches_469_board() {
    // Record boards obey the mandatory center clue (the only one bucas
    // verifies); the corner clues are usually ignored by record hunters.
    let cells = decode_cells();
    let puzzle = official_puzzle();
    let center = puzzle
        .hints
        .iter()
        .find(|h| h.pos == 135)
        .expect("center clue at cell 135");
    assert_eq!(center.piece, 138);
    let expected = rotated(puzzle.pieces[center.piece as usize], center.rot);
    assert_eq!(cells[135], expected);
    assert_eq!(cells[135], [8, 9, 9, 12]); // "ijjm"
}

#[test]
fn all_five_hints_match_official_clues_board() {
    // The viewer's bundled "Clues" board pins exactly the 5 official clue
    // pieces, but in motifs_order=jef letters, so this also validates the
    // jef→bucas translation table.
    const JEF_TO_BUCAS: &[u8; 23] = b"atojeqlgbupkfrmhcwvsnid";
    let clue_cells: [(u16, &str); 5] = [
        (34, "rgou"),  // piece 208 (1-based) at (2,2)
        (45, "rtrj"),  // piece 255 at (13,2)
        (135, "vddo"), // piece 139 at (7,8), the center clue
        (210, "jdso"), // piece 181 at (2,13)
        (221, "fskn"), // piece 249 at (13,13)
    ];
    let puzzle = official_puzzle();
    assert_eq!(puzzle.hints.len(), 5);
    for (pos, jef) in clue_cells {
        let cell: Vec<u8> = jef
            .bytes()
            .map(|b| {
                let bucas_letter = JEF_TO_BUCAS[(b - b'a') as usize];
                bucas_letter - b'a'
            })
            .collect();
        let hint = puzzle
            .hints
            .iter()
            .find(|h| h.pos == pos)
            .unwrap_or_else(|| panic!("no hint at pos {pos}"));
        let expected = rotated(puzzle.pieces[hint.piece as usize], hint.rot);
        assert_eq!(cell, expected, "clue at pos {pos}");
    }
}

#[test]
fn board_reconstruction_via_piece_matching_scores_469() {
    // Recover (piece_id, rotation) per cell the way our viewer importer does,
    // then score through the engine's score_board.
    let cells = decode_cells();
    let puzzle = official_puzzle();
    let mut used = vec![false; puzzle.pieces.len()];
    let mut board = vec![-1i32; 256];
    for (pos, &cell) in cells.iter().enumerate() {
        let mut found = false;
        'pieces: for (pid, &e) in puzzle.pieces.iter().enumerate() {
            if used[pid] {
                continue;
            }
            for r in 0..4u8 {
                if rotated(e, r) == cell {
                    used[pid] = true;
                    board[pos] = pid as i32 * 4 + i32::from(r);
                    found = true;
                    break 'pieces;
                }
            }
        }
        assert!(found, "no unused piece matches cell {pos}");
    }
    assert_eq!(eternity2_engine::score_board(&puzzle, &board), 469);
}
