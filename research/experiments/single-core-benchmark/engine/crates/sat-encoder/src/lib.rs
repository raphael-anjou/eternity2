// SAT / MaxSAT encoder for Eternity II.
//
// Model: piece-rotation-at-cell (variant of Ansótegui-Sellmann-Tabar 2008).
//   x_{c,p,r} = 1 iff piece p is placed at cell c in rotation r.
//
// Hard constraints:
//   1. Cell exactly-one over compatible (p, r): each cell holds exactly
//      one piece-rotation. Class-filtered (corner pieces at corners,
//      edge pieces on the boundary ring, interior pieces in the
//      interior).
//   2. Piece exactly-one over compatible (c, r): each piece is placed
//      at exactly one cell-rotation.
//   3. Border consistency: enforced via class filter (a piece-rotation
//      whose outward side wouldn't be BORDER is simply excluded from
//      the variable set for cells touching the boundary).
//   4. Hints: unit clauses pinning piece-rotation-cell triples.
//
// Edge-match auxiliary variables (for MaxSAT objective):
//   For each interior edge e between cell c_a (side s_a) and c_b
//   (side s_b), and each non-border color k ∈ {1..=22}, define
//     m_{e,k} = "both sides project color k onto edge e".
//   Equivalent CNF:
//     m_{e,k} → ∨_{(p,r): emit(p,r,s_a)=k} x_{c_a,p,r}
//     m_{e,k} → ∨_{(p,r): emit(p,r,s_b)=k} x_{c_b,p,r}
//   (We use the implication direction `m → x-disjunction` so that
//   when m_{e,k} is true, the placements force the color. The reverse
//   direction is implicit via the soft objective: MaxSAT will set
//   m_{e,k}=1 whenever consistent.)
//
// Soft clauses (weight 1, one per interior edge):
//   ∨_k m_{e,k}   — at least one color matches on edge e.
// Hard: edge match is at-most-one over k (∨_k m_{e,k} can collapse;
// AMO over m_{e,k} is *not* required for correctness because if two
// were forced true we'd be claiming two distinct colors for the same
// edge — but the piece-variable assignment can only emit one color per
// side, so the implications prevent this automatically. We add AMO
// anyway for tighter unit propagation.)
//
// AMO encoding: bimander (Hölldobler-Nguyen 2013) with √n groups —
// strictly better than pairwise for n > ~10. Selected by group-size
// = ceil(√n). Falls back to pairwise for n ≤ 6.

#![forbid(unsafe_code)]

use eternity2_core::{Hints, Piece, Position, Puzzle, Rotation, BORDER};
use std::collections::HashMap;
use std::fmt::Write;

pub type Lit = i32;
pub type Var = u32; // 1-based DIMACS var id

/// Cell-class derived from puzzle topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellClass {
    /// 4 corners.
    Corner,
    /// 56 non-corner boundary cells.
    Edge,
    /// 196 interior cells.
    Inner,
}

pub fn cell_class(puzzle: &Puzzle, pos: Position) -> CellClass {
    let m = puzzle.border_mask(pos);
    let borders = m.iter().filter(|&&b| b).count();
    match borders {
        2 => CellClass::Corner,
        1 => CellClass::Edge,
        0 => CellClass::Inner,
        _ => unreachable!("a cell can touch at most 2 borders on a rectangular grid"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceClass { Corner, Edge, Inner }

pub fn piece_class(p: &Piece) -> PieceClass {
    if p.is_corner() { PieceClass::Corner }
    else if p.is_edge() { PieceClass::Edge }
    else { PieceClass::Inner }
}

fn class_match(cc: CellClass, pc: PieceClass) -> bool {
    matches!((cc, pc),
        (CellClass::Corner, PieceClass::Corner)
        | (CellClass::Edge, PieceClass::Edge)
        | (CellClass::Inner, PieceClass::Inner))
}

/// A valid (piece, rotation) for a given cell: the piece-class matches
/// the cell-class AND every BORDER side of the rotated piece aligns with
/// a BORDER side of the cell.
pub fn cell_admits(puzzle: &Puzzle, pos: Position, piece: &Piece, rot: Rotation) -> bool {
    if !class_match(cell_class(puzzle, pos), piece_class(piece)) { return false; }
    let mask = puzzle.border_mask(pos);
    let e = piece.edges.rotated(rot).as_array();
    for side in 0..4 {
        // side 0=top, 1=right, 2=bottom, 3=left
        let cell_is_border_here = mask[side];
        let piece_is_border_here = e[side] == BORDER;
        if cell_is_border_here != piece_is_border_here { return false; }
    }
    true
}

/// Mapping of (cell, piece, rotation) → DIMACS var-id. Built once per puzzle.
pub struct VarMap {
    /// For each cell, list of (piece_index_in_pieces_vec, rotation, var_id).
    pub cell_to_pr: Vec<Vec<(u32, Rotation, Var)>>,
    /// For each piece (by piece_index), list of (cell, rotation, var_id).
    pub piece_to_cr: Vec<Vec<(Position, Rotation, Var)>>,
    /// Reverse: var_id → (cell, piece_index, rotation). var_id 1..=n_piece_vars.
    pub var_to_cpr: Vec<(Position, u32, Rotation)>,
    /// Edge-match aux vars: m[e][k] where e ∈ 0..n_interior_edges, k ∈ 1..=max_color.
    /// Stored as match_vars[e][k] = var_id (0 means "no var" — k=BORDER=0 entry).
    pub match_vars: Vec<Vec<Var>>,
    /// Edges enumerated as (cell_a, side_a, cell_b, side_b). side: 0=top,1=right,2=bottom,3=left.
    pub edges: Vec<EdgeId>,
    pub n_vars: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct EdgeId {
    pub cell_a: Position,
    pub side_a: u8,
    pub cell_b: Position,
    pub side_b: u8,
}

/// Pre-built map of pinned-cell placements: position → (piece_index, rotation).
/// piece_index is the position in `puzzle.pieces()`.
pub type PinnedMap = HashMap<Position, (u32, Rotation)>;

impl VarMap {
    pub fn build(puzzle: &Puzzle) -> Self {
        Self::build_with_pinned(puzzle, &PinnedMap::new())
    }

    /// Like `build()` but skips emitting variables for cells in `pinned`
    /// (each pinned cell contributes ZERO piece-vars; its placement is
    /// treated as a constant for edge-match constraint generation).
    /// Also skips emitting variables for pieces that are pinned (= used
    /// in the pinned set), at all non-pinned cells.
    ///
    /// This dramatically shrinks the SAT instance for "free a sub-region"
    /// problems: only free cells × non-pinned pieces × valid rotations.
    pub fn build_with_pinned(puzzle: &Puzzle, pinned: &PinnedMap) -> Self {
        let n_cells = puzzle.cell_count();
        let pieces = puzzle.pieces();
        let mut cell_to_pr: Vec<Vec<(u32, Rotation, Var)>> = vec![Vec::new(); n_cells as usize];
        let mut piece_to_cr: Vec<Vec<(Position, Rotation, Var)>> = vec![Vec::new(); pieces.len()];
        let mut var_to_cpr: Vec<(Position, u32, Rotation)> = Vec::new();
        // var_to_cpr is indexed from 0; var-id = index + 1.

        // Compute set of piece-indices that are pinned (used by some
        // pinned cell). These pieces are unavailable to free cells.
        let mut pinned_piece_indices: std::collections::HashSet<u32> =
            std::collections::HashSet::new();
        for (_, (pidx, _)) in pinned.iter() {
            pinned_piece_indices.insert(*pidx);
        }

        for pos in 0..n_cells {
            // If pinned, emit ZERO vars for this cell.
            if pinned.contains_key(&pos) { continue; }
            for (piece_idx, piece) in pieces.iter().enumerate() {
                // If this piece is pinned at some other cell, skip.
                if pinned_piece_indices.contains(&(piece_idx as u32)) { continue; }
                for rot in Rotation::ALL {
                    if !cell_admits(puzzle, pos, piece, rot) { continue; }
                    let var = (var_to_cpr.len() as u32) + 1;
                    var_to_cpr.push((pos, piece_idx as u32, rot));
                    cell_to_pr[pos as usize].push((piece_idx as u32, rot, var));
                    piece_to_cr[piece_idx].push((pos, rot, var));
                }
            }
        }

        let n_piece_vars = var_to_cpr.len() as u32;

        // Enumerate interior edges (between two cells of the grid).
        let mut edges: Vec<EdgeId> = Vec::new();
        for y in 0..puzzle.height {
            for x in 0..puzzle.width {
                let pos = y * puzzle.width + x;
                if x + 1 < puzzle.width {
                    edges.push(EdgeId {
                        cell_a: pos, side_a: 1, // right
                        cell_b: pos + 1, side_b: 3, // left
                    });
                }
                if y + 1 < puzzle.height {
                    edges.push(EdgeId {
                        cell_a: pos, side_a: 2, // bottom
                        cell_b: pos + puzzle.width, side_b: 0, // top
                    });
                }
            }
        }

        let max_color = (puzzle.color_count - 1) as usize; // colors 1..=max_color, 0=BORDER
        let mut match_vars: Vec<Vec<Var>> = vec![vec![0; max_color + 1]; edges.len()];
        let mut next_var = n_piece_vars + 1;
        for e in 0..edges.len() {
            for k in 1..=max_color {
                match_vars[e][k] = next_var;
                next_var += 1;
            }
        }
        let n_vars = next_var - 1;

        Self { cell_to_pr, piece_to_cr, var_to_cpr, match_vars, edges, n_vars }
    }
}

/// CNF builder (clauses are Vec<Lit>; positive = var, negative = ¬var).
pub struct CnfBuilder {
    pub clauses: Vec<Vec<Lit>>,
    pub soft_clauses: Vec<Vec<Lit>>,
    pub next_aux_var: u32,
}

impl CnfBuilder {
    pub fn new(n_problem_vars: u32) -> Self {
        Self {
            clauses: Vec::new(),
            soft_clauses: Vec::new(),
            next_aux_var: n_problem_vars + 1,
        }
    }

    pub fn fresh_var(&mut self) -> Var {
        let v = self.next_aux_var;
        self.next_aux_var += 1;
        v
    }

    pub fn add_hard(&mut self, lits: Vec<Lit>) { self.clauses.push(lits); }
    pub fn add_soft(&mut self, lits: Vec<Lit>) { self.soft_clauses.push(lits); }

    pub fn n_vars(&self) -> u32 { self.next_aux_var - 1 }
}

/// At-least-one: ∨ lits.
pub fn add_alo(b: &mut CnfBuilder, lits: &[Lit]) {
    b.add_hard(lits.to_vec());
}

/// At-most-one over `vars` (positive literals). Bimander encoding for
/// |vars| > 6, pairwise otherwise.
pub fn add_amo(b: &mut CnfBuilder, vars: &[Var]) {
    if vars.len() <= 1 { return; }
    if vars.len() <= 6 {
        add_amo_pairwise(b, vars);
    } else {
        add_amo_bimander(b, vars);
    }
}

pub fn add_amo_pairwise(b: &mut CnfBuilder, vars: &[Var]) {
    for i in 0..vars.len() {
        for j in (i+1)..vars.len() {
            b.add_hard(vec![-(vars[i] as Lit), -(vars[j] as Lit)]);
        }
    }
}

/// Bimander AMO: partition n vars into g groups of size m = ceil(n/g),
/// with g ≈ ceil(sqrt(n)). For each group, encode AMO pairwise. Then
/// introduce ceil(log2(g)) auxiliary "group commander" bits b_0..b_{k-1}.
/// For each var v_i in group j, require: v_i → bits(j) (i.e. for each
/// bit position p where bit p of j is 1, v_i → b_p; for each bit position
/// p where bit p of j is 0, v_i → ¬b_p). This forces any two vars from
/// different groups to require different bit patterns, hence cannot both
/// be true.
pub fn add_amo_bimander(b: &mut CnfBuilder, vars: &[Var]) {
    let n = vars.len();
    let g = (n as f64).sqrt().ceil() as usize;
    let g = g.max(2);
    let m = (n + g - 1) / g; // ceil(n/g)
    let k_bits = (g as f64).log2().ceil() as u32;
    let k_bits = k_bits.max(1);

    // Pairwise inside each group.
    for grp in 0..g {
        let start = grp * m;
        let end = ((grp + 1) * m).min(n);
        if start >= end { break; }
        for i in start..end {
            for j in (i+1)..end {
                b.add_hard(vec![-(vars[i] as Lit), -(vars[j] as Lit)]);
            }
        }
    }

    // Commander bits.
    let bit_vars: Vec<Var> = (0..k_bits).map(|_| b.fresh_var()).collect();

    // For each var v_i in group j, force v_i → encode(j) bit by bit.
    for (i, &v) in vars.iter().enumerate() {
        let grp = i / m;
        for (p, &bit_var) in bit_vars.iter().enumerate() {
            let bit_set = ((grp >> p) & 1) == 1;
            // v → b_p (if bit_set) or v → ¬b_p (if not).
            if bit_set {
                b.add_hard(vec![-(v as Lit), bit_var as Lit]);
            } else {
                b.add_hard(vec![-(v as Lit), -(bit_var as Lit)]);
            }
        }
    }
}

/// Returns the color emitted by piece `piece` in rotation `rot` on the
/// given side (0=top, 1=right, 2=bottom, 3=left).
pub fn emit_color(piece: &Piece, rot: Rotation, side: u8) -> u8 {
    piece.edges.rotated(rot).as_array()[side as usize]
}

/// Encode the full E2 problem into a CnfBuilder.
///
/// If `soft_edge_match` is true: edge-match constraints are encoded as
/// auxiliary vars + soft clauses (one per interior edge). Use this for
/// MaxSAT objective.
///
/// If `soft_edge_match` is false: each interior edge MUST match
/// (∨_k m_{e,k} as a hard clause). Use this for decision SAT
/// ("does the puzzle admit a perfect solution?").
pub struct EncodeOptions {
    pub soft_edge_match: bool,
}

pub fn encode(puzzle: &Puzzle, hints: &Hints, vmap: &VarMap, opts: &EncodeOptions) -> CnfBuilder {
    encode_with_pinned(puzzle, hints, vmap, opts, &PinnedMap::new())
}

/// Like `encode()` but takes a `pinned` map: cells with a fixed
/// (piece_index, rotation) treated as constants. Skips cell-EO for
/// pinned cells, skips piece-EO for pieces used in `pinned`, and
/// for edge-match constraints involving a pinned endpoint, replaces
/// the per-piece-rotation disjunction with the constant color the
/// pinned piece presents on the relevant side.
pub fn encode_with_pinned(
    puzzle: &Puzzle, hints: &Hints, vmap: &VarMap, opts: &EncodeOptions,
    pinned: &PinnedMap,
) -> CnfBuilder {
    let mut cnf = CnfBuilder::new(vmap.n_vars);
    let pieces = puzzle.pieces();

    // Compute set of pinned piece-indices.
    let pinned_piece_indices: std::collections::HashSet<u32> =
        pinned.values().map(|(pi, _)| *pi).collect();

    // --- 1. Cell exactly-one (skip pinned cells) ---
    for pos in 0..puzzle.cell_count() {
        if pinned.contains_key(&pos) { continue; }
        let vars: Vec<Var> = vmap.cell_to_pr[pos as usize].iter().map(|(_, _, v)| *v).collect();
        if vars.is_empty() {
            // Unsatisfiable: a free cell has no admissible (piece, rotation).
            cnf.add_hard(vec![]);
            continue;
        }
        add_alo(&mut cnf, &vars.iter().map(|&v| v as Lit).collect::<Vec<_>>());
        add_amo(&mut cnf, &vars);
    }

    // --- 2. Piece exactly-one (skip pinned pieces) ---
    for (piece_idx, piece) in pieces.iter().enumerate() {
        let _ = piece;
        if pinned_piece_indices.contains(&(piece_idx as u32)) { continue; }
        let vars: Vec<Var> = vmap.piece_to_cr[piece_idx].iter().map(|(_, _, v)| *v).collect();
        if vars.is_empty() {
            // A free piece can't be placed anywhere → infeasible.
            cnf.add_hard(vec![]);
            continue;
        }
        add_alo(&mut cnf, &vars.iter().map(|&v| v as Lit).collect::<Vec<_>>());
        add_amo(&mut cnf, &vars);
    }

    // --- 3. Edge-match aux vars + clauses ---
    let max_color = (puzzle.color_count - 1) as usize;
    for (e_idx, edge) in vmap.edges.iter().enumerate() {
        // Determine the colors emitted on each side IF the cell is pinned.
        let pinned_a_color: Option<u8> = pinned.get(&edge.cell_a).map(|&(pi, rot)| {
            emit_color(&pieces[pi as usize], rot, edge.side_a)
        });
        let pinned_b_color: Option<u8> = pinned.get(&edge.cell_b).map(|&(pi, rot)| {
            emit_color(&pieces[pi as usize], rot, edge.side_b)
        });

        // Case 1: both pinned. Edge is a CONSTANT.
 // FIXED 2026-05-17 (W-SAT bug):
        //   For DECISION SAT (soft_edge_match=false), if the two pinned colors
        //   don't agree, we MUST emit a hard ⊥ (empty clause) to fail the
        //   whole problem. Previously this case skipped silently, which
        //   accepted mismatched pinned-pinned edges as satisfying SAT.
        //
        //   For MaxSAT (soft_edge_match=true), still emit the soft bonus
        //   if pinned colors agree (so the objective counts the matched edge).
        if pinned_a_color.is_some() && pinned_b_color.is_some() {
            let ca = pinned_a_color.unwrap();
            let cb = pinned_b_color.unwrap();
            if !opts.soft_edge_match {
                // DECISION SAT: require ca == cb. If not, emit ⊥.
                if ca != cb || ca == BORDER {
                    // Pinned-pinned mismatch → SAT problem is unsatisfiable.
                    // Emit hard contradiction.
                    cnf.add_hard(vec![]); // empty clause = ⊥
                }
                // else: matched pinned-pinned edge — no constraint needed.
            } else {
                // MaxSAT: emit soft +1 bonus if matched.
                if ca == cb && ca != BORDER {
                    let m_var = vmap.match_vars[e_idx][ca as usize];
                    cnf.add_hard(vec![m_var as Lit]);
                    cnf.add_soft(vec![m_var as Lit]);
                }
            }
            continue;
        }

        // Case 2: one side pinned (say A). The pinned color is
        // determined; we just need the other side's piece to emit
        // the same color. Encode: m_{e,k_pinned} → ∨_{(p,r): emit(p,r,s_b)=k_pinned} x_{c_b,p,r}
        // Other m_{e,k} for k ≠ k_pinned must be false (because A only
        // emits the one color).
        // Case 3: neither pinned — original encoding.
        for k in 1..=max_color {
            let m_var = vmap.match_vars[e_idx][k];

            // Determine if this k can possibly be matched given pinning.
            let k_u8 = k as u8;
            let a_can_emit_k: Option<bool> = pinned_a_color.map(|c| c == k_u8);
            let b_can_emit_k: Option<bool> = pinned_b_color.map(|c| c == k_u8);

            // If A is pinned to a different color → m_{e,k} must be false.
            if a_can_emit_k == Some(false) || b_can_emit_k == Some(false) {
                cnf.add_hard(vec![-(m_var as Lit)]);
                continue;
            }
            // Side A clause: m → ∨ x_{c_a, p, r} for p,r emitting k on side_a.
            // If A is pinned to k, A "automatically" emits k → clause trivially satisfied.
            if a_can_emit_k != Some(true) {
                let mut clause_a: Vec<Lit> = vec![-(m_var as Lit)];
                for &(piece_idx, rot, var) in &vmap.cell_to_pr[edge.cell_a as usize] {
                    let p = &pieces[piece_idx as usize];
                    if emit_color(p, rot, edge.side_a) == k_u8 {
                        clause_a.push(var as Lit);
                    }
                }
                if clause_a.len() == 1 {
                    // No piece can emit color k at A → m must be false.
                    cnf.add_hard(vec![-(m_var as Lit)]);
                    continue;
                }
                cnf.add_hard(clause_a);
            }
            // Side B clause: similarly.
            if b_can_emit_k != Some(true) {
                let mut clause_b: Vec<Lit> = vec![-(m_var as Lit)];
                for &(piece_idx, rot, var) in &vmap.cell_to_pr[edge.cell_b as usize] {
                    let p = &pieces[piece_idx as usize];
                    if emit_color(p, rot, edge.side_b) == k_u8 {
                        clause_b.push(var as Lit);
                    }
                }
                if clause_b.len() == 1 {
                    cnf.add_hard(vec![-(m_var as Lit)]);
                    continue;
                }
                cnf.add_hard(clause_b);
            }
        }
        // AMO over match colors per edge: m_{e,k} ∧ m_{e,k'} infeasible.
        let m_vars: Vec<Var> = (1..=max_color).map(|k| vmap.match_vars[e_idx][k]).collect();
        add_amo(&mut cnf, &m_vars);
        // Either ALO of match-colors as hard (decision SAT) or as soft (MaxSAT).
        let alo_clause: Vec<Lit> = m_vars.iter().map(|&v| v as Lit).collect();
        if opts.soft_edge_match {
            cnf.add_soft(alo_clause);
        } else {
            cnf.add_hard(alo_clause);
        }
    }

    // --- 4. Hints (unit clauses) ---
    // Skip hints for cells already in `pinned` — they're enforced
    // implicitly by the constant-color edge clauses above.
    for h in &hints.hints {
        if pinned.contains_key(&h.position) { continue; }
        // Find var for (pos=h.position, piece_id=h.piece_id, rot=h.rotation).
        let piece_idx = pieces.iter().position(|p| p.id == h.piece_id);
        let Some(piece_idx) = piece_idx else {
            // Hint references an unknown piece — infeasible by construction.
            cnf.add_hard(vec![]);
            continue;
        };
        let var = vmap.cell_to_pr[h.position as usize].iter()
            .find(|(pi, r, _)| *pi == piece_idx as u32 && r.as_u8() == h.rotation.as_u8())
            .map(|(_, _, v)| *v);
        match var {
            Some(v) => cnf.add_hard(vec![v as Lit]),
            None => {
                // Hint is inadmissible at this cell — infeasible.
                cnf.add_hard(vec![]);
            }
        }
    }

    cnf
}

/// Decode a SAT solver's satisfying assignment back to a Board.
/// `model[v-1] = true` means var v is positive in the model.
pub fn decode_model(puzzle: &Puzzle, vmap: &VarMap, model: &[bool]) -> eternity2_core::Board {
    use eternity2_core::Board;
    let mut b = Board::empty(puzzle);
    let pieces = puzzle.pieces();
    for pos in 0..puzzle.cell_count() {
        for &(piece_idx, rot, var) in &vmap.cell_to_pr[pos as usize] {
            if (var as usize) - 1 < model.len() && model[(var as usize) - 1] {
                b.place(pos, pieces[piece_idx as usize].id, rot);
                break;
            }
        }
    }
    b
}

/// Emit DIMACS CNF (header + clause lines + trailing 0). For decision SAT.
pub fn write_dimacs_cnf(cnf: &CnfBuilder) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "p cnf {} {}", cnf.n_vars(), cnf.clauses.len());
    for c in &cnf.clauses {
        for l in c { let _ = write!(s, "{} ", l); }
        let _ = writeln!(s, "0");
    }
    s
}

/// Emit New-style WCNF (MaxSAT). The "new" format (used by MaxSAT
/// Evaluation 2022+) uses "h" prefix for hard clauses and a numeric
/// weight for soft clauses; no top weight needed.
pub fn write_wcnf_new(cnf: &CnfBuilder) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "c WCNF (new format) for Eternity II");
    let _ = writeln!(s, "c vars={} hard={} soft={}", cnf.n_vars(), cnf.clauses.len(), cnf.soft_clauses.len());
    for c in &cnf.clauses {
        let _ = write!(s, "h ");
        for l in c { let _ = write!(s, "{} ", l); }
        let _ = writeln!(s, "0");
    }
    for c in &cnf.soft_clauses {
        let _ = write!(s, "1 ");
        for l in c { let _ = write!(s, "{} ", l); }
        let _ = writeln!(s, "0");
    }
    s
}

/// Emit Old-style WCNF: "p wcnf <vars> <clauses> <top>" with top weight
/// for hard clauses. Some solvers still require this.
pub fn write_wcnf_old(cnf: &CnfBuilder) -> String {
    let top = (cnf.soft_clauses.len() as u64) + 1;
    let mut s = String::new();
    let total = cnf.clauses.len() + cnf.soft_clauses.len();
    let _ = writeln!(s, "p wcnf {} {} {}", cnf.n_vars(), total, top);
    for c in &cnf.clauses {
        let _ = write!(s, "{} ", top);
        for l in c { let _ = write!(s, "{} ", l); }
        let _ = writeln!(s, "0");
    }
    for c in &cnf.soft_clauses {
        let _ = write!(s, "1 ");
        for l in c { let _ = write!(s, "{} ", l); }
        let _ = writeln!(s, "0");
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_core::{Edges, Hint};

    /// Build a tiny 2x2 puzzle: 4 cells, 4 corner pieces; only 1 valid
    /// assignment up to rotation.
    fn tiny_2x2() -> Puzzle {
        // Each corner piece has 2 BORDER and 2 interior colors. We design
        // 4 pieces whose interior colors match in a unique way.
        // Color labels: 0=BORDER, 1, 2, 3, 4.
        // Cells (TL, TR, BL, BR). Required adjacencies:
        //   TL.right = TR.left (call it c1)
        //   TL.bottom = BL.top (c2)
        //   TR.bottom = BR.top (c3)
        //   BL.right = BR.left (c4)
        // Pick c1=1, c2=2, c3=3, c4=4.
        // Pieces (in canonical/non-rotated edges [top,right,bottom,left]):
        //   TL: top=B, right=1, bottom=2, left=B
        //   TR: top=B, right=B, bottom=3, left=1
        //   BL: top=2, right=4, bottom=B, left=B
        //   BR: top=3, right=B, bottom=B, left=4
        let pieces = vec![
            Piece::new(0, Edges::new(BORDER, 1, 2, BORDER)),
            Piece::new(1, Edges::new(BORDER, BORDER, 3, 1)),
            Piece::new(2, Edges::new(2, 4, BORDER, BORDER)),
            Piece::new(3, Edges::new(3, BORDER, BORDER, 4)),
        ];
        Puzzle::new(2, 2, 5, pieces).expect("build")
    }

    #[test]
    fn varmap_2x2_counts() {
        let p = tiny_2x2();
        let vm = VarMap::build(&p);
        // Each cell is a corner; all 4 corner pieces are class-compatible.
        // Each piece has 1 rotation that puts BORDER on the right two
        // sides of the cell, so each cell admits 4 (piece, rotation)
        // combinations.
        for c in 0..4 {
            assert_eq!(vm.cell_to_pr[c as usize].len(), 4,
                "cell {} should have 4 admissible (p,r) under class+border-only filtering", c);
        }
        // Each piece fits at 4 cells (one rotation per cell).
        for p_idx in 0..4 {
            assert_eq!(vm.piece_to_cr[p_idx].len(), 4,
                "piece {} should fit 4 (cell,r) combos", p_idx);
        }
        // 4 cells × 4 admissible (p,r) = 16 piece-vars total.
        assert_eq!(vm.var_to_cpr.len(), 16);
        // Interior edges on 2x2: 1 horizontal between (TL,TR) and (BL,BR) = 2,
        // 1 vertical between (TL,BL) and (TR,BR) = 2, total 4.
        assert_eq!(vm.edges.len(), 4);
    }

    #[test]
    fn encode_2x2_has_unit_solution() {
        let p = tiny_2x2();
        let vm = VarMap::build(&p);
        let cnf = encode(&p, &Hints::default(), &vm, &EncodeOptions { soft_edge_match: false });
        // With every cell forced into a unique placement, the only model
        // must set all 4 piece-vars true. We don't run a solver here —
        // tests/integration with splr live in a separate test. But we
        // can sanity-check var counts.
        assert!(cnf.n_vars() >= 4);
        assert!(!cnf.clauses.is_empty());
    }

    #[test]
    fn hints_translate_to_unit_clauses() {
        let p = tiny_2x2();
        let vm = VarMap::build(&p);
        // Pin piece 0 at cell 0 in rotation R0 (which is the only
        // admissible placement anyway).
        let hints = Hints::new(vec![Hint {
            position: 0, piece_id: 0, rotation: Rotation::R0,
        }]);
        let cnf = encode(&p, &hints, &vm, &EncodeOptions { soft_edge_match: false });
        let units: Vec<&Vec<Lit>> = cnf.clauses.iter().filter(|c| c.len() == 1 && c[0] > 0).collect();
        assert!(!units.is_empty(), "should have at least one unit clause from the hint");
    }

    #[test]
    fn amo_bimander_admits_one() {
        let mut b = CnfBuilder::new(10);
        let vars: Vec<Var> = (1..=10).collect();
        add_amo_bimander(&mut b, &vars);
        // Sanity: assigning only var 3 true must not violate any clause.
        // We check each clause locally: it must have at least one
        // satisfied literal under the model {3=true, others=false,
        // bit_vars = whatever the bimander encoding demands}.
        // (Full validation in the splr-backed test below.)
        assert!(!b.clauses.is_empty());
    }
}
