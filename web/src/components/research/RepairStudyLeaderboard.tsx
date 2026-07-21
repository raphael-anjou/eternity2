import { useMemo, useState } from "react";
import { Line, LineChart, ResponsiveContainer, Tooltip, XAxis, YAxis } from "recharts";
import { HorizontalScoreChart } from "@/components/research/HorizontalScoreChart";
import { VariantScoreSpread, ViewToggle } from "@/components/research/VariantScoreSpread";
import { FamilyLegend, FamilyTag } from "@/components/research/FamilyLegend";
import { formatKM } from "@/lib/format";
import { useT, useLang, pick, type Dict } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import data from "@/data/repair-study.json";

// The repair-study results, rendered from the committed run data
// (web/src/data/repair-study.json, derived from the experiment's results.jsonl by
// research/experiments/repair-study/scripts/make_site_json.py) so the page can
// never drift from what was measured.
//
// Four pieces:
//   1. the leaderboard — mean final score per variant over ten corner-pinned
//      instances, coloured by FAMILY (identity, never rank), family also named
//      in the label so colour is never the only signal;
//   2. the lift panel — the mean gain each variant added over its own starting
//      board, the repair loop's actual contribution isolated from construction;
//   3. the stall panel — for each variant, the iteration the best last improved
//      against the total iterations run, the study's headline: improvement
//      arrives early, then the loop grinds without moving;
//   4. the matrix — every variant as a delta over its parent, generated from the
//      engine registry, not hand-kept.
//
// The palette is the DFS study's validated five-hue family set, reused so the two
// sibling studies read the same; family is always named in text, so colour is
// secondary. (dataviz: CVD deutan ΔE 25.2 / tritan 8.4, normal-vision 26.5; the
// two lighter hues get relief from per-bar labels and the table view.)

type Variant = {
  name: string;
  display: string;
  family: string;
  parent: string | null;
  delta: string;
  start: string;
  destroy: string;
  repair: string;
  accept: string;
  restart: string;
  n?: number;
  mean?: number | null;
  best?: number | null;
  worst?: number | null;
  // Every instance's raw final score, in run order, for the per-variant spread.
  scores?: number[] | null;
  mean_lift?: number | null;
  start_score?: number | null;
  mean_last_best_iter?: number | null;
  mean_iterations?: number | null;
  accept_rate?: number | null;
  mean_destroy?: number | null;
  median_ips?: number | null;
  ips_unit?: string;
  restarts?: number | null;
  // Convergence curve as [iteration, best-score] pairs (downsampled, true
  // iteration preserved so a log x-axis reads correctly; first point at iter 1).
  curve?: [number, number][] | null;
};

// `as unknown as` (not a plain `as`): the JSON infers `curve` as number[][],
// which doesn't structurally overlap `[number, number][]`, so TS rejects the
// direct cast. The shape is guaranteed by the generating script (see below).
const D = data as unknown as {
  budget_s: number;
  seed: number;
  n_instances: number;
  max_score: number;
  curve_stride: number;
  variants: Variant[];
};

// One hue per family (the DFS study's validated set). Family is also always named
// in text, so colour is secondary, never the sole signal.
const FAMILY: Record<string, { fill: string } & Dict<string>> = {
  start: { fill: "#f59e0b", en: "starting board", fr: "plateau de départ", es: "tablero de partida" },
  destroy: { fill: "#3b82f6", en: "destroy operator", fr: "opérateur de destruction", es: "operador de destrucción" },
  repair: { fill: "#10b981", en: "repair", fr: "réparation", es: "reparación" },
  accept: { fill: "#8b5cf6", en: "acceptance", fr: "acceptation", es: "aceptación" },
  restart: { fill: "#ef4444", en: "restart", fr: "redémarrage", es: "reinicio" },
};
const familyFill = (f: string) => FAMILY[f]?.fill ?? "#94a3b8";

// The variants whose convergence curve the stall figure draws: the surprising
// winner that keeps improving (random-destroy), the strongest acceptance rule
// (accept-anneal), the plain conflict-driven loop that stalls early
// (greedy-mismatch), and the component destroy that collapses almost at once.
// Kept small so the chart stays legible; every variant's numbers are in the table.
const CURVE_KEYS = ["random-destroy", "accept-anneal", "greedy-mismatch", "component-destroy"];

const T = {
  en: {
    boardTitle: "The leaderboard — mean final score by variant",
    boardIntro:
      "Mean matched-edge score of the board each variant finished with, over ten corner-pinned variants of the official puzzle, single core, 60 s per run. Colour marks the family; the family is named on every bar, so colour is never the only signal.",
    viewBars: "mean",
    viewSpread: "spread",
    spreadIntro:
      "The same runs, but every instance's final score shown as a tick, with the mean marked. The mean bar hides how much each variant swings from instance to instance; this shows it. Where two variants sit close on the mean, the spread is the fair reading of whether they truly differ.",
    spreadAxis: "final score per instance (matched edges, out of 480)",
    spreadMean: "mean",
    spreadRange: "range",
    spreadInstances: "instances",
    of: "/ 480",
    liftTitle: "What the loop added over its starting board",
    liftIntro:
      "The same runs, but showing only the lift: the mean gain each variant made over the board it started from. This isolates the repair loop's own contribution from the construction it began with — a variant that starts high can add little and still finish high.",
    liftAxis: "mean lift (matched edges)",
    stallTitle: "Where improvement stops",
    stallIntro:
      "For four representative variants, the best score so far against the iteration count (sampled every 200 iterations, medians over the ten instances). The point of the study in one picture: on a strong starting board the conflict-driven loop flattens within a few thousand iterations and the remaining hundreds of thousands move nothing, while blind random destroy keeps finding gains far longer.",
    stallAxis: "iterations",
    stallY: "best score so far",
    matrixTitle: "What stacks on what",
    matrixIntro:
      "Every variant is the same destroy-and-repair loop declared as one change over its parent. This table is generated from the engine registry, so it always matches the code that ran. `last best` is the mean iteration the global best last improved; compare it to `iters` to read how early each run stalled.",
    colVariant: "variant",
    colFamily: "family",
    colDelta: "the one change it adds over its parent",
    colMean: "mean",
    colLift: "lift",
    colLastBest: "last best",
    colIters: "iters",
    colAccept: "accept",
    tblStatsTitle: "Every variant, every raised statistic",
    busy: "Drawing…",
  },
  fr: {
    boardTitle: "Le classement — score final moyen par variante",
    boardIntro:
      "Score moyen (arêtes appariées) du plateau final de chaque variante, sur dix variantes à coins fixés du puzzle officiel, un cœur, 60 s par run. La couleur marque la famille, nommée sur chaque barre : la couleur n'est jamais le seul signal.",
    viewBars: "moyenne",
    viewSpread: "dispersion",
    spreadIntro:
      "Les mêmes runs, mais le score final de chaque instance apparaît sous forme de repère, la moyenne étant marquée. La barre de moyenne masque l'ampleur des écarts d'une instance à l'autre ; ceci la montre. Là où deux variantes se tiennent de près en moyenne, la dispersion dit s'il y a vraiment une différence.",
    spreadAxis: "score final par instance (arêtes appariées, sur 480)",
    spreadMean: "moyenne",
    spreadRange: "plage",
    spreadInstances: "instances",
    of: "/ 480",
    liftTitle: "Ce que la boucle a ajouté à son plateau de départ",
    liftIntro:
      "Les mêmes runs, mais montrant seulement le gain : l'amélioration moyenne apportée par chaque variante par rapport au plateau dont elle part. Cela isole la contribution propre de la boucle de réparation de la construction initiale — une variante qui part haut peut ajouter peu et finir haut malgré tout.",
    liftAxis: "gain moyen (arêtes appariées)",
    stallTitle: "Où l'amélioration s'arrête",
    stallIntro:
      "Pour quatre variantes représentatives, le meilleur score atteint en fonction du nombre d'itérations (échantillonné toutes les 200 itérations, médianes sur les dix instances). Le cœur de l'étude en une image : sur un bon plateau de départ, la boucle guidée par les conflits se stabilise en quelques milliers d'itérations et les centaines de milliers suivantes ne bougent rien, tandis que la destruction aléatoire aveugle continue à trouver des gains bien plus longtemps.",
    stallAxis: "itérations",
    stallY: "meilleur score atteint",
    matrixTitle: "Ce qui s'empile sur quoi",
    matrixIntro:
      "Chaque variante est la même boucle destruction-réparation déclarée comme un seul changement par rapport à son parent. Ce tableau est généré depuis le registre du moteur : il correspond toujours au code exécuté. « dernier gain » est l'itération moyenne où le meilleur score a progressé pour la dernière fois ; comparez-la à « itér. » pour lire quand chaque run a stagné.",
    colVariant: "variante",
    colFamily: "famille",
    colDelta: "le changement qu'elle ajoute à son parent",
    colMean: "moy.",
    colLift: "gain",
    colLastBest: "dernier gain",
    colIters: "itér.",
    colAccept: "accept.",
    tblStatsTitle: "Chaque variante, chaque statistique relevée",
    busy: "Tracé…",
  },
  es: {
    boardTitle: "La clasificación — puntuación final media por variante",
    boardIntro:
      "Puntuación media (aristas coincidentes) del tablero con el que terminó cada variante, sobre diez variantes con esquinas fijadas del puzzle oficial, un solo núcleo, 60 s por ejecución. El color marca la familia, nombrada en cada barra: el color nunca es la única señal.",
    viewBars: "media",
    viewSpread: "dispersión",
    spreadIntro:
      "Las mismas ejecuciones, pero la puntuación final de cada instancia se muestra como una marca, con la media señalada. La barra de media oculta cuánto oscila cada variante de una instancia a otra; esto lo muestra. Donde dos variantes quedan cerca en la media, la dispersión es la lectura justa de si de verdad difieren.",
    spreadAxis: "puntuación final por instancia (aristas coincidentes, sobre 480)",
    spreadMean: "media",
    spreadRange: "rango",
    spreadInstances: "instancias",
    of: "/ 480",
    liftTitle: "Lo que el bucle añadió sobre su tablero de partida",
    liftIntro:
      "Las mismas ejecuciones, pero mostrando solo la mejora: la ganancia media que cada variante logró sobre el tablero del que partió. Esto aísla la contribución propia del bucle de reparación de la construcción con la que empezó — una variante que parte alto puede añadir poco y aun así terminar alto.",
    liftAxis: "mejora media (aristas coincidentes)",
    stallTitle: "Dónde se detiene la mejora",
    stallIntro:
      "Para cuatro variantes representativas, la mejor puntuación hasta el momento frente al número de iteraciones (muestreada cada 200 iteraciones, medianas sobre las diez instancias). El sentido del estudio en una sola imagen: sobre un buen tablero de partida, el bucle guiado por conflictos se aplana en pocos miles de iteraciones y los cientos de miles restantes no mueven nada, mientras que la destrucción aleatoria ciega sigue encontrando ganancias durante mucho más tiempo.",
    stallAxis: "iteraciones",
    stallY: "mejor puntuación hasta el momento",
    matrixTitle: "Qué se apila sobre qué",
    matrixIntro:
      "Cada variante es el mismo bucle de destrucción y reparación declarado como un único cambio sobre su padre. Esta tabla se genera a partir del registro del motor, por lo que siempre coincide con el código que se ejecutó. «último mejor» es la iteración media en la que el mejor global mejoró por última vez; compárala con «iter.» para leer cuán pronto se estancó cada ejecución.",
    colVariant: "variante",
    colFamily: "familia",
    colDelta: "el único cambio que añade sobre su padre",
    colMean: "media",
    colLift: "mejora",
    colLastBest: "último mejor",
    colIters: "iter.",
    colAccept: "acept.",
    tblStatsTitle: "Cada variante, cada estadística registrada",
    busy: "Dibujando…",
  },
};

export function RepairStudyLeaderboard() {
  const t = useT(T);
  const { lang } = useLang();
  const isClient = useIsClient();
  const [view, setView] = useState<"bars" | "spread">("bars");
  const familyLabel = (f: string) => (FAMILY[f] ? pick(FAMILY[f], lang) : f);

  const scored = useMemo(
    () =>
      D.variants
        .filter((v) => v.mean != null)
        .slice()
        .sort((a, b) => (b.mean ?? 0) - (a.mean ?? 0)),
    [],
  );

  const byLift = useMemo(
    () =>
      D.variants
        .filter((v) => v.mean_lift != null)
        .slice()
        .sort((a, b) => (b.mean_lift ?? 0) - (a.mean_lift ?? 0)),
    [],
  );
  // The lift chart previously let recharts auto-scale the x-axis (no explicit
  // domain); the shared chart needs an explicit max, so round the largest lift
  // up to a tidy tick to keep the same visual headroom.
  const liftMax = useMemo(() => {
    const max = Math.max(0, ...byLift.map((v) => v.mean_lift ?? 0));
    return Math.ceil(max / 20) * 20 || 1;
  }, [byLift]);

  // The convergence-curve series. Each variant has its own [iter, score] pairs
  // with independent iteration positions, so we give each Line its own data
  // rather than merging on a shared index. Points are {iter, score}; the shared
  // log x-axis spans the widest iteration range across the picks.
  const curveData = useMemo(() => {
    const picks = CURVE_KEYS.map((k) => D.variants.find((v) => v.name === k)).filter(
      (v): v is Variant => !!v && Array.isArray(v.curve) && v.curve.length > 0,
    );
    const series = picks.map((v) => ({
      variant: v,
      points: (v.curve as [number, number][]).map(([iter, score]) => ({ iter, score })),
    }));
    const allIters = series.flatMap((s) => s.points.map((p) => p.iter));
    const allScores = series.flatMap((s) => s.points.map((p) => p.score));
    const iterMin = allIters.length ? Math.min(...allIters) : 1;
    const iterMax = allIters.length ? Math.max(...allIters) : 1;
    const scoreMin = allScores.length ? Math.min(...allScores) : 0;
    const scoreMax = allScores.length ? Math.max(...allScores) : 1;
    return { series, picks, iterMin, iterMax, scoreMin, scoreMax };
  }, []);

  return (
    <div className="not-prose space-y-10">
      {/* 1. Leaderboard: aggregate mean bars, or the per-instance spread. */}
      <section>
        <div className="flex flex-wrap items-start justify-between gap-3">
          <h3 className="text-base font-semibold tracking-tight">{t.boardTitle}</h3>
          <ViewToggle
            value={view}
            onChange={setView}
            barsLabel={t.viewBars}
            spreadLabel={t.viewSpread}
          />
        </div>
        <p className="mt-1 text-sm leading-relaxed text-muted-foreground">
          {view === "bars" ? t.boardIntro : t.spreadIntro}
        </p>
        <div className="mt-4">
          {view === "bars" ? (
            <HorizontalScoreChart
              rows={scored}
              valueKey="mean"
              domainMax={480}
              colorOf={(v) => familyFill(v.family)}
              busyLabel={t.busy}
              tooltip={(v) => (
                <div className="rounded-md border bg-popover px-3 py-2 text-xs shadow-md">
                  <div className="font-semibold">{v.display}</div>
                  <div className="mt-1 text-muted-foreground">{familyLabel(v.family)}</div>
                  <div className="mt-1">
                    mean {v.mean} · best {v.best} · worst {v.worst} {t.of}
                  </div>
                  <div className="text-muted-foreground">
                    lift +{v.mean_lift} · {formatKM(v.mean_iterations, 1000)} iters
                  </div>
                </div>
              )}
            />
          ) : (
            <VariantScoreSpread
              rows={scored}
              familyFill={familyFill}
              labels={{
                axis: t.spreadAxis,
                mean: t.spreadMean,
                range: t.spreadRange,
                instances: t.spreadInstances,
              }}
            />
          )}
        </div>
        <FamilyLegend families={FAMILY} lang={lang} />
      </section>

      {/* 2. Lift: the loop's own contribution */}
      <section>
        <h3 className="text-base font-semibold tracking-tight">{t.liftTitle}</h3>
        <p className="mt-1 text-sm leading-relaxed text-muted-foreground">{t.liftIntro}</p>
        <div className="mt-4">
          <HorizontalScoreChart
            rows={byLift}
            valueKey="mean_lift"
            domainMax={liftMax}
            colorOf={(v) => familyFill(v.family)}
            busyLabel={t.busy}
            tooltip={(v) => (
              <div className="rounded-md border bg-popover px-3 py-2 text-xs shadow-md">
                <div className="font-semibold">{v.display}</div>
                <div className="mt-1">
                  start {v.start_score} → {v.mean} (lift +{v.mean_lift})
                </div>
              </div>
            )}
          />
        </div>
        <p className="mt-2 text-xs text-muted-foreground">{t.liftAxis}</p>
      </section>

      {/* 3. The stall: convergence curves */}
      <section>
        <h3 className="text-base font-semibold tracking-tight">{t.stallTitle}</h3>
        <p className="mt-1 text-sm leading-relaxed text-muted-foreground">{t.stallIntro}</p>
        <div className="mt-4 h-[420px] w-full">
          {isClient ? (
            <ResponsiveContainer width="100%" height="100%">
              <LineChart accessibilityLayer margin={{ top: 8, right: 24, bottom: 24, left: 8 }}>
                <XAxis
                  dataKey="iter"
                  type="number"
                  scale="log"
                  allowDataOverflow
                  domain={[curveData.iterMin, curveData.iterMax]}
                  tickFormatter={(n: number) => formatKM(n, 1000)}
                  tick={{ fontSize: 11, fill: "currentColor" }}
                  className="text-muted-foreground"
                  label={{ value: t.stallAxis, position: "insideBottom", offset: -12, fontSize: 11, fill: "currentColor" }}
                />
                <YAxis
                  dataKey="score"
                  type="number"
                  tick={{ fontSize: 11, fill: "currentColor" }}
                  className="text-muted-foreground"
                  domain={[curveData.scoreMin - 5, curveData.scoreMax + 5]}
                  width={44}
                />
                <Tooltip
                  content={({ active, payload }) => {
                    if (!active || !payload?.length) return null;
                    const point = payload[0]?.payload as { iter?: number } | undefined;
                    const iter = point?.iter;
                    return (
                      <div className="rounded-md border bg-popover px-3 py-2 text-xs shadow-md">
                        <div className="font-semibold">
                          {t.stallAxis} {formatKM(Number(iter), 1000)}
                        </div>
                        {payload.map((p) => (
                          <div key={String(p.name)} className="mt-0.5" style={{ color: p.color }}>
                            {p.name}: {p.value}
                          </div>
                        ))}
                      </div>
                    );
                  }}
                />
                {curveData.series.map((s) => (
                  <Line
                    key={s.variant.name}
                    data={s.points}
                    name={s.variant.display}
                    type="monotone"
                    dataKey="score"
                    stroke={familyFill(s.variant.family)}
                    strokeWidth={2}
                    dot={false}
                    isAnimationActive={false}
                  />
                ))}
              </LineChart>
            </ResponsiveContainer>
          ) : (
            <Busy label={t.busy} />
          )}
        </div>
        <div className="mt-3 flex flex-wrap gap-x-4 gap-y-1.5 text-xs text-muted-foreground">
          {curveData.picks.map((v) => (
            <span key={v.name} className="inline-flex items-center gap-1.5">
              <span className="inline-block h-2.5 w-2.5 rounded-sm" style={{ backgroundColor: familyFill(v.family) }} aria-hidden />
              {v.display}
            </span>
          ))}
        </div>
      </section>

      {/* 4. The what-stacks-on-what matrix */}
      <section>
        <h3 className="text-base font-semibold tracking-tight">{t.matrixTitle}</h3>
        <p className="mt-1 text-sm leading-relaxed text-muted-foreground">{t.matrixIntro}</p>
        <div className="mt-4 overflow-x-auto">
          <table className="w-full min-w-[720px] border-collapse text-sm">
            <thead>
              <tr className="border-b text-left text-xs text-muted-foreground">
                <th className="py-2 pr-4 font-medium">{t.colVariant}</th>
                <th className="py-2 pr-4 font-medium">{t.colFamily}</th>
                <th className="py-2 pr-4 font-medium">{t.colDelta}</th>
                <th className="py-2 pr-2 text-right font-medium">{t.colMean}</th>
                <th className="py-2 pr-2 text-right font-medium">{t.colLift}</th>
                <th className="py-2 pr-2 text-right font-medium">{t.colLastBest}</th>
                <th className="py-2 pr-2 text-right font-medium">{t.colIters}</th>
                <th className="py-2 text-right font-medium">{t.colAccept}</th>
              </tr>
            </thead>
            <tbody>
              {D.variants.map((v) => (
                <tr key={v.name} className="border-b align-top last:border-0">
                  <td className="py-2 pr-4 font-medium">{v.display}</td>
                  <td className="py-2 pr-4">
                    <FamilyTag color={familyFill(v.family)} label={familyLabel(v.family)} />
                  </td>
                  <td className="py-2 pr-4 text-muted-foreground">{v.delta}</td>
                  <td className="py-2 pr-2 text-right tabular-nums">{v.mean ?? "—"}</td>
                  <td className="py-2 pr-2 text-right tabular-nums">
                    {v.mean_lift == null ? "—" : `+${v.mean_lift}`}
                  </td>
                  <td className="py-2 pr-2 text-right tabular-nums text-muted-foreground">
                    {formatKM(v.mean_last_best_iter, 1000)}
                  </td>
                  <td className="py-2 pr-2 text-right tabular-nums text-muted-foreground">
                    {formatKM(v.mean_iterations, 1000)}
                  </td>
                  <td className="py-2 text-right tabular-nums text-muted-foreground">
                    {v.accept_rate ?? "—"}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </section>
    </div>
  );
}

function Busy({ label }: { label: string }) {
  return (
    <div className="flex h-full items-center justify-center text-sm text-muted-foreground">{label}</div>
  );
}

