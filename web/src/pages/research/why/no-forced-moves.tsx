import { pageMeta } from "@/seo";
import {
  Bar,
  BarChart,
  CartesianGrid,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { LocalizedLink } from "@/components/LocalizedLink";
import { RelatedRail } from "@/components/research/RelatedRail";
import { ForcedMovesLab } from "@/components/research/ForcedMovesLab";
import data from "@/data/no-forced-moves.json";

const ARTICLE_URL =
  "https://github.com/raphael-anjou/eternity2/tree/main/research/topics/no-forced-moves";

const histogram = data.histogram;

const T = {
  en: {
    backLabel: "← Why it's hard",
    title: "No forced moves",
    lede: "The usual way to crack a logic puzzle is to find a spot where only one piece fits, place it, and repeat. That lever doesn't exist here.",
    p1: "Every one of the 196 interior pieces has between 73 and 137 other pieces that could legally sit beside it. The typical piece has over a hundred options. Not a single piece is ever pinned to one choice.",
    p2: "This is the flip side of forbidden patterns. There, almost every combination of pieces is impossible. You'd think all those rules would corner pieces into place. They don't: the constraints rule out combinations without ever cornering an individual piece, so a solver never gets a free, forced move to build on.",
    chartTitle: "How many neighbours each piece allows",
    chartIntro: "The 196 interior pieces, bucketed by how many right-hand neighbours each one accepts. The whole distribution sits far from one.",
    xLabel: "right-hand partners",
    yLabel: "pieces",
    statsForced: "pieces forced to a single option",
    statsRange: "partners per piece (min to max)",
    statsMean: "average partners",
    tryTitle: "See it on a real puzzle",
    tryIntro: "The engine fills a few cells; then we count, live, how many pieces legally fit the next one. It almost never drops to one.",
    whyTitle: "Why it matters",
    why: "Put this beside forbidden patterns and the real shape of the difficulty appears. Locally the puzzle looks loose: any piece fits next to plenty of others, so there's nothing to propagate and no chain of forced moves to ride. Globally almost every combination is illegal. The hardness lives in that gap: lots of local freedom, almost no global consistency. A solver has to make a long run of free-looking choices that only turn out wrong much later.",
    reproTitle: "Reproduce it",
    reproBody: "Computed exactly from the official set; reproduces identically every run.",
    sourceLink: "Article, source and results on GitHub",
    busy: "Loading chart…",
  },
  fr: {
    backLabel: "← Pourquoi c'est dur",
    title: "Aucun coup forcé",
    lede: "La façon habituelle de résoudre un casse-tête logique : trouver un endroit où une seule pièce convient, la poser, recommencer. Ce levier n'existe pas ici.",
    p1: "Chacune des 196 pièces intérieures a entre 73 et 137 autres pièces qui peuvent légalement se placer à côté d'elle. La pièce typique a plus de cent options. Pas une seule pièce n'est jamais réduite à un choix unique.",
    p2: "C'est le revers des motifs interdits. Là-bas, presque toute combinaison de pièces est impossible. On croirait que toutes ces règles coincent les pièces en place. Non : les contraintes éliminent des combinaisons sans jamais coincer une pièce individuelle, donc un solveur n'obtient jamais de coup forcé gratuit sur lequel bâtir.",
    chartTitle: "Combien de voisines chaque pièce autorise",
    chartIntro: "Les 196 pièces intérieures, regroupées selon le nombre de voisines de droite que chacune accepte. Toute la distribution est loin de un.",
    xLabel: "partenaires de droite",
    yLabel: "pièces",
    statsForced: "pièces réduites à un seul choix",
    statsRange: "partenaires par pièce (min à max)",
    statsMean: "partenaires en moyenne",
    tryTitle: "Voyez-le sur un vrai puzzle",
    tryIntro: "Le moteur remplit quelques cases ; puis on compte, en direct, combien de pièces conviennent à la suivante. Cela ne descend presque jamais à une.",
    whyTitle: "Pourquoi c'est important",
    why: "Mettez ceci à côté des motifs interdits et la vraie forme de la difficulté apparaît. Localement, le puzzle semble lâche : toute pièce s'accorde avec beaucoup d'autres, donc rien à propager et aucune chaîne de coups forcés à suivre. Globalement, presque toute combinaison est illégale. La difficulté vit dans cet écart : beaucoup de liberté locale, presque aucune cohérence globale. Un solveur doit enchaîner de longs choix d'apparence libre qui ne se révèlent faux que bien plus tard.",
    reproTitle: "Le reproduire",
    reproBody: "Calculé exactement à partir du jeu officiel ; se reproduit à l'identique à chaque exécution.",
    sourceLink: "Article, code source et résultats sur GitHub",
    busy: "Chargement du graphique…",
  },
};

export default function NoForcedMoves() {
  const t = useT(T);
  const isClient = useIsClient();
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

      <section className="mx-auto max-w-2xl space-y-4 text-base leading-relaxed text-muted-foreground">
        <p className="text-foreground">{t.p1}</p>
        <p>{t.p2}</p>
      </section>

      <section className="mx-auto grid max-w-2xl grid-cols-3 gap-3 text-center">
        <div className="rounded-lg border p-4">
          <div className="text-3xl font-bold tabular-nums">{data.forcedPieces}</div>
          <div className="mt-1 text-xs text-muted-foreground">{t.statsForced}</div>
        </div>
        <div className="rounded-lg border p-4">
          <div className="text-3xl font-bold tabular-nums">
            {data.minPartners}–{data.maxPartners}
          </div>
          <div className="mt-1 text-xs text-muted-foreground">{t.statsRange}</div>
        </div>
        <div className="rounded-lg border p-4">
          <div className="text-3xl font-bold tabular-nums">{data.meanPartners}</div>
          <div className="mt-1 text-xs text-muted-foreground">{t.statsMean}</div>
        </div>
      </section>

      <section className="space-y-3">
        <h2 className="text-center text-2xl font-semibold tracking-tight">{t.chartTitle}</h2>
        <p className="mx-auto max-w-2xl text-center text-sm text-muted-foreground">{t.chartIntro}</p>
        <div className="mx-auto h-72 max-w-2xl rounded-lg border p-2">
          {isClient ? (
            <ResponsiveContainer width="100%" height="100%" minWidth={0} minHeight={0}>
              <BarChart data={histogram} margin={{ top: 10, right: 16, bottom: 18, left: 8 }}>
                <CartesianGrid strokeDasharray="3 3" className="stroke-muted" vertical={false} />
                <XAxis
                  dataKey="bucket"
                  label={{ value: t.xLabel, position: "insideBottom", offset: -8, fontSize: 12 }}
                  fontSize={11}
                />
                <YAxis
                  label={{ value: t.yLabel, angle: -90, position: "insideLeft", fontSize: 12 }}
                  width={40}
                  fontSize={12}
                />
                <Tooltip
                  formatter={(v) => [String(v), t.yLabel]}
                  labelFormatter={(b) => `${String(b)} ${t.xLabel}`}
                />
                <Bar dataKey="count" fill="#38bdf8" radius={[4, 4, 0, 0]} />
              </BarChart>
            </ResponsiveContainer>
          ) : (
            <div className="flex h-full items-center justify-center text-sm text-muted-foreground">
              {t.busy}
            </div>
          )}
        </div>
      </section>

      <section className="space-y-4">
        <div className="mx-auto max-w-2xl text-center">
          <h2 className="text-2xl font-semibold tracking-tight">{t.tryTitle}</h2>
          <p className="mt-1 text-sm text-muted-foreground">{t.tryIntro}</p>
        </div>
        <div className="mx-auto max-w-3xl">
          <ForcedMovesLab />
        </div>
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.whyTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.why}</p>
      </section>

      <section className="mx-auto max-w-2xl space-y-3 rounded-lg border bg-muted/30 p-5">
        <h2 className="text-lg font-semibold tracking-tight">{t.reproTitle}</h2>
        <p className="text-sm text-muted-foreground">{t.reproBody}</p>
        <pre className="overflow-x-auto rounded-md bg-background p-3 text-xs">
          <code>{`cd research/topics/no-forced-moves/compute
cargo run --release > ../results/partner-counts.json`}</code>
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

      <RelatedRail path="/research/why/no-forced-moves" />
    </div>
  );
}

export const meta = pageMeta("no-forced-moves");
