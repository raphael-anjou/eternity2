// One small grid per path kind, with an arrow from each cell to the next,
// so "fill order" stops being abstract.

import { useMemo } from "react";
import { useEngine } from "@/engine/useEngine";
import { getPath } from "@/engine";

const KINDS = [
  "row-major",
  "column-major",
  "snake",
  "spiral-in",
  "spiral-out",
  "diagonal",
  "border-first",
  "double-snake",
];
const N = 5;
const CELL = 40;

function Diagram({ kind }: { kind: string }) {
  const path = useMemo(() => Array.from(getPath(kind, N, N, 0)), [kind]);
  const cx = (c: number) => (c % N) * CELL + CELL / 2;
  const cy = (c: number) => Math.floor(c / N) * CELL + CELL / 2;
  return (
    <div className="flex w-full flex-col items-center gap-1">
      <svg
        viewBox={`0 0 ${N * CELL} ${N * CELL}`}
        className="aspect-square w-full max-w-[200px] rounded-md border bg-background"
      >
        <defs>
          <marker
            id={`arr-${kind}`}
            viewBox="0 0 10 10"
            refX="8"
            refY="5"
            markerWidth="5"
            markerHeight="5"
            orient="auto-start-reverse"
          >
            <path d="M0,0 L10,5 L0,10 z" fill="currentColor" />
          </marker>
        </defs>
        {Array.from({ length: N + 1 }, (_, i) => (
          <g key={i} className="stroke-border">
            <line x1={i * CELL} y1={0} x2={i * CELL} y2={N * CELL} strokeWidth={1} />
            <line x1={0} y1={i * CELL} x2={N * CELL} y2={i * CELL} strokeWidth={1} />
          </g>
        ))}
        {path.slice(0, -1).map((c, i) => {
          const next = path[i + 1];
          const t = i / (path.length - 2);
          const hue = 0 + t * 120;
          // Shorten the segment a bit so arrowheads stay inside cells.
          const x1 = cx(c), y1 = cy(c), x2 = cx(next), y2 = cy(next);
          const dx = x2 - x1, dy = y2 - y1;
          const len = Math.hypot(dx, dy) || 1;
          const trim = Math.min(8, len / 4);
          return (
            <line
              key={i}
              x1={x1 + (dx / len) * trim}
              y1={y1 + (dy / len) * trim}
              x2={x2 - (dx / len) * trim}
              y2={y2 - (dy / len) * trim}
              stroke={`hsl(${hue} 75% 45%)`}
              strokeWidth={2.5}
              markerEnd={`url(#arr-${kind})`}
              color={`hsl(${hue} 75% 45%)`}
            />
          );
        })}
        <circle cx={cx(path[0])} cy={cy(path[0])} r={5} fill="hsl(0 75% 45%)" />
      </svg>
      <span className="text-xs font-medium text-muted-foreground">{kind}</span>
    </div>
  );
}

export function PathDiagrams() {
  const engineReady = useEngine();
  if (!engineReady) return null;
  return (
    <div className="mx-auto grid max-w-3xl grid-cols-2 justify-items-center gap-4 sm:grid-cols-4">
      {KINDS.map((k) => (
        <Diagram key={k} kind={k} />
      ))}
    </div>
  );
}
