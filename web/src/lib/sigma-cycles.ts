// Real σ-cycle machinery for the basin-hopping demo. Given two complete boards
// of the same puzzle (each cell → pieceId*4+rotation), σ is the permutation of
// board positions that carries board A's piece arrangement to board B's: for the
// piece sitting at cell i in A, σ(i) is the cell that same piece occupies in B.
// Decomposing σ into disjoint cycles gives the "loops" the research describes —
// apply a whole loop and you've moved every piece in it to the next one's spot.
//
// This is computed live from two real engine solutions; nothing here is faked.

/** Extract pieceId (dropping rotation) per cell from an engine board. */
function pieceIds(board: Int32Array): number[] {
  return Array.from(board, (v) => (v >= 0 ? v >> 2 : -1));
}

/**
 * σ as a position permutation A→B. sigma[i] = the cell in B holding the piece
 * that sits at cell i in A. Both boards must be complete and share a piece set.
 */
export function sigmaPermutation(boardA: Int32Array, boardB: Int32Array): number[] {
  const a = pieceIds(boardA);
  const b = pieceIds(boardB);
  const posInB = new Map<number, number>();
  b.forEach((pid, pos) => posInB.set(pid, pos));
  return a.map((pid) => posInB.get(pid) ?? -1);
}

/** Disjoint non-trivial cycles of a permutation (fixed points dropped). */
export function decomposeCycles(sigma: number[]): number[][] {
  const seen = new Array(sigma.length).fill(false);
  const cycles: number[][] = [];
  for (let i = 0; i < sigma.length; i++) {
    const si = sigma[i] ?? -1;
    if (seen[i] || si === i || si < 0) {
      seen[i] = true;
      continue;
    }
    const cycle: number[] = [];
    let j = i;
    while (!seen[j]) {
      seen[j] = true;
      cycle.push(j);
      j = sigma[j] ?? j;
    }
    if (cycle.length > 1) cycles.push(cycle);
  }
  cycles.sort((x, y) => y.length - x.length);
  return cycles;
}

/**
 * Apply a chosen set of cycles to board A, moving each piece one step along its
 * cycle toward board B. `fraction` in [0,1] applies only the first `fraction` of
 * each selected cycle's edges — the "partial application" the research scores.
 * Returns a new engine board (cell → pieceId*4+rot). Cells not covered by an
 * applied edge keep board A's piece; cells that receive a moved piece take it
 * with board B's rotation (so a fully-applied cycle reproduces B exactly there).
 */
export function applyCycles(
  boardA: Int32Array,
  boardB: Int32Array,
  cycles: number[][],
  selected: Set<number>,
  fraction: number,
): Int32Array {
  const out = Int32Array.from(boardA);
  // Map pieceId → its placement value in B (to copy the correct rotation).
  const bPlacementByPiece = new Map<number, number>();
  boardB.forEach((v) => {
    if (v >= 0) bPlacementByPiece.set(v >> 2, v);
  });

  cycles.forEach((cycle, ci) => {
    if (!selected.has(ci)) return;
    const edges = Math.max(0, Math.min(cycle.length, Math.round(cycle.length * fraction)));
    // Moving piece at cycle[k] into cycle's next slot means: the destination
    // cell cycle[k] receives the piece that A had at the previous cell. We model
    // "apply the loop" as: for the first `edges` positions, set the cell to the
    // piece B wants there (its B placement). That is exactly one step around.
    for (let k = 0; k < edges; k++) {
      const cell = cycle[k];
      if (cell === undefined) continue;
      const bVal = boardB[cell];
      if (bVal === undefined || bVal < 0) continue;
      const placement = bPlacementByPiece.get(bVal >> 2);
      if (placement !== undefined) out[cell] = placement;
    }
  });
  return out;
}
