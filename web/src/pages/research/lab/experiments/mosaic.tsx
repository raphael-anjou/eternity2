import { pageMeta } from "@/seo";
import { InventionLayout, type InventionCopy } from "@/components/research/InventionLayout";

// MOSAIC (vol-206): tile the board into 4x4 blocks, solve each block to exact
// MaxSAT optimality with its boundary to already-placed neighbours as SOFT
// targets (so a block never goes infeasible), and reserve the scarcest pieces
// for later blocks to fight piece-theft. From scratch, no warm start, no ALNS:
// 448/480. All figures from the vault concept mosaic-window-maxsat.

const copy: { en: InventionCopy; fr: InventionCopy } = {
  en: {
    name: "MOSAIC",
    tagline: "Tile the board into small blocks, solve each one to proven optimality, and glue them together — paying for the seams instead of forbidding them. From scratch, with no record to copy, it reaches 448.",
    idea: "Eternity II has no local structure you can exploit globally — but a small window of it, a 4×4 block, solves to perfect optimality in seconds. MOSAIC is built on that single fact: if you can solve a block exactly, can you compose sixteen exact blocks into a whole board?",
    how: (
      <>
        <p>
          The 16×16 board is cut into sixteen 4×4 blocks, filled one at a time. Each block is handed
          to an exact MaxSAT solver, which finds the best possible placement of pieces in it. The
          trick is in how a block meets its already-placed neighbours: those shared edges are not hard
          requirements but <em>soft</em> targets the block is rewarded for matching. So a block can
          never become impossible — it simply pays for any seam it can't match, and always completes.
        </p>
        <p>
          The second idea fights piece theft directly. Before filling a block, MOSAIC holds back the
          globally scarcest pieces, so the last blocks aren't starved of the rare pieces their seams
          will demand. Tuning how much to reserve is the one real knob; too little and the corner
          starves, too much and the early blocks suffer.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          The window primitive does what it promises: a 4×4 block solves to its 24-edge optimum in
          about thirty seconds, a 3×3 in eleven — confirming the puzzle really is tractable in the
          small. Composed across the whole board, from scratch and with no warm start, MOSAIC reaches
          448 of 480. The reservation sweet spot is around eight percent of the pool.
        </p>
        <p>
          The shortfall is informative: almost all of it is in the last three blocks of the
          bottom-right corner, where the pool finally runs thin — piece theft again, now visible as a
          single bright spot on the board. Exact-in-the-small does compose, but the composition order
          spends its freedom early and pays for it at the end, the same shape every method here runs
          into.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Would a non-row-major block order — spiralling in, or solving the constrained corner first —
          move the depletion off the hardest block? Could blocks overlap, so seams are solved twice
          and reconciled? And does a faster (Rust) primitive make a larger block size, with its
          stronger exact guarantee, affordable?
        </p>
      </>
    ),
  },
  fr: {
    name: "MOSAIC",
    tagline: "Découper le plateau en petits blocs, résoudre chacun à l'optimum prouvé, puis les recoller — en payant les jointures au lieu de les interdire. De zéro, sans record à copier, il atteint 448.",
    idea: "Eternity II n'a aucune structure locale exploitable globalement — mais une petite fenêtre, un bloc 4×4, se résout à l'optimum parfait en quelques secondes. MOSAIC repose sur ce seul fait : si l'on sait résoudre un bloc exactement, peut-on composer seize blocs exacts en un plateau entier ?",
    how: (
      <>
        <p>
          Le plateau 16×16 est découpé en seize blocs 4×4, remplis un par un. Chaque bloc est confié à
          un solveur MaxSAT exact, qui trouve le meilleur placement possible des pièces. L'astuce est
          dans la façon dont un bloc rencontre ses voisins déjà posés : ces bords partagés ne sont pas
          des exigences dures mais des cibles <em>souples</em> que le bloc est récompensé d'apparier.
          Un bloc ne peut donc jamais devenir impossible — il paie simplement toute jointure qu'il ne
          peut apparier, et se termine toujours.
        </p>
        <p>
          La seconde idée combat directement le vol de pièce. Avant de remplir un bloc, MOSAIC retient
          les pièces globalement les plus rares, pour que les derniers blocs ne soient pas privés des
          pièces rares que leurs jointures réclameront. Régler la quantité à réserver est le seul vrai
          bouton ; trop peu et le coin meurt de faim, trop et les premiers blocs en souffrent.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          La primitive de fenêtre tient sa promesse : un bloc 4×4 se résout à son optimum de 24 bords
          en une trentaine de secondes, un 3×3 en onze — confirmant que le puzzle est bien traitable en
          petit. Composé sur tout le plateau, de zéro et sans amorce, MOSAIC atteint 448 sur 480. Le
          point idéal de réservation est autour de huit pour cent du stock.
        </p>
        <p>
          Le déficit est instructif : il est presque entièrement dans les trois derniers blocs du coin
          inférieur droit, là où le stock s'épuise enfin — le vol de pièce, à nouveau, visible cette
          fois comme un seul point vif sur le plateau. L'exact-en-petit se compose bien, mais l'ordre de
          composition dépense sa liberté tôt et la paie à la fin, la forme même que rencontre chaque
          méthode ici.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Un ordre de blocs non row-major — en spirale, ou résolvant d'abord le coin contraint —
          déplacerait-il l'épuisement hors du bloc le plus dur ? Les blocs pourraient-ils se
          chevaucher, pour que les jointures soient résolues deux fois et réconciliées ? Et une
          primitive plus rapide (en Rust) rendrait-elle abordable une taille de bloc plus grande, à la
          garantie exacte plus forte ?
        </p>
      </>
    ),
  },
};

export default function Mosaic() {
  return <InventionLayout copy={copy} score={448} reproducibility="deterministic" />;
}

export const meta = pageMeta("exp-mosaic");
