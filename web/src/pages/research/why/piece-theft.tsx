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
import { PieceTheftDiagram } from "@/components/research/PieceTheftDiagram";
import data from "@/data/piece-theft.json";

const ARTICLE_URL =
  "https://github.com/raphael-anjou/eternity2/tree/main/research/topics/piece-theft";

const histogram = data.histogram;

const T = {
  en: {
    backLabel: "← Why it's hard",
    title: "Piece theft, where solvers die",
    lede: "A solver fills a few rows for free, then hits a wall in the middle of the board. Here's the mechanism: a scarce piece spent in the wrong place, rows ago.",
    p1: "Fill the board top-left to bottom-right and every new cell already knows two of its colors: the north color from the piece above, the west color from the piece to the left. The cell needs an unused piece that can show that exact pair. Those demands are scarce, with only about three possible pieces on average, and 47 of them have just one.",
    p2: "So a board that looks healthy, most pieces still in the box, can already be doomed. Somewhere back up the board, the single piece that could ever serve an upcoming cell was used for something else. When the solver finally reaches that cell, there's nothing to place.",
    vizTitle: "How a cell dies",
    chartTitle: "How many pieces can serve a demand",
    chartNote: "Each (north, west) demand a cell can have, bucketed by how many interior pieces could serve it. Most demands have one to three.",
    serversAxis: "pieces that can serve the demand",
    demandsAxis: "demands",
    statUnique: "demands served by a single piece",
    statMean: "pieces per demand, on average",
    statTotal: "distinct demands that occur",
    whyTitle: "Why it matters",
    why1: "A tempting fix is a global check: do the remaining pieces still cover the remaining cells? It doesn't help. Globally the supply is fine; the failure is one scarce piece misallocated, not a shortage. So a global lookahead sees nothing wrong right up until the cell turns out to have no server, which is why this wall resisted so many attempts to prune it early.",
    why2: "Set this beside no forced moves and the trap is complete. Every piece has dozens of places it could go, so the solver is never told where a scarce piece must be saved, yet each scarce piece has exactly one demand it must be saved for. Freedom to place, no guidance on where to save.",
    related: "See why no piece is ever forced",
    reproTitle: "Reproduce it",
    reproBody: "Computed exactly from the official set; identical every run.",
    sourceLink: "Article, source and results on GitHub",
    busy: "Loading chart…",
  },
  fr: {
    backLabel: "← Pourquoi c'est dur",
    title: "Le vol de pièce, là où les solveurs meurent",
    lede: "Un solveur remplit quelques rangées gratuitement, puis frappe un mur au milieu du plateau. Voici le mécanisme : une pièce rare dépensée au mauvais endroit, des rangées plus tôt.",
    p1: "Remplissez le plateau de haut-gauche vers bas-droite et chaque nouvelle cellule connaît déjà deux de ses couleurs : la couleur nord de la pièce du dessus, la couleur ouest de la pièce de gauche. La cellule a besoin d'une pièce inutilisée capable de montrer cette paire exacte. Ces demandes sont rares, avec environ trois pièces possibles en moyenne, et 47 d'entre elles n'en ont qu'une.",
    p2: "Ainsi un plateau qui paraît sain, la plupart des pièces encore dans la boîte, peut déjà être condamné. Quelque part plus haut, la seule pièce capable de servir une cellule à venir a été utilisée ailleurs. Quand le solveur atteint enfin cette cellule, il n'y a rien à poser.",
    vizTitle: "Comment une cellule meurt",
    chartTitle: "Combien de pièces peuvent servir une demande",
    chartNote: "Chaque demande (nord, ouest) qu'une cellule peut avoir, regroupée selon le nombre de pièces intérieures capables de la servir. La plupart en ont une à trois.",
    serversAxis: "pièces capables de servir la demande",
    demandsAxis: "demandes",
    statUnique: "demandes servies par une seule pièce",
    statMean: "pièces par demande, en moyenne",
    statTotal: "demandes distinctes qui apparaissent",
    whyTitle: "Pourquoi c'est important",
    why1: "Une correction tentante est une vérification globale : les pièces restantes couvrent-elles encore les cellules restantes ? Ça n'aide pas. Globalement l'offre est suffisante ; l'échec est une pièce rare mal affectée, pas une pénurie. Un regard global ne voit donc rien d'anormal jusqu'à ce que la cellule se révèle sans serveur, et c'est pourquoi ce mur a résisté à tant de tentatives d'élagage précoce.",
    why2: "Placez ceci à côté de « aucun coup forcé » et le piège est complet. Chaque pièce a des dizaines d'endroits où elle pourrait aller, donc le solveur n'apprend jamais où une pièce rare doit être gardée, alors que chaque pièce rare a exactement une demande pour laquelle elle doit l'être. Liberté de poser, aucun guide sur où garder.",
    related: "Voir pourquoi aucune pièce n'est jamais forcée",
    reproTitle: "Le reproduire",
    reproBody: "Calculé exactement à partir du jeu officiel ; identique à chaque exécution.",
    sourceLink: "Article, code source et résultats sur GitHub",
    busy: "Chargement du graphique…",
  },
};

export default function PieceTheft() {
  const t = useT(T);
  const isClient = useIsClient();
  return (
    <div className="space-y-12">
      <div className="mx-auto max-w-3xl text-center">
        <LocalizedLink to="/research/why" className="text-sm text-muted-foreground hover:text-foreground">
          {t.backLabel}
        </LocalizedLink>
        <h1 className="mt-3 text-4xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-4 text-xl text-muted-foreground">{t.lede}</p>
      </div>

      <section className="mx-auto max-w-2xl space-y-4 text-base leading-relaxed text-muted-foreground">
        <p>{t.p1}</p>
        <p className="text-foreground">{t.p2}</p>
      </section>

      <section className="space-y-4">
        <h2 className="text-center text-2xl font-semibold tracking-tight">{t.vizTitle}</h2>
        <PieceTheftDiagram />
      </section>

      <section className="mx-auto grid max-w-2xl grid-cols-3 gap-3 text-center">
        <div className="rounded-lg border p-4">
          <div className="text-3xl font-bold tabular-nums text-red-600">{data.uniqueServerDemands}</div>
          <div className="mt-1 text-xs text-muted-foreground">{t.statUnique}</div>
        </div>
        <div className="rounded-lg border p-4">
          <div className="text-3xl font-bold tabular-nums">{data.meanServers}</div>
          <div className="mt-1 text-xs text-muted-foreground">{t.statMean}</div>
        </div>
        <div className="rounded-lg border p-4">
          <div className="text-3xl font-bold tabular-nums">{data.occurringDemands}</div>
          <div className="mt-1 text-xs text-muted-foreground">{t.statTotal}</div>
        </div>
      </section>

      <section className="space-y-3">
        <h2 className="text-center text-2xl font-semibold tracking-tight">{t.chartTitle}</h2>
        <div className="mx-auto h-72 max-w-2xl rounded-lg border p-2">
          {isClient ? (
            <ResponsiveContainer width="100%" height="100%" minWidth={0} minHeight={0}>
              <BarChart data={histogram} margin={{ top: 10, right: 16, bottom: 18, left: 8 }}>
                <CartesianGrid strokeDasharray="3 3" className="stroke-muted" vertical={false} />
                <XAxis
                  dataKey="servers"
                  label={{ value: t.serversAxis, position: "insideBottom", offset: -8, fontSize: 12 }}
                  fontSize={12}
                />
                <YAxis
                  label={{ value: t.demandsAxis, angle: -90, position: "insideLeft", fontSize: 12 }}
                  width={40}
                  fontSize={12}
                />
                <Tooltip
                  formatter={(v) => [String(v), t.demandsAxis]}
                  labelFormatter={(s) => `${String(s)} ${t.serversAxis}`}
                />
                <Bar dataKey="demands" fill="#a78bfa" radius={[4, 4, 0, 0]} />
              </BarChart>
            </ResponsiveContainer>
          ) : (
            <div className="flex h-full items-center justify-center text-sm text-muted-foreground">{t.busy}</div>
          )}
        </div>
        <p className="mx-auto max-w-2xl text-center text-xs text-muted-foreground">{t.chartNote}</p>
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.whyTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.why1}</p>
        <p className="text-base leading-relaxed text-muted-foreground">{t.why2}</p>
        <LocalizedLink to="/research/why/no-forced-moves" className="inline-block text-sm font-medium underline hover:text-foreground">
          {t.related}
        </LocalizedLink>
      </section>

      <section className="mx-auto max-w-2xl space-y-3 rounded-lg border bg-muted/30 p-5">
        <h2 className="text-lg font-semibold tracking-tight">{t.reproTitle}</h2>
        <p className="text-sm text-muted-foreground">{t.reproBody}</p>
        <pre className="overflow-x-auto rounded-md bg-background p-3 text-xs">
          <code>{`cd research/topics/piece-theft/compute
cargo run --release > ../results/piece-theft.json`}</code>
        </pre>
        <a href={ARTICLE_URL} target="_blank" rel="noreferrer" className="inline-block text-sm font-medium underline hover:text-foreground">
          {t.sourceLink}
        </a>
      </section>
    </div>
  );
}

export const meta = pageMeta("piece-theft");
