//! Cross-validation tests. The load-bearing one is scorer parity: a known
//! 469/480 community board, expressed as bucas edges, must score 469 through
//! our canonical scorer — the same number the sibling benchmark's `score_cells`
//! returns for it. If this passes, the study's scorer is the same source of
//! truth as the rest of the site.

use dfs_core::{score_cells, Board, MAX_SCORE_16, N};

use crate::{board_to_bucas_url, Instance};

const VARIANT_00: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../../variants/variant_00.json");
const VARIANT_00_CSV: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../../variants/variant_00.csv");

// A verified McGavin/Blackwood 469/480 board as bucas board_edges (4 URDL
// letters per cell), lifted from the benchmark's cross-validation test.
const MCGAVIN_469_EDGES: &str = "adcaaendadweadhdafwdafgfaemfacqeaencafqeadofaepdaeueaeseadteaacdcnfansnnwuwshlsuwvvlgsgvmtrsqvgtntqvkkntokvkpllkulplsnhltovncacofqfangwqwwvgstgwvphtgnnprqungqmqqkwhuwokvwlwlrqwpnqrhsknvpnscadpfvcawkrvvrnkglorhuqlnvhuusuvmonpwnspouqnlkvuqmokqhwmkrphlslrdadscpcarqvpntmqopstqrqphmorulwmnkolsntkqoqnuvgoosnqwsoqphssoggkdadgcrfavmkrmupmsthuqqwtoriqwmmrorrmtrgrqrtrgtmrnlktogllsquggtnqdaftfoeaknvopgtnhhlgwushiowumnlorkmngtrktlqtmnwlkounllwougolnkpgfackelfavhnltrwhluqrsrtuwvlrlgvvmkigrwpkqlhwwokluisowquiooiqpprocaepfufanhhuwkuhqtjktpjtloupvmjoijjmpppjhhrpkprhskppuvtkitvvrvmteabvfhcahwjhuhmwjnthjronuhhrjqphjskqpgmsrusgrhouptwhtkmtvrtkmhmrbafhcsbajwgsmsowtjisoqtjhmkqpjnmkgrjmpqgsuvpoujuwovummsotlgmmtplfabtbteagiwtomniiwpmtwvwksmwnplsrilpqwoivkvwjhukvrkhsjprgwvjppvwbaepeibawilinqwiphiqvjshmlljlkillqikovjqvijvuksikgikpnigvjnnvomjeabobjfalijjwgiiisjgsksslqjkiigqisoijuhsjosuslgoirilijurnvijmqlvbaeqfjbajrijingrjminsutmjtrugigtommihjumsiujggjiimhgujvmijpjlthjeaetbdaaidadgcadidactcadrbacgbabmdabufadubafjbabhcabvbacpcabhbaceaab";

#[test]
fn scorer_agrees_with_known_469() {
    let inst = Instance::from_site_json(VARIANT_00).expect("load variant 00");
    let cells: Vec<[u8; 4]> = MCGAVIN_469_EDGES
        .as_bytes()
        .chunks(4)
        .map(|c| [c[0] - b'a', c[1] - b'a', c[2] - b'a', c[3] - b'a'])
        .collect();
    assert_eq!(cells.len(), N);
    assert_eq!(score_cells(&cells), 469, "canonical scorer must return 469");
    // And the same board, rebuilt onto a Board via piece matching, re-scores 469
    // through the board path too (exercises to_edge_cells + rotated).
    let mut used = vec![false; inst.pieces.len()];
    let mut board = Board::new();
    for (pos, &cell) in cells.iter().enumerate() {
        'find: for (pid, p) in inst.pieces.iter() {
            if used[pid as usize] {
                continue;
            }
            for r in 0..4 {
                if p.rotated(r) == cell {
                    board.place(pos, pid, r);
                    used[pid as usize] = true;
                    break 'find;
                }
            }
        }
    }
    let out = inst.finish(&board);
    assert_eq!(out.score, 469);
    assert_eq!(out.breaks, MAX_SCORE_16 - 469);
}

#[test]
fn empty_board_scores_zero_and_full_breaks() {
    let inst = Instance::from_site_json(VARIANT_00).expect("load variant 00");
    let out = inst.finish(&Board::new());
    assert_eq!(out.score, 0);
    assert_eq!(out.breaks, MAX_SCORE_16);
    // Bucas URL for an empty board is all 'a' (border color 0).
    assert!(out.bucas_url.contains(&"a".repeat(8)));
}

#[test]
fn variant_loads_256_pieces() {
    let inst = Instance::from_site_json(VARIANT_00).expect("load variant 00");
    assert_eq!(inst.pieces.len(), 256);
    assert_eq!(inst.max_score(), 480);
    // Variant 00 pins 5 official clues + 3 corners = 8 hints.
    assert_eq!(inst.hints.len(), 8);
}

#[test]
fn csv_and_site_agree_on_pieces() {
    // The same variant read from CSV and from site JSON must yield the same
    // piece set and the same hint set — proving the two readers are consistent.
    let from_site = Instance::from_site_json(VARIANT_00).expect("site");
    let from_csv = crate::load_puzzle_csv(VARIANT_00_CSV).expect("csv");
    assert_eq!(from_site.pieces.len(), from_csv.pieces.len());
    for (id, p) in from_site.pieces.iter() {
        assert_eq!(p.edges, from_csv.pieces.edges(id), "piece {id} edges differ");
    }
    let mut a: Vec<_> = from_site.hints.iter().map(|h| (h.pos, h.piece, h.rot)).collect();
    let mut b: Vec<_> = from_csv.hints.iter().map(|h| (h.pos, h.piece, h.rot)).collect();
    a.sort_unstable();
    b.sort_unstable();
    assert_eq!(a, b, "hint sets differ between site JSON and CSV");
}

#[test]
fn round_trip_site_csv_site() {
    // site -> csv -> site preserves the piece set (the conversion utility is
    // lossless on pieces).
    let inst = Instance::from_site_json(VARIANT_00).expect("site");
    let csv = inst.to_csv();
    let back = crate::parse_puzzle_csv("rt", &csv).expect("parse round-tripped csv");
    assert_eq!(inst.pieces.len(), back.pieces.len());
    for (id, p) in inst.pieces.iter() {
        assert_eq!(p.edges, back.pieces.edges(id));
    }
}

#[test]
fn bucas_url_shape() {
    // 256 cells * 4 letters = 1024 edge letters in the URL.
    let cells = vec![[1u8, 2, 3, 4]; N];
    let url = board_to_bucas_url("t", 16, 16, &cells);
    let edges = url.split("board_edges=").nth(1).unwrap();
    assert_eq!(edges.len(), N * 4);
}
