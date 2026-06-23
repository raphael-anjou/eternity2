import { pageMeta } from "@/seo";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";

// Index of the named algorithms, grouped by the kind of idea each one is. Each
// ready one has its own page under this folder. `score` is the one-line outcome;
// `live` flags the ones with a real-engine demo you can drive in the browser, so
// the index reads as a real, organised catalogue rather than a flat title list.
type Family = "scratch" | "corpus" | "concentrate" | "anchor" | "exact" | "decode";

type Invention = {
  key: string;
  score?: number;
  to?: string;
  ready: boolean;
  family: Family;
  live?: boolean;
};

// Ordering within the array also orders cards inside each family group.
const INVENTIONS: Invention[] = [
  { key: "prior", score: 460, to: "/research/lab/inventions/prior", ready: true, family: "scratch" },
  { key: "keyring", score: 460, to: "/research/lab/inventions/keyring", ready: true, family: "scratch" },
  { key: "gauntlet", score: 458, to: "/research/lab/inventions/gauntlet", ready: true, family: "scratch", live: true },
  { key: "staged", score: 436, to: "/research/lab/inventions/staged", ready: true, family: "scratch" },
  { key: "palimpsest", score: 463, to: "/research/lab/inventions/palimpsest", ready: true, family: "corpus" },
  { key: "ladder", score: 451, to: "/research/lab/inventions/ladder", ready: true, family: "concentrate", live: true },
  { key: "cloister", score: 453, to: "/research/lab/inventions/cloister", ready: true, family: "anchor", live: true },
  { key: "midden", score: 452, to: "/research/lab/inventions/midden", ready: true, family: "anchor" },
  { key: "bandsaw", score: 437, to: "/research/lab/inventions/bandsaw", ready: true, family: "exact" },
  { key: "replay", score: 460, to: "/research/lab/inventions/replay", ready: true, family: "decode" },
];

const FAMILY_ORDER: Family[] = ["scratch", "corpus", "concentrate", "anchor", "exact", "decode"];

const T = {
  en: {
    backLabel: "← The lab notebook",
    title: "Inventions",
    intro:
      "The named algorithms built to push the score. Each one has its own idea, its result, and the questions it left open. The best of them reaches 463 of 480 matched edges; the community's best on the same puzzle is 469. Methods and boards are here in full.",
    soon: "In preparation",
    scoreSuffix: "/ 480",
    liveBadge: "live demo",
    families: {
      scratch: {
        title: "Build from scratch",
        body: "Grow a board from an empty grid, with no record to copy — the construction itself is the idea. The lever is which cell to fill next and which piece to trust.",
      },
      corpus: {
        title: "Learn from the corpus",
        body: "Mine every strong board ever found for the choices they share, then use that statistical knowledge to steer or unlock a new one.",
      },
      concentrate: {
        title: "Concentrate the effort",
        body: "Spend the search budget where progress is real instead of spreading it evenly — flood cheap probes, keep the deepest, promote them.",
      },
      anchor: {
        title: "Anchor and constrain",
        body: "Fix part of the board, or decide where damage is allowed, so the rest of the search is constrained from the very first placement.",
      },
      exact: {
        title: "Solve a piece exactly",
        body: "Take a sub-region — a band, an endgame — and solve it to proven optimality, to measure exactly how far ahead a board can be decided.",
      },
      decode: {
        title: "Decode and replay",
        body: "Rebuild the community's own record boards move for move, to reveal the technique ordinary solvers miss.",
      },
    } as Record<string, { title: string; body: string }>,
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
      cloister: {
        title: "CLOISTER",
        body: "Fixes a perfect border, then searches the interior with the frame's edges as hard constraints from cell one. Isolates the border-interior coupling.",
      },
      midden: {
        title: "MIDDEN",
        body: "Decides where, not when, a board may break: confine mismatches to a chosen shape and search it. A dispersed lattice extends the perfect run.",
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
    liveBadge: "démo en direct",
    families: {
      scratch: {
        title: "Construire de zéro",
        body: "Faire croître un plateau depuis une grille vide, sans record à copier — la construction elle-même est l'idée. Le levier : quelle case remplir ensuite et à quelle pièce se fier.",
      },
      corpus: {
        title: "Apprendre du corpus",
        body: "Exploiter tous les bons plateaux connus pour les choix qu'ils partagent, puis utiliser ce savoir statistique pour orienter ou débloquer un nouveau plateau.",
      },
      concentrate: {
        title: "Concentrer l'effort",
        body: "Dépenser le budget de recherche là où le progrès est réel plutôt que de l'étaler — inonder de sondes bon marché, garder les plus profondes, les promouvoir.",
      },
      anchor: {
        title: "Ancrer et contraindre",
        body: "Fixer une partie du plateau, ou décider où les défauts sont permis, pour que le reste de la recherche soit contraint dès la première pose.",
      },
      exact: {
        title: "Résoudre une partie exactement",
        body: "Prendre une sous-région — une bande, une fin de partie — et la résoudre à l'optimum prouvé, pour mesurer jusqu'où un plateau peut être décidé à l'avance.",
      },
      decode: {
        title: "Décoder et rejouer",
        body: "Reconstruire coup par coup les plateaux records de la communauté, pour révéler la technique que les solveurs ordinaires manquent.",
      },
    } as Record<string, { title: string; body: string }>,
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
      cloister: {
        title: "CLOISTER",
        body: "Fixe une bordure parfaite, puis cherche l'intérieur avec les bords du cadre comme contraintes dures dès la première cellule. Isole le couplage bordure-intérieur.",
      },
      midden: {
        title: "MIDDEN",
        body: "Décide où, et non quand, un plateau peut casser : confiner les défauts à une forme choisie et la chercher. Un réseau dispersé prolonge la suite parfaite.",
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

      <div className="space-y-10">
        {FAMILY_ORDER.map((fam) => {
          const members = INVENTIONS.filter((inv) => inv.family === fam);
          if (!members.length) return null;
          const famCopy = t.families[fam];
          return (
            <section key={fam} className="space-y-3">
              <div className="max-w-3xl">
                <h2 className="text-xl font-semibold tracking-tight">{famCopy?.title}</h2>
                <p className="mt-1 text-sm text-muted-foreground">{famCopy?.body}</p>
              </div>
              <div className="grid gap-3 md:grid-cols-2">
                {members.map((inv) => {
                  const copy = t.inventions[inv.key as keyof typeof t.inventions];
                  const card = (
                    <Card className="h-full transition-shadow group-hover:shadow-md">
                      <CardHeader className="pb-2">
                        <div className="flex items-start justify-between gap-2">
                          <CardTitle className="flex items-center gap-2 text-base tracking-tight group-hover:underline">
                            {copy.title}
                            {inv.live && (
                              <span className="rounded-full bg-emerald-500/15 px-2 py-0.5 text-[10px] font-medium tracking-wide text-emerald-600 uppercase dark:text-emerald-400">
                                {t.liveBadge}
                              </span>
                            )}
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
              </div>
            </section>
          );
        })}
      </div>
    </div>
  );
}

export const meta = pageMeta("inventions");
