import { useMemo } from "react";
import {
  Bar,
  BarChart,
  Cell,
  LabelList,
  Line,
  LineChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { useT, useLang } from "@/i18n";
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
const FAMILY: Record<string, { fill: string; en: string; fr: string }> = {
  start: { fill: "#f59e0b", en: "starting board", fr: "plateau de départ" },
  destroy: { fill: "#3b82f6", en: "destroy operator", fr: "opérateur de destruction" },
  repair: { fill: "#10b981", en: "repair", fr: "réparation" },
  accept: { fill: "#8b5cf6", en: "acceptance", fr: "acceptation" },
  restart: { fill: "#ef4444", en: "restart", fr: "redémarrage" },
};
const familyFill = (f: string) => FAMILY[f]?.fill ?? "#94a3b8";

// The variants whose convergence curve the stall figure draws: the surprising
// winner that keeps improving (random-destroy), the strongest acceptance rule
// (accept-anneal), the plain conflict-driven loop that stalls early
// (greedy-mismatch), and the component destroy that collapses almost at once.
// Kept small so the chart stays legible; every variant's numbers are in the table.
const CURVE_KEYS = ["random-destroy", "accept-anneal", "greedy-mismatch", "component-destroy"];

function fmtInt(n: number | null | undefined): string {
  if (n == null) return "—";
  if (n >= 1e6) return `${(n / 1e6).toFixed(1)}M`;
  if (n >= 1e3) return `${Math.round(n / 1e3)}K`;
  return `${Math.round(n)}`;
}

const T = {
  en: {
    boardTitle: "The leaderboard — mean final score by variant",
    boardIntro:
      "Mean matched-edge score of the board each variant finished with, over ten corner-pinned variants of the official puzzle, single core, 60 s per run. Colour marks the family; the family is named on every bar, so colour is never the only signal.",
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
};

export function RepairStudyLeaderboard() {
  const t = useT(T);
  const { lang } = useLang();
  const isClient = useIsClient();
  const familyLabel = (f: string) => FAMILY[f]?.[lang] ?? f;

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
      {/* 1. Leaderboard: final score */}
      <section>
        <h3 className="text-base font-semibold tracking-tight">{t.boardTitle}</h3>
        <p className="mt-1 text-sm leading-relaxed text-muted-foreground">{t.boardIntro}</p>
        <div className="mt-4 h-[520px] w-full">
          {isClient ? (
            <ResponsiveContainer width="100%" height="100%">
              <BarChart layout="vertical" data={scored} margin={{ top: 8, right: 48, bottom: 8, left: 8 }}>
                <XAxis
                  type="number"
                  domain={[0, 480]}
                  tick={{ fontSize: 11, fill: "currentColor" }}
                  className="text-muted-foreground"
                />
                <YAxis
                  type="category"
                  dataKey="display"
                  width={150}
                  tick={{ fontSize: 11, fill: "currentColor" }}
                  className="text-muted-foreground"
                />
                <Tooltip
                  cursor={{ fill: "currentColor", opacity: 0.06 }}
                  content={({ active, payload }) => {
                    if (!active || !payload?.length) return null;
                    const v = payload[0]?.payload as Variant | undefined;
                    if (!v) return null;
                    return (
                      <div className="rounded-md border bg-popover px-3 py-2 text-xs shadow-md">
                        <div className="font-semibold">{v.display}</div>
                        <div className="mt-1 text-muted-foreground">{familyLabel(v.family)}</div>
                        <div className="mt-1">
                          mean {v.mean} · best {v.best} · worst {v.worst} {t.of}
                        </div>
                        <div className="text-muted-foreground">
                          lift +{v.mean_lift} · {fmtInt(v.mean_iterations)} iters
                        </div>
                      </div>
                    );
                  }}
                />
                <Bar dataKey="mean" radius={[0, 4, 4, 0]} isAnimationActive={false}>
                  {scored.map((v) => (
                    <Cell key={v.name} fill={familyFill(v.family)} />
                  ))}
                  <LabelList dataKey="mean" position="right" fontSize={11} className="fill-foreground" />
                </Bar>
              </BarChart>
            </ResponsiveContainer>
          ) : (
            <Busy label={t.busy} />
          )}
        </div>
        <FamilyLegend lang={lang} />
      </section>

      {/* 2. Lift: the loop's own contribution */}
      <section>
        <h3 className="text-base font-semibold tracking-tight">{t.liftTitle}</h3>
        <p className="mt-1 text-sm leading-relaxed text-muted-foreground">{t.liftIntro}</p>
        <div className="mt-4 h-[520px] w-full">
          {isClient ? (
            <ResponsiveContainer width="100%" height="100%">
              <BarChart layout="vertical" data={byLift} margin={{ top: 8, right: 48, bottom: 8, left: 8 }}>
                <XAxis
                  type="number"
                  tick={{ fontSize: 11, fill: "currentColor" }}
                  className="text-muted-foreground"
                />
                <YAxis
                  type="category"
                  dataKey="display"
                  width={150}
                  tick={{ fontSize: 11, fill: "currentColor" }}
                  className="text-muted-foreground"
                />
                <Tooltip
                  cursor={{ fill: "currentColor", opacity: 0.06 }}
                  content={({ active, payload }) => {
                    if (!active || !payload?.length) return null;
                    const v = payload[0]?.payload as Variant | undefined;
                    if (!v) return null;
                    return (
                      <div className="rounded-md border bg-popover px-3 py-2 text-xs shadow-md">
                        <div className="font-semibold">{v.display}</div>
                        <div className="mt-1">
                          start {v.start_score} → {v.mean} (lift +{v.mean_lift})
                        </div>
                      </div>
                    );
                  }}
                />
                <Bar dataKey="mean_lift" radius={[0, 4, 4, 0]} isAnimationActive={false}>
                  {byLift.map((v) => (
                    <Cell key={v.name} fill={familyFill(v.family)} />
                  ))}
                  <LabelList dataKey="mean_lift" position="right" fontSize={11} className="fill-foreground" />
                </Bar>
              </BarChart>
            </ResponsiveContainer>
          ) : (
            <Busy label={t.busy} />
          )}
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
              <LineChart margin={{ top: 8, right: 24, bottom: 24, left: 8 }}>
                <XAxis
                  dataKey="iter"
                  type="number"
                  scale="log"
                  allowDataOverflow
                  domain={[curveData.iterMin, curveData.iterMax]}
                  tickFormatter={(n: number) => fmtInt(n)}
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
                          {t.stallAxis} {fmtInt(Number(iter))}
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
                    <FamilyTag family={v.family} label={familyLabel(v.family)} />
                  </td>
                  <td className="py-2 pr-4 text-muted-foreground">{v.delta}</td>
                  <td className="py-2 pr-2 text-right tabular-nums">{v.mean ?? "—"}</td>
                  <td className="py-2 pr-2 text-right tabular-nums">
                    {v.mean_lift == null ? "—" : `+${v.mean_lift}`}
                  </td>
                  <td className="py-2 pr-2 text-right tabular-nums text-muted-foreground">
                    {fmtInt(v.mean_last_best_iter)}
                  </td>
                  <td className="py-2 pr-2 text-right tabular-nums text-muted-foreground">
                    {fmtInt(v.mean_iterations)}
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

function FamilyTag({ family, label }: { family: string; label: string }) {
  return (
    <span className="inline-flex items-center gap-1.5">
      <span
        className="inline-block h-2.5 w-2.5 shrink-0 rounded-sm"
        style={{ backgroundColor: familyFill(family) }}
        aria-hidden
      />
      <span className="text-muted-foreground">{label}</span>
    </span>
  );
}

function FamilyLegend({ lang }: { lang: "en" | "fr" }) {
  return (
    <div className="mt-3 flex flex-wrap gap-x-4 gap-y-1.5 text-xs text-muted-foreground">
      {Object.entries(FAMILY).map(([key, f]) => (
        <span key={key} className="inline-flex items-center gap-1.5">
          <span className="inline-block h-2.5 w-2.5 rounded-sm" style={{ backgroundColor: f.fill }} aria-hidden />
          {f[lang]}
        </span>
      ))}
    </div>
  );
}
