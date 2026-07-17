// Per-page <title> and description, in both languages, for SEO and social
// previews. Keyed by route id (see routes.ts). Each page's `meta` export reads
// its entry via pageMeta(); root.tsx supplies sensible site-wide fallbacks.

import type { Lang } from "@/i18n";
import { langFromPath } from "@/i18n";
import { pageUpdated } from "@/page-updated";
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
        "New to Eternity II, or not sure where to look? Tell us who you are, from curious visitor to researcher, and get a short guided path through the site.",
    },
    fr: {
      title: "Par où commencer" + SUFFIX,
      description:
        "Nouveau sur Eternity II, ou perdu ? Dites-nous qui vous êtes, du simple curieux au chercheur, et suivez un court parcours guidé à travers le site.",
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
        "Non. Le record est de 470 arêtes sur 480 (Joshua Blackwood, 2021), égalé mais jamais battu. La solution complète reste introuvable. À jour en juillet 2026.",
    },
  },
  puzzle: {
    en: {
      title: "What is the Eternity II puzzle?" + SUFFIX,
      description:
        "What the Eternity II puzzle actually is: 256 edge-matching tiles, 22 colors, a 16×16 board, and a $2,000,000 prize that went unclaimed.",
    },
    fr: {
      title: "Qu'est-ce que le puzzle Eternity II ?" + SUFFIX,
      description:
        "Le puzzle Eternity II en clair : 256 tuiles à côtés appariés, 22 couleurs, un plateau 16×16 et un prix de 2 000 000 $ jamais remporté.",
    },
  },
  scam: {
    en: {
      title: "Is Eternity II a scam? Can it be solved?" + SUFFIX,
      description:
        "No, it is not a scam, and yes, a solution almost certainly exists. The sourced facts on the $2,000,000 puzzle that expired with no winner.",
    },
    fr: {
      title: "Eternity II, une arnaque ? A-t-il une solution ?" + SUFFIX,
      description:
        "Non, ce n'est pas une arnaque, et oui, une solution existe presque à coup sûr. Les faits sourcés sur le puzzle à 2 000 000 $ resté sans vainqueur.",
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
        "Regardez les solveurs à l'œuvre, comparez les ordres de remplissage et résolvez le puzzle à la main, propulsé par un moteur Rust dans votre navigateur.",
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
      title: "How to solve Eternity II with a computer" + SUFFIX,
      description:
        "How computers attack Eternity II — depth-first search, backtracking, the exponential wall and the color/size difficulty peak — with live measurements.",
    },
    fr: {
      title: "Comment résoudre Eternity II par ordinateur" + SUFFIX,
      description:
        "Comment les ordinateurs s'attaquent à Eternity II : recherche en profondeur, retour en arrière, mur exponentiel et pic de difficulté, mesures à l'appui.",
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
        "Passez d'un format de plateau Eternity II à l'autre : liens bucas, chaînes board_edges, numéros board_pieces. Collez-en un, récupérez les autres.",
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

// FAQ structured data for the two question-titled pages. Both answer a real
// query people type ("has Eternity II been solved?", "is it a scam?"), so a
// FAQPage node makes them eligible for FAQ rich results and gives AI answers a
// clean, sourced Q&A to lift. Answers mirror the sourced facts on the pages
// themselves; keep them in sync when a record changes.
type QA = { q: string; a: string };
const PAGE_FAQ: Partial<Record<keyof typeof PAGES, { en: QA[]; fr: QA[] }>> = {
  puzzle: {
    en: [
      {
        q: "What is the Eternity II puzzle?",
        a: "Eternity II is an edge-matching puzzle released in 2007: 256 square tiles must fill a 16×16 board so that every shared edge shows the same colored pattern, with the border edges grey. It carried a $2,000,000 prize for the first full solution.",
      },
      {
        q: "How many pieces does Eternity II have?",
        a: "256 square pieces, each with a colored half-motif on all four edges, drawn from 22 different colors/motifs. They fill a 16×16 grid — 480 internal edges that must all match.",
      },
      {
        q: "Why is Eternity II so hard?",
        a: "The number of ways to arrange the pieces is about 1.115 × 10^557 — vastly more than the atoms in the observable universe. No algorithm can check them all, and the puzzle was deliberately designed to resist brute-force computer search.",
      },
    ],
    fr: [
      {
        q: "Qu'est-ce que le puzzle Eternity II ?",
        a: "Eternity II est un puzzle d'appariement de côtés sorti en 2007 : 256 tuiles carrées doivent remplir un plateau 16×16 de sorte que chaque côté partagé montre le même motif coloré, avec un bord gris. Il était doté d'un prix de 2 000 000 $ pour la première solution complète.",
      },
      {
        q: "Combien de pièces compte Eternity II ?",
        a: "256 pièces carrées, chacune portant un demi-motif coloré sur ses quatre côtés, parmi 22 couleurs/motifs. Elles remplissent une grille 16×16, soit 480 arêtes internes à faire toutes coïncider.",
      },
      {
        q: "Pourquoi Eternity II est-il si difficile ?",
        a: "Le nombre d'arrangements possibles est d'environ 1,115 × 10^557 — bien plus que le nombre d'atomes dans l'univers observable. Aucun algorithme ne peut tous les vérifier, et le puzzle a été conçu pour résister à la recherche par force brute.",
      },
    ],
  },
  status: {
    en: [
      {
        q: "Has Eternity II been solved?",
        a: "No. As of July 2026 the puzzle has never been fully solved. The best public board matches 470 of 480 edges (Joshua Blackwood, 2021), tied but never beaten, and no perfect 480-edge solution has ever been found.",
      },
      {
        q: "What is the record score for Eternity II?",
        a: "470 of 480 matched edges, set by Joshua Blackwood in 2021. Earlier notable boards reached 467 (Louis Verhaard) and 469 (Blackwood & McGavin).",
      },
      {
        q: "Is the $2,000,000 prize still available?",
        a: "No. The prize deadline passed on 31 December 2010 with no complete solution submitted, so the $2,000,000 went unclaimed and the contest is closed.",
      },
    ],
    fr: [
      {
        q: "Eternity II a-t-il été résolu ?",
        a: "Non. En juillet 2026, le puzzle n'a jamais été entièrement résolu. Le meilleur plateau public apparie 470 des 480 arêtes (Joshua Blackwood, 2021), égalé mais jamais battu, et aucune solution parfaite à 480 arêtes n'a jamais été trouvée.",
      },
      {
        q: "Quel est le record d'Eternity II ?",
        a: "470 arêtes appariées sur 480, établi par Joshua Blackwood en 2021. Des plateaux notables antérieurs avaient atteint 467 (Louis Verhaard) et 469 (Blackwood & McGavin).",
      },
      {
        q: "Le prix de 2 000 000 $ est-il toujours à gagner ?",
        a: "Non. La date limite du prix, le 31 décembre 2010, est passée sans solution complète soumise : les 2 000 000 $ n'ont jamais été remportés et le concours est clos.",
      },
    ],
  },
  scam: {
    en: [
      {
        q: "Is Eternity II a scam?",
        a: "No. It was a genuine contest with a real, escrowed $2,000,000 prize. The prize simply expired unclaimed in 2010 because no one found a complete solution in time — the puzzle is extraordinarily hard, not rigged.",
      },
      {
        q: "Does Eternity II have a solution?",
        a: "Almost certainly yes. Statistical estimates of the piece set imply many perfect solutions exist; the difficulty is finding one in a search space of about 1.1 × 10^557 arrangements, not whether one exists.",
      },
    ],
    fr: [
      {
        q: "Eternity II est-il une arnaque ?",
        a: "Non. C'était un vrai concours doté d'un prix réel de 2 000 000 $ placé sous séquestre. Le prix a simplement expiré sans vainqueur en 2010 faute de solution complète trouvée à temps : le puzzle est extraordinairement difficile, pas truqué.",
      },
      {
        q: "Eternity II a-t-il une solution ?",
        a: "Presque certainement oui. Les estimations statistiques du jeu de pièces impliquent l'existence de nombreuses solutions parfaites ; la difficulté est d'en trouver une dans un espace de recherche d'environ 1,1 × 10^557 arrangements, pas de savoir s'il en existe une.",
      },
    ],
  },
};

/** The visible Q&A for a page (same data that feeds the FAQPage JSON-LD), so a
 *  page can render the questions on-screen. Google requires FAQ structured data
 *  to be visible on the page; rendering from this shared source keeps the markup
 *  and the visible text identical. Empty array if the page has no FAQ. */
export function pageFaq(pageKey: keyof typeof PAGES, lang: Lang): QA[] {
  return PAGE_FAQ[pageKey]?.[lang] ?? [];
}

/** A FAQPage JSON-LD node for a page's Q&A, or null if the page has none. */
function faqLd(pageKey: keyof typeof PAGES, lang: Lang) {
  const qa = PAGE_FAQ[pageKey]?.[lang];
  if (!qa || qa.length === 0) return null;
  return {
    "script:ld+json": {
      "@context": "https://schema.org",
      "@type": "FAQPage",
      mainEntity: qa.map((item) => ({
        "@type": "Question",
        name: item.q,
        acceptedAnswer: { "@type": "Answer", text: item.a },
      })),
    },
  };
}

/** A WebPage JSON-LD node carrying the page's declared last-update date, or
 *  null if the page has no declared date. Gives search and answer engines a
 *  machine-readable freshness signal to match the sitemap <lastmod>. */
function updatedLd(pathname: string, lang: Lang) {
  const updated = pageUpdated(pathname);
  if (!updated) return null;
  return {
    "script:ld+json": {
      "@context": "https://schema.org",
      "@type": "WebPage",
      url: canonicalUrl(pathname),
      dateModified: updated,
      inLanguage: lang,
    },
  };
}

/**
 * Build a React Router `meta` descriptor list for a page. `pageKey` matches a
 * PAGES entry; the language comes from the URL of the page being prerendered.
 * Pages registered in PAGE_FAQ also get a FAQPage structured-data node, and
 * every page carries a WebPage node with its declared dateModified.
 */
export function pageMeta(pageKey: keyof typeof PAGES) {
  return ({ location }: { location: { pathname: string } }) => {
    const lang: Lang = langFromPath(location.pathname);
    const group = PAGES[pageKey];
    const entry = (group ?? HOME_PAGE)[lang];
    const faq = faqLd(pageKey, lang);
    const updated = updatedLd(location.pathname, lang);
    return [
      { title: entry.title },
      { name: "description", content: entry.description },
      { property: "og:title", content: entry.title },
      { property: "og:description", content: entry.description },
      { property: "og:url", content: canonicalUrl(location.pathname) },
      ...(faq ? [faq] : []),
      ...(updated ? [updated] : []),
    ];
  };
}
