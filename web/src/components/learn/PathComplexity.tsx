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
import { getGeneratedPuzzleFramed, getPath, getPathKinds } from "@/engine";
import {
  estimateComplexity,
  COMPLEX_THEORY_SOURCE_URL,
  type ComplexityEstimate,
} from "@/engine-extras/complexity";

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
    sourcePrefix: "Method:",
    sourceName: "McGavin's reference implementation of Owen's complex theory",
    caveat: (
      <>
        An exact port of Peter McGavin's{" "}
        <a className="underline" href={COMPLEX_THEORY_SOURCE_URL} target="_blank" rel="noreferrer">
          reference implementation
        </a>{" "}
        of Owen's theory (validated to reproduce his published table — 14,702 solutions at the last
        cell). Still a first-moment estimate: it treats colours as independent, so it is accurate to
        ~×2 and{" "}
        <LocalizedLink className="underline" to="/research/why/entropy-area-law">
          blind to the distinctness collapse
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
    sourcePrefix: "Méthode :",
    sourceName: "implémentation de référence de McGavin de la théorie complexe d'Owen",
    caveat: (
      <>
        Un portage exact de{" "}
        <a className="underline" href={COMPLEX_THEORY_SOURCE_URL} target="_blank" rel="noreferrer">
          l'implémentation de référence
        </a>{" "}
        de Peter McGavin de la théorie d'Owen (validé pour reproduire sa table publiée — 14 702
        solutions à la dernière case). Reste une estimation au premier moment : elle traite les
        couleurs comme indépendantes, donc précise à ~×2 et{" "}
        <LocalizedLink className="underline" to="/research/why/entropy-area-law">
          aveugle à l'effondrement de distinction
        </LocalizedLink>
        . À utiliser pour comparer des ordres, pas comme un vrai décompte.
      </>
    ),
  },
  es: {
    title: "Predice el coste antes de la carrera",
    intro: (
      <>
        La{" "}
        <LocalizedLink className="underline" to="/research/why/complex-theory">
          teoría compleja
        </LocalizedLink>{" "}
        de Brendan Owen evalúa un orden de relleno sin ejecutarlo: el número esperado de nodos que la
        búsqueda visita en su profundidad más ancha (el <em>pico de meseta</em>) predice qué orden
        gana. Cuanto más bajo, mejor.
      </>
    ),
    yourPath: "Tu trayecto",
    peak: "pico de meseta",
    atDepth: (d: number) => `a profundidad ${d}`,
    drawPrompt: "Traza un trayecto en la cuadrícula para compararlo.",
    best: "mejor predicho",
    chartTitle: "Ramas esperadas en cada profundidad (escala logarítmica)",
    chartX: "profundidad (celdas colocadas)",
    sourcePrefix: "Método:",
    sourceName: "implementación de referencia de McGavin de la teoría compleja de Owen",
    caveat: (
      <>
        Un port exacto de la{" "}
        <a className="underline" href={COMPLEX_THEORY_SOURCE_URL} target="_blank" rel="noreferrer">
          implementación de referencia
        </a>{" "}
        de Peter McGavin de la teoría de Owen (validado para reproducir su tabla publicada: 14 702
        soluciones en la última celda). Sigue siendo una estimación de primer momento: trata los
        colores como independientes, así que es precisa hasta ~×2 y{" "}
        <LocalizedLink className="underline" to="/research/why/entropy-area-law">
          ciega al colapso de distinción
        </LocalizedLink>
        . Úsala para comparar órdenes, no como un recuento real.
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
  framed = false,
  hints = [],
}: {
  size: number;
  colors: number;
  seed: number;
  order: number[];
  framed?: boolean;
  hints?: number[];
}) {
  const t = useT(T);
  const engineReady = useEngine();
  // Hints are part of the configuration, not the drawn order — pre-placed before
  // every path, so they don't appear in the custom order but do shape the
  // estimate (exactly as McGavin's reference pre-places its hint squares).
  const hintKey = hints.join(",");

  const rows = useMemo<Row[]>(() => {
    if (!engineReady) return [];
    const puzzle = getGeneratedPuzzleFramed(size, colors, seed, framed);
    const hintCells = hintKey ? hintKey.split(",").map(Number) : [];
    const classics = ["row-major", "snake", "spiral-in", "diagonal"];
    const out: Row[] = [];
    if (order.length > 0) {
      out.push({
        id: "custom",
        label: t.yourPath,
        est: estimateComplexity(puzzle, order, hintCells),
        isCustom: true,
      });
    }
    const kinds = new Set(getPathKinds());
    for (const k of classics) {
      if (!kinds.has(k)) continue;
      // A classic path must skip hinted cells (they're pre-placed).
      const path = Array.from(getPath(k, size, size, seed)).filter((c) => !hintCells.includes(c));
      out.push({ id: k, label: k, est: estimateComplexity(puzzle, path, hintCells), isCustom: false });
    }
    return out.sort((a, b) => a.est.peakLog10 - b.est.peakLog10);
  }, [engineReady, size, colors, seed, framed, order, hintKey, t.yourPath]);

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
            <p className="-mt-2 text-center text-[10px] text-muted-foreground">
              {t.sourcePrefix}{" "}
              <a className="underline" href={COMPLEX_THEORY_SOURCE_URL} target="_blank" rel="noreferrer">
                {t.sourceName}
              </a>
            </p>
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
