// Per-page <title> and description, in both languages, for SEO and social
// previews. Keyed by route id (see routes.ts). Each page's `meta` export reads
// its entry via pageMeta(); root.tsx supplies sensible site-wide fallbacks.

import type { Lang } from "@/i18n";
import { langFromPath } from "@/i18n";
import { absoluteUrl } from "@/site";

type Entry = { title: string; description: string };

const SUFFIX = " · Eternity II";

// Default page metadata, used as the fallback for any unknown page key.
const HOME_PAGE: { en: Entry; fr: Entry } = {
  en: {
    title: "Eternity II: the puzzle that beat everyone",
    description:
      "Play it, watch real solvers run in your browser, learn the algorithms, explore the research. The $2,000,000 puzzle nobody ever solved.",
  },
  fr: {
    title: "Eternity II : le puzzle que personne n'a battu",
    description:
      "Jouez, regardez de vrais solveurs à l'œuvre dans votre navigateur, découvrez les algorithmes et la recherche. Le casse-tête à 2 000 000 $ resté invaincu.",
  },
};

const PAGES: Record<string, { en: Entry; fr: Entry }> = {
  home: HOME_PAGE,
  puzzle: {
    en: {
      title: "The Puzzle" + SUFFIX,
      description:
        "What Eternity II actually is: 256 edge-matching tiles, 22 colors, a 16×16 board, and a prize that went unclaimed.",
    },
    fr: {
      title: "Le Puzzle" + SUFFIX,
      description:
        "Eternity II en clair : 256 tuiles à côtés appariés, 22 couleurs, un plateau 16×16 et un prix jamais remporté.",
    },
  },
  playground: {
    en: {
      title: "Playground" + SUFFIX,
      description:
        "Watch solvers run, race fill orders, and solve puzzles by hand — all powered by a Rust engine in your browser.",
    },
    fr: {
      title: "Aire de jeu" + SUFFIX,
      description:
        "Regardez les solveurs à l'œuvre, comparez les ordres de remplissage et résolvez le puzzle à la main — le tout propulsé par un moteur Rust, dans votre navigateur.",
    },
  },
  watch: {
    en: {
      title: "Watch a solver" + SUFFIX,
      description: "Watch a real depth-first solver search Eternity II live, step by step.",
    },
    fr: {
      title: "Regarder un solveur" + SUFFIX,
      description: "Suivez en direct un vrai solveur explorer Eternity II, étape par étape.",
    },
  },
  solve: {
    en: {
      title: "Solve by hand" + SUFFIX,
      description: "Place edge-matching tiles yourself and feel why Eternity II is so hard.",
    },
    fr: {
      title: "Résoudre à la main" + SUFFIX,
      description: "Placez vous-même les tuiles à côtés appariés et comprenez pourquoi Eternity II est si redoutable.",
    },
  },
  paths: {
    en: {
      title: "Fill orders" + SUFFIX,
      description: "Design the order a solver fills the board in, and race it against the classics.",
    },
    fr: {
      title: "Ordres de remplissage" + SUFFIX,
      description: "Choisissez l'ordre dans lequel le solveur remplit le plateau, puis défiez les ordres classiques.",
    },
  },
  print: {
    en: {
      title: "Print & play" + SUFFIX,
      description: "Generate printable Eternity II piece sheets to cut out and solve on paper.",
    },
    fr: {
      title: "Imprimer & jouer" + SUFFIX,
      description: "Générez des planches de pièces Eternity II à imprimer, à découper et à résoudre sur papier.",
    },
  },
  algorithms: {
    en: {
      title: "Algorithms" + SUFFIX,
      description:
        "How computers attack Eternity II — depth-first search, backtracking, the exponential wall and the color/size difficulty peak — with live measurements.",
    },
    fr: {
      title: "Algorithmes" + SUFFIX,
      description:
        "Comment les ordinateurs s'attaquent à Eternity II : recherche en profondeur, retour en arrière (backtracking), mur exponentiel et pic de difficulté couleur/taille, mesures à l'appui.",
    },
  },
  research: {
    en: {
      title: "Research" + SUFFIX,
      description: "The records, the methods and the open questions behind the unsolved Eternity II puzzle.",
    },
    fr: {
      title: "Recherche" + SUFFIX,
      description: "Les records, les méthodes et les grandes questions encore ouvertes autour d'Eternity II, le puzzle jamais résolu.",
    },
  },
  why: {
    en: {
      title: "Why it's hard" + SUFFIX,
      description:
        "Why Eternity II resists every approach: a puzzle designed to defeat statistical shortcuts, and the structural walls — rigidity, entropy, forbidden patterns — that explain the gap to a full solution.",
    },
    fr: {
      title: "Pourquoi c'est dur" + SUFFIX,
      description:
        "Pourquoi Eternity II déjoue toutes les approches : un puzzle conçu pour contrer les raccourcis statistiques, et les murs structurels — rigidité, entropie, motifs interdits — qui expliquent l'écart jusqu'à une solution complète.",
    },
  },
  build: {
    en: {
      title: "Build a solver" + SUFFIX,
      description:
        "Everything you need to write an Eternity II solver: validation data, the literature ranked by usefulness, the record timeline and methods, the dead ends, and how to run the code yourself.",
    },
    fr: {
      title: "Construire un solveur" + SUFFIX,
      description:
        "Tout pour écrire un solveur Eternity II : données de validation, littérature classée par utilité, chronologie des records et méthodes, impasses, et comment exécuter le code vous-même.",
    },
  },
  lab: {
    en: {
      title: "The lab notebook" + SUFFIX,
      description:
        "An open notebook of original Eternity II research: structural findings, the named algorithms built to attack the puzzle, and notable boards — all reproducible from source.",
    },
    fr: {
      title: "Le carnet de laboratoire" + SUFFIX,
      description:
        "Un carnet ouvert de recherche originale sur Eternity II : résultats structurels, algorithmes nommés conçus pour attaquer le puzzle, et plateaux notables — le tout reproductible depuis les sources.",
    },
  },
  experiments: {
    en: {
      title: "Experiments" + SUFFIX,
      description:
        "The named search experiments built to push the Eternity II score, each written up with its idea, its result, and the questions it left open. The best reaches 463 of 480; the community's best is 469.",
    },
    fr: {
      title: "Expériences" + SUFFIX,
      description:
        "Les expériences de recherche nommées, conçues pour pousser le score d'Eternity II, chacune rédigée avec son idée, son résultat et les questions laissées ouvertes. La meilleure atteint 463 sur 480 ; le meilleur de la communauté est 469.",
    },
  },
  "experiments-log": {
    en: {
      title: "Experiments log" + SUFFIX,
      description:
        "Everything we tried on Eternity II and how it turned out: hundreds of attempts — successes, half-built ideas, and dead ends — searchable and filterable by outcome, so you can build on them.",
    },
    fr: {
      title: "Journal des expériences" + SUFFIX,
      description:
        "Tout ce que nous avons essayé sur Eternity II et le résultat : des centaines de tentatives — succès, idées à moitié construites et impasses — cherchables et filtrables par résultat, pour bâtir dessus.",
    },
  },
  findings: {
    en: {
      title: "Findings" + SUFFIX,
      description:
        "The standing structural results about Eternity II: design signatures, the walls no local method gets past, and the proofs of why the records are stuck. All reproducible.",
    },
    fr: {
      title: "Résultats" + SUFFIX,
      description:
        "Les résultats structurels établis sur Eternity II : signatures de conception, murs qu'aucune méthode locale ne franchit, et preuves expliquant pourquoi les records sont bloqués. Tout reproductible.",
    },
  },
  basins: {
    en: {
      title: "Notable boards" + SUFFIX,
      description:
        "A gallery of notable Eternity II boards: community records and the best boards this project produced, each opening in the viewer to check edge by edge.",
    },
    fr: {
      title: "Plateaux notables" + SUFFIX,
      description:
        "Une galerie de plateaux Eternity II notables : records de la communauté et meilleurs plateaux de ce projet, chacun s'ouvrant dans le visualiseur pour vérifier bord par bord.",
    },
  },
  inventions: {
    en: {
      title: "Inventions" + SUFFIX,
      description:
        "The named algorithms built to push the Eternity II score: PALIMPSEST (463), KEYRING (460), GAUNTLET (458) and more. Each idea, its result, and the open questions, with verified boards.",
    },
    fr: {
      title: "Inventions" + SUFFIX,
      description:
        "Les algorithmes nommés conçus pour pousser le score d'Eternity II : PALIMPSEST (463), KEYRING (460), GAUNTLET (458) et d'autres. Chaque idée, son résultat et les questions ouvertes, avec des plateaux vérifiés.",
    },
  },
  "exp-palimpsest": {
    en: {
      title: "PALIMPSEST" + SUFFIX,
      description:
        "Read every strong Eternity II board to find the habits that quietly hold a board back, then break them. Produced the project's best board: 463 of 480.",
    },
    fr: {
      title: "PALIMPSEST" + SUFFIX,
      description:
        "Lire tous les bons plateaux d'Eternity II pour repérer les habitudes qui freinent un plateau, puis les casser. A produit le meilleur plateau du projet : 463 sur 480.",
    },
  },
  "exp-keyring": {
    en: {
      title: "KEYRING" + SUFFIX,
      description:
        "Build an Eternity II board from scratch, ranking each next piece by three signals learned from strong boards. Reached 460 in a new board family.",
    },
    fr: {
      title: "KEYRING" + SUFFIX,
      description:
        "Construire un plateau Eternity II de zéro en classant chaque pièce par trois signaux appris des bons plateaux. A atteint 460 dans une nouvelle famille.",
    },
  },
  "exp-gauntlet": {
    en: {
      title: "GAUNTLET" + SUFFIX,
      description:
        "Run the Eternity II beam search in several fill directions at once to reach different regions. Found a new 458 board in a fresh family.",
    },
    fr: {
      title: "GAUNTLET" + SUFFIX,
      description:
        "Lancer la recherche en faisceau d'Eternity II dans plusieurs directions à la fois pour atteindre des régions différentes. A trouvé un nouveau plateau 458.",
    },
  },
  "exp-prior": {
    en: {
      title: "PRIOR" + SUFFIX,
      description:
        "Build an Eternity II board from nothing, guided by where pieces tend to sit in strong boards. Reaches 460 from scratch, no anchor board copied.",
    },
    fr: {
      title: "PRIOR" + SUFFIX,
      description:
        "Construire un plateau Eternity II à partir de rien, guidé par l'endroit où les pièces se trouvent dans les bons plateaux. Atteint 460 de zéro, sans copier de plateau d'ancrage.",
    },
  },
  "exp-staged": {
    en: {
      title: "STAGED" + SUFFIX,
      description:
        "Build the whole Eternity II board from scratch with no pre-set frame, letting the border emerge last. Reaches 436 frame-free, measuring what the frame anchor is worth.",
    },
    fr: {
      title: "STAGED" + SUFFIX,
      description:
        "Construire tout le plateau Eternity II de zéro sans cadre préétabli, en laissant la bordure émerger en dernier. Atteint 436 sans cadre, mesurant ce que vaut l'ancrage par le cadre.",
    },
  },
  "exp-mosaic": {
    en: {
      title: "MOSAIC" + SUFFIX,
      description:
        "Tile Eternity II into 4×4 blocks, solve each to exact MaxSAT optimality with soft seams, and reserve scarce pieces for later blocks. From scratch, no warm start: 448/480.",
    },
    fr: {
      title: "MOSAIC" + SUFFIX,
      description:
        "Découper Eternity II en blocs 4×4, résoudre chacun à l'optimum MaxSAT exact avec des jointures souples, et réserver les pièces rares aux blocs suivants. De zéro, sans amorce : 448/480.",
    },
  },
  "exp-lodestone": {
    en: {
      title: "LODESTONE" + SUFFIX,
      description:
        "A scarce-demand prior that nudges a from-scratch Eternity II beam to commit rare pieces early. A tiebreaker only: a consistent median +2 and tighter variance, not a record-mover.",
    },
    fr: {
      title: "LODESTONE" + SUFFIX,
      description:
        "Un prior de demande rare qui pousse un faisceau Eternity II parti de zéro à engager tôt les pièces rares. Un simple départage : +2 médian constant et variance resserrée, pas un déplaceur de record.",
    },
  },
  "exp-bandsaw": {
    en: {
      title: "BANDSAW" + SUFFIX,
      description:
        "Solve a band of Eternity II rows exactly by meeting in the middle, proving the best ending and measuring how far ahead an endgame can be decided.",
    },
    fr: {
      title: "BANDSAW" + SUFFIX,
      description:
        "Résoudre exactement une bande de rangées d'Eternity II en se rejoignant au milieu, prouvant la meilleure fin et mesurant jusqu'où une fin de partie se décide à l'avance.",
    },
  },
  "exp-cloister": {
    en: {
      title: "CLOISTER" + SUFFIX,
      description:
        "Fix a perfect Eternity II border, then search the interior with the frame's edges as hard constraints from cell one. Reaches a 453 standalone interior and measures border-interior coupling.",
    },
    fr: {
      title: "CLOISTER" + SUFFIX,
      description:
        "Fixer une bordure Eternity II parfaite, puis chercher l'intérieur avec les bords du cadre comme contraintes dures dès la première cellule. Atteint un intérieur autonome 453 et mesure le couplage bordure-intérieur.",
    },
  },
  "exp-midden": {
    en: {
      title: "MIDDEN" + SUFFIX,
      description:
        "Decide where, not when, an Eternity II board may break: confine mismatches to a chosen mask and search its shape. A dispersed lattice extends the perfect run to the 170s.",
    },
    fr: {
      title: "MIDDEN" + SUFFIX,
      description:
        "Décider où, et non quand, un plateau Eternity II peut casser : confiner les défauts à un masque choisi et chercher sa forme. Un réseau dispersé prolonge la suite parfaite jusqu'aux 170.",
    },
  },
  "exp-ladder": {
    en: {
      title: "LADDER" + SUFFIX,
      description:
        "Flood Eternity II with cheap short searches, keep the deepest starts, and promote survivors through longer rounds. Reached a 451 strict board with no record to copy.",
    },
    fr: {
      title: "LADDER" + SUFFIX,
      description:
        "Inonder Eternity II de recherches courtes et bon marché, garder les départs les plus profonds, et promouvoir les survivants. A atteint un plateau 451 strict sans record à copier.",
    },
  },
  "exp-replay": {
    en: {
      title: "REPLAY" + SUFFIX,
      description:
        "Rebuild the community's strict 460 boards exactly and discover the move ordinary solvers miss: paying two mismatches at one cell. Explains the 457-458 plateau.",
    },
    fr: {
      title: "REPLAY" + SUFFIX,
      description:
        "Reconstruire exactement les plateaux 460 stricts de la communauté et découvrir le coup que les solveurs manquent : payer deux défauts sur une cellule. Explique le plateau 457-458.",
    },
  },
  solvers: {
    en: {
      title: "Solver catalogue" + SUFFIX,
      description:
        "How the Eternity II record solvers actually search: backtracking, the fill-order effect, Blackwood's schedule + break-index, and McGavin's high-throughput engine. With a live solver to watch.",
    },
    fr: {
      title: "Catalogue des solveurs" + SUFFIX,
      description:
        "Comment les solveurs records d'Eternity II cherchent : backtracking, effet de l'ordre de remplissage, calendrier + index de rupture de Blackwood, et moteur haut débit de McGavin. Avec un solveur en direct à regarder.",
    },
  },
  "dead-ends": {
    en: {
      title: "Dead ends" + SUFFIX,
      description:
        "Approaches that look promising for Eternity II but don't work: symmetry breaking, more compute, survey propagation, tensor counting, relaxation bounds, learned heuristics. What's known, so you don't waste the time.",
    },
    fr: {
      title: "Impasses" + SUFFIX,
      description:
        "Des approches qui semblent prometteuses pour Eternity II mais qui échouent : casser les symétries, plus de calcul, survey propagation, comptage par tenseurs, bornes de relaxation, heuristiques apprises. Ce qu'on sait, pour ne pas perdre de temps.",
    },
  },
  // "border-balance" migrated to web/content/research/why/border-balance.mdx —
  // MDX pages carry their SEO meta in frontmatter.
  "run-it-yourself": {
    en: {
      title: "Run it yourself" + SUFFIX,
      description:
        "Clone the repository, build the Eternity II engine, run the site locally, and reproduce every research result with a handful of commands.",
    },
    fr: {
      title: "À vous de jouer" + SUFFIX,
      description:
        "Clonez le dépôt, compilez le moteur Eternity II, lancez le site en local et reproduisez chaque résultat de recherche en quelques commandes.",
    },
  },
  reference: {
    en: {
      title: "Reference numbers" + SUFFIX,
      description: "Exact placement counts for small blocks of the official Eternity II board — known-good values to validate your own solver against.",
    },
    fr: {
      title: "Valeurs de référence" + SUFFIX,
      description: "Comptages exacts de placements pour de petits blocs du plateau officiel d'Eternity II — des valeurs sûres pour valider votre propre solveur.",
    },
  },
  papers: {
    en: {
      title: "Papers" + SUFFIX,
      description: "The academic literature on Eternity II and edge-matching puzzles, ranked by how useful each paper is if you actually want to write a solver.",
    },
    fr: {
      title: "Articles scientifiques" + SUFFIX,
      description: "La littérature scientifique sur Eternity II et les casse-tête d'assemblage par les bords, classée selon son utilité réelle pour écrire un solveur.",
    },
  },
  records: {
    en: {
      title: "Records & solvers" + SUFFIX,
      description: "The Eternity II record timeline — who reached 469/480 and how — plus the key community solvers and why some '480' boards aren't the real puzzle.",
    },
    fr: {
      title: "Records & solveurs" + SUFFIX,
      description: "La chronologie des records d'Eternity II — qui a atteint 469/480 et comment — les principaux solveurs de la communauté et pourquoi certains plateaux « 480 » ne sont pas le vrai puzzle.",
    },
  },
  viewer: {
    en: {
      title: "Board Viewer" + SUFFIX,
      description: "Paste a board and inspect it: score, conflicts, and known record solutions to Eternity II.",
    },
    fr: {
      title: "Visualiseur" + SUFFIX,
      description: "Collez un plateau et passez-le au crible : score, conflits et meilleures solutions connues d'Eternity II.",
    },
  },
};

/**
 * Bare page title (suffix stripped) for a registered page key — used by the
 * research sidebar to label not-yet-migrated TSX pages without duplicating
 * their titles. Goes away when the research migration to MDX completes.
 */
export function pageTitle(pageKey: string, lang: Lang): string {
  const entry = (PAGES[pageKey] ?? HOME_PAGE)[lang];
  return entry.title.replace(SUFFIX, "");
}

/** Page description for a registered page key (research search + nav). */
export function pageDescription(pageKey: string, lang: Lang): string {
  return (PAGES[pageKey] ?? HOME_PAGE)[lang].description;
}

/**
 * Build a React Router `meta` descriptor list for a page. `pageKey` matches a
 * PAGES entry; the language comes from the URL of the page being prerendered.
 */
export function pageMeta(pageKey: keyof typeof PAGES) {
  return ({ location }: { location: { pathname: string } }) => {
    const lang: Lang = langFromPath(location.pathname);
    const group = PAGES[pageKey];
    const entry = (group ?? HOME_PAGE)[lang];
    return [
      { title: entry.title },
      { name: "description", content: entry.description },
      { property: "og:title", content: entry.title },
      { property: "og:description", content: entry.description },
      { property: "og:url", content: absoluteUrl(location.pathname) },
    ];
  };
}
