// Gallery of the 2026 five-clue record wave: every board whose e2.bucas.name
// link was shared, rendered from its own edges and linking into our viewer so
// each edge is checkable. Two data sets, kept visually distinct:
//   - FIVE_CLUE_RECORDS_2026 — full boards scored by matched edges /480
//   - LINEAR_PARTIALS_2026    — partial constructions, counted by pieces placed
// Scores/counts were re-verified with scoreCells before publishing.

import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { LazyBoardPreview } from "@/components/research/LazyBoardPreview";
import {
  FIVE_CLUE_RECORDS_2026,
  LINEAR_PARTIALS_2026,
} from "@/data/community-boards-2026";

const T = {
  en: {
    recordsHeading: "Five-clue record boards (464 → 460)",
    recordsIntro:
      "Every board below is a full 256-piece layout that respects all five official clues, scored by matched edges out of 480. Benjamin Riotte's 464 broke Bruno Gauthier's 460, which had stood since 2023; Igor Pejic reached the same 463–464 range independently. These are the boards a verification link was shared for — the full census is far larger (10 boards at 464, 290 at 463, 2213 at 462, 38 at 461). Click any board to re-score it edge by edge in the viewer.",
    partialsHeading: "Linear & spiral-in partials (One Small Step)",
    partialsIntro:
      "A parallel, different achievement: partial boards built as long linear runs or spiral-in constructions, shared on the community Discord between December 2025 and February 2026. The number is the pieces placed (or the continuous run length), not an edge score — these track the consecutive-partial lineage, not the record ladder.",
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
    partialsHeading: "Partiels linéaires et en spirale (One Small Step)",
    partialsIntro:
      "Un accomplissement parallèle et différent : des plateaux partiels construits en longues séries linéaires ou en spirale, partagés sur le Discord de la communauté entre décembre 2025 et février 2026. Le nombre indique les pièces posées (ou la longueur de la série continue), pas un score d'arêtes — ils relèvent de la lignée des partiels consécutifs, pas de l'échelle des records.",
    open: "Ouvrir dans le visualiseur",
    census: "Plateaux distincts rapportés, par score",
    censusCols: { score: "Score", broken: "Arêtes brisées", found: "Plateaux distincts trouvés" },
    censusNote:
      "Le décompte de l'auteur lui-même. Beaucoup de plateaux aux scores plus bas sont des variantes proches d'un même bassin ; les totaux bruts surestiment donc le nombre de solutions structurellement indépendantes.",
    dist: (n: string) => `${n} tuiles du 460 public`,
    piecesPlaced: "posées",
  },
};

const CENSUS: { score: number; broken: number; found: number }[] = [
  { score: 464, broken: 16, found: 10 },
  { score: 463, broken: 17, found: 290 },
  { score: 462, broken: 18, found: 2213 },
  { score: 461, broken: 19, found: 38 },
];

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
              {CENSUS.map((c) => (
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
          {LINEAR_PARTIALS_2026.map((b) => (
            <Thumb
              key={b.id}
              params={b.params}
              href={`/viewer?${b.params}`}
              badge={b.note.match(/\d+/)?.[0] ?? "·"}
              cta={t.open}
              caption={`${b.date.split(" ")[0]} · ${b.note}`}
            />
          ))}
        </div>
      </section>
    </div>
  );
}
