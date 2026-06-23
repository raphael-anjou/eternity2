import { pageMeta } from "@/seo";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { KNOWN_BOARDS } from "@/data/known-boards";

// Records we have a real, bundled board for (e2.bucas.name viewer data) get a
// clickable preview into our own /viewer. Keyed by the record row's `board`.
const BOARD_PARAMS: Record<string, string> = Object.fromEntries(
  KNOWN_BOARDS.map((b) => [b.id, b.params]),
);

// Distilled from our research vault's community history note, which is itself
// distilled from ~11,500 groups.io messages (2007–2026) + the Discord archive
// + 124 decoded community boards. `variant` flags whether a score is on the
// real (canonical 5-clue) puzzle or a different piece set, because several
// headline "480" boards are NOT the canonical puzzle.
type RecordRow = {
  date: string;
  score: string;
  author: string;
  canonical: "canonical" | "variant";
  method: string;
  // id into BOARD_PARAMS when we have a real board to preview in the viewer.
  board?: string;
};

const RECORDS: RecordRow[] = [
  { date: "2008-09", score: "467", author: "Louis Verhaard", canonical: "canonical", method: "Set-composition swap-annealing", board: "Louis_Verhaard_467" },
  { date: "2020-08-31", score: "468", author: "Joshua Blackwood", canonical: "canonical", method: "Blackwood's solver (pre-release); board relayed to the group by Jef Bucas", board: "Joshua_Blackwood_468" },
  { date: "2020-09-09", score: "469", author: "Peter McGavin", canonical: "canonical", method: "Blackwood's solver — the community ceiling", board: "JBlackwood+PMcGavin_469" },
  { date: "2020-11", score: "469", author: "various (~6–7 boards)", canonical: "canonical", method: "Blackwood's solver; mostly single-piece swaps of McGavin's" },
  { date: "2021-03-30", score: "470", author: "Joshua Blackwood", canonical: "variant", method: "1-clue Blackwood variant, retuned schedule", board: "Joshua_Blackwood_470" },
  { date: "2021", score: "470", author: "Jef Bucas", canonical: "variant", method: "Blackwood's algorithm with a different choice of heuristic colors — shows the heuristic isn't tied to one color set; C. Fernandez reported further variations" },
  { date: "2023-03-09", score: "460", author: "Bruno Gauthier", canonical: "canonical", method: "Eternity II Editor (Java) — best canonical 5-clue of 2023" },
  { date: "2023-10", score: "“480”", author: "various", canonical: "variant", method: "Mixed Clue-1 + Clue-2 piece sets — NOT the canonical puzzle" },
  { date: "2025-07", score: "470", author: "onesmallstep", canonical: "variant", method: "1-clue variant — continued 470 ties" },
];

const T = {
  en: {
    title: "Records & solvers",
    intro:
      "Eternity II has never been solved — but fifteen years of community effort have pushed the best board steadily upward, and the strongest methods are documented in the mailing list and Discord, not in journals. This page distills that history: who holds what, how they did it, and why some headline '480' boards are not actually the real puzzle.",
    bestTitle: "The state of the art",
    best:
      "The community ceiling on the real (canonical, 5-clue) puzzle is 469 of 480 matched edges — Peter McGavin, 2020, using Joshua Blackwood's solver. No published academic solver has matched it. The full solution (480) has never been found by anyone; the gap of 11 edges has stood since 2020.",
    timelineTitle: "Record timeline",
    cols: { date: "Date", score: "Score", author: "Author", puzzle: "Puzzle", method: "Method", preview: "Preview" },
    canonical: "canonical",
    variant: "variant",
    view: "View board",
    falsePositiveTitle: "Why some “480” boards don't count",
    falsePositive:
      "The real puzzle is the canonical 5-clue board TOMY sold: 256 specific pieces, five pinned clue positions, a 16×16 frame, 22 colors. Several boards posted as 480/480 use different piece sets — Brendan Owen's smaller Clue-1 / Clue-2 designs, the unframed TopCoder variant, or boards mixing pieces from multiple expansion sets. They have solutions; those solutions do not transfer to the canonical puzzle. The canonical 480 remains unfound.",
    methodsTitle: "The key algorithms",
    methods: [
      [
        "Selby–Riordan generator (2007)",
        "The puzzle was generated to have about one expected solution. Brendan Owen and Günter Stertenbrink derived in 2007 that ~17 interior colors and ~5 border colors put it exactly at the unique-solution boundary — later confirmed as the SAT/CSP phase-transition peak. The hardness is by design.",
      ],
      [
        "Verhaard set-annealing (2008)",
        "Louis Verhaard's 467 annealed on the composition of piece sets between regions, not just on positions — an early metaheuristic that held the record for over a decade.",
      ],
      [
        "Blackwood heuristic-schedule + break-index (2020)",
        "The dominant modern algorithm behind every 469/470 board. A scan-row backtracker driven by a piecewise-linear color-exhaustion schedule plus a small set of allowed 'break' depths, run for tens of billions of iterations before restart. Blackwood reported that SAT solvers, GPUs and pre-solved 2×2 caches did not help.",
      ],
      [
        "McGavin's engine (2020–2026)",
        "A C backtracker hitting ~295 million tile placements per second on one core — roughly 4× a typical community engine — via PGO, aggressive compiler flags and auto-generated per-cell unrolled code. Raw speed plus the right scan order (bottom-left start, left-to-right rows) is half the story of the 469.",
      ],
      [
        "Joe's prune-back policy (2026)",
        "A recent, concrete idea: almost all backtracking time is spent very deep, so if a search spends too long below a depth threshold without progress, prune back to that depth and restart. Measured 17–49% fewer iterations depending on clue count.",
      ],
    ],
    castTitle: "Cast of contributors",
    castNote: "A non-exhaustive who's-who of the people behind the records and the infrastructure.",
    cast: [
      ["Peter McGavin", "First verified 469 on the canonical puzzle; ~295M nps C engine; scan-order theory."],
      ["Joshua Blackwood", "The 469/470-class algorithm (heuristic schedule + break-index); the modern standard."],
      ["Jef Bucas", "libblackwood (a C port of Blackwood's solver) and bucas.name — the board viewer + URL format the community shares boards in."],
      ["Louis Verhaard", "The 467 record (2008) and set-composition swap-annealing."],
      ["Brendan Owen & Günter Stertenbrink", "The 2007 hardness derivation: why 17/5 colors sit at the unique-solution boundary."],
      ["Marijn Heule", "The canonical SAT encoding (2008) and ongoing SAT experiments."],
      ["Al Hopfer", "The NS-1 deficit invariant — a parity condition that makes the 14×14 interior tractable in minutes."],
      ["Ramon van der Werf", "A public solver blog and Wang-tile analysis of the puzzle."],
    ],
    linkBlackwoodSrc: "Blackwood's solver (original C#)",
    linkBlackwood: "Bucas's libblackwood (C port)",
    linkBlog: "Ramon van der Werf's solver blog",
    sourceNote:
      "Sourced from our research notebook's community history, itself distilled from ~11,500 mailing-list messages and the community Discord.",
    back: "Back to research",
  },
  fr: {
    title: "Records & solveurs",
    intro:
      "Eternity II n'a jamais été résolu — mais quinze ans d'efforts de la communauté ont fait progresser le meilleur plateau, et les méthodes les plus fortes sont documentées dans la liste de diffusion et sur Discord, pas dans des revues. Cette page distille cette histoire : qui détient quoi, comment, et pourquoi certains plateaux « 480 » ne sont pas le vrai puzzle.",
    bestTitle: "L'état de l'art",
    best:
      "Le plafond communautaire sur le vrai puzzle (canonique, 5 indices) est de 469 bords appariés sur 480 — Peter McGavin, 2020, avec le solveur de Joshua Blackwood. Aucun solveur académique publié ne l'a égalé. La solution complète (480) n'a jamais été trouvée ; l'écart de 11 bords tient depuis 2020.",
    timelineTitle: "Chronologie des records",
    cols: { date: "Date", score: "Score", author: "Auteur", puzzle: "Puzzle", method: "Méthode", preview: "Aperçu" },
    canonical: "canonique",
    variant: "variante",
    view: "Voir le plateau",
    falsePositiveTitle: "Pourquoi certains plateaux « 480 » ne comptent pas",
    falsePositive:
      "Le vrai puzzle est le plateau canonique à 5 indices vendu par TOMY : 256 pièces précises, cinq indices épinglés, un cadre 16×16, 22 couleurs. Plusieurs plateaux publiés en 480/480 utilisent d'autres jeux de pièces — les designs Clue-1 / Clue-2 plus petits de Brendan Owen, la variante non encadrée de TopCoder, ou des plateaux mélangeant des pièces de plusieurs jeux d'extension. Ils ont des solutions ; celles-ci ne se transposent pas au puzzle canonique. Le 480 canonique reste introuvé.",
    methodsTitle: "Les algorithmes clés",
    methods: [
      [
        "Générateur Selby–Riordan (2007)",
        "Le puzzle a été généré pour n'avoir qu'environ une solution attendue. Brendan Owen et Günter Stertenbrink ont dérivé en 2007 que ≈17 couleurs intérieures et ≈5 de bord le placent pile à la frontière d'unicité — confirmé plus tard comme le pic de transition de phase SAT/CSP. La difficulté est voulue.",
      ],
      [
        "Recuit sur jeux de Verhaard (2008)",
        "Le 467 de Louis Verhaard recuit sur la composition des jeux de pièces entre régions, pas seulement sur les positions — une métaheuristique précoce qui a tenu le record plus d'une décennie.",
      ],
      [
        "Schedule heuristique + break-index de Blackwood (2020)",
        "L'algorithme moderne dominant derrière chaque plateau 469/470. Un backtracker en balayage de lignes piloté par un objectif d'épuisement des couleurs linéaire par morceaux plus un petit ensemble de profondeurs de « rupture » autorisées, lancé sur des dizaines de milliards d'itérations avant redémarrage. Blackwood a rapporté que les solveurs SAT, les GPU et les caches de 2×2 pré-résolus n'aidaient pas.",
      ],
      [
        "Le moteur de McGavin (2020–2026)",
        "Un backtracker C atteignant ≈295 millions de placements de tuiles par seconde sur un cœur — environ 4× un moteur communautaire typique — via PGO, options de compilation agressives et code déroulé par case auto-généré. La vitesse brute plus le bon ordre de balayage (départ bas-gauche, lignes de gauche à droite) est la moitié de l'histoire du 469.",
      ],
      [
        "La politique de prune-back de Joe (2026)",
        "Une idée récente et concrète : presque tout le temps de retour sur trace se passe très en profondeur ; donc si une recherche passe trop de temps sous un seuil de profondeur sans progrès, on élague jusqu'à ce seuil et on redémarre. Mesuré : 17 à 49 % d'itérations en moins selon le nombre d'indices.",
      ],
    ],
    castTitle: "Les contributeurs",
    castNote: "Un who's-who non exhaustif des personnes derrière les records et l'infrastructure.",
    cast: [
      ["Peter McGavin", "Premier 469 vérifié sur le puzzle canonique ; moteur C ≈295M nps ; théorie de l'ordre de balayage."],
      ["Joshua Blackwood", "L'algorithme de classe 469/470 (schedule heuristique + break-index) ; le standard moderne."],
      ["Jef Bucas", "libblackwood (un port C du solveur de Blackwood) et bucas.name — le visualiseur et le format d'URL avec lequel la communauté partage les plateaux."],
      ["Louis Verhaard", "Le record 467 (2008) et le recuit par composition de jeux."],
      ["Brendan Owen & Günter Stertenbrink", "La dérivation de difficulté de 2007 : pourquoi 17/5 couleurs sont à la frontière d'unicité."],
      ["Marijn Heule", "L'encodage SAT canonique (2008) et des expériences SAT continues."],
      ["Al Hopfer", "L'invariant de déficit NS-1 — une condition de parité qui rend l'intérieur 14×14 traitable en quelques minutes."],
      ["Ramon van der Werf", "Un blog public de solveur et une analyse en tuiles de Wang du puzzle."],
    ],
    linkBlackwoodSrc: "Le solveur de Blackwood (C# original)",
    linkBlackwood: "libblackwood de Bucas (portage C)",
    linkBlog: "Le blog de solveur de Ramon van der Werf",
    sourceNote:
      "Tiré de l'historique communautaire de notre carnet de recherche, lui-même distillé de ≈11 500 messages de la liste de diffusion et du Discord de la communauté.",
    back: "Retour à la recherche",
  },
};

const BADGE: Record<RecordRow["canonical"], string> = {
  canonical: "bg-emerald-100 text-emerald-900 dark:bg-emerald-950 dark:text-emerald-200",
  variant: "bg-muted text-muted-foreground",
};

export default function Records() {
  const t = useT(T);
  return (
    <div className="space-y-12">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-2 max-w-3xl text-muted-foreground">{t.intro}</p>
      </div>

      <section className="max-w-3xl space-y-3">
        <h2 className="text-2xl font-semibold tracking-tight">{t.bestTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.best}</p>
      </section>

      <section className="space-y-3">
        <h2 className="text-2xl font-semibold tracking-tight">{t.timelineTitle}</h2>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>{t.cols.date}</TableHead>
              <TableHead className="text-right">{t.cols.score}</TableHead>
              <TableHead>{t.cols.author}</TableHead>
              <TableHead>{t.cols.puzzle}</TableHead>
              <TableHead>{t.cols.method}</TableHead>
              <TableHead>{t.cols.preview}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {RECORDS.map((r, i) => (
              <TableRow key={`${r.date}-${i}`}>
                <TableCell className="whitespace-nowrap text-muted-foreground">{r.date}</TableCell>
                <TableCell className="text-right font-semibold tabular-nums">{r.score}</TableCell>
                <TableCell className="whitespace-nowrap">{r.author}</TableCell>
                <TableCell>
                  <span
                    className={`rounded px-1.5 py-0.5 text-[10px] font-medium uppercase tracking-wide ${BADGE[r.canonical]}`}
                  >
                    {r.canonical === "canonical" ? t.canonical : t.variant}
                  </span>
                </TableCell>
                <TableCell className="text-sm text-muted-foreground">{r.method}</TableCell>
                <TableCell className="whitespace-nowrap">
                  {r.board && BOARD_PARAMS[r.board] ? (
                    <LocalizedLink
                      to={`/viewer?${BOARD_PARAMS[r.board]}`}
                      className="text-sm underline underline-offset-2 hover:text-foreground"
                    >
                      {t.view}
                    </LocalizedLink>
                  ) : (
                    <span className="text-sm text-muted-foreground">—</span>
                  )}
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </section>

      <section className="max-w-3xl space-y-3">
        <h2 className="text-xl font-semibold tracking-tight">{t.falsePositiveTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.falsePositive}</p>
      </section>

      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.methodsTitle}</h2>
        <dl className="max-w-3xl space-y-3 text-sm">
          {t.methods.map(([term, desc]) => (
            <div key={term}>
              <dt className="font-medium">{term}</dt>
              <dd className="mt-0.5 text-muted-foreground">{desc}</dd>
            </div>
          ))}
        </dl>
      </section>

      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.castTitle}</h2>
        <p className="max-w-3xl text-sm text-muted-foreground">{t.castNote}</p>
        <dl className="max-w-3xl space-y-2 text-sm">
          {t.cast.map(([name, desc]) => (
            <div key={name} className="grid grid-cols-1 gap-1 sm:grid-cols-[14rem_1fr] sm:gap-3">
              <dt className="font-medium">{name}</dt>
              <dd className="text-muted-foreground">{desc}</dd>
            </div>
          ))}
        </dl>
        <div className="flex flex-wrap gap-x-6 gap-y-1 pt-1 text-sm">
          <a
            href="https://github.com/jblackwood345/EternityII_Solver"
            target="_blank"
            rel="noreferrer"
            className="underline underline-offset-2 hover:text-foreground"
          >
            {t.linkBlackwoodSrc}
          </a>
          <a
            href="https://github.com/jfbucas/libblackwood"
            target="_blank"
            rel="noreferrer"
            className="underline underline-offset-2 hover:text-foreground"
          >
            {t.linkBlackwood}
          </a>
          <a
            href="https://ramonvdw.github.io/e2solver23/"
            target="_blank"
            rel="noreferrer"
            className="underline underline-offset-2 hover:text-foreground"
          >
            {t.linkBlog}
          </a>
        </div>
      </section>

      <p className="max-w-3xl text-xs text-muted-foreground">{t.sourceNote}</p>

      <div>
        <LocalizedLink
          to="/research"
          className="text-sm underline underline-offset-2 hover:text-foreground"
        >
          {"←"} {t.back}
        </LocalizedLink>
      </div>
    </div>
  );
}

export const meta = pageMeta("records");
