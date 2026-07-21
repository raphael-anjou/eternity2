import { useState } from "react";

import { useT, useLang } from "@/i18n";
import {
  clueShape,
  clusteredClues,
  contiguous,
  hintgeoScattered18,
  latticePerLine,
} from "@/lib/hint-layouts";

import { HintBoardTile } from "./HintBoard";

// A gallery of the hint geometries the study measures, all rendered from the one
// shared HintBoard primitive so the visual language is identical. A toggle shows
// the "free-score floor" (banked seams) so the reader can see, at a glance, which
// layouts hand the solver correct edges for nothing — the confound the count
// axis has to correct for.

const N = 16;

type Item = { key: string; title: Record<"en" | "fr" | "es", string>; cells: number[] };

const ITEMS: Item[] = [
  { key: "clue", title: { en: "The five-clue shape", fr: "La forme à cinq indices", es: "La forma de cinco pistas" }, cells: clueShape(N) },
  { key: "spread9", title: { en: "Spread lattice (3/line)", fr: "Réseau dispersé (3/ligne)", es: "Retícula dispersa (3/línea)" }, cells: latticePerLine(N, 3) },
  { key: "spread16", title: { en: "Spread lattice (4/line)", fr: "Réseau dispersé (4/ligne)", es: "Retícula dispersa (4/línea)" }, cells: latticePerLine(N, 4) },
  { key: "spread36", title: { en: "Spread lattice (6/line)", fr: "Réseau dispersé (6/ligne)", es: "Retícula dispersa (6/línea)" }, cells: latticePerLine(N, 6) },
  { key: "scattered18", title: { en: "Scattered 18 (list shape)", fr: "18 dispersés (forme de la liste)", es: "18 dispersas (forma de la lista)" }, cells: hintgeoScattered18(N) },
  { key: "contig18", title: { en: "Contiguous 18", fr: "18 contigus", es: "18 contiguas" }, cells: contiguous(N, 18) },
  { key: "blocks3", title: { en: "Clustered 3×3 blocks", fr: "Blocs 3×3 groupés", es: "Bloques 3×3 agrupados" }, cells: clusteredClues(N, 3) },
  { key: "blocks4", title: { en: "Clustered 4×4 blocks", fr: "Blocs 4×4 groupés", es: "Bloques 4×4 agrupados" }, cells: clusteredClues(N, 4) },
];

const T = {
  en: {
    showFloor: "Show free-score seams",
    hideFloor: "Hide free-score seams",
    caption:
      "Ten geometries, one board. Toggle the seams to see the free-score floor: clustered layouts bank a pile of guaranteed-correct edges just by being pinned next to each other, while a spread layout banks none. That floor is why a raw score comparison flatters clustering — and why the study measures reached depth and solved-rate instead.",
  },
  fr: {
    showFloor: "Afficher les arêtes gratuites",
    hideFloor: "Masquer les arêtes gratuites",
    caption:
      "Dix géométries, un plateau. Activez les arêtes pour voir le plancher de score gratuit : les dispositions groupées engrangent une pile d'arêtes garanties correctes simplement en étant épinglées côte à côte, alors qu'une disposition dispersée n'en engrange aucune. Ce plancher explique pourquoi une comparaison de score brute flatte le regroupement — et pourquoi l'étude mesure plutôt la profondeur atteinte et le taux de résolution.",
  },
  es: {
    showFloor: "Mostrar aristas gratis",
    hideFloor: "Ocultar aristas gratis",
    caption:
      "Diez geometrías, un tablero. Activa las aristas para ver el suelo de puntuación gratis: las disposiciones agrupadas acumulan un montón de aristas garantizadas como correctas solo por estar fijadas una junto a otra, mientras que una disposición dispersa no acumula ninguna. Ese suelo explica por qué una comparación de puntuación bruta favorece al agrupamiento — y por qué el estudio mide en su lugar la profundidad alcanzada y la tasa de resolución.",
  },
};

export function HintLayoutGallery() {
  const t = useT(T);
  const { lang } = useLang();
  // The gallery titles carry en/fr/es; fall back to en for any other lang.
  const L: "en" | "fr" | "es" = lang === "fr" || lang === "es" ? lang : "en";
  const [showFloor, setShowFloor] = useState(false);

  return (
    <div className="space-y-3">
      <button
        onClick={() => setShowFloor((s) => !s)}
        className="rounded-md bg-muted px-3 py-1.5 text-xs font-medium hover:bg-muted-foreground/10"
        aria-pressed={showFloor}
      >
        {showFloor ? t.hideFloor : t.showFloor}
      </button>
      <div className="grid grid-cols-2 gap-4 sm:grid-cols-4">
        {ITEMS.map((it) => (
          <HintBoardTile
            key={it.key}
            n={N}
            cells={it.cells}
            title={it.title[L]}
            showFloor={showFloor}
          />
        ))}
      </div>
      <p className="mx-auto max-w-2xl text-center text-xs text-muted-foreground">{t.caption}</p>
    </div>
  );
}
