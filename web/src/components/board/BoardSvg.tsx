// SVG board renderer, bucas-compatible: 256-unit cells, motif <use> quads.
// Cells-driven so it can show engine boards, imported bucas boards, or
// hand-played boards alike.

import { memo } from "react";
import type { ReactNode } from "react";
import { DIRECTION_ROTATION } from "@/lib/motifs";
import type { Edges } from "@/lib/bucas";

export const CELL = 256;

export interface BoardSvgProps {
  width: number;
  height: number;
  /** Edge colors per cell (URDL), null = empty. */
  cells: (Edges | null)[];
  /** [pos, URDL direction] half-edges to mark as conflicts. */
  conflicts?: [number, number][];
  /** Cells to outline (e.g. official hints or the last placement). */
  highlight?: number[];
  /** 1-based numbers to print on pieces. */
  pieceNumbers?: (number | null)[];
  onCellClick?: (pos: number) => void;
  /** HTML5 drop target per cell; receives the dataTransfer text payload. */
  onCellDrop?: (pos: number, data: string) => void;
  /** Extra SVG painted above the pieces (path arrows, ranks…). */
  overlay?: ReactNode;
  className?: string;
}

function PieceUses({ pos, x, y, edges }: { pos: number; x: number; y: number; edges: Edges }) {
  const cx = x * CELL + CELL / 2;
  const cy = y * CELL + CELL / 2;
  return (
    <g data-pos={pos}>
      {edges.map((c, dir) => (
        <use
          key={dir}
          href={`#e2m-${c}`}
          transform={`translate(${cx} ${cy}) rotate(${DIRECTION_ROTATION[dir]})`}
        />
      ))}
      {/* subtle bevel */}
      <path
        d={`M${x * CELL + 5},${y * CELL + CELL - 5} V${y * CELL + 5} H${x * CELL + CELL - 5}`}
        fill="none"
        stroke="#ffffff"
        strokeWidth={5}
        opacity={0.25}
      />
      <path
        d={`M${x * CELL + CELL - 4},${y * CELL + 4} V${y * CELL + CELL - 4} H${x * CELL + 4}`}
        fill="none"
        stroke="#000000"
        strokeWidth={5}
        opacity={0.3}
      />
    </g>
  );
}

export const BoardSvg = memo(function BoardSvg({
  width,
  height,
  cells,
  conflicts,
  highlight,
  pieceNumbers,
  onCellClick,
  onCellDrop,
  overlay,
  className,
}: BoardSvgProps) {
  const W = width * CELL;
  const H = height * CELL;
  return (
    <svg
      viewBox={`-6 -6 ${W + 12} ${H + 12}`}
      className={className}
      style={{ display: "block", width: "100%", height: "auto" }}
    >
      <rect x={-6} y={-6} width={W + 12} height={H + 12} rx={10} fill="#1c1917" />
      {/* empty-cell backdrop */}
      {cells.map((cell, pos) =>
        cell ? null : (
          <rect
            key={pos}
            x={(pos % width) * CELL}
            y={Math.floor(pos / width) * CELL}
            width={CELL}
            height={CELL}
            fill={(pos + Math.floor(pos / width)) % 2 === 0 ? "#292524" : "#33302c"}
          />
        ),
      )}
      {cells.map((cell, pos) =>
        cell ? (
          <PieceUses key={pos} pos={pos} x={pos % width} y={Math.floor(pos / width)} edges={cell} />
        ) : null,
      )}
      {/* grid */}
      {Array.from({ length: width + 1 }, (_, i) => (
        <line key={`v${i}`} x1={i * CELL} y1={0} x2={i * CELL} y2={H} stroke="#000" strokeWidth={3} />
      ))}
      {Array.from({ length: height + 1 }, (_, i) => (
        <line key={`h${i}`} x1={0} y1={i * CELL} x2={W} y2={i * CELL} stroke="#000" strokeWidth={3} />
      ))}
      {conflicts?.map(([pos, dir], i) => (
        <use
          key={i}
          href="#e2m-conflict"
          transform={`translate(${(pos % width) * CELL + CELL / 2} ${Math.floor(pos / width) * CELL + CELL / 2}) rotate(${DIRECTION_ROTATION[dir]})`}
        />
      ))}
      {highlight?.map((pos) => (
        <rect
          key={pos}
          x={(pos % width) * CELL + 6}
          y={Math.floor(pos / width) * CELL + 6}
          width={CELL - 12}
          height={CELL - 12}
          fill="none"
          stroke="#ff1133"
          strokeWidth={16}
          rx={8}
        />
      ))}
      {pieceNumbers?.map((n, pos) =>
        n ? (
          <text
            key={pos}
            x={(pos % width) * CELL + CELL / 2}
            y={Math.floor(pos / width) * CELL + CELL / 2 + 24}
            textAnchor="middle"
            fontSize={72}
            fontWeight={700}
            fill="#fff"
            stroke="#000"
            strokeWidth={5}
            paintOrder="stroke"
            style={{ pointerEvents: "none" }}
          >
            {n}
          </text>
        ) : null,
      )}
      {overlay}
      {(onCellClick || onCellDrop) &&
        cells.map((_, pos) => (
          <rect
            key={pos}
            x={(pos % width) * CELL}
            y={Math.floor(pos / width) * CELL}
            width={CELL}
            height={CELL}
            fill="transparent"
            style={{ cursor: "pointer" }}
            onClick={onCellClick ? () => onCellClick(pos) : undefined}
            onDragOver={onCellDrop ? (e) => e.preventDefault() : undefined}
            onDrop={
              onCellDrop
                ? (e) => {
                    e.preventDefault();
                    onCellDrop(pos, e.dataTransfer.getData("text/plain"));
                  }
                : undefined
            }
          />
        ))}
    </svg>
  );
});
