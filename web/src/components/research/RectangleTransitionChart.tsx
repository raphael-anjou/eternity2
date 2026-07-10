import {
  Area,
  AreaChart,
  CartesianGrid,
  ReferenceArea,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import data from "@/data/rectangle-transition.json";

// Brendan Owen's measured phase transition (groups.io msg 4909, 2008): tile a
// 2xL rectangle with random sets of interior E2 pieces, 400 sets per length,
// and count how many are solvable. The share solvable falls to zero for
// L = 4..15 then climbs steeply back from ~L = 18, an empirical S-curve.

const T = {
  en: {
    x: "rectangle length (2 x L)",
    y: "% of random piece sets solvable",
    dead: "dead zone: 0%",
    tip: "solvable",
    caption:
      "Each point tiles a 2xL rectangle with 400 random sets of interior pieces (60 at length 22) and reports the share that can be completed with no mismatch. Short strips are easy, then solutions vanish entirely from length 4 to 15, then reappear and climb steeply past 18: a phase transition measured directly, not modelled.",
    busy: "Loading chart…",
  },
  fr: {
    x: "longueur du rectangle (2 x L)",
    y: "% de jeux de pièces aléatoires résolubles",
    dead: "zone morte : 0 %",
    tip: "résolubles",
    caption:
      "Chaque point pave un rectangle 2xL avec 400 jeux de pièces intérieures tirés au hasard (60 à la longueur 22) et donne la proportion qui se complète sans défaut. Les bandes courtes sont faciles, puis les solutions disparaissent totalement de la longueur 4 à 15, avant de réapparaître et de grimper fortement au-delà de 18 : une transition de phase mesurée directement, non modélisée.",
    busy: "Chargement du graphique…",
  },
};

export function RectangleTransitionChart() {
  const t = useT(T);
  const isClient = useIsClient();

  if (!isClient) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  return (
    <div className="space-y-2">
      <div className="h-72 rounded-lg border p-2" role="img" aria-label={t.caption}>
        <ResponsiveContainer width="100%" height="100%" minWidth={0} minHeight={0}>
          <AreaChart data={data.rows} margin={{ top: 10, right: 16, bottom: 20, left: 8 }}>
            <defs>
              <linearGradient id="rectFill" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stopColor="#38bdf8" stopOpacity={0.7} />
                <stop offset="100%" stopColor="#38bdf8" stopOpacity={0.05} />
              </linearGradient>
            </defs>
            <CartesianGrid strokeDasharray="3 3" className="stroke-muted" vertical={false} />
            <ReferenceArea x1={4} x2={15} fill="#f43f5e" fillOpacity={0.06} label={{ value: t.dead, fontSize: 11, fill: "currentColor" }} />
            <XAxis
              dataKey="length"
              label={{ value: t.x, position: "insideBottom", offset: -10, fontSize: 12 }}
              fontSize={11}
            />
            <YAxis
              domain={[0, 100]}
              label={{ value: t.y, angle: -90, position: "insideLeft", fontSize: 11, style: { textAnchor: "middle" } }}
              width={48}
              fontSize={11}
              tickFormatter={(v) => `${v}%`}
            />
            <Tooltip
              formatter={(v) => [`${Number(v).toFixed(2)}%`, t.tip]}
              labelFormatter={(l) => `2 x ${String(l)}`}
            />
            <Area
              type="monotone"
              dataKey="percent"
              stroke="#0ea5e9"
              strokeWidth={2}
              fill="url(#rectFill)"
              dot={{ r: 2 }}
            />
          </AreaChart>
        </ResponsiveContainer>
      </div>
      <p className="mx-auto max-w-2xl text-center text-xs text-muted-foreground">{t.caption}</p>
    </div>
  );
}
