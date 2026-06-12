// A single motif shown the way it really exists on a piece: as one triangle
// (here pointing up). Pieces are made of four of these.

export function MotifSwatch({ color, width = 56 }: { color: number; width?: number }) {
  return (
    <svg viewBox="-128 -128 256 128" width={width} height={width / 2} aria-hidden>
      <use href={`#e2m-${color}`} transform="rotate(90)" />
    </svg>
  );
}
