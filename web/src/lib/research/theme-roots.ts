// Shared presentation for the nine solving-path themes. Labels live in the topic
// registry (content/research/topics.json), the single source of truth, so they
// never drift; this file only adds the accent colour and one-line hook that the
// registry doesn't carry, used by the homepage tree, the navbar chips, and the
// left rail alike.
//
// The hooks are localized here rather than in topics.json because they are
// presentation copy, not content: the registry carries the label and the full
// description, this carries the one-liner the tree node shows on hover.

import type { Dict } from "@/i18n";

/** Themes that are meta (not a solving method) and stay off the "paths" views.
 *  Currently empty: every remaining theme is a solving path. Kept as the seam so
 *  a future meta theme can be added without touching the sidebar logic. */
export const NON_PATH_THEMES = new Set<string>([]);

export interface ThemeRootStyle {
  /** One-line hook, shown on the tree node's tooltip. Localized: this string
   *  renders on the homepage tree, the navbar chips and the left rail, so an
   *  English-only value put English text in front of every FR and ES reader.
   *  Resolve it with pick(style.hook, lang) at the call site. */
  hook: Dict<string>;
  /** Accent colour (hex), echoing the wiki's kind-dot palette. */
  color: string;
}

/** slug → hook + accent colour, for every solving-path theme. */
export const THEME_ROOTS: Record<string, ThemeRootStyle> = {
  structure: {
    hook: {
      en: "The design tuned to be unsolvable, and the walls that prove it.",
      fr: "Un puzzle conçu pour résister, et les murs qui le prouvent.",
      es: "Un diseño pensado para resistir, y los muros que lo demuestran.",
    },
    color: "#a78bfa", // violet
  },
  "search-space": {
    hook: {
      en: "Throw away hopeless states before the search wastes time on them.",
      fr: "Écarter les états sans espoir avant que la recherche s'y épuise.",
      es: "Descartar los estados sin salida antes de que la búsqueda se agote en ellos.",
    },
    color: "#38bdf8", // sky
  },
  backtracking: {
    hook: {
      en: "DFS done seriously: fill orders, restarts, break indices.",
      fr: "Le DFS pris au sérieux : ordres de remplissage, redémarrages, indices de rupture.",
      es: "DFS en serio: órdenes de relleno, reinicios, índices de ruptura.",
    },
    color: "#22d3ee", // cyan
  },
  speed: {
    hook: {
      en: "Bit tricks and cache, and why raw speed alone won't crack it.",
      fr: "Astuces de bits et cache, et pourquoi la vitesse brute ne suffira pas.",
      es: "Trucos de bits y caché, y por qué la velocidad bruta no bastará.",
    },
    color: "#fbbf24", // amber
  },
  construction: {
    hook: {
      en: "Beam search, priors, staged assembly: compose, don't dig.",
      fr: "Recherche en faisceau, a priori, assemblage par étapes : composer plutôt que creuser.",
      es: "Búsqueda en haz, priors, montaje por etapas: componer en vez de excavar.",
    },
    color: "#34d399", // emerald
  },
  "local-search": {
    hook: {
      en: "Improve a board you already have; the rigidity walls that stop you.",
      fr: "Améliorer un plateau déjà en main, et les murs de rigidité qui vous arrêtent.",
      es: "Mejorar un tablero que ya tienes, y los muros de rigidez que te frenan.",
    },
    color: "#4ade80", // green
  },
  "exact-methods": {
    hook: {
      en: "Solvers with proofs: SAT, MIP, exact-cover, meet-in-the-middle.",
      fr: "Des solveurs qui prouvent : SAT, PLNE, couverture exacte, meet-in-the-middle.",
      es: "Solucionadores con demostración: SAT, PLEM, cobertura exacta, meet-in-the-middle.",
    },
    color: "#60a5fa", // blue
  },
  learning: {
    hook: {
      en: "Belief propagation, learned heuristics, corpus priors.",
      fr: "Propagation de croyance, heuristiques apprises, a priori tirés du corpus.",
      es: "Propagación de creencias, heurísticas aprendidas, priors del corpus.",
    },
    color: "#f472b6", // pink
  },
  hardware: {
    hook: {
      en: "GPU, FPGA, distributed sweeps: silicon thrown at the wall.",
      fr: "GPU, FPGA, balayages distribués : du silicium jeté contre le mur.",
      es: "GPU, FPGA, barridos distribuidos: silicio lanzado contra el muro.",
    },
    color: "#fb923c", // orange
  },
};
