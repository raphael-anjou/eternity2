import { pageMeta } from "@/seo";
import { InventionLayout, type InventionCopy } from "@/components/research/InventionLayout";
import { KeyringDiagram } from "@/components/research/KeyringDiagram";

const copy: { en: InventionCopy; fr: InventionCopy } = {
  en: {
    name: "KEYRING",
    tagline: "Build a board from scratch, ranking each next piece by three signals learned from past strong boards. Reached 460 in a board family no earlier search had cracked.",
    idea: "Building a board one piece at a time, the hard part is deciding which piece to place next when several would fit. A single rule of thumb tends to march the search into the same dead ends every time. KEYRING carries three different learned hunches at once, a keyring of them, and lets them vote, which keeps the search from over-trusting any one signal.",
    how: (
      <>
        <p>
          From the library of strong boards, KEYRING learns three things. First, where each piece
          likes to sit: how often a piece appears in each position across good boards. Second, which
          pieces like to be neighbours: how often two pieces end up touching. Third, which little 2x2
          patches show up in good boards versus bad ones.
        </p>
        <p>
          It then fills the board with a beam search, keeping many partial boards alive at once. When
          it has to choose the next piece, it scores each option by how many edges it matches, nudged
          by the three learned signals together. A small amount of randomness keeps the many parallel
          attempts from collapsing onto the same path.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          KEYRING reached 460 of 480, and did it in a corner arrangement where no board had reached
          that level before, so it's not just another route to a known board but a genuinely new
          region. Across repeated runs it landed a strong board far more often than the simpler
          single-signal version it grew out of.
        </p>
        <p>
          It is not the project's top score (that's 463), but finding a high board in a fresh family
          matters: the strong boards are known to be isolated from each other, so each new family is
          its own foothold.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Would grading the patch signal by degree, rather than treating patches as simply good or
          bad, help further? Could the weights on the three signals shift as the board fills, trusting
          structure more late in the build? And can this new family be pushed past 460 with a longer
          refinement?
        </p>
      </>
    ),
  },
  fr: {
    name: "KEYRING",
    tagline: "Construire un plateau de zéro en classant chaque pièce suivante par trois signaux appris des bons plateaux passés. A atteint 460 dans une famille de plateaux qu'aucune recherche antérieure n'avait percée.",
    idea: "En construisant un plateau pièce par pièce, le difficile est de décider quelle pièce poser ensuite quand plusieurs conviennent. Une seule règle empirique tend à mener la recherche dans les mêmes impasses à chaque fois. KEYRING porte trois intuitions apprises à la fois, un trousseau, et les laisse voter, ce qui évite de trop se fier à un seul signal.",
    how: (
      <>
        <p>
          À partir de la bibliothèque des bons plateaux, KEYRING apprend trois choses. D'abord, où
          chaque pièce aime se trouver : à quelle fréquence une pièce apparaît à chaque position dans
          les bons plateaux. Ensuite, quelles pièces aiment être voisines : combien de fois deux
          pièces finissent en contact. Enfin, quels petits carrés 2x2 apparaissent dans les bons
          plateaux par rapport aux mauvais.
        </p>
        <p>
          Il remplit ensuite le plateau par une recherche en faisceau, gardant de nombreux plateaux
          partiels en vie à la fois. Quand il doit choisir la pièce suivante, il évalue chaque option
          par le nombre de bords qu'elle apparie, ajusté par les trois signaux appris réunis. Un peu
          de hasard empêche les nombreuses tentatives parallèles de retomber sur le même chemin.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          KEYRING a atteint 460 sur 480, dans une disposition de coins où aucun plateau n'était
          parvenu à ce niveau auparavant : ce n'est donc pas une nouvelle route vers un plateau connu,
          mais une région vraiment nouvelle. Sur des exécutions répétées, il a obtenu un bon plateau
          bien plus souvent que la version à signal unique dont il est issu.
        </p>
        <p>
          Ce n'est pas le meilleur score du projet (463), mais trouver un plateau élevé dans une
          famille inédite compte : les bons plateaux sont connus pour être isolés les uns des autres,
          donc chaque nouvelle famille est un point d'appui à part.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Noter le signal des carrés par degré, plutôt que de les traiter simplement comme bons ou
          mauvais, aiderait-il davantage ? Les poids des trois signaux pourraient-ils évoluer à mesure
          que le plateau se remplit, en faisant davantage confiance à la structure en fin de
          construction ? Et cette nouvelle famille peut-elle dépasser 460 avec un affinage plus long ?
        </p>
      </>
    ),
  },
};

export default function Keyring() {
  return (
    <InventionLayout
      copy={copy}
      score={460}
      boardId="v181-keyring-460"
      reproducibility="stochastic"
      visual={<KeyringDiagram />}
    />
  );
}

export const meta = pageMeta("exp-keyring");
