import { pageMeta } from "@/seo";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";

// The research section is organised as three doors. This page is the chooser:
// a newcomer reads "Why it's hard", a solver author lives in "Build a solver",
// and a researcher mines "The lab notebook". Each door is its own hub that fans
// out as far as the material goes.
const DOORS = [
  { key: "why", to: "/research/why" },
  { key: "build", to: "/research/build" },
  { key: "lab", to: "/research/lab" },
];

const T = {
  en: {
    title: "Research",
    intro:
      "For the people who want to actually solve it. Pick a door: why the puzzle is so hard, what you need to build a solver, or the open notebook of original work — including this project's.",
    doors: {
      why: {
        title: "Why it's hard",
        body: "The design that was tuned to resist cleverness, and the structural walls that explain the gap between the best known board and a full solution.",
      },
      build: {
        title: "Build a solver",
        body: "Validation data, the literature ranked by usefulness, the record timeline and the methods behind it, the dead ends — and how to run the code yourself.",
      },
      lab: {
        title: "The lab notebook",
        body: "Original findings, the named algorithms built to attack the puzzle, and the notable boards that came out — all reproducible from source.",
      },
    },
  },
  fr: {
    title: "Recherche",
    intro:
      "Pour celles et ceux qui veulent vraiment le résoudre. Choisissez une porte : pourquoi le puzzle est si dur, ce qu'il faut pour construire un solveur, ou le carnet ouvert des travaux originaux — y compris ceux de ce projet.",
    doors: {
      why: {
        title: "Pourquoi c'est dur",
        body: "Une conception calibrée pour déjouer l'ingéniosité, et les murs structurels qui expliquent l'écart entre le meilleur plateau connu et une solution complète.",
      },
      build: {
        title: "Construire un solveur",
        body: "Données de validation, littérature classée par utilité, chronologie des records et méthodes associées, impasses — et comment exécuter le code vous-même.",
      },
      lab: {
        title: "Le carnet de laboratoire",
        body: "Résultats originaux, algorithmes nommés conçus pour attaquer le puzzle, et plateaux notables obtenus — le tout reproductible depuis les sources.",
      },
    },
  },
};

export default function Research() {
  const t = useT(T);
  return (
    <div className="space-y-12">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-2 max-w-3xl text-muted-foreground">{t.intro}</p>
      </div>

      <section className="grid gap-4 md:grid-cols-3">
        {DOORS.map((door) => {
          const copy = t.doors[door.key as keyof typeof t.doors];
          return (
            <LocalizedLink key={door.key} to={door.to} className="group block">
              <Card className="h-full transition-shadow group-hover:shadow-md">
                <CardHeader>
                  <CardTitle className="group-hover:underline">{copy.title}</CardTitle>
                </CardHeader>
                <CardContent className="text-sm text-muted-foreground">{copy.body}</CardContent>
              </Card>
            </LocalizedLink>
          );
        })}
      </section>
    </div>
  );
}

export const meta = pageMeta("research");
