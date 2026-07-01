import { pageMeta } from "@/seo";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";

// Door 2 of the research section: everything you need if you're actually writing
// a solver. Existing pages (reference / papers / records) are re-homed here;
// dead-ends and the solver catalogue are published through the research pipeline
// and shown as upcoming until they land (no dead links).
type Tool = { key: string; to?: string; ready: boolean };

const TOOLS: Tool[] = [
  { key: "reference", to: "/research/reference", ready: true },
  { key: "papers", to: "/research/papers", ready: true },
  { key: "records", to: "/research/records", ready: true },
  { key: "solvers", ready: true, to: "/research/build/solvers" },
  { key: "deadEnds", ready: true, to: "/research/build/dead-ends" },
  { key: "quickstart", ready: true, to: "/research/build/run-it-yourself" },
];

const T = {
  en: {
    title: "Build a solver",
    intro:
      "The practitioner's corner. Validation data to check your code against, the literature ranked by usefulness, the record timeline and the methods behind it, and the dead ends so you don't spend a month on an approach that provably can't work.",
    backLabel: "← Research",
    soon: "In preparation",
    tools: {
      reference: {
        title: "Reference numbers",
        body: "Exact placement counts for small blocks of the official board — known-good values to validate your own edge-matching and constraint code against.",
      },
      papers: {
        title: "Papers",
        body: "The academic literature, ranked by how useful each paper actually is for writing a solver — from the NP-completeness proof and the phase-transition result to the constraint-propagation methods behind record boards.",
      },
      records: {
        title: "Records & solvers",
        body: "The record timeline — who reached 470/480 and how — plus the key community solvers and why some '480' boards aren't the real puzzle.",
      },
      solvers: {
        title: "Solver catalogue",
        body: "Each notable solver broken down: the search order, the propagation, the schedule and the break-index tricks — with reproducible runs where the budget is reasonable.",
      },
      deadEnds: {
        title: "Dead ends",
        body: "Approaches that look promising and provably do not move the needle on Eternity II, each with the evidence — so the community stops re-discovering the same walls.",
      },
      quickstart: {
        title: "Run it yourself",
        body: "Clone, build the engine, and run a search in a handful of commands. The engine and every result here are reproducible from the source.",
      },
    },
  },
  fr: {
    title: "Construire un solveur",
    intro:
      "Le coin des praticiens. Des données pour valider votre code, la littérature classée par utilité, la chronologie des records et les méthodes qui les sous-tendent, et les impasses — pour ne pas passer un mois sur une approche dont on peut prouver qu'elle ne mène nulle part.",
    backLabel: "← Recherche",
    soon: "En préparation",
    tools: {
      reference: {
        title: "Valeurs de référence",
        body: "Comptages exacts de placements pour de petits blocs du plateau officiel — des valeurs sûres pour valider votre code d'assemblage et de contraintes.",
      },
      papers: {
        title: "Articles scientifiques",
        body: "La littérature scientifique, classée selon l'utilité réelle de chaque article pour écrire un solveur — de la preuve de NP-complétude au résultat de transition de phase jusqu'aux méthodes de propagation de contraintes derrière les plateaux records.",
      },
      records: {
        title: "Records & solveurs",
        body: "La chronologie des records — qui a atteint 470/480 et comment — les principaux solveurs de la communauté et pourquoi certains plateaux « 480 » ne sont pas le vrai puzzle.",
      },
      solvers: {
        title: "Catalogue des solveurs",
        body: "Chaque solveur notable décortiqué : l'ordre de recherche, la propagation, le calendrier et les astuces d'index de rupture — avec des exécutions reproductibles quand le budget est raisonnable.",
      },
      deadEnds: {
        title: "Impasses",
        body: "Des approches qui semblent prometteuses mais dont on peut prouver qu'elles ne font pas bouger Eternity II, chacune avec ses preuves — pour que la communauté cesse de redécouvrir les mêmes murs.",
      },
      quickstart: {
        title: "À vous de jouer",
        body: "Cloner, compiler le moteur et lancer une recherche en quelques commandes. Le moteur et chacun des résultats présentés ici sont reproductibles depuis les sources.",
      },
    },
  },
};

export default function BuildHub() {
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
        {TOOLS.map((tool) => {
          const copy = t.tools[tool.key as keyof typeof t.tools];
          const card = (
            <Card className="h-full transition-shadow group-hover:shadow-md">
              <CardHeader className="pb-2">
                <div className="flex items-start justify-between gap-2">
                  <CardTitle className="text-base group-hover:underline">{copy.title}</CardTitle>
                  {!tool.ready && (
                    <span className="shrink-0 rounded-full bg-muted px-2 py-0.5 text-[10px] font-medium tracking-wide text-muted-foreground uppercase">
                      {t.soon}
                    </span>
                  )}
                </div>
              </CardHeader>
              <CardContent className="text-sm text-muted-foreground">{copy.body}</CardContent>
            </Card>
          );
          return tool.ready && tool.to ? (
            <LocalizedLink key={tool.key} to={tool.to} className="group block">
              {card}
            </LocalizedLink>
          ) : (
            <div key={tool.key} className="group block opacity-75">
              {card}
            </div>
          );
        })}
      </section>
    </div>
  );
}

export const meta = pageMeta("build");
