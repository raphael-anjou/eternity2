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
import { useT, useLang } from "@/i18n";
import { useIsClient } from "@/lib/utils";
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
// 461 record and the community 470 ceiling. Throughput units differ by family
// and are never cross-compared; each row shows its own native unit.

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
};

const D = data as {
  budgetS: number;
  maxScore: number;
  record: number;
  community: number;
  variantIds: number[];
  algos: Algo[];
};

// One hue per family, reused from the site's existing family palette
// (ExperimentScoreChart) so the research pages read as one system.
const FAMILY: Record<string, { fill: string; en: string; fr: string }> = {
  producer: { fill: "#a78bfa", en: "beam producer", fr: "producteur (beam)" },
  alns: { fill: "#34d399", en: "ALNS local search", fr: "recherche locale ALNS" },
  backtracker: { fill: "#fb7185", en: "backtracker", fr: "backtracking" },
  csp: { fill: "#38bdf8", en: "CSP + AC-3", fr: "CSP + AC-3" },
  naive: { fill: "#fbbf24", en: "naive DFS", fr: "DFS naïf" },
};
const familyFill = (f: string) => FAMILY[f]?.fill ?? "#94a3b8";

// Score domain: start below the weakest so every bar has visible length, end at
// the 480 maximum so the record/ceiling ticks sit in a true-to-scale position.
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
      "Mean score over ten corner-pinned variants, single core, 60 s each. The dark segment marks each engine's worst-to-best range; colour marks the family.",
    record: "our 461",
    ceiling: "community 470",
    npsUnit: "median throughput (native unit, never cross-compared)",
    heatTitle: "Every algorithm across all ten corner variants",
    heatIntro:
      "One 60 s run per cell. The producer is flat at 453–456; the CSP engines are bimodal, collapsing to ~55 on hostile corners.",
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
      "Score moyen sur dix variantes à coins fixés, un cœur, 60 s chacune. Le segment sombre marque l'écart pire-au-meilleur de chaque moteur ; la couleur marque la famille.",
    record: "notre 461",
    ceiling: "communauté 470",
    npsUnit: "débit médian (unité native, jamais comparée entre familles)",
    heatTitle: "Chaque algorithme sur les dix variantes de coins",
    heatIntro:
      "Un run de 60 s par cellule. Le producteur est stable à 453–456 ; les moteurs CSP sont bimodaux, s'effondrant à ~55 sur les coins hostiles.",
    busy: "Tracé…",
    legendTitle: "famille",
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
function heatColor(v: number | null): string {
  if (v == null) return "var(--muted)";
  const t = Math.max(0, Math.min(1, (v - 40) / (456 - 40)));
  // emerald ramp: light (low) → dark (high), via alpha over the surface.
  return `color-mix(in srgb, #10b981 ${12 + t * 78}%, transparent)`;
}

export function BenchmarkLeaderboard() {
  const t = useT(T);
  const { lang } = useLang();
  const isClient = useIsClient();
  const algos = D.algos;
  const height = Math.max(320, algos.length * 32 + 48);

  const blackwood = algos.find((a) => a.algo === "blackwood");
  const producer = algos.find((a) => a.algo === "producer");

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

      {/* 2. the leaderboard */}
      <div>
        <h2 className="text-xl font-semibold tracking-tight">{t.boardTitle}</h2>
        <p className="mb-1 mt-1 text-sm text-muted-foreground">{t.boardIntro}</p>
        {/* legend: family identity, always present (>2 series) */}
        <div className="mb-2 flex flex-wrap gap-x-4 gap-y-1 font-mono text-xs text-muted-foreground">
          {familiesPresent.map((f) => (
            <span key={f} className="inline-flex items-center gap-1.5">
              <i
                aria-hidden
                className="inline-block h-2.5 w-2.5 rounded-sm"
                style={{ background: familyFill(f) }}
              />
              {FAMILY[f] ? (lang === "fr" ? FAMILY[f].fr : FAMILY[f].en) : f}
            </span>
          ))}
        </div>
        <div
          className="rounded-lg border p-3"
          style={{ height }}
          role="img"
          aria-label={`${t.boardTitle}. ${t.boardIntro}`}
        >
          {isClient ? (
            <ResponsiveContainer width="100%" height="100%" minWidth={0} minHeight={0}>
              <BarChart
                layout="vertical"
                data={algos}
                margin={{ top: 6, right: 44, bottom: 6, left: 8 }}
                barCategoryGap={6}
              >
                <XAxis
                  type="number"
                  domain={[DOMAIN_LO, D.maxScore]}
                  tickCount={7}
                  fontSize={11}
                  allowDecimals={false}
                />
                <YAxis
                  type="category"
                  dataKey="algo"
                  width={128}
                  fontSize={11}
                  tickLine={false}
                  axisLine={false}
                />
                <Tooltip
                  cursor={{ fill: "transparent" }}
                  formatter={(_v, _n, item) => {
                    const a = item.payload as Algo;
                    return [
                      `mean ${a.mean} · range ${a.worst}–${a.best} · σ${a.std} · ${fmtNps(a.nps, a.npsUnit)}`,
                      a.algo,
                    ];
                  }}
                />
                <ReferenceLine
                  x={D.record}
                  stroke="#a78bfa"
                  strokeDasharray="4 3"
                  label={{ value: t.record, position: "top", fontSize: 10, fill: "#a78bfa" }}
                />
                <ReferenceLine
                  x={D.community}
                  stroke="#f59e0b"
                  strokeDasharray="4 3"
                  label={{ value: t.ceiling, position: "top", fontSize: 10, fill: "#f59e0b" }}
                />
                <Bar dataKey="mean" radius={[0, 4, 4, 0]} isAnimationActive={false}>
                  <LabelList
                    dataKey="mean"
                    position="right"
                    fontSize={11}
                    className="fill-foreground"
                  />
                  {algos.map((a) => (
                    <Cell key={a.algo} fill={familyFill(a.family)} />
                  ))}
                </Bar>
              </BarChart>
            </ResponsiveContainer>
          ) : (
            <div className="flex h-full items-center justify-center text-sm text-muted-foreground">
              {t.busy}
            </div>
          )}
        </div>
      </div>

      {/* 3. the per-variant heatmap across all algos */}
      <div>
        <h2 className="text-xl font-semibold tracking-tight">{t.heatTitle}</h2>
        <p className="mb-2 mt-1 text-sm text-muted-foreground">{t.heatIntro}</p>
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
