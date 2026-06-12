// A single standalone piece (used in trays, motif legends, hint cards).

import { DIRECTION_ROTATION } from "@/lib/motifs";
import type { Edges } from "@/lib/bucas";

export function PieceSvg({
  edges,
  size = 64,
  className,
  selected,
}: {
  edges: Edges;
  size?: number;
  className?: string;
  selected?: boolean;
}) {
  return (
    <svg
      viewBox="-130 -130 260 260"
      width={size}
      height={size}
      className={className}
      style={{ display: "block" }}
    >
      {edges.map((c, dir) => (
        <use key={dir} href={`#e2m-${c}`} transform={`rotate(${DIRECTION_ROTATION[dir]})`} />
      ))}
      <rect
        x={-128}
        y={-128}
        width={256}
        height={256}
        fill="none"
        stroke={selected ? "#fbbf24" : "#000"}
        strokeWidth={selected ? 14 : 4}
        rx={selected ? 6 : 0}
      />
    </svg>
  );
}
