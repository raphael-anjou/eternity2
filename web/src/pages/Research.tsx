import { pageMeta } from "@/seo";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useT } from "@/i18n";

// Academic citations (authors, titles, venues, years) are not translated.
// Only the per-entry notes are localized, via T.paperNotes / T.communityNotes,
// which are indexed in the same order as these arrays.
const PAPERS = [
  {
    authors: "E. D. Demaine, M. L. Demaine",
    year: 2007,
    title: "Jigsaw Puzzles, Edge Matching, and Polyomino Packing: Connections and Complexity",
    venue: "Graphs and Combinatorics 23",
    url: "https://erikdemaine.org/papers/Jigsaw_GC/",
  },
  {
    authors: "C. Ansótegui, R. Béjar, C. Fernández, C. Mateu",
    year: 2008,
    title: "Edge Matching Puzzles as Hard SAT/CSP Benchmarks",
    venue: "CP 2008",
    url: "https://link.springer.com/chapter/10.1007/978-3-540-85958-1_39",
  },
  {
    authors: "C. Ansótegui, R. Béjar, C. Fernández, C. Mateu",
    year: 2008,
    title: "How Hard is a Commercial Puzzle: the Eternity II Challenge",
    venue: "CCIA 2008",
    url: "https://www.iiia.csic.es/~levy/papers/CCIA08b.pdf",
  },
  {
    authors: "M. J. H. Heule",
    year: 2009,
    title: "Solving edge-matching problems with satisfiability solvers",
    venue: "SAT 2009 workshop",
    url: "https://www.cs.cmu.edu/~mheule/publications/eternity.pdf",
  },
  {
    authors: "T. Benoist, E. Bourreau",
    year: 2008,
    title: "Fast Global Filtering for Eternity II",
    venue: "Constraint Programming Letters 3",
    url: "https://www.cs.uic.edu/pub/ConstraintProgrammingLetters/Volume3/BenoistBourreau.pdf",
  },
  {
    authors: "P. Schaus, Y. Deville",
    year: 2008,
    title: "Hybridization of CP and VLNS for Eternity II",
    venue: "JFPC 2008",
    url: "https://webperso.info.ucl.ac.be/~pschaus/assets/publi/jfpc2008_eternity.pdf",
  },
  {
    authors: "T. Wauters, W. Vancroonenburg, G. Vanden Berghe",
    year: 2012,
    title: "A guide-and-observe hyper-heuristic approach to the Eternity II puzzle",
    venue: "J. Math. Modelling and Algorithms",
    url: "https://link.springer.com/article/10.1007/s10852-012-9178-4",
  },
  {
    authors: "F. Salassa, W. Vancroonenburg, T. Wauters, F. Della Croce, G. Vanden Berghe",
    year: 2017,
    title: "MILP and Max-Clique based heuristics for the Eternity II puzzle",
    venue: "arXiv:1709.00252",
    url: "https://arxiv.org/abs/1709.00252",
  },
];

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
    engineeredTitle: "Engineered to resist cleverness",
    engineeredBody:
      "Eternity I fell in 2000 because Alex Selby and Oliver Riordan discovered the puzzle had vastly more solutions than its designer believed, and aimed their search at the most \"solution-dense\" regions. For Eternity II, the publisher hired the winners: Selby and Riordan helped design and stress-test the new puzzle so that no such statistical shortcut survives. The visible fingerprints of that vetting: a single designed solution baked into balanced color counts, no rotationally-symmetric pieces, no duplicate pieces, rare motifs confined to the frame, and piece-count/color-count parameters sitting at the empirical hardness peak (later confirmed by Ansótegui et al.). The puzzle isn't accidentally hard. It was tuned to be.",
    papersTitle: "Papers",
    communityTitle: "Community infrastructure",
    paperNotes: [
      "Proves edge-matching puzzles NP-complete: the theoretical license for 'there is no known fast algorithm'.",
      "Encodes Eternity-style puzzles for SAT/CSP solvers and shows generated instances sit right at the hardness peak.",
      "The Eternity II instance itself, analyzed: why its size and color counts were chosen so well.",
      "What happens when you throw industrial SAT solvers at edge matching (spoiler: walls, everywhere).",
      "Constraint-propagation machinery purpose-built for E2: counting arguments that prune impossible regions early.",
      "Constraint programming plus very-large-neighborhood repair: the family of methods behind most record boards.",
      "Hyper-heuristics: an algorithm that learns which of its own moves are working.",
      "Mixed-integer programming and clique-finding as repair tools for near-complete boards.",
    ],
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
    engineeredTitle: "Conçu pour déjouer l'ingéniosité",
    engineeredBody:
      "Eternity I est tombé en 2000 parce qu'Alex Selby et Oliver Riordan ont compris que le puzzle admettait infiniment plus de solutions que ne le croyait son créateur, et qu'ils ont orienté leur recherche vers les régions les plus « riches en solutions ». Pour Eternity II, l'éditeur a recruté les gagnants : Selby et Riordan ont participé à la conception du nouveau puzzle et l'ont mis à l'épreuve pour qu'aucun raccourci statistique de ce type ne tienne. Les traces visibles de ce travail de blindage : une solution unique pensée dès le départ et noyée dans des répartitions de couleurs équilibrées, aucune pièce invariante par rotation, aucune pièce en double, des motifs rares confinés au cadre, et un nombre de pièces et de couleurs calé pile sur le pic de difficulté observé (ce qu'Ansótegui et al. confirmeront plus tard). Ce puzzle n'est pas difficile par hasard : il a été calibré pour l'être.",
    papersTitle: "Articles scientifiques",
    communityTitle: "Les outils de la communauté",
    paperNotes: [
      "Prouve que les casse-tête d'assemblage par les bords (edge-matching) sont NP-complets : la caution théorique du « on ne connaît aucun algorithme rapide ».",
      "Traduit les puzzles de type Eternity pour les solveurs SAT/CSP et montre que les instances ainsi générées tombent pile au pic de difficulté.",
      "L'instance Eternity II elle-même, disséquée : pourquoi sa taille et son nombre de couleurs ont été si bien choisis.",
      "Ce qui arrive quand on lâche des solveurs SAT industriels sur un puzzle d'assemblage par les bords (verdict : des murs, partout).",
      "Une mécanique de propagation de contraintes taillée sur mesure pour E2 : des arguments de comptage qui écartent très tôt les régions impossibles.",
      "Programmation par contraintes et réparation par très grands voisinages : la famille de méthodes derrière la plupart des plateaux records.",
      "Les hyper-heuristiques : un algorithme qui apprend lesquels de ses propres coups portent leurs fruits.",
      "La programmation linéaire en nombres entiers et la recherche de cliques comme outils de réparation pour les plateaux presque complets.",
    ],
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
        <h2 className="text-2xl font-semibold tracking-tight">{t.papersTitle}</h2>
        <div className="grid gap-3">
          {PAPERS.map((p, i) => (
            <a key={p.title} href={p.url} target="_blank" rel="noreferrer" className="group">
              <Card className="transition-shadow group-hover:shadow-md">
                <CardContent className="pt-4">
                  <div className="flex flex-wrap items-baseline gap-x-2">
                    <span className="font-medium group-hover:underline">{p.title}</span>
                    <span className="text-xs text-muted-foreground">
                      {p.authors} · {p.venue} · {p.year}
                    </span>
                  </div>
                  <p className="mt-1 text-sm text-muted-foreground">{t.paperNotes[i]}</p>
                </CardContent>
              </Card>
            </a>
          ))}
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
