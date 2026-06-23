// One source of truth for cross-links across the research section. Every
// research page is a node here; `related` lists the other pages it should point
// at, so a reader who lands on any page can always step sideways into the
// finding that explains it, the invention that probes it, or the experiment log
// behind it. The <RelatedRail> component reads this map by the page's own path.
//
// Keeping the graph in one file (rather than hand-wiring links per page) means
// the cross-linking stays consistent and is trivial to audit: every edge is
// visible here, and a missing back-link is a one-line fix.

import type { Lang } from "@/i18n";

export type LinkKind = "why" | "invention" | "finding" | "tool" | "reference";

export interface ResearchNode {
  /** Path without language prefix, e.g. "/research/why/rigidity-wall". */
  path: string;
  kind: LinkKind;
  title: { en: string; fr: string };
  /** One-line "why you'd click this from a neighbour" blurb. */
  blurb: { en: string; fr: string };
  /** Paths of related nodes, most relevant first. */
  related: string[];
}

const N = (
  path: string,
  kind: LinkKind,
  en: string,
  fr: string,
  blurbEn: string,
  blurbFr: string,
  related: string[],
): ResearchNode => ({
  path,
  kind,
  title: { en, fr },
  blurb: { en: blurbEn, fr: blurbFr },
  related,
});

export const RESEARCH_NODES: ResearchNode[] = [
  // ---- Why it's hard (the findings) ----
  N(
    "/research/why/walls-and-methods",
    "why",
    "Which wall stops which method",
    "Quel mur arrête quelle méthode",
    "The bridge: every invented algorithm lined up against the wall it attacks and the score where it saturated.",
    "Le pont : chaque algorithme inventé aligné face au mur qu'il attaque et au score où il a saturé.",
    [
      "/research/why/prune-vs-speed",
      "/research/why/rigidity-wall",
      "/research/lab/inventions",
      "/research/why/no-forced-moves",
    ],
  ),
  N(
    "/research/why/prune-vs-speed",
    "why",
    "Why a faster computer doesn't help",
    "Pourquoi un ordinateur plus rapide n'aide pas",
    "Shrinking the search space beats searching it faster, exponentially — and E2 resists shrinking.",
    "Réduire l'espace de recherche bat l'explorer plus vite, exponentiellement — et E2 résiste à la réduction.",
    [
      "/research/why/walls-and-methods",
      "/research/why/complex-theory",
      "/research/why/no-forced-moves",
      "/research/why/phase-transition",
      "/research/why/rigidity-wall",
    ],
  ),
  N(
    "/research/why/complex-theory",
    "why",
    "Complex theory: counting the search",
    "La théorie complexe : compter la recherche",
    "Estimate the tree's width at every depth: ~14,702 solutions with one clue, the funnel where 99% of the time goes.",
    "Estime la largeur de l'arbre à chaque profondeur : ~14 702 solutions avec un indice, l'entonnoir où passe 99 % du temps.",
    [
      "/research/why/prune-vs-speed",
      "/research/why/phase-transition",
      "/research/why/entropy-area-law",
    ],
  ),
  N(
    "/research/why/phase-transition",
    "why",
    "The phase-transition argument",
    "L'argument de la transition de phase",
    "Why the puzzle sits exactly on the SAT/CSP hardness peak — about one expected solution.",
    "Pourquoi le puzzle est pile sur le pic de difficulté SAT/CSP — environ une solution attendue.",
    ["/research/why/prune-vs-speed", "/research/why/entropy-area-law", "/research/why/no-forced-moves"],
  ),
  N(
    "/research/why/rigidity-wall",
    "why",
    "The rigidity wall",
    "Le mur de rigidité",
    "Every record board is frozen: no small rearrangement improves it.",
    "Tout plateau record est figé : aucun petit réarrangement ne l'améliore.",
    [
      "/research/why/sigma-cycles",
      "/research/why/walls-and-methods",
      "/research/why/prune-vs-speed",
      "/research/lab/inventions/palimpsest",
      "/research/lab/inventions/midden",
    ],
  ),
  N(
    "/research/why/sigma-cycles",
    "why",
    "Why basin-hopping is impossible",
    "Pourquoi sauter de bassin en bassin est impossible",
    "Two great boards differ by one giant indivisible swap; every smaller piece scores worse.",
    "Deux bons plateaux diffèrent par un seul échange géant indivisible ; tout morceau plus petit fait moins bien.",
    ["/research/why/rigidity-wall", "/research/lab/inventions/keyring"],
  ),
  N(
    "/research/why/entropy-area-law",
    "why",
    "Entropy and the area law",
    "Entropie et loi d'aire",
    "Why the count of distinct partial boards collapses past ~80 cells — a wall no local signal sees.",
    "Pourquoi le nombre de plateaux partiels distincts s'effondre au-delà de ~80 cellules — un mur qu'aucun signal local ne voit.",
    ["/research/why/phase-transition", "/research/why/prune-vs-speed", "/research/why/forbidden-patterns"],
  ),
  N(
    "/research/why/forbidden-patterns",
    "why",
    "Forbidden patterns",
    "Motifs interdits",
    "Almost every small colour patch is illegal under the piece set — a sharp fingerprint of validity.",
    "Presque tout petit carré de couleurs est illégal — une signature nette de la validité.",
    ["/research/why/entropy-area-law", "/research/lab/inventions/keyring"],
  ),
  N(
    "/research/why/no-forced-moves",
    "why",
    "No forced moves",
    "Aucun coup forcé",
    "No piece is ever pinned: 73–137 neighbours each. Local freedom, no global consistency.",
    "Aucune pièce n'est coincée : 73 à 137 voisines chacune. Liberté locale, aucune cohérence globale.",
    ["/research/why/prune-vs-speed", "/research/why/piece-theft", "/research/why/phase-transition"],
  ),
  N(
    "/research/why/piece-theft",
    "why",
    "Piece theft, where solvers die",
    "Le vol de pièce, là où les solveurs meurent",
    "Spend a cell's only supplier elsewhere and a future cell dies while the box still looks full.",
    "Dépensez l'unique fournisseur d'une cellule ailleurs et une cellule future meurt alors que la boîte semble pleine.",
    ["/research/why/no-forced-moves", "/research/lab/inventions/prior"],
  ),
  N(
    "/research/why/rare-color-geography",
    "why",
    "The rare colours live on the frame",
    "Les couleurs rares vivent sur le cadre",
    "Five of 22 colours appear only on the border ring — design you can see.",
    "Cinq des 22 couleurs n'apparaissent que sur le cadre — une conception visible.",
    ["/research/why/forbidden-patterns", "/research/lab/inventions/cloister"],
  ),

  // ---- Inventions ----
  N(
    "/research/lab/inventions/palimpsest",
    "invention",
    "PALIMPSEST",
    "PALIMPSEST",
    "Reads the corpus to find the choices that quietly lock a board below the top, then targets them.",
    "Lit le corpus pour repérer les choix qui bloquent un plateau sous le sommet, puis les cible.",
    ["/research/why/rigidity-wall", "/research/lab/inventions/keyring"],
  ),
  N(
    "/research/lab/inventions/keyring",
    "invention",
    "KEYRING",
    "KEYRING",
    "From-scratch build ranking each next piece by three learned signals. Found a new board family.",
    "Construction de zéro classant chaque pièce par trois signaux appris. A trouvé une famille inédite.",
    [
      "/research/lab/inventions/prior",
      "/research/why/sigma-cycles",
      "/research/why/forbidden-patterns",
    ],
  ),
  N(
    "/research/lab/inventions/gauntlet",
    "invention",
    "GAUNTLET",
    "GAUNTLET",
    "Beam search in several fill directions at once, to land in different regions.",
    "Recherche en faisceau dans plusieurs directions à la fois, pour atterrir dans des régions différentes.",
    ["/research/lab/inventions/prior"],
  ),
  N(
    "/research/lab/inventions/prior",
    "invention",
    "PRIOR",
    "PRIOR",
    "From-nothing beam search, breaking ties by where pieces tend to sit in strong boards.",
    "Recherche en faisceau partie de rien, départageant par l'endroit où les pièces se trouvent.",
    [
      "/research/lab/inventions/keyring",
      "/research/lab/inventions/gauntlet",
      "/research/why/piece-theft",
    ],
  ),
  N(
    "/research/lab/inventions/staged",
    "invention",
    "STAGED",
    "STAGED",
    "Builds the whole board in stages with no pre-set frame; the border emerges last.",
    "Construit le plateau par étapes sans cadre préétabli ; la bordure émerge en dernier.",
    ["/research/lab/inventions/cloister", "/research/lab/inventions/bandsaw"],
  ),
  N(
    "/research/lab/inventions/bandsaw",
    "invention",
    "BANDSAW",
    "BANDSAW",
    "Solves a band of rows exactly by meeting in the middle, to measure how far ahead an endgame decides.",
    "Résout une bande de rangées exactement en se rejoignant au milieu, pour mesurer jusqu'où une fin se décide.",
    ["/research/lab/inventions/staged", "/research/why/entropy-area-law"],
  ),
  N(
    "/research/lab/inventions/ladder",
    "invention",
    "LADDER",
    "LADDER",
    "Floods the board with cheap short searches, promoting only the deepest starts. Reached a 451 strict board.",
    "Inonde le plateau de recherches courtes, ne promouvant que les départs les plus profonds. A atteint un 451 strict.",
    ["/research/lab/inventions/replay", "/research/lab/inventions/prior"],
  ),
  N(
    "/research/lab/inventions/replay",
    "invention",
    "REPLAY",
    "REPLAY",
    "Rebuilds the community's strict 460 boards exactly, revealing the double-break move solvers miss.",
    "Reconstruit exactement les 460 stricts de la communauté, révélant le double-défaut que les solveurs manquent.",
    ["/research/lab/inventions/ladder", "/research/records", "/research/why/rigidity-wall"],
  ),
  N(
    "/research/lab/inventions/cloister",
    "invention",
    "CLOISTER",
    "CLOISTER",
    "Fixes a perfect border, then searches the interior against it. Isolates border–interior coupling.",
    "Fixe une bordure parfaite, puis cherche l'intérieur contre elle. Isole le couplage bordure–intérieur.",
    [
      "/research/lab/inventions/staged",
      "/research/why/rare-color-geography",
      "/research/lab/inventions/midden",
    ],
  ),
  N(
    "/research/lab/inventions/midden",
    "invention",
    "MIDDEN",
    "MIDDEN",
    "Decides where, not when, a board may break: confine mismatches to a chosen shape and search it.",
    "Décide où, et non quand, un plateau peut casser : confiner les défauts à une forme choisie.",
    [
      "/research/lab/inventions/ladder",
      "/research/why/rigidity-wall",
      "/research/lab/inventions/cloister",
    ],
  ),

  // ---- Lab hubs & tools ----
  N(
    "/research/lab/inventions",
    "invention",
    "Inventions",
    "Inventions",
    "The ten named algorithms built to push the score, grouped by the kind of idea each one is.",
    "Les dix algorithmes nommés conçus pour pousser le score, regroupés par type d'idée.",
    ["/research/why/walls-and-methods", "/research/why/rigidity-wall", "/research/records"],
  ),
  N(
    "/research/records",
    "reference",
    "Record boards",
    "Plateaux records",
    "The verified best boards, every score recomputed edge by edge in the viewer.",
    "Les meilleurs plateaux vérifiés, chaque score recalculé bord par bord.",
    ["/research/lab/inventions", "/research/why/rigidity-wall"],
  ),
  N(
    "/research/build/run-it-yourself",
    "tool",
    "Run it yourself",
    "À vous de jouer",
    "Clone the repo and reproduce every number on this site.",
    "Clonez le dépôt et reproduisez chaque chiffre du site.",
    ["/research/build/dead-ends"],
  ),
  N(
    "/research/build/dead-ends",
    "reference",
    "Dead ends",
    "Impasses",
    "Whole approaches that were ruled out, so you can skip them.",
    "Des approches entières écartées, pour vous les épargner.",
    ["/research/build/run-it-yourself"],
  ),
];

const BY_PATH = new Map(RESEARCH_NODES.map((n) => [n.path, n]));

export function relatedFor(path: string): ResearchNode[] {
  const node = BY_PATH.get(path);
  if (!node) return [];
  return node.related.map((p) => BY_PATH.get(p)).filter((n): n is ResearchNode => Boolean(n));
}

export function nodeFor(path: string): ResearchNode | undefined {
  return BY_PATH.get(path);
}

export const KIND_LABEL: Record<LinkKind, { en: string; fr: string }> = {
  why: { en: "Why it's hard", fr: "Pourquoi c'est dur" },
  invention: { en: "Invention", fr: "Invention" },
  finding: { en: "Finding", fr: "Résultat" },
  tool: { en: "Tool", fr: "Outil" },
  reference: { en: "Reference", fr: "Référence" },
};

export function kindLabel(kind: LinkKind, lang: Lang): string {
  return KIND_LABEL[kind][lang];
}
