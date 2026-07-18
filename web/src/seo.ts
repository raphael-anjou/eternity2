// Per-page <title> and description, in both languages, for SEO and social
// previews. Keyed by route id (see routes.ts). Each page's `meta` export reads
// its entry via pageMeta(); root.tsx supplies sensible site-wide fallbacks.

import type { Dict, Lang } from "@/i18n";
import { langFromPath, pick } from "@/i18n";
import { pageUpdated } from "@/page-updated";
import { canonicalUrl } from "@/site";

type Entry = { title: string; description: string };

const SUFFIX = " · Eternity II";

// Default page metadata, used as the fallback for any unknown page key.
const HOME_PAGE: Dict<Entry> = {
  en: {
    title: "Eternity II: the puzzle that beat everyone",
    description:
      "Play the Eternity II puzzle online, free and in your browser: solve it by hand, watch real solvers run, learn the algorithms, explore the research. The $2,000,000 puzzle nobody ever solved.",
  },
  fr: {
    title: "Eternity II : le puzzle que personne n'a battu",
    description:
      "Jouez au puzzle Eternity II en ligne, gratuitement dans votre navigateur : résolvez-le à la main, regardez de vrais solveurs, découvrez les algorithmes et la recherche. Le casse-tête à 2 000 000 $ resté invaincu.",
  },
  es: {
    title: "Eternity II: el puzzle que venció a todos",
    description:
      "Juega al puzzle Eternity II en línea, gratis y en tu navegador: resuélvelo a mano, mira correr solucionadores reales, aprende los algoritmos y explora la investigación. El puzzle de 2 000 000 $ que nadie llegó a resolver.",
  },
};

const PAGES: Record<string, Dict<Entry>> = {
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
    es: {
      title: "Por dónde empezar" + SUFFIX,
      description:
        "¿Nuevo en Eternity II o no sabes por dónde mirar? Dinos quién eres, desde el simple curioso hasta el investigador, y sigue un breve recorrido guiado por el sitio.",
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
    es: {
      title: "¿Se ha resuelto Eternity II?" + SUFFIX,
      description:
        "No. El récord actual es de 470 aristas coincidentes de 480 (Joshua Blackwood, 2021), igualado pero nunca superado. La solución completa nunca se ha encontrado. Actualizado en julio de 2026.",
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
    es: {
      title: "¿Qué es el puzzle Eternity II?" + SUFFIX,
      description:
        "Qué es en realidad el puzzle Eternity II: 256 piezas de encaje de bordes, 22 colores, un tablero de 16×16 y un premio de 2 000 000 $ que quedó sin reclamar.",
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
    es: {
      title: "¿Es Eternity II una estafa? ¿Tiene solución?" + SUFFIX,
      description:
        "No, no es una estafa, y sí, casi con total seguridad existe una solución. Los hechos documentados sobre el puzzle de 2 000 000 $ que caducó sin ganador.",
    },
  },
  playground: {
    en: {
      title: "Play Eternity II online" + SUFFIX,
      description:
        "Play the Eternity II puzzle online, free and in your browser — no app, no download. Solve one by hand, watch a real solver run at a million steps a second, or design its search path.",
    },
    fr: {
      title: "Jouer à Eternity II en ligne" + SUFFIX,
      description:
        "Jouez au puzzle Eternity II en ligne, gratuitement dans votre navigateur — sans appli ni téléchargement. Résolvez-en un à la main, regardez un vrai solveur à un million d'étapes par seconde, ou dessinez son parcours.",
    },
    es: {
      title: "Jugar a Eternity II en línea" + SUFFIX,
      description:
        "Juega al puzzle Eternity II en línea, gratis y en tu navegador, sin app ni descargas. Resuelve uno a mano, mira correr un solucionador real a un millón de pasos por segundo o diseña su recorrido de búsqueda.",
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
    es: {
      title: "Ver un solucionador" + SUFFIX,
      description: "Observa en directo cómo un solucionador en profundidad busca Eternity II, paso a paso.",
    },
  },
  solve: {
    en: {
      title: "Solve the Eternity II puzzle online" + SUFFIX,
      description:
        "Play Eternity II online: place the edge-matching tiles yourself, beat the clock, and feel why the puzzle is so hard. Free, in your browser, no download.",
    },
    fr: {
      title: "Résoudre le puzzle Eternity II en ligne" + SUFFIX,
      description:
        "Jouez à Eternity II en ligne : placez vous-même les tuiles à côtés appariés, battez le chrono et comprenez pourquoi ce puzzle est si redoutable. Gratuit, dans votre navigateur, sans téléchargement.",
    },
    es: {
      title: "Resolver el puzzle Eternity II en línea" + SUFFIX,
      description:
        "Juega a Eternity II en línea: coloca tú mismo las piezas de encaje de bordes, gana al reloj y comprende por qué el puzzle es tan difícil. Gratis, en tu navegador, sin descargas.",
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
    es: {
      title: "Órdenes de relleno" + SUFFIX,
      description: "Diseña el orden en que un solucionador rellena el tablero y compítelo contra los órdenes clásicos.",
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
    es: {
      title: "Imprimir y jugar" + SUFFIX,
      description: "Genera láminas de piezas de Eternity II listas para imprimir, recortar y resolver sobre papel.",
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
    es: {
      title: "Cómo resolver Eternity II con un ordenador" + SUFFIX,
      description:
        "Cómo atacan las máquinas Eternity II: búsqueda en profundidad, backtracking, el muro exponencial y el pico de dificultad de color y tamaño, con mediciones en directo.",
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
    es: {
      title: "Tableros destacados" + SUFFIX,
      description:
        "Una galería de tableros de Eternity II destacados: récords de la comunidad y los mejores tableros de este proyecto, cada uno abriéndose en el visor para comprobarlo arista por arista.",
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
    es: {
      title: "Invenciones" + SUFFIX,
      description:
        "Los algoritmos con nombre creados para elevar la puntuación de Eternity II: PALIMPSEST (463), KEYRING (460), GAUNTLET (458) y más. Cada idea, su resultado y las preguntas abiertas, con tableros verificados.",
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
    es: {
      title: "Visor de tableros" + SUFFIX,
      description: "Pega un tablero e inspecciónalo: puntuación, conflictos y las soluciones récord conocidas de Eternity II.",
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
    es: {
      title: "Conversor de formatos" + SUFFIX,
      description:
        "Convierte entre los formatos de tablero de Eternity II: enlaces bucas, cadenas board_edges y números board_pieces. Pega uno y obtén los demás, con vista previa en vivo.",
    },
  },
};

/**
 * Bare page title (suffix stripped) for a registered page key — used by the
 * research sidebar to label not-yet-migrated TSX pages without duplicating
 * their titles. Goes away when the research migration to MDX completes.
 */
export function pageTitle(pageKey: string, lang: Lang): string {
  const entry = pick(PAGES[pageKey] ?? HOME_PAGE, lang);
  return entry.title.replace(SUFFIX, "");
}

/** Page description for a registered page key (research search + nav). */
export function pageDescription(pageKey: string, lang: Lang): string {
  return pick(PAGES[pageKey] ?? HOME_PAGE, lang).description;
}

// FAQ structured data for the two question-titled pages. Both answer a real
// query people type ("has Eternity II been solved?", "is it a scam?"), so a
// FAQPage node makes them eligible for FAQ rich results and gives AI answers a
// clean, sourced Q&A to lift. Answers mirror the sourced facts on the pages
// themselves; keep them in sync when a record changes.
type QA = { q: string; a: string };
const PAGE_FAQ: Partial<Record<keyof typeof PAGES, Dict<QA[]>>> = {
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
    es: [
      {
        q: "¿Qué es el puzzle Eternity II?",
        a: "Eternity II es un puzzle de encaje de bordes lanzado en 2007: 256 piezas cuadradas deben llenar un tablero de 16×16 de modo que cada arista compartida muestre el mismo patrón de color, con los bordes exteriores en gris. Ofrecía un premio de 2 000 000 $ por la primera solución completa.",
      },
      {
        q: "¿Cuántas piezas tiene Eternity II?",
        a: "256 piezas cuadradas, cada una con medio motivo de color en sus cuatro aristas, tomados de 22 colores/motivos distintos. Llenan una cuadrícula de 16×16: 480 aristas internas que deben coincidir todas.",
      },
      {
        q: "¿Por qué es tan difícil Eternity II?",
        a: "El número de formas de disponer las piezas es de unos 1,115 × 10^557, muchísimo más que los átomos del universo observable. Ningún algoritmo puede comprobarlas todas, y el puzzle se diseñó a propósito para resistir la búsqueda por fuerza bruta.",
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
    es: [
      {
        q: "¿Se ha resuelto Eternity II?",
        a: "No. A fecha de julio de 2026, el puzzle nunca se ha resuelto por completo. El mejor tablero público hace coincidir 470 de 480 aristas (Joshua Blackwood, 2021), igualado pero nunca superado, y nunca se ha encontrado una solución perfecta de 480 aristas.",
      },
      {
        q: "¿Cuál es el récord de puntuación de Eternity II?",
        a: "470 de 480 aristas coincidentes, logrado por Joshua Blackwood en 2021. Tableros destacados anteriores alcanzaron 467 (Louis Verhaard) y 469 (Blackwood y McGavin).",
      },
      {
        q: "¿Sigue disponible el premio de 2 000 000 $?",
        a: "No. El plazo del premio venció el 31 de diciembre de 2010 sin que se presentara una solución completa, así que los 2 000 000 $ quedaron sin reclamar y el concurso está cerrado.",
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
    es: [
      {
        q: "¿Es Eternity II una estafa?",
        a: "No. Fue un concurso auténtico con un premio real de 2 000 000 $ depositado en garantía. El premio simplemente caducó sin reclamar en 2010 porque nadie encontró a tiempo una solución completa: el puzzle es extraordinariamente difícil, no está amañado.",
      },
      {
        q: "¿Tiene solución Eternity II?",
        a: "Casi con total seguridad, sí. Las estimaciones estadísticas del conjunto de piezas implican que existen muchas soluciones perfectas; la dificultad está en encontrar una en un espacio de búsqueda de unos 1,1 × 10^557 disposiciones, no en si existe.",
      },
    ],
  },
};

/** The visible Q&A for a page (same data that feeds the FAQPage JSON-LD), so a
 *  page can render the questions on-screen. Google requires FAQ structured data
 *  to be visible on the page; rendering from this shared source keeps the markup
 *  and the visible text identical. Empty array if the page has no FAQ. */
export function pageFaq(pageKey: keyof typeof PAGES, lang: Lang): QA[] {
  const faq = PAGE_FAQ[pageKey];
  return faq ? pick(faq, lang) : [];
}

/** A FAQPage JSON-LD node for a page's Q&A, or null if the page has none. */
function faqLd(pageKey: keyof typeof PAGES, lang: Lang) {
  const faq = PAGE_FAQ[pageKey];
  const qa = faq ? pick(faq, lang) : undefined;
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

// Pages that are genuine in-browser applications, not articles. A
// WebApplication node lets them qualify as software/app results and advertise
// that they run free in the browser — the intent behind "eternity 2 puzzle
// online" and the tool queries. Values are schema.org applicationCategory terms.
const PAGE_APP: Partial<Record<keyof typeof PAGES, string>> = {
  playground: "GameApplication",
  solve: "GameApplication",
  watch: "GameApplication",
  paths: "GameApplication",
  viewer: "UtilitiesApplication",
  convert: "UtilitiesApplication",
};

/** A WebApplication JSON-LD node for a page that is a real in-browser tool/game,
 *  or null. Declares it free (offers price 0) and browser-only, so it can
 *  surface as an app result and answer "play Eternity II online" directly. */
function appLd(pageKey: keyof typeof PAGES, lang: Lang, pathname: string) {
  const category = PAGE_APP[pageKey];
  if (!category) return null;
  const entry = pick(PAGES[pageKey] ?? HOME_PAGE, lang);
  return {
    "script:ld+json": {
      "@context": "https://schema.org",
      "@type": "WebApplication",
      name: entry.title.replace(SUFFIX, ""),
      description: entry.description,
      url: canonicalUrl(pathname),
      applicationCategory: category,
      operatingSystem: "Any (modern web browser)",
      browserRequirements: "Requires JavaScript and WebAssembly",
      inLanguage: lang,
      isPartOf: { "@id": "https://eternity2.dev/#website" },
      publisher: { "@id": "https://eternity2.dev/#org" },
      // Free to play: an explicit zero-price Offer is the schema.org idiom.
      offers: { "@type": "Offer", price: "0", priceCurrency: "USD" },
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
 * Pages registered in PAGE_FAQ also get a FAQPage node, pages in PAGE_APP a
 * WebApplication node, and every page a WebPage node with its dateModified.
 */
export function pageMeta(pageKey: keyof typeof PAGES) {
  return ({ location }: { location: { pathname: string } }) => {
    const lang: Lang = langFromPath(location.pathname);
    const group = PAGES[pageKey];
    const entry = pick(group ?? HOME_PAGE, lang);
    const faq = faqLd(pageKey, lang);
    const app = appLd(pageKey, lang, location.pathname);
    const updated = updatedLd(location.pathname, lang);
    return [
      { title: entry.title },
      { name: "description", content: entry.description },
      { property: "og:title", content: entry.title },
      { property: "og:description", content: entry.description },
      { property: "og:url", content: canonicalUrl(location.pathname) },
      ...(faq ? [faq] : []),
      ...(app ? [app] : []),
      ...(updated ? [updated] : []),
    ];
  };
}
