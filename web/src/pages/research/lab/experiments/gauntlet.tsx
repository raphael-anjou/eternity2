import { pageMeta } from "@/seo";
import { InventionLayout, type InventionCopy } from "@/components/research/InventionLayout";
import { GauntletDiagram } from "@/components/research/GauntletDiagram";
import { GauntletLiveRace } from "@/components/research/GauntletLiveRace";
import { GauntletStepThrough } from "@/components/research/GauntletStepThrough";

const copy: { en: InventionCopy; fr: InventionCopy } = {
  en: {
    name: "GAUNTLET",
    tagline: "Run the same beam search across nine different scan orders, so it lands in different regions instead of always converging to the same one. The zigzag order found a brand-new 458 board.",
    idea: "A beam search that always fills the board in the same order tends to discover the same kind of board, no matter the random seed. The order you visit the cells quietly decides which region you end up in — and across sixteen seeds, a single scan order produced just one distinct corner-arrangement family. GAUNTLET turns that into a tool: run the search across nine genuinely different scan orders, and you sample genuinely different regions.",
    how: (
      <>
        <p>
          GAUNTLET forks the constructive PRIOR beam and gives it a scan order it can swap: row,
          row-reversed, column, column-reversed, zigzag, zigzag-reversed, spiral-in, spiral-out, and
          diagonal — nine in all. Because the order of cells changes which partial boards survive at
          each step, each order explores a different trajectory rather than crowding into one. A sweep
          of 9 scans × 4 seeds produced <strong>18 distinct corner-arrangement signatures</strong>,
          against just one across sixteen seeds of the single-scan beam it grew from.
        </p>
        <p>
          Each full run produces a complete board; the strongest are then polished by a 30-minute ALNS
          lift. The finding is sharp: <em>scan order is a stronger diversity axis than the random
          seed.</em> That is the lever, and it is what the later KEYRING and PRIOR build on.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          The zigzag order at seed 99, lifted, reached 458 of 480 in the corner arrangement
          cp=(3,0,1,2) — the first board at or above 458 ever found in that arrangement in our
          database. It is at least 246 of 256 cells away from anything we had at that level: a
          genuinely new basin, not another route to a known one.
        </p>
        <p>
          So GAUNTLET's contribution is reach rather than a record: it opened a new region of strong
          boards. A second round (32 more lifts) topped out at 457 with no 461, which says the new
          family saturates like the others — but the family itself was the prize.
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
    tagline: "Lancer la même recherche en faisceau à travers neuf ordres de parcours différents, pour atterrir dans des régions différentes au lieu de toujours converger vers la même. L'ordre en zigzag a trouvé un plateau 458 inédit.",
    idea: "Une recherche en faisceau qui remplit toujours le plateau dans le même ordre tend à découvrir le même genre de plateau, quel que soit le hasard — et sur seize graines, un seul ordre de parcours n'a produit qu'une seule famille de coins distincte. L'ordre de visite des cases décide discrètement de la région où l'on aboutit. GAUNTLET en fait un outil : lancer la recherche à travers neuf ordres vraiment différents, et l'on échantillonne des régions vraiment différentes.",
    how: (
      <>
        <p>
          GAUNTLET reprend le faisceau constructif PRIOR et lui donne un ordre de parcours
          interchangeable : rangée, rangée inversée, colonne, colonne inversée, zigzag, zigzag inversé,
          spirale entrante, spirale sortante et diagonale — neuf en tout. Comme l'ordre des cases
          change quels plateaux partiels survivent à chaque étape, chaque ordre explore une
          trajectoire différente. Un balayage de 9 ordres × 4 graines a produit{" "}
          <strong>18 signatures de coins distinctes</strong>, contre une seule sur seize graines du
          faisceau à ordre unique dont il est issu.
        </p>
        <p>
          Chaque exécution complète produit un plateau entier ; les meilleurs sont ensuite polis par un
          affinage ALNS de 30 minutes. Le constat est net : <em>l'ordre de parcours est un axe de
          diversité plus fort que la graine aléatoire.</em> C'est le levier, et c'est ce sur quoi
          KEYRING et PRIOR s'appuient ensuite.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          L'ordre en zigzag à la graine 99, affiné, a atteint 458 sur 480 dans la disposition de coins
          cp=(3,0,1,2) — le premier plateau à 458 ou plus jamais trouvé dans cette disposition dans
          notre base. Il est à au moins 246 cases sur 256 de tout ce que nous avions à ce niveau : un
          bassin vraiment nouveau, pas une autre route vers un bassin connu.
        </p>
        <p>
          La contribution de GAUNTLET est donc la portée plutôt qu'un record : il a ouvert une nouvelle
          région de bons plateaux. Un second tour (32 affinages de plus) a plafonné à 457 sans aucun
          461, ce qui montre que la nouvelle famille sature comme les autres — mais la famille
          elle-même était le prix.
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
    <InventionLayout
      copy={copy}
      score={458}
      boardId="v175-gauntlet-458"
      reproducibility="stochastic"
      visual={
        <div className="space-y-6">
          <GauntletDiagram />
          <GauntletStepThrough />
          <GauntletLiveRace />
        </div>
      }
    />
  );
}

export const meta = pageMeta("exp-gauntlet");
