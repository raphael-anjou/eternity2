// The 22 Eternity II edge motifs + the grey border, as drawn by the
// e2.bucas.name viewer. Geometry: a cell is 256×256 units; each motif is
// defined in a frame where (0,0) is the cell center and the motif fills the
// LEFT-edge triangle; it is then rotated about the cell center to face
// up/right/down. Motif data (C) 2019 Jef Bucas, GPL-3.0, from
// https://github.com/jfbucas/eternityII-viewer (init.js), reused with
// attribution under the same license.

export interface Motif {
  /** Background triangle fill. */
  bg: string;
  /** Decorative path (drawn clipped to the triangle), or null for plain grey. */
  path: string | null;
  pathFill: string | null;
  pathStroke: string | null;
}

/** The grey border motif (color id 0), also the fallback for out-of-range colors. */
export const BORDER_MOTIF: Motif = { bg: "#9a9a9a", path: null, pathFill: null, pathStroke: null };

// Index = color id = bucas letter - 'a'. 0 is the grey border.
export const MOTIFS: Motif[] = [
  BORDER_MOTIF,
  { bg: "#f88512", path: "m-128,-80 h 16 a64,64 30 0,0 64,64 v 32 a64,64 30 0,0 -64,64 h -16", pathFill: "#80d5f8", pathStroke: "#9ea599" },
  { bg: "#155c8c", path: "m-128,-64 a32,32 30 0,1 32,32 a32,32 30 0,1 0,64 a32,32 30 0,1 -32,32 v -32 a32,32 30 0,0 0,-64", pathFill: "#fef102", pathStroke: "#7c8c48" },
  { bg: "#ec35a0", path: "m-128,-80 h 16 a64,64 30 0,0 64,64 v 32 a64,64 30 0,0 -64,64 h -16 v -48 a32,32 30 1,0 0,-64", pathFill: "#81d1f0", pathStroke: "#af4f8d" },
  { bg: "#33b441", path: "m-128,-64 h 32 l 32,32 v  64 l -32,32 h -32 v -16 a48,48 30 1,0 0,-96", pathFill: "#265e93", pathStroke: "#3b6c8c" },
  { bg: "#831b43", path: "m-128,0 m-8,-40 a16,16 30 1,1 16,0 l 32,32 a16,16 30 1,1 0,16 l -32,32 a16,16 30 1,1 -16,0 l 8,-16 l 24,-24 l -24,-24", pathFill: "#f48614", pathStroke: "#b76742" },
  { bg: "#ee3ea8", path: "m-8,0 m-128,0 v -8 v -32 a16,16 30 1,1 16,0 v 32 h 32 a16,16 30 1,1 0,16 h -32 v 32 a16,16 30 1,1 -16,0 v -32", pathFill: "#f0ed24", pathStroke: "#d7ad60" },
  { bg: "#864ba3", path: "m-128,-96 l 32,32 l -8,40 l 40,-8 l 32,32 l -32,32 l -40,-8 l 8,40 l -32,32", pathFill: "#b6e8f9", pathStroke: "#8d8db2" },
  { bg: "#eded25", path: "m-128,-32 l 32,-32 v 48 l 32,16 l -32,16 v 48 l -32,-32", pathFill: "#43aee6", pathStroke: "#92ad65" },
  { bg: "#854aa3", path: "m-128,-32 v -32 a64,64 30 0,1 0,128 v -40 l 24, 24 l 24,-24 l -24,-24 l 24,-24 l -24,-24 l -24,24", pathFill: "#eced25", pathStroke: "#c9bb4b" },
  { bg: "#32b459", path: "m-128,0 m-8,0 v -8 v -32 a16,16 30 1,1 16,0 v 32 h 32 a16,16 30 1,1 0,16 h -32 v 32 a16,16 30 1,1 -16,0 v -32", pathFill: "#ee3ea8", pathStroke: "#698367" },
  { bg: "#ac3c6b", path: "m-128,-32 v -32 a64,64 30 0,1 0,128 v -40 l 24, 24 l 24,-24 l -24,-24 l 24,-24 l -24,-24 l -24,24", pathFill: "#2bb35a", pathStroke: "#76615e" },
  { bg: "#2bb35a", path: "m-128,-96 l 32,32 l -8,40 l 40,-8 l 32,32 l -32,32 l -40,-8 l 8,40 l -32,32", pathFill: "#f4892a", pathStroke: "#778e3d" },
  { bg: "#ac3c6b", path: "m-128,-32 l 32,-32 v 48 l 32,16 l -32,16 v 48 l -32,-32", pathFill: "#eced29", pathStroke: "#944b53" },
  { bg: "#5cc9f2", path: "m-128,-96 l 32,32 l -8,8 l 32,32 l 8,-8 l 32,32 l -32,32 l -8,-8 l -32,32 l 8,8 l -32,32", pathFill: "#ee3fa8", pathStroke: "#8682bc" },
  { bg: "#eded25", path: "m-128,-96 l 96,96 l -96, 96 v -32 l 64,-64 l -64,-64", pathFill: "#2bb356", pathStroke: "#cbcd2a" },
  { bg: "#5cc9f2", path: "m-128,-96 l 32,32 l -8,40 l 40,-8 l 32,32 l -32,32 l -40,-8 l 8,40 l -32,32", pathFill: "#ee3fa8", pathStroke: "#8682bc" },
  { bg: "#fdf103", path: "m-128,-96 l 32,32 l -8,8 l 32,32 l 8,-8 l 32,32 l -32,32 l -8,-8 l -32,32 l 8,8 l -32,32", pathFill: "#145c8c", pathStroke: "#8e9743" },
  { bg: "#f88826", path: "m-128,-32 l 32,-32 v 48 l 32,16 l -32,16 v 48 l -32,-32", pathFill: "#864ca4", pathStroke: "#b56844" },
  { bg: "#26638e", path: "m-8,0 m-128,0 v -8 v -32 a16,16 30 1,1 16,0 v 32 h 32 a16,16 30 1,1 0,16 h -32 v 32 a16,16 30 1,1 -16,0 v -32", pathFill: "#f38622", pathStroke: "#c1732d" },
  { bg: "#265e92", path: "m-128,-96 l 96,96 l -96, 96 v -32 l 64,-64 l -64,-64", pathFill: "#75cff2", pathStroke: "#4585ad" },
  { bg: "#ed3da5", path: "m-128,-96 l 32,32 l -8,8 l 32,32 l 8,-8 l 32,32 l -32,32 l -8,-8 l -32,32 l 8,8 l -32,32", pathFill: "#fdf102", pathStroke: "#edc524" },
  { bg: "#145c8c", path: "m-128,-32 v -32 a64,64 30 0,1 0,128 v -40 l 24, 24 l 24,-24 l -24,-24 l 24,-24 l -24,-24 l -24,24", pathFill: "#eced25", pathStroke: "#a95397" },
];

/** Motif for a color, clamped into range. Border (0) is the fallback for any
 *  out-of-range color, so callers never deal with `undefined` (the array
 *  index would otherwise be `Motif | undefined` under noUncheckedIndexedAccess). */
export function motifFor(color: number): Motif {
  return MOTIFS[color] ?? BORDER_MOTIF;
}

/** Rotation (degrees, about the cell center) that points the left-edge motif
 *  at each URDL direction, bucas convention. */
export const DIRECTION_ROTATION = [90, 180, -90, 0] as const;

/** Rotation for a URDL direction (0..3), clamped. Total accessor over the
 *  4-tuple so indexed reads don't widen to `number | undefined`. */
export function directionRotation(dir: number): number {
  switch (dir & 3) {
    case 1:
      return DIRECTION_ROTATION[1];
    case 2:
      return DIRECTION_ROTATION[2];
    case 3:
      return DIRECTION_ROTATION[3];
    default:
      return DIRECTION_ROTATION[0];
  }
}

export function colorToLetter(color: number): string {
  return String.fromCharCode(97 + color);
}

export function letterToColor(letter: string): number {
  return letter.charCodeAt(0) - 97;
}
