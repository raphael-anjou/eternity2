import { pageMeta } from "@/seo";
import { InventionLayout, type InventionCopy } from "@/components/research/InventionLayout";
import { MaskShapeDiagram } from "@/components/research/MaskShapeDiagram";

const copy: { en: InventionCopy; fr: InventionCopy } = {
  en: {
    name: "MIDDEN",
    tagline: "Decide in advance not when a board may break, but where: confine every mismatch to a chosen shape of cells, and search for the best shape.",
    idea: "Most break-allowing solvers control when a mismatch is permitted, at certain depths. MIDDEN controls where instead. It fixes a mask, a chosen set of cells, and rules that mismatches may only be paid inside it; everywhere else must match perfectly. Then it searches over the shape of that mask. Existing methods say when to take damage; MIDDEN asks where damage should live.",
    how: (
      <>
        <p>
          Pick a mask: a couple of rows, a couple of columns, a scattered lattice of cells, or a set
          chosen by color. Run the search forcing perfect matches outside the mask and allowing
          mismatches only inside it. Different mask shapes lead the search into different parts of the
          space, so the mask becomes a design knob rather than a fixed rule.
        </p>
        <p>
          Comparing shapes shows which geometry of allowed damage lets a board grow a long perfect run
          before it has to spend a mismatch.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          A dispersed lattice of allowed-damage cells extends the longest perfect run markedly further
          than concentrating the damage in a row or two, pushing the perfect wall from around 150 cells
          to the 170s. The mechanism is clear; what stays open is the economics, turning a longer
          perfect run into a higher final score once the endgame has to absorb the deferred damage.
        </p>
        <p>
          So MIDDEN is a genuinely new lever, the spatial complement to the usual timing controls, with
          a measured effect on how far a board stays perfect, but not yet a finished route to a record.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Which mask shapes convert a longer perfect run into actual matched edges at the end, rather
          than just deferring the damage? Can spatial masks be combined with timing controls so a board
          is gated in both where and when it may break? And is there a mask that mirrors where the best
          known boards actually carry their mismatches?
        </p>
      </>
    ),
  },
  fr: {
    name: "MIDDEN",
    tagline: "Décider à l'avance non pas quand un plateau peut casser, mais où : confiner chaque défaut à une forme de cellules choisie, et chercher la meilleure forme.",
    idea: "La plupart des solveurs à tolérance de défauts contrôlent quand un défaut est permis, à certaines profondeurs. MIDDEN contrôle plutôt où. Il fixe un masque, un ensemble de cellules choisi, et impose que les défauts ne soient payés qu'à l'intérieur ; partout ailleurs doit s'accorder parfaitement. Puis il cherche sur la forme de ce masque. Les méthodes existantes disent quand subir des dégâts ; MIDDEN demande où les dégâts doivent vivre.",
    how: (
      <>
        <p>
          On choisit un masque : deux rangées, deux colonnes, un réseau dispersé de cellules, ou un
          ensemble choisi par couleur. On lance la recherche en forçant les accords parfaits hors du
          masque et en n'autorisant les défauts qu'à l'intérieur. Des formes de masque différentes
          mènent la recherche dans des parties différentes de l'espace, si bien que le masque devient un
          bouton de conception plutôt qu'une règle fixe.
        </p>
        <p>
          Comparer les formes montre quelle géométrie de dégâts autorisés permet à un plateau de faire
          une longue suite parfaite avant de devoir dépenser un défaut.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          Un réseau dispersé de cellules à dégâts autorisés prolonge nettement la plus longue suite
          parfaite, davantage que de concentrer les dégâts sur une ou deux rangées, poussant le mur
          parfait d'environ 150 cellules vers les 170. Le mécanisme est clair ; ce qui reste ouvert,
          c'est l'économie, transformer une suite parfaite plus longue en un meilleur score final une
          fois que la fin de partie doit absorber les dégâts différés.
        </p>
        <p>
          MIDDEN est donc un levier vraiment nouveau, le complément spatial des contrôles de timing
          habituels, avec un effet mesuré sur la distance où un plateau reste parfait, mais pas encore
          une route achevée vers un record.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Quelles formes de masque convertissent une suite parfaite plus longue en bords réellement
          accordés à la fin, au lieu de seulement différer les dégâts ? Les masques spatiaux peuvent-ils
          se combiner aux contrôles de timing pour qu'un plateau soit régulé à la fois où et quand il
          peut casser ? Et existe-t-il un masque qui reflète où les meilleurs plateaux connus portent
          réellement leurs défauts ?
        </p>
      </>
    ),
  },
};

export default function Midden() {
  return (
    <InventionLayout
      copy={copy}
      score={452}
      reproducibility="seeded"
      visual={<MaskShapeDiagram />}
    />
  );
}

export const meta = pageMeta("exp-midden");
