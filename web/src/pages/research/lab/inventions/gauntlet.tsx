import { pageMeta } from "@/seo";
import { InventionLayout, type InventionCopy } from "@/components/research/InventionLayout";

const copy: { en: InventionCopy; fr: InventionCopy } = {
  en: {
    name: "GAUNTLET",
    tagline: "Run the same beam search in several fill directions at once, so it lands in different regions instead of always converging to the same one. Found a new 458 board.",
    idea: "A beam search that always fills the board in the same order tends to discover the same kind of board, no matter the random seed. The order you visit the cells quietly decides which region you end up in. GAUNTLET turns that into a tool: run the search several times with genuinely different fill orders, and you sample genuinely different regions.",
    how: (
      <>
        <p>
          GAUNTLET runs the constructive beam in four traversal orders, row by row, column by column,
          a zigzag, and reversed, each with a little randomness in how ties are broken. Because the
          order of cells changes which partial boards survive at each step, the four runs explore
          different trajectories rather than crowding into one.
        </p>
        <p>
          Each full run produces a complete board; the best is kept and then polished by local
          refinement. The point is coverage: several seeds across several orders, casting a wider net
          than any single order could.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          The zigzag order, refined, reached 458 of 480 in a corner arrangement that had no board at
          that level before. The score ties the existing best in its class, but the board sits in a
          different family from every earlier strong board, differing from the closest one in almost
          every cell.
        </p>
        <p>
          So GAUNTLET's contribution is reach rather than a record: it opened a new region of strong
          boards, which is exactly what the later KEYRING built on to go a step higher.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Does this new family have the same rigid local structure as the others, or a different
          shape? And can a longer refinement lift it past 458, the way a fresh family sometimes has
          more room than a well-worn one?
        </p>
      </>
    ),
  },
  fr: {
    name: "GAUNTLET",
    tagline: "Lancer la même recherche en faisceau dans plusieurs directions de remplissage à la fois, pour atterrir dans des régions différentes au lieu de toujours converger vers la même. A trouvé un nouveau plateau 458.",
    idea: "Une recherche en faisceau qui remplit toujours le plateau dans le même ordre tend à découvrir le même genre de plateau, quel que soit le hasard. L'ordre de visite des cases décide discrètement de la région où l'on aboutit. GAUNTLET en fait un outil : lancer la recherche plusieurs fois avec des ordres de remplissage vraiment différents, et l'on échantillonne des régions vraiment différentes.",
    how: (
      <>
        <p>
          GAUNTLET lance le faisceau constructif dans quatre ordres de parcours : rangée par rangée,
          colonne par colonne, en zigzag, et inversé, chacun avec un peu de hasard dans la résolution
          des égalités. Comme l'ordre des cases change quels plateaux partiels survivent à chaque
          étape, les quatre exécutions explorent des trajectoires différentes au lieu de se masser sur
          une seule.
        </p>
        <p>
          Chaque exécution complète produit un plateau entier ; le meilleur est conservé puis poli par
          affinage local. L'objectif est la couverture : plusieurs graines sur plusieurs ordres,
          jetant un filet plus large qu'aucun ordre unique ne le pourrait.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          L'ordre en zigzag, affiné, a atteint 458 sur 480 dans une disposition de coins qui n'avait
          aucun plateau à ce niveau auparavant. Le score égale le meilleur existant de sa catégorie,
          mais le plateau appartient à une famille différente de tous les bons plateaux antérieurs,
          différant du plus proche dans presque chaque case.
        </p>
        <p>
          La contribution de GAUNTLET est donc la portée plutôt qu'un record : il a ouvert une
          nouvelle région de bons plateaux, ce sur quoi KEYRING s'est ensuite appuyé pour monter d'un
          cran.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Cette nouvelle famille a-t-elle la même structure locale rigide que les autres, ou une
          forme différente ? Et un affinage plus long peut-il la pousser au-delà de 458, comme une
          famille fraîche a parfois plus de marge qu'une famille bien usée ?
        </p>
      </>
    ),
  },
};

export default function Gauntlet() {
  return (
    <InventionLayout copy={copy} score={458} boardId="v175-gauntlet-458" reproducibility="stochastic" />
  );
}

export const meta = pageMeta("inv-gauntlet");
