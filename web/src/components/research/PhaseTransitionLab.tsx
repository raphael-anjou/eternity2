import { useState } from "react";
import {
  CartesianGrid,
  Line,
  LineChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { LocalizedLink } from "@/components/LocalizedLink";
import stats from "@/data/difficulty.json";

// The difficulty wall, drawn from the same measurements the Algorithms page
// charts (web/src/data/difficulty.json, produced by the engine's stats binary):
// the median work to solve a generated puzzle, by board size and number of
// colors. Here it's framed for the phase-transition story. No re-solving in the
// browser; this is measured data, shown.

interface Row {
  size: number;
  colors: number;
  median: number;
  censored: number;
}
const rows = stats.difficulty as Row[];

// Sizes worth showing the wall for (3x3/4x4 are near-flat).
const SIZES = [5, 6, 7];
const COLOR_MIN = 2;
const COLOR_MAX = 14; // the measured range

// Build one chart series per size: colors -> median work.
function seriesFor(size: number) {
  return rows
    .filter((r) => r.size === size && r.colors >= COLOR_MIN && r.colors <= COLOR_MAX)
    .sort((a, b) => a.colors - b.colors)
    .map((r) => ({ colors: r.colors, work: Math.max(1, r.median) }));
}

function colorFor(size: number): string {
  if (size === 5) return "#94a3b8";
  if (size === 6) return "#38bdf8";
  return "#f59e0b";
}

const T = {
  en: {
    title: "The wall, measured",
    intro:
      "This is the same data the Algorithms page charts: the median work to solve a generated puzzle, by board size and number of colors. The y-axis is logarithmic, so each gridline is ten times the one below. Read it as the puzzle's difficulty knob.",
    read: "On a 7x7 board the work climbs past sixty million around five or six colors, then falls away by a factor of a hundred thousand as colors increase. The bigger the board, the sharper and further-right the spike. The full 16x16 lives far up and to the right of anything you can chart here, which is why its 17-color setting sits squarely in the hard region.",
    yLabel: "work to solve (median, log)",
    xLabel: "colors",
    size: "Board size",
    note: "Work is the number of placements the solver makes; values are medians over many generated puzzles. Points at the scarce-color end are lower bounds where a run was stopped early.",
    seeAlso: "See the full difficulty charts on the Algorithms page",
    busy: "Loading chart…",
  },
  fr: {
    title: "Le mur, mesuré",
    intro:
      "Voici les mêmes données que la page Algorithmes : le travail médian pour résoudre un puzzle généré, selon la taille du plateau et le nombre de couleurs. L'axe vertical est logarithmique : chaque ligne vaut dix fois celle du dessous. Lisez-le comme le bouton de difficulté du puzzle.",
    read: "Sur un plateau 7x7, le travail dépasse les soixante millions autour de cinq ou six couleurs, puis chute d'un facteur cent mille à mesure que les couleurs augmentent. Plus le plateau est grand, plus le pic est net et décalé vers la droite. Le 16x16 complet se situe bien plus haut et à droite que tout ce qu'on peut tracer ici, et c'est pourquoi son réglage à 17 couleurs tombe en pleine région difficile.",
    yLabel: "travail pour résoudre (médiane, log)",
    xLabel: "couleurs",
    size: "Taille du plateau",
    note: "Le travail est le nombre de placements effectués par le solveur ; les valeurs sont des médianes sur de nombreux puzzles générés. Les points du côté pauvre en couleurs sont des bornes inférieures là où une exécution a été arrêtée tôt.",
    seeAlso: "Voir les graphiques de difficulté complets sur la page Algorithmes",
    busy: "Chargement du graphique…",
  },
};

export function PhaseTransitionLab() {
  const t = useT(T);
  const isClient = useIsClient();
  const [shown, setShown] = useState<number[]>([...SIZES]);

  const toggle = (s: number) =>
    setShown((cur) => (cur.includes(s) ? cur.filter((x) => x !== s) : [...cur, s].sort()));

  return (
    <div className="space-y-4">
      <p className="max-w-3xl text-sm text-muted-foreground">{t.intro}</p>

      <div className="flex flex-wrap items-center gap-2">
        <span className="text-sm text-muted-foreground">{t.size}:</span>
        {SIZES.map((s) => (
          <button
            key={s}
            onClick={() => toggle(s)}
            className={
              "rounded-md border px-3 py-1 text-sm font-medium transition-colors " +
              (shown.includes(s)
                ? "bg-primary text-primary-foreground"
                : "text-muted-foreground hover:bg-muted hover:text-foreground")
            }
            style={shown.includes(s) ? { backgroundColor: colorFor(s), borderColor: colorFor(s) } : undefined}
          >
            {s}×{s}
          </button>
        ))}
      </div>

      <div className="h-96 w-full rounded-lg border p-2" role="img" aria-label={`${t.title}. ${t.intro}`}>
        {isClient ? (
          <ResponsiveContainer width="100%" height="100%" minWidth={0} minHeight={0}>
            <LineChart accessibilityLayer margin={{ top: 10, right: 16, bottom: 18, left: 8 }}>
              <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
              <XAxis
                dataKey="colors"
                type="number"
                domain={[COLOR_MIN, COLOR_MAX]}
                tickCount={7}
                allowDuplicatedCategory={false}
                label={{ value: t.xLabel, position: "insideBottom", offset: -8, fontSize: 12 }}
                fontSize={12}
              />
              <YAxis
                scale="log"
                domain={[1, "dataMax"]}
                allowDataOverflow
                tickFormatter={(n) => {
                  const v = Number(n);
                  if (v >= 1_000_000) return `${Math.round(v / 1_000_000)}M`;
                  if (v >= 1000) return `${Math.round(v / 1000)}k`;
                  return String(v);
                }}
                width={48}
                fontSize={12}
              />
              <Tooltip
                formatter={(v) => [Number(v).toLocaleString(), t.yLabel]}
                labelFormatter={(c) => `${String(c)} ${t.xLabel}`}
              />
              {SIZES.filter((s) => shown.includes(s)).map((s) => (
                <Line
                  key={s}
                  data={seriesFor(s)}
                  dataKey="work"
                  name={`${s}×${s}`}
                  stroke={colorFor(s)}
                  strokeWidth={2}
                  dot={{ r: 2 }}
                  isAnimationActive={false}
                />
              ))}
            </LineChart>
          </ResponsiveContainer>
        ) : (
          <div className="flex h-full items-center justify-center text-sm text-muted-foreground">
            {t.busy}
          </div>
        )}
      </div>

      <p className="max-w-3xl text-sm text-muted-foreground">{t.read}</p>
      <p className="max-w-3xl text-xs text-muted-foreground">{t.note}</p>
      <LocalizedLink
        to="/algorithms"
        className="inline-block text-sm font-medium underline hover:text-foreground"
      >
        {t.seeAlso}
      </LocalizedLink>
    </div>
  );
}
