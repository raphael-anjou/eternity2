// The contribution axis: what KIND of result a research page is (a solver that
// scores, an analysis, a dead-end negative, ...), independent of where the page
// lives in the wiki tree. The by-contribution hubs (ContributionPages.tsx) and
// the doc route (pages/research/doc.tsx) share this data and routing logic; the
// components only render it. Keeping the data + routing here (rather than in the
// component file) lets those hubs stay Fast-Refresh-clean.

import type { Lang } from "@/i18n";
import { researchDocs } from "./manifest";
import type { ContributionKind, ResearchDoc } from "./types";

/** Base URL for the by-contribution hubs. */
export const BY_CONTRIBUTION_BASE = "/research/lab/experiments/by-contribution";

// The contribution axis, in the display order the methodology page uses (solver
// first, then everything else, with the negative shelf near the end).
export const CONTRIBUTION_ORDER: readonly ContributionKind[] = [
  "solver",
  "analysis",
  "reconstruction",
  "theory",
  "method",
  "measurement",
  "tool",
  "exposition",
  "negative",
];

const CONTRIBUTION_LABEL: Record<ContributionKind, Record<Lang, string>> = {
  solver: { en: "Solvers", fr: "Solveurs", es: "Solucionadores" },
  analysis: { en: "Analyses", fr: "Analyses", es: "Análisis" },
  reconstruction: { en: "Reconstructions", fr: "Reconstructions", es: "Reconstrucciones" },
  theory: { en: "Theory", fr: "Théorie", es: "Teoría" },
  method: { en: "Methods", fr: "Méthodes", es: "Métodos" },
  measurement: { en: "Measurements", fr: "Mesures", es: "Mediciones" },
  negative: { en: "What was ruled out", fr: "Ce qui a été écarté", es: "Lo que se descartó" },
  tool: { en: "Tools", fr: "Outils", es: "Herramientas" },
  exposition: { en: "Explainers", fr: "Exposés", es: "Explicaciones" },
};

// One line per kind: what this shelf collects. Reader-facing; translated.
const CONTRIBUTION_BLURB: Record<ContributionKind, Record<Lang, string>> = {
  solver: {
    en: "Runs that produce a competitive board by searching. Only these earn a row on the leaderboard.",
    fr: "Des runs qui produisent un plateau compétitif par recherche. Seuls ceux-là gagnent une ligne au classement.",
    es: "Runs que producen un tablero competitivo mediante búsqueda. Solo estos ganan una fila en la clasificación.",
  },
  analysis: {
    en: "Pages that prove or compute a property of an existing board or of the instance, without producing a new board.",
    fr: "Des pages qui démontrent ou calculent une propriété d'un plateau existant ou de l'instance, sans produire de nouveau plateau.",
    es: "Páginas que demuestran o calculan una propiedad de un tablero existente o de la instancia, sin producir un tablero nuevo.",
  },
  reconstruction: {
    en: "Decodes and rebuilds of the community's known work, to extract the insight behind a number that is theirs.",
    fr: "Des décodages et reconstitutions du travail connu de la communauté, pour en extraire l'enseignement derrière un nombre qui est le leur.",
    es: "Decodificaciones y reconstrucciones del trabajo conocido de la comunidad, para extraer la idea tras un número que es suyo.",
  },
  theory: {
    en: "Mathematical properties, laws, and impossibility proofs about the puzzle.",
    fr: "Des propriétés mathématiques, des lois et des preuves d'impossibilité sur le casse-tête.",
    es: "Propiedades matemáticas, leyes y pruebas de imposibilidad sobre el rompecabezas.",
  },
  method: {
    en: "Techniques described for reuse, rather than a single scored run.",
    fr: "Des techniques décrites pour être réutilisées, plutôt qu'un unique run scoré.",
    es: "Técnicas descritas para su reutilización, en lugar de una única ejecución con puntuación.",
  },
  measurement: {
    en: "Benchmarks and empirical observations about solvers or instances.",
    fr: "Des benchmarks et des observations empiriques sur les solveurs ou les instances.",
    es: "Benchmarks y observaciones empíricas sobre solucionadores o instancias.",
  },
  negative: {
    en: "Sound dead ends, kept as first-class results: an idea run with rigour that did not pay off, and what its ceiling turned out to be.",
    fr: "Des impasses solides, gardées comme résultats à part entière : une idée menée avec rigueur qui n'a pas payé, et ce qu'a été son plafond.",
    es: "Callejones sin salida sólidos, guardados como resultados de pleno derecho: una idea llevada con rigor que no dio fruto, y cuál resultó ser su techo.",
  },
  tool: {
    en: "Software artifacts: engines, viewers, and checkers.",
    fr: "Des artefacts logiciels : moteurs, visionneuses et vérificateurs.",
    es: "Artefactos de software: motores, visores y verificadores.",
  },
  exposition: {
    en: "Explainers written for a reader, not a scored run.",
    fr: "Des exposés écrits pour un lecteur, non un run scoré.",
    es: "Explicaciones escritas para un lector, no un run con puntuación.",
  },
};

/** Localized shelf label for a contribution kind. */
export function contributionLabel(kind: ContributionKind, lang: Lang): string {
  return CONTRIBUTION_LABEL[kind][lang] ?? CONTRIBUTION_LABEL[kind].en;
}

/** Localized one-line description of what a contribution shelf collects. */
export function contributionBlurb(kind: ContributionKind, lang: Lang): string {
  return CONTRIBUTION_BLURB[kind][lang] ?? CONTRIBUTION_BLURB[kind].en;
}

/** Title + description for a contribution hub, for the route's <meta> tags. */
export function contributionMeta(
  kind: ContributionKind,
  lang: Lang,
): { title: string; description: string } {
  return { title: contributionLabel(kind, lang), description: contributionBlurb(kind, lang) };
}

/** Pages that declare a given contribution, in reading order. */
export function contributionMembers(lang: Lang, kind: ContributionKind): ResearchDoc[] {
  return researchDocs(lang).filter((d) => d.contribution === kind);
}

/** by-contribution index URL, or a hub's ContributionKind, or null if not one
 *  of these routes. Returns "" for the index. */
export function contributionRouteFor(path: string): "" | ContributionKind | null {
  if (path === BY_CONTRIBUTION_BASE) return "";
  const m = new RegExp(`^${BY_CONTRIBUTION_BASE}/([a-z-]+)$`).exec(path);
  const slug = m?.[1];
  if (!slug) return null;
  return (CONTRIBUTION_ORDER as readonly string[]).includes(slug)
    ? (slug as ContributionKind)
    : null;
}
