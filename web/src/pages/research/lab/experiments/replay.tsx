import { pageMeta } from "@/seo";
import { InventionLayout, type InventionCopy } from "@/components/research/InventionLayout";
import { DoubleBreakDiagram } from "@/components/research/DoubleBreakDiagram";
import { DoubleBreakLab } from "@/components/research/DoubleBreakLab";

const copy: { en: InventionCopy; fr: InventionCopy } = {
  en: {
    name: "REPLAY",
    tagline: "Rebuild the community's strict 460 boards exactly, and in doing so discover the move ordinary solvers can't make: paying two mismatches at a single cell.",
    idea: "The community's best fully-clued boards reach 460, but the project's own break-allowing search stalled at 457 or 458 no matter what. REPLAY set out to reproduce those 460 boards exactly, piece for piece, to learn what they were doing that our search could not. The answer turned out to be a single overlooked move.",
    how: (
      <>
        <p>
          A break-allowing search normally lets a cell carry at most one mismatch as it's placed.
          REPLAY relaxes that to allow two at certain cells, and reorders how candidate placements are
          ranked so that the moves a known good board actually used outrank cheaper-looking ones. With
          those two changes it can walk the same path the witness board took.
        </p>
        <p>
          Played back this way, the community 460 boards rebuild exactly, every piece in place, and the
          score checks out. The reproduction is the proof that the missing ingredient was real.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          Both community strict-460 boards replay exactly to 460. The discovery: each contains four or
          five cells that pay two mismatches at once. A search that allows only one mismatch per cell
          literally cannot reach those boards, which is exactly why the project's earlier runs
          saturated at 457 to 458. Allowing the double break lifts the strict ladder to 460.
        </p>
        <p>
          It's a clean explanation of a long-standing plateau, and a caution: a reasonable-looking rule
          (one break per cell) silently fenced off the very boards we were chasing.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Does allowing two breaks per cell open a path to 461 and beyond, or just to the known 460s?
          Are there boards needing a triple break? And can the double-break cells be predicted from a
          partial board rather than discovered by replay?
        </p>
      </>
    ),
  },
  fr: {
    name: "REPLAY",
    tagline: "Reconstruire exactement les plateaux 460 stricts de la communauté, et découvrir ainsi le coup que les solveurs ordinaires ne peuvent pas faire : payer deux défauts sur une même cellule.",
    idea: "Les meilleurs plateaux pleinement indicés de la communauté atteignent 460, mais la recherche à tolérance de défauts du projet calait à 457 ou 458 quoi qu'on fasse. REPLAY a entrepris de reproduire ces plateaux 460 exactement, pièce par pièce, pour comprendre ce qu'ils faisaient que notre recherche ne pouvait pas. La réponse s'est révélée être un unique coup négligé.",
    how: (
      <>
        <p>
          Une recherche à tolérance de défauts laisse normalement une cellule porter au plus un défaut
          à sa pose. REPLAY assouplit cela pour en autoriser deux sur certaines cellules, et réordonne
          le classement des placements candidats pour que les coups réellement utilisés par un bon
          plateau connu passent devant des coups d'apparence moins chère. Avec ces deux changements, il
          peut suivre le chemin même qu'a pris le plateau témoin.
        </p>
        <p>
          Rejoués ainsi, les plateaux 460 de la communauté se reconstruisent exactement, chaque pièce à
          sa place, et le score est confirmé. La reproduction est la preuve que l'ingrédient manquant
          était réel.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          Les deux plateaux 460 stricts de la communauté se rejouent exactement à 460. La découverte :
          chacun contient quatre ou cinq cellules qui paient deux défauts à la fois. Une recherche qui
          n'autorise qu'un défaut par cellule ne peut littéralement pas atteindre ces plateaux, et c'est
          précisément pourquoi les exécutions antérieures du projet saturaient à 457-458. Autoriser le
          double défaut hisse l'échelle stricte à 460.
        </p>
        <p>
          C'est une explication nette d'un plateau persistant, et une mise en garde : une règle
          d'apparence raisonnable (un défaut par cellule) avait discrètement clôturé les plateaux mêmes
          que nous poursuivions.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Autoriser deux défauts par cellule ouvre-t-il une voie vers 461 et au-delà, ou seulement vers
          les 460 connus ? Existe-t-il des plateaux exigeant un triple défaut ? Et les cellules à double
          défaut peuvent-elles être prédites depuis un plateau partiel plutôt que découvertes par
          rejeu ?
        </p>
      </>
    ),
  },
};

export default function Replay() {
  return (
    <InventionLayout
      copy={copy}
      score={460}
      reproducibility="seeded"
      visual={
        <div className="space-y-6">
          <DoubleBreakDiagram />
          <DoubleBreakLab />
        </div>
      }
    />
  );
}

export const meta = pageMeta("exp-replay");
