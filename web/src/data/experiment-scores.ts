import type { ScoreDatum } from "@/components/research/ExperimentScoreChart";

// The best score each of Raphaël Anjou's named experiments reached, the single
// source of truth for his hub's score chart and results table. It lived inline
// in three copies (index.mdx and its .fr/.es siblings); pulling it here keeps
// the three renderings in step and lets the aggregate hub reuse the same
// families. Author, month, rigor and reproducibility are read from each page's
// frontmatter via the manifest, so only the score, label and family live here.
//
// Two provenance groups that must never be conflated:
//   explore — the best board a method reached in exploratory runs on 8 cores,
//             wall-clock not logged; the headline numbers the write-ups quote.
//   bench   — the standardized single-core bench, one core, 60 s each; lower,
//             and directly comparable across methods.
export const RAPHAEL_ANJOU_SCORES: ScoreDatum[] = [
  // Group 1: the best board each method reached (explore).
  { key: "prior", label: "PRIOR", score: 460, family: "scratch", group: "explore" },
  { key: "keyring", label: "KEYRING", score: 460, family: "scratch", group: "explore" },
  { key: "gauntlet", label: "GAUNTLET", score: 458, family: "scratch", group: "explore" },
  { key: "lodestone", label: "LODESTONE", score: 451, family: "scratch", group: "explore" },
  { key: "staged", label: "STAGED", score: 436, family: "scratch", group: "explore" },
  { key: "palimpsest", label: "PALIMPSEST", score: 463, family: "corpus", group: "explore" },
  { key: "ladder", label: "LADDER", score: 451, family: "concentrate", group: "explore" },
  { key: "cloister", label: "CLOISTER", score: 453, family: "anchor", group: "explore" },
  { key: "midden", label: "MIDDEN", score: 452, family: "anchor", group: "explore" },
  { key: "bandsaw", label: "BANDSAW", score: 437, family: "exact", group: "explore" },
  { key: "mosaic", label: "MOSAIC", score: 448, family: "exact", group: "explore" },
  { key: "replay", label: "REPLAY", score: 460, family: "decode", group: "explore" },
  // Group 2: the standardized single-core bench, one core, 60 s each (bench).
  { key: "bench-verhaard", label: "VERHAARD", score: 451, family: "bench", group: "bench" },
  { key: "bench-repair", label: "REPAIR START-DFS", score: 449, family: "bench", group: "bench" },
  { key: "bench-dfs", label: "DFS BREAK-1", score: 435, family: "bench", group: "bench" },
];
