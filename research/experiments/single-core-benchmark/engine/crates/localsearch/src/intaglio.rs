// INTAGLIO — forbidden 2x2 patch detection.
//
// For a 4-tuple of piece IDs (p_tl, p_tr, p_bl, p_br), a 2x2 patch is
// FEASIBLE if some rotation assignment (r_tl, r_tr, r_bl, r_br) makes:
//   - p_tl.E (rotated) == p_tr.W (rotated)  (internal vertical edge)
//   - p_tl.S (rotated) == p_bl.N (rotated)  (internal horizontal edge)
//   - p_tr.S (rotated) == p_br.N (rotated)
//   - p_bl.E (rotated) == p_br.W (rotated)
//   - all internal edges are non-border (color != 0)
//
// V138-139 finding: 99.72% of random 4-tuples are forbidden on canonical
// E2. Forbidden-count anti-correlates with board score (LOW 109, MID 60,
// HIGH 29 medians). Used as secondary-objective tiebreak in ALNS.

use eternity2_core::{Board, PieceId, Puzzle, Rotation, BORDER};

/// Check if a 2x2 patch of 4 piece IDs is feasible under SOME rotation
/// assignment. Returns true if forbidden (NO assignment works).
pub fn is_forbidden_2x2(
    puzzle: &Puzzle,
    p_tl: PieceId,
    p_tr: PieceId,
    p_bl: PieceId,
    p_br: PieceId,
) -> bool {
    let tl_piece = match puzzle.piece(p_tl) { Some(p) => p, None => return true };
    let tr_piece = match puzzle.piece(p_tr) { Some(p) => p, None => return true };
    let bl_piece = match puzzle.piece(p_bl) { Some(p) => p, None => return true };
    let br_piece = match puzzle.piece(p_br) { Some(p) => p, None => return true };

    for r_tl in 0..4u8 {
        let tl = tl_piece.edges.rotated(Rotation::from_u8(r_tl).unwrap()).as_array();
        if tl[1] == BORDER || tl[2] == BORDER { continue; }
        for r_tr in 0..4u8 {
            let tr = tr_piece.edges.rotated(Rotation::from_u8(r_tr).unwrap()).as_array();
            if tl[1] != tr[3] { continue; }
            if tr[2] == BORDER { continue; }
            for r_bl in 0..4u8 {
                let bl = bl_piece.edges.rotated(Rotation::from_u8(r_bl).unwrap()).as_array();
                if tl[2] != bl[0] { continue; }
                if bl[1] == BORDER { continue; }
                for r_br in 0..4u8 {
                    let br = br_piece.edges.rotated(Rotation::from_u8(r_br).unwrap()).as_array();
                    if tr[2] != br[0] { continue; }
                    if bl[1] != br[3] { continue; }
                    return false; // feasible
                }
            }
        }
    }
    true
}

/// Count all forbidden 2x2 patches on the board. Scans (W-1) × (H-1)
/// interior corners. Uses piece IDs only (rotation-agnostic via the
/// is_forbidden_2x2 check).
pub fn count_forbidden_2x2(puzzle: &Puzzle, board: &Board) -> u32 {
    let w = puzzle.width;
    let h = puzzle.height;
    let mut forbidden = 0u32;
    for y in 0..(h - 1) {
        for x in 0..(w - 1) {
            let p_tl = match board.get(y * w + x) { Some((pid, _)) => pid, None => continue };
            let p_tr = match board.get(y * w + (x + 1)) { Some((pid, _)) => pid, None => continue };
            let p_bl = match board.get((y + 1) * w + x) { Some((pid, _)) => pid, None => continue };
            let p_br = match board.get((y + 1) * w + (x + 1)) { Some((pid, _)) => pid, None => continue };
            if is_forbidden_2x2(puzzle, p_tl, p_tr, p_bl, p_br) {
                forbidden += 1;
            }
        }
    }
    forbidden
}

/// Count forbidden 2x2 patches that include a CHANGED position.
/// Useful for incremental scoring after a swap-pair move.
pub fn count_forbidden_2x2_near(
    puzzle: &Puzzle,
    board: &Board,
    pos: eternity2_core::Position,
) -> u32 {
    let w = puzzle.width;
    let h = puzzle.height;
    let x = pos % w;
    let y = pos / w;
    let mut forbidden = 0u32;
    // The pos can be TL/TR/BL/BR of up to 4 different 2x2 patches.
    for dy in 0..=1u32 {
        for dx in 0..=1u32 {
            if dy > y || dx > x { continue; }
            let ty = y - dy;
            let tx = x - dx;
            if ty + 1 >= h || tx + 1 >= w { continue; }
            let p_tl = match board.get(ty * w + tx) { Some((pid, _)) => pid, None => continue };
            let p_tr = match board.get(ty * w + (tx + 1)) { Some((pid, _)) => pid, None => continue };
            let p_bl = match board.get((ty + 1) * w + tx) { Some((pid, _)) => pid, None => continue };
            let p_br = match board.get((ty + 1) * w + (tx + 1)) { Some((pid, _)) => pid, None => continue };
            if is_forbidden_2x2(puzzle, p_tl, p_tr, p_bl, p_br) {
                forbidden += 1;
            }
        }
    }
    forbidden
}

/// Check if a 2x3 (2 rows × 3 cols) patch is feasible under some rotation.
pub fn is_forbidden_2x3(
    puzzle: &Puzzle,
    p_tl: PieceId, p_tm: PieceId, p_tr: PieceId,
    p_bl: PieceId, p_bm: PieceId, p_br: PieceId,
) -> bool {
    let tl_p = match puzzle.piece(p_tl) { Some(p) => p, None => return true };
    let tm_p = match puzzle.piece(p_tm) { Some(p) => p, None => return true };
    let tr_p = match puzzle.piece(p_tr) { Some(p) => p, None => return true };
    let bl_p = match puzzle.piece(p_bl) { Some(p) => p, None => return true };
    let bm_p = match puzzle.piece(p_bm) { Some(p) => p, None => return true };
    let br_p = match puzzle.piece(p_br) { Some(p) => p, None => return true };
    for r_tl in 0..4u8 {
        let tl = tl_p.edges.rotated(Rotation::from_u8(r_tl).unwrap()).as_array();
        if tl[1] == BORDER || tl[2] == BORDER { continue; }
        for r_tm in 0..4u8 {
            let tm = tm_p.edges.rotated(Rotation::from_u8(r_tm).unwrap()).as_array();
            if tl[1] != tm[3] { continue; }
            if tm[1] == BORDER || tm[2] == BORDER { continue; }
            for r_tr in 0..4u8 {
                let tr = tr_p.edges.rotated(Rotation::from_u8(r_tr).unwrap()).as_array();
                if tm[1] != tr[3] { continue; }
                if tr[2] == BORDER { continue; }
                for r_bl in 0..4u8 {
                    let bl = bl_p.edges.rotated(Rotation::from_u8(r_bl).unwrap()).as_array();
                    if tl[2] != bl[0] { continue; }
                    if bl[1] == BORDER { continue; }
                    for r_bm in 0..4u8 {
                        let bm = bm_p.edges.rotated(Rotation::from_u8(r_bm).unwrap()).as_array();
                        if tm[2] != bm[0] { continue; }
                        if bl[1] != bm[3] { continue; }
                        if bm[1] == BORDER { continue; }
                        for r_br in 0..4u8 {
                            let br = br_p.edges.rotated(Rotation::from_u8(r_br).unwrap()).as_array();
                            if tr[2] != br[0] { continue; }
                            if bm[1] != br[3] { continue; }
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}

/// Enumerate all forbidden 2x3 patches on the board. Returns (top-left
/// position, set of 6 positions that form the patch).
pub fn enumerate_forbidden_2x3(
    puzzle: &Puzzle,
    board: &Board,
) -> Vec<(u32, [u32; 6])> {
    let w = puzzle.width;
    let h = puzzle.height;
    let mut out = Vec::new();
    for y in 0..(h - 1) {
        for x in 0..(w - 2) {
            let tl_pos = y * w + x;
            let tm_pos = y * w + x + 1;
            let tr_pos = y * w + x + 2;
            let bl_pos = (y + 1) * w + x;
            let bm_pos = (y + 1) * w + x + 1;
            let br_pos = (y + 1) * w + x + 2;
            let p_tl = match board.get(tl_pos) { Some((p, _)) => p, None => continue };
            let p_tm = match board.get(tm_pos) { Some((p, _)) => p, None => continue };
            let p_tr = match board.get(tr_pos) { Some((p, _)) => p, None => continue };
            let p_bl = match board.get(bl_pos) { Some((p, _)) => p, None => continue };
            let p_bm = match board.get(bm_pos) { Some((p, _)) => p, None => continue };
            let p_br = match board.get(br_pos) { Some((p, _)) => p, None => continue };
            if is_forbidden_2x3(puzzle, p_tl, p_tm, p_tr, p_bl, p_bm, p_br) {
                out.push((tl_pos, [tl_pos, tm_pos, tr_pos, bl_pos, bm_pos, br_pos]));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_generator::{generate, GeneratorConfig};

    #[test]
    fn count_forbidden_runs() {
        let puzzle = generate(GeneratorConfig {
            size: 5, interior_colors: 4, seed: 0xCAFEBABE,
        }).expect("generate");
        let mut board = Board::empty(&puzzle);
        for i in 0..(puzzle.width * puzzle.height) {
            let pid: PieceId = i as PieceId;
            board.place(i, pid, Rotation::R0);
        }
        let n = count_forbidden_2x2(&puzzle, &board);
        // Just verify it runs (some forbidden expected on random placement).
        let _ = n;
    }
}
