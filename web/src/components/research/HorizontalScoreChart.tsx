import type { ReactNode } from "react";
import {
  Bar,
  BarChart,
  Cell,
  LabelList,
  ReferenceLine,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { useIsClient } from "@/lib/utils";

// One horizontal bar chart, reused by every leaderboard on the site (the DFS
// study's score and depth panels, the community unpinned grid, and any future
// score-by-row chart). It exists because those charts were copies of the same
// BarChart + XAxis/YAxis/Tooltip/ReferenceLine/Bar boilerplate, which meant a
// layout fix (like the clipped top reference-line label) had to be made in each
// place. Now there is one place.
//
// It is deliberately small and unopinionated: the caller supplies the rows, the
// value/label keys, a per-row colour, optional reference lines, and an optional
// custom tooltip and bar-label. It renders nothing about families or units; that
// stays with the caller so this component has no domain knowledge.

export type RefLine = {
  x: number;
  color?: string; // defaults to currentColor (muted)
  label?: string;
  labelPosition?: "top" | "insideTopRight" | "insideBottomRight";
  angle?: number;
  dashed?: boolean;
  opacity?: number;
};

export type ScoreChartRow = { name: string } & Record<string, unknown>;

export function HorizontalScoreChart<Row extends ScoreChartRow>({
  rows,
  valueKey,
  categoryKey = "display",
  domainMax,
  colorOf,
  fillOpacityOf,
  referenceLines = [],
  height = 520,
  categoryWidth = 150,
  busyLabel = "Drawing…",
  tooltip,
  barLabel,
}: {
  rows: Row[];
  // Data keys are plain strings: recharts' dataKey type does not accept a
  // `keyof Row` union cleanly, and the caller passes a literal field name.
  valueKey: string;
  categoryKey?: string;
  domainMax: number;
  colorOf: (row: Row) => string;
  fillOpacityOf?: (row: Row) => number;
  referenceLines?: RefLine[];
  height?: number;
  categoryWidth?: number;
  busyLabel?: string;
  tooltip?: (row: Row) => ReactNode;
  // A custom right-side label per bar; falls back to the raw value.
  barLabel?: (row: Row) => string;
}) {
  const isClient = useIsClient();

  return (
    <div className="w-full" style={{ height }}>
      {isClient ? (
        <ResponsiveContainer width="100%" height="100%">
          <BarChart
            accessibilityLayer
            layout="vertical"
            data={rows}
            // Top margin is generous on purpose: a reference line with a `top`
            // label (e.g. "record 464") draws its text above the plot, and a
            // tight top margin clips it. This was the bug that motivated the
            // shared component.
            margin={{ top: 28, right: 64, bottom: 8, left: 8 }}
          >
            <XAxis
              type="number"
              domain={[0, domainMax]}
              tick={{ fontSize: 11, fill: "currentColor" }}
              className="text-muted-foreground"
            />
            <YAxis
              type="category"
              dataKey={categoryKey}
              width={categoryWidth}
              tick={{ fontSize: 11, fill: "currentColor" }}
              className="text-muted-foreground"
            />
            {tooltip && (
              <Tooltip
                cursor={{ fill: "currentColor", opacity: 0.06 }}
                content={({ active, payload }) => {
                  if (!active || !payload?.length) return null;
                  const row = payload[0]?.payload as Row | undefined;
                  if (!row) return null;
                  return <>{tooltip(row)}</>;
                }}
              />
            )}
            {referenceLines.map((r, i) => (
              <ReferenceLine
                key={`ref-${i}-${r.x}`}
                x={r.x}
                stroke={r.color ?? "currentColor"}
                {...(r.color ? {} : { className: "text-muted-foreground" })}
                {...(r.dashed === false ? {} : { strokeDasharray: "4 4" })}
                strokeOpacity={r.opacity ?? 1}
                {...(r.label
                  ? {
                      label: {
                        value: r.label,
                        position: r.labelPosition ?? "top",
                        fontSize: r.labelPosition && r.labelPosition !== "top" ? 9 : 10,
                        fill: r.color ?? "currentColor",
                        ...(r.angle != null ? { angle: r.angle } : {}),
                      },
                    }
                  : {})}
              />
            ))}
            <Bar dataKey={valueKey} radius={[0, 4, 4, 0]} isAnimationActive={false}>
              {rows.map((row) => (
                <Cell
                  key={row.name}
                  fill={colorOf(row)}
                  fillOpacity={fillOpacityOf ? fillOpacityOf(row) : 1}
                />
              ))}
              <LabelList
                dataKey={valueKey}
                position="right"
                fontSize={11}
                className="fill-foreground"
                content={(props) => {
                  const { x, y, width, height: h, index } = props as {
                    x: number;
                    y: number;
                    width: number;
                    height: number;
                    index: number;
                  };
                  const row = rows[index];
                  if (!row) return null;
                  const raw = row[valueKey];
                  const text = barLabel
                    ? barLabel(row)
                    : typeof raw === "number" || typeof raw === "string"
                      ? String(raw)
                      : "";
                  return (
                    <text
                      x={x + width + 6}
                      y={y + h / 2}
                      fontSize={11}
                      dominantBaseline="central"
                      className="fill-foreground"
                    >
                      {text}
                    </text>
                  );
                }}
              />
            </Bar>
          </BarChart>
        </ResponsiveContainer>
      ) : (
        <div className="flex h-full items-center justify-center text-sm text-muted-foreground">
          {busyLabel}
        </div>
      )}
    </div>
  );
}
