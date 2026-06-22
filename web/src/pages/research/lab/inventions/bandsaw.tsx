import { pageMeta } from "@/seo";
import { InventionLayout, type InventionCopy } from "@/components/research/InventionLayout";
import { MeetInMiddleDiagram } from "@/components/research/MeetInMiddleDiagram";

const copy: { en: InventionCopy; fr: InventionCopy } = {
  en: {
    name: "BANDSAW",
    tagline: "Solve a band of rows exactly by meeting in the middle, to find the true best ending and to measure how far ahead an endgame can be decided.",
    idea: "Heuristic search guesses; it never knows it has the best possible finish. BANDSAW is the opposite: for a band of rows near the bottom, it computes the exact best completion, with a proof that nothing scores higher. The aim isn't speed, it's certainty, and the certainty doubles as a ruler for how hard the endgame really is.",
    how: (
      <>
        <p>
          Split the band into a top half and a bottom half. Enumerate every way to fill the top half
          up to a small mismatch budget, keyed by two things: which pieces it used, and the row of
          colors it leaves dangling at the seam. Enumerate the bottom half the same way, but only from
          the pieces the top half didn't use. Then join the two halves wherever their seam colors
          agree and their piece sets don't overlap. That meet-in-the-middle join finds the exact best
          completion without walking the whole tree.
        </p>
        <p>
          Exact lower-bound tables, computed by working backwards column by column, let it prune
          branches that already can't beat the budget, and it raises the budget step by step until a
          round finds nothing new, which proves the best score for that band.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          On a 10x10 testbed BANDSAW settles the endgame exactly and pins the budget where exactness
          stops being affordable: the search tree grows about twenty-fold per extra mismatch, on both
          sides, so meeting in the middle stops paying off at full board size. That negative is the
          useful part: it tells you exactly where exact methods give out and heuristics must take over.
          The exact pieces that survived, the suffix lower-bound tables and the pruned branch-and-bound,
          became reusable instruments. A frame-free board scoring 437 came out of the same machinery.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Can the lower-bound tables scale to the full 16x16 endgame, or does the seam state space grow
          too large? At what band size does memory rather than time become the limit? And can the rare
          cases where the middle join does fire be spotted in advance and finished exactly?
        </p>
      </>
    ),
  },
  fr: {
    name: "BANDSAW",
    tagline: "Résoudre une bande de rangées exactement en se rejoignant au milieu, pour trouver la vraie meilleure fin et mesurer jusqu'où une fin de partie peut être décidée à l'avance.",
    idea: "La recherche heuristique devine ; elle ne sait jamais qu'elle tient la meilleure fin possible. BANDSAW fait l'inverse : pour une bande de rangées près du bas, il calcule la meilleure complétion exacte, avec une preuve que rien ne fait mieux. Le but n'est pas la vitesse mais la certitude, et cette certitude sert aussi de règle pour mesurer la difficulté réelle de la fin de partie.",
    how: (
      <>
        <p>
          On coupe la bande en une moitié haute et une moitié basse. On énumère toutes les façons de
          remplir la moitié haute jusqu'à un petit budget de défauts, indexées par deux choses : les
          pièces utilisées, et la rangée de couleurs laissée pendante à la couture. On énumère la moitié
          basse de même, mais seulement à partir des pièces que le haut n'a pas utilisées. Puis on joint
          les deux moitiés là où leurs couleurs de couture s'accordent et où leurs ensembles de pièces
          ne se chevauchent pas. Cette jonction au milieu trouve la meilleure complétion exacte sans
          parcourir tout l'arbre.
        </p>
        <p>
          Des tables de bornes inférieures exactes, calculées à rebours colonne par colonne, permettent
          d'élaguer les branches qui ne peuvent déjà plus battre le budget, et on relève le budget pas à
          pas jusqu'à ce qu'un tour ne trouve rien de neuf, ce qui prouve le meilleur score de la bande.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          Sur un banc d'essai 10x10, BANDSAW règle la fin de partie exactement et fixe le budget où
          l'exactitude cesse d'être abordable : l'arbre de recherche croît d'environ vingt fois par
          défaut supplémentaire, des deux côtés, donc se rejoindre au milieu cesse de payer à pleine
          taille. Ce négatif est l'utile : il dit précisément où les méthodes exactes cèdent et où les
          heuristiques doivent prendre le relais. Les pièces exactes qui ont survécu, les tables de
          bornes inférieures de suffixe et le branch-and-bound élagué, sont devenues des instruments
          réutilisables. Un plateau sans cadre à 437 est sorti de la même mécanique.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Les tables de bornes inférieures peuvent-elles passer à la fin de partie 16x16 complète, ou
          l'espace d'états de couture devient-il trop grand ? À quelle taille de bande la mémoire plutôt
          que le temps devient-elle la limite ? Et les rares cas où la jonction du milieu se déclenche
          peuvent-ils être repérés à l'avance et finis exactement ?
        </p>
      </>
    ),
  },
};

export default function Bandsaw() {
  return (
    <InventionLayout
      copy={copy}
      score={437}
      reproducibility="deterministic"
      visual={<MeetInMiddleDiagram />}
    />
  );
}

export const meta = pageMeta("inv-bandsaw");
