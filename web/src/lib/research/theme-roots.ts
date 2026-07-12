// Shared presentation for the nine solving-path themes (the topic registry
// minus the meta "records" theme). Labels live in the topic registry
// (content/research/topics.json) — the single source of truth — so they never
// drift; this file only adds the accent colour and one-line hook that the
// registry doesn't carry, used by the homepage tree, the navbar chips, and the
// left rail alike.

/** Themes that are meta (not a solving method) and stay off the "paths" views. */
export const NON_PATH_THEMES = new Set(["records"]);

export interface ThemeRootStyle {
  /** One-line hook, shown on the tree node's tooltip. */
  hook: string;
  /** Accent colour (hex), echoing the wiki's kind-dot palette. */
  color: string;
}

/** slug → hook + accent colour, for every solving-path theme. */
export const THEME_ROOTS: Record<string, ThemeRootStyle> = {
  structure: {
    hook: "The design tuned to be unsolvable, and the walls that prove it.",
    color: "#a78bfa", // violet
  },
  "search-space": {
    hook: "Throw away hopeless states before the search wastes time on them.",
    color: "#38bdf8", // sky
  },
  backtracking: {
    hook: "DFS done seriously: fill orders, restarts, break indices.",
    color: "#22d3ee", // cyan
  },
  speed: {
    hook: "Bit tricks and cache, and why raw speed alone won't crack it.",
    color: "#fbbf24", // amber
  },
  construction: {
    hook: "Beam search, priors, staged assembly: compose, don't dig.",
    color: "#34d399", // emerald
  },
  "local-search": {
    hook: "Improve a board you already have; the rigidity walls that stop you.",
    color: "#4ade80", // green
  },
  "exact-methods": {
    hook: "Solvers with proofs: SAT, MIP, exact-cover, meet-in-the-middle.",
    color: "#60a5fa", // blue
  },
  learning: {
    hook: "Belief propagation, learned heuristics, corpus priors.",
    color: "#f472b6", // pink
  },
  hardware: {
    hook: "GPU, FPGA, distributed sweeps: silicon thrown at the wall.",
    color: "#fb923c", // orange
  },
};
