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
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";

// A single glance at the whole gallery: every experiment's best score on one
// axis, the 470 community ceiling and the 480 target marked. Bars are coloured
// by family so the "build from scratch" cluster, the lone corpus peak, etc. are
// legible. Data is passed in by the caller (the experiments hub MDX), so this
// stays a pure presentational chart with no separate source of truth.
//
// Scores split into two provenance groups that must never be conflated: the
// exploratory best boards (8-core, wall-clock not logged) and the standardized
// bench (one core, 60 s). Each renders as its own captioned panel so a reader
// never mistakes a 463 explored on 8 cores for a single-core-minute result.

export type ScoreDatum = {
  key: string;
  label: string;
  score: number;
  family: string;
  /** "explore" = 8-core exploratory best (wall-clock not logged); "bench" =
   *  standardized single-core, 60 s. Defaults to "explore". */
  group?: "explore" | "bench";
};

const CEILING = 470;
const DOMAIN_LO = 420;
const DOMAIN_HI = 480;

// One hue per family, matching the family groupings on the Hub.
const FAMILY_FILL: Record<string, string> = {
  scratch: "#38bdf8", // sky
  corpus: "#a78bfa", // violet
  concentrate: "#fbbf24", // amber
  anchor: "#34d399", // emerald
  exact: "#fb7185", // rose
  decode: "#60a5fa", // blue
  bench: "#94a3b8", // slate — the single-core baseline family
};

const T = {
  en: {
    title: "Every experiment, by best score",
    intro:
      "Bars are the best board each method produced; the amber line is the community ceiling (470), the right edge is a full solution (480).",
    exploreTitle: "Best boards found",
    exploreNote:
      "Exploratory runs on 8 cores; wall-clock not logged. These are the highest boards each method reached, the numbers the write-ups quote.",
    benchTitle: "On one core, 60 seconds each",
    benchNote:
      "The standardized single-core bench: the DFS and repair studies and the reimplemented engines, every run pinned to one core for sixty seconds. Lower, and directly comparable.",
    ceiling: "community ceiling 470",
    busy: "Drawing…",
    tip: "/ 480",
  },
  fr: {
    title: "Chaque expérience, par meilleur score",
    intro:
      "Les barres sont le meilleur plateau produit par chaque méthode ; la ligne ambre est le plafond communautaire (470), le bord droit une solution complète (480).",
    exploreTitle: "Meilleurs plateaux obtenus",
    exploreNote:
      "Exécutions exploratoires sur 8 cœurs ; temps non journalisé. Ce sont les meilleurs plateaux atteints par chaque méthode, les scores cités dans les articles.",
    benchTitle: "Sur un cœur, 60 secondes chacun",
    benchNote:
      "Le banc standardisé mono-cœur : les études DFS et réparation et les moteurs réimplémentés, chaque exécution limitée à un cœur pendant soixante secondes. Plus bas, et directement comparable.",
    ceiling: "plafond communautaire 470",
    busy: "Tracé…",
    tip: "/ 480",
  },
};

function ScorePanel({
  title,
  note,
  rows,
  tip,
  ceiling,
  busy,
  isClient,
}: {
  title: string;
  note: string;
  rows: ScoreDatum[];
  tip: string;
  ceiling: string;
  busy: string;
  isClient: boolean;
}) {
  // Tallest score first reads as a ladder down from the ceiling.
  const sorted = [...rows].sort((a, b) => b.score - a.score);
  const height = Math.max(180, sorted.length * 34 + 40);
  return (
    <div className="space-y-2">
      <div className="mx-auto max-w-3xl">
        <h3 className="text-lg font-semibold tracking-tight">{title}</h3>
        <p className="mt-0.5 text-sm text-muted-foreground">{note}</p>
      </div>
      <div
        className="mx-auto max-w-3xl rounded-lg border p-3"
        style={{ height }}
        role="img"
        aria-label={`${title}. ${note}`}
      >
        {isClient ? (
          <ResponsiveContainer width="100%" height="100%" minWidth={0} minHeight={0}>
            <BarChart
              layout="vertical"
              data={sorted}
              margin={{ top: 24, right: 40, bottom: 6, left: 8 }}
              barCategoryGap={8}
            >
              <XAxis
                type="number"
                domain={[DOMAIN_LO, DOMAIN_HI]}
                tickCount={7}
                fontSize={11}
                allowDecimals={false}
              />
              <YAxis
                type="category"
                dataKey="label"
                width={92}
                fontSize={12}
                tickLine={false}
                axisLine={false}
              />
              <Tooltip
                cursor={{ fill: "transparent" }}
                formatter={(v) => [`${String(v)} ${tip}`, ""]}
              />
              <ReferenceLine
                x={CEILING}
                stroke="#f59e0b"
                strokeDasharray="4 3"
                label={{
                  value: ceiling,
                  position: "top",
                  fontSize: 10,
                  fill: "#f59e0b",
                  textAnchor: "end",
                  dx: -2,
                }}
              />
              <Bar dataKey="score" radius={[0, 4, 4, 0]} isAnimationActive={false}>
                <LabelList dataKey="score" position="right" fontSize={11} className="fill-foreground" />
                {sorted.map((d) => (
                  <Cell key={d.key} fill={FAMILY_FILL[d.family] ?? "#94a3b8"} />
                ))}
              </Bar>
            </BarChart>
          </ResponsiveContainer>
        ) : (
          <div className="flex h-full items-center justify-center text-sm text-muted-foreground">
            {busy}
          </div>
        )}
      </div>
    </div>
  );
}

export function ExperimentScoreChart({ data }: { data: ScoreDatum[] }) {
  const t = useT(T);
  const isClient = useIsClient();

  const explore = data.filter((d) => (d.group ?? "explore") === "explore");
  const bench = data.filter((d) => d.group === "bench");

  return (
    <section className="space-y-5">
      <div className="mx-auto max-w-3xl">
        <h2 className="text-2xl font-semibold tracking-tight">{t.title}</h2>
        <p className="mt-1 text-sm text-muted-foreground">{t.intro}</p>
      </div>
      <ScorePanel
        title={t.exploreTitle}
        note={t.exploreNote}
        rows={explore}
        tip={t.tip}
        ceiling={t.ceiling}
        busy={t.busy}
        isClient={isClient}
      />
      {bench.length > 0 && (
        <ScorePanel
          title={t.benchTitle}
          note={t.benchNote}
          rows={bench}
          tip={t.tip}
          ceiling={t.ceiling}
          busy={t.busy}
          isClient={isClient}
        />
      )}
    </section>
  );
}
