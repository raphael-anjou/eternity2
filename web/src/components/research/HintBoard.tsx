import { maxScore, pinnedSeamFloor } from "@/lib/hint-layouts";

// The one reusable board primitive for the Hint Study. Renders an n×n grid with
// a set of hinted cells highlighted, and optionally draws the "banked" seams
// (interior edges with both endpoints pinned — the free-score floor). Every
// layout illustration in the study is built from this, so the visual language is
// identical everywhere: grey = empty, amber = hint, a thin line = a banked seam.

export type HintBoardProps = {
  n: number;
  /** hinted cells, row-major indices */
  cells: number[];
  /** draw the banked (both-endpoints-pinned) seams as connecting bars */
  showFloor?: boolean;
  /** px per cell in the viewBox (visual only; SVG scales to container) */
  cell?: number;
  className?: string;
  ariaLabel?: string;
};

export function HintBoard({
  n,
  cells,
  showFloor = false,
  cell = 16,
  className,
  ariaLabel,
}: HintBoardProps) {
  const hinted = new Set(cells);
  const size = n * cell;

  // Banked seams: pairs of adjacent pinned cells. Drawn as a short bar bridging
  // the two cell centres, so the reader sees the free correct edges pile up.
  const seams: Array<{ x1: number; y1: number; x2: number; y2: number }> = [];
  if (showFloor) {
    for (let r = 0; r < n; r++)
      for (let c = 0; c < n; c++) {
        const i = r * n + c;
        const cx = c * cell + cell / 2;
        const cy = r * cell + cell / 2;
        if (c < n - 1 && hinted.has(i) && hinted.has(i + 1))
          seams.push({ x1: cx, y1: cy, x2: cx + cell, y2: cy });
        if (r < n - 1 && hinted.has(i) && hinted.has(i + n))
          seams.push({ x1: cx, y1: cy, x2: cx, y2: cy + cell });
      }
  }

  return (
    <svg
      viewBox={`0 0 ${size} ${size}`}
      className={className ?? "w-full rounded-lg border bg-card"}
      role="img"
      aria-label={ariaLabel ?? `${cells.length} hints on a ${n}×${n} board`}
    >
      {Array.from({ length: n * n }, (_, i) => {
        const r = Math.floor(i / n);
        const c = i % n;
        return (
          <rect
            key={i}
            x={c * cell + 1}
            y={r * cell + 1}
            width={cell - 2}
            height={cell - 2}
            rx={2}
            className={hinted.has(i) ? "fill-amber-400" : "fill-muted"}
          />
        );
      })}
      {seams.map((s, i) => (
        <line
          key={`seam-${i}`}
          x1={s.x1}
          y1={s.y1}
          x2={s.x2}
          y2={s.y2}
          className="stroke-emerald-500"
          strokeWidth={cell * 0.28}
          strokeLinecap="round"
        />
      ))}
    </svg>
  );
}

/** A labelled board tile with its hint count and (optionally) its free-score
 * floor — the atom the layout gallery and the ladder are built from. */
export function HintBoardTile({
  n,
  cells,
  title,
  subtitle,
  showFloor = false,
}: {
  n: number;
  cells: number[];
  title: string;
  subtitle?: string;
  showFloor?: boolean;
}) {
  const floor = pinnedSeamFloor(n, cells);
  const max = maxScore(n);
  return (
    <figure className="space-y-2">
      <HintBoard n={n} cells={cells} showFloor={showFloor} ariaLabel={title} />
      <figcaption className="text-center text-xs">
        <span className="font-semibold">{title}</span>
        <br />
        <span className="text-muted-foreground">
          {cells.length} {cells.length === 1 ? "hint" : "hints"}
          {showFloor && floor > 0 ? ` · ${floor}/${max} free seams` : ""}
          {subtitle ? ` · ${subtitle}` : ""}
        </span>
      </figcaption>
    </figure>
  );
}
