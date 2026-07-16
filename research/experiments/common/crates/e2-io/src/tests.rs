//! Cross-validation tests. The load-bearing one is scorer parity: a known
//! 469/480 community board, expressed as bucas edges, must score 469 through
//! this crate's canonical scorer — the same number the site's `score_cells`
//! returns for it. If this passes, every engine built on this library scores
//! boards against the same source of truth as the rest of the site.

use e2_core::{score_cells, Board, MAX_SCORE_16, N};

use crate::{board_to_bucas_url, Instance};

// A self-contained fixture — one corner-pinned variant, checked into this crate
// so the shared library's tests do not reach into any one consumer's directory.
const VARIANT_00: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/variant_00.json");
const VARIANT_00_CSV: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/variant_00.csv");

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
fn match_board_recovers_full_placement_from_edges() {
    // The known 469 board, expressed as edges only, matched back against the
    // official set, re-scores 469 and recovers all 256 pieces — this is the
    // url2json migration path: an edge-only URL becomes a complete document.
    let inst = Instance::from_site_json(VARIANT_00).expect("load variant 00");
    let cells = crate::parse_board_edges(MCGAVIN_469_EDGES).expect("parse edges");
    assert_eq!(cells.len(), N);
    let out = inst.match_board(&cells);
    assert_eq!(out.score, 469);
    let doc = out.to_doc();
    let placed = doc.board.iter().filter(|&&c| c >= 0).count();
    assert_eq!(placed, N, "all 256 pieces recovered by matching");
    assert!(doc.board_pieces.chars().any(|c| c != '0'));
    assert!(doc.url.starts_with("https://eternity2.dev/viewer?"));
}

#[test]
fn placement_to_edges_round_trips_via_rotated() {
    // The exact path the `board_doc` binary uses to build a canonical BoardDoc
    // from a board's explicit (piece_id, rotation) placement: rebuild each cell's
    // URDL quad with e2_core::rotated and confirm the board re-emits its own edge
    // string. This pins the rotation convention the published dataset depends on.
    use e2_core::rotated;
    let inst = Instance::from_site_json(VARIANT_00).expect("load variant 00");
    let source_cells = crate::parse_board_edges(MCGAVIN_469_EDGES).expect("parse edges");
    // Recover a concrete placement (piece*4+rot codes) for the known board.
    let codes = inst.match_board(&source_cells).board;
    assert_eq!(codes.len(), N);

    // Rebuild cells straight from the codes, as the binary does from placement.
    let mut rebuilt = vec![[0u8; 4]; N];
    for (pos, &code) in codes.iter().enumerate() {
        if code < 0 {
            continue;
        }
        let piece_id = (code >> 2) as u16;
        let rot = (code & 0b11) as u8;
        rebuilt[pos] = rotated(inst.pieces.edges(piece_id), rot);
    }
    assert_eq!(rebuilt, source_cells, "placement re-emits the exact source edges");
    assert_eq!(e2_core::score_cells(&rebuilt), 469, "and re-scores canonically");
}

#[test]
fn empty_board_scores_zero_and_full_breaks() {
    let inst = Instance::from_site_json(VARIANT_00).expect("load variant 00");
    let out = inst.finish(&Board::new());
    assert_eq!(out.score, 0);
    assert_eq!(out.breaks, MAX_SCORE_16);
    // The canonical URL is now an eternity2.dev viewer link, and an empty
    // board's board_edges is all 'a' (border color 0).
    assert!(out.url.starts_with("https://eternity2.dev/viewer?"));
    let doc = out.to_doc();
    assert!(doc.board_edges.contains(&"a".repeat(8)));
    assert_eq!(doc.board_edges.len(), N * 4);
    // The doc round-trips through its own canonical JSON.
    let back: crate::BoardDoc = serde_json::from_str(&doc.to_json()).expect("doc json");
    assert_eq!(back, doc);
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
