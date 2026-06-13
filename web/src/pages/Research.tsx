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
      "Pour celles et ceux qui veulent vraiment le résoudre. Les publications académiques, l'infrastructure de la communauté, et un cahier de laboratoire ouvert sur tout ce qui a été tenté, y compris par ce projet.",
    engineeredTitle: "Conçu pour résister à l'ingéniosité",
    engineeredBody:
      "Eternity I est tombé en 2000 parce qu'Alex Selby et Oliver Riordan ont découvert que le puzzle comptait beaucoup plus de solutions que son créateur ne le pensait, et ont dirigé leur recherche vers les régions les plus « denses en solutions ». Pour Eternity II, l'éditeur a embauché les vainqueurs : Selby et Riordan ont aidé à concevoir et à éprouver le nouveau puzzle, afin qu'aucun raccourci statistique de ce genre ne subsiste. Les empreintes visibles de ce travail : une solution unique voulue dès la conception, intégrée à des comptes de couleurs équilibrés, aucune pièce symétrique par rotation, aucune pièce en double, des motifs rares cantonnés au cadre, et un nombre de pièces et de couleurs placé exactement au pic de difficulté observé (confirmé plus tard par Ansótegui et al.). Ce puzzle n'est pas difficile par accident. Il a été réglé pour l'être.",
    papersTitle: "Publications",
    communityTitle: "Infrastructure de la communauté",
    paperNotes: [
      "Démontre que les puzzles d'assemblage de côtés sont NP-complets : la caution théorique du « il n'existe aucun algorithme rapide connu ».",
      "Encode des puzzles de type Eternity pour des solveurs SAT/CSP et montre que les instances générées se situent exactement au pic de difficulté.",
      "L'instance Eternity II elle-même, passée au crible : pourquoi sa taille et ses comptes de couleurs ont été si bien choisis.",
      "Ce qui se passe quand on lance des solveurs SAT industriels sur un puzzle d'assemblage de côtés (réponse : des murs, partout).",
      "Une machinerie de propagation de contraintes conçue sur mesure pour E2 : des arguments de comptage qui éliminent très tôt les régions impossibles.",
      "Programmation par contraintes et réparation par très grands voisinages : la famille de méthodes derrière la plupart des plateaux records.",
      "Les hyper-heuristiques : un algorithme qui apprend lesquels de ses propres coups fonctionnent.",
      "La programmation linéaire en nombres entiers et la recherche de cliques comme outils de réparation pour des plateaux presque complets.",
    ],
    communityNotes: [
      "Le visualiseur GPL de Jef Bucas ; son format d'URL est la langue commune de la communauté (et ce site la parle nativement).",
      "Un serveur Discord actif où les membres partagent leurs essais, leurs records et leur code, en temps réel.",
      "La liste de diffusion active : des records, des techniques et plus de 15 ans de savoir accumulé.",
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
