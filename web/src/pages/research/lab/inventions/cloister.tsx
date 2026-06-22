import { pageMeta } from "@/seo";
import { InventionLayout, type InventionCopy } from "@/components/research/InventionLayout";
import { BorderAnchorDiagram } from "@/components/research/BorderAnchorDiagram";

const copy: { en: InventionCopy; fr: InventionCopy } = {
  en: {
    name: "CLOISTER",
    tagline: "Fix a perfect border, then search the interior with the border's edges treated as hard constraints from the very first cell.",
    idea: "The border and the interior are usually solved together, which wastes effort: the interior keeps proposing pieces that can't possibly meet the border later. CLOISTER pins a perfect 60-piece frame first, then searches the 14x14 interior with the frame's inward-facing edges as real constraints from cell one, so a doomed interior is rejected immediately instead of at the end.",
    how: (
      <>
        <p>
          Start from a complete, perfectly-matched border. The interior search is a break-allowing
          depth-first fill, but the cells along the inner edge of the frame must match the frame, and
          that requirement is live from the first placement, not checked only when the interior is
          finished. Exact endgames score the last region, including how well it seals against the rim.
        </p>
        <p>
          Because the border is fixed, the search collects something a post-hoc interior can't: the
          handful of extra edges that come from the interior actually fitting the rim it was built
          against, rather than being grafted onto a rim later.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          As a standalone interior solver CLOISTER reaches an interior score of 453 unhinted, in
          minutes, and confirms a real effect: an interior built against its own rim attaches with
          a few more matched edges than the same-quality interior attached after the fact. With all
          five official clues forced it settles in the high 440s to low 450s.
        </p>
        <p>
          It doesn't break the top records, and it saturates like everything else does near the wall.
          But it cleanly isolates and measures the border-interior coupling that whole-board search
          blurs together.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          How much of the rim-compatibility bonus can be collected without fixing the border first?
          Does scanning many borders, rather than one, find an interior that attaches better still?
          And where exactly does the strict-clue version's ceiling come from?
        </p>
      </>
    ),
  },
  fr: {
    name: "CLOISTER",
    tagline: "Fixer une bordure parfaite, puis chercher l'intérieur en traitant les bords de la bordure comme des contraintes dures dès la toute première cellule.",
    idea: "La bordure et l'intérieur sont d'ordinaire résolus ensemble, ce qui gaspille de l'effort : l'intérieur propose sans cesse des pièces qui ne pourront jamais rejoindre la bordure plus tard. CLOISTER fixe d'abord un cadre parfait de 60 pièces, puis cherche l'intérieur 14x14 avec les bords intérieurs du cadre comme vraies contraintes dès la première cellule, si bien qu'un intérieur condamné est rejeté immédiatement plutôt qu'à la fin.",
    how: (
      <>
        <p>
          On part d'une bordure complète et parfaitement accordée. La recherche d'intérieur est un
          remplissage en profondeur à tolérance de défauts, mais les cellules le long du bord intérieur
          du cadre doivent s'accorder au cadre, et cette exigence est active dès la première pose, pas
          vérifiée seulement une fois l'intérieur fini. Des fins de partie exactes évaluent la dernière
          région, y compris son étanchéité contre le pourtour.
        </p>
        <p>
          Parce que la bordure est fixe, la recherche collecte ce qu'un intérieur posé après coup ne
          peut pas : la poignée de bords supplémentaires qui viennent de ce que l'intérieur s'ajuste
          vraiment au pourtour contre lequel il a été bâti.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          Comme solveur d'intérieur autonome, CLOISTER atteint un score intérieur de 453 sans indices,
          en quelques minutes, et confirme un effet réel : un intérieur bâti contre son propre pourtour
          s'attache avec quelques bords accordés de plus que le même intérieur attaché après coup. Avec
          les cinq indices officiels forcés, il se stabilise dans les hauts 440 à bas 450.
        </p>
        <p>
          Il ne bat pas les records de tête et sature comme tout le reste près du mur. Mais il isole et
          mesure proprement le couplage bordure-intérieur que la recherche plateau-entier mélange.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Quelle part du bonus de compatibilité du pourtour peut-on collecter sans fixer d'abord la
          bordure ? Balayer plusieurs bordures, plutôt qu'une, trouve-t-il un intérieur qui s'attache
          encore mieux ? Et d'où vient exactement le plafond de la version à indices stricts ?
        </p>
      </>
    ),
  },
};

export default function Cloister() {
  return (
    <InventionLayout
      copy={copy}
      score={453}
      reproducibility="seeded"
      visual={<BorderAnchorDiagram />}
    />
  );
}

export const meta = pageMeta("inv-cloister");
