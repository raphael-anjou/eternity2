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

const COMMUNITY = [
  { name: "e2.bucas.name board viewer", url: "https://e2.bucas.name" },
  { name: "Eternity II Discord", url: "https://discord.gg/Ny5xs3q8w" },
  { name: "groups.io/g/eternity2", url: "https://groups.io/g/eternity2" },
];

const T = {
  en: {
    wip: "This is a work in progress. The website is young and slowly adding years of research.",
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
        body: "Original findings and the named algorithms built to attack the puzzle — Raphaël's experiments, all reproducible from source, with room for other puzzlers' work to come.",
      },
    },
    communityTitle: "Community infrastructure",
    communityNotes: [
      "Jef Bucas's GPL viewer; its URL format is the community's lingua franca (and this site speaks it natively).",
      "An active Discord server where puzzlers share their runs, records and code, in real time.",
      "The active mailing list: records, techniques, and 15+ years of accumulated folklore.",
    ],
  },
  fr: {
    wip: "Ce site est en construction. Il est jeune et intègre peu à peu des années de recherche.",
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
        body: "Résultats originaux et algorithmes nommés conçus pour attaquer le puzzle — les expériences de Raphaël, le tout reproductible depuis les sources, avec de la place pour les travaux d'autres puzzlers à venir.",
      },
    },
    communityTitle: "Les outils de la communauté",
    communityNotes: [
      "Le visualiseur GPL de Jef Bucas : son format d'URL fait office de langue commune dans la communauté (et ce site le parle couramment).",
      "Un serveur Discord vivant où les passionnés partagent en temps réel leurs essais, leurs records et leur code.",
      "La liste de diffusion historique : des records, des techniques et plus de quinze ans de savoir accumulé.",
    ],
  },
};

export default function Research() {
  const t = useT(T);
  return (
    <div className="space-y-12">
      <div className="rounded-lg border border-amber-300/60 bg-amber-500/10 px-4 py-3 text-sm text-amber-900 dark:border-amber-400/30 dark:text-amber-200">
        {t.wip}
      </div>

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

      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.communityTitle}</h2>
        <div className="grid gap-3 md:grid-cols-3">
          {COMMUNITY.map((c, i) => (
            <a key={c.url} href={c.url} target="_blank" rel="noreferrer" className="group">
              <Card className="h-full transition-shadow group-hover:shadow-md">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm group-hover:underline">{c.name}</CardTitle>
                </CardHeader>
                <CardContent className="text-sm text-muted-foreground">
                  {t.communityNotes[i]}
                </CardContent>
              </Card>
            </a>
          ))}
        </div>
      </section>
    </div>
  );
}

export const meta = pageMeta("research");
