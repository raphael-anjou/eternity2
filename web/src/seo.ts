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
  "forbidden-patterns": {
    en: {
      title: "Forbidden patterns" + SUFFIX,
      description:
        "Almost every small patch of Eternity II pieces is impossible to assemble — 99.72% of 2x2 squares can never be made to match. The exact counts, and why they explain the puzzle's difficulty.",
    },
    fr: {
      title: "Motifs interdits" + SUFFIX,
      description:
        "Presque tout petit groupe de pièces d'Eternity II est impossible à assembler — 99,72 % des carrés 2x2 ne peuvent jamais s'accorder. Les comptages exacts, et pourquoi ils expliquent la difficulté.",
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
