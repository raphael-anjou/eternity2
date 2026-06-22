import { pageMeta } from "@/seo";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";

// Door 3 of the research section: the open lab notebook. The original findings,
// the named algorithms, and the notable boards produced while attacking the
// puzzle. Everything published here is reproducible from source — deterministic
// results ship a runnable script and its committed output; stochastic or
// long-running ones ship the script and the board (verifiable in the viewer),
// honestly labelled. Each area is published through the research pipeline and
// shown as upcoming until it lands.
type Area = { key: string; to?: string; ready: boolean };

const AREAS: Area[] = [
  { key: "findings", ready: false },
  { key: "inventions", ready: true, to: "/research/lab/inventions" },
  { key: "basins", ready: false },
];

const T = {
  en: {
    title: "The lab notebook",
    intro:
      "An open notebook of original work on Eternity II: the structural findings, the algorithms built to attack it, and the notable boards that came out. Everything here is reproducible from source — and where a search is stochastic or long-running, the script and the resulting board are published anyway, plainly labelled.",
    backLabel: "← Research",
    soon: "In preparation",
    reproTitle: "Everything is reproducible",
    reproBody:
      "No result on this site is an unbacked claim. Deterministic computations ship a runnable script and the exact output it produces. Searches that depend on randomness or take hours ship the same script plus the board they found — which you can load into the viewer and check edge by edge. The label on each result tells you which kind it is.",
    areas: {
      findings: {
        title: "Findings",
        body: "Standing structural results about the puzzle — rigidity, the entropy wall, indecomposable rearrangements, forbidden patterns — each written up with the computation behind it.",
      },
      inventions: {
        title: "Inventions",
        body: "The named algorithms built to push the score: what problem each attacks, the idea, the result it reached, and the open questions it left. Methods and code in full.",
      },
      basins: {
        title: "Notable boards",
        body: "A gallery of record and boundary boards as case studies — each loadable in the viewer and annotated with what makes it interesting.",
      },
    },
  },
  fr: {
    title: "Le carnet de laboratoire",
    intro:
      "Un carnet ouvert de travaux originaux sur Eternity II : les résultats structurels, les algorithmes conçus pour l'attaquer et les plateaux notables qui en sont sortis. Tout ici est reproductible depuis les sources — et lorsqu'une recherche est aléatoire ou longue, le script et le plateau obtenu sont quand même publiés, clairement étiquetés.",
    backLabel: "← Recherche",
    soon: "En préparation",
    reproTitle: "Tout est reproductible",
    reproBody:
      "Aucun résultat de ce site n'est une affirmation sans preuve. Les calculs déterministes sont accompagnés d'un script exécutable et de la sortie exacte qu'il produit. Les recherches qui dépendent du hasard ou prennent des heures fournissent le même script ainsi que le plateau trouvé — que vous pouvez charger dans le visualiseur et vérifier bord par bord. L'étiquette de chaque résultat indique de quel type il s'agit.",
    areas: {
      findings: {
        title: "Résultats",
        body: "Des résultats structurels établis sur le puzzle — rigidité, mur d'entropie, réarrangements indécomposables, motifs interdits — chacun rédigé avec le calcul qui le sous-tend.",
      },
      inventions: {
        title: "Inventions",
        body: "Les algorithmes nommés conçus pour pousser le score : le problème que chacun attaque, l'idée, le résultat atteint et les questions restées ouvertes. Méthodes et code en entier.",
      },
      basins: {
        title: "Plateaux notables",
        body: "Une galerie de plateaux records et limites présentés comme des études de cas — chacun chargeable dans le visualiseur et annoté de ce qui le rend intéressant.",
      },
    },
  },
};

export default function LabHub() {
  const t = useT(T);
  return (
    <div className="space-y-12">
      <div>
        <LocalizedLink
          to="/research"
          className="text-sm text-muted-foreground hover:text-foreground"
        >
          {t.backLabel}
        </LocalizedLink>
        <h1 className="mt-2 text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-2 max-w-3xl text-muted-foreground">{t.intro}</p>
      </div>

      <section className="grid gap-3 md:grid-cols-3">
        {AREAS.map((area) => {
          const copy = t.areas[area.key as keyof typeof t.areas];
          const card = (
            <Card className="h-full transition-shadow group-hover:shadow-md">
              <CardHeader className="pb-2">
                <div className="flex items-start justify-between gap-2">
                  <CardTitle className="text-base group-hover:underline">{copy.title}</CardTitle>
                  {!area.ready && (
                    <span className="shrink-0 rounded-full bg-muted px-2 py-0.5 text-[10px] font-medium tracking-wide text-muted-foreground uppercase">
                      {t.soon}
                    </span>
                  )}
                </div>
              </CardHeader>
              <CardContent className="text-sm text-muted-foreground">{copy.body}</CardContent>
            </Card>
          );
          return area.ready && area.to ? (
            <LocalizedLink key={area.key} to={area.to} className="group block">
              {card}
            </LocalizedLink>
          ) : (
            <div key={area.key} className="group block opacity-75">
              {card}
            </div>
          );
        })}
      </section>

      <section className="max-w-3xl space-y-3 rounded-lg border bg-muted/30 p-5">
        <h2 className="text-lg font-semibold tracking-tight">{t.reproTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.reproBody}</p>
      </section>
    </div>
  );
}

export const meta = pageMeta("lab");
