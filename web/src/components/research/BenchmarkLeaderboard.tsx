import { useMemo } from "react";
import { HorizontalScoreChart } from "@/components/research/HorizontalScoreChart";
import { useT, useLang, pick, type Dict } from "@/i18n";
import data from "@/data/single-core-benchmark.json";

// The single-core benchmark leaderboard, rendered from the committed run data
// (web/src/data/single-core-benchmark.json, derived from the experiment's
// results.jsonl) so the page can never drift from what was measured.
//
// Three pieces, in the order the finding reads:
//   1. the throughput-vs-score paradox (two stat tiles + a verdict),
//   2. the ranked leaderboard (mean score, best marked, worst..best range),
//   3. a per-variant heatmap across every algorithm (not just the winner).
//
// Bars are coloured by algorithm FAMILY (categorical identity, never by rank),
// with a legend, so colour is never the only signal. Reference ticks mark our
// community 470 ceiling. Throughput units differ by family
// and are never cross-compared; each row shows its own native unit.
//
// Algo names carry an `anjou-` ownership prefix for Raphaël's engines; the two
// community engines (blackwood, verhaard) stay bare. The prefix is site-side
// only — the backing engine, its registry (`run_algo --list`) and the committed
// results.jsonl use the canonical unprefixed names, so the experiment stays
// reproducible against those. The site JSON is generated from results.jsonl by
// research/experiments/single-core-benchmark/scripts/make_site_json.py.

type Algo = {
  algo: string;
  family: string;
  mean: number;
  best: number;
  worst: number;
  std: number;
  nps: number | null;
  npsUnit: string;
  variants: (number | null)[];
  // "contender" rows compete on score and appear on the headline leaderboard;
  // rows without a role are the CSP-preset study (their own page). Set from the
  // manifest (solvers.toml `role`), never string-matched by name here.
  role?: string;
};

const D = data as {
  budgetS: number;
  maxScore: number;
  community: number;
  variantIds: number[];
  // Which two rows the throughput-vs-score paradox tiles show, keyed by role.
  // Set from the manifest (solvers.toml `paradox` tag) — never a hard-coded
  // algo name here, so renaming/adding solvers can't break the tiles.
  paradox: { throughput?: string; quality?: string; [role: string]: string | undefined };
  algos: Algo[];
};

// One hue per family, reused from the site's existing family palette
// (ExperimentScoreChart) so the research pages read as one system.
const FAMILY: Record<string, { fill: string } & Dict<string>> = {
  producer: { fill: "#a78bfa", en: "beam producer", fr: "producteur (beam)", es: "productor beam" },
  alns: { fill: "#34d399", en: "ALNS local search", fr: "recherche locale ALNS", es: "búsqueda local ALNS" },
  backtracker: { fill: "#fb7185", en: "backtracker", fr: "backtracking", es: "backtracking" },
  csp: { fill: "#38bdf8", en: "CSP + AC-3", fr: "CSP + AC-3", es: "CSP + AC-3" },
  naive: { fill: "#fbbf24", en: "naive DFS", fr: "DFS naïf", es: "DFS ingenuo" },
};
const familyFill = (f: string) => FAMILY[f]?.fill ?? "#94a3b8";

// Score domain: start below the weakest so every bar has visible length, end at
// the 480 maximum so the 5-clue-record tick sits in a true-to-scale position.
const DOMAIN_LO = 0;

const T = {
  en: {
    paradoxHead: "The result that matters — throughput is not score",
    blackwoodWho: "Blackwood backtracker",
    blackwoodSub: "the community's record algorithm",
    producerWho: "Our beam producer",
    producerSub: "comb fill-order, 150× fewer nodes/s",
    reaches: "reaches",
    verdict:
      "Blackwood explores 149× more nodes per second and still finishes 20 points lower. At one core, the win comes from node quality, not node count.",
    boardTitle: "The leaderboard",
    boardIntro:
      "Mean score over ten corner-pinned variants, single core, 60 s each. The methods that compete on score; the CSP-preset ablations live on their own page. Colour marks the family.",
    presetsTitle: "CSP presets, one engine, many knobs",
    presetsIntro:
      "Mean score over ten corner-pinned variants, single core, 60 s each. These are not separate solvers: they are one CSP engine (arc-consistency + a variable/value ordering) run under different presets. The best reaches ~183, less than half a contender's score, so none earns a leaderboard row.",
    ceiling: "5-clue record 464",
    npsUnit: "median throughput (native unit, never cross-compared)",
    heatTitle: "Every algorithm across all ten corner variants",
    heatIntro:
      "One 60 s run per cell, shaded against this table's own range. The backtrackers are flat near the top; the CSP engines are bimodal, collapsing to ~55 on hostile corners.",
    heatIntroPresets:
      "One 60 s run per cell. The presets are bimodal: several corners solve near the top, then the same knobs collapse to ~55 on hostile corners.",
    busy: "Drawing…",
    legendTitle: "family",
    of: "/ 480",
  },
  fr: {
    paradoxHead: "Le résultat qui compte — le débit n'est pas le score",
    blackwoodWho: "Backtracker de Blackwood",
    blackwoodSub: "l'algorithme du record communautaire",
    producerWho: "Notre producteur beam",
    producerSub: "ordre en peigne, 150× moins de nœuds/s",
    reaches: "atteint",
    verdict:
      "Blackwood explore 149× plus de nœuds par seconde et finit pourtant 20 points plus bas. Sur un cœur, la victoire vient de la qualité des nœuds, pas de leur nombre.",
    boardTitle: "Le classement",
    boardIntro:
      "Score moyen sur dix variantes à coins fixés, un cœur, 60 s chacune. Les méthodes qui rivalisent au score ; les préréglages CSP ont leur propre page. La couleur marque la famille.",
    presetsTitle: "Préréglages CSP : un moteur, plusieurs réglages",
    presetsIntro:
      "Score moyen sur dix variantes à coins fixés, un cœur, 60 s chacune. Ce ne sont pas des solveurs distincts : c'est un seul moteur CSP (consistance d'arc + un ordre variable/valeur) lancé sous différents préréglages. Le meilleur atteint ~183, moins de la moitié du score d'un prétendant ; aucun n'obtient donc de ligne au classement.",
    ceiling: "record 5 indices 464",
    npsUnit: "débit médian (unité native, jamais comparée entre familles)",
    heatTitle: "Chaque algorithme sur les dix variantes de coins",
    heatIntro:
      "Un run de 60 s par cellule, nuancé sur l'amplitude propre à ce tableau. Les backtrackers restent groupés en haut ; les moteurs CSP sont bimodaux et s'effondrent à ~55 sur les coins hostiles.",
    heatIntroPresets:
      "Un run de 60 s par cellule. Les préréglages sont bimodaux : plusieurs coins se résolvent près du sommet, puis les mêmes réglages s'effondrent à ~55 sur les coins hostiles.",
    busy: "Tracé…",
    legendTitle: "famille",
    of: "/ 480",
  },
  es: {
    paradoxHead: "El resultado que importa — el rendimiento no es la puntuación",
    blackwoodWho: "Backtracker de Blackwood",
    blackwoodSub: "el algoritmo del récord de la comunidad",
    producerWho: "Nuestro productor beam",
    producerSub: "orden de relleno en peine, 150× menos nodos/s",
    reaches: "alcanza",
    verdict:
      "Blackwood explora 149× más nodos por segundo y aun así termina 20 puntos por debajo. En un solo núcleo, la victoria viene de la calidad de los nodos, no de su cantidad.",
    boardTitle: "La clasificación",
    boardIntro:
      "Puntuación media sobre diez variantes con esquinas fijadas, un solo núcleo, 60 s cada una. Los métodos que compiten por la puntuación; las ablaciones de preajustes CSP tienen su propia página. El color marca la familia.",
    presetsTitle: "Preajustes CSP: un motor, muchos ajustes",
    presetsIntro:
      "Puntuación media sobre diez variantes con esquinas fijadas, un solo núcleo, 60 s cada una. No son solucionadores distintos: son un único motor CSP (consistencia de arco + un orden de variable/valor) ejecutado con distintos preajustes. El mejor alcanza ~183, menos de la mitad de la puntuación de un contendiente, así que ninguno obtiene una fila en la clasificación.",
    ceiling: "récord 5 pistas 464",
    npsUnit: "rendimiento mediano (unidad nativa, nunca comparada entre familias)",
    heatTitle: "Cada algoritmo sobre las diez variantes de esquinas",
    heatIntro:
      "Una ejecución de 60 s por celda, sombreada según el rango propio de esta tabla. Los backtrackers se agrupan cerca del máximo; los motores CSP son bimodales y se desploman a ~55 en las esquinas hostiles.",
    heatIntroPresets:
      "Una ejecución de 60 s por celda. Los preajustes son bimodales: varias esquinas se resuelven cerca del máximo, luego los mismos ajustes se desploman a ~55 en las esquinas hostiles.",
    busy: "Dibujando…",
    legendTitle: "familia",
    of: "/ 480",
  },
};

// Compact throughput number, null-safe: 40.6M, 273K.
function npsNum(nps: number | null): string {
  if (nps == null) return "—";
  return nps >= 1e6 ? `${(nps / 1e6).toFixed(1)}M` : `${Math.round(nps / 1e3)}K`;
}
function fmtNps(nps: number | null, unit: string): string {
  return nps == null ? "—" : `${npsNum(nps)} ${unit}`;
}

// Heatmap cell colour: a single-hue sequential ramp over the score range, so a
// weak cell reads pale and a strong one saturated (magnitude, one hue).
//
// The domain is derived from the cells actually rendered, never hard-coded: the
// two views span very different ranges (contenders sit in a tight 431–451 band,
// the presets spread 35–349), and the set of engines changes as they are added
// or withheld. A fixed domain silently rots — it used to end at the producer's
// 456, so once that engine left the grid nothing reached full saturation and
// the palest cell clamped below the floor.
type HeatScale = (v: number | null) => string;
function makeHeatScale(values: number[]): HeatScale {
  const lo = Math.min(...values);
  const hi = Math.max(...values);
  const span = hi - lo;
  return (v) => {
    if (v == null) return "var(--muted)";
    // A degenerate span (one row, or every score equal) would divide by zero;
    // give those cells the full-strength end rather than NaN.
    const t = span > 0 ? Math.max(0, Math.min(1, (v - lo) / span)) : 1;
    // emerald ramp: light (low) → dark (high), via alpha over the surface.
    return `color-mix(in srgb, #10b981 ${12 + t * 78}%, transparent)`;
  };
}

// Two views of the same committed run data:
//   "contenders" (default) — the headline leaderboard: only rows the manifest
//     tags role:contender (they compete on score). Shows the paradox + verdict.
//   "presets" — the CSP-preset study page: every other row (the AC-3 / ordering
//     ablations of one engine, plus the naive_spiral variant). No paradox tiles;
//     the best preset (~183) is less than half a contender's score, so it isn't
//     a leaderboard rival, it's a same-engine ablation sweep shown on its own.
export function BenchmarkLeaderboard({
  view = "contenders",
}: {
  view?: "contenders" | "presets";
}) {
  const t = useT(T);
  const { lang } = useLang();
  const isContenders = view === "contenders";
  const algos = D.algos.filter((a) =>
    isContenders ? a.role === "contender" : a.role !== "contender",
  );
  const height = Math.max(320, algos.length * 32 + 48);
  // Scale the heatmap to the cells this view actually draws (see makeHeatScale).
  const heatColor = useMemo(() => {
    const cells = algos.flatMap((a) => a.variants).filter((v): v is number => v != null);
    return makeHeatScale(cells.length ? cells : [0]);
  }, [algos]);

  // The paradox tiles compare a high-throughput / lower-score engine against a
  // low-throughput / higher-score one. WHICH rows is a manifest fact carried in
  // D.paradox (by role), so no algo name is hard-coded here. Only the headline
  // (contenders) view shows them.
  const blackwood = isContenders
    ? D.algos.find((a) => a.algo === D.paradox.throughput)
    : undefined;
  const producer = isContenders
    ? D.algos.find((a) => a.algo === D.paradox.quality)
    : undefined;

  const familiesPresent = Array.from(new Set(algos.map((a) => a.family)));

  return (
    <section className="space-y-6">
      {/* 1. the paradox */}
      {blackwood && producer && (
        <div className="overflow-hidden rounded-xl border">
          <div className="border-b bg-muted/30 px-4 py-2 font-mono text-xs uppercase tracking-wider text-muted-foreground">
            {t.paradoxHead}
          </div>
          <div className="grid gap-px bg-border sm:grid-cols-2">
            {[
              { who: t.blackwoodWho, sub: t.blackwoodSub, a: blackwood },
              { who: t.producerWho, sub: t.producerSub, a: producer },
            ].map(({ who, sub, a }) => (
              <div key={a.algo} className="bg-background p-4">
                <div className="font-semibold">{who}</div>
                <div className="mb-3 text-xs text-muted-foreground">{sub}</div>
                <div className="font-mono text-3xl font-semibold tabular-nums">
                  {npsNum(a.nps)}
                  <span className="ml-1 text-sm font-normal text-muted-foreground">
                    {a.npsUnit}
                  </span>
                </div>
                <div className="mt-1 font-mono text-sm text-muted-foreground">
                  {t.reaches}{" "}
                  <b className="text-base text-foreground">{a.best}</b>
                </div>
              </div>
            ))}
          </div>
          <div className="border-t bg-amber-500/10 px-4 py-3 text-sm">{t.verdict}</div>
        </div>
      )}

      {/* 2. the leaderboard (contenders) or the preset sweep (presets view) */}
      <div>
        <h2 className="text-xl font-semibold tracking-tight">
          {isContenders ? t.boardTitle : t.presetsTitle}
        </h2>
        <p className="mb-1 mt-1 text-sm text-muted-foreground">
          {isContenders ? t.boardIntro : t.presetsIntro}
        </p>
        {/* legend: family identity, always present (>2 series) */}
        <div className="mb-2 flex flex-wrap gap-x-4 gap-y-1 font-mono text-xs text-muted-foreground">
          {familiesPresent.map((f) => (
            <span key={f} className="inline-flex items-center gap-1.5">
              <i
                aria-hidden
                className="inline-block h-2.5 w-2.5 rounded-sm"
                style={{ background: familyFill(f) }}
              />
              {FAMILY[f] ? pick(FAMILY[f], lang) : f}
            </span>
          ))}
        </div>
        <div role="img" aria-label={`${t.boardTitle}. ${t.boardIntro}`}>
          <HorizontalScoreChart
            rows={algos}
            valueKey="mean"
            categoryKey="algo"
            domainMin={DOMAIN_LO}
            domainMax={D.maxScore}
            tickCount={7}
            allowDecimals={false}
            categoryWidth={172}
            barCategoryGap={6}
            height={height}
            colorOf={(a) => familyFill(a.family)}
            busyLabel={t.busy}
            tooltip={(a) => (
              <div className="rounded-md border bg-popover px-3 py-2 text-xs shadow-md">
                <div className="font-semibold">{a.algo}</div>
                <div className="mt-1 text-muted-foreground">
                  mean {a.mean} · range {a.worst}–{a.best} · σ{a.std} · {fmtNps(a.nps, a.npsUnit)}
                </div>
              </div>
            )}
            // These variants pin all 5 clues, so the meaningful ceiling is the
            // 5-clue community record (464), not the all-hints 470. Anchor the
            // label to the END so it grows leftward and never clips.
            referenceLines={[
              {
                x: D.community,
                color: "#f59e0b",
                label: t.ceiling,
                textAnchor: "end",
                dx: -2,
              },
            ]}
          />
        </div>
      </div>

      {/* 3. the per-variant heatmap across all algos */}
      <div>
        <h2 className="text-xl font-semibold tracking-tight">{t.heatTitle}</h2>
        <p className="mb-2 mt-1 text-sm text-muted-foreground">
          {isContenders ? t.heatIntro : t.heatIntroPresets}
        </p>
        <div className="overflow-x-auto">
          <table className="w-full border-collapse text-center font-mono text-xs">
            <thead>
              <tr className="text-muted-foreground">
                <th className="px-2 py-1 text-left font-normal">algo</th>
                {D.variantIds.map((v) => (
                  <th key={v} className="px-1 py-1 font-normal tabular-nums">
                    v{String(v).padStart(2, "0")}
                  </th>
                ))}
              </tr>
            </thead>
            <tbody>
              {algos.map((a) => (
                <tr key={a.algo}>
                  <td className="whitespace-nowrap px-2 py-0.5 text-left text-foreground">
                    {a.algo}
                  </td>
                  {a.variants.map((s, i) => (
                    <td
                      key={i}
                      className="px-1 py-0.5 tabular-nums text-foreground"
                      style={{ background: heatColor(s) }}
                      title={`${a.algo} · v${String(D.variantIds[i]).padStart(2, "0")}: ${s ?? "—"}`}
                    >
                      {s ?? "—"}
                    </td>
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </section>
  );
}
