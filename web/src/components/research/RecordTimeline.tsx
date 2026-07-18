import { useMemo, useState } from "react";
import { useT } from "@/i18n";
import { RECORDS, type RecordRow } from "@/data/records-timeline";

const T = {
  en: { chartAria: "Eternity II community record score over time, 2008 to 2026" },
  fr: { chartAria: "Score record de la communauté Eternity II au fil du temps, de 2008 à 2026" },
  es: { chartAria: "Puntuación récord de la comunidad de Eternity II a lo largo del tiempo, de 2008 a 2026" },
};

// The community record climb, plotted. The records table lists every
// announcement; this chart shows the *shape* of the history — the 467 of 2008,
// the long silence to 2020, the fast 468 → 469 → 470 rise on Blackwood's
// solver, and the flat line since 2021 that is the open record. A second,
// lower track follows the strict-five-clue boards (460 → 464). Hover or focus a
// point to see who found it and how.
//
// Data comes from the shared records-timeline module (single source of truth
// with the table). Milestone rows with no numeric score (release, deadline) are
// dropped from the plot but power the x-axis span.

interface Plotted {
  row: RecordRow;
  /** Fractional year, e.g. 2020.67. */
  t: number;
  score: number;
  /** Which track: the open (starter-only) record vs the strict-5-clue record. */
  track: "open" | "strict";
}

/** Parse a partial date (YYYY | YYYY-MM | YYYY-MM-DD) to a fractional year. */
function toYear(date: string): number {
  const [y, m, d] = date.split("-").map(Number);
  return (y ?? 0) + ((m ?? 1) - 1) / 12 + ((d ?? 1) - 1) / 365;
}

/** The strict-five-clue boards are a separate, lower track; everything else
 *  numeric is the open (starter-only) record. Identified by author, matching
 *  the table's own strict-vs-open framing. */
const STRICT_AUTHORS = new Set(["Bruno Gauthier", "Benjamin Riotte"]);

export function RecordTimeline() {
  const t = useT(T);
  const [active, setActive] = useState<number | null>(null);

  const points = useMemo<Plotted[]>(() => {
    return RECORDS.flatMap((row) => {
      const n = Number(row.score);
      if (row.canonical !== "canonical" || !Number.isFinite(n)) return [];
      return [
        {
          row,
          t: toYear(row.date),
          score: n,
          track: STRICT_AUTHORS.has(row.author) ? "strict" : "open",
        },
      ];
    });
  }, []);

  // Geometry.
  const W = 560;
  const H = 300;
  const padL = 34;
  const padR = 16;
  const padT = 20;
  const padB = 40;

  const years = RECORDS.map((r) => toYear(r.date));
  const tMin = Math.min(...years);
  const tMax = Math.max(...years);
  const sMin = 455; // a little below the lowest plotted strict board (460)
  const sMax = 480; // the unreached solution

  const x = (t: number) => padL + ((t - tMin) / (tMax - tMin)) * (W - padL - padR);
  const y = (s: number) => padT + (1 - (s - sMin) / (sMax - sMin)) * (H - padT - padB);

  const open = points.filter((p) => p.track === "open").sort((a, b) => a.t - b.t);
  const strict = points.filter((p) => p.track === "strict").sort((a, b) => a.t - b.t);
  const line = (ps: Plotted[]) => ps.map((p) => `${x(p.t)},${y(p.score)}`).join(" ");

  const yTicks = [455, 460, 465, 470, 475, 480];
  const xTicks = [2008, 2012, 2016, 2020, 2024];

  const activePoint = active !== null ? points[active] : null;

  return (
    <div className="space-y-3">
      <div className="mx-auto max-w-2xl">
        <svg
          viewBox={`0 0 ${W} ${H}`}
          className="w-full rounded-lg border bg-card"
          role="img"
          aria-label={t.chartAria}
        >
          {/* the 480 solution line — the unreached target */}
          <line x1={padL} y1={y(480)} x2={W - padR} y2={y(480)} className="stroke-emerald-500/50" strokeWidth={1} strokeDasharray="4 3" />
          <text x={W - padR} y={y(480) - 4} textAnchor="end" className="fill-emerald-600 text-[8px] dark:fill-emerald-400">
            480 — a full solution (never reached)
          </text>

          {/* the plateau band: 470 held since 2021 */}
          <rect x={x(2021.25)} y={padT} width={W - padR - x(2021.25)} height={y(470) - padT} className="fill-amber-400/10" />
          <text x={(x(2021.25) + W - padR) / 2} y={padT + 10} textAnchor="middle" className="fill-amber-600/80 text-[8px] dark:fill-amber-400/80">
            open record flat since 2021
          </text>

          {/* y gridlines */}
          {yTicks.map((s) => (
            <g key={s}>
              <line x1={padL} y1={y(s)} x2={W - padR} y2={y(s)} className="stroke-muted" strokeWidth={0.5} />
              <text x={4} y={y(s) + 3} className="fill-muted-foreground text-[8px]">{s}</text>
            </g>
          ))}
          {/* x ticks */}
          {xTicks.map((yr) => (
            <text key={yr} x={x(yr)} y={H - padB + 16} textAnchor="middle" className="fill-muted-foreground text-[8px]">{yr}</text>
          ))}

          {/* strict-clue track (lower, dashed) */}
          <polyline points={line(strict)} fill="none" className="stroke-sky-500/70" strokeWidth={1.5} strokeDasharray="3 2" />
          {/* open record track */}
          <polyline points={line(open)} fill="none" className="stroke-foreground" strokeWidth={1.75} />

          {/* points (open then strict, so both draw on top of lines) */}
          {points.map((p, i) => {
            const isActive = i === active;
            const strictTrack = p.track === "strict";
            return (
              <g key={`${p.row.date}-${p.row.author}`}>
                <circle
                  cx={x(p.t)}
                  cy={y(p.score)}
                  r={isActive ? 5 : 3.2}
                  className={
                    strictTrack
                      ? "fill-sky-500 stroke-card"
                      : "fill-foreground stroke-card"
                  }
                  strokeWidth={1}
                  tabIndex={0}
                  role="button"
                  aria-label={`${p.row.score} — ${p.row.author}, ${p.row.date}`}
                  onMouseEnter={() => setActive(i)}
                  onFocus={() => setActive(i)}
                  onMouseLeave={() => setActive((a) => (a === i ? null : a))}
                  onBlur={() => setActive((a) => (a === i ? null : a))}
                  style={{ cursor: "pointer" }}
                />
              </g>
            );
          })}
        </svg>
      </div>

      {/* legend + readout */}
      <div className="mx-auto flex max-w-2xl flex-wrap items-center justify-center gap-x-4 gap-y-1 text-xs text-muted-foreground">
        <span className="flex items-center gap-1.5">
          <span className="inline-block h-2 w-2 rounded-full bg-foreground" /> open record (starter-only)
        </span>
        <span className="flex items-center gap-1.5">
          <span className="inline-block h-2 w-2 rounded-full bg-sky-500" /> strict five-clue record
        </span>
      </div>

      <div
        className="mx-auto min-h-[3.5rem] max-w-2xl rounded-md border bg-muted/30 p-3 text-xs leading-relaxed"
        aria-live="polite"
      >
        {activePoint ? (
          <>
            <span className="font-semibold text-foreground">
              {activePoint.score} · {activePoint.row.author}
            </span>
            <span className="text-muted-foreground"> · {activePoint.row.date}</span>
            <p className="mt-1 text-muted-foreground">{activePoint.row.method}</p>
          </>
        ) : (
          <span className="text-muted-foreground">
            Hover or tab through a point to see who found it and how. The open record rose from
            Verhaard&rsquo;s 467 (2008) to Blackwood&rsquo;s 470 (2021), then stopped; the strict
            five-clue track climbed from 460 to 464 in 2026.
          </span>
        )}
      </div>
    </div>
  );
}
