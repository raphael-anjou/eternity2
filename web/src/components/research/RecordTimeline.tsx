import { useMemo, useState } from "react";
import { useT } from "@/i18n";
import { RECORDS, type RecordRow } from "@/data/records-timeline";

const T = {
  en: {
    chartAria: "Eternity II community record score over time, 2008 to 2026",
    zoomFull: "Full history",
    zoomRecent: "Recent window",
    zoomHint: "Zoom the axes to the recent window (458 to 470, 2020 to 2026) to separate the closely spaced modern boards.",
    legendOpen: "open record (starter-only)",
    legendStrict: "strict five-clue record",
    solutionLabel: "480: a full solution (never reached)",
    plateauLabel: "open record flat since 2021",
    idle:
      "Hover or tab through a point to see who found it and how. The open record rose from Verhaard's 467 (2008) to Blackwood's 470 (2021), then stopped; the strict five-clue track climbed from 460 to 464 in 2026.",
  },
  fr: {
    chartAria: "Score record de la communauté Eternity II au fil du temps, de 2008 à 2026",
    zoomFull: "Toute l'histoire",
    zoomRecent: "Fenêtre récente",
    zoomHint: "Zoomer les axes sur la fenêtre récente (458 à 470, 2020 à 2026) pour séparer les plateaux modernes très rapprochés.",
    legendOpen: "record ouvert (pièce de départ seule)",
    legendStrict: "record strict à cinq indices",
    solutionLabel: "480 : une solution complète (jamais atteinte)",
    plateauLabel: "record ouvert stable depuis 2021",
    idle:
      "Survolez un point ou parcourez-les au clavier pour voir qui l'a trouvé et comment. Le record ouvert est monté du 467 de Verhaard (2008) au 470 de Blackwood (2021), puis s'est arrêté ; la piste stricte à cinq indices est passée de 460 à 464 en 2026.",
  },
  es: {
    chartAria: "Puntuación récord de la comunidad de Eternity II a lo largo del tiempo, de 2008 a 2026",
    zoomFull: "Toda la historia",
    zoomRecent: "Ventana reciente",
    zoomHint: "Amplía los ejes a la ventana reciente (458 a 470, 2020 a 2026) para separar los tableros modernos, muy juntos.",
    legendOpen: "récord abierto (solo pieza de partida)",
    legendStrict: "récord estricto de cinco pistas",
    solutionLabel: "480: una solución completa (nunca alcanzada)",
    plateauLabel: "récord abierto estable desde 2021",
    idle:
      "Pasa el cursor o recorre los puntos con el teclado para ver quién lo encontró y cómo. El récord abierto subió del 467 de Verhaard (2008) al 470 de Blackwood (2021), y luego se detuvo; la pista estricta de cinco pistas subió de 460 a 464 en 2026.",
  },
};

// The community record climb, plotted. The records table lists every
// announcement; this chart shows the *shape* of the history: the 467 of 2008,
// the long silence to 2020, the fast 468 to 469 to 470 rise on Blackwood's
// solver, and the flat line since 2021 that is the open record. A second,
// lower track follows the strict-five-clue boards (460 to 464). Hover or focus a
// point to see who found it and how. A zoom toggle rescales both axes to the
// recent window so the closely spaced modern boards separate out.
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
  const [zoomed, setZoomed] = useState(false);

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
  // Full view spans the whole history; the zoomed view rescales both axes to the
  // recent window (2020 to 2026, 458 to 470) so the tightly clustered modern
  // boards stop overlapping. The 480 solution line stays visible in the full
  // view only; in the zoomed view the top of the axis is the standing record.
  const tMin = zoomed ? 2020 : Math.min(...years);
  const tMax = zoomed ? 2026.6 : Math.max(...years);
  const sMin = zoomed ? 458 : 455; // a little below the lowest plotted board in view
  const sMax = zoomed ? 470 : 480; // the unreached solution (full) or standing record (zoom)

  const x = (t: number) => padL + ((t - tMin) / (tMax - tMin)) * (W - padL - padR);
  const y = (s: number) => padT + (1 - (s - sMin) / (sMax - sMin)) * (H - padT - padB);

  const open = points.filter((p) => p.track === "open").sort((a, b) => a.t - b.t);
  const strict = points.filter((p) => p.track === "strict").sort((a, b) => a.t - b.t);
  const line = (ps: Plotted[]) => ps.map((p) => `${x(p.t)},${y(p.score)}`).join(" ");

  const yTicks = zoomed ? [458, 460, 462, 464, 466, 468, 470] : [455, 460, 465, 470, 475, 480];
  const xTicks = zoomed ? [2020, 2022, 2024, 2026] : [2008, 2012, 2016, 2020, 2024];

  const activePoint = active !== null ? points[active] : null;

  return (
    <div className="space-y-3">
      <div className="mx-auto flex max-w-2xl items-center justify-end">
        <div className="inline-flex rounded-md border p-0.5 text-xs" role="group" aria-label={t.zoomHint}>
          <button
            type="button"
            aria-pressed={!zoomed}
            onClick={() => setZoomed(false)}
            className={`rounded px-2 py-1 transition-colors ${
              !zoomed ? "bg-foreground text-background" : "text-muted-foreground hover:text-foreground"
            }`}
          >
            {t.zoomFull}
          </button>
          <button
            type="button"
            aria-pressed={zoomed}
            onClick={() => setZoomed(true)}
            className={`rounded px-2 py-1 transition-colors ${
              zoomed ? "bg-foreground text-background" : "text-muted-foreground hover:text-foreground"
            }`}
          >
            {t.zoomRecent}
          </button>
        </div>
      </div>
      <div className="mx-auto max-w-2xl">
        <svg
          viewBox={`0 0 ${W} ${H}`}
          className="w-full rounded-lg border bg-card"
          aria-labelledby="record-timeline-title"
        >
          {/* Named via <title>, NOT role="img": role="img" would flatten the
              chart and hide the focusable record points (role="button") inside
              it from assistive tech. */}
          <title id="record-timeline-title">{t.chartAria}</title>
          {/* the 480 solution line: the unreached target (in the full view only;
              the zoomed axis tops out at the standing 470 record) */}
          {!zoomed && (
            <>
              <line x1={padL} y1={y(480)} x2={W - padR} y2={y(480)} className="stroke-emerald-500/50" strokeWidth={1} strokeDasharray="4 3" />
              <text x={W - padR} y={y(480) - 4} textAnchor="end" className="fill-emerald-600 text-[8px] dark:fill-emerald-400">
                {t.solutionLabel}
              </text>
            </>
          )}

          {/* the plateau band: 470 held since 2021 */}
          <rect x={x(2021.25)} y={padT} width={W - padR - x(2021.25)} height={y(470) - padT} className="fill-amber-400/10" />
          <text x={(x(2021.25) + W - padR) / 2} y={padT + 10} textAnchor="middle" className="fill-amber-600/80 text-[8px] dark:fill-amber-400/80">
            {t.plateauLabel}
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
                  aria-label={`${p.row.score}, ${p.row.author}, ${p.row.date}`}
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
          <span className="inline-block h-2 w-2 rounded-full bg-foreground" /> {t.legendOpen}
        </span>
        <span className="flex items-center gap-1.5">
          <span className="inline-block h-2 w-2 rounded-full bg-sky-500" /> {t.legendStrict}
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
          <span className="text-muted-foreground">{t.idle}</span>
        )}
      </div>
    </div>
  );
}
