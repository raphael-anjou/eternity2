// Gallery of the 2026 five-clue record wave: every board whose e2.bucas.name
// link was shared, rendered from its own edges and linking into our viewer so
// each edge is checkable. Two data sets, kept visually distinct:
//   - FIVE_CLUE_RECORDS_2026    — full boards scored by matched edges /480
//   - PARTIALS_WITH_HINTS_2026  — spiral-in partials respecting all five clues
//   - PARTIALS_OTHER_2026       — partial-hint / run-length partials, off the ladder
// Scores/counts were re-verified from each board's edges before publishing.

import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { LazyBoardPreview } from "@/components/research/LazyBoardPreview";
import {
  FIVE_CLUE_RECORDS_2026,
  FIVE_CLUE_CENSUS_2026,
  PARTIALS_WITH_HINTS_2026,
  PARTIALS_OTHER_2026,
  type LinearPartial,
} from "@/data/community-boards-2026";

const T = {
  en: {
    recordsHeading: "Five-clue record boards (464 → 460)",
    recordsIntro:
      "Every board below is a full 256-piece layout that respects all five official clues, scored by matched edges out of 480. Benjamin Riotte's 464 broke Bruno Gauthier's 460, which had stood since 2023; Igor Pejic reached the same 463–464 range independently. These are the boards a verification link was shared for — the full census is far larger (10 boards at 464, 290 at 463, 2213 at 462, 38 at 461). Click any board to re-score it edge by edge in the viewer.",
    partialsHeading: "Spiral-in partials, all five clues",
    partialsIntro:
      "A parallel, different pursuit: partial boards that place as many pieces as possible while respecting all five official clues, built by a spiral-in fill. The number is the pieces placed, not an edge score. Peter McGavin and Patrick Hegland posted these to the groups.io SAT thread in 2024; One Small Step continued the line on Discord in 2025–26. The counts are re-verified from each board. Thanks to Peter McGavin for flagging that the SAT-thread boards belonged here.",
    partialsOtherHeading: "Other partials & run-length records",
    partialsOtherIntro:
      "Kept for the record, but off the clean ladder above: boards that respect only the starter clue, boards from a source without piece ids (so clue compliance can't be verified), and boards reported as a continuous-run length rather than a placement count. Each is labelled with its caveat.",
    hintsFull: "all 5 clues",
    hintsPartial: (n: number) => `${n}/5 clues`,
    hintsUnverified: "clues unverified",
    metricRun: "continuous run",
    metricLinear: "linear",
    placedLabel: "placed",
    open: "Open in the viewer",
    census: "Distinct boards reported by score",
    censusCols: { score: "Score", broken: "Broken edges", found: "Distinct boards found" },
    censusNote:
      "The author's own count. Many boards at the lower scores are close variants within one basin, so the raw totals overstate the number of structurally independent solutions.",
    dist: (n: string) => `${n} tiles from the public 460`,
    piecesPlaced: "placed",
  },
  fr: {
    recordsHeading: "Plateaux records à cinq indices (464 → 460)",
    recordsIntro:
      "Chaque plateau ci-dessous est une disposition complète de 256 pièces qui respecte les cinq indices officiels, notée en arêtes appariées sur 480. Le 464 de Benjamin Riotte a battu le 460 de Bruno Gauthier, qui tenait depuis 2023 ; Igor Pejic a atteint la même plage 463–464 de façon indépendante. Ce sont les plateaux pour lesquels un lien de vérification a été partagé — le recensement complet est bien plus vaste (10 plateaux à 464, 290 à 463, 2213 à 462, 38 à 461). Cliquez sur un plateau pour le re-noter arête par arête dans le visualiseur.",
    partialsHeading: "Partiels en spirale, les cinq indices",
    partialsIntro:
      "Une quête parallèle et différente : des plateaux partiels qui posent le plus de pièces possible tout en respectant les cinq indices officiels, construits par un remplissage en spirale. Le nombre indique les pièces posées, pas un score d'arêtes. Peter McGavin et Patrick Hegland les ont publiés dans le fil SAT de groups.io en 2024 ; One Small Step a prolongé la lignée sur Discord en 2025–26. Les décomptes sont revérifiés depuis chaque plateau. Merci à Peter McGavin d'avoir signalé que les plateaux du fil SAT avaient leur place ici.",
    partialsOtherHeading: "Autres partiels et records de série continue",
    partialsOtherIntro:
      "Conservés pour mémoire, mais hors de l'échelle ci-dessus : des plateaux ne respectant que l'indice de départ, des plateaux issus d'une source sans identifiants de pièces (le respect des indices n'est donc pas vérifiable), et des plateaux rapportés comme une longueur de série continue plutôt qu'un nombre de pièces posées. Chacun porte sa mise en garde.",
    hintsFull: "5 indices",
    hintsPartial: (n: number) => `${n}/5 indices`,
    hintsUnverified: "indices non vérifiés",
    metricRun: "série continue",
    metricLinear: "linéaire",
    placedLabel: "posées",
    open: "Ouvrir dans le visualiseur",
    census: "Plateaux distincts rapportés, par score",
    censusCols: { score: "Score", broken: "Arêtes brisées", found: "Plateaux distincts trouvés" },
    censusNote:
      "Le décompte de l'auteur lui-même. Beaucoup de plateaux aux scores plus bas sont des variantes proches d'un même bassin ; les totaux bruts surestiment donc le nombre de solutions structurellement indépendantes.",
    dist: (n: string) => `${n} tuiles du 460 public`,
    piecesPlaced: "posées",
  },
  es: {
    recordsHeading: "Tableros récord con cinco pistas (464 → 460)",
    recordsIntro:
      "Cada tablero de abajo es una disposición completa de 256 piezas que respeta las cinco pistas oficiales, puntuada por aristas coincidentes sobre 480. El 464 de Benjamin Riotte superó el 460 de Bruno Gauthier, que se mantenía desde 2023; Igor Pejic alcanzó el mismo rango 463–464 de forma independiente. Estos son los tableros para los que se compartió un enlace de verificación: el censo completo es mucho mayor (10 tableros a 464, 290 a 463, 2213 a 462, 38 a 461). Haz clic en cualquier tablero para volver a puntuarlo arista por arista en el visor.",
    partialsHeading: "Parciales en espiral, las cinco pistas",
    partialsIntro:
      "Una búsqueda paralela y distinta: tableros parciales que colocan tantas piezas como sea posible respetando las cinco pistas oficiales, construidos con un relleno en espiral. El número indica las piezas colocadas, no una puntuación de aristas. Peter McGavin y Patrick Hegland los publicaron en el hilo SAT de groups.io en 2024; One Small Step continuó la línea en Discord en 2025–26. Los recuentos están reverificados a partir de cada tablero. Gracias a Peter McGavin por señalar que los tableros del hilo SAT tenían su lugar aquí.",
    partialsOtherHeading: "Otros parciales y récords de serie continua",
    partialsOtherIntro:
      "Conservados para dejar constancia, pero fuera de la escala limpia de arriba: tableros que solo respetan la pista inicial, tableros de una fuente sin identificadores de pieza (por lo que no se puede verificar el cumplimiento de las pistas) y tableros reportados como una longitud de serie continua en lugar de un recuento de piezas colocadas. Cada uno lleva su advertencia.",
    hintsFull: "las 5 pistas",
    hintsPartial: (n: number) => `${n}/5 pistas`,
    hintsUnverified: "pistas sin verificar",
    metricRun: "serie continua",
    metricLinear: "lineal",
    placedLabel: "colocadas",
    open: "Abrir en el visor",
    census: "Tableros distintos reportados, por puntuación",
    censusCols: { score: "Puntuación", broken: "Aristas rotas", found: "Tableros distintos encontrados" },
    censusNote:
      "El recuento del propio autor. Muchos tableros con puntuaciones más bajas son variantes cercanas dentro de una misma cuenca, de modo que los totales brutos sobrestiman el número de soluciones estructuralmente independientes.",
    dist: (n: string) => `${n} piezas del 460 público`,
    piecesPlaced: "colocadas",
  },
};

function Thumb({
  params,
  href,
  badge,
  caption,
  cta,
}: {
  params: string;
  href: string;
  badge: string;
  caption?: string;
  cta: string;
}) {
  return (
    <div className="space-y-1.5">
      <LocalizedLink
        to={href}
        aria-label={cta}
        className="block rounded-lg border p-1.5 transition-shadow hover:shadow-md"
      >
        <div className="relative">
          <LazyBoardPreview params={params} />
          <span className="absolute right-1 top-1 rounded bg-background/85 px-1.5 py-0.5 text-[11px] font-semibold tabular-nums shadow-sm">
            {badge}
          </span>
        </div>
      </LocalizedLink>
      {caption ? <p className="text-[11px] leading-tight text-muted-foreground">{caption}</p> : null}
    </div>
  );
}

export function BoardsFoundGallery() {
  const t = useT(T);
  return (
    <div className="my-8 space-y-10">
      <section className="space-y-4">
        <div className="max-w-3xl space-y-2">
          <h3 className="text-lg font-semibold tracking-tight">{t.recordsHeading}</h3>
          <p className="text-sm leading-relaxed text-muted-foreground">{t.recordsIntro}</p>
        </div>

        <div className="max-w-2xl space-y-2">
          <p className="text-sm font-medium">{t.census}</p>
          <table className="w-full max-w-md text-sm">
            <thead>
              <tr className="border-b text-left text-xs text-muted-foreground">
                <th className="py-1 pr-4 font-medium">{t.censusCols.score}</th>
                <th className="py-1 pr-4 font-medium">{t.censusCols.broken}</th>
                <th className="py-1 font-medium">{t.censusCols.found}</th>
              </tr>
            </thead>
            <tbody>
              {FIVE_CLUE_CENSUS_2026.map((c) => (
                <tr key={c.score} className="border-b last:border-0">
                  <td className="py-1 pr-4 font-semibold tabular-nums">{c.score} / 480</td>
                  <td className="py-1 pr-4 tabular-nums text-muted-foreground">{c.broken}</td>
                  <td className="py-1 tabular-nums text-muted-foreground">{c.found.toLocaleString()}</td>
                </tr>
              ))}
            </tbody>
          </table>
          <p className="max-w-xl text-xs text-muted-foreground">{t.censusNote}</p>
        </div>

        <div className="grid grid-cols-2 gap-4 sm:grid-cols-3 lg:grid-cols-4">
          {FIVE_CLUE_RECORDS_2026.map((b) => (
            <Thumb
              key={b.id}
              params={b.params}
              href={`/viewer?${b.params}`}
              badge={`${b.score}`}
              cta={t.open}
              caption={
                b.author === "Igor Pejic"
                  ? `${b.score} · Igor Pejic`
                  : b.note
                    ? `${b.score} · ${t.dist(b.note.replace(/\D/g, ""))}`
                    : `${b.score} · Benjamin Riotte`
              }
            />
          ))}
        </div>
      </section>

      <section className="space-y-4">
        <div className="max-w-3xl space-y-2">
          <h3 className="text-lg font-semibold tracking-tight">{t.partialsHeading}</h3>
          <p className="text-sm leading-relaxed text-muted-foreground">{t.partialsIntro}</p>
        </div>
        <div className="grid grid-cols-2 gap-4 sm:grid-cols-3 lg:grid-cols-4">
          {PARTIALS_WITH_HINTS_2026.map((b) => (
            <PartialThumb key={b.id} b={b} t={t} />
          ))}
        </div>
      </section>

      <section className="space-y-4">
        <div className="max-w-3xl space-y-2">
          <h3 className="text-lg font-semibold tracking-tight">{t.partialsOtherHeading}</h3>
          <p className="text-sm leading-relaxed text-muted-foreground">{t.partialsOtherIntro}</p>
        </div>
        <div className="grid grid-cols-2 gap-4 sm:grid-cols-3 lg:grid-cols-4">
          {PARTIALS_OTHER_2026.map((b) => (
            <PartialThumb key={b.id} b={b} t={t} />
          ))}
        </div>
      </section>
    </div>
  );
}

/** A partial board thumb: badge = pieces placed, caption = author + clue status
 *  (+ any caveat), so a spiral-in record and a starter-only board never read as
 *  the same thing. */
function PartialThumb({ b, t }: { b: LinearPartial; t: (typeof T)["en"] }) {
  const clue =
    b.hints === null
      ? t.hintsUnverified
      : b.hints === 5
        ? t.hintsFull
        : t.hintsPartial(b.hints);
  const metric =
    b.metric === "continuous-run" ? ` · ${t.metricRun}` : b.metric === "linear" ? ` · ${t.metricLinear}` : "";
  const caption = `${b.author} · ${clue}${metric}${b.note ? ` · ${b.note}` : ""}`;
  return (
    <Thumb
      params={b.params}
      href={`/viewer?${b.params}`}
      badge={`${b.placed}`}
      cta={t.open}
      caption={caption}
    />
  );
}
