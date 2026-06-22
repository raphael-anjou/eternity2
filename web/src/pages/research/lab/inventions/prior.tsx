import { pageMeta } from "@/seo";
import { InventionLayout, type InventionCopy } from "@/components/research/InventionLayout";

const copy: { en: InventionCopy; fr: InventionCopy } = {
  en: {
    name: "PRIOR",
    tagline: "Build a board from nothing, breaking ties by where pieces tend to sit in the strong boards we already have. It reaches a high score with no starting board to copy.",
    idea: "Most strong boards on this site are found by taking an existing good board and improving it. PRIOR asks a harder question: can you build a competitive board from an empty grid, with no board to anchor to? The trick is to let the crowd of past good boards quietly guide the construction without copying any single one of them.",
    how: (
      <>
        <p>
          From the library of boards scoring well, PRIOR learns one simple thing: for each position on
          the board, how often each piece shows up there. That gives a gentle preference, a prior, for
          what tends to belong where.
        </p>
        <p>
          Then it builds with a beam search, keeping many partial boards alive at once and extending
          them cell by cell. When two options match the same number of edges, the prior breaks the tie
          toward the piece that's more typical of strong boards in that spot. A diversity rule keeps
          the many parallel attempts from collapsing onto the same path. No single board is copied; the
          guidance is statistical.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          From scratch, PRIOR reaches the mid-450s, and with a sharper prior built from only the very
          best boards it climbs higher. Followed by local refinement it reaches 460, the board shown
          here. That a from-nothing build lands this close to the records is the point: the structure
          of good boards is partly learnable, and you don't need to start from one to get there.
        </p>
        <p>
          It is not the project's top score (463), and the final lift here leans on a refinement step,
          which the board's label notes. But as a clean-slate result it's the strongest the project
          has, and it's the foundation the later multi-signal builders grew from.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          How sharp can the prior get before it overfits? Built from only a handful of top boards the
          signal is strong but thin. Could a separate prior per board-family capture structure the
          combined one averages away? And how much of the final gap is the construction versus the
          refinement that follows it?
        </p>
      </>
    ),
  },
  fr: {
    name: "PRIOR",
    tagline: "Construire un plateau à partir de rien, en départageant les égalités selon l'endroit où les pièces se trouvent dans les bons plateaux déjà connus. Il atteint un score élevé sans plateau de départ à copier.",
    idea: "La plupart des bons plateaux de ce site sont trouvés en partant d'un bon plateau existant pour l'améliorer. PRIOR pose une question plus dure : peut-on construire un plateau compétitif depuis une grille vide, sans plateau d'ancrage ? L'astuce est de laisser la foule des bons plateaux passés guider discrètement la construction sans en copier aucun.",
    how: (
      <>
        <p>
          À partir de la bibliothèque des plateaux bien notés, PRIOR apprend une chose simple : pour
          chaque position du plateau, à quelle fréquence chaque pièce y apparaît. Cela donne une
          préférence douce, un prior, sur ce qui tend à aller où.
        </p>
        <p>
          Puis il construit par une recherche en faisceau, gardant de nombreux plateaux partiels en vie
          et les étendant case par case. Quand deux options apparient le même nombre de bords, le prior
          tranche en faveur de la pièce la plus typique des bons plateaux à cet endroit. Une règle de
          diversité empêche les tentatives parallèles de retomber sur le même chemin. Aucun plateau
          n'est copié ; le guidage est statistique.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          De zéro, PRIOR atteint le milieu des 450, et avec un prior plus pointu bâti uniquement sur les
          tout meilleurs plateaux il monte plus haut. Suivi d'un affinage local, il atteint 460, le
          plateau montré ici. Qu'une construction partie de rien arrive si près des records est tout
          l'intérêt : la structure des bons plateaux est en partie apprenable, et il n'est pas nécessaire
          d'en partir pour y arriver.
        </p>
        <p>
          Ce n'est pas le meilleur score du projet (463), et la dernière poussée s'appuie ici sur une
          étape d'affinage, ce que l'étiquette du plateau signale. Mais comme résultat à partir de zéro,
          c'est le plus fort du projet, et c'est la base sur laquelle les constructeurs multi-signaux
          ultérieurs se sont développés.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Jusqu'où le prior peut-il être affûté avant de surapprendre ? Bâti sur une poignée de plateaux
          de tête, le signal est fort mais mince. Un prior distinct par famille de plateaux capterait-il
          une structure que le prior combiné gomme en moyenne ? Et quelle part de l'écart final tient à
          la construction plutôt qu'à l'affinage qui la suit ?
        </p>
      </>
    ),
  },
};

export default function Prior() {
  return (
    <InventionLayout copy={copy} score={460} boardId="v155-prior-460" reproducibility="seeded" />
  );
}

export const meta = pageMeta("inv-prior");
