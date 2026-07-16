import { useMemo } from "react";
import {
  Bar,
  BarChart,
  ReferenceLine,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import data from "@/data/dataset-corpus.json";

// The score distribution of the 1,078-board strong-board corpus: one vertical
// bar per score value across the full 400..469 range, gaps included as zero-
// height bars so the shape is read honestly (a clean rise to a peak near 454,
// then a thin tail into the 460s). Data is derived from the dataset build's
// stats.json (web/src/data/dataset-corpus.json), so the chart never drifts from
// the published corpus.
//
// One measure, one hue: this is a magnitude histogram, so it uses a single
// sequential blue (no categorical palette, no legend). The peak bin and the
// start of the 460+ tail are marked with reference lines to anchor the two
// facts the prose states, without a second colour (a blue/purple split fails
// the colourblind-separation check, so distinctions are shown by lines, not
// hue). Every bar carries its exact count in the tooltip.

type Bin = { score: number; count: number };

const D = data as {
  boards: number;
  score_min: number;
  score_max: number;
  distinct_families: number;
  histogram: Bin[];
};

const BAR_FILL = "#3b82f6";

// Scores at or above this are the notable strong tail the prose calls out.
const TAIL_FROM = 460;

const T = {
  en: {
    title: "How many boards score what",
    intro:
      "Every one of the 1,078 boards, counted by its score. One bar per score from 400 to 469; a missing score is a zero-height gap, so the true shape shows through: a steady climb to a peak in the mid-450s, then a thin tail of rare boards into the 460s. The dashed line marks where that 460-and-up tail begins.",
    xAxis: "score (matched edges, out of 480)",
    yAxis: "number of boards",
    peak: "peak",
    tail: "460+",
    boards: "boards",
    at: "at",
    busy: "Drawing…",
  },
  fr: {
    title: "Combien de plateaux à chaque score",
    intro:
      "Chacun des 1 078 plateaux, compté par son score. Une barre par score de 400 à 469 ; un score absent est un creux de hauteur nulle, si bien que la vraie forme apparaît : une montée régulière vers un pic au milieu des 450, puis une fine traîne de plateaux rares jusque dans les 460. La ligne pointillée marque le début de cette traîne à partir de 460.",
    xAxis: "score (arêtes appariées, sur 480)",
    yAxis: "nombre de plateaux",
    peak: "pic",
    tail: "460+",
    boards: "plateaux",
    at: "à",
    busy: "Tracé…",
  },
};

export function CorpusScoreDistribution() {
  const t = useT(T);
  const isClient = useIsClient();

  const peak = useMemo(
    () =>
      D.histogram.reduce<Bin>(
        (best, b) => (b.count > best.count ? b : best),
        { score: D.score_min, count: 0 },
      ),
    [],
  );

  return (
    <div className="not-prose my-6">
      <h3 className="text-base font-semibold tracking-tight">{t.title}</h3>
      <p className="mt-1 text-sm leading-relaxed text-muted-foreground">{t.intro}</p>
      <div className="mt-4 h-[360px] w-full">
        {isClient ? (
          <ResponsiveContainer width="100%" height="100%">
            <BarChart data={D.histogram} margin={{ top: 24, right: 12, bottom: 24, left: 8 }} barCategoryGap={1}>
              <XAxis
                dataKey="score"
                type="category"
                interval={4}
                tick={{ fontSize: 11, fill: "currentColor" }}
                className="text-muted-foreground"
                label={{
                  value: t.xAxis,
                  position: "insideBottom",
                  offset: -14,
                  fontSize: 11,
                  fill: "currentColor",
                }}
              />
              <YAxis
                tick={{ fontSize: 11, fill: "currentColor" }}
                className="text-muted-foreground"
                label={{
                  value: t.yAxis,
                  angle: -90,
                  position: "insideLeft",
                  offset: 12,
                  style: { textAnchor: "middle", fontSize: 11, fill: "currentColor" },
                }}
              />
              <Tooltip
                cursor={{ fill: "currentColor", opacity: 0.06 }}
                content={({ active, payload }) => {
                  if (!active || !payload?.length) return null;
                  const b = payload[0]?.payload as Bin | undefined;
                  if (!b) return null;
                  return (
                    <div className="rounded-md border bg-popover px-3 py-2 text-xs shadow-md">
                      <div className="font-semibold">
                        {b.count} {t.boards} {t.at} {b.score}
                      </div>
                    </div>
                  );
                }}
              />
              <ReferenceLine
                x={peak.score}
                stroke="currentColor"
                strokeDasharray="3 3"
                className="text-muted-foreground"
                label={{ value: `${t.peak} ${peak.score}`, position: "top", fontSize: 10, fill: "currentColor" }}
              />
              <ReferenceLine
                x={TAIL_FROM}
                stroke="currentColor"
                strokeDasharray="2 4"
                className="text-muted-foreground/70"
                label={{ value: t.tail, position: "top", fontSize: 10, fill: "currentColor" }}
              />
              <Bar dataKey="count" fill={BAR_FILL} radius={[2, 2, 0, 0]} isAnimationActive={false} />
            </BarChart>
          </ResponsiveContainer>
        ) : (
          <div className="flex h-full items-center justify-center text-sm text-muted-foreground">
            {t.busy}
          </div>
        )}
      </div>
    </div>
  );
}
