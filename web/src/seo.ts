// Per-page <title> and description, in both languages, for SEO and social
// previews. Keyed by route id (see routes.ts). Each page's `meta` export reads
// its entry via pageMeta(); root.tsx supplies sensible site-wide fallbacks.

import type { Lang } from "@/i18n";
import { langFromPath } from "@/i18n";
import { canonicalUrl } from "@/site";

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
  start: {
    en: {
      title: "Start here" + SUFFIX,
      description:
        "New to Eternity II, or not sure where to look? Pick who you are here as, from curious visitor to researcher, and get a short guided path through the parts of the site pitched at you.",
    },
    fr: {
      title: "Par où commencer" + SUFFIX,
      description:
        "Nouveau venu sur Eternity II, ou vous ne savez pas où chercher ? Choisissez à quel titre vous êtes là, du simple curieux au chercheur, et suivez un court parcours guidé dans les parties du site faites pour vous.",
    },
  },
  status: {
    en: {
      title: "Has Eternity II been solved?" + SUFFIX,
      description:
        "No. The current record is 470 of 480 matched edges (Joshua Blackwood, 2021), tied but never beaten. The full solution has never been found. Updated July 2026.",
    },
    fr: {
      title: "Eternity II a-t-il été résolu ?" + SUFFIX,
      description:
        "Non. Le record actuel est de 470 arêtes appariées sur 480 (Joshua Blackwood, 2021), égalé mais jamais battu. La solution complète n'a jamais été trouvée. À jour en juillet 2026.",
    },
  },
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
  scam: {
    en: {
      title: "Is Eternity II a scam? Can it be solved?" + SUFFIX,
      description:
        "No, it is not a scam, and yes, a solution almost certainly exists. The sourced facts on the $2,000,000 puzzle that expired with no winner, and why hard is not the same as impossible.",
    },
    fr: {
      title: "Eternity II, une arnaque ? A-t-il une solution ?" + SUFFIX,
      description:
        "Non, ce n'est pas une arnaque, et oui, une solution existe presque à coup sûr. Les faits sourcés sur le puzzle à 2 000 000 $ resté sans vainqueur, et pourquoi difficile ne veut pas dire impossible.",
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
  // "border-balance" migrated to web/content/research/why/border-balance.mdx —
  // MDX pages carry their SEO meta in frontmatter.
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
  convert: {
    en: {
      title: "Format converter" + SUFFIX,
      description:
        "Convert between Eternity II board formats: bucas URLs, board_edges letter strings and board_pieces numbers. Paste one, get the others, with a live preview.",
    },
    fr: {
      title: "Convertisseur de formats" + SUFFIX,
      description:
        "Passez d'un format de plateau Eternity II à l'autre : liens bucas, chaînes board_edges et numéros board_pieces. Collez-en un, récupérez les autres, avec un aperçu.",
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
      { property: "og:url", content: canonicalUrl(location.pathname) },
    ];
  };
}
