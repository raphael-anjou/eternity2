import { pageMeta } from "@/seo";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";

// Index of the named algorithms. Each ready one has its own page under this
// folder; the rest are listed honestly as upcoming. `score` and `note` give the
// one-line outcome so the index reads as a real catalogue, not just titles.
type Invention = {
  key: string;
  score?: number;
  to?: string;
  ready: boolean;
};

const INVENTIONS: Invention[] = [
  { key: "palimpsest", score: 463, to: "/research/lab/inventions/palimpsest", ready: true },
  { key: "keyring", score: 460, to: "/research/lab/inventions/keyring", ready: true },
  { key: "gauntlet", score: 458, to: "/research/lab/inventions/gauntlet", ready: true },
  { key: "prior", score: 460, to: "/research/lab/inventions/prior", ready: true },
  { key: "staged", score: 436, to: "/research/lab/inventions/staged", ready: true },
  { key: "bandsaw", score: 437, to: "/research/lab/inventions/bandsaw", ready: true },
  { key: "ladder", score: 451, to: "/research/lab/inventions/ladder", ready: true },
  { key: "replay", score: 460, to: "/research/lab/inventions/replay", ready: true },
];

const T = {
  en: {
    backLabel: "← The lab notebook",
    title: "Inventions",
    intro:
      "The named algorithms built to push the score. Each one has its own idea, its result, and the questions it left open. The best of them reaches 463 of 480 matched edges; the community's best on the same puzzle is 469. Methods and boards are here in full.",
    soon: "In preparation",
    scoreSuffix: "/ 480",
    inventions: {
      palimpsest: {
        title: "PALIMPSEST",
        body: "Reads the whole corpus of strong boards to find the recurring choices that quietly lock a board below the top, then targets them. Produced this project's best board.",
      },
      keyring: {
        title: "KEYRING",
        body: "Builds a board from scratch, ranking each next piece by three signals learned from past strong boards. Found a high board in a previously unseen family.",
      },
      gauntlet: {
        title: "GAUNTLET",
        body: "Runs a beam search in several fill directions at once to land in different regions, instead of always converging to the same one.",
      },
      prior: {
        title: "PRIOR",
        body: "A from-scratch beam search that breaks ties using where pieces tend to sit in strong boards. Reaches high scores with no anchor board at all.",
      },
      staged: {
        title: "STAGED",
        body: "Builds the whole board in stages with no pre-set frame, letting the border emerge last from what's left.",
      },
      bandsaw: {
        title: "BANDSAW",
        body: "Solves a band of rows exactly by meeting in the middle, to measure precisely how far ahead an endgame can be decided.",
      },
      ladder: {
        title: "LADDER",
        body: "Floods the board with cheap short searches and promotes only the deepest starts through longer rounds. Reached a 451 strict board with no record to copy.",
      },
      replay: {
        title: "REPLAY",
        body: "Rebuilds the community's strict 460 boards exactly, revealing the move ordinary solvers miss: two mismatches at one cell.",
      },
    },
  },
  fr: {
    backLabel: "← Le carnet de laboratoire",
    title: "Inventions",
    intro:
      "Les algorithmes nommés conçus pour pousser le score. Chacun a son idée, son résultat et les questions qu'il a laissées. Le meilleur atteint 463 bords appariés sur 480 ; le meilleur de la communauté sur le même puzzle est 469. Méthodes et plateaux sont ici en entier.",
    soon: "En préparation",
    scoreSuffix: "/ 480",
    inventions: {
      palimpsest: {
        title: "PALIMPSEST",
        body: "Lit tout le corpus des bons plateaux pour repérer les choix récurrents qui bloquent discrètement un plateau sous le sommet, puis les cible. A produit le meilleur plateau de ce projet.",
      },
      keyring: {
        title: "KEYRING",
        body: "Construit un plateau de zéro en classant chaque pièce suivante par trois signaux appris des bons plateaux passés. A trouvé un plateau élevé dans une famille jusque-là inédite.",
      },
      gauntlet: {
        title: "GAUNTLET",
        body: "Lance une recherche en faisceau dans plusieurs directions de remplissage à la fois pour atterrir dans des régions différentes, au lieu de toujours converger vers la même.",
      },
      prior: {
        title: "PRIOR",
        body: "Une recherche en faisceau partant de zéro qui départage les égalités selon l'endroit où les pièces se trouvent dans les bons plateaux. Atteint des scores élevés sans aucun plateau de départ.",
      },
      staged: {
        title: "STAGED",
        body: "Construit tout le plateau par étapes sans cadre préétabli, en laissant la bordure émerger en dernier à partir de ce qui reste.",
      },
      bandsaw: {
        title: "BANDSAW",
        body: "Résout exactement une bande de rangées en se rejoignant au milieu, pour mesurer précisément jusqu'où une fin de partie peut être décidée à l'avance.",
      },
      ladder: {
        title: "LADDER",
        body: "Inonde le plateau de recherches courtes et bon marché et ne promeut que les départs les plus profonds. A atteint un plateau 451 strict sans record à copier.",
      },
      replay: {
        title: "REPLAY",
        body: "Reconstruit exactement les plateaux 460 stricts de la communauté, révélant le coup que les solveurs manquent : deux défauts sur une seule cellule.",
      },
    },
  },
};

export default function InventionsHub() {
  const t = useT(T);
  return (
    <div className="space-y-10">
      <div>
        <LocalizedLink
          to="/research/lab"
          className="text-sm text-muted-foreground hover:text-foreground"
        >
          {t.backLabel}
        </LocalizedLink>
        <h1 className="mt-2 text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-2 max-w-3xl text-muted-foreground">{t.intro}</p>
      </div>

      <section className="grid gap-3 md:grid-cols-2">
        {INVENTIONS.map((inv) => {
          const copy = t.inventions[inv.key as keyof typeof t.inventions];
          const card = (
            <Card className="h-full transition-shadow group-hover:shadow-md">
              <CardHeader className="pb-2">
                <div className="flex items-start justify-between gap-2">
                  <CardTitle className="text-base tracking-tight group-hover:underline">
                    {copy.title}
                  </CardTitle>
                  {inv.ready && inv.score ? (
                    <span className="shrink-0 text-xs text-muted-foreground">
                      <span className="font-bold text-foreground tabular-nums">{inv.score}</span>{" "}
                      {t.scoreSuffix}
                    </span>
                  ) : (
                    <span className="shrink-0 rounded-full bg-muted px-2 py-0.5 text-[10px] font-medium tracking-wide text-muted-foreground uppercase">
                      {t.soon}
                    </span>
                  )}
                </div>
              </CardHeader>
              <CardContent className="text-sm text-muted-foreground">{copy.body}</CardContent>
            </Card>
          );
          return inv.ready && inv.to ? (
            <LocalizedLink key={inv.key} to={inv.to} className="group block">
              {card}
            </LocalizedLink>
          ) : (
            <div key={inv.key} className="group block opacity-75">
              {card}
            </div>
          );
        })}
      </section>
    </div>
  );
}

export const meta = pageMeta("inventions");
