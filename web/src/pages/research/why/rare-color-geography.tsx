import { pageMeta } from "@/seo";
import {
  Bar,
  BarChart,
  CartesianGrid,
  Legend,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { LocalizedLink } from "@/components/LocalizedLink";
import { RelatedRail } from "@/components/research/RelatedRail";
import { MotifSwatch } from "@/components/board/MotifSwatch";
import { colorToLetter } from "@/lib/motifs";
import data from "@/data/rare-color-geography.json";

const ARTICLE_URL =
  "https://github.com/raphael-anjou/eternity2/tree/main/research/topics/rare-color-geography";

const rareColors = data.rareColors;
const chartData = data.colors.map((c) => ({
  color: c.color,
  frame: c.frame,
  interior: c.interior,
}));

// A 16x16 schematic: outer ring = frame (where rare colors live), inside = interior.
const RING_CELLS: { x: number; y: number; frame: boolean }[] = [];
for (let r = 0; r < 16; r++) {
  for (let c = 0; c < 16; c++) {
    RING_CELLS.push({ x: c, y: r, frame: r === 0 || r === 15 || c === 0 || c === 15 });
  }
}

const T = {
  en: {
    backLabel: "← Why it's hard",
    title: "The rare colors live on the frame",
    lede: "Five of Eternity II's 22 colors appear only along the border ring, each on exactly 24 edges, never once in the interior.",
    p1: "The 22 colors don't play equal roles. Sort them by where their edges sit and they split into two clean classes: five rare colors fenced off to the border, and seventeen common colors that do almost all their work inside the board.",
    ringTitle: "Where the rare colors are allowed",
    ringNote: "The outer ring is the 60-piece border. Every edge carrying one of the five rare colors sits there. The interior never shows a rare color.",
    swatchTitle: "The five rare colors",
    statEdges: "edges each",
    statTotal: "rare edges, all on the frame",
    statInterior: "rare edges in the interior",
    chartTitle: "Frame vs interior, color by color",
    chartNote: "Colors 1 to 5 are pure frame. The rest are overwhelmingly interior.",
    frame: "frame",
    interior: "interior",
    colorAxis: "color",
    edgesAxis: "edges",
    whyTitle: "Why it matters",
    why: "This is design you can see. Eternity I fell because its pieces had exploitable statistical regularities; Eternity II's colors were balanced on purpose to remove those handholds. Fencing the scarce colors to the border keeps them away from the vast interior, where an attacker would most want a rare, highly-constraining signal to grab. The interior is left to seventeen common colors, which is exactly why interior search has so little to grip. The border, where the rare colors make matches tight, is why every strong board gets its border essentially perfect.",
    related: "See why the interior gives no forced moves",
    reproTitle: "Reproduce it",
    reproBody: "Computed exactly from the official set; reproduces identically every run.",
    sourceLink: "Article, source and results on GitHub",
  },
  fr: {
    backLabel: "← Pourquoi c'est dur",
    title: "Les couleurs rares vivent sur le cadre",
    lede: "Cinq des 22 couleurs d'Eternity II n'apparaissent que le long du cadre, chacune sur exactement 24 bords, jamais une seule fois à l'intérieur.",
    p1: "Les 22 couleurs ne jouent pas des rôles égaux. Triez-les selon où se trouvent leurs bords et elles se séparent en deux classes nettes : cinq couleurs rares cantonnées au bord, et dix-sept couleurs communes qui font presque tout le travail à l'intérieur du plateau.",
    ringTitle: "Où les couleurs rares sont autorisées",
    ringNote: "L'anneau extérieur est la bordure de 60 pièces. Chaque bord portant l'une des cinq couleurs rares s'y trouve. L'intérieur ne montre jamais de couleur rare.",
    swatchTitle: "Les cinq couleurs rares",
    statEdges: "bords chacune",
    statTotal: "bords rares, tous sur le cadre",
    statInterior: "bords rares à l'intérieur",
    chartTitle: "Cadre contre intérieur, couleur par couleur",
    chartNote: "Les couleurs 1 à 5 sont purement cadre. Les autres sont massivement intérieures.",
    frame: "cadre",
    interior: "intérieur",
    colorAxis: "couleur",
    edgesAxis: "bords",
    whyTitle: "Pourquoi c'est important",
    why: "C'est une conception visible. Eternity I est tombé car ses pièces avaient des régularités statistiques exploitables ; les couleurs d'Eternity II ont été équilibrées exprès pour supprimer ces prises. Cantonner les couleurs rares au bord les éloigne du vaste intérieur, là où un attaquant voudrait le plus un signal rare et très contraignant à saisir. L'intérieur est laissé à dix-sept couleurs communes, et c'est précisément pourquoi la recherche intérieure a si peu de prise. Le bord, où les couleurs rares rendent les accords serrés, explique pourquoi tout bon plateau obtient un bord quasi parfait.",
    related: "Voir pourquoi l'intérieur ne donne aucun coup forcé",
    reproTitle: "Le reproduire",
    reproBody: "Calculé exactement à partir du jeu officiel ; se reproduit à l'identique à chaque exécution.",
    sourceLink: "Article, code source et résultats sur GitHub",
  },
};

export default function RareColorGeography() {
  const t = useT(T);
  const isClient = useIsClient();
  const cell = 18;
  return (
    <div className="space-y-12">
      <div className="mx-auto max-w-3xl text-center">
        <LocalizedLink
          to="/research/why"
          className="text-sm text-muted-foreground hover:text-foreground"
        >
          {t.backLabel}
        </LocalizedLink>
        <h1 className="mt-3 text-4xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-4 text-xl text-muted-foreground">{t.lede}</p>
      </div>

      <section className="mx-auto max-w-2xl text-base leading-relaxed text-muted-foreground">
        <p>{t.p1}</p>
      </section>

      <section className="space-y-4">
        <h2 className="text-center text-2xl font-semibold tracking-tight">{t.ringTitle}</h2>
        <div className="mx-auto max-w-xs">
          <svg viewBox={`0 0 ${16 * cell} ${16 * cell}`} className="w-full rounded-lg border bg-card">
            {RING_CELLS.map((c, i) => (
              <rect
                key={i}
                x={c.x * cell + 1}
                y={c.y * cell + 1}
                width={cell - 2}
                height={cell - 2}
                rx={2}
                className={c.frame ? "fill-amber-400" : "fill-muted"}
              />
            ))}
          </svg>
        </div>
        <p className="mx-auto max-w-md text-center text-xs text-muted-foreground">{t.ringNote}</p>
      </section>

      <section className="space-y-4">
        <h2 className="text-center text-2xl font-semibold tracking-tight">{t.swatchTitle}</h2>
        <div className="flex flex-wrap justify-center gap-4">
          {rareColors.map((c) => (
            <div key={c} className="flex flex-col items-center gap-1">
              <MotifSwatch color={c} width={52} />
              <span className="font-mono text-xs text-muted-foreground">
                &apos;{colorToLetter(c)}&apos;
              </span>
            </div>
          ))}
        </div>
        <div className="mx-auto grid max-w-md grid-cols-3 gap-3 text-center">
          <div className="rounded-lg border p-3">
            <div className="text-2xl font-bold tabular-nums">{data.edgesPerRareColor}</div>
            <div className="text-xs text-muted-foreground">{t.statEdges}</div>
          </div>
          <div className="rounded-lg border p-3">
            <div className="text-2xl font-bold tabular-nums">{data.rareEdgesTotal}</div>
            <div className="text-xs text-muted-foreground">{t.statTotal}</div>
          </div>
          <div className="rounded-lg border p-3">
            <div className="text-2xl font-bold tabular-nums text-emerald-600">0</div>
            <div className="text-xs text-muted-foreground">{t.statInterior}</div>
          </div>
        </div>
      </section>

      <section className="space-y-3">
        <h2 className="text-center text-2xl font-semibold tracking-tight">{t.chartTitle}</h2>
        <div className="mx-auto h-72 max-w-3xl rounded-lg border p-2">
          {isClient ? (
            <ResponsiveContainer width="100%" height="100%" minWidth={0} minHeight={0}>
              <BarChart data={chartData} margin={{ top: 10, right: 16, bottom: 18, left: 8 }}>
                <CartesianGrid strokeDasharray="3 3" className="stroke-muted" vertical={false} />
                <XAxis
                  dataKey="color"
                  label={{ value: t.colorAxis, position: "insideBottom", offset: -8, fontSize: 12 }}
                  fontSize={11}
                />
                <YAxis
                  label={{ value: t.edgesAxis, angle: -90, position: "insideLeft", fontSize: 12 }}
                  width={40}
                  fontSize={12}
                />
                <Tooltip />
                <Legend />
                <Bar dataKey="frame" name={t.frame} stackId="a" fill="#f59e0b" />
                <Bar dataKey="interior" name={t.interior} stackId="a" fill="#38bdf8" radius={[3, 3, 0, 0]} />
              </BarChart>
            </ResponsiveContainer>
          ) : (
            <div className="flex h-full items-center justify-center text-sm text-muted-foreground">
              …
            </div>
          )}
        </div>
        <p className="mx-auto max-w-2xl text-center text-xs text-muted-foreground">{t.chartNote}</p>
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.whyTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.why}</p>
        <LocalizedLink
          to="/research/why/no-forced-moves"
          className="inline-block text-sm font-medium underline hover:text-foreground"
        >
          {t.related}
        </LocalizedLink>
      </section>

      <section className="mx-auto max-w-2xl space-y-3 rounded-lg border bg-muted/30 p-5">
        <h2 className="text-lg font-semibold tracking-tight">{t.reproTitle}</h2>
        <p className="text-sm text-muted-foreground">{t.reproBody}</p>
        <pre className="overflow-x-auto rounded-md bg-background p-3 text-xs">
          <code>{`cd research/topics/rare-color-geography/compute
cargo run --release > ../results/color-geography.json`}</code>
        </pre>
        <a
          href={ARTICLE_URL}
          target="_blank"
          rel="noreferrer"
          className="inline-block text-sm font-medium underline hover:text-foreground"
        >
          {t.sourceLink}
        </a>
      </section>

      <RelatedRail path="/research/why/rare-color-geography" />
    </div>
  );
}

export const meta = pageMeta("rare-color-geography");
