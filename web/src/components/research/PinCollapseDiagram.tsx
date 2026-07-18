import { useT, useLang, pick } from "@/i18n";
import { useIsClient } from "@/lib/utils";

// The corner-pin collapse, drawn to scale. McGavin's C fills 205 of 256 cells on
// its native five-clue puzzle, but only 21 once three corners are pinned; its
// fixed scan path cannot absorb a pin. Blackwood's C# reaches 34 on the pinned
// five-clue. Each bar is depth-reached against the 256-cell ceiling, so the
// collapse is visible at a glance. Numbers match results/community.json.

const CEIL = 256;

type Row = { key: string; en: string; fr: string; es: string; depth: number; strong?: boolean };

const ROWS: Row[] = [
  {
    key: "mcgavin-native",
    en: "McGavin C — native 5-clue",
    fr: "McGavin C — 5 indices natif",
    es: "McGavin C — 5 pistas nativo",
    depth: 205,
    strong: true,
  },
  {
    key: "mcgavin-pinned",
    en: "McGavin C — our corner-pinned",
    fr: "McGavin C — nos coins fixés",
    es: "McGavin C — con nuestras esquinas fijadas",
    depth: 21,
  },
  {
    key: "blackwood",
    en: "Blackwood C# — pinned 5-clue",
    fr: "Blackwood C# — 5 indices fixés",
    es: "Blackwood C# — 5 pistas fijadas",
    depth: 34,
  },
];

const T = {
  en: {
    caption:
      "Depth reached out of 256. McGavin's engine fills 205 cells on the puzzle its scan path was compiled for, but pinning three corners collapses it to 21: the fixed path meets a pinned corner it cannot satisfy and dead-ends almost at once. Blackwood, tuned for the near-unconstrained one-clue instance, stalls at 34 on the pinned five-clue. This is why an engine built around one clue configuration cannot be dropped onto another, and why the study's own pin-native engines are the runnable stand-ins.",
    axis: "cells placed (of 256)",
    busy: "Loading…",
  },
  fr: {
    caption:
      "Profondeur atteinte sur 256. Le moteur de McGavin pose 205 cases sur le puzzle pour lequel son parcours a été compilé, mais fixer trois coins le fait chuter à 21 : le parcours figé rencontre un coin fixé qu'il ne peut satisfaire et se bloque presque aussitôt. Blackwood, réglé pour l'instance à un seul indice, plafonne à 34 sur les cinq indices fixés. Voilà pourquoi un moteur bâti autour d'une configuration d'indices ne peut être transposé à une autre, et pourquoi les moteurs de l'étude, qui gèrent nativement les indices, sont les substituts exécutables.",
    axis: "cases posées (sur 256)",
    busy: "Chargement…",
  },
  es: {
    caption:
      "Profundidad alcanzada sobre 256. El motor de McGavin coloca 205 celdas en el puzzle para el que se compiló su recorrido, pero fijar tres esquinas lo hace caer a 21: el recorrido rígido encuentra una esquina fijada que no puede satisfacer y se estanca casi de inmediato. Blackwood, ajustado para la instancia de una sola pista, casi sin restricciones, se detiene en 34 sobre las cinco pistas fijadas. Por eso un motor construido en torno a una configuración de pistas no puede trasladarse a otra, y por eso los propios motores del estudio, que gestionan las pistas de forma nativa, son los sustitutos ejecutables.",
    axis: "celdas colocadas (de 256)",
    busy: "Cargando…",
  },
};

export function PinCollapseDiagram() {
  const t = useT(T);
  const { lang } = useLang();
  const isClient = useIsClient();

  if (!isClient) {
    return (
      <div className="flex h-48 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  return (
    <div className="space-y-3">
      <div className="space-y-2.5">
        {ROWS.map((r) => {
          const pct = (r.depth / CEIL) * 100;
          return (
            <div key={r.key} className="text-sm">
              <div className="mb-1 flex items-baseline justify-between">
                <span className="font-medium">{pick(r, lang)}</span>
                <span className="tabular-nums text-muted-foreground">{r.depth} / 256</span>
              </div>
              <div
                className="h-4 w-full overflow-hidden rounded bg-muted"
                role="img"
                aria-label={`${r.en}: ${r.depth} of 256 cells placed`}
              >
                <div
                  className="h-full rounded"
                  style={{
                    width: `${pct}%`,
                    minWidth: "2px",
                    // Native run in violet (the community hue); collapsed runs muted.
                    backgroundColor: r.strong ? "#8b5cf6" : "#c4b5fd",
                  }}
                />
              </div>
            </div>
          );
        })}
      </div>
      <p className="text-center text-xs text-muted-foreground">{t.axis}</p>
      <p className="text-sm leading-relaxed text-muted-foreground">{t.caption}</p>
    </div>
  );
}
