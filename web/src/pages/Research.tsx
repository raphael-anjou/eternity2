import { pageMeta } from "@/seo";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";

// Academic citations live on the dedicated /research/papers page (Papers.tsx).
// Only the per-entry community notes are localized here, via T.communityNotes,
// indexed in the same order as this array.
const COMMUNITY = [
  {
    name: "e2.bucas.name board viewer",
    url: "https://e2.bucas.name",
  },
  {
    name: "Eternity II Discord",
    url: "https://discord.gg/Ny5xs3q8w",
  },
  {
    name: "groups.io/g/eternity2",
    url: "https://groups.io/g/eternity2",
  },
];

const T = {
  en: {
    title: "Research",
    intro:
      "For the people who want to actually solve it. The academic record, the community infrastructure, and an open lab notebook of what has been tried, including by this project.",
    toolsTitle: "Tools for solver authors",
    referenceTitle: "Reference numbers",
    referenceBody:
      "Exact placement counts for small blocks of the official board — known-good values to validate your own edge-matching and constraint code against.",
    recordsCardTitle: "Records & solvers",
    recordsCardBody:
      "The record timeline — who reached 469/480 and how — plus the key community solvers and why some '480' boards aren't the real puzzle.",
    papersCardTitle: "Papers",
    papersCardBody:
      "The academic literature on Eternity II — pulled from our 200-volume research notebook and ranked by how useful each paper actually is for writing a solver, from the NP-completeness proof and the phase-transition result to the constraint-propagation methods behind record boards.",
    engineeredTitle: "Engineered to resist cleverness",
    engineeredBody:
      "Eternity I fell in 2000 because Alex Selby and Oliver Riordan discovered the puzzle had vastly more solutions than its designer believed, and aimed their search at the most \"solution-dense\" regions. For Eternity II, the publisher hired the winners: Selby and Riordan helped design and stress-test the new puzzle so that no such statistical shortcut survives. The visible fingerprints of that vetting: a single designed solution baked into balanced color counts, no rotationally-symmetric pieces, no duplicate pieces, rare motifs confined to the frame, and piece-count/color-count parameters sitting at the empirical hardness peak (later confirmed by Ansótegui et al.). The puzzle isn't accidentally hard. It was tuned to be.",
    communityTitle: "Community infrastructure",
    communityNotes: [
      "Jef Bucas's GPL viewer; its URL format is the community's lingua franca (and this site speaks it natively).",
      "An active Discord server where solvers share their runs, records and code, in real time.",
      "The active mailing list: records, techniques, and 15+ years of accumulated folklore.",
    ],
  },
  fr: {
    title: "Recherche",
    intro:
      "Pour celles et ceux qui veulent vraiment le résoudre. Les travaux académiques, les outils de la communauté et un carnet de laboratoire ouvert recensant tout ce qui a été tenté — y compris par ce projet.",
    toolsTitle: "Outils pour les auteurs de solveurs",
    referenceTitle: "Valeurs de référence",
    referenceBody:
      "Comptages exacts de placements pour de petits blocs du plateau officiel — des valeurs sûres pour valider votre code d'assemblage et de contraintes.",
    recordsCardTitle: "Records & solveurs",
    recordsCardBody:
      "La chronologie des records — qui a atteint 469/480 et comment — les principaux solveurs de la communauté et pourquoi certains plateaux « 480 » ne sont pas le vrai puzzle.",
    papersCardTitle: "Articles scientifiques",
    papersCardBody:
      "La littérature scientifique sur Eternity II — tirée de notre carnet de recherche de 200 volumes et classée selon l'utilité réelle de chaque article pour écrire un solveur, de la preuve de NP-complétude au résultat de transition de phase jusqu'aux méthodes de propagation de contraintes derrière les plateaux records.",
    engineeredTitle: "Conçu pour déjouer l'ingéniosité",
    engineeredBody:
      "Eternity I est tombé en 2000 parce qu'Alex Selby et Oliver Riordan ont compris que le puzzle admettait infiniment plus de solutions que ne le croyait son créateur, et qu'ils ont orienté leur recherche vers les régions les plus « riches en solutions ». Pour Eternity II, l'éditeur a recruté les gagnants : Selby et Riordan ont participé à la conception du nouveau puzzle et l'ont mis à l'épreuve pour qu'aucun raccourci statistique de ce type ne tienne. Les traces visibles de ce travail de blindage : une solution unique pensée dès le départ et noyée dans des répartitions de couleurs équilibrées, aucune pièce invariante par rotation, aucune pièce en double, des motifs rares confinés au cadre, et un nombre de pièces et de couleurs calé pile sur le pic de difficulté observé (ce qu'Ansótegui et al. confirmeront plus tard). Ce puzzle n'est pas difficile par hasard : il a été calibré pour l'être.",
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
      <div>
        <h1 className="text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-2 max-w-3xl text-muted-foreground">{t.intro}</p>
      </div>

      <section className="max-w-3xl space-y-3">
        <h2 className="text-2xl font-semibold tracking-tight">{t.engineeredTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.engineeredBody}</p>
      </section>

      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.toolsTitle}</h2>
        <div className="grid gap-3 md:grid-cols-3">
          <LocalizedLink to="/research/reference" className="group block">
            <Card className="h-full transition-shadow group-hover:shadow-md">
              <CardHeader className="pb-2">
                <CardTitle className="text-base group-hover:underline">{t.referenceTitle}</CardTitle>
              </CardHeader>
              <CardContent className="text-sm text-muted-foreground">{t.referenceBody}</CardContent>
            </Card>
          </LocalizedLink>
          <LocalizedLink to="/research/papers" className="group block">
            <Card className="h-full transition-shadow group-hover:shadow-md">
              <CardHeader className="pb-2">
                <CardTitle className="text-base group-hover:underline">{t.papersCardTitle}</CardTitle>
              </CardHeader>
              <CardContent className="text-sm text-muted-foreground">{t.papersCardBody}</CardContent>
            </Card>
          </LocalizedLink>
          <LocalizedLink to="/research/records" className="group block">
            <Card className="h-full transition-shadow group-hover:shadow-md">
              <CardHeader className="pb-2">
                <CardTitle className="text-base group-hover:underline">{t.recordsCardTitle}</CardTitle>
              </CardHeader>
              <CardContent className="text-sm text-muted-foreground">{t.recordsCardBody}</CardContent>
            </Card>
          </LocalizedLink>
        </div>
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
