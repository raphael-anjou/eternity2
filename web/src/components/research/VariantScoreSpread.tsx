import { useMemo } from "react";
import { cn } from "@/lib/utils";

// Per-variant score spread: one row per variant, every instance's raw score
// drawn as a tick on a shared score axis, with the mean marked. It exists to
// make the study's run-to-run variance visible instead of collapsing it into a
// single mean bar, so a reader can see directly that (for example) the one-break
// variant is tightly clustered while the two-break variant ranges much wider,
// and that the propagator variants overlap so heavily they are statistically
// indistinguishable. The mean of the ticks is exactly the mean bar above, so the
// two views never disagree.
//
// Rendered as plain SVG (not recharts), like the CollapseBar on the DFS
// leaderboard: it is deterministic, prerenders into the static HTML with no
// client-only busy state, and carries no layout shift. Colour is the FAMILY hue
// each leaderboard already maps (identity, never rank); the family is always
// named in the row label and the caller's legend, so colour is never the only
// signal. Every reader-facing string is supplied by the caller (`labels`) so the
// trilingual copy lives in each leaderboard's existing T dict.

export type SpreadRow = {
  name: string;
  display: string;
  family: string;
  mean?: number | null;
  best?: number | null;
  worst?: number | null;
  scores?: number[] | null;
};

export type SpreadLabels = {
  /** Axis caption, e.g. "score (matched edges, out of 480)". */
  axis: string;
  /** Tooltip / a11y prefix for the mean marker, e.g. "mean". */
  mean: string;
  /** Word for the low..high range in the a11y summary, e.g. "range". */
  range: string;
  /** Unit-of-count word for the a11y summary, e.g. "instances". */
  instances: string;
};

// A two-option segmented toggle, used by both study leaderboards to switch the
// section-1 view between the aggregate mean bars and this per-instance spread.
// Kept here so the two sibling leaderboards share one control instead of each
// rebuilding it. Labels are supplied by the caller (trilingual T dicts).
export function ViewToggle({
  value,
  onChange,
  barsLabel,
  spreadLabel,
}: {
  value: "bars" | "spread";
  onChange: (v: "bars" | "spread") => void;
  barsLabel: string;
  spreadLabel: string;
}) {
  const opts: { key: "bars" | "spread"; label: string }[] = [
    { key: "bars", label: barsLabel },
    { key: "spread", label: spreadLabel },
  ];
  return (
    <div className="inline-flex shrink-0 rounded-md border p-0.5 text-xs" role="group">
      {opts.map((o) => (
        <button
          key={o.key}
          type="button"
          onClick={() => onChange(o.key)}
          aria-pressed={value === o.key}
          className={cn(
            "rounded-[5px] px-2.5 py-1 font-medium transition-colors",
            value === o.key
              ? "bg-foreground text-background"
              : "text-muted-foreground hover:text-foreground",
          )}
        >
          {o.label}
        </button>
      ))}
    </div>
  );
}

// A "nice" step so the axis ticks land on round numbers across either study's
// very different natural range (the DFS study spans ~18..435, the repair study
// ~200..360). Picks the largest of 10/20/25/50/100 that yields a handful of
// ticks over the span.
function niceStep(span: number): number {
  for (const step of [10, 20, 25, 50, 100]) {
    if (span / step <= 8) return step;
  }
  return 100;
}

export function VariantScoreSpread({
  rows,
  familyFill,
  domainMin,
  domainMax,
  labels,
}: {
  rows: SpreadRow[];
  familyFill: (family: string) => string;
  /** Axis lower bound. Omit to fit the data (floored to the step below the min). */
  domainMin?: number;
  /** Axis upper bound. Omit to fit the data (ceiled to the step above the max). */
  domainMax?: number;
  labels: SpreadLabels;
}) {
  const plotted = useMemo(
    () => rows.filter((r) => Array.isArray(r.scores) && r.scores.length > 0),
    [rows],
  );

  const { lo, hi, ticks } = useMemo(() => {
    const all = plotted.flatMap((r) => r.scores as number[]);
    const dataMin = all.length ? Math.min(...all) : 0;
    const dataMax = all.length ? Math.max(...all) : 1;
    const span = Math.max(1, dataMax - dataMin);
    const step = niceStep(span);
    const lo = domainMin ?? Math.floor(dataMin / step) * step;
    const hiRaw = domainMax ?? Math.ceil(dataMax / step) * step;
    const hi = hiRaw <= lo ? lo + step : hiRaw;
    const ticks: number[] = [];
    for (let x = Math.ceil(lo / step) * step; x <= hi; x += step) ticks.push(x);
    if (ticks[0] !== lo) ticks.unshift(lo);
    return { lo, hi, ticks };
  }, [plotted, domainMin, domainMax]);

  // Horizontal position, 0..100 %, of a score on the shared axis.
  const xOf = (v: number) => ((v - lo) / (hi - lo)) * 100;

  if (plotted.length === 0) return null;

  return (
    <div className="w-full">
      <div className="space-y-2.5">
        {plotted.map((r) => {
          const scores = r.scores as number[];
          const fill = familyFill(r.family);
          const worst = r.worst ?? Math.min(...scores);
          const best = r.best ?? Math.max(...scores);
          const mean =
            r.mean ?? Math.round((scores.reduce((s, x) => s + x, 0) / scores.length) * 10) / 10;
          // The min..max whisker sits behind the ticks so the range reads at a
          // glance even where ticks overlap.
          const left = xOf(worst);
          const width = Math.max(0.4, xOf(best) - xOf(worst));
          const a11y = `${r.display}: ${labels.mean} ${mean}, ${labels.range} ${worst}–${best}, ${scores.length} ${labels.instances}`;
          return (
            <div key={r.name} className="flex items-center gap-3">
              <div className="w-[140px] shrink-0 truncate text-xs font-medium" title={r.display}>
                {r.display}
              </div>
              <div
                className="relative h-6 flex-1"
                role="img"
                aria-label={a11y}
                title={a11y}
              >
                {/* baseline track */}
                <div className="absolute inset-x-0 top-1/2 h-px -translate-y-1/2 bg-border" aria-hidden />
                {/* min..max whisker in the family hue, faint */}
                <div
                  className="absolute top-1/2 h-1 -translate-y-1/2 rounded-full"
                  style={{
                    left: `${left}%`,
                    width: `${width}%`,
                    backgroundColor: fill,
                    opacity: 0.22,
                  }}
                  aria-hidden
                />
                {/* one tick per instance; overlapping ticks stack in opacity so
                    density is visible without a second colour. */}
                {scores.map((s, i) => (
                  <span
                    key={i}
                    className="absolute top-1/2 h-3.5 w-[3px] -translate-x-1/2 -translate-y-1/2 rounded-full"
                    style={{ left: `${xOf(s)}%`, backgroundColor: fill, opacity: 0.55 }}
                    aria-hidden
                  />
                ))}
                {/* the mean marker: a taller, solid diamond-capped rule */}
                <span
                  className="absolute top-1/2 h-5 w-[2px] -translate-x-1/2 -translate-y-1/2"
                  style={{ left: `${xOf(mean)}%`, backgroundColor: "currentColor" }}
                  aria-hidden
                />
                <span
                  className="absolute top-1/2 h-2 w-2 -translate-x-1/2 -translate-y-1/2 rotate-45"
                  style={{ left: `${xOf(mean)}%`, backgroundColor: "currentColor" }}
                  aria-hidden
                />
              </div>
              <div className="w-[112px] shrink-0 text-right text-xs tabular-nums text-muted-foreground">
                {labels.mean} {mean}{" "}
                <span className="text-muted-foreground/70">
                  ({worst}
                  <span aria-hidden>–</span>
                  {best})
                </span>
              </div>
            </div>
          );
        })}
      </div>

      {/* shared axis */}
      <div className="mt-3 flex items-center gap-3">
        <div className="w-[140px] shrink-0" aria-hidden />
        <div className="relative h-5 flex-1">
          <div className="absolute inset-x-0 top-0 h-px bg-border" aria-hidden />
          {ticks.map((tk) => (
            <span
              key={tk}
              className="absolute top-0 -translate-x-1/2 text-[10px] tabular-nums text-muted-foreground"
              style={{ left: `${xOf(tk)}%` }}
            >
              <span className="absolute left-1/2 top-0 h-1 w-px -translate-x-1/2 bg-border" aria-hidden />
              <span className="mt-1.5 inline-block">{tk}</span>
            </span>
          ))}
        </div>
        <div className="w-[112px] shrink-0" aria-hidden />
      </div>
      <p className="mt-3 text-xs text-muted-foreground">{labels.axis}</p>
    </div>
  );
}
