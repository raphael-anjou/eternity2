// The translated labels for the `outcome` frontmatter vocabulary (how an
// experiment or finding ended), plus the one-line tooltip for each value.
// Single source of truth: the docs shell's OutcomeBadge and the experiment
// results table both read these maps, so the wording cannot drift between the
// page header and the hub tables.

import type { Lang } from "@/i18n";
import type { OutcomeKind } from "./types";

export const OUTCOME_LABELS: Record<Lang, Record<OutcomeKind, string>> = {
  en: {
    plateaued: "plateaued",
    refuted: "refuted",
    parked: "parked",
    "new-basin": "new basin",
    superseded: "superseded",
  },
  fr: {
    plateaued: "plafonné",
    refuted: "réfuté",
    parked: "mis de côté",
    "new-basin": "nouveau bassin",
    superseded: "dépassé",
  },
  es: {
    plateaued: "estancado",
    refuted: "refutado",
    parked: "aparcado",
    "new-basin": "nueva cuenca",
    superseded: "superado",
  },
};

export const OUTCOME_TITLES: Record<Lang, Record<OutcomeKind, string>> = {
  en: {
    plateaued: "reached a measured ceiling",
    refuted: "a rigorously-run dead end",
    parked: "set aside, not exhausted",
    "new-basin": "opened a new family or region",
    superseded: "beaten by a later result",
  },
  fr: {
    plateaued: "a atteint un plafond mesuré",
    refuted: "une impasse étudiée avec rigueur",
    parked: "mis de côté, pas épuisé",
    "new-basin": "a ouvert une nouvelle famille ou région",
    superseded: "battu par un résultat ultérieur",
  },
  es: {
    plateaued: "alcanzó un techo medido",
    refuted: "un callejón sin salida estudiado con rigor",
    parked: "apartado, no agotado",
    "new-basin": "abrió una nueva familia o región",
    superseded: "batido por un resultado posterior",
  },
};
