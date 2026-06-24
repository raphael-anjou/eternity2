// Live complex-theory estimate for the search-path playground (Dan Karlsson,
// groups.io): score a fill order *before* racing it. For the path the user drew
// and each classic, we run Brendan Owen's first-moment model over the actual
// puzzle and report the plateau peak — the expected node count at the search's
// widest depth, the number that predicts which order wins.
//
// The model is faithful in the growth + plateau regime (it reproduces McGavin's
// published 16×16 curve depth-for-depth) and, like every first-moment estimate,
// is blind to the end-game distinctness collapse — so we report the peak, not a
// solution count, and link out to the caveat. Estimates are in single-piece
// units; a block-built path is scored by the order it implies, cell by cell.

import { useMemo } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { LocalizedLink } from "@/components/LocalizedLink";
import { getGeneratedPuzzle, getPath, getPathKinds } from "@/engine";
import { estimateComplexity, type ComplexityEstimate } from "@/engine-ts/complexity";

const T = {
  en: {
    title: "Predict the cost before you race",
    intro: (
      <>
        Brendan Owen's{" "}
        <LocalizedLink className="underline" to="/research/why/complex-theory">
          complex theory
        </LocalizedLink>{" "}
        scores a fill order without running it: the expected number of nodes the search visits at
        its widest depth (the <em>plateau peak</em>) predicts which order wins. Lower is better.
      </>
    ),
    yourPath: "Your path",
    peak: "plateau peak",
    atDepth: (d: number) => `at depth ${d}`,
    drawPrompt: "Draw a path on the grid to see how it compares.",
    best: "predicted best",
    chartTitle: "Expected branches at each depth (log scale)",
    chartX: "depth (cells placed)",
    caveat: (
      <>
        A first-moment estimate: it counts partial boards without checking they are distinct, so it
        over-counts past the plateau and is{" "}
        <LocalizedLink className="underline" to="/research/why/entropy-area-law">
          blind to the end-game collapse
        </LocalizedLink>
        . Use it to compare orders, not as a true count.
      </>
    ),
  },
  fr: {
    title: "Prédire le coût avant la course",
    intro: (
      <>
        La{" "}
        <LocalizedLink className="underline" to="/research/why/complex-theory">
          théorie complexe
        </LocalizedLink>{" "}
        de Brendan Owen évalue un ordre de remplissage sans le lancer : le nombre de nœuds attendu à
        la profondeur la plus large (le <em>pic de plateau</em>) prédit quel ordre gagne. Plus c'est
        bas, mieux c'est.
      </>
    ),
    yourPath: "Votre parcours",
    peak: "pic de plateau",
    atDepth: (d: number) => `à la profondeur ${d}`,
    drawPrompt: "Tracez un parcours sur la grille pour le comparer.",
    best: "meilleur prédit",
    chartTitle: "Branches attendues à chaque profondeur (échelle log)",
    chartX: "profondeur (cases posées)",
    caveat: (
      <>
        Une estimation au premier moment : elle compte les plateaux partiels sans vérifier qu'ils
        sont distincts, donc surcompte au-delà du plateau et est{" "}
        <LocalizedLink className="underline" to="/research/why/entropy-area-law">
          aveugle à l'effondrement final
        </LocalizedLink>
        . À utiliser pour comparer des ordres, pas comme un vrai décompte.
      </>
    ),
  },
};

/** Compact "m×10^x" / plain number for a log10 value. */
function fmtLog(log10: number): string {
  if (!isFinite(log10)) return "—";
  if (log10 < 4) return Math.round(10 ** log10).toLocaleString();
  const exp = Math.floor(log10);
  const mant = 10 ** (log10 - exp);
  return `${mant.toFixed(1)}×10^${exp}`;
}

interface Row {
  id: string;
  label: string;
  est: ComplexityEstimate;
  isCustom: boolean;
}

export function PathComplexity({
  size,
  colors,
  seed,
  order,
}: {
  size: number;
  colors: number;
  seed: number;
  order: number[];
}) {
  const t = useT(T);
  const engineReady = useEngine();

  const rows = useMemo<Row[]>(() => {
    if (!engineReady) return [];
    const puzzle = getGeneratedPuzzle(size, colors, seed);
    const classics = ["row-major", "snake", "spiral-in", "diagonal"];
    const out: Row[] = [];
    if (order.length > 0) {
      out.push({
        id: "custom",
        label: t.yourPath,
        est: estimateComplexity(puzzle, order),
        isCustom: true,
      });
    }
    const kinds = new Set(getPathKinds());
    for (const k of classics) {
      if (!kinds.has(k)) continue;
      const path = Array.from(getPath(k, size, size, seed));
      out.push({ id: k, label: k, est: estimateComplexity(puzzle, path), isCustom: false });
    }
    return out.sort((a, b) => a.est.peakLog10 - b.est.peakLog10);
  }, [engineReady, size, colors, seed, order, t.yourPath]);

  const bestId = rows[0]?.id;

  return (
    <details className="group rounded-lg border bg-card/40">
      <summary className="flex cursor-pointer list-none items-center justify-between gap-2 p-4 text-sm font-semibold [&::-webkit-details-marker]:hidden">
        <span>{t.title}</span>
        <svg
          viewBox="0 0 16 16"
          className="h-4 w-4 shrink-0 text-muted-foreground transition-transform group-open:rotate-180"
          fill="none"
          stroke="currentColor"
          strokeWidth={1.6}
        >
          <path d="M4 6l4 4 4-4" strokeLinecap="round" strokeLinejoin="round" />
        </svg>
      </summary>
      <div className="space-y-4 px-4 pb-4">
        <p className="max-w-xl text-xs text-muted-foreground">{t.intro}</p>

        {rows.length === 0 ? (
          <p className="text-xs text-muted-foreground">{t.drawPrompt}</p>
        ) : (
          <>
            <CurveChart rows={rows} chartTitle={t.chartTitle} chartX={t.chartX} />
            <ul className="space-y-1.5">
              {rows.map((r) => (
                <li
                  key={r.id}
                  className={`flex items-center justify-between rounded px-2 py-1 text-sm ${
                    r.isCustom ? "bg-primary/10 font-medium" : ""
                  }`}
                >
                  <span className="flex items-center gap-2">
                    <span
                      className="inline-block h-2.5 w-2.5 rounded-sm"
                      style={{ background: colorFor(r) }}
                    />
                    {r.label}
                    {r.id === bestId && (
                      <span className="rounded bg-emerald-500/15 px-1.5 py-0.5 text-[10px] text-emerald-600 dark:text-emerald-400">
                        {t.best}
                      </span>
                    )}
                  </span>
                  <span className="tabular-nums text-muted-foreground">
                    {t.peak}{" "}
                    <span className="font-medium text-foreground">{fmtLog(r.est.peakLog10)}</span>{" "}
                    <span className="text-[11px]">{t.atDepth(r.est.peakDepth)}</span>
                  </span>
                </li>
              ))}
            </ul>
            <p className="text-[11px] text-muted-foreground">{t.caveat}</p>
          </>
        )}
      </div>
    </details>
  );
}

function colorFor(r: Row): string {
  if (r.isCustom) return "hsl(270 70% 55%)";
  const palette: Record<string, string> = {
    "row-major": "hsl(210 70% 50%)",
    snake: "hsl(160 60% 45%)",
    "spiral-in": "hsl(30 80% 50%)",
    diagonal: "hsl(0 70% 55%)",
  };
  return palette[r.id] ?? "hsl(0 0% 60%)";
}

/** Overlaid log-scale depth→branches curves for all rows. */
function CurveChart({
  rows,
  chartTitle,
  chartX,
}: {
  rows: Row[];
  chartTitle: string;
  chartX: string;
}) {
  const W = 520;
  const H = 200;
  const padL = 34;
  const padR = 10;
  const padT = 12;
  const padB = 26;

  const maxDepth = Math.max(...rows.map((r) => r.est.curve.length), 1);
  // Cap the y-axis at each curve's peak: past the peak the first-moment curve
  // keeps drifting (end-game blindness), so we plot the cumulative up to the
  // peak depth, which is the meaningful, comparable part.
  const maxLog = Math.max(...rows.map((r) => r.est.peakLog10), 1);

  const x = (d: number) => padL + (d / maxDepth) * (W - padL - padR);
  const y = (logv: number) => padT + (1 - logv / maxLog) * (H - padT - padB);

  const gridLevels: number[] = [];
  const step = Math.max(1, Math.round(maxLog / 5));
  for (let lv = 0; lv <= maxLog; lv += step) gridLevels.push(lv);

  return (
    <div className="space-y-1">
      <p className="text-center text-[11px] font-medium text-muted-foreground">{chartTitle}</p>
      <svg viewBox={`0 0 ${W} ${H}`} className="w-full rounded-lg border bg-card">
        {gridLevels.map((lv) => (
          <g key={lv}>
            <line
              x1={padL}
              y1={y(lv)}
              x2={W - padR}
              y2={y(lv)}
              className="stroke-muted"
              strokeWidth={0.5}
            />
            <text x={3} y={y(lv) + 3} className="fill-muted-foreground text-[8px]">
              10^{lv}
            </text>
          </g>
        ))}
        {rows.map((r) => {
          // Plot cumulative up to the peak depth (the comparable region).
          const pts = r.est.curve
            .slice(0, r.est.peakDepth)
            .map((p) => `${x(p.depth)},${y(Math.log10(Math.max(p.cumulative, 1)))}`)
            .join(" ");
          return (
            <polyline
              key={r.id}
              points={pts}
              fill="none"
              stroke={colorFor(r)}
              strokeWidth={r.isCustom ? 2.2 : 1.3}
              strokeOpacity={r.isCustom ? 1 : 0.85}
            />
          );
        })}
        {[0, Math.round(maxDepth / 2), maxDepth].map((d) => (
          <text
            key={d}
            x={x(d)}
            y={H - padB + 14}
            textAnchor="middle"
            className="fill-muted-foreground text-[8px]"
          >
            {d}
          </text>
        ))}
        <text x={W / 2} y={H - 2} textAnchor="middle" className="fill-muted-foreground text-[9px]">
          {chartX}
        </text>
      </svg>
    </div>
  );
}
