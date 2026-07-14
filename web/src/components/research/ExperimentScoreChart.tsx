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

export type ScoreDatum = { key: string; label: string; score: number; family: string };

const CEILING = 470;
const DOMAIN_LO = 430;
const DOMAIN_HI = 480;

// One hue per family, matching the family groupings on the Hub.
const FAMILY_FILL: Record<string, string> = {
  scratch: "#38bdf8", // sky
  corpus: "#a78bfa", // violet
  concentrate: "#fbbf24", // amber
  anchor: "#34d399", // emerald
  exact: "#fb7185", // rose
  decode: "#60a5fa", // blue
};

const T = {
  en: {
    title: "Every experiment, by best score",
    intro:
      "Bars are the best board each method produced; the amber line is the community ceiling (470), the right edge is a full solution (480). Colour marks the family.",
    ceiling: "community ceiling 470",
    busy: "Drawing…",
    tip: "/ 480",
  },
  fr: {
    title: "Chaque expérience, par meilleur score",
    intro:
      "Les barres sont le meilleur plateau produit par chaque méthode ; la ligne ambre est le plafond communautaire (470), le bord droit une solution complète (480). La couleur marque la famille.",
    ceiling: "plafond communautaire 470",
    busy: "Tracé…",
    tip: "/ 480",
  },
};

export function ExperimentScoreChart({ data }: { data: ScoreDatum[] }) {
  const t = useT(T);
  const isClient = useIsClient();
  // Tallest score first reads as a ladder down from the ceiling.
  const sorted = [...data].sort((a, b) => b.score - a.score);
  const height = Math.max(220, sorted.length * 34 + 40);

  return (
    <section className="space-y-3">
      <div className="mx-auto max-w-3xl">
        <h2 className="text-2xl font-semibold tracking-tight">{t.title}</h2>
        <p className="mt-1 text-sm text-muted-foreground">{t.intro}</p>
      </div>
      <div className="mx-auto max-w-3xl rounded-lg border p-3" style={{ height }} role="img" aria-label={`${t.title}. ${t.intro}`}>
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
                tickCount={6}
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
                formatter={(v) => [`${String(v)} ${t.tip}`, ""]}
              />
              {/* Ceiling sits near the right edge; anchor the label to the end
                  so it grows leftward and never clips. Top margin gives it room. */}
              <ReferenceLine
                x={CEILING}
                stroke="#f59e0b"
                strokeDasharray="4 3"
                label={{
                  value: t.ceiling,
                  position: "insideTopRight",
                  fontSize: 10,
                  fill: "#f59e0b",
                  textAnchor: "end",
                  dy: -2,
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
            {t.busy}
          </div>
        )}
      </div>
    </section>
  );
}
